use std::fs;

use rsonpath::lookup_table::{
    lut_distance::{self},
    lut_naive::{self},
};

#[test]
fn test_naive_john() {
    let file = fs::File::open("tests/john.json").expect("Failed to open file");
    let lut_naive = lut_naive::build(&file).expect("Failed to build lookup table");

    let keys = [0, 67];
    let expected_values = [117, 96];

    assert_eq!(keys.len(), expected_values.len());

    for (key, &expected_value) in keys.iter().zip(expected_values.iter()) {
        assert_eq!(lut_naive.get(key).copied(), Some(expected_value));
    }
}

#[test]
fn test_distance_john() {
    let file = fs::File::open("tests/john.json").expect("Failed to open file");
    let lut_distance = lut_distance::build(&file).expect("Failed to build lookup table");

    let keys = [0, 67];
    let expected_values = [117, 96];

    assert_eq!(keys.len(), expected_values.len());

    for (key, &expected_value) in keys.iter().zip(expected_values.iter()) {
        assert_eq!(lut_distance.get(key), Some(expected_value));
    }
}
