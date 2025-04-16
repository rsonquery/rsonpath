use rsonpath::lookup_table::{
    lut_hash_map::LutHashMap,
    lut_hash_map_double::LutHashMapDouble,
    lut_hash_map_group::LutHashMapGroup,
    lut_perfect_naive::LutPerfectNaive,
    lut_phf::LutPHF,
    lut_phf_double::LutPHFDouble,
    lut_phf_group::LutPHFGroup,
    lut_ptr_hash_double::LutPtrHashDouble,
    pair_data,
    performance::lut_query_data::{BESTBUY_SHORT, JOHN_BIG, POKEMON_MINI, TWITTER_SHORT},
    LookUpTable, DISTANCE_CUT_OFF,
};

// Macro to generate individual test functions for each (lut_type, json_file) combination
macro_rules! build_test {
    ($lut_type:ident, $test_name:ident, $json_file:ident) => {
        #[test]
        fn $test_name() {
            let json_path = format!("../../{}", $json_file);
            let lut = $lut_type::build(&json_path, 0).expect(&format!(
                "Fail @ building {}. Input = {}",
                stringify!($lut_type),
                json_path
            ));

            compare_valid(&lut, &json_path);
        }
    };
}

fn compare_valid(lut: &dyn LookUpTable, json_path: &str) {
    let (keys, values) =
        pair_data::get_keys_and_values_absolute(json_path, DISTANCE_CUT_OFF).expect("Fail @ finding pairs.");

    let mut count_incorrect: u64 = 0;
    for (i, key) in keys.iter().enumerate() {
        if values[i] != lut.get(key).expect("Fail at getting value.") {
            count_incorrect += 1;
        }
    }
    assert_eq!(count_incorrect, 0);
}

// Example usage:
build_test!(LutHashMap, hash_map_john_big, JOHN_BIG);
build_test!(LutHashMap, hash_map_pokemon, POKEMON_MINI);
build_test!(LutHashMap, hash_map_twitter_short, TWITTER_SHORT);
build_test!(LutHashMap, hash_map_crossref0, BESTBUY_SHORT);

// LutHashMapDouble
build_test!(LutHashMapDouble, hash_map_double_john_big, JOHN_BIG);
build_test!(LutHashMapDouble, hash_map_double_pokemon, POKEMON_MINI);
build_test!(LutHashMapDouble, hash_map_double_twitter_short, TWITTER_SHORT);
build_test!(LutHashMapDouble, hash_map_double_crossref0, BESTBUY_SHORT);

// LutHashMapGroup
build_test!(LutHashMapGroup, hash_map_group_john_big, JOHN_BIG);
build_test!(LutHashMapGroup, hash_map_group_pokemon, POKEMON_MINI);
build_test!(LutHashMapGroup, hash_map_group_twitter_short, TWITTER_SHORT);
build_test!(LutHashMapGroup, hash_map_group_crossref0, BESTBUY_SHORT);

// LutPerfectNaive
build_test!(LutPerfectNaive, perfect_naive_john_big, JOHN_BIG);
build_test!(LutPerfectNaive, perfect_naive_pokemon, POKEMON_MINI);
build_test!(LutPerfectNaive, perfect_naive_twitter_short, TWITTER_SHORT);
build_test!(LutPerfectNaive, perfect_naive_crossref0, BESTBUY_SHORT);

// LutPHF
build_test!(LutPHF, phf_john_big, JOHN_BIG);
build_test!(LutPHF, phf_pokemon, POKEMON_MINI);
build_test!(LutPHF, phf_twitter_short, TWITTER_SHORT);
build_test!(LutPHF, phf_crossref0, BESTBUY_SHORT);

// LutPHFDouble
build_test!(LutPHFDouble, phf_double_john_big, JOHN_BIG);
build_test!(LutPHFDouble, phf_double_pokemon, POKEMON_MINI);
build_test!(LutPHFDouble, phf_double_twitter_short, TWITTER_SHORT);
build_test!(LutPHFDouble, phf_double_crossref0, BESTBUY_SHORT);

// LutPHFGroup
build_test!(LutPHFGroup, phf_group_john_big, JOHN_BIG);
build_test!(LutPHFGroup, phf_group_pokemon, POKEMON_MINI);
build_test!(LutPHFGroup, phf_group_twitter_short, TWITTER_SHORT);
build_test!(LutPHFGroup, phf_group_crossref0, BESTBUY_SHORT);

// LutPtrHashDouble
build_test!(LutPtrHashDouble, ptr_hash_john_big, JOHN_BIG);
build_test!(LutPtrHashDouble, ptr_hash_pokemon, POKEMON_MINI);
build_test!(LutPtrHashDouble, ptr_hash_twitter_short, TWITTER_SHORT);
build_test!(LutPtrHashDouble, ptr_hash_crossref0, BESTBUY_SHORT);
