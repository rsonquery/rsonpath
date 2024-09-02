// use ph::fmph::{self, Function};
// use serde::{Deserialize, Serialize};
// use serde_cbor;
// use serde_json;
// use std::fs::File;
// use std::io::{Error, ErrorKind, Read, Write};

// use crate::lookup_table::util;

// #[derive(Serialize, Deserialize)]
// pub struct LutPerfectHashing {
//     hash_function: fmph::Function, // The minimal perfect hash function
//     values: Vec<usize>,            // The array storing the values
// }

// impl LutPerfectHashing {
//     #[inline]
//     pub fn init(keys: Vec<usize>, values: Vec<usize>) -> Self {
//         assert_eq!(keys.len(), values.len(), "Keys and values must have the same length.");

//         // Create the MPHF for the given keys
//         let hash_function = fmph::Function::from_slice(&keys).expect("Failed to create the MPHF.");

//         // The values array will be in the same order as the keys, since MPHF provides unique indices
//         Self {
//             hash_function,
//             values,
//         }
//     }

//     #[inline]
//     pub fn get(&self, key: &usize) -> Option<&usize> {
//         // Use the MPHF to get the index
//         let index = self.hash_function.get(key)?;

//         // Safely access the values array
//         self.values.get(index as usize)
//     }

//     #[inline]
//     pub fn serialize(&self, path: &str) -> std::io::Result<()> {
//         let serialized_data = match util::get_filetype_from_path(path).as_str() {
//             "json" => serde_json::to_vec(&self).expect("Serialize failed."),
//             "cbor" => serde_cbor::to_vec(&self).expect("Serialize failed."),
//             _ => return Err(Error::new(ErrorKind::InvalidInput, "Serialize: Unsupported format")),
//         };
//         let mut file = File::create(path)?;
//         file.write_all(&serialized_data)?;
//         Ok(())
//     }

//     #[inline]
//     pub fn deserialize(path: &str) -> std::io::Result<Self> {
//         let mut file = File::open(path)?;
//         let mut contents = Vec::new();
//         file.read_to_end(&mut contents)?;
//         let deserialized: Self = match util::get_filetype_from_path(path).as_str() {
//             "json" => serde_json::from_slice(&contents).expect("Deserialize: Data has no JSON format."),
//             "cbor" => serde_cbor::from_slice(&contents).expect("Deserialize: Data has no CBOR format."),
//             _ => return Err(Error::new(ErrorKind::InvalidInput, "Deserialize: Unsupported format")),
//         };
//         Ok(deserialized)
//     }

//     #[inline]
//     pub fn overview(&self) {
//         if !self.values.is_empty() {
//             println!("lut-perfect-hashing Overview:");
//             println!("  #Entries: {}", self.values.len());
//             println!("  CBOR: {} bytes", self.estimate_cbor_size());
//             println!("  JSON: {} bytes", self.estimate_json_size());

//             // Print up to the first 10 pairs
//             println!("  First 10 pairs:");
//             for (i, value) in self.values.iter().enumerate().take(10) {
//                 println!("    {}. Index: {}, Value: {}", i + 1, i, value);
//             }
//         } else {
//             println!("The table is empty.");
//         }
//     }

//     // Returns number of bytes or 0 when table is empty
//     #[inline]
//     pub fn estimate_json_size(&self) -> usize {
//         if !self.values.is_empty() {
//             return serde_json::to_vec(&self).expect("Failed to serialize to JSON.").len();
//         }

//         println!("The table is empty.");
//         0
//     }

//     // Returns number of bytes or 0 when table is empty
//     #[inline]
//     pub fn estimate_cbor_size(&self) -> usize {
//         if !self.values.is_empty() {
//             return serde_cbor::to_vec(&self).expect("Failed to serialize to JSON.").len();
//         }

//         println!("The table is empty.");
//         0
//     }
// }

// // source: https://crates.io/crates/ph
// pub fn demo_perfect_hashing(){
//     let keys = ['a', 'b', 'z'];
//     let f = fmph::Function::from(keys.as_ref());

//     // f assigns each key a unique number from the set {0, 1, 2}
//     for k in keys {
//         println!("The key {} is assigned the value {}.", k, f.get(&k).unwrap());
//     }

//     let mut values = [f.get(&'a').unwrap(), f.get(&'b').unwrap(), f.get(&'z').unwrap()];
//     values.sort();
//     assert_eq!(values, [0, 1, 2]);
// }

// pub fn test_perfect_hashing() {
//     println!("###########");
//     let char_keys = ['w', 'e', 'r', 't'];
//     let char_function = fmph::Function::from(&char_keys[..]);

//     println!("Key, Value:");
//     for k in char_keys {
//         println!("{}, {}", k, char_function.get(&k).unwrap());
//     }

//     println!("###########");
//     let str_keys = ["www", "eee", "rrr", "ttt"];
//     let str_function = fmph::Function::from(&str_keys[..]);

//     println!("Key, Value:");
//     for k in str_keys {
//         println!("{}, {}", k, str_function.get(k).unwrap());
//     }
// }
