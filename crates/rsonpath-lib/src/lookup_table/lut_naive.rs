use serde::{Deserialize, Serialize};
use serde_cbor;
use serde_json;
use std::collections::HashMap;
use std::fs::File;
use std::io::{Read, Write};

use crate::lookup_table::util;

#[derive(Serialize, Deserialize, Debug)]
pub struct LutNaive {
    table: HashMap<usize, usize>,
}

impl LutNaive {
    pub fn init(start_capacity: Option<usize>) -> Self {
        let mut size = start_capacity.unwrap_or(0);
        LutNaive {
            table: HashMap::with_capacity(size),
        }
    }

    pub fn put(&mut self, key: usize, value: usize) {
        self.table.insert(key, value);
    }

    pub fn get(&self, key: &usize) -> Option<&usize> {
        self.table.get(key)
    }

    pub fn serialize(&self, path: &str) -> std::io::Result<()> {
        let serialized_data = match util::get_filetype_from_path(path).as_str() {
            "json" => serde_json::to_vec(&self).expect("Serialize failed."),
            "cbor" => serde_cbor::to_vec(&self).expect("Serialize failed."),
            _ => panic!("Serialize: Unsupported format"), // TODO return error here
        };
        let mut file = File::create(path)?;
        file.write_all(&serialized_data)?;
        Ok(())
    }

    pub fn deserialize(path: &str) -> std::io::Result<Self> {
        let mut file = File::open(path)?;
        let mut contents = Vec::new();
        file.read_to_end(&mut contents)?;
        let deserialized: LutNaive = match util::get_filetype_from_path(path).as_str() {
            "json" => serde_json::from_slice(&contents).expect("Deserialize: Data has no JSON format."),
            "cbor" => serde_cbor::from_slice(&contents).expect("Deserialize: Data has no CBOR format."),
            _ => panic!("Deserialize: Unsupported format"), // TODO return error here
        };
        Ok(deserialized)
    }

    pub fn overview(&self) {
        if !self.table.is_empty() {
            println!("lut-naive Overview:");
            println!("  #Entries: {}", self.table.len());
            println!("  Capacity: {}", self.table.capacity());

            // Serialize to JSON and CBOR to estimate file sizes
            let json_data = serde_json::to_vec(&self).expect("Failed to serialize to JSON.");
            let cbor_data = serde_cbor::to_vec(&self).expect("Failed to serialize to CBOR.");
            println!("  JSON: {} bytes", json_data.len());
            println!("  CBOR: {} bytes", cbor_data.len());

            // Calculate and print the average, maximum, and minimum of (value - key) called the distance
            let mut total_distance = 0usize;
            let mut max_distance = usize::MIN;
            let mut min_distance = usize::MAX;
            for (key, value) in self.table.iter() {
                let distance = (*value).saturating_sub(*key); // Ensures non-negative distances
                total_distance += distance;
                max_distance = max_distance.max(distance);
                min_distance = min_distance.min(distance);
            }
            let average_distance = total_distance as f64 / self.table.len() as f64;

            println!("  Average distance (value - key): {:.2}", average_distance);
            println!("  MAX distance (value - key): {}", max_distance);
            println!("  MIN distance (value - key): {}", min_distance);

            // Print up to the first 10 pairs
            println!("  First 10 pairs:");
            for (i, (key, value)) in self.table.iter().take(10).enumerate() {
                println!("    {}. Key: {}, Value: {}", i + 1, key, value);
            }
        } else {
            println!("The table is empty.");
        }
    }
}
