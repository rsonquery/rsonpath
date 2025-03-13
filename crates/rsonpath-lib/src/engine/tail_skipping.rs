#![allow(clippy::expect_used)] // Enforcing the classifier invariant is clunky without this.
use crate::{
    classification::{
        depth::{DepthBlock, DepthIterator, DepthIteratorResumeOutcome},
        quotes::QuoteClassifiedIterator,
        simd::{dispatch_simd, Simd},
        structural::{BracketType, StructuralIterator},
        ResumeClassifierState,
    },
    debug,
    engine::{
        error::EngineError,
        skip_tracker::{track_distance_ite, track_distance_lut},
    },
    input::InputBlockIterator,
    lookup_table::{
        lut_hash_map,
        performance::{
            lut_query_data::POKEMON_MINI,
            lut_skip_evaluation::{self, SkipMode, DISTANCE_CUT_OFF, USE_SKIP_ABORT_STRATEGY},
        },
        LookUpTable, LUT,
    },
    FallibleIterator, MaskType, BLOCK_SIZE,
};
use std::{marker::PhantomData, time::Instant};

pub(crate) struct TailSkip<'i, I, Q, S, V, const N: usize> {
    classifier: Option<S>,
    simd: V,
    _phantom: (PhantomData<&'i ()>, PhantomData<(I, Q)>),
}

impl<'i, I, V> TailSkip<'i, I, V::QuotesClassifier<'i, I>, V::StructuralClassifier<'i, I>, V, BLOCK_SIZE>
where
    I: InputBlockIterator<'i, BLOCK_SIZE>,
    V: Simd,
{
    pub(crate) fn new(classifier: V::StructuralClassifier<'i, I>, simd: V) -> Self {
        Self {
            classifier: Some(classifier),
            simd,
            _phantom: (PhantomData, PhantomData),
        }
    }

    /// Returns the index position where the parser skips to. Given the opening bracket this returns the position of
    /// the closing bracket.
    ///
    /// The skip is based either on opening_idx + lookup-table (LUT) to find goal position via a data structure OR
    /// given just the BracketType the parser iteratively reads blocks until the closing bracket is found. If the skip
    /// has a long distance then using the LUT should be faster.
    pub(crate) fn skip(
        &mut self,
        opening_idx_padded: usize,
        bracket_type: BracketType,
        lut: Option<&LUT>,
        padding: usize,
    ) -> Result<usize, EngineError> {
        if lut_skip_evaluation::TRACK_SKIPPING_DURING_PERFORMANCE_TEST {
            let start_skip = Instant::now();
            let result = self.skip_choice(opening_idx_padded, bracket_type, lut, padding);
            let skip_time = start_skip.elapsed().as_nanos() as u64;
            lut_skip_evaluation::add_skip_time(skip_time);
            return result;
        } else {
            self.skip_choice(opening_idx_padded, bracket_type, lut, padding)
        }
    }

    fn skip_choice(
        &mut self,
        opening_idx_padded: usize,
        bracket_type: BracketType,
        lut: Option<&LUT>,
        padding: usize,
    ) -> Result<usize, EngineError> {
        if let Some(lut) = lut {
            if USE_SKIP_ABORT_STRATEGY {
                self.skip_without_lut_abort(opening_idx_padded, bracket_type, lut, padding)
            } else {
                self.skip_with_lut(opening_idx_padded, bracket_type, lut, padding)
            }
        } else {
            self.skip_without_lut(bracket_type)
        }
    }

    // 0. Use LUT to get opening -> closing index
    // 1. Tell the Structural Classifier (self.classifier) to jump
    // 2. S tells its quote classifier to jump
    // 3. Q tells the InputIterator to jump
    // 4. Implement jump in InputBlockIterators
    // 5. Q needs to reclassify the new current block.
    // 6. S needs to reclassify the new current block.
    // 7. This function returns the skipped-to index.
    fn skip_with_lut(
        &mut self,
        open_idx_pad: usize,
        bracket_type: BracketType,
        lut: &LUT,
        padding: usize,
    ) -> Result<usize, EngineError> {
        let open_idx = open_idx_pad - padding;

        // TODO delete this
        // let lut_hash_map = lut_hash_map::LutHashMap::build(POKEMON_MINI, 0).expect("Fail @ build lut hash map");
        // print!("Try skip with key {} ... ", open_idx);

        // 0. Use LUT to get opening -> closing index. Can fail if key is not in LUT
        if let Some(lut_idx) = lut.get(&open_idx) {
            // TODO delete this if
            // println!("LUT");
            // if let Some(lut_hash_map_idx) = lut_hash_map.get(&open_idx){
            //     if lut_hash_map_idx == lut_idx {
            //         println!("SKIP OK: Key {}, Found {}, Expected {}", open_idx, lut_idx, lut_hash_map_idx);
            //     } else {
            //         println!("SKIP NOT OK: Key {}, Found {}, Expected {}", open_idx, lut_idx, lut_hash_map_idx);
            //     }
            // } else {
            //     println!("FFFFFFFFFFFFFFFF SKIP: Key {}, Found {}, Expected NO-HIT", open_idx, lut_idx);
            // }

            // Shift index by 1 or its off aligned
            let close_idx = lut_idx + 1;
            let close_idx_pad = padding + close_idx;

            debug!(
                "LUT:({},{}) No-PAD:({},{})",
                open_idx_pad, close_idx_pad, open_idx, close_idx
            );

            if !(lut_skip_evaluation::MODE == SkipMode::OFF) {
                // Only for tracking jumps and not needed in normal runs
                let distance = close_idx - open_idx;
                track_distance_lut(distance);
            }

            // 1. Tell the Structural Classifier (self.classifier) to jump
            self.classifier
                .as_mut()
                .expect("tail skip must always hold a classifier")
                .jump_to_idx(close_idx_pad, false)?;

            // 7. This function returns the skipped-to index.
            Ok(close_idx_pad)
        } else {
            // TODO delete
            // println!("ITE");

            // Do this when you were not able to find any values in the LUT
            let close_idx_pad = self.skip_without_lut(bracket_type)?;
            let close_idx = close_idx_pad - padding;

            debug!(
                "ITE:({},{}) No-PAD:({},{})",
                open_idx_pad, close_idx_pad, open_idx, close_idx
            );

            if !(lut_skip_evaluation::MODE == SkipMode::OFF) {
                // Only for tracking jumps and not needed in normal runs
                let distance = close_idx - open_idx;
                track_distance_ite(distance);
            }

            Ok(close_idx_pad)
        }
    }

    // Idea: skip the first 1000 or so bytes normally and only if the jump is big use the LUT access
    fn skip_without_lut_abort(
        &mut self,
        opening_idx_padded: usize,
        bracket_type: BracketType,
        lut: &LUT,
        padding: usize,
    ) -> Result<usize, EngineError> {
        dispatch_simd!(self.simd; self, bracket_type =>
        fn <'i, I, V>(
            tail_skip: &mut TailSkip<'i, I, V::QuotesClassifier<'i, I>, V::StructuralClassifier<'i, I>, V, BLOCK_SIZE>,
            opening: BracketType) -> Result<usize, EngineError>
        where
            I: InputBlockIterator<'i, BLOCK_SIZE>,
            V: Simd
        {
            let mut idx = 0;
            let mut err = None;

            let classifier = tail_skip.classifier.take().expect("tail skip must always hold a classifier");

            let mut skip_with_lut_now = false;

            tail_skip.classifier = Some('a: {
                let resume_state = classifier.stop();
                let DepthIteratorResumeOutcome(first_vector, mut depth_classifier) =
                    tail_skip.simd.resume_depth_classification(resume_state, opening);

                let mut current_vector = match first_vector {
                    Some(v) => Some(v),
                    None => match depth_classifier.next() {
                        Ok(v) => v,
                        Err(e) => {
                            err = Some(e);
                            let resume_state = depth_classifier.stop(None);
                            break 'a tail_skip.simd.resume_structural_classification(resume_state);
                        }
                    },
                };
                let mut current_depth = 1;


                let mut distance_counter = 0;
                'outer: while let Some(ref mut vector) = current_vector {
                    distance_counter = distance_counter + BLOCK_SIZE;
                    if distance_counter > DISTANCE_CUT_OFF {
                        // TODO stop skipping ITE style and skip LUT style
                        skip_with_lut_now = true;

                        let resume_state = depth_classifier.stop(current_vector);
                        idx = resume_state.get_idx();
                        break 'a tail_skip.simd.resume_structural_classification(resume_state);
                    }

                    vector.add_depth(current_depth);
                    if vector.estimate_lowest_possible_depth() <= 0 {
                        while vector.advance_to_next_depth_decrease() {
                            if vector.get_depth() == 0 {
                                // debug!("Encountered depth 0, breaking.");
                                break 'outer;
                            }
                        }
                    }
                    current_depth = vector.depth_at_end();
                    current_vector = match depth_classifier.next() {
                        Ok(v) => v,
                        Err(e) => {
                            err = Some(e);
                            let resume_state = depth_classifier.stop(None);
                            break 'a tail_skip.simd.resume_structural_classification(resume_state);
                        }
                    };
                }

                let resume_state = depth_classifier.stop(current_vector);
                idx = resume_state.get_idx();
                tail_skip.simd.resume_structural_classification(resume_state)
            });



            if let Some(err) = err {
                Err(err.into())
            } else {
                Ok(idx)
            }
        })
    }

    // TODO Ricardo uncomment every debug that was commented out here
    fn skip_without_lut(&mut self, bracket_type: BracketType) -> Result<usize, EngineError> {
        dispatch_simd!(self.simd; self, bracket_type =>
        fn <'i, I, V>(
            tail_skip: &mut TailSkip<'i, I, V::QuotesClassifier<'i, I>, V::StructuralClassifier<'i, I>, V, BLOCK_SIZE>,
            opening: BracketType) -> Result<usize, EngineError>
        where
            I: InputBlockIterator<'i, BLOCK_SIZE>,
            V: Simd
        {
            let mut idx = 0;
            let mut err = None;

            let classifier = tail_skip.classifier.take().expect("tail skip must always hold a classifier");

            tail_skip.classifier = Some('a: {
                let resume_state = classifier.stop();
                let DepthIteratorResumeOutcome(first_vector, mut depth_classifier) =
                    tail_skip.simd.resume_depth_classification(resume_state, opening);

                let mut current_vector = match first_vector {
                    Some(v) => Some(v),
                    None => match depth_classifier.next() {
                        Ok(v) => v,
                        Err(e) => {
                            err = Some(e);
                            let resume_state = depth_classifier.stop(None);
                            break 'a tail_skip.simd.resume_structural_classification(resume_state);
                        }
                    },
                };
                let mut current_depth = 1;

                'outer: while let Some(ref mut vector) = current_vector {
                    vector.add_depth(current_depth);

                    // debug!("Fetched vector, current depth is {current_depth}");
                    // debug!("Estimate: {}", vector.estimate_lowest_possible_depth());

                    if vector.estimate_lowest_possible_depth() <= 0 {
                        while vector.advance_to_next_depth_decrease() {
                            if vector.get_depth() == 0 {
                                // debug!("Encountered depth 0, breaking.");
                                break 'outer;
                            }
                        }
                    }

                    current_depth = vector.depth_at_end();
                    current_vector = match depth_classifier.next() {
                        Ok(v) => v,
                        Err(e) => {
                            err = Some(e);
                            let resume_state = depth_classifier.stop(None);
                            break 'a tail_skip.simd.resume_structural_classification(resume_state);
                        }
                    };
                }

                // debug!("Skipping complete, resuming structural classification.");
                let resume_state = depth_classifier.stop(current_vector);
                // debug!("Finished at {}", resume_state.get_idx());
                idx = resume_state.get_idx();
                tail_skip.simd.resume_structural_classification(resume_state)
            });


            if let Some(err) = err {
                Err(err.into())
            } else {
                Ok(idx)
            }
        })
    }

    pub(crate) fn stop(self) -> ResumeClassifierState<'i, I, V::QuotesClassifier<'i, I>, MaskType, BLOCK_SIZE> {
        self.classifier.expect("tail skip must always hold a classifier").stop()
    }
}

impl<'i, I, Q, S, V, const N: usize> std::ops::Deref for TailSkip<'i, I, Q, S, V, N>
where
    I: InputBlockIterator<'i, N>,
    Q: QuoteClassifiedIterator<'i, I, MaskType, N>,
    S: StructuralIterator<'i, I, Q, MaskType, N>,
    V: Simd,
{
    type Target = S;

    fn deref(&self) -> &Self::Target {
        self.classifier
            .as_ref()
            .expect("tail skip must always hold a classifier")
    }
}

impl<'i, I, Q, S, V, const N: usize> std::ops::DerefMut for TailSkip<'i, I, Q, S, V, N>
where
    I: InputBlockIterator<'i, N>,
    Q: QuoteClassifiedIterator<'i, I, MaskType, N>,
    S: StructuralIterator<'i, I, Q, MaskType, N>,
    V: Simd,
{
    fn deref_mut(&mut self) -> &mut Self::Target {
        self.classifier
            .as_mut()
            .expect("tail skip must always hold a classifier")
    }
}

#[cfg(test)]
mod tests {
    use crate::{
        classification::{
            simd::{self, config_simd, Simd},
            structural::{BracketType, Structural},
        },
        engine::{error::EngineError, tail_skipping::TailSkip},
        input::{Input, OwnedBytes},
        result::empty::EmptyRecorder,
        FallibleIterator,
    };

    /// Skipping that ends at the very end of a block is an edge case that triggered a bug once.
    #[test]
    fn skipping_over_block_boundary() -> Result<(), EngineError> {
        // Force the bytes to be misaligned from the 128B boundary.
        #[repr(C, align(128))]
        struct Misaligned {
            /// Misalign by 1B.
            pad: u8,
            /// JSON goes here.
            arr: [u8; 37],
        }
        impl std::borrow::Borrow<[u8]> for &Misaligned {
            fn borrow(&self) -> &[u8] {
                &self.arr
            }
        }
        // We will model the query $.a..b which causes one skip from the second curly
        // and then at the end a skip of the entire object.
        let json = r#"{"a":[{"c":{"d":[42,43,44],"b":45}}]}"#;
        let mut misaligned = Misaligned { pad: 0, arr: [0; 37] };
        misaligned.arr.copy_from_slice(json.as_bytes());
        let input = OwnedBytes::new(&misaligned);

        let simd = simd::configure();
        config_simd!(simd => |simd| {
            let recorder = EmptyRecorder;
            let iter = input.iter_blocks(&recorder);
            let quote_classifier = simd.classify_quoted_sequences(iter);
            let structural_classifier = simd.classify_structural_characters(quote_classifier);
            let mut classifier = TailSkip::new(structural_classifier, simd);

            assert_eq!(Some(Structural::Opening(BracketType::Curly, 91)), classifier.next()?);
            assert_eq!(Some(Structural::Opening(BracketType::Square, 96)), classifier.next()?);
            assert_eq!(Some(Structural::Opening(BracketType::Curly, 97)), classifier.next()?);

            // We've read this one
            //       v
            // {"a":[{"c":{"d":[42,43,44],"b":45}}]}
            //                                  ^ and skip to here.
            let end_idx1 = classifier.skip_without_lut(BracketType::Curly)?;
            assert_eq!(126, end_idx1);

            // Now we expect to skip to the very end of the document.
            let end_idx2 = classifier.skip_without_lut(BracketType::Curly)?;
            assert_eq!(128, end_idx2);

            Ok::<(), EngineError>(())
        })?;
        Ok(())
    }
}
