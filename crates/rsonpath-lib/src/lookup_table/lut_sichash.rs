use super::{
    lut_phf_double::THRESHOLD_16_BITS,
    performance::lut_query_data::{BESTBUY, GOOGLE, TWITTER, WALMART},
    LookUpTable,
};

use std::{
    ffi::{c_void, CString},
    os::raw::c_char,
    str::{self},
};

use std::io::{BufReader, Read};

use std::{collections::VecDeque, fs};

use crate::{
    engine::{Compiler, Engine, RsonpathEngine},
    input::OwnedBytes,
    lookup_table::{
        self, lut_hash_map, pair_finder,
        performance::{
            lut_query_data::{
                ALPHABET, JOHN, JOHN_BIG, POKEMON_SHORT, QUERY_BESTBUY, QUERY_GOOGLE, QUERY_POKEMON_SHORT,
                QUERY_TWITTER, TWITTER_MINI,
            },
            lut_skip_evaluation::DISTANCE_CUT_OFF,
        },
        LUT,
    },
};

use crate::{
    classification::{
        self,
        simd::Simd,
        structural::{BracketType, Structural, StructuralIterator},
    },
    input::{self, error, Input},
    lookup_table::performance::lut_query_data,
    result::empty::EmptyRecorder,
    FallibleIterator,
};

/// Helper struct, because it makes the code shorter and cleaner to read.
#[derive(Clone, Default)]
pub struct PairDataSicHash {
    pub keys: Vec<String>,
    pub keys_lengths: Vec<usize>,
    pub values: Vec<u16>,
    pub keys_64: Vec<String>,
    pub keys_64_lengths: Vec<usize>,
    pub values_64: Vec<u64>,
}

impl PairDataSicHash {
    #[inline]
    #[must_use]
    pub fn new() -> Self {
        Self {
            keys: vec![],
            keys_lengths: vec![],
            values: vec![],
            keys_64: vec![],
            keys_64_lengths: vec![],
            values_64: vec![],
        }
    }
}

// The underlying code is based on the SicHash reporsitory (https://github.com/ByteHamster/SicHash) which was wrapped
// into a dynamic library (.so) so it can be called here from rust.
#[link(name = "sichash_ffi", kind = "dylib")]
extern "C" {
    fn build_phf(
        keys: *const *const c_char,
        keys_lengths: *const usize,
        values: *const u16,
        length: usize,
        keys_64: *const *const c_char,
        keys_64_lengths: *const usize,
        values_64: *const u64,
        length_64: usize,
    ) -> *mut c_void;

    fn say_hello();
    fn get_value(instance: *mut c_void, key: *const c_char, key_length: usize) -> u64;
    fn get_allocated_bytes(instance: *mut c_void) -> usize;
    fn drop(instance: *mut c_void);
}
pub struct LutSicHash {
    lut: *mut c_void,
}

impl LutSicHash {
    pub fn say_hello() {
        unsafe {
            say_hello();
        }
    }

    pub fn new(pair_data: PairDataSicHash) -> Self {
        let keys_lengths: &[usize] = &pair_data.keys_lengths;
        let values: &[u16] = &pair_data.values;
        let keys_64_lengths: &[usize] = &pair_data.keys_64_lengths;
        let values_64: &[u64] = &pair_data.values_64;

        let (_c_keys, keys_ptrs) = LutSicHash::convert_keys(&pair_data.keys);
        let (_c_keys_64, keys_64_ptrs) = LutSicHash::convert_keys(&pair_data.keys_64);

        let lut = unsafe {
            build_phf(
                keys_ptrs.as_ptr(),
                keys_lengths.as_ptr(),
                values.as_ptr(),
                values.len(),
                keys_64_ptrs.as_ptr(),
                keys_64_lengths.as_ptr(),
                values_64.as_ptr(),
                values_64.len(),
            )
        };

        Self { lut }
    }

    /// We count the distances between the opening and closing brackets. We save the start position as key and
    /// distance to the closing bracket in the value. Creates a key-value list for values which fit in a 16 bit
    /// representation and another key-value list for the ones that do not. Ignore all pairs with distances <
    /// distance_cutoff.
    #[inline]
    pub(crate) fn find_all_pairs<I, V>(
        input: &I,
        simd: V,
        distance_cutoff: usize,
    ) -> Result<PairDataSicHash, error::InputError>
    where
        I: Input,
        V: Simd,
    {
        let iter = input.iter_blocks::<_, 64>(&EmptyRecorder);
        let quote_classifier = simd.classify_quoted_sequences(iter);
        let mut structural_classifier = simd.classify_structural_characters(quote_classifier);
        structural_classifier.turn_colons_and_commas_off();

        // Initialize two empty stacks: one for "[" and one for "{", to remember the order we have found them
        let mut square_bracket_stack: VecDeque<usize> = VecDeque::new();
        let mut curly_bracket_stack: VecDeque<usize> = VecDeque::new();

        let mut pairs = PairDataSicHash::new();

        while let Some(event) = structural_classifier.next()? {
            match event {
                Structural::Opening(b, idx_open) => match b {
                    BracketType::Square => square_bracket_stack.push_back(idx_open),
                    BracketType::Curly => curly_bracket_stack.push_back(idx_open),
                },
                Structural::Closing(b, idx_close) => {
                    let idx_open = match b {
                        BracketType::Square => square_bracket_stack.pop_back().expect("Unmatched closing }"),
                        BracketType::Curly => curly_bracket_stack.pop_back().expect("Unmatched closing }"),
                    };

                    // Check if distance can be represented with 16 or less bits
                    let distance = idx_close - idx_open;
                    if distance > distance_cutoff {
                        let key_string = idx_open.to_string();
                        let key_length = key_string.len();
                        if distance < THRESHOLD_16_BITS {
                            // Can fit into 16 bit
                            pairs.values.push(distance.try_into().expect("Fail at pushing value."));
                        } else {
                            // Cannot fit into 16 bit
                            pairs.values.push(0);

                            pairs.keys_64.push(key_string.clone());
                            pairs.keys_64_lengths.push(key_length);
                            pairs.values_64.push(distance as u64);
                        }
                        pairs.keys.push(key_string.clone());
                        pairs.keys_lengths.push(key_length);
                    }
                }
                Structural::Colon(_) | Structural::Comma(_) => unreachable!(),
            }
        }

        // TODO delete this test code, currently SicHashPHF will cause a segmentation fault if called with an empty list.
        // That is why need to add one artificial entry in both lists. The currently picked dummy can cause issues at
        // the moment
        let dummy_number: usize = 999999999999;
        let dummy_string_1 = dummy_number.to_string();
        let dummy_string_2 = dummy_number.to_string();
        let dummy_length = dummy_string_1.len();
        pairs.keys.push(dummy_string_1);
        pairs.keys_lengths.push(dummy_length);
        pairs.values.push(0);
        pairs.keys_64.push(dummy_string_2);
        pairs.keys_64_lengths.push(dummy_length);
        pairs.values_64.push(999999999999 + 1);

        // println!("----");
        // println!("Table data keys");
        // for i in 0..pairs.keys.len() {
        //     println!("  {}: key {}, length {}, value {}", i, pairs.keys[i], pairs.keys_lengths[i], pairs.values[i]);
        // }
        // println!("Table data keys_64");
        // for i in 0..pairs.keys_64.len() {
        //     println!("  {}: key {}, length {}, value {}", i, pairs.keys_64[i], pairs.keys_64_lengths[i], pairs.values_64[i]);
        // }
        // println!("----");

        Ok(pairs)
    }

    // TODO: this is a highly inefficient copy of large lists, which should be get rid of. At the moment it is the
    // only working solution we have.
    fn convert_keys(keys: &[String]) -> (Vec<CString>, Vec<*const c_char>) {
        let c_keys: Vec<CString> = keys.iter().map(|s| CString::new(s.as_str()).unwrap()).collect();
        let ptrs: Vec<*const c_char> = c_keys.iter().map(|s| s.as_ptr()).collect();
        (c_keys, ptrs)
    }
}

impl Drop for LutSicHash {
    fn drop(&mut self) {
        unsafe { drop(self.lut) };
    }
}

impl LookUpTable for LutSicHash {
    #[inline]
    fn build(json_path: &str, distance_cutoff: usize) -> Result<Self, Box<dyn std::error::Error>> {
        let file = fs::File::open(json_path).expect("Failed to open file");
        // SAFETY: We keep the file open throughout the entire duration.
        let input = unsafe { input::MmapInput::map_file(&file)? };
        let simd_c = classification::simd::configure();

        let lut_phf_double = classification::simd::config_simd!(simd_c => |simd| {
            classification::simd::dispatch_simd!(simd; input, simd, distance_cutoff => fn<I, V>(
                input: I,
                simd: V,
                distance_cutoff: usize,
            ) -> Result<LutSicHash, error::InputError> where
            I: Input,
            V: Simd, {
                    let pair_data = LutSicHash::find_all_pairs::<I, V>(&input, simd, distance_cutoff)?;
                    Ok(LutSicHash::new(pair_data))
                })
        });
        lut_phf_double.map_err(|e| Box::new(e) as Box<dyn std::error::Error>)
    }

    #[inline]
    fn get(&self, key: &usize) -> Option<usize> {
        // Convert the numeric key to a string
        let key_str = key.to_string();
        let key_length = key_str.len();
        let c_key = CString::new(key_str).ok()?;
        let key_ptr = c_key.as_ptr();

        let distance = unsafe { get_value(self.lut, key_ptr, key_length) } as usize;
        Some(key + distance)
    }

    #[inline]
    fn allocated_bytes(&self) -> usize {
        unsafe { get_allocated_bytes(self.lut) }
    }
}

// ########################
// #### Test functions ####
// ########################
pub fn test_sichash_lut() {
    // query_and_build_test();

    // test_build_correctness(GOOGLE);
    // test_build_correctness(WALMART);
    // test_build_correctness(BESTBUY);
    // test_build_correctness(TWITTER);
    // test_build_correctness(POKEMON_SHORT);

    // test_query_correctness(lut_query_data::QUERY_POKEMON_MINI);
    test_query_correctness(lut_query_data::QUERY_GOOGLE);
    // test_query_correctness(lut_query_data::QUERY_TWITTER);
    // test_query_correctness(lut_query_data::QUERY_BESTBUY);
    // test_query_correctness(lut_query_data::QUERY_POKEMON_SHORT);
}

pub fn test_build_correctness(json_path: &str) {
    println!("Building LUT: {}", json_path);
    let lut = LUT::build(&json_path, 0).expect("Fail @ building LUT");
    println!("Building LUT (Hashmap): {}", json_path);
    let lut_hash_map = lut_hash_map::LutHashMap::build(&json_path, 0).expect("Fail @ building LUT");

    println!("Testing keys ...");
    let (keys, values) = pair_finder::get_keys_and_values(json_path).expect("Fail @ finding pairs.");
    let mut count_incorrect = 0;
    for (i, key) in keys.iter().enumerate() {
        let value = lut.get(key).expect("Fail at getting value.");
        let value_hash = lut_hash_map.get(key).expect("Fail at getting value.");
        if value != values[i] || value != value_hash {
            count_incorrect += 1;
            println!(
                "  i: {}, Key {}, Value {}, Expected: {}, Hash {}",
                i, key, value, values[i], value_hash
            );
        }
    }

    println!(" Correct {}/{}", keys.len() - count_incorrect, keys.len());
    println!(" Incorrect {}/{}", count_incorrect, keys.len());

    std::mem::drop(lut);
}

pub fn test_query_correctness(test_data: (&str, &[(&str, &str)])) {
    let (json_path, queries) = test_data;
    println!("Building LUT: {}", json_path);
    let mut lut = LUT::build(&json_path, DISTANCE_CUT_OFF).expect("Fail @ building LUT");

    // Run all queries
    println!("Checking queries:");
    for &(query_name, query_text) in queries {
        print!(" Query: {} ... ", query_name);
        let input = {
            let mut file = BufReader::new(fs::File::open(json_path).expect("Fail @ open File"));
            let mut buf = vec![];
            file.read_to_end(&mut buf).expect("Fail @ file read");
            OwnedBytes::new(buf)
        };
        let query = rsonpath_syntax::parse(query_text).expect("Fail @ parse query");

        // Query normally and skip iteratively (ITE)
        let mut engine = RsonpathEngine::compile_query(&query).expect("Fail @ compile query");
        let count = engine.count(&input).expect("Failed to run query normally");

        // Query normally and skip using the lookup table (LUT)
        engine.add_lut(lut);
        let lut_count = engine.count(&input).expect("LUT: Failed to run query normally");

        if lut_count != count {
            println!("Found {}, Expected {}", lut_count, count);
        } else {
            println!("Correct");
        }

        lut = engine.take_lut().expect("Failed to retrieve LUT from engine");
    }

    std::mem::drop(lut);
}

fn query_and_build_test() {
    let test_data = lut_query_data::QUERY_POKEMON_SHORT;
    let (json_path, queries) = test_data;

    println!("Building LUT: {}", json_path);
    let mut lut = LUT::build(&json_path, 0).expect("Fail @ building LUT");
    println!("Building LUT (Hashmap): {}", json_path);
    let lut_hash_map = lut_hash_map::LutHashMap::build(&json_path, 0).expect("Fail @ building LUT");

    // TEST BUILD
    println!("Testing keys ...");
    let (keys, values) = pair_finder::get_keys_and_values(json_path).expect("Fail @ finding pairs.");
    let mut count_incorrect = 0;
    for (i, key) in keys.iter().enumerate() {
        let value = lut.get(key).expect("Fail at getting value.");
        let value_hash = lut_hash_map.get(key).expect("Fail at getting value.");
        if value != values[i] || value != value_hash {
            count_incorrect += 1;
            println!(
                "  i: {}, Key {}, Value {}, Expected: {}, Hash {}",
                i, key, value, values[i], value_hash
            );
        }
    }
    println!(" Correct {}/{}", keys.len() - count_incorrect, keys.len());
    println!(" Incorrect {}/{}", count_incorrect, keys.len());

    // QUERIES
    // Run all queries
    for &(query_name, query_text) in queries {
        print!(" Query: {} ... ", query_name);
        let input = {
            let mut file = BufReader::new(fs::File::open(json_path).expect("Fail @ open File"));
            let mut buf = vec![];
            file.read_to_end(&mut buf).expect("Fail @ file read");
            OwnedBytes::new(buf)
        };
        let query = rsonpath_syntax::parse(query_text).expect("Fail @ parse query");

        // Query normally and skip iteratively (ITE)
        let mut engine = RsonpathEngine::compile_query(&query).expect("Fail @ compile query");
        let result = engine.count(&input).expect("Failed to run query normally");

        // Query normally and skip using the lookup table (LUT)
        engine.add_lut(lut);
        let lut_result = engine.count(&input).expect("LUT: Failed to run query normally");

        if lut_result != result {
            println!("Found {}, Expected {}", lut_result, result);
        } else {
            println!("Correct");
        }

        lut = engine.take_lut().expect("Failed to retrieve LUT from engine");
    }

    // TEST BUILD AGAIN
    println!("Testing keys ...");
    let (keys, values) = pair_finder::get_keys_and_values(json_path).expect("Fail @ finding pairs.");
    for (i, key) in keys.iter().enumerate() {
        let value = lut.get(key).expect("Fail at getting value.");
        let value_hash = lut_hash_map.get(key).expect("Fail at getting value.");
        if value != values[i] || value != value_hash {
            count_incorrect += 1;
            println!(
                "  i: {}, Key {}, Value {}, Expected: {}, Hash {}",
                i, key, value, values[i], value_hash
            );
        }
    }
    println!(" Correct {}/{}", keys.len() - count_incorrect, keys.len());
    println!(" Incorrect {}/{}", count_incorrect, keys.len());
}
