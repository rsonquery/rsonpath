use super::LookUpTable;
use crate::{
    classification::{self, simd::Simd},
    input::{self, error, Input},
    lookup_table::pair_data,
};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::fs;

#[derive(Serialize, Deserialize, Debug)]
pub struct LutHashMap {
    hash_map: HashMap<usize, usize>,
    cutoff: usize,
}
impl LookUpTable for LutHashMap {
    #[inline]
    fn build(json_path: &str, cutoff: usize) -> Result<Self, Box<dyn std::error::Error>> {
        let file = fs::File::open(json_path).expect("Failed to open file");
        // SAFETY: We keep the file open throughout the entire duration.
        let input = unsafe { input::MmapInput::map_file(&file)? };
        let simd_c = classification::simd::configure();

        classification::simd::config_simd!(simd_c => |simd| {
            classification::simd::dispatch_simd!(simd; input, simd, cutoff => fn<I, V>(
                input: I,
                simd: V,
                cutoff: usize,
            ) -> Result<LutHashMap, error::InputError> where
            I: Input,
            V: Simd,{
                    let (keys, values) = pair_data::find_pairs_absolute::<I, V>(&input, simd, cutoff)?;
                    let hash_map: HashMap<usize, usize> = keys.into_iter().zip(values.into_iter()).collect();
                    Ok(LutHashMap{ hash_map, cutoff })
                })
        })
        .map_err(|e| Box::new(e) as Box<dyn std::error::Error>)
    }

    #[inline]
    #[must_use]
    fn get(&self, key: &usize) -> Option<usize> {
        self.hash_map.get(key).copied()
    }

    fn get_cutoff(&self) -> usize {
        self.cutoff
    }

    #[inline]
    fn allocated_bytes(&self) -> usize {
        let mut total_size = 0;
        total_size += std::mem::size_of::<Self>();
        total_size += self.hash_map.capacity() * (std::mem::size_of::<usize>() + std::mem::size_of::<usize>());
        total_size
    }
}
