use super::lut_phf::DEFAULT_THREADED;
use super::{lut_phf::DEFAULT_LAMBDA, lut_phf_double::LutPHFDouble};
use crate::lookup_table::pair_data::PairData;
use crate::lookup_table::{pair_data, LookUpTable, LookUpTableLambda};
use crate::{
    classification::{self, simd::Simd},
    input::{self, error, Input},
};
use rayon::prelude::*;
use std::fs;

// A bit map that only keeps the lower 4 bit because we currently have 16 lut in the group. 16 is represented by 4 bits.
const DEFAULT_BIT_MASK: usize = 0xF;

pub struct LutPHFGroup {
    pub lut_doubles: Vec<LutPHFDouble>,
    pub bit_mask: usize,
    pub cutoff: usize,
}

impl LookUpTable for LutPHFGroup {
    #[inline]
    fn build(json_path: &str, cutoff: usize) -> Result<Self, Box<dyn std::error::Error>> {
        Self::build_buckets(DEFAULT_LAMBDA, json_path, cutoff, DEFAULT_BIT_MASK, DEFAULT_THREADED)
    }

    #[inline]
    fn get(&self, key: &usize) -> Option<usize> {
        // Logical AND with BIT_MASK to get the correct index
        let bucket = key & self.bit_mask;
        self.lut_doubles[bucket].get(key)
    }

    fn get_cutoff(&self) -> usize {
        self.cutoff
    }
}

impl LookUpTableLambda for LutPHFGroup {
    #[inline]
    fn build_lambda(
        lambda: usize,
        json_path: &str,
        cutoff: usize,
        threaded: bool,
    ) -> Result<Self, Box<dyn std::error::Error>> {
        Self::build_buckets(lambda, json_path, cutoff, DEFAULT_BIT_MASK, threaded)
    }
}

impl LutPHFGroup {
    #[inline]
    pub fn build_buckets(
        lambda: usize,
        json_path: &str,
        cutoff: usize,
        bit_mask: usize,
        threaded: bool,
    ) -> Result<Self, Box<dyn std::error::Error>> {
        let file = fs::File::open(json_path).expect("Failed to open file");
        // SAFETY: We keep the file open throughout the entire duration.
        let input = unsafe { input::MmapInput::map_file(&file)? };
        let simd_c = classification::simd::configure();

        let lut_phf_group = classification::simd::config_simd!(simd_c => |simd| {
            classification::simd::dispatch_simd!(simd; input, simd, lambda, bit_mask, cutoff, threaded => fn<I, V>(
                input: I,
                simd: V,
                lambda: usize,
                bit_mask: usize,
                cutoff: usize,
                threaded: bool,
            ) -> Result<LutPHFGroup, error::InputError> where
            I: Input,
            V: Simd,{
                let pair_data_buckets = pair_data::find_pairs_buckets::<I, V>(&input, simd, bit_mask, cutoff)?;
                Ok(LutPHFGroup::build_lut_doubles(lambda, pair_data_buckets, bit_mask, threaded, cutoff))
            })
        });
        lut_phf_group.map_err(|e| Box::new(e) as Box<dyn std::error::Error>)
    }

    fn build_lut_doubles(
        lambda: usize,
        pair_data_buckets: Vec<PairData>,
        bit_mask: usize,
        threaded: bool,
        cutoff: usize,
    ) -> Self {
        let lut_doubles: Vec<LutPHFDouble> = pair_data_buckets
            .into_par_iter()
            .map(|pair_data| LutPHFDouble::build_double(lambda, &pair_data, threaded, cutoff))
            .collect();

        Self {
            lut_doubles,
            bit_mask,
            cutoff,
        }
    }
}
