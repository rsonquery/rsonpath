use super::{
    lut_phf::phf_generator_double_hash::{self, HashState},
    LookUpTable, LookUpTableLambda,
};
use crate::{
    classification::{
        self,
        simd::Simd,
        structural::{BracketType, Structural, StructuralIterator},
    },
    input::{self, error, Input},
    result::empty::EmptyRecorder,
    FallibleIterator,
};
use std::{
    collections::{HashMap, VecDeque},
    fs,
};

pub struct LutPHFDouble {
    pub lambda: usize,
    pub hash_state_16: HashState<u16>,
    pub hash_state_64: HashState<usize>,
}

pub const THRESHOLD_16_BITS: usize = 65536; // = 2^16
pub const BUILD_LAMBDA: usize = 1; // Range = [1, ... , 5]
pub const MAX_LAMBDA: usize = 5; // 5 because the source paper did so

impl LookUpTable for LutPHFDouble {
    #[inline]
    fn build(json_path: &str) -> Result<Self, Box<dyn std::error::Error>> {
        LutPHFDouble::build_with_lambda(BUILD_LAMBDA, json_path)
    }

    #[inline]
    fn get(&self, key: &usize) -> Option<usize> {
        // Hashmap u16
        if let Some(value_16) = self.hash_state_16.get(key) {
            if value_16 != 0 {
                return Some(key + (value_16 as usize));
            }
        }

        // Hashmap usize
        if let Some(value_64) = self.hash_state_64.get(key) {
            return Some(key + value_64);
        }

        None
    }

    #[inline]
    fn allocated_bytes(&self) -> usize {
        let mut total_size = std::mem::size_of::<Self>();
        total_size += self.hash_state_16.allocated_bytes();
        total_size += self.hash_state_64.allocated_bytes();

        total_size
    }
}

impl LookUpTableLambda for LutPHFDouble {
    #[inline]
    fn build_with_lambda(lambda: usize, json_path: &str) -> Result<Self, Box<dyn std::error::Error>> {
        let file = fs::File::open(json_path).expect("Failed to open file");
        // SAFETY: We keep the file open throughout the entire duration.
        let input = unsafe { input::MmapInput::map_file(&file)? };
        let simd_c = classification::simd::configure();

        let lut_phf_double = classification::simd::config_simd!(simd_c => |simd| {
            classification::simd::dispatch_simd!(simd; input, simd, lambda => fn<I, V>(
                input: I,
                simd: V,
                lambda: usize,
            ) -> Result<LutPHFDouble, error::InputError> where
            I: Input,
            V: Simd, {
                    let pair_data = LutPHFDouble::find_all_pairs::<I, V>(&input, simd)?;
                    Ok(LutPHFDouble::build_double(lambda, pair_data))
                })
        });
        lut_phf_double.map_err(|e| Box::new(e) as Box<dyn std::error::Error>)
    }
}

/// Struct to group categorized key-value pairs
#[derive(Clone)]
pub struct PairData {
    pub keys_16: Vec<usize>,
    pub values_16: Vec<u16>,
    pub keys_64: Vec<usize>,
    pub values_64: Vec<usize>,
}

impl PairData {
    pub fn new() -> Self {
        Self {
            keys_16: vec![],
            values_16: vec![],
            keys_64: vec![],
            values_64: vec![],
        }
    }
}

impl LutPHFDouble {
    pub fn build_double(lambda: usize, pair_data: PairData) -> Self {
        // 1) Build hash_state for the values of size u16
        let hash_state_16_usize = phf_generator_double_hash::build(lambda, &pair_data.keys_16);

        // 2) Find conflicts and set conflict positions to 0 in `values_16`, save (index, value) in conflict_indexes
        let mut conflict_indexes: HashMap<usize, u16> = HashMap::with_capacity(pair_data.keys_64.len());
        let mut values_16 = pair_data.values_16.clone();
        for key_64 in &pair_data.keys_64 {
            let idx = hash_state_16_usize
                .get(key_64)
                .expect("Fail @ get idx from hash_state_16");

            conflict_indexes.insert(idx, values_16[idx]);
            values_16[idx] = 0; // Set conflict position to 0
        }

        // 3) Init conflict_keys and conflict_values
        let mut conflict_keys: Vec<usize> = pair_data.keys_64.clone();
        let mut conflict_values: Vec<usize> = pair_data.values_64.clone();

        // 4) Collect all conflict keys and values from keys_16
        for key_16 in &pair_data.keys_16 {
            let idx = hash_state_16_usize
                .get(key_16)
                .expect("Fail @ get idx from hash_state_16");
            let value_16 = values_16[idx];

            if value_16 == 0 {
                // Found conflict
                conflict_keys.push(*key_16);
                conflict_values.push(conflict_indexes[&idx].into());
            }
        }

        // 5) Build hash_state_64
        let hash_state_64 = Self::build_single(lambda, &conflict_keys, &conflict_values);

        // 6) Replace indexes with actual values from values_16
        let map_16: Vec<u16> = hash_state_16_usize.map.into_iter().map(|idx| values_16[idx]).collect();
        let hash_state_16: HashState<u16> = HashState {
            lambda,
            hash_key: hash_state_16_usize.hash_key,
            displacements: hash_state_16_usize.displacements,
            map: map_16,
        };

        Self {
            lambda,
            hash_state_16,
            hash_state_64,
        }
    }

    fn build_single(lambda: usize, keys: &[usize], values: &[usize]) -> HashState<usize> {
        let mut hash_state = phf_generator_double_hash::build(lambda, keys);

        // Replace indexes with values
        hash_state.map = hash_state.map.into_iter().map(|idx| values[idx]).collect();

        hash_state
    }

    /// We count the distances between the opening and closing brackets. We save the start position as key and
    /// distance to the closing bracket in the value. Creates a key-value list for values which fit in a 16 bit
    /// representation and another key-value list for the ones that do not.
    pub fn find_all_pairs<I, V>(input: &I, simd: V) -> Result<PairData, error::InputError>
    where
        I: Input,
        V: Simd,
    {
        let iter = input.iter_blocks::<_, 64>(&EmptyRecorder);
        let quote_classifier = simd.classify_quoted_sequences(iter);
        let mut structural_classifier = simd.classify_structural_characters(quote_classifier);
        structural_classifier.turn_colons_and_commas_off();

        // Initialize two empty stacks: one for "[" and one for "{"
        let mut square_bracket_stack: VecDeque<usize> = VecDeque::new();
        let mut curly_bracket_stack: VecDeque<usize> = VecDeque::new();

        let mut pairs = PairData::new();

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
                    if distance < THRESHOLD_16_BITS {
                        // Can fit into 16 bit
                        pairs.keys_16.push(idx_open);
                        pairs
                            .values_16
                            .push(distance.try_into().expect("Fail at pushing value."));
                    } else {
                        // Cannot fit into 16 bit
                        pairs.keys_64.push(idx_open);
                        pairs.values_64.push(distance);
                    }
                }
                Structural::Colon(_) | Structural::Comma(_) => unreachable!(),
            }
        }

        Ok(pairs)
    }
}
