use rsonpath::lookup_table::{
    lut_distance::LutDistance, lut_naive::LutNaive, lut_perfect_naive::LutPerfectNaive, lut_phf::LutPHF,
    lut_phf_double::LutPHFDouble, pair_finder, LookUpTable,
};
use std::fmt::format;

const TEST_JSON_FILES: [&str; 3] = [
    "tests/json/john_big.json",
    "tests/json/pokemon_(6MB).json",
    "tests/json/twitter_short_(80MB).json",
    // "tests/json/crossref0_(320MB).json",
    // "tests/json/google_map_large_record_(1.1GB).json",
];

macro_rules! test_lut {
    ($lut_type:ident, $test_name:ident) => {
        #[test]
        fn $test_name() {
            for json_path in &TEST_JSON_FILES {
                let lut = $lut_type::build(json_path).expect(&format!(
                    "Fail @ building {}. Input = {}",
                    stringify!($lut_type),
                    json_path
                ));
                compare_valid(&lut, json_path);
            }
        }
    };
}

test_lut!(LutNaive, lut_naive);
test_lut!(LutDistance, lut_distance);
test_lut!(LutPHF, lut_phf);
test_lut!(LutPHFDouble, test_lut_phf_double);
test_lut!(LutPerfectNaive, lut_perfect_naive);

fn compare_valid(lut: &dyn LookUpTable, json_path: &str) {
    let (keys, values) = pair_finder::get_keys_and_values(json_path).expect("Fail @ finding pairs.");

    for (i, key) in keys.iter().enumerate() {
        assert_eq!(values[i], lut.get(key).expect("Fail at getting value."));
    }
}
