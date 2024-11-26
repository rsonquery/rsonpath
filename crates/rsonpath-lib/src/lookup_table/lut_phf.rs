use super::{LookUpTable, LookUpTableLambda};
use crate::{
    classification::{self, simd::Simd},
    input::{self, error, Input},
    lookup_table::lut_hash_map::LutHashMap,
};
use phf_generator_double_hash::HashState;
use std::fs;

pub mod phf_generator;
pub mod phf_generator_double_hash;
pub mod phf_shared;

pub const BUILD_LAMBDA: usize = 1; // Range = [1, ... , 5]
pub const MAX_LAMBDA: usize = 5; // 5 because the source paper did so

pub struct LutPHF {
    pub hash_state: HashState<usize>,
    pub values: Vec<usize>,
}

impl LookUpTable for LutPHF {
    #[inline]
    fn build(json_path: &str) -> Result<Self, Box<dyn std::error::Error>> {
        LutPHF::build_with_lambda(BUILD_LAMBDA, json_path)
    }

    #[inline]
    #[must_use]
    fn get(&self, key: &usize) -> Option<usize> {
        self.hash_state
            .get(key)
            .and_then(|index| self.values.get(index).map(|&value| key + value))
    }

    #[inline]
    fn allocated_bytes(&self) -> usize {
        let mut total_size = std::mem::size_of::<Self>();
        total_size += self.hash_state.allocated_bytes();
        total_size += self.values.capacity() * std::mem::size_of::<usize>();
        total_size
    }
}

impl LookUpTableLambda for LutPHF {
    #[inline]
    fn build_with_lambda(lambda: usize, json_path: &str) -> Result<Self, Box<dyn std::error::Error>> {
        let file = fs::File::open(json_path).expect("Failed to open file");
        // SAFETY: We keep the file open throughout the entire duration.
        let input = unsafe { input::MmapInput::map_file(&file)? };
        let simd_c = classification::simd::configure();

        let lut_phf_double = classification::simd::config_simd!(simd_c => |simd| {
            classification::simd::dispatch_simd!(simd; input, simd, lambda => fn<I, V>(
                input: I,
                simd: V,
                lambda: usize,
            ) -> Result<LutPHF, error::InputError> where
            I: Input,
            V: Simd, {
                    let (keys, values) = LutHashMap::find_all_pairs::<I, V>(&input, simd)?;
                    let hash_state = phf_generator_double_hash::build(lambda, &keys);
                    Ok(LutPHF { hash_state, values })
                })
        });
        lut_phf_double.map_err(|e| Box::new(e) as Box<dyn std::error::Error>)
    }
}
