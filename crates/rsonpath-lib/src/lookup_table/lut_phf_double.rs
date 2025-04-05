use std::{collections::VecDeque, fs};

use super::{
    lut_phf::{
        phf_generator_double_hash::{self, HashState},
        DEFAULT_LAMBDA, DEFAULT_THREADED,
    },
    LookUpTable, LookUpTableLambda,
};
use crate::{
    classification::{
        self,
        simd::Simd,
        structural::{BracketType, Structural, StructuralIterator},
    },
    input::{self, error, Input},
    result::empty::EmptyRecorder,
    FallibleIterator,
};

// 2^16, since we want to consider all values that fit into a 16 bit representation
pub const THRESHOLD_16_BITS: usize = u16::MAX as usize;

/// Helper struct, because it makes the code shorter and cleaner to read.
#[derive(Clone, Default)]
pub struct PairData {
    pub keys: Vec<usize>,
    pub values: Vec<u16>,
    pub keys_64: Vec<usize>,
    pub values_64: Vec<usize>,
}

impl PairData {
    #[inline]
    #[must_use]
    pub fn new() -> Self {
        Self {
            keys: vec![],
            values: vec![],
            keys_64: vec![],
            values_64: vec![],
        }
    }
}
pub struct LutPHFDouble {
    pub lambda: usize,
    pub hash_state_16: HashState<u16>,
    pub hash_state_64: HashState<usize>,
}

impl LookUpTable for LutPHFDouble {
    #[inline]
    fn build(json_path: &str, cutoff: usize) -> Result<Self, Box<dyn std::error::Error>> {
        Self::build_lambda(DEFAULT_LAMBDA, json_path, cutoff, DEFAULT_THREADED)
    }

    #[inline]
    fn get(&self, key: &usize) -> Option<usize> {
        // Look for a value for a given key. Search first in `hash_state_16` because it represents
        // >99% of the keys. On rare misses (indicated by value == 0), check `hash_state_64`. If the hash_state
        // returns None the key was never part of the original key set at build time
        if let Some(value_16) = self.hash_state_16.get(key) {
            if value_16 == 0 {
                if let Some(value_64) = self.hash_state_64.get(key) {
                    return Some(*key + value_64);
                } else {
                    return None;
                }
            }
            return Some(*key + value_16 as usize);
        }
        None
    }

    #[inline]
    fn allocated_bytes(&self) -> usize {
        let mut total_size = std::mem::size_of::<Self>();
        total_size += self.hash_state_16.allocated_bytes();
        total_size += self.hash_state_64.allocated_bytes();

        total_size
    }
}

impl LookUpTableLambda for LutPHFDouble {
    #[inline]
    fn build_lambda(
        lambda: usize,
        json_path: &str,
        cutoff: usize,
        threaded: bool,
    ) -> Result<Self, Box<dyn std::error::Error>> {
        let file = fs::File::open(json_path).expect("Failed to open file");
        // SAFETY: We keep the file open throughout the entire duration.
        let input = unsafe { input::MmapInput::map_file(&file)? };
        let simd_c = classification::simd::configure();

        let lut_phf_double = classification::simd::config_simd!(simd_c => |simd| {
            classification::simd::dispatch_simd!(simd; input, simd, lambda, cutoff, threaded => fn<I, V>(
                input: I,
                simd: V,
                lambda: usize,
                cutoff: usize,
                threaded: bool,
            ) -> Result<LutPHFDouble, error::InputError> where
            I: Input,
            V: Simd, {
                    let pair_data = LutPHFDouble::find_all_pairs::<I, V>(&input, simd, cutoff)?;
                    Ok(LutPHFDouble::build_double(lambda, &pair_data, threaded))
                })
        });
        lut_phf_double.map_err(|e| Box::new(e) as Box<dyn std::error::Error>)
    }
}

impl LutPHFDouble {
    #[inline]
    #[must_use]
    pub fn build_double(lambda: usize, pair_data: &PairData, threaded: bool) -> Self {
        // 1) Build hash_16
        let hash_state_16_usize = phf_generator_double_hash::build(lambda, &pair_data.keys, threaded);
        // Replace indexes with actual values from values_16
        let map_16: Vec<u16> = hash_state_16_usize
            .map
            .into_iter()
            .map(|idx| pair_data.values[idx])
            .collect();
        let hash_state_16: HashState<u16> = HashState {
            lambda,
            hash_key: hash_state_16_usize.hash_key,
            displacements: hash_state_16_usize.displacements,
            map: map_16,
        };

        // 2) Build hash_64
        let mut hash_state_64 = phf_generator_double_hash::build(lambda, &pair_data.keys_64, threaded);
        // Replace indexes with values, since otherwise we would map to a index position and not the actual value
        hash_state_64.map = hash_state_64
            .map
            .into_iter()
            .map(|idx| pair_data.values_64[idx])
            .collect();

        Self {
            lambda,
            hash_state_16,
            hash_state_64,
        }
    }

    /// We count the distances between the opening and closing brackets. We save the start position as key and
    /// distance to the closing bracket in the value. Creates a key-value list for values which fit in a 16 bit
    /// representation and another key-value list for the ones that do not.
    #[inline]
    pub(crate) fn find_all_pairs<I, V>(input: &I, simd: V, cutoff: usize) -> Result<PairData, error::InputError>
    where
        I: Input,
        V: Simd,
    {
        let iter = input.iter_blocks::<_, 64>(&EmptyRecorder);
        let quote_classifier = simd.classify_quoted_sequences(iter);
        let mut structural_classifier = simd.classify_structural_characters(quote_classifier);
        structural_classifier.turn_colons_and_commas_off();

        // Initialize two empty stacks: one for "[" and one for "{", to remember the order we have found them
        let mut square_bracket_stack: VecDeque<usize> = VecDeque::new();
        let mut curly_bracket_stack: VecDeque<usize> = VecDeque::new();

        let mut pairs = PairData::new();

        while let Some(event) = structural_classifier.next()? {
            match event {
                Structural::Opening(b, idx_open) => match b {
                    BracketType::Square => square_bracket_stack.push_back(idx_open),
                    BracketType::Curly => curly_bracket_stack.push_back(idx_open),
                },
                Structural::Closing(b, idx_close) => {
                    let idx_open = match b {
                        BracketType::Square => square_bracket_stack.pop_back().expect("Unmatched closing }"),
                        BracketType::Curly => curly_bracket_stack.pop_back().expect("Unmatched closing }"),
                    };

                    // Check if distance can be represented with 16 or less bits
                    let distance = idx_close - idx_open;
                    if distance >= cutoff {
                        if distance < THRESHOLD_16_BITS {
                            // Can fit into 16 bit
                            pairs.keys.push(idx_open);
                            pairs.values.push(distance.try_into().expect("Fail at pushing value."));
                        } else {
                            // Cannot fit into 16 bit
                            pairs.keys.push(idx_open);
                            pairs.values.push(0);
                            pairs.keys_64.push(idx_open);
                            pairs.values_64.push(distance);
                        }
                    }
                }
                Structural::Colon(_) | Structural::Comma(_) => unreachable!(),
            }
        }

        Ok(pairs)
    }
}
