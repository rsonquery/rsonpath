use super::lut_hash_map_double::{PairData, THRESHOLD_16_BITS};
use super::lut_phf::DEFAULT_THREADED;
use super::{lut_phf::DEFAULT_LAMBDA, lut_phf_double::LutPHFDouble};
use super::{LookUpTable, LookUpTableLambda};
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
use rayon::prelude::*;
use std::{collections::VecDeque, fs};

// A bit map that only keeps the lower 4 bit because we currently have 16 lut in the group. 16 is represented by 4 bits.
const DEFAULT_BIT_MASK: usize = 0xF;

pub struct LutPHFGroup {
    // Vector length is always 16 at the moment because we did not the add the parameter for it yet
    pub lut_doubles: Vec<LutPHFDouble>,
    pub bit_mask: usize,
}

impl LookUpTable for LutPHFGroup {
    #[inline]
    fn build(json_path: &str) -> Result<Self, Box<dyn std::error::Error>> {
        LutPHFGroup::build_buckets(DEFAULT_LAMBDA, json_path, DEFAULT_BIT_MASK, DEFAULT_THREADED)
    }

    #[inline]
    fn get(&self, key: &usize) -> Option<usize> {
        // Logical AND with BIT_MASK to get the correct index
        let lut_double_index = key & self.bit_mask;
        self.lut_doubles[lut_double_index].get(key)
    }

    #[inline]
    fn allocated_bytes(&self) -> usize {
        let mut total_size = std::mem::size_of::<Self>();
        for lut_double in &self.lut_doubles {
            total_size += lut_double.allocated_bytes();
        }
        total_size
    }
}

impl LookUpTableLambda for LutPHFGroup {
    #[inline]
    fn build_lambda(lambda: usize, json_path: &str, threaded: bool) -> Result<Self, Box<dyn std::error::Error>> {
        LutPHFGroup::build_buckets(lambda, json_path, DEFAULT_BIT_MASK, threaded)
    }
}

impl LutPHFGroup {
    #[inline]
    pub fn build_buckets(
        lambda: usize,
        json_path: &str,
        bit_mask: usize,
        threaded: bool,
    ) -> Result<Self, Box<dyn std::error::Error>> {
        let file = fs::File::open(json_path).expect("Failed to open file");
        // SAFETY: We keep the file open throughout the entire duration.
        let input = unsafe { input::MmapInput::map_file(&file)? };
        let simd_c = classification::simd::configure();

        let lut_perfect_naive = classification::simd::config_simd!(simd_c => |simd| {
            classification::simd::dispatch_simd!(simd; input, simd, lambda, bit_mask, threaded => fn<I, V>(
                input: I,
                simd: V,
                lambda: usize,
                bit_mask: usize,
                threaded: bool,
            ) -> Result<LutPHFGroup, error::InputError> where
            I: Input,
            V: Simd,{
                let lut_doubles_pair_data = LutPHFGroup::find_all_pairs::<I, V>(&input, simd, bit_mask)?;
                Ok(LutPHFGroup::build_lut_doubles(lambda, lut_doubles_pair_data, bit_mask, threaded))
            })
        });
        lut_perfect_naive.map_err(|e| Box::new(e) as Box<dyn std::error::Error>)
    }

    fn build_lut_doubles(lambda: usize, lut_doubles_pair_data: Vec<PairData>, bit_mask: usize, threaded: bool) -> Self {
        let lut_doubles: Vec<LutPHFDouble> = lut_doubles_pair_data
            .into_par_iter()
            .map(|pair_data| LutPHFDouble::build_double(lambda, pair_data, threaded))
            .collect();

        Self { lut_doubles, bit_mask }
    }

    fn find_all_pairs<I, V>(input: &I, simd: V, bit_mask: usize) -> Result<Vec<PairData>, error::InputError>
    where
        I: Input,
        V: Simd,
    {
        let iter = input.iter_blocks::<_, 64>(&EmptyRecorder);
        let quote_classifier = simd.classify_quoted_sequences(iter);
        let mut structural_classifier = simd.classify_structural_characters(quote_classifier);
        structural_classifier.turn_colons_and_commas_off();

        // Initialize a vector of PairData for each lut_double used
        let num_buckets = bit_mask + 1;
        let mut lut_doubles_pair_data: Vec<PairData> = vec![
            PairData {
                keys_16: vec![],
                values_16: vec![],
                keys_64: vec![],
                values_64: vec![],
            };
            num_buckets
        ];

        // Stacks for open brackets
        let mut square_bracket_stack: VecDeque<usize> = VecDeque::new();
        let mut curly_bracket_stack: VecDeque<usize> = VecDeque::new();

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

                    // Map to correct lut_double using the bit mask on the idx_open (= key)
                    let lut_double = &mut lut_doubles_pair_data[idx_open & bit_mask];

                    let distance = idx_close - idx_open;
                    if distance < THRESHOLD_16_BITS {
                        // Can fit into 16 bits
                        lut_double.keys_16.push(idx_open);
                        lut_double
                            .values_16
                            .push(distance.try_into().expect("Fail @ convert to 16 bit"));
                    } else {
                        // Needs 64 bits
                        lut_double.keys_64.push(idx_open);
                        lut_double.values_64.push(distance);
                    }
                }
                Structural::Colon(_) | Structural::Comma(_) => unreachable!(),
            }
        }

        Ok(lut_doubles_pair_data)
    }
}
