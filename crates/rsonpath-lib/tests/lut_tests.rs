use itertools::enumerate;
use rsonpath::lookup_table::{
    lut_distance::LutDistance, lut_naive::LutNaive, lut_perfect_naive::LutPerfectNaive, lut_phf::LutPHF,
    lut_phf_double::LutPHFDouble, pair_finder, LookUpTable,
};

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

#[test]
fn debug_lut_phf_double() {
    let json_path = "tests/json/pokemon_(6MB).json";
    // let json_path = "tests/json/twitter_short_(80MB).json";

    let (keys, values) = pair_finder::get_keys_and_values(json_path).expect("Fail @ finding pairs.");
    let lut = LutPHFDouble::build(json_path).expect("Fail @ building lut_phf_double");

    let mut count_correct = 0;
    let mut count_incorrect = 0;
    for (i, key) in keys.iter().enumerate() {
        let left = values[i];
        let right = lut.get(key).expect("fail");
        if left != right {
            let distance = left - key;
            println!(
                "Key: {}, Expected: {}, Found: {}, Expected Dist. {}",
                key, left, right, distance
            );
            count_incorrect += 1;
        } else {
            count_correct += 1;
        }
    }

    let total = count_correct + count_incorrect;
    println!("Correct: {}/{}", count_correct, total);
    println!("Incorrect: {}/{}", count_incorrect, total);
    assert_eq!(count_incorrect, 0);
}
