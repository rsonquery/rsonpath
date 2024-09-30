use proptest::char::range;
use rsonpath::lookup_table::{
    lut_naive::{self, LutNaive},
    lut_perfect_naive::LutPerfectNaive,
    lut_phf::LutPHF,
    pair_finder, util_path, LookUpTable,
};
use std::{fmt::format, fs};

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
    let keys = vec![18, 816];
    let expected_values = vec![11880, 895];

    test_lut_naive(json_path, keys, expected_values);
}

fn test_lut_naive(json_path: &str, keys: Vec<usize>, expected_values: Vec<usize>) {
    assert_eq!(keys.len(), expected_values.len());

    let lut_naive = LutNaive::build(&json_path).expect("Failed to build lookup table");

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

    let lut_naive = LutNaive::build(&json_path).expect("Failed to build lut_naive");
    let lut_perfect_naive = LutPerfectNaive::build_with_json(&file).expect("Failed to build lut_perfect_naive");

    let path = format!(
        "tests/json/serialize/{}_lut_perfect_naive_.cbor",
        util_path::get_filename_from_path(json_path)
    );
    let _ = lut_perfect_naive.serialize(&path);
    let lut_perfect_naive_deserialized =
        LutPerfectNaive::deserialize(&path).expect(&format!("Could not find {}", path));

    let keys: Vec<usize> = lut_naive.get_keys();

    for key in keys {
        let value = lut_naive.get(&key);

        assert_eq!(lut_perfect_naive.get(&key), value);
        assert_eq!(lut_perfect_naive_deserialized.get(&key), value);
    }
}

#[test]
fn phf_john() {
    test_lut_phf("tests/json/john.json");
}

#[test]
fn phf_pokemon_6mb() {
    test_lut_phf("tests/json/pokemon_(6MB).json");
}

#[test]
fn phf_short_80mb() {
    test_lut_phf("tests/json/twitter_short_(80MB).json");
}

fn test_lut_phf(json_path: &str) {
    let file = fs::File::open(json_path).expect(&format!("Failed to open file {}", json_path));

    let lut_naive = LutNaive::build(&json_path).expect("Failed to build lut_naive");
    let lut_phf = LutPHF::build_with_json(&file).expect("Failed to build lut_phf");

    let keys: Vec<usize> = lut_naive.get_keys();
    for key in keys {
        assert_eq!(lut_phf.get(&key), lut_naive.get(&key));
    }
}

// #[test]
// fn phf_double_john_big() {
//     test_lut_phf_double("tests/json/john_big.json");
// }

// #[test]
// fn phf_double_pokemon_6mb() {
//     test_lut_phf_double("tests/json/pokemon_(6MB).json");
// }

// #[test]
// fn phf_double_short_80mb() {
//     test_lut_phf_double("tests/json/twitter_short_(80MB).json");
// }

// #[test]
// fn phf_double_crossref0_320mb() {
//     test_lut_phf_double("tests/json/crossref0_(320MB).json");
// }

// #[test]
// fn phf_double_google_map_large_record_1_gb() {
//     // This took ~30 min
//     test_lut_phf_double("tests/json/google_map_large_record_(1.1GB).json");
// }

#[test]
fn test_lut_naive_john_big() {
    compare_lut_naive("tests/json/john_big.json");
}

#[test]
fn test_lut_naive_pokemon() {
    compare_lut_naive("tests/json/pokemon_(6MB).json");
}

#[test]
fn test_lut_naive_google_map_large_record() {
    compare_lut_naive("tests/json/google_map_large_record_(1.1GB).json");
}

fn compare_lut_naive(json_path: &str) {
    let lut_naive = LutNaive::build(&json_path).expect(&format!("Fail @ building lut_naive. Input = {}", json_path));
    compare_valid(&lut_naive, json_path);
}

fn compare_valid(lut: &dyn LookUpTable, json_path: &str) {
    let (keys, values) = pair_finder::get_keys_and_values(&json_path).expect("Fail @ finding pairs.");

    for (i, key) in keys.iter().enumerate() {
        assert_eq!(values[i], lut.get(key).expect("Fail at getting value."));
    }
}
