use log::debug;
use ptr_hash::{PtrHash, PtrHashParams};

use std::fs;

use super::{pair_data::PairData, LookUpTable};
use crate::{
    classification::{self, simd::Simd},
    input::{self, error, Input},
    lookup_table::pair_data,
};

pub struct LutPtrHashDouble {
    ptr_hash: PtrHash<usize>,
    values: Vec<u16>,
    ptr_hash_64: PtrHash<usize>,
    values_64: Vec<usize>,
    cutoff: usize,
}

impl LookUpTable for LutPtrHashDouble {
    fn build(json_path: &str, cutoff: usize) -> Result<Self, Box<dyn std::error::Error>>
    where
        Self: Sized,
    {
        let file = fs::File::open(json_path).expect("Failed to open file");
        // SAFETY: We keep the file open throughout the entire duration.
        let input = unsafe { input::MmapInput::map_file(&file)? };
        let simd_c = classification::simd::configure();

        let lut_phf_double = classification::simd::config_simd!(simd_c => |simd| {
            classification::simd::dispatch_simd!(simd; input, simd, cutoff => fn<I, V>(
                input: I,
                simd: V,
                cutoff: usize,
            ) -> Result<LutPtrHashDouble, error::InputError> where
            I: Input,
            V: Simd, {
                    // let start_search = std::time::Instant::now();
                    // let pair_data = pair_data::find_pairs(&input, simd, cutoff)?;
                    // let search_time = start_search.elapsed().as_secs_f64();
                    // println!("    - Search time:     {search_time}");
                    // Ok(LutPtrHashDouble::build_double(pair_data, cutoff))

                    let pair_data = pair_data::find_pairs(&input, simd, cutoff)?;
                    Ok(LutPtrHashDouble::build_double(pair_data, cutoff))
                })
        });
        lut_phf_double.map_err(|e| Box::new(e) as Box<dyn std::error::Error>)
    }

    fn get(&self, key: &usize) -> Option<usize> {
        debug!("PtrHash: Call get({})", key);

        let mut dst: usize = self.values[self.ptr_hash.index(key)] as usize;
        // Check ptr_hash and if that returns 0 then check ptr_hash_64
        if dst == 0 {
            dst = self.values_64[self.ptr_hash_64.index(key)];
        }

        Some(*key + dst)
    }

    fn allocated_bytes(&self) -> usize {
        0
    }

    fn get_cutoff(&self) -> usize {
        self.cutoff
    }
}

impl LutPtrHashDouble {
    fn build_double(pair_data: PairData, cutoff: usize) -> Self {
        let keys = pair_data.keys;
        let input_values = pair_data.values;

        // Build minimal perfect hash function (mphf)
        let ptr_hash = <PtrHash<usize>>::new(&keys, PtrHashParams::default());
        // Sort values depending on the new mphf
        let mut values: Vec<u16> = vec![0; input_values.len()];
        for (i, key) in keys.iter().enumerate() {
            values[ptr_hash.index(key)] = input_values[i];
        }

        // Do the same for usize
        let keys_64 = pair_data.keys_64;
        let input_values_64 = pair_data.values_64;

        let ptr_hash_64 = <PtrHash<usize>>::new(&keys_64, PtrHashParams::default());
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
            cutoff,
        }
    }
}
