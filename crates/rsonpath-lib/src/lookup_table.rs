pub mod distance_counter;
pub mod lut_hash_map;
pub mod lut_hash_map_double;
pub mod lut_hash_map_group;
pub mod lut_perfect_naive;
pub mod lut_phf;
pub mod lut_phf_double;
pub mod lut_phf_group;
pub mod lut_sichash;
pub mod packed_stacked_frame;
pub mod pair_finder;
pub mod performance;
pub mod pokemon_test_data_generator;
pub mod query_with_lut;
pub mod sichash_test_data_generator;
pub mod util_path;

pub type LUT = lut_hash_map::LutHashMap;

// Cannot work until PackedStackFrame logic is implemented since it cannot return None on untrained keys
// pub type LUT = lut_sichash::LutSicHash;

/// Lookup-table = LUT
pub trait LookUpTable {
    fn build(json_path: &str, distance_cutoff: usize) -> Result<Self, Box<dyn std::error::Error>>
    // TODO if time? fn build(json_path: &str) -> Result<Self, Box<dyn std::error::Error + Sync + Send>>
    where
        Self: Sized;

    fn get(&self, key: &usize) -> Option<usize>;

    fn allocated_bytes(&self) -> usize;
}

pub trait LookUpTableLambda: LookUpTable {
    fn build_lambda(
        lambda: usize,
        json_path: &str,
        distance_cutoff: usize,
        threaded: bool,
    ) -> Result<Self, Box<dyn std::error::Error>>
    where
        Self: Sized;
}

// pub struct LookUpTableBuilder {
//     json_path: Option<String>,
//     distance_cutoff: Option<usize>,
//     lambda: Option<usize>,
//     threaded: bool,
// }

// impl LookUpTableBuilder {
//     pub fn new() -> Self {
//         LookUpTableBuilder {
//             json_path: None,
//             distance_cutoff: Some(0),
//             lambda: None,
//             threaded: false,
//         }
//     }

//     pub fn json_path(mut self, path: &str) -> Self {
//         self.json_path = Some(path.to_string());
//         self
//     }

//     pub fn distance_cutoff(mut self, cutoff: usize) -> Self {
//         self.distance_cutoff = Some(cutoff);
//         self
//     }

//     pub fn lambda(mut self, lambda: usize) -> Self {
//         self.lambda = Some(lambda);
//         self
//     }

//     pub fn threaded(mut self, threaded: bool) -> Self {
//         self.threaded = threaded;
//         self
//     }

//     pub fn build<T: LookUpTable>(self) -> Result<T, Box<dyn std::error::Error>> {
//         let json_path = self
//             .json_path
//             .ok_or("Error: `json_path` must be set before calling `build`")?;
//         let distance_cutoff = self
//             .distance_cutoff
//             .ok_or("Error: `distance_cutoff` must be set before calling `build`")?;

//         if json_path.is_empty() {
//             return Err("Error: `json_path` cannot be empty".into());
//         }
//         if distance_cutoff == 0 {
//             return Err("Error: `distance_cutoff` must be greater than 0".into());
//         }

//         T::build(&json_path, distance_cutoff)
//     }

//     // Example usage:
//     // let lut_lambda = LookUpTableBuilder::new()
//     // .json_path("data.json")
//     // .distance_cutoff(5)
//     // .lambda(42)
//     // .threaded(true)
//     // .build_lambda::<LookUpTableImpl>()?;
//     pub fn build_lambda<T: LookUpTableLambda>(self) -> Result<T, Box<dyn std::error::Error>> {
//         let json_path = self
//             .json_path
//             .ok_or("Error: `json_path` must be set before calling `build_lambda`")?;
//         let distance_cutoff = self
//             .distance_cutoff
//             .ok_or("Error: `distance_cutoff` must be set before calling `build_lambda`")?;
//         let lambda = self
//             .lambda
//             .ok_or("Error: `lambda` must be set before calling `build_lambda`")?;

//         if json_path.is_empty() {
//             return Err("Error: `json_path` cannot be empty".into());
//         }
//         if distance_cutoff < 0 {
//             return Err("Error: `distance_cutoff` must be greater than 0".into());
//         }
//         if lambda == 0 {
//             return Err("Error: `lambda` must be greater than 0".into());
//         }

//         T::build_lambda(lambda, &json_path, distance_cutoff, self.threaded)
//     }
// }
