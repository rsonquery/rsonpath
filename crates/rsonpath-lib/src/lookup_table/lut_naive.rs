use serde::{Deserialize, Serialize};
use serde_cbor;
use serde_json;
use std::collections::HashMap;
use std::collections::VecDeque;
use std::fs::File;
use std::io::{Error, ErrorKind, Read, Write};

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

use crate::lookup_table::util_path;

#[derive(Serialize, Deserialize, Debug)]
pub struct LutNaive {
    table: HashMap<usize, usize>,
}

impl LutNaive {
    #[inline]
    pub fn build_with_json(file: &File) -> Result<Self, Box<dyn std::error::Error>> {
        // SAFETY: We keep the file open throughout the entire duration.
        let input = unsafe { input::MmapInput::map_file(file)? };
        let simd_c = classification::simd::configure();

        classification::simd::config_simd!(simd_c => |simd| {
            classification::simd::dispatch_simd!(simd; input, simd => fn<I, V>(
                input: I,
                simd: V,
            ) -> Result<LutNaive, error::InputError> where
            I: Input,
            V: Simd,{
                    LutNaive::find_pairs_and_build_lut::<I, V>(&input, simd)
                })
        })
        .map_err(|e| Box::new(e) as Box<dyn std::error::Error>)
    }

    #[inline]
    #[must_use]
    pub fn init(start_capacity: Option<usize>) -> Self {
        let size = start_capacity.unwrap_or(0);
        Self {
            table: HashMap::with_capacity(size),
        }
    }

    #[inline]
    pub fn put(&mut self, key: usize, value: usize) {
        self.table.insert(key, value);
    }

    #[inline]
    #[must_use]
    pub fn get(&self, key: &usize) -> Option<usize> {
        self.table.get(key).copied()
    }

    #[inline]
    #[must_use]
    pub fn get_keys(&self) -> Vec<usize> {
        self.table.keys().copied().collect()
    }

    #[inline]
    pub fn serialize(&self, path: &str) -> std::io::Result<()> {
        let serialized_data = match util_path::get_filetype_from_path(path).as_str() {
            "json" => serde_json::to_vec(&self).expect("Serialize failed."),
            "cbor" => serde_cbor::to_vec(&self).expect("Serialize failed."),
            _ => {
                return Err(std::io::Error::new(
                    ErrorKind::InvalidInput,
                    "Serialize: Unsupported format",
                ))
            }
        };
        let mut file = File::create(path)?;
        file.write_all(&serialized_data)?;
        Ok(())
    }

    #[inline]
    pub fn deserialize(path: &str) -> std::io::Result<Self> {
        let mut file = File::open(path)?;
        let mut contents = Vec::new();
        file.read_to_end(&mut contents)?;
        let deserialized: Self = match util_path::get_filetype_from_path(path).as_str() {
            "json" => serde_json::from_slice(&contents).expect("Deserialize: Data has no JSON format."),
            "cbor" => serde_cbor::from_slice(&contents).expect("Deserialize: Data has no CBOR format."),
            _ => return Err(Error::new(ErrorKind::InvalidInput, "Deserialize: Unsupported format")),
        };
        Ok(deserialized)
    }

    #[inline]
    #[must_use]
    pub fn estimate_json_size(&self) -> usize {
        if !self.table.is_empty() {
            return serde_json::to_vec(&self).expect("Failed to serialize to JSON.").len();
        }

        println!("The table is empty.");
        0
    }

    #[inline]
    #[must_use]
    pub fn estimate_cbor_size(&self) -> usize {
        if !self.table.is_empty() {
            return serde_cbor::to_vec(&self).expect("Failed to serialize to JSON.").len();
        }

        println!("The table is empty.");
        0
    }

    #[inline]
    pub fn overview(&self) {
        if !self.table.is_empty() {
            println!("lut-naive Overview:");
            println!("  #Entries: {}", self.table.len());
            println!("  Capacity: {}", self.table.capacity());

            // Serialize to JSON and CBOR to estimate file sizes
            println!("  CBOR: {} bytes", self.estimate_cbor_size());
            println!("  JSON: {} bytes", self.estimate_json_size());

            // Calculate and print the average, maximum, and minimum of (value - key) called the distance
            let mut total_distance = 0_usize;
            let mut max_distance = usize::MIN;
            let mut min_distance = usize::MAX;
            for (key, value) in &self.table {
                let distance = (*value).saturating_sub(*key); // Ensures non-negative distances
                total_distance += distance;
                max_distance = max_distance.max(distance);
                min_distance = min_distance.min(distance);
            }
            let average_distance = total_distance as f64 / self.table.len() as f64;

            println!("  Average distance (value - key): {:.2}", average_distance);
            println!("  MAX distance (value - key): {}", max_distance);
            println!("  MIN distance (value - key): {}", min_distance);

            // Print up to the first 10 pairs
            println!("  First 10 pairs:");
            for (i, (key, value)) in self.table.iter().take(10).enumerate() {
                println!("    {}. Key: {}, Value: {}", i + 1, key, value);
            }
        } else {
            println!("The table is empty.");
        }
    }

    #[inline(always)]
    fn find_pairs_and_build_lut<I, V>(input: &I, simd: V) -> Result<Self, error::InputError>
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
        let mut lut_naive = Self::init(Some(10));

        while let Some(event) = structural_classifier.next()? {
            match event {
                Structural::Opening(b, idx_open) => match b {
                    BracketType::Square => square_bracket_stack.push_back(idx_open),
                    BracketType::Curly => curly_bracket_stack.push_back(idx_open),
                },
                Structural::Closing(b, idx_close) => match b {
                    BracketType::Square => {
                        let idx_open = square_bracket_stack.pop_back().expect("Unmatched closing ]");
                        // println!("[ at index {idx_open} AND ] at index {idx_close}");
                        lut_naive.put(idx_open, idx_close);
                    }
                    BracketType::Curly => {
                        let idx_open = curly_bracket_stack.pop_back().expect("Unmatched closing }");
                        // println!("{{ at index {idx_open} AND }} at index {idx_close}");
                        lut_naive.put(idx_open, idx_close);
                    }
                },
                Structural::Colon(_) | Structural::Comma(_) => unreachable!(),
            }
        }

        Ok(lut_naive)
    }
}
