use serde_json::{json, Value};
use std::fs;
use std::path::Path;

pub fn generate_bigger_version(json_path: &str) {
    // Read the original JSON file
    let file_content = fs::read_to_string(json_path).expect("Failed to read JSON file");

    // Parse the JSON content
    let data: Value = serde_json::from_str(&file_content).expect("Failed to parse JSON");

    // Extract the 'cfgs' array
    let cfgs = data["cfgs"].clone();

    // Create a new object to store the expanded data
    let mut new_data = serde_json::Map::new();

    // Add duplicated cfgs with new keys (cfg1, cfg2, ..., cfg50)
    for i in 1..=50 {
        new_data.insert(format!("cfg{}", i), cfgs.clone());
    }

    // Retain tail-data fields
    if let Some(tail_data) = data.get("tail-data") {
        new_data.insert("tail-data".to_string(), tail_data.clone());
    }
    if let Some(tail_data_2) = data.get("tail-data-2") {
        new_data.insert("tail-data-2".to_string(), tail_data_2.clone());
    }

    // Convert to JSON
    let new_json = json!(new_data);

    // Define the new file path
    let new_file_path = Path::new(json_path).with_file_name("pokemon_big.json");

    // Write the new JSON data to the file without pretty formatting
    fs::write(
        new_file_path,
        serde_json::to_string(&new_json).expect("Failed to serialize JSON"),
    )
    .expect("Failed to write JSON file");
}
