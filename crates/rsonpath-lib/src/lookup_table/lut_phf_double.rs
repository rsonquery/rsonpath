use super::{
    lut_phf::{
        phf_generator_double_hash::{self, HashState},
        phf_shared,
    },
    LookUpTable,
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
use rand::{distributions::Standard, rngs::SmallRng};
use rand::{Rng, SeedableRng};
use std::fs::File;
use std::{
    collections::{HashMap, VecDeque},
    fs,
};

pub struct LutPHFDouble {
    pub hash_state_16: HashState,
    pub hash_state_64: HashState,
}

const THRESHOLD_16_BITS: usize = 65536; // = 2^16

impl LookUpTable for LutPHFDouble {
    #[inline]
    fn build(json_path: &str) -> Result<Self, Box<dyn std::error::Error>> {
        // SAFETY: We keep the file open throughout the entire duration.
        let file = fs::File::open(json_path).expect("Failed to open file");
        let input = unsafe { input::MmapInput::map_file(&file)? };
        let simd_c = classification::simd::configure();

        let lut_perfect_naive = classification::simd::config_simd!(simd_c => |simd| {
            classification::simd::dispatch_simd!(simd; input, simd => fn<I, V>(
                input: I,
                simd: V,
            ) -> Result<LutPHFDouble, error::InputError> where
            I: Input,
            V: Simd,{
                    let (keys_16, values_16, keys_64, values_64) = LutPHFDouble::find_all_pairs::<I, V>(&input, simd)?;
                    Ok(LutPHFDouble::build_double(keys_16, values_16, keys_64, values_64))
                })
        });
        lut_perfect_naive.map_err(|e| Box::new(e) as Box<dyn std::error::Error>)
    }

    fn get(&self, key: &usize) -> Option<usize> {
        // Hashmap u16
        if let Some(value_16) = self.hash_state_16.get(key) {
            if value_16 != 0 {
                return Some(key + value_16);
            }
        }

        // Hashmap usize
        if let Some(value_64) = self.hash_state_64.get(key) {
            return Some(key + value_64);
        }

        None
    }
}

impl LutPHFDouble {
    #[inline]
    #[must_use]
    pub fn build_double(
        keys_16: Vec<usize>,
        mut values_16: Vec<u16>,
        keys_64: Vec<usize>,
        values_64: Vec<usize>,
    ) -> Self {
        // Build hash_state for the values of size u16
        let mut hash_state_16 = SmallRng::seed_from_u64(phf_generator_double_hash::FIXED_SEED)
            .sample_iter(Standard)
            .find_map(|hash_key| phf_generator_double_hash::try_generate_hash(&keys_16, hash_key))
            .expect("failed to solve PHF");

        // 1) Find conflicts and set conflict positions to 0 in `values_16`, save (index, value) in conflict_indexes
        let mut conflict_indexes: HashMap<usize, u16> = HashMap::with_capacity(keys_64.len());
        for key_64 in &keys_64 {
            let idx = hash_state_16
                .get(key_64)
                .expect("Fail @ getting idx from hash_state_16");

            conflict_indexes.insert(idx, values_16[idx]);
            values_16[idx] = 0; // Set conflict position to 0
        }

        // 2) Init conflict_keys and conflict_values
        let mut conflict_keys: Vec<usize> = keys_64.clone();
        let mut conflict_values: Vec<usize> = values_64.clone();

        // 3) Collect all conflict keys and values from keys_16
        for key_16 in &keys_16 {
            let idx = hash_state_16
                .get(key_16)
                .expect("Fail @ getting idx from hash_state_16");
            let value_16 = values_16[idx];

            if value_16 == 0 {
                // Found conflict
                conflict_keys.push(*key_16);
                conflict_values.push(conflict_indexes[&idx].into());
            }
        }

        // 4) Build hast_state_64
        let hash_state_64 = Self::build_single(conflict_keys, conflict_values);

        // 5) Replace indexes with values
        // TODO: fix the mapping of usize to u16 -> make HashState generic to support usize and u16
        hash_state_16.map = hash_state_16.map.iter().map(|&idx| values_16[idx].into()).collect();

        LutPHFDouble {
            hash_state_16,
            hash_state_64,
        }
    }

    fn build_single(keys: Vec<usize>, values: Vec<usize>) -> HashState {
        let mut hash_state = SmallRng::seed_from_u64(phf_generator_double_hash::FIXED_SEED)
            .sample_iter(Standard)
            .find_map(|hash_key| phf_generator_double_hash::try_generate_hash(&keys, hash_key))
            .expect("failed to solve PHF");

        // Replace indexes with values
        hash_state.map = hash_state.map.iter().map(|&idx| values[idx].into()).collect();

        hash_state
    }

    /// We count the distances between the opening and closing brackets. We save the start position as key and
    /// distance to the closing bracket in the value. Creates a key-value list for values which fit in a 16 bit
    /// representation and another key-value list for the ones that do not.
    fn find_all_pairs<I, V>(
        input: &I,
        simd: V,
    ) -> Result<(Vec<usize>, Vec<u16>, Vec<usize>, Vec<usize>), error::InputError>
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

        // keys[i] and values[i] form a pair
        let mut keys_16: Vec<usize> = vec![];
        let mut values_16: Vec<u16> = vec![];
        let mut keys_64: Vec<usize> = vec![];
        let mut values_64: Vec<usize> = vec![];

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
                        keys_16.push(idx_open);
                        values_16.push(distance.try_into().unwrap());
                    } else {
                        keys_64.push(idx_open);
                        values_64.push(distance);
                    }
                }
                Structural::Colon(_) | Structural::Comma(_) => unreachable!(),
            }
        }

        Ok((keys_16, values_16, keys_64, values_64))
    }
}
