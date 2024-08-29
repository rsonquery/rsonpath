use serde::{Deserialize, Serialize};
use serde_cbor;
use serde_json;
use std::collections::HashMap;
use std::env;
use std::fs::File;
use std::io::{Read, Write};

#[derive(Serialize, Deserialize, Debug)]
struct LutNaive {
    table: HashMap<u64, u64>,
}

impl LutNaive {
    fn init(start_capacity: Option<usize>) -> Self {
        let mut size = start_capacity.unwrap_or(0);
        LutNaive {
            table: HashMap::with_capacity(size),
        }
    }

    fn put(&mut self, key: u64, value: u64) {
        self.table.insert(key, value);
    }

    fn get(&self, key: &u64) -> Option<&u64> {
        self.table.get(key)
    }

    // Serialize this data structure into a .json or .cbor file based on the given format
    fn serialize(&self, path: &str, format: &str) -> std::io::Result<()> {
        let serialized_data = match format {
            "json" => serde_json::to_vec(&self).expect("Serialize: Data has no JSON format."),
            "cbor" => serde_cbor::to_vec(&self).expect("Serialize: Data has no CBOR format. Abort."),
            _ => panic!("Unsupported format"),
        };
        let mut file = File::create(path)?;
        file.write_all(&serialized_data)?;
        Ok(())
    }

    // Deserialize the data structure from a .json or .cbor file based on the given format
    fn deserialize(path: &str, format: &str) -> std::io::Result<Self> {
        let mut file = File::open(path)?;
        let mut contents = Vec::new();
        file.read_to_end(&mut contents)?;
        let deserialized: LutNaive = match format {
            "json" => serde_json::from_slice(&contents).expect("Deserialize: Data has no JSON format."),
            "cbor" => serde_cbor::from_slice(&contents).expect("Deserialize: Data has no CBOR format."),
            _ => panic!("Unsupported format"),
        };
        Ok(deserialized)
    }

    fn overview(&self) {
        if !self.table.is_empty() {
            println!("lut-naive Overview:");
            println!("  #Entries: {}", self.table.len());
            println!("  Capacity: {}", self.table.capacity());

            // Serialize to JSON and CBOR to estimate file sizes
            let json_data = serde_json::to_vec(&self).expect("Failed to serialize to JSON.");
            let cbor_data = serde_cbor::to_vec(&self).expect("Failed to serialize to CBOR.");
            println!("  JSON: {} bytes", json_data.len());
            println!("  CBOR: {} bytes", cbor_data.len());

            // Calculate and print the average of (value - key) called the distance
            let mut total_distance = 0u64;
            for (key, value) in self.table.iter() {
                total_distance += value - key;
            }
            let average_distance = total_distance as f64 / self.table.len() as f64;
            println!("  Average distance (value - key): {:.2}", average_distance);

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

pub fn example_usage(path: &str, format: &str) {
    let mut lut_naive = LutNaive::init(Some(10));

    // Add Data
    lut_naive.put(1, 100);
    lut_naive.put(2, 200);

    for i in 1..=100 {
        lut_naive.put(i, i * 100);
    }

    if let Some(value) = lut_naive.get(&50) {
        println!("Value for key '50': {}", value);
    }

    // Serialize with specified format
    lut_naive
        .serialize(path, format)
        .expect("Failed to serialize the LutNaive");

    // Deserialize with specified format
    let lut_naive_des = LutNaive::deserialize(path, format).expect("Failed to deserialize the LutNaive");

    if let Some(value) = lut_naive_des.get(&50) {
        println!("Deserialized value for key '50': {}", value);
    }
    if let Some(value) = lut_naive_des.get(&100) {
        println!("Deserialized value for key '100': {}", value);
    }

    lut_naive.overview();
}
