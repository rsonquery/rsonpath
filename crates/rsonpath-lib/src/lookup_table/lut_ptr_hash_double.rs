use ptr_hash::{PtrHash, PtrHashParams};

use std::fs;

use super::{lut_phf_double::PairData, LookUpTable};
use crate::{
    classification::{self, simd::Simd},
    input::{self, error, Input},
    lookup_table::lut_phf_double::LutPHFDouble,
};

pub struct LutPtrHashDouble {
    ptr_hash: PtrHash,
    values: Vec<u16>,
    ptr_hash_64: PtrHash,
    values_64: Vec<usize>,
}

impl LookUpTable for LutPtrHashDouble {
    fn build(json_path: &str, distance_cutoff: usize) -> Result<Self, Box<dyn std::error::Error>>
    where
        Self: Sized,
    {
        let file = fs::File::open(json_path).expect("Failed to open file");
        // SAFETY: We keep the file open throughout the entire duration.
        let input = unsafe { input::MmapInput::map_file(&file)? };
        let simd_c = classification::simd::configure();

        let lut_phf_double = classification::simd::config_simd!(simd_c => |simd| {
            classification::simd::dispatch_simd!(simd; input, simd, distance_cutoff => fn<I, V>(
                input: I,
                simd: V,
                distance_cutoff: usize,
            ) -> Result<LutPtrHashDouble, error::InputError> where
            I: Input,
            V: Simd, {
                    // let start_search = std::time::Instant::now();
                    // let pair_data = LutPHFDouble::find_all_pairs(&input, simd, distance_cutoff)?;
                    // let search_time = start_search.elapsed().as_secs_f64();
                    // println!("    - Search time:      {search_time}");
                    // Ok(LutPtrHashDouble::build_double(pair_data))

                    let pair_data = LutPHFDouble::find_all_pairs(&input, simd, distance_cutoff)?;
                    Ok(LutPtrHashDouble::build_double(pair_data))
                })
        });
        lut_phf_double.map_err(|e| Box::new(e) as Box<dyn std::error::Error>)
    }

    fn get(&self, key: &usize) -> Option<usize> {
        let key_64 = (*key) as u64;
        let mut dst: usize = self.values[self.ptr_hash.index(&key_64)] as usize;
        // Check ptr_hash and if that returns 0 then check ptr_hash_64
        if dst == 0 {
            dst = self.values_64[self.ptr_hash_64.index(&key_64)];
        }

        Some(*key + dst)
    }

    fn allocated_bytes(&self) -> usize {
        0
    }
}

impl LutPtrHashDouble {
    fn build_double(pair_data: PairData) -> Self {
        let keys: Vec<u64> = pair_data.keys.iter().map(|&k| k as u64).collect();
        let input_values = pair_data.values;

        // Build minimal perfect hash function (mphf)
        let ptr_hash = <PtrHash>::new(&keys, PtrHashParams::default());
        // Sort values depending on the new mphf
        let mut values: Vec<u16> = vec![0; input_values.len()];
        for (i, key) in keys.iter().enumerate() {
            values[ptr_hash.index(key)] = input_values[i];
        }

        // Do the same for usize
        let keys_64: Vec<u64> = pair_data.keys_64.iter().map(|&k| k as u64).collect();
        let input_values_64 = pair_data.values_64;

        let ptr_hash_64 = <PtrHash>::new(&keys_64, PtrHashParams::default());
        let mut values_64: Vec<usize> = vec![0; input_values_64.len()];
        for (i, key) in keys_64.iter().enumerate() {
            let new_i = ptr_hash_64.index(key);
            values_64[new_i] = input_values_64[i];
        }

        Self {
            ptr_hash,
            values,
            ptr_hash_64,
            values_64,
        }
    }
}
