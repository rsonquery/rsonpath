use super::LookUpTable;
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

// 65536 = 2^16, since we want to consider all values that fit into a 16 bit representation
pub const THRESHOLD_16_BITS: usize = 65536;

pub struct LutHashMapDouble {
    pub hash_map_16: HashMap<usize, u16>,
    pub hash_map_64: HashMap<usize, usize>,
}

impl LookUpTable for LutHashMapDouble {
    #[inline]
    fn build(json_path: &str) -> Result<Self, Box<dyn std::error::Error>> {
        let file = fs::File::open(json_path).expect("Failed to open file");
        // SAFETY: We keep the file open throughout the entire duration.
        let input = unsafe { input::MmapInput::map_file(&file)? };
        let simd_c = classification::simd::configure();

        let lut_phf_double = classification::simd::config_simd!(simd_c => |simd| {
            classification::simd::dispatch_simd!(simd; input, simd => fn<I, V>(
                input: I,
                simd: V,
            ) -> Result<LutHashMapDouble, error::InputError> where
            I: Input,
            V: Simd, {
                    let pair_data = LutHashMapDouble::find_all_pairs::<I, V>(&input, simd)?;
                    Ok(LutHashMapDouble::build_double(pair_data))
                })
        });
        lut_phf_double.map_err(|e| Box::new(e) as Box<dyn std::error::Error>)
    }

    #[inline]
    fn get(&self, key: &usize) -> Option<usize> {
        // Look for a value for a given key, search first in hash_map_16 since it represents usually >99% of the keys
        // and only on the rare cases of key misses we have to look into hash_map_64 which covers the rest.
        if let Some(&value_16) = self.hash_map_16.get(key) {
            Some(*key + value_16 as usize)
        } else if let Some(&value_64) = self.hash_map_64.get(key) {
            Some(*key + value_64)
        } else {
            // Neither map contains the key which should never happen because we added all keys and values at build
            println!("Ups! You asked for a key that is not in the LutHashMap. Key = {}", key);
            None
        }
    }

    #[inline]
    fn allocated_bytes(&self) -> usize {
        let mut total_size = 0;

        total_size += std::mem::size_of::<Self>();
        total_size += self.hash_map_16.capacity() * (std::mem::size_of::<usize>() + std::mem::size_of::<u16>());
        total_size += self.hash_map_64.capacity() * (std::mem::size_of::<usize>() + std::mem::size_of::<usize>());

        total_size
    }
}

impl LutHashMapDouble {
    pub fn build_double(pd: PairData) -> Self {
        let hash_map_16: HashMap<usize, u16> = pd.keys_16.into_iter().zip(pd.values_16.into_iter()).collect();
        let hash_map_64: HashMap<usize, usize> = pd.keys_64.into_iter().zip(pd.values_64.into_iter()).collect();

        Self {
            hash_map_16,
            hash_map_64,
        }
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

        // Initialize two empty stacks: one for "[" and one for "{", to remember the order we have found them
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

/// Helper struct, because it makes the code shorter and cleaner to read.
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
