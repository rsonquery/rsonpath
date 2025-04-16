use crate::lookup_table::implementations::lut_hash_map::LutHashMap;
use crate::lookup_table::implementations::lut_hash_map_double::LutHashMapDouble;
use crate::lookup_table::implementations::lut_phf_group::LutPHFGroup;
use crate::lookup_table::implementations::lut_ptr_hash_double::LutPtrHashDouble;
use crate::lookup_table::implementations::lut_vfunc_double::LutVFuncDouble;

pub mod analysis;
pub mod implementations;
pub mod packed_stacked_frame;
pub mod pair_data;
pub mod performance;
pub mod pokemon_test_data_generator;
pub mod query_with_lut;
pub mod sichash_test_data_generator;
pub mod util_path;

pub const DISTANCE_CUT_OFF: usize = 0;
pub const USE_SKIP_ABORT_STRATEGY: bool = true;

// pub type LUT = LutHashMap;
// pub type LUT = LutHashMapDouble;
// pub type LUT = LutSicHash;
// pub type LUT = LutPHFGroup;
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
