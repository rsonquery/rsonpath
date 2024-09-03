// use ph::fmph;
// use serde::{Deserialize, Serialize};
// use serde_cbor;
// use serde_json;
// use std::collections::HashMap;
// use std::fs::File;
// use std::io::{Error, ErrorKind, Read, Write};

// use crate::lookup_table::util;

// pub struct LutNaive {
//     keys: Vec<u64>,
//     values: Vec<u64>,

//     #[serde(skip_serializing)]
//     phf: fmph::Function,
// }

// impl LutNaive {
//     #[inline]
//     pub fn init(keys: Vec<u64>, values: Vec<u64>) -> Self {
//         // Ensure keys and values are of the same length
//         assert_eq!(keys.len(), values.len(), "Keys and values must have the same length");

//         let phf = fmph::Function::from(&keys[..]);

//         Self { keys, values, phf }
//     }

//     #[inline]
//     pub fn get(&self, key: &u64) -> Option<&u64> {
//         if let Some(index) = self.phf.get(key) {
//             if index < self.values.len() {
//                 return Some(&self.values[index]);
//             }
//         }
//         None
//     }
// }
