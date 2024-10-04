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
use phf_generator::HashState;
use std::{collections::VecDeque, fs};

use super::LookUpTable;

pub mod phf_generator;
pub mod phf_generator_double_hash;
pub mod phf_shared;

pub struct LutPHF {
    pub hash_state: HashState,
    pub values: Vec<usize>,
}

impl LookUpTable for LutPHF {
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
            ) -> Result<LutPHF, error::InputError> where
            I: Input,
            V: Simd,{
                    let (keys, values) = LutPHF::find_all_pairs::<I, V>(&input, simd)?;
                    Ok(LutPHF::build_with_keys_and_values(keys, values))
                })
        });
        lut_perfect_naive.map_err(|e| Box::new(e) as Box<dyn std::error::Error>)
    }

    #[inline]
    #[must_use]
    fn get(&self, key: &usize) -> Option<usize> {
        self.hash_state
            .get_index(key)
            .and_then(|index| self.values.get(index).map(|&value| key + value))
    }
}

impl LutPHF {
    #[inline]
    #[must_use]
    pub fn build_with_keys_and_values(keys: Vec<usize>, values: Vec<usize>) -> Self {
        let hash_state = phf_generator::generate_hash(&keys);
        Self { hash_state, values }
    }

    fn find_all_pairs<I, V>(input: &I, simd: V) -> Result<(Vec<usize>, Vec<usize>), error::InputError>
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

                    keys.push(idx_open);
                    values.push(idx_close - idx_open);
                }
                Structural::Colon(_) | Structural::Comma(_) => unreachable!(),
            }
        }

        Ok((keys, values))
    }
}
