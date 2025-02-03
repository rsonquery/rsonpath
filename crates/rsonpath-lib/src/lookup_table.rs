pub mod count_distances;
pub mod lut_hash_map;
pub mod lut_hash_map_double;
pub mod lut_hash_map_group;
pub mod lut_perfect_naive;
pub mod lut_phf;
pub mod lut_phf_double;
pub mod lut_phf_group;
pub mod packed_stacked_frame;
pub mod pair_finder;
pub mod performance;
pub mod query_with_lut;
pub mod sichash_test_data_generator;
pub mod util_path;

pub type LookUpTableImpl = lut_hash_map::LutHashMap;

/// Throughout this project the abbreviation for LookUpTable will be LUT or lut
pub trait LookUpTable {
    fn build(json_path: &str) -> Result<Self, Box<dyn std::error::Error>>
    where
        Self: Sized;

    fn get(&self, key: &usize) -> Option<usize>;

    fn allocated_bytes(&self) -> usize;
}

pub trait LookUpTableLambda: LookUpTable {
    fn build_lambda(lambda: usize, json_path: &str, threaded: bool) -> Result<Self, Box<dyn std::error::Error>>
    where
        Self: Sized;
}
