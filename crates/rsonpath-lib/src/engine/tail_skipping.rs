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
        performance::lut_skip_evaluation::{self, SkipMode},
        LookUpTable, LUT, USE_SKIP_ABORT_STRATEGY,
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
        idx_open: usize, // padded
        idx: usize,      // padded
        bracket_type: BracketType,
        lut: Option<&LUT>,
        padding: usize,
    ) -> Result<usize, EngineError> {
        if lut_skip_evaluation::TRACK_SKIPPING_TIME_DURING_PERFORMANCE_TEST {
            let start_skip = Instant::now();
            let result = self.skip_choice(idx_open, idx, bracket_type, lut, padding);
            let skip_time = start_skip.elapsed().as_nanos() as u64;
            lut_skip_evaluation::add_skip_time(skip_time);
            return result;
        } else {
            self.skip_choice(idx_open, idx, bracket_type, lut, padding)
        }
    }

    fn skip_choice(
        &mut self,
        idx_open: usize, // padded
        idx: usize,      // padded
        bracket_type: BracketType,
        lut: Option<&LUT>,
        padding: usize,
    ) -> Result<usize, EngineError> {
        if let Some(lut) = lut {
            if USE_SKIP_ABORT_STRATEGY {
                // use ITE mainly and LUT only when the jump distance exceeds CUTOFF
                self.skip_lut_abort(idx_open, idx, bracket_type, lut, padding)
            } else {
                // use only the LUT, even for short distances
                self.skip_lut(idx_open, idx, bracket_type, lut, padding)
            }
        } else {
            // default
            let idx_close = self.skip_ite(bracket_type)?;
            if idx >= padding && idx_open >= padding && idx_close >= padding {
                debug!(
                    "ITE: {}: ({}, {}) No-PAD: {}, ({}, {})",
                    idx,
                    idx_open,
                    idx_close,
                    idx - padding,
                    idx_open - padding,
                    idx_close - padding,
                );
            } else {
                debug!(
                    "ITE: {}: ({}, {}) Cannot show padding, cause values < padding = {}",
                    idx, idx_open, idx_close, padding
                );
            }

            Ok(idx_close)
        }
    }

    // Skip using the LUT as helper. If the LUT has a miss skip ITE style.
    fn skip_lut(
        &mut self,
        idx_open: usize, // padded
        idx: usize,      // padded
        bracket_type: BracketType,
        lut: &LUT,
        padding: usize,
    ) -> Result<usize, EngineError> {
        // Get value from LUT, can hit or miss.
        if let Some(idx_lut) = lut.get(&(idx_open - padding)) {
            // Note: shift index by 1 or its off aligned
            let idx_close = idx_lut + 1 + padding;

            // Only for tracking jumps and not needed in normal runs
            if !(lut_skip_evaluation::SKIP_MODE == SkipMode::OFF) {
                let distance = idx_close - idx;
                debug!("Track distance = {distance}");
                track_distance_lut(distance);
            }

            if idx >= padding && idx_open >= padding && idx_close >= padding {
                debug!(
                    "LUT: {}: ({}, {}) No-PAD: {}, ({}, {})",
                    idx,
                    idx_open,
                    idx_close,
                    idx - padding,
                    idx_open - padding,
                    idx_close - padding,
                );
            } else {
                debug!(
                    "LUT: {}: ({}, {}) Cannot show padding, cause values < padding = {}",
                    idx, idx_open, idx_close, padding
                );
            }

            self.classifier
                .as_mut()
                .expect("tail skip must always hold a classifier")
                .jump_to_idx(idx_close, false)?;

            Ok(idx_close)
        } else {
            // LUT had no hit, skip ITE style
            let idx_close = self.skip_ite(bracket_type)?;

            // Only for tracking jumps and not needed in normal runs
            if !(lut_skip_evaluation::SKIP_MODE == SkipMode::OFF) {
                let distance = idx_close - idx;
                debug!("Track distance = {distance}");
                track_distance_ite(distance);
            }

            debug!(
                "ITE: {}: ({}, {}) No-PAD: {}, ({}, {})",
                idx,
                idx_open,
                idx_close,
                idx - padding,
                idx_open - padding,
                idx_close - padding,
            );

            Ok(idx_close)
        }
    }

    /// With CUTOFF we defined a minimum distance value that all the LUTs will track. That means when we
    /// skip we want to skip normally ITE style until we reach the CUTOFF. Then we want to query the LUT,
    /// since we know it will cover the larger skips, and then skip LUT style, because it should be faster.
    fn skip_lut_abort(
        &mut self,
        idx_open: usize,
        idx: usize,
        bracket_type: BracketType,
        lut: &LUT,
        padding: usize,
    ) -> Result<usize, EngineError> {
        dispatch_simd!(self.simd; self, bracket_type, idx_open, idx, lut, padding =>
        fn <'i, I, V>(
            tail_skip: &mut TailSkip<'i, I, V::QuotesClassifier<'i, I>, V::StructuralClassifier<'i, I>, V, BLOCK_SIZE>,
            opening: BracketType, idx_open: usize, idx: usize, lut: &LUT, padding: usize) -> Result<usize, EngineError>
        where
            I: InputBlockIterator<'i, BLOCK_SIZE>,
            V: Simd
        {
            let mut idx_close = 0;
            let mut err = None;
            let mut skip_with_lut = false;
            let mut idx_lut: usize = 0;
            let idx_open_no_pad = idx_open - padding;

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
                let mut track_skipped_distance = true;
                let mut skipped_distance = 0;

                'outer: while let Some(ref mut vector) = current_vector {
                    if track_skipped_distance {
                        skipped_distance = skipped_distance + BLOCK_SIZE;
                        if skipped_distance > lut.get_cutoff() {
                            // Check if the LUT has a hit
                            if let Some(lut_result) = lut.get(&idx_open_no_pad) {
                                // Stop skipping ITE style and skip LUT style
                                skip_with_lut = true;
                                idx_lut = lut_result;

                                let resume_state = depth_classifier.stop(current_vector);
                                idx_close = resume_state.get_idx();
                                break 'a tail_skip.simd.resume_structural_classification(resume_state)
                            } else {
                                // The LUT has no hit, continue iterating ITE style. (Note: Once the PackedStackFrame
                                // logic is implemented this should never happen and the LUT always has a hit.)
                                track_skipped_distance = false;
                            }
                        }
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
                idx_close = resume_state.get_idx();
                tail_skip.simd.resume_structural_classification(resume_state)
            });

            // Skip LUT style if skipped_distance > CUTOFF
            if skip_with_lut {
                // Shift index by 1 or its off aligned
                let idx_close = idx_lut + 1 + padding;

                // Tell the Structural Classifier to jump
                tail_skip.classifier
                    .as_mut()
                    .expect("tail skip must always hold a classifier")
                    .jump_to_idx(idx_close, false)?;

                if !(lut_skip_evaluation::SKIP_MODE == SkipMode::OFF) {
                    let distance = idx_close - idx;
                    debug!("LUT: Track distance = {distance}");
                    track_distance_lut(distance);
                }
                if idx >= padding && idx_open >= padding && idx_close >= padding {
                    debug!(
                        "LUT: {}: ({}, {}) No-PAD: {}, ({}, {})",
                        idx,
                        idx_open,
                        idx_close,
                        idx - padding,
                        idx_open - padding,
                        idx_close - padding,
                    );
                } else {
                    debug!(
                        "LUT: {}: ({}, {}) Cannot show padding, cause values < padding = {}",
                        idx, idx_open, idx_close, padding
                    );
                }

                return Ok(idx_close);
            }

            if let Some(err) = err {
                Err(err.into())
            } else {
                if !(lut_skip_evaluation::SKIP_MODE == SkipMode::OFF) {
                    let distance = idx_close - idx;
                    debug!("ITE: Track distance = {distance}");
                    track_distance_ite(distance);
                }

                debug!(
                    "ITE: {}: ({}, {}) No-PAD: {}, ({}, {})",
                    idx,
                    idx_open,
                    idx_close,
                    idx - padding,
                    idx_open - padding,
                    idx_close - padding,
                );


                Ok(idx_close)
            }
        })
    }

    // TODO Ricardo reenable the out commented debug lines
    fn skip_ite(&mut self, bracket_type: BracketType) -> Result<usize, EngineError> {
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
            let end_idx1 = classifier.skip_ite(BracketType::Curly)?;
            assert_eq!(126, end_idx1);

            // Now we expect to skip to the very end of the document.
            let end_idx2 = classifier.skip_ite(BracketType::Curly)?;
            assert_eq!(128, end_idx2);

            Ok::<(), EngineError>(())
        })?;
        Ok(())
    }
}
