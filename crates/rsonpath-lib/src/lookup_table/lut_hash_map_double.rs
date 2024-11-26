use super::{lut_phf_double::PairData, LookUpTable};
use crate::{
    classification::{self, simd::Simd},
    input::{self, error, Input},
    lookup_table::lut_phf_double::LutPHFDouble,
};
use std::{collections::HashMap, fs};

pub struct LutHashMapDouble {
    pub hash_map_16: HashMap<usize, u16>,
    pub hash_map_64: HashMap<usize, usize>,
}

impl LookUpTable for LutHashMapDouble {
    #[inline]
    fn build(json_path: &str) -> Result<Self, Box<dyn std::error::Error>> {
        let file = fs::File::open(json_path).expect("Failed to open file");
        // SAFETY: We keep the file open throughout the entire duration.
        let input = unsafe { input::MmapInput::map_file(&file)? };
        let simd_c = classification::simd::configure();

        let lut_phf_double = classification::simd::config_simd!(simd_c => |simd| {
            classification::simd::dispatch_simd!(simd; input, simd => fn<I, V>(
                input: I,
                simd: V,
            ) -> Result<LutHashMapDouble, error::InputError> where
            I: Input,
            V: Simd, {
                    let pair_data = LutPHFDouble::find_all_pairs::<I, V>(&input, simd)?;
                    Ok(LutHashMapDouble::build_double(pair_data))
                })
        });
        lut_phf_double.map_err(|e| Box::new(e) as Box<dyn std::error::Error>)
    }

    #[inline]
    fn get(&self, key: &usize) -> Option<usize> {
        // Look for a value for a given key, search first in hash_map_16 since it represents usually >99% of the keys
        if let Some(&value_16) = self.hash_map_16.get(key) {
            // key in hash_map_16
            Some(*key + value_16 as usize)
        } else if let Some(&value_64) = self.hash_map_64.get(key) {
            // key in hash_map_64
            Some(*key + value_64)
        } else {
            // Neither map contains the key which should never happen because we added all keys and values at build
            None
        }
    }

    #[inline]
    fn allocated_bytes(&self) -> usize {
        let mut total_size = 0;

        total_size += std::mem::size_of::<Self>();
        total_size += self.hash_map_16.capacity() * (std::mem::size_of::<usize>() + std::mem::size_of::<u16>());
        total_size += self.hash_map_64.capacity() * (std::mem::size_of::<usize>() + std::mem::size_of::<usize>());

        total_size
    }
}

impl LutHashMapDouble {
    pub fn build_double(pd: PairData) -> Self {
        let hash_map_16: HashMap<usize, u16> = pd.keys_16.into_iter().zip(pd.values_16.into_iter()).collect();
        let hash_map_64: HashMap<usize, usize> = pd.keys_64.into_iter().zip(pd.values_64.into_iter()).collect();

        Self {
            hash_map_16,
            hash_map_64,
        }
    }
}
