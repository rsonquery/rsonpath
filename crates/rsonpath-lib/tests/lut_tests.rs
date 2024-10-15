use rsonpath::lookup_table::{
    lut_distance::LutDistance, lut_naive::LutNaive, lut_perfect_naive::LutPerfectNaive, lut_phf::LutPHF,
    lut_phf_double::LutPHFDouble, pair_finder, LookUpTable,
};

// JSON files
const JOHN_BIG_JSON: &str = "tests/json/john_big.json";
const POKEMON_JSON: &str = "tests/json/pokemon_(6MB).json";
const TWITTER_SHORT_JSON: &str = "tests/json/twitter_short_(80MB).json";
const CROSSREF0_JSON: &str = "tests/json/crossref0_(320MB).json";

// Macro to generate individual test functions for each (lut_type, json_file) combination
macro_rules! test_lut_with_json {
    ($lut_type:ident, $test_name:ident, $json_file:ident) => {
        #[test]
        fn $test_name() {
            let json_path = $json_file;
            let lut = $lut_type::build(json_path).expect(&format!(
                "Fail @ building {}. Input = {}",
                stringify!($lut_type),
                json_path
            ));
            compare_valid(&lut, json_path);
        }
    };
}

// Generate test functions for LutNaive
test_lut_with_json!(LutNaive, test_lut_naive_john_big, JOHN_BIG_JSON);
test_lut_with_json!(LutNaive, test_lut_naive_pokemon, POKEMON_JSON);
test_lut_with_json!(LutNaive, test_lut_naive_twitter_short, TWITTER_SHORT_JSON);
test_lut_with_json!(LutNaive, test_lut_naive_crossref0, CROSSREF0_JSON);

// Generate test functions for LutDistance
test_lut_with_json!(LutDistance, test_lut_distance_john_big, JOHN_BIG_JSON);
test_lut_with_json!(LutDistance, test_lut_distance_pokemon, POKEMON_JSON);
test_lut_with_json!(LutDistance, test_lut_distance_twitter_short, TWITTER_SHORT_JSON);
test_lut_with_json!(LutDistance, test_lut_distance_crossref0, CROSSREF0_JSON);

// Generate test functions for LutPHF
test_lut_with_json!(LutPHF, test_lut_phf_john_big, JOHN_BIG_JSON);
test_lut_with_json!(LutPHF, test_lut_phf_pokemon, POKEMON_JSON);
test_lut_with_json!(LutPHF, test_lut_phf_twitter_short, TWITTER_SHORT_JSON);
test_lut_with_json!(LutPHF, test_lut_phf_crossref0, CROSSREF0_JSON);

// Generate test functions for LutPHFDouble
test_lut_with_json!(LutPHFDouble, test_lut_phf_double_john_big, JOHN_BIG_JSON);
test_lut_with_json!(LutPHFDouble, test_lut_phf_double_pokemon, POKEMON_JSON);
test_lut_with_json!(LutPHFDouble, test_lut_phf_double_twitter_short, TWITTER_SHORT_JSON);
test_lut_with_json!(LutPHFDouble, test_lut_phf_double_crossref0, CROSSREF0_JSON);

// Generate test functions for LutPerfectNaive
test_lut_with_json!(LutPerfectNaive, test_lut_perfect_naive_john_big, JOHN_BIG_JSON);
test_lut_with_json!(LutPerfectNaive, test_lut_perfect_naive_pokemon, POKEMON_JSON);
test_lut_with_json!(
    LutPerfectNaive,
    test_lut_perfect_naive_twitter_short,
    TWITTER_SHORT_JSON
);
test_lut_with_json!(LutPerfectNaive, test_lut_perfect_naive_crossref0, CROSSREF0_JSON);

fn compare_valid(lut: &dyn LookUpTable, json_path: &str) {
    let (keys, values) = pair_finder::get_keys_and_values(json_path).expect("Fail @ finding pairs.");

    let mut count_incorrect: u64 = 0;
    for (i, key) in keys.iter().enumerate() {
        if values[i] != lut.get(key).expect("Fail at getting value.") {
            count_incorrect += 1;
        }
    }
    assert_eq!(count_incorrect, 0);
}

#[test]
fn debug_lut_phf_double() {
    // let json_path = "tests/json/pokemon_(6MB).json";
    let json_path = "tests/json/twitter_short_(80MB).json";

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
