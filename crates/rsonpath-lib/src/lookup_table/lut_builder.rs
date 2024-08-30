use std::collections::VecDeque;
use std::error::Error;
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

use crate::lookup_table::lut_naive;

#[inline]
pub fn run(file: &File) -> Result<(), Box<dyn Error>> {
    // SAFETY: We keep the file open throughout the entire duration.
    let input = unsafe { input::MmapInput::map_file(file)? };
    let simd_c = classification::simd::configure();

    classification::simd::config_simd!(simd_c => |simd| {
        classification::simd::dispatch_simd!(simd; input, simd => fn<I, V>(
            input: I,
            simd: V,
        ) -> Result<(), error::InputError> where
        I: Input,
        V: Simd,{
                run_impl::<I, V>(&input, simd)
            })
    })?;

    Ok(())
}

fn extract_filename_from_path(path: &str) -> String {
    let path = std::path::Path::new(path);
    let filename = path.file_stem().expect("Failed to extract filename");
    filename.to_string_lossy().into_owned()
}

#[inline(always)]
fn run_impl<I, V>(input: &I, simd: V) -> Result<(), error::InputError>
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
    let mut lut_naive = lut_naive::LutNaive::init(Some(10));

    while let Some(event) = structural_classifier.next()? {
        match event {
            Structural::Opening(b, idx_open) => match b {
                BracketType::Square => square_bracket_stack.push_back(idx_open),
                BracketType::Curly => curly_bracket_stack.push_back(idx_open),
            },
            Structural::Closing(b, idx_close) => match b {
                BracketType::Square => {
                    let idx_open = square_bracket_stack.pop_back().expect("Unmatched closing ]");
                    println!("[ at index {idx_open} AND ] at index {idx_close}");
                    lut_naive.put(idx_open, idx_close);
                }
                BracketType::Curly => {
                    let idx_open = curly_bracket_stack.pop_back().expect("Unmatched closing }");
                    println!("{{ at index {idx_open} AND }} at index {idx_close}");
                    lut_naive.put(idx_open, idx_close);
                }
            },
            Structural::Colon(_) | Structural::Comma(_) => unreachable!(),
        }
    }

    let filename = "test_a";
    lut_naive.overview();
    // lut_naive.serialize(&format!(".a_ricardo/output/{}.json", filename))?;
    // lut_naive.serialize(&format!(".a_ricardo/output/{}.cbor", filename))?;

    Ok(())
}
