use super::{lut_hash_map_double::LutHashMapDouble, pair_data::PairData, LookUpTable};
use crate::{
    classification::{self, simd::Simd},
    input::{self, error, Input},
    lookup_table::pair_data,
    FallibleIterator,
};
use rayon::prelude::*;
use std::fs;

// A bit map that only keeps the lower 4 bit because we currently have 16 lut in the group. 16 is represented by 4 bits.
const DEFAULT_BIT_MASK: usize = 0xF;

pub struct LutHashMapGroup {
    pub lut_doubles: Vec<LutHashMapDouble>,
    pub bit_mask: usize,
    pub cutoff: usize,
}

impl LookUpTable for LutHashMapGroup {
    #[inline]
    fn build(json_path: &str, cutoff: usize) -> Result<Self, Box<dyn std::error::Error>> {
        Self::build_buckets(json_path, DEFAULT_BIT_MASK, cutoff)
    }

    #[inline]
    fn get(&self, key: &usize) -> Option<usize> {
        // Logical AND with BIT_MASK to get the correct index
        self.lut_doubles[key & self.bit_mask].get(key)
    }

    fn get_cutoff(&self) -> usize {
        self.cutoff
    }
}

impl LutHashMapGroup {
    #[inline]
    pub fn build_buckets(json_path: &str, bit_mask: usize, cutoff: usize) -> Result<Self, Box<dyn std::error::Error>> {
        let file = fs::File::open(json_path).expect("Failed to open file");
        // SAFETY: We keep the file open throughout the entire duration.
        let input = unsafe { input::MmapInput::map_file(&file)? };
        let simd_c = classification::simd::configure();

        let lut_perfect_naive = classification::simd::config_simd!(simd_c => |simd| {
            classification::simd::dispatch_simd!(simd; input, simd, bit_mask, cutoff => fn<I, V>(
                input: I,
                simd: V,
                bit_mask: usize,
                cutoff: usize,
            ) -> Result<LutHashMapGroup, error::InputError> where
            I: Input,
            V: Simd,{
                let pair_data_buckets = pair_data::find_pairs_buckets::<I, V>(&input, simd, bit_mask, cutoff)?;
                Ok(LutHashMapGroup::build_lut_doubles(pair_data_buckets, bit_mask, cutoff))
            })
        });
        lut_perfect_naive.map_err(|e| Box::new(e) as Box<dyn std::error::Error>)
    }

    fn build_lut_doubles(pair_data_buckets: Vec<PairData>, bit_mask: usize, cutoff: usize) -> Self {
        let lut_doubles: Vec<LutHashMapDouble> = pair_data_buckets
            .into_par_iter()
            .map(|pair_data| LutHashMapDouble::build_double(pair_data, cutoff))
            .collect();

        Self {
            lut_doubles,
            bit_mask,
            cutoff,
        }
    }
}
