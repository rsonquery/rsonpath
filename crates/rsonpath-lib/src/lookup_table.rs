pub mod count_distances;
pub mod lut_hash_map;
pub mod lut_hash_map_double;
pub mod lut_perfect_naive;
pub mod lut_phf;
pub mod lut_phf_double;
pub mod lut_phf_group;
pub mod pair_finder;
pub mod performance;
pub mod sichash_test_data_generator;
pub mod util_path;

pub trait LookUpTable {
    fn build(json_path: &str) -> Result<Self, Box<dyn std::error::Error>>
    where
        Self: Sized;

    fn get(&self, key: &usize) -> Option<usize>;

    fn allocated_bytes(&self) -> usize;
}

pub trait LookUpTableLambda: LookUpTable {
    fn build_with_lambda(lambda: usize, json_path: &str) -> Result<Self, Box<dyn std::error::Error>>
    where
        Self: Sized;
}
