use serde::{Deserialize, Serialize};
use serde_json;
use std::collections::HashMap;
use std::env;
use std::fs::File;
use std::io::{Read, Write};

#[derive(Serialize, Deserialize, Debug)]
struct LutNaive {
    capacity: usize,
    table: HashMap<u64, u64>,
}

impl LutNaive {
    fn init(size: Option<usize>) -> Self {
        let mut size = size.unwrap_or(0);
        LutNaive {
            capacity: size,
            table: HashMap::with_capacity(size),
        }
    }

    fn put(&mut self, key: u64, value: u64) {
        self.table.insert(key, value);
        self.capacity = self.table.capacity();
    }

    fn get(&self, key: &u64) -> Option<&u64> {
        self.table.get(key)
    }

    // Serialize this data structure into a .json file and save it at the given path
    fn serialize(&self, path: &str) -> std::io::Result<()> {
        let serialized_data = serde_json::to_string(&self).expect("Serialization failed");
        let mut file = File::create(path)?;
        file.write_all(serialized_data.as_bytes())?;
        Ok(())
    }

    // Deserialize the data structure from a .json file at the given path
    fn deserialize(path: &str) -> std::io::Result<Self> {
        let mut file = File::open(path)?;
        let mut contents = String::new();
        file.read_to_string(&mut contents)?;
        let deserialized: LutNaive = serde_json::from_str(&contents).expect("Deserialization failed");
        Ok(deserialized)
    }
}

pub fn example_usage(path: &str) {
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

    if let Ok(current_dir) = env::current_dir() {
        println!("The current directory is {}", current_dir.display());
    }

    // Serialize
    lut_naive.serialize(path).expect("Failed to serialize the LutNaive");

    // Deserialize from JSON file
    let lut_naive_des = LutNaive::deserialize(path).expect("Failed to deserialize the LutNaive");

    if let Some(value) = lut_naive_des.get(&50) {
        println!("Deserialized value for key '50': {}", value);
    }
    if let Some(value) = lut_naive_des.get(&100) {
        println!("Deserialized value for key '100': {}", value);
    }
}
