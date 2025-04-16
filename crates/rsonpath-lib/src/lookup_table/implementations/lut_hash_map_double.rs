use crate::lookup_table::pair_data::PairData;
use crate::lookup_table::LookUpTable;
use crate::{
    classification::{self, simd::Simd},
    input::{self, error, Input},
    lookup_table::pair_data,
};
use std::{collections::HashMap, fs};

pub struct LutHashMapDouble {
    pub hash_map: HashMap<usize, u16>,
    pub hash_map_64: HashMap<usize, usize>,
    pub cutoff: usize,
}

impl LookUpTable for LutHashMapDouble {
    #[inline]
    fn build(json_path: &str, cutoff: usize) -> Result<Self, Box<dyn std::error::Error>> {
        let file = fs::File::open(json_path).expect("Failed to open file");
        // SAFETY: We keep the file open throughout the entire duration.
        let input = unsafe { input::MmapInput::map_file(&file)? };
        let simd_c = classification::simd::configure();

        let lut_phf_double = classification::simd::config_simd!(simd_c => |simd| {
            classification::simd::dispatch_simd!(simd; input, simd, cutoff => fn<I, V>(
                input: I,
                simd: V,
                cutoff: usize,
            ) -> Result<LutHashMapDouble, error::InputError> where
            I: Input,
            V: Simd, {
                    let pair_data = pair_data::find_pairs::<I, V>(&input, simd, cutoff)?;
                    Ok(LutHashMapDouble::build_double(pair_data, cutoff))
                })
        });
        lut_phf_double.map_err(|e| Box::new(e) as Box<dyn std::error::Error>)
    }

    #[inline]
    fn get(&self, key: &usize) -> Option<usize> {
        // Look for a value for a given key, search first in hash_map_16 since it represents usually >99% of the keys
        // and only on the rare cases of key misses we have to look into hash_map_64 which covers the rest.
        if let Some(&value_16) = self.hash_map.get(key) {
            if value_16 == 0 {
                if let Some(&value_64) = self.hash_map_64.get(key) {
                    Some(*key + value_64)
                } else {
                    println!("Should never happen");
                    None
                }
            } else {
                Some(*key + value_16 as usize)
            }
        } else {
            // Neither map contains the key which should never happen because we added all keys and values at build
            println!("Ups! You asked for a key that is not in the LutHashMap. Key = {}", key);
            None
        }
    }

    fn get_cutoff(&self) -> usize {
        self.cutoff
    }
}

impl LutHashMapDouble {
    #[inline]
    #[must_use]
    pub fn build_double(pd: PairData, cutoff: usize) -> Self {
        let hash_map: HashMap<usize, u16> = pd.keys.into_iter().zip(pd.values).collect();
        let hash_map_64: HashMap<usize, usize> = pd.keys_64.into_iter().zip(pd.values_64).collect();

        Self {
            hash_map,
            hash_map_64,
            cutoff,
        }
    }
}
