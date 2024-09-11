use std::collections::VecDeque;
use std::fs::File;

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

#[derive(Clone)]
pub enum Entry {
    Number(usize),
    Bucket(Bucket),
}
#[derive(Clone)]
pub struct LutPerfectNaive {
    buckets: Vec<Option<Entry>>,
    size: usize,
}

impl LutPerfectNaive {
    pub fn init(keys: Vec<usize>, values: Vec<usize>) -> Self {
        let size = keys.len() * 2;
        let mut helper_table = vec![vec![]; size];

        for (key, value) in keys.iter().zip(values.iter()) {
            let hash = key % size;
            helper_table[hash].push((*key, *value));
        }

        let mut buckets = vec![None; size];

        for (i, sub_table) in helper_table.into_iter().enumerate() {
            if !sub_table.is_empty() {
                if sub_table.len() == 1 {
                    let (_key, value) = sub_table[0];
                    buckets[i] = Some(Entry::Number(value));
                } else {
                    let keys: Vec<usize> = sub_table.iter().map(|(k, _)| *k).collect();
                    let values: Vec<usize> = sub_table.iter().map(|(_, v)| *v).collect();
                    buckets[i] = Some(Entry::Bucket(Bucket::new(keys, values)));
                }
            }
        }

        Self { buckets, size }
    }

    pub fn get(&self, key: &usize) -> Option<usize> {
        match &self.buckets[key % self.size] {
            Some(Entry::Number(v)) => Some(*v),
            Some(Entry::Bucket(bucket)) => bucket.get(key),
            None => None,
        }
    }
}

#[derive(Clone)]
pub struct Bucket {
    elements: Vec<Option<usize>>,
    size: usize,
}

impl Bucket {
    pub fn new(keys: Vec<usize>, values: Vec<usize>) -> Self {
        let mut size = keys.len() * 2;

        let elements = loop {
            let mut arr = vec![None; size];
            let mut collision = false;

            for (key, value) in keys.iter().zip(values.iter()) {
                let hash = key % size;
                if arr[hash].is_some() {
                    collision = true;
                    break;
                }
                arr[hash] = Some(*value);
            }

            if !collision {
                break arr;
            }
            size *= 2;
        };

        Self { elements, size }
    }

    pub fn get(&self, key: &usize) -> Option<usize> {
        let hash = key % self.size;
        self.elements[hash]
    }
}

#[inline]
pub fn build(file: &File) -> Result<LutPerfectNaive, Box<dyn std::error::Error>> {
    // SAFETY: We keep the file open throughout the entire duration.
    let input = unsafe { input::MmapInput::map_file(file)? };
    let simd_c = classification::simd::configure();

    classification::simd::config_simd!(simd_c => |simd| {
        classification::simd::dispatch_simd!(simd; input, simd => fn<I, V>(
            input: I,
            simd: V,
        ) -> Result<LutPerfectNaive, error::InputError> where
        I: Input,
        V: Simd,{
                fill::<I, V>(&input, simd)
            })
    })
    .map_err(|e| Box::new(e) as Box<dyn std::error::Error>)
}

#[inline(always)]
fn fill<I, V>(input: &I, simd: V) -> Result<LutPerfectNaive, error::InputError>
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
    let mut keys = vec![];
    let mut values = vec![];

    while let Some(event) = structural_classifier.next()? {
        match event {
            Structural::Opening(b, idx_open) => match b {
                BracketType::Square => square_bracket_stack.push_back(idx_open),
                BracketType::Curly => curly_bracket_stack.push_back(idx_open),
            },
            Structural::Closing(b, idx_close) => match b {
                BracketType::Square => {
                    let idx_open = square_bracket_stack.pop_back().expect("Unmatched closing ]");
                    keys.push(idx_open);
                    values.push(idx_close);
                }
                BracketType::Curly => {
                    let idx_open = curly_bracket_stack.pop_back().expect("Unmatched closing }");
                    keys.push(idx_open);
                    values.push(idx_close);
                }
            },
            Structural::Colon(_) | Structural::Comma(_) => unreachable!(),
        }
    }

    Ok(LutPerfectNaive::init(keys, values))
}
