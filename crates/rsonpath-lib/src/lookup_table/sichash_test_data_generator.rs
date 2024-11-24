use std::{
    collections::VecDeque,
    fs::{self, File},
};

use crate::{
    classification::{
        self,
        simd::Simd,
        structural::{BracketType, Structural, StructuralIterator},
    },
    input::{self, Input},
    lookup_table::util_path,
    result::empty::EmptyRecorder,
    FallibleIterator,
};

pub const SICHASH_DATA_DIR: &str = "sichash_data";

#[inline]
pub fn generate_test_data_for_sichash(dir_path: &str, csv_path: &str) {
    let dir = fs::read_dir(dir_path).expect("Failed to read directory");

    println!("Generating test data for sichash:");
    for file in dir {
        let file = file.expect("Failed to get entry");
        let path = file.path();

        if path.is_file() {
            if let Some(extension) = path.extension() {
                if extension == "json" {
                    let json_path = path.to_str().expect("Failed to convert path to string");
                    println!("  - {}", json_path);
                    generate_csv(json_path, csv_path);
                }
            }
        }
    }
}

fn generate_csv(json_path: &str, csv_path: &str) {
    let file = std::fs::File::open(json_path).expect("Fail to open file");
    let filename = util_path::extract_filename(json_path);

    // SAFETY: We keep the file open throughout the entire duration.
    let input = unsafe { input::MmapInput::map_file(&file).expect("Failed to map file") };
    let simd_c = classification::simd::configure();

    let (keys, values) = classification::simd::config_simd!(simd_c => |simd| {
        classification::simd::dispatch_simd!(simd; input, simd => fn<I, V>(
            input: I,
            simd: V,
        ) -> (Vec<usize>, Vec<usize>) where
        I: Input,
        V: Simd,{
                collect_keys_and_values::<I, V>(&input, simd)
            })
    });

    // Save in CSV: First column = keys, second column = values
    let path = format!(
        "{}/{}/{}_{}_sichash_data.csv",
        csv_path,
        SICHASH_DATA_DIR,
        filename,
        keys.len()
    );
    let mut wtr = csv::Writer::from_writer(File::create(&path).expect("Failed to create CSV file"));
    wtr.write_record(["keys", "values"]).expect("Failed @ header line");

    for (key, value) in keys.iter().zip(values.iter()) {
        wtr.write_record(&[key.to_string(), value.to_string()]).expect("@data");
    }

    wtr.flush().expect("Failed to flush CSV writer");
}

fn collect_keys_and_values<I, V>(input: &I, simd: V) -> (Vec<usize>, Vec<usize>)
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

    // Data structure to track frequency of distances
    let mut keys = vec![];
    let mut values = vec![];

    while let Some(event) = structural_classifier
        .next()
        .expect("Failed to get next structural event")
    {
        match event {
            Structural::Opening(b, idx_open) => match b {
                BracketType::Square => square_bracket_stack.push_back(idx_open),
                BracketType::Curly => curly_bracket_stack.push_back(idx_open),
            },
            Structural::Closing(b, idx_close) => match b {
                BracketType::Curly => {
                    let idx_open = curly_bracket_stack.pop_back().expect("Unmatched }");
                    keys.push(idx_open);
                    values.push(idx_close - idx_open);
                }
                BracketType::Square => {
                    let idx_open = square_bracket_stack.pop_back().expect("Unmatched ]");
                    keys.push(idx_open);
                    values.push(idx_close - idx_open);
                }
            },
            Structural::Colon(_) | Structural::Comma(_) => unreachable!(),
        }
    }

    (keys, values)
}
