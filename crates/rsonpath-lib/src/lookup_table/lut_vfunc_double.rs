use std::fs;

use dsi_progress_logger::no_logging;
use sux::{
    func::{VBuilder, VFunc},
    utils::FromIntoIterator,
};

use crate::{
    classification::{self, simd::Simd},
    input::{self, error, Input},
    lookup_table::pair_data,
};

use super::{pair_data::PairData, LookUpTable};

pub struct LutVFuncDouble {
    vfunc: VFunc<usize, u16>,
    vfunc_64: VFunc<usize, usize>,
    cutoff: usize,
}

impl LookUpTable for LutVFuncDouble {
    fn build(json_path: &str, cutoff: usize) -> Result<Self, Box<dyn std::error::Error>>
    where
        Self: Sized,
    {
        let file = fs::File::open(json_path).expect("Failed to open file");
        // SAFETY: We keep the file open throughout the entire duration.
        let input = unsafe { input::MmapInput::map_file(&file)? };
        let simd_c = classification::simd::configure();

        let lut_vfunc_double = classification::simd::config_simd!(simd_c => |simd| {
            classification::simd::dispatch_simd!(simd; input, simd, cutoff => fn<I, V>(
                input: I,
                simd: V,
                cutoff: usize,
            ) -> Result<LutVFuncDouble, error::InputError> where
            I: Input,
            V: Simd, {
                    let start_search = std::time::Instant::now();
                    let pair_data = pair_data::find_pairs(&input, simd, cutoff)?;
                    let search_time = start_search.elapsed().as_secs_f64();
                    println!("    - Search time:      {search_time}");
                    Ok(LutVFuncDouble::build_double(pair_data, cutoff))

                    // let pair_data = pair_data::find_pairs(&input, simd, cutoff)?;
                    // Ok(LutVFuncDouble::build_double(pair_data, cutoff))
                })
        });
        lut_vfunc_double.map_err(|e| Box::new(e) as Box<dyn std::error::Error>)
    }

    fn get(&self, key: &usize) -> Option<usize> {
        let mut result = self.vfunc.get(key) as usize;
        if result == 0 {
            result = self.vfunc_64.get(key);
        }

        return Some(key + result);
    }

    fn allocated_bytes(&self) -> usize {
        // todo!()
        0
    }

    fn get_cutoff(&self) -> usize {
        self.cutoff
    }
}

impl LutVFuncDouble {
    fn build_double(pair_data: PairData, cutoff: usize) -> Self {
        let keys = pair_data.keys;
        let values = pair_data.values;
        let keys_64 = pair_data.keys_64;
        let values_64 = pair_data.values_64;

        // vFunc
        let builder = VBuilder::<u16, Box<[u16]>>::default().expected_num_keys(keys.len());
        let vfunc = builder
            .try_build_func(
                FromIntoIterator::from(keys),
                FromIntoIterator::from(values),
                no_logging![],
            )
            .expect("Some build error");

        // vFunc_64
        let builder_64 = VBuilder::<usize, Box<[usize]>>::default().expected_num_keys(keys_64.len());
        let vfunc_64 = builder_64
            .try_build_func(
                FromIntoIterator::from(keys_64),
                FromIntoIterator::from(values_64),
                no_logging![],
            )
            .expect("Some build error");

        Self {
            vfunc,
            vfunc_64,
            cutoff,
        }
    }
}
