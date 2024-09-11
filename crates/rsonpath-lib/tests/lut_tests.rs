use std::fs;

use rsonpath::lookup_table::{
    lut_distance::{self},
    lut_naive::{self},
    lut_perfect_naive,
};

#[test]
fn naive_john() {
    let json_path = "tests/json/john.json";
    let keys = vec![0, 67];
    let expected_values = vec![117, 96];

    test_lut_naive(json_path, keys, expected_values);
}

#[test]
fn naive_john_big() {
    let json_path = "tests/json/john_big.json";
    let keys = vec![0, 16, 258];
    let expected_values = vec![1159, 1157, 477];

    test_lut_naive(json_path, keys, expected_values);
}

#[test]
fn naive_pokemon() {
    let json_path = "tests/json/pokemon_(6MB).json";
    let keys = vec![18, 814];
    let expected_values = vec![11878, 894];

    test_lut_naive(json_path, keys, expected_values);
}

fn test_lut_naive(json_path: &str, keys: Vec<usize>, expected_values: Vec<usize>) {
    assert_eq!(keys.len(), expected_values.len());
    let file = fs::File::open(json_path).expect("Failed to open file");

    let lut_naive = lut_naive::build(&file).expect("Failed to build lookup table");

    for (key, &expected_value) in keys.iter().zip(expected_values.iter()) {
        let actual_value = lut_naive.get(&key).expect("Key not found in lut_naive");
        if actual_value != expected_value {
            println!(
                "Mismatch for key {}: expected {}, got {}",
                key, expected_value, actual_value
            );
        }
        assert_eq!(actual_value, expected_value);
    }
}

#[test]
fn perfect_naive_john() {
    test_lut_perfect_naive("tests/json/john.json");
}

#[test]
fn perfect_naive_pokemon_6mb() {
    test_lut_perfect_naive("tests/json/pokemon_(6MB).json");
}

#[test]
fn perfect_naive_twitter_short_80mb() {
    test_lut_perfect_naive("tests/json/twitter_short_(80MB).json");
}

fn test_lut_perfect_naive(json_path: &str) {
    let file = fs::File::open(json_path).expect(&format!("Failed to open file {}", json_path));

    let lut_naive = lut_naive::build(&file).expect("Failed to build lut_naive");
    let lut_perfect_naive = lut_perfect_naive::build(&file).expect("Failed to build lut_perfect_naive");

    let keys: Vec<usize> = lut_naive.get_keys();

    for key in keys {
        assert_eq!(lut_naive.get(&key), lut_perfect_naive.get(&key));
    }
}
