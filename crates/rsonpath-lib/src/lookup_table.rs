pub mod distance_counter;
pub mod lut_hash_map;
pub mod lut_hash_map_double;
pub mod lut_hash_map_group;
pub mod lut_perfect_naive;
pub mod lut_phf;
pub mod lut_phf_double;
pub mod lut_phf_group;
pub mod lut_ptr_hash_double;
pub mod lut_sichash;
pub mod lut_vfunc_double;
pub mod packed_stacked_frame;
pub mod pair_data;
pub mod performance;
pub mod pokemon_test_data_generator;
pub mod query_with_lut;
pub mod sichash_test_data_generator;
pub mod util_path;

pub const DISTANCE_CUT_OFF: usize = 0;
pub const USE_SKIP_ABORT_STRATEGY: bool = true;

// pub type LUT = lut_hash_map::LutHashMap;
// pub type LUT = lut_hash_map_double::LutHashMapDouble;
// pub type LUT = lut_sichash::LutSicHash;
pub type LUT = lut_ptr_hash_double::LutPtrHashDouble;
// pub type LUT = lut_vfunc_double::LutVFuncDouble;

/// Lookup-table = LUT
pub trait LookUpTable {
    fn build(json_path: &str, cutoff: usize) -> Result<Self, Box<dyn std::error::Error>>
    where
        Self: Sized;

    fn get(&self, key: &usize) -> Option<usize>;

    fn get_cutoff(&self) -> usize;

    fn allocated_bytes(&self) -> usize;
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
