use log::debug;
use smallvec::SmallVec;

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
use std::collections::VecDeque;

// 65536 = 2^16, since we want to consider all values that fit into a 16 bit representation
pub const THRESHOLD_16_BITS: usize = u16::MAX as usize;

/// Helper struct, because it makes the code shorter and cleaner to read.
#[derive(Clone, Default)]
pub struct PairData {
    pub keys: Vec<usize>,
    pub values: Vec<u16>,
    pub keys_64: Vec<usize>,
    pub values_64: Vec<usize>,
}

impl PairData {
    #[inline]
    #[must_use]
    pub fn new() -> Self {
        Self {
            keys: vec![],
            values: vec![],
            keys_64: vec![],
            values_64: vec![],
        }
    }

    #[inline]
    #[must_use]
    pub fn with_capacity(start_capacity: usize, start_capacity_64: usize) -> Self {
        Self {
            keys: Vec::with_capacity(start_capacity),
            values: Vec::with_capacity(start_capacity),
            keys_64: Vec::with_capacity(start_capacity_64),
            values_64: Vec::with_capacity(start_capacity_64),
        }
    }
}

/// We count the distances between the opening and closing brackets. We save the start position as key and
/// distance to the closing bracket in the value. Creates a key-value list for values which fit in a 16 bit
/// representation and another key-value list for the ones that do not.
#[inline]
pub(crate) fn find_pairs<I, V>(input: &I, simd: V, cutoff: usize) -> Result<PairData, error::InputError>
where
    I: Input,
    V: Simd,
{
    let iter = input.iter_blocks::<_, 64>(&EmptyRecorder);
    let quote_classifier = simd.classify_quoted_sequences(iter);
    let mut structural_classifier = simd.classify_structural_characters(quote_classifier);
    structural_classifier.turn_colons_and_commas_off();

    let mut square_bracket_stack: SmallVec<[usize; 64]> = SmallVec::new();
    let mut curly_bracket_stack: SmallVec<[usize; 64]> = SmallVec::new();

    let mut pairs = PairData::new();
    // let mut pairs = PairData::with_capacity(10_000_000, 2000);

    while let Some(event) = structural_classifier.next()? {
        match event {
            Structural::Opening(b, idx_open) => match b {
                BracketType::Square => square_bracket_stack.push(idx_open),
                BracketType::Curly => curly_bracket_stack.push(idx_open),
            },
            Structural::Closing(b, idx_close) => {
                let idx_open = match b {
                    BracketType::Square => square_bracket_stack.pop().expect("Unmatched closing ]"),
                    BracketType::Curly => curly_bracket_stack.pop().expect("Unmatched closing }"),
                };

                let distance = idx_close - idx_open;
                if distance <= cutoff {
                    continue;
                }

                if distance < THRESHOLD_16_BITS {
                    pairs.keys.push(idx_open);
                    pairs.values.push(distance.try_into().expect("Fail at pushing value."));
                } else {
                    pairs.keys.push(idx_open);
                    pairs.values.push(0);
                    pairs.keys_64.push(idx_open);
                    pairs.values_64.push(distance);
                }
            }
            Structural::Colon(_) | Structural::Comma(_) => unreachable!(),
        }
    }

    debug!("Found keys and values:");
    for (key, value) in pairs.keys.iter().zip(pairs.values.iter()) {
        debug!("({}, {})", key, value);
    }
    debug!("Found keys_64 and values_64:");
    for (key_64, value_64) in pairs.keys_64.iter().zip(pairs.values_64.iter()) {
        debug!("({}, {})", key_64, value_64);
    }

    // println!("Sizes: keys= {}, keys64= {}", pairs.keys.len(), pairs.keys_64.len());

    Ok(pairs)
}

// Its basically the same as find_pairs but now it uses a bitmask to create one key,value pair object per bucket
pub(crate) fn find_pairs_buckets<I, V>(
    input: &I,
    simd: V,
    bit_mask: usize,
    cutoff: usize,
) -> Result<Vec<PairData>, error::InputError>
where
    I: Input,
    V: Simd,
{
    let iter = input.iter_blocks::<_, 64>(&EmptyRecorder);
    let quote_classifier = simd.classify_quoted_sequences(iter);
    let mut structural_classifier = simd.classify_structural_characters(quote_classifier);
    structural_classifier.turn_colons_and_commas_off();

    // Initialize a vector of PairData for each lut_double used
    let num_buckets = bit_mask + 1;
    let mut lut_doubles_pair_data: Vec<PairData> = vec![
        PairData {
            keys: vec![],
            values: vec![],
            keys_64: vec![],
            values_64: vec![],
        };
        num_buckets
    ];

    // Stacks for open brackets
    let mut square_bracket_stack: VecDeque<usize> = VecDeque::new();
    let mut curly_bracket_stack: VecDeque<usize> = VecDeque::new();

    while let Some(event) = structural_classifier.next()? {
        match event {
            Structural::Opening(b, idx_open) => match b {
                BracketType::Square => square_bracket_stack.push_back(idx_open),
                BracketType::Curly => curly_bracket_stack.push_back(idx_open),
            },
            Structural::Closing(b, idx_close) => {
                let idx_open = match b {
                    BracketType::Square => square_bracket_stack.pop_back().expect("Unmatched closing ]"),
                    BracketType::Curly => curly_bracket_stack.pop_back().expect("Unmatched closing }"),
                };

                let distance = idx_close - idx_open;
                if distance <= cutoff {
                    continue;
                }

                // Map to correct bucket using the bit mask on the idx_open (= key)
                let lut_double = &mut lut_doubles_pair_data[idx_open & bit_mask];

                if distance < THRESHOLD_16_BITS {
                    // Can fit into 16 bits
                    lut_double.keys.push(idx_open);
                    lut_double
                        .values
                        .push(distance.try_into().expect("Fail @ convert to 16 bit"));
                } else {
                    // Needs 64 bits
                    lut_double.keys_64.push(idx_open);
                    lut_double.values_64.push(distance);
                }
            }
            Structural::Colon(_) | Structural::Comma(_) => unreachable!(),
        }
    }

    // debug!("Found keys and values:");
    // let mut i = 0;
    // for pair_data in &lut_doubles_pair_data {
    //     debug!("bucket:{}", i);
    //     i += 1;
    //     for (key, value) in pair_data.keys.iter().zip(pair_data.values.iter()) {
    //         debug!("({}, {})", key, value);
    //     }
    // }

    Ok(lut_doubles_pair_data)
}

// Almost like find_pairs, but instead of distances we just track absolute positions of the values
#[inline]
pub(crate) fn find_pairs_absolute<I, V>(
    input: &I,
    simd: V,
    cutoff: usize,
) -> Result<(Vec<usize>, Vec<usize>), error::InputError>
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
    let mut keys: Vec<usize> = vec![];
    let mut values: Vec<usize> = vec![];

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

                let distance = idx_close - idx_open;
                if distance <= cutoff {
                    continue;
                }

                if distance > cutoff {
                    keys.push(idx_open);
                    values.push(idx_close);
                }
            }
            Structural::Colon(_) | Structural::Comma(_) => unreachable!(),
        }
    }

    // debug!("Found keys and values:");
    // for (key, value) in keys.iter().zip(values.iter()) {
    //     debug!("({}, {})", key, value);
    // }

    Ok((keys, values))
}

/// Used to check correctness for results of LUT implementations
/// key = position of the opening bracket, value = position of the closing bracket
#[inline]
pub fn get_keys_and_values(
    json_path: &str,
    cutoff: usize,
) -> Result<(Vec<usize>, Vec<usize>), Box<dyn std::error::Error>> {
    let file = std::fs::File::open(json_path).expect("Fail at opening file");
    // SAFETY: We keep the file open throughout the entire duration.
    let input = unsafe { input::MmapInput::map_file(&file)? };
    let simd_c = classification::simd::configure();

    classification::simd::config_simd!(simd_c => |simd| {
        classification::simd::dispatch_simd!(simd; input, simd, cutoff => fn<I, V>(
            input: I,
            simd: V,
            cutoff: usize,
        ) -> Result<(Vec<usize>, Vec<usize>), error::InputError> where
        I: Input,
        V: Simd,{
                find_pairs_absolute::<I, V>(&input, simd, cutoff)
            })
    })
    .map_err(|e| Box::new(e) as Box<dyn std::error::Error>)
}
