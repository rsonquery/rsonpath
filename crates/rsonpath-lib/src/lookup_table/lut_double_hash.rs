use crate::{
    classification::{
        self,
        simd::Simd,
        structural::{BracketType, Structural, StructuralIterator},
    },
    input::{self, error, Input},
    lookup_table::util_path,
    result::empty::EmptyRecorder,
    FallibleIterator,
};
use serde::{Deserialize, Serialize};
use serde_cbor;
use serde_json;
use std::collections::HashMap;
use std::collections::VecDeque;
use std::fs::File;
use std::io::{Error, ErrorKind, Read, Write};

use crate::lookup_table::lut_phf::phf_generator;

#[derive(Serialize, Deserialize, Debug)]
pub struct LutDoubleHash {
    table: HashMap<usize, u16>,
}

const THRESHOLD_16_BITS: usize = 65536;

impl LutDoubleHash {
    #[inline]
    pub fn build_with_json(json_file: &File) -> Result<Self, Box<dyn std::error::Error>> {
        // SAFETY: We keep the file open throughout the entire duration.
        let input = unsafe { input::MmapInput::map_file(json_file)? };
        let simd_c = classification::simd::configure();

        let lut_perfect_naive = classification::simd::config_simd!(simd_c => |simd| {
            classification::simd::dispatch_simd!(simd; input, simd => fn<I, V>(
                input: I,
                simd: V,
            ) -> Result<LutDoubleHash, error::InputError> where
            I: Input,
            V: Simd,{
                    let (keys_16, values_16, keys_64, values_64) = LutDoubleHash::find_all_pairs::<I, V>(&input, simd)?;
                    Ok(LutDoubleHash::build_with_keys_and_values(keys_16, values_16, keys_64, values_64))
                })
        });
        lut_perfect_naive.map_err(|e| Box::new(e) as Box<dyn std::error::Error>)
    }

    #[inline]
    #[must_use]
    pub fn build_with_keys_and_values(
        keys_16: Vec<usize>,
        values_16: Vec<u16>,
        keys_64: Vec<usize>,
        values_64: Vec<usize>,
    ) -> Self {
        // TODO
    }

    pub fn get() {
        // TODO
    }

    pub fn print_stats() {
        // TODO
    }

    /// We count the distances between the opening and closing parenthesis. We save the start position as key and
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
