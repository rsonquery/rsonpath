pub mod count_distances;
pub mod lut_distance;
pub mod lut_naive;
pub mod lut_perfect_naive;
pub mod lut_phf;
pub mod lut_phf_double;
pub mod lut_phf_group;
pub mod pair_finder;
pub mod performance;
pub mod util_path;

pub trait LookUpTable {
    fn build(json_path: &str) -> Result<Self, Box<dyn std::error::Error>>
    where
        Self: Sized;

    fn get(&self, key: &usize) -> Option<usize>;
}
