use rsonpath::lookup_table::{
    lut_hash_map::LutHashMap, lut_hash_map_double::LutHashMapDouble, lut_hash_map_group::LutHashMapGroup,
    lut_perfect_naive::LutPerfectNaive, lut_phf::LutPHF, lut_phf_double::LutPHFDouble, lut_phf_group::LutPHFGroup,
    pair_finder, LookUpTable,
};

// JSON files
const JOHN_BIG_JSON: &str = "tests/json/john_big.json";
const POKEMON_JSON: &str = "tests/json/pokemon_(6MB).json";
const TWITTER_SHORT_JSON: &str = "tests/json/twitter_short_(80MB).json";
const BESTBUY_JSON: &str = "tests/json/bestbuy_short_(103MB).json";

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

// Generate test functions for LutHashMap
test_lut_with_json!(LutHashMap, hash_map_john_big, JOHN_BIG_JSON);
test_lut_with_json!(LutHashMap, hash_map_pokemon, POKEMON_JSON);
test_lut_with_json!(LutHashMap, hash_map_twitter_short, TWITTER_SHORT_JSON);
test_lut_with_json!(LutHashMap, hash_map_crossref0, BESTBUY_JSON);

// Generate test functions for LutHashMapDouble
test_lut_with_json!(LutHashMapDouble, hash_map_double_john_big, JOHN_BIG_JSON);
test_lut_with_json!(LutHashMapDouble, hash_map_double_pokemon, POKEMON_JSON);
test_lut_with_json!(LutHashMapDouble, hash_map_double_twitter_short, TWITTER_SHORT_JSON);
test_lut_with_json!(LutHashMapDouble, hash_map_double_crossref0, BESTBUY_JSON);

// Generate test functions for LutHashMapGroup
test_lut_with_json!(LutHashMapGroup, hash_map_group_john_big, JOHN_BIG_JSON);
test_lut_with_json!(LutHashMapGroup, hash_map_group_pokemon, POKEMON_JSON);
test_lut_with_json!(LutHashMapGroup, hash_map_group_twitter_short, TWITTER_SHORT_JSON);
test_lut_with_json!(LutHashMapGroup, hash_map_group_crossref0, BESTBUY_JSON);

// Generate test functions for LutPerfectNaive
test_lut_with_json!(LutPerfectNaive, perfect_naive_john_big, JOHN_BIG_JSON);
test_lut_with_json!(LutPerfectNaive, perfect_naive_pokemon, POKEMON_JSON);
test_lut_with_json!(LutPerfectNaive, perfect_naive_twitter_short, TWITTER_SHORT_JSON);
test_lut_with_json!(LutPerfectNaive, perfect_naive_crossref0, BESTBUY_JSON);

// Generate test functions for LutPHF
test_lut_with_json!(LutPHF, phf_john_big, JOHN_BIG_JSON);
test_lut_with_json!(LutPHF, phf_pokemon, POKEMON_JSON);
test_lut_with_json!(LutPHF, phf_twitter_short, TWITTER_SHORT_JSON);
test_lut_with_json!(LutPHF, phf_crossref0, BESTBUY_JSON);

// Generate test functions for LutPHFDouble
test_lut_with_json!(LutPHFDouble, phf_double_john_big, JOHN_BIG_JSON);
test_lut_with_json!(LutPHFDouble, phf_double_pokemon, POKEMON_JSON);
test_lut_with_json!(LutPHFDouble, phf_double_twitter_short, TWITTER_SHORT_JSON);
test_lut_with_json!(LutPHFDouble, phf_double_crossref0, BESTBUY_JSON);

// Generate test functions for LutPHFGroup
test_lut_with_json!(LutPHFGroup, phf_group_john_big, JOHN_BIG_JSON);
test_lut_with_json!(LutPHFGroup, phf_group_pokemon, POKEMON_JSON);
test_lut_with_json!(LutPHFGroup, phf_group_twitter_short, TWITTER_SHORT_JSON);
test_lut_with_json!(LutPHFGroup, phf_group_crossref0, BESTBUY_JSON);

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
fn debug_lut_group_buckets() {
    let json_path = "tests/json/pokemon_(6MB).json";
    // let json_path = "tests/json/john_big.json";
    // let json_path = "tests/json/twitter_short_(80MB).json";

    let (keys, values) = pair_finder::get_keys_and_values(json_path).expect("Fail @ finding pairs.");
    let lut = LutPHFGroup::build_buckets(1, json_path, 63, false).expect("Fail @ building lut_phf_double");

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

#[test]
fn debug_lut_phf_double() {
    let json_path = "tests/json/pokemon_(6MB).json";
    // let json_path = "tests/json/john_big.json";
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
