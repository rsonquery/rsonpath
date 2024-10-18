use super::lut_phf_double::LutPHFDouble;
use super::{lut_phf_double::THRESHOLD_16_BITS, LookUpTable};
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
use itertools::izip;
use std::{collections::VecDeque, fs};

const BIT_MASK: usize = 0xF; // Keeps the lower 4 bit

pub struct LutPHFGroup {
    pub buckets: Vec<LutPHFDouble>, // always size = BUCKETS_SIZE
}

impl LookUpTable for LutPHFGroup {
    #[inline]
    fn build(json_path: &str) -> Result<Self, Box<dyn std::error::Error>> {
        let file = fs::File::open(json_path).expect("Failed to open file");
        // SAFETY: We keep the file open throughout the entire duration.
        let input = unsafe { input::MmapInput::map_file(&file)? };
        let simd_c = classification::simd::configure();

        let lut_perfect_naive = classification::simd::config_simd!(simd_c => |simd| {
            classification::simd::dispatch_simd!(simd; input, simd => fn<I, V>(
                input: I,
                simd: V,
            ) -> Result<LutPHFGroup, error::InputError> where
            I: Input,
            V: Simd,{
                let (bucket_keys_16, bucket_values_16, bucket_keys_64, bucket_values_64) =
                    LutPHFGroup::find_all_pairs::<I, V>(&input, simd)?;
                Ok(LutPHFGroup::build_buckets(bucket_keys_16, bucket_values_16, bucket_keys_64, bucket_values_64))
            })
        });
        lut_perfect_naive.map_err(|e| Box::new(e) as Box<dyn std::error::Error>)
    }

    #[inline]
    fn get(&self, key: &usize) -> Option<usize> {
        let bucket_index = key & BIT_MASK; // Logical AND with BIT_MASK to get the bucket index
        self.buckets[bucket_index].get(key)
    }
}

impl LutPHFGroup {
    fn build_buckets(
        bucket_keys_16: Vec<Vec<usize>>,
        bucket_values_16: Vec<Vec<u16>>,
        bucket_keys_64: Vec<Vec<usize>>,
        bucket_values_64: Vec<Vec<usize>>,
    ) -> Self {
        let buckets = izip!(bucket_keys_16, bucket_values_16, bucket_keys_64, bucket_values_64)
            .map(|(keys_16, values_16, keys_64, values_64)| {
                LutPHFDouble::build_double(keys_16, values_16, keys_64, values_64)
            })
            .collect();

        LutPHFGroup { buckets }
    }

    fn find_all_pairs<I, V>(
        input: &I,
        simd: V,
    ) -> Result<(Vec<Vec<usize>>, Vec<Vec<u16>>, Vec<Vec<usize>>, Vec<Vec<usize>>), error::InputError>
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

        // Create for each bucket a keys_16, values_16, keys_64, values_64. Init them as empty vectors
        let mut bucket_keys_16: Vec<Vec<usize>> = vec![vec![]; 16];
        let mut bucket_values_16: Vec<Vec<u16>> = vec![vec![]; 16];
        let mut bucket_keys_64: Vec<Vec<usize>> = vec![vec![]; 16];
        let mut bucket_values_64: Vec<Vec<usize>> = vec![vec![]; 16];

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

                    let bucket_index = idx_open & BIT_MASK; // AND with BIT_MASK to get the bucket index

                    let distance = idx_close - idx_open;
                    if distance < THRESHOLD_16_BITS {
                        // Can fit into 16 bits
                        bucket_keys_16[bucket_index].push(idx_open);
                        bucket_values_16[bucket_index]
                            .push(distance.try_into().expect("Failed to convert distance to 16 bits"));
                    } else {
                        // Needs 64 bits
                        bucket_keys_64[bucket_index].push(idx_open);
                        bucket_values_64[bucket_index].push(distance);
                    }
                }
                Structural::Colon(_) | Structural::Comma(_) => unreachable!(),
            }
        }

        Ok((bucket_keys_16, bucket_values_16, bucket_keys_64, bucket_values_64))
    }
}
