use crate::lookup_table::implementations::lut_hash_map::LutHashMap;
use crate::lookup_table::implementations::lut_hash_map_double::LutHashMapDouble;
use crate::lookup_table::implementations::lut_phf_group::LutPHFGroup;
use crate::lookup_table::implementations::lut_ptr_hash_double::LutPtrHashDouble;
use crate::lookup_table::implementations::lut_vfunc_double::LutVFuncDouble;
use crate::lookup_table::performance::lut_skip_evaluation::SkipMode;

pub mod analysis;
pub mod implementations;
pub mod packed_stacked_frame;
pub mod pair_data;
pub mod performance;
pub mod pokemon_test_data_generator;
pub mod query_with_lut;
pub mod sichash_test_data_generator;
pub mod util_path;

// CONFIG
pub const USE_SKIP_ABORT_STRATEGY: bool = false;
pub const SKIP_MODE: SkipMode = SkipMode::OFF;
pub const TRACK_SKIPPING_TIME_DURING_PERFORMANCE_TEST: bool = true;
pub const REPETITIONS: u64 = 1;
pub const QUERY_REPETITIONS: usize = 1;

// pub type LUT = LutHashMap;
// pub type LUT = LutHashMapDouble;
// pub type LUT = LutHashMapGroup;
// pub type LUT = LutPHF;
// pub type LUT = LutPHFDouble;
// pub type LUT = LutPHFGroup;
// pub type LUT = LutSicHash;
pub type LUT = LutPtrHashDouble;
// pub type LUT = LutVFuncDouble;

/// Lookup-table = LUT
pub trait LookUpTable {
    fn build(json_path: &str, cutoff: usize) -> Result<Self, Box<dyn std::error::Error>>
    where
        Self: Sized;

    fn get(&self, key: &usize) -> Option<usize>;

    fn get_cutoff(&self) -> usize;
}

pub trait LookUpTableLambda: LookUpTable {
    fn build_lambda(
        lambda: usize,
        json_path: &str,
        cutoff: usize,
        threaded: bool,
    ) -> Result<Self, Box<dyn std::error::Error>>
    where
        Self: Sized;
}

// TODO if time? fn build(json_path: &str) -> Result<Self, Box<dyn std::error::Error + Sync + Send>>
// TODO if time? Consider builder pattern
