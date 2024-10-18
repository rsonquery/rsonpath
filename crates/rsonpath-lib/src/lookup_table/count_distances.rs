use std::{
    collections::{HashMap, VecDeque},
    fs::{self, File},
    io::{self, Write},
    process::Command,
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

pub const DISTANCE_EVAL_DIR: &str = "distance_distribution";

/// Count the distances for each json file of the given directory
pub fn count_distances_in_dir(dir_path: &str, csv_path: &str) {
    let dir = fs::read_dir(dir_path).expect("Failed to read directory");

    println!("Counting Distances:");
    for file in dir {
        let file = file.expect("Failed to get entry");
        let path = file.path();

        if path.is_file() {
            if let Some(extension) = path.extension() {
                if extension == "json" {
                    let json_path = path.to_str().expect("Failed to convert path to string");
                    println!("  Processing: {}", json_path);
                    count_distances_with_simd(json_path, csv_path);
                }
            }
        }
    }
}

fn count_distances_with_simd(json_path: &str, csv_path: &str) {
    let file = std::fs::File::open(json_path).expect("Fail to open file");
    let filename = util_path::extract_filename(json_path);

    // SAFETY: We keep the file open throughout the entire duration.
    let input = unsafe { input::MmapInput::map_file(&file).expect("Failed to map file") };
    let simd_c = classification::simd::configure();

    let distance_frequencies = classification::simd::config_simd!(simd_c => |simd| {
        classification::simd::dispatch_simd!(simd; input, simd => fn<I, V>(
            input: I,
            simd: V,
        ) -> HashMap<usize, usize> where
        I: Input,
        V: Simd,{
                count_distances::<I, V>(&input, simd)
            })
    });

    // Save in CSV: First column = distance, second column = frequency
    let path = format!("{}/{}/{}_distances.csv", csv_path, DISTANCE_EVAL_DIR, filename);
    let mut wtr = csv::Writer::from_writer(File::create(&path).expect("Failed to create CSV file"));
    wtr.write_record(&["distance", "frequency"])
        .expect("Failed to write CSV header");
    for (distance, frequency) in distance_frequencies {
        wtr.write_record(&[distance.to_string(), frequency.to_string()])
            .expect("Failed to write record");
    }
    wtr.flush().expect("Failed to flush CSV writer");

    // Plot it with python
    run_python_statistics_builder(&path);
}

fn count_distances<I, V>(input: &I, simd: V) -> HashMap<usize, usize>
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
    let mut distance_frequencies: HashMap<usize, usize> = HashMap::new();

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
                BracketType::Square => {
                    let idx_open = square_bracket_stack.pop_back().expect("Unmatched closing ]");
                    let distance = idx_close - idx_open;
                    *distance_frequencies.entry(distance).or_insert(0) += 1;
                }
                BracketType::Curly => {
                    let idx_open = curly_bracket_stack.pop_back().expect("Unmatched closing }");
                    let distance = idx_close - idx_open;
                    *distance_frequencies.entry(distance).or_insert(0) += 1;
                }
            },
            Structural::Colon(_) | Structural::Comma(_) => unreachable!(),
        }
    }

    distance_frequencies
}

fn run_python_statistics_builder(csv_path: &str) {
    let output = Command::new("python")
        .arg("crates/rsonpath-lib/src/lookup_table/python_statistic/count_distances.py")
        .arg(csv_path)
        .output()
        .expect(&format!("Failed to open csv_path: {}", csv_path));

    if output.status.success() {
        if let Err(e) = io::stdout().write_all(&output.stdout) {
            eprintln!("Failed to write stdout: {}", e);
        }
    } else {
        eprintln!("Error: {}", String::from_utf8_lossy(&output.stderr));
    }
}
