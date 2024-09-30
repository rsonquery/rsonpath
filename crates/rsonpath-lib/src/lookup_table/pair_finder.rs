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

/// key = position of the opening bracket, value = position of the closing bracket
#[inline]
pub fn get_keys_and_values(json_path: &str) -> Result<(Vec<usize>, Vec<usize>), Box<dyn std::error::Error>> {
    // SAFETY: We keep the file open throughout the entire duration.
    let file = std::fs::File::open(json_path).expect("Fail at opening file");
    let input = unsafe { input::MmapInput::map_file(&file)? };
    let simd_c = classification::simd::configure();

    classification::simd::config_simd!(simd_c => |simd| {
        classification::simd::dispatch_simd!(simd; input, simd => fn<I, V>(
            input: I,
            simd: V,
        ) -> Result<(Vec<usize>, Vec<usize>), error::InputError> where
        I: Input,
        V: Simd,{
                find_all_pairs::<I, V>(&input, simd)
            })
    })
    .map_err(|e| Box::new(e) as Box<dyn std::error::Error>)
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
                    BracketType::Square => square_bracket_stack.pop_back().expect("Unmatched closing ]"),
                    BracketType::Curly => curly_bracket_stack.pop_back().expect("Unmatched closing }"),
                };

                keys.push(idx_open);
                values.push(idx_close);
            }
            Structural::Colon(_) | Structural::Comma(_) => unreachable!(),
        }
    }

    Ok((keys, values))
}
