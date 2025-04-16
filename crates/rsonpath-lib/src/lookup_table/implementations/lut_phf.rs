use crate::lookup_table::{LookUpTable, LookUpTableLambda};
use crate::{
    classification::{self, simd::Simd},
    input::{self, error, Input},
    lookup_table::pair_data,
};
use phf_generator_double_hash::HashState;
use std::fs;

pub mod phf_generator;
pub mod phf_generator_double_hash;
pub mod phf_shared;

pub const DEFAULT_LAMBDA: usize = 1; // Range = [1, ... , 5]
pub const DEFAULT_THREADED: bool = false;
pub const MAX_LAMBDA: usize = 5; // 5 because the source paper did so

pub struct LutPHF {
    pub hash_state: HashState<usize>,
    pub values: Vec<usize>,
    pub cutoff: usize,
}

impl LookUpTable for LutPHF {
    #[inline]
    fn build(json_path: &str, cutoff: usize) -> Result<Self, Box<dyn std::error::Error>> {
        Self::build_lambda(DEFAULT_LAMBDA, json_path, cutoff, DEFAULT_THREADED)
    }

    #[inline]
    #[must_use]
    fn get(&self, key: &usize) -> Option<usize> {
        self.hash_state
            .get(key)
            .and_then(|index| self.values.get(index))
            .copied()
    }

    fn get_cutoff(&self) -> usize {
        self.cutoff
    }
}

impl LookUpTableLambda for LutPHF {
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
            ) -> Result<LutPHF, error::InputError> where
            I: Input,
            V: Simd, {
                    let (keys, values) = pair_data::find_pairs_absolute::<I, V>(&input, simd, cutoff)?;
                    let hash_state = phf_generator_double_hash::build(lambda, &keys, threaded);
                    Ok(LutPHF { hash_state, values, cutoff })
                })
        });
        lut_phf_double.map_err(|e| Box::new(e) as Box<dyn std::error::Error>)
    }
}
