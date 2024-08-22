use std::{error::Error, fs::File};

use crate::{
    classification::{
        self,
        simd::Simd,
        structural::{BracketType, Structural, StructuralIterator},
    },
    input::{self, Input},
    result::empty::EmptyRecorder,
    FallibleIterator,
};

#[inline]
pub fn run(file: &File) -> Result<(), Box<dyn Error>> {
    let input = unsafe { input::MmapInput::map_file(file)? };
    let simd_c = classification::simd::configure();

    classification::simd::config_simd!(simd_c => |simd| {
            let iter = input.iter_blocks(&EmptyRecorder);
            let quote_classifier = simd.classify_quoted_sequences(iter);
            let mut structural_classifier = simd.classify_structural_characters(quote_classifier);
            structural_classifier.turn_colons_and_commas_off();

            while let Some(event) = structural_classifier.next()? {
                match event {
                    Structural::Closing(b, idx) => {
                        match b {
                            BracketType::Square => println!("] at index {idx}"),
                            BracketType::Curly => println!("}} at index {idx}"),
                        }
                    },
                    Structural::Opening(b, idx) =>{
                        match b {
                            BracketType::Square => println!("[ at index {idx}"),
                            BracketType::Curly => println!("{{ at index {idx}"),
                        }
                    },
                    Structural::Colon(_) => unreachable!(),
                    Structural::Comma(_) => unreachable!(),
                }
            }

            Ok::<_, input::error::InputError>(())
        })?;

    Ok(())
}

/*

        classification::simd::dispatch_simd!(simd; input => fn<I, V>(input: I) -> Result<(), Box<dyn Error>>
        where
            I: input::Input,
            V: Simd {
                Ok(())
            })
*/
