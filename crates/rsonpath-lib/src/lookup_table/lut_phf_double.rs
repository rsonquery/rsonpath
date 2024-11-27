use super::{
    lut_hash_map_double::PairData,
    lut_phf::{
        phf_generator_double_hash::{self, HashState},
        DEFAULT_LAMBDA, DEFAULT_THREADED,
    },
    LookUpTable, LookUpTableLambda,
};
use crate::{
    classification::{self, simd::Simd},
    input::{self, error, Input},
    lookup_table::lut_hash_map_double::LutHashMapDouble,
};
use std::{collections::HashMap, fs};

pub struct LutPHFDouble {
    pub lambda: usize,
    pub hash_state_16: HashState<u16>,
    pub hash_state_64: HashState<usize>,
}

impl LookUpTable for LutPHFDouble {
    #[inline]
    fn build(json_path: &str) -> Result<Self, Box<dyn std::error::Error>> {
        LutPHFDouble::build_with_lambda(DEFAULT_LAMBDA, json_path, DEFAULT_THREADED)
    }

    #[inline]
    fn get(&self, key: &usize) -> Option<usize> {
        // Look for a value for a given key. Search first in `hash_state_16` because it represents
        // >99% of the keys. On rare misses (indicated by value == 0), check `hash_state_64`. If the hash_state
        // None the key was never part of the original key set at build time
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
    fn build_with_lambda(lambda: usize, json_path: &str, threaded: bool) -> Result<Self, Box<dyn std::error::Error>> {
        let file = fs::File::open(json_path).expect("Failed to open file");
        // SAFETY: We keep the file open throughout the entire duration.
        let input = unsafe { input::MmapInput::map_file(&file)? };
        let simd_c = classification::simd::configure();

        let lut_phf_double = classification::simd::config_simd!(simd_c => |simd| {
            classification::simd::dispatch_simd!(simd; input, simd, lambda, threaded => fn<I, V>(
                input: I,
                simd: V,
                lambda: usize,
                threaded: bool,
            ) -> Result<LutPHFDouble, error::InputError> where
            I: Input,
            V: Simd, {
                    let pair_data = LutHashMapDouble::find_all_pairs::<I, V>(&input, simd)?;
                    Ok(LutPHFDouble::build_double(lambda, pair_data, threaded))
                })
        });
        lut_phf_double.map_err(|e| Box::new(e) as Box<dyn std::error::Error>)
    }
}

impl LutPHFDouble {
    pub fn build_double(lambda: usize, pair_data: PairData, threaded: bool) -> Self {
        // 1) Build hash_state for the values of size u16
        let hash_state_16_usize = phf_generator_double_hash::build(lambda, &pair_data.keys_16, threaded);

        // 2) Find conflicts and set conflict positions to 0 in `values_16`, save (index, value) in conflict_indexes
        // We set the conflict position to 0, because the distance = 0 never naturally appears. This is because the
        // distance between { and } in "{}" is always 1. So 0 is the only value we can use as conflict indicator.
        let mut conflict_indexes: HashMap<usize, u16> = HashMap::with_capacity(pair_data.keys_64.len());
        let mut values_16 = pair_data.values_16.clone();
        for key_64 in &pair_data.keys_64 {
            let idx = hash_state_16_usize
                .get(key_64)
                .expect("Fail @ get idx from hash_state_16");

            conflict_indexes.insert(idx, values_16[idx]);
            // Mark conflict position with 0
            values_16[idx] = 0;
        }

        // 3) Init conflict_keys and conflict_values
        let mut conflict_keys: Vec<usize> = pair_data.keys_64.clone();
        let mut conflict_values: Vec<usize> = pair_data.values_64.clone();

        // 4) Collect all conflict keys and values from keys_16
        for key_16 in &pair_data.keys_16 {
            let idx = hash_state_16_usize
                .get(key_16)
                .expect("Fail @ get idx from hash_state_16");
            let value_16 = values_16[idx];

            if value_16 == 0 {
                // Found conflict
                conflict_keys.push(*key_16);
                conflict_values.push(conflict_indexes[&idx].into());
            }
        }

        // 5) Build hash_state_64
        let mut hash_state_64 = phf_generator_double_hash::build(lambda, &conflict_keys, threaded);
        // Replace indexes with values, since otherwise we would map to a index position and not the actual value
        hash_state_64.map = hash_state_64.map.into_iter().map(|idx| conflict_values[idx]).collect();

        // 6) Replace indexes with actual values from values_16
        let map_16: Vec<u16> = hash_state_16_usize.map.into_iter().map(|idx| values_16[idx]).collect();
        let hash_state_16: HashState<u16> = HashState {
            lambda,
            hash_key: hash_state_16_usize.hash_key,
            displacements: hash_state_16_usize.displacements,
            map: map_16,
        };

        Self {
            lambda,
            hash_state_16,
            hash_state_64,
        }
    }
}
