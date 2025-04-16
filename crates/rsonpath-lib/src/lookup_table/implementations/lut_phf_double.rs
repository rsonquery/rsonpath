use std::fs;

use super::lut_phf::{
    phf_generator_double_hash::{self, HashState},
    DEFAULT_LAMBDA, DEFAULT_THREADED,
};
use crate::lookup_table::pair_data::PairData;
use crate::lookup_table::{LookUpTable, LookUpTableLambda};
use crate::{
    classification::{self, simd::Simd},
    input::{self, error, Input},
    lookup_table::pair_data,
    FallibleIterator,
};

pub struct LutPHFDouble {
    pub lambda: usize,
    pub hash_state: HashState<u16>,
    pub hash_state_64: HashState<usize>,
    pub cutoff: usize,
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
        if let Some(value_16) = self.hash_state.get(key) {
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

    fn get_cutoff(&self) -> usize {
        self.cutoff
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
                    let pair_data = pair_data::find_pairs::<I, V>(&input, simd, cutoff)?;
                    Ok(LutPHFDouble::build_double(lambda, &pair_data, threaded, cutoff))
                })
        });
        lut_phf_double.map_err(|e| Box::new(e) as Box<dyn std::error::Error>)
    }
}

impl LutPHFDouble {
    #[inline]
    #[must_use]
    pub fn build_double(lambda: usize, pair_data: &PairData, threaded: bool, cutoff: usize) -> Self {
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
            hash_state: hash_state_16,
            hash_state_64,
            cutoff,
        }
    }
}
