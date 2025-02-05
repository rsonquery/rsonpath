use super::LookUpTable;
use crate::debug;
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
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::collections::VecDeque;
use std::fs;

#[derive(Serialize, Deserialize, Debug)]
pub struct LutHashMap {
    hash_map: HashMap<usize, usize>,
}

impl LookUpTable for LutHashMap {
    #[inline]
    fn build(json_path: &str) -> Result<Self, Box<dyn std::error::Error>> {
        let file = fs::File::open(json_path).expect("Failed to open file");
        // SAFETY: We keep the file open throughout the entire duration.
        let input = unsafe { input::MmapInput::map_file(&file)? };
        let simd_c = classification::simd::configure();

        classification::simd::config_simd!(simd_c => |simd| {
            classification::simd::dispatch_simd!(simd; input, simd => fn<I, V>(
                input: I,
                simd: V,
            ) -> Result<LutHashMap, error::InputError> where
            I: Input,
            V: Simd,{
                    let (keys, values) = LutHashMap::find_all_pairs::<I, V>(&input, simd)?;
                    let hash_map: HashMap<usize, usize> = keys.into_iter().zip(values.into_iter()).collect();
                    Ok(LutHashMap{ hash_map })
                })
        })
        .map_err(|e| Box::new(e) as Box<dyn std::error::Error>)
    }

    #[inline]
    #[must_use]
    fn get(&self, key: &usize) -> Option<usize> {
        self.hash_map.get(key).copied()
    }

    #[inline]
    fn allocated_bytes(&self) -> usize {
        let mut total_size = 0;
        total_size += std::mem::size_of::<Self>();
        total_size += self.hash_map.capacity() * (std::mem::size_of::<usize>() + std::mem::size_of::<usize>());
        total_size
    }
}

impl LutHashMap {
    #[inline]
    pub(crate) fn find_all_pairs<I, V>(input: &I, simd: V) -> Result<(Vec<usize>, Vec<usize>), error::InputError>
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

                    // TODO this is only needed for experiments
                    // let distance = idx_close - idx_open;
                    // if distance > 200 {
                    keys.push(idx_open);
                    values.push(idx_close);
                    // }
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
}
