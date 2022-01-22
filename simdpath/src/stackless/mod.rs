//! Stackless implementation of a JSONPath query engine.
//!
//! Core engine for processing of JSONPath queries, based on the
//! [Stackless Processing of Streamed Trees](https://hal.archives-ouvertes.fr/hal-03021960) paper.
//! Entire query execution is done without recursion or an explicit stack, linearly through
//! the JSON structure, which allows efficient SIMD operations and optimized register usage.
//!
//! This implementation should be more performant than [`stack_based`](super::stack_based)
//! even on targets that don't support AVX2 SIMD operations.

#[allow(clippy::all)]
mod automata;

use crate::bytes::align::{alignment, AlignedBytes};
use crate::engine::result::CountResult;
use crate::engine::{Input, Runner};
use crate::query::{JsonPathQuery, JsonPathQueryNode, JsonPathQueryNodeType, Label};

/// Stackless runner for a fixed JSONPath query.
///
/// The runner is stateless, meaning that it can be executed
/// on any number of separate inputs, even on separate threads.
pub struct StacklessRunner<'a> {
    labels: Vec<&'a Label>,
}

impl<'a> StacklessRunner<'a> {
    /// Compile a query into a [`StacklessRunner`].
    ///
    /// Compilation time is proportional to the length of the query.
    pub fn compile_query(query: &JsonPathQuery) -> StacklessRunner<'_> {
        let labels = query_to_descendant_pattern_labels(query);

        automata::assert_supported_size!(labels.len());

        StacklessRunner { labels }
    }
}

impl<'a> Runner for StacklessRunner<'a> {
    fn count(&self, input: &Input) -> CountResult {
        #[cfg(all(
            not(feature = "nosimd"),
            any(target_arch = "x86", target_arch = "x86_64")
        ))]
        if self.labels.len() == 3 {
            let count = unsafe { custom_automaton3(&self.labels, input) };
            return CountResult { count };
        }

        let count = automata::dispatch_automaton(&self.labels, input);

        CountResult { count }
    }
}

fn query_to_descendant_pattern_labels(query: &JsonPathQuery) -> Vec<&Label> {
    debug_assert!(query.root().is_root());
    let mut node_opt = query.root().child();
    let mut result = vec![];

    while let Some(node) = node_opt {
        match node {
            JsonPathQueryNode::Descendant(label_node) => match label_node.as_ref() {
                JsonPathQueryNode::Label(label, next_node) => {
                    result.push(label);
                    node_opt = next_node.as_deref();
                }
                _ => panic! {"Unexpected type of node, expected Label."},
            },
            _ => panic! {"Unexpected type of node, expected Descendant."},
        }
    }

    result
}

#[cfg(all(
    not(feature = "nosimd"),
    any(target_arch = "x86", target_arch = "x86_64")
))]
enum BlockEvent {
    Closing(usize),
    Colon(usize),
    Opening(usize),
}

#[cfg(all(
    not(feature = "nosimd"),
    any(target_arch = "x86", target_arch = "x86_64")
))]
struct BlockEventSource<'a> {
    block: &'a [u8],
    structural_mask: u64,
}

#[cfg(all(
    not(feature = "nosimd"),
    any(target_arch = "x86", target_arch = "x86_64")
))]
impl<'a> BlockEventSource<'a> {
    #[inline(always)]
    pub fn new(block: &'a [u8], structural_mask: u64) -> Self {
        Self {
            block,
            structural_mask,
        }
    }

    #[inline(always)]
    pub fn poll(&mut self) -> Option<BlockEvent> {
        use BlockEvent::*;
        let next_event_idx = self.structural_mask.trailing_zeros();

        if next_event_idx == 64 {
            return None;
        }

        let bit_mask = 1 << next_event_idx;

        self.structural_mask ^= bit_mask;

        let idx = next_event_idx as usize;
        let event = match self.block[idx] {
            b']' | b'}' => Closing(idx),
            b'[' | b'{' => Opening(idx),
            _ => Colon(idx),
        };

        Some(event)
    }
}
/*fn reverse(x: u64) -> u64 {
    let mut res = 0u64;
    for i in 0..64 {
        let bit = (x & (1 << i)) >> i;
        res |= bit << (63 - i);
    }
    res
}
macro_rules! bin {
    ($name: expr, $e:expr) => {
        log::debug!("{: >24}: {:064b} ({})", $name, reverse($e), $e);
    };
}*/

#[cfg(all(
    not(feature = "nosimd"),
    any(target_arch = "x86", target_arch = "x86_64")
))]
#[target_feature(enable = "avx2")]
unsafe fn custom_automaton3(labels: &[&Label], bytes: &AlignedBytes<alignment::Page>) -> usize {
    use crate::bytes::simd::BLOCK_SIZE;
    #[cfg(target_arch = "x86")]
    use core::arch::x86::*;
    #[cfg(target_arch = "x86_64")]
    use core::arch::x86_64::*;

    debug_assert_eq!(labels.len(), 3usize);
    let mut depth: isize = 0;
    let mut state: u8 = 0;
    let mut count: usize = 0;
    let mut regs = [0isize; 3];
    let mut block: &[u8] = bytes;
    let mut offset = 0usize;

    let lower_nibble_mask_array: [u8; 32] = [
        0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x01, 0x02, 0x00, 0x02, 0x00,
        0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x01, 0x02, 0x00, 0x02,
        0x00, 0x00,
    ];
    let upper_nibble_mask_array: [u8; 32] = [
        0xFF, 0xFF, 0xFF, 0x01, 0xFF, 0x02, 0xFF, 0x02, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF,
        0xFF, 0xFF, 0xFF, 0xFF, 0x01, 0xFF, 0x02, 0xFF, 0x02, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF,
        0xFF, 0xFF,
    ];

    let lower_nibble_mask = _mm256_loadu_si256(lower_nibble_mask_array.as_ptr() as *const __m256i);
    let upper_nibble_mask = _mm256_loadu_si256(upper_nibble_mask_array.as_ptr() as *const __m256i);

    let upper_nibble_zeroing_mask = _mm256_set1_epi8(0x0F);

    let quote_mask = _mm256_set1_epi8(b'"' as i8);
    let slash_mask = _mm256_set1_epi8(b'\\' as i8);

    let mut prev_quote_bit = 0u64;
    let mut prev_slash_bit = 0u64;

    let all_ones128 = _mm_set1_epi8(0xFFu8 as i8);

    while !block.is_empty() {
        //log::debug!("Block start.");
        /*log::debug!(
            "{: >24}: {}",
            "block",
            std::str::from_utf8_unchecked(
                &block[..64]
                    .iter()
                    .map(|x| if x.is_ascii_whitespace() { b' ' } else { *x })
                    .collect::<Vec<_>>()
            )
        );*/

        let byte_vector1 = _mm256_load_si256(block.as_ptr() as *const __m256i);
        let byte_vector2 = _mm256_load_si256(block[BLOCK_SIZE..].as_ptr() as *const __m256i);
        let shifted_byte_vector1 = _mm256_srli_epi16::<4>(byte_vector1);
        let shifted_byte_vector2 = _mm256_srli_epi16::<4>(byte_vector2);
        let upper_nibble_byte_vector1 =
            _mm256_and_si256(shifted_byte_vector1, upper_nibble_zeroing_mask);
        let upper_nibble_byte_vector2 =
            _mm256_and_si256(shifted_byte_vector2, upper_nibble_zeroing_mask);
        let lower_nibble_lookup1 = _mm256_shuffle_epi8(lower_nibble_mask, byte_vector1);
        let lower_nibble_lookup2 = _mm256_shuffle_epi8(lower_nibble_mask, byte_vector2);
        let upper_nibble_lookup1 =
            _mm256_shuffle_epi8(upper_nibble_mask, upper_nibble_byte_vector1);
        let upper_nibble_lookup2 =
            _mm256_shuffle_epi8(upper_nibble_mask, upper_nibble_byte_vector2);
        let structural_vector1 = _mm256_cmpeq_epi8(lower_nibble_lookup1, upper_nibble_lookup1);
        let structural_vector2 = _mm256_cmpeq_epi8(lower_nibble_lookup2, upper_nibble_lookup2);
        let structural1 = _mm256_movemask_epi8(structural_vector1) as u32;
        let structural2 = _mm256_movemask_epi8(structural_vector2) as u32;

        let structural = (structural1 as u64) | ((structural2 as u64) << 32);
        //bin!("structural", structural);

        let slash_cmp1 = _mm256_cmpeq_epi8(byte_vector1, slash_mask);
        let slash_cmp2 = _mm256_cmpeq_epi8(byte_vector2, slash_mask);
        let slashes1 = _mm256_movemask_epi8(slash_cmp1) as u32;
        let slashes2 = _mm256_movemask_epi8(slash_cmp2) as u32;
        let slashes = (slashes1 as u64) | ((slashes2 as u64) << 32);
        //bin!("slashes", slashes);

        let even = 0b0101010101010101010101010101010101010101010101010101010101010101u64;
        let odd = 0b1010101010101010101010101010101010101010101010101010101010101010u64;
        let starts = slashes & !(slashes << 1) & !prev_slash_bit;
        let even_starts = even & starts;
        let odd_starts = odd & starts;

        let ends_of_even_starts = (even_starts.wrapping_add(slashes)) & !slashes;
        let ends_of_odd_starts = (odd_starts.wrapping_add(slashes)) & !slashes;

        let escaped = (ends_of_even_starts & odd) | (ends_of_odd_starts & even) | prev_slash_bit;
        //bin!("escaped", escaped);
        prev_slash_bit = (slashes & !ends_of_even_starts & !ends_of_odd_starts & (1 << 63)) >> 63;
        //bin!("prev_slash_bit", prev_slash_bit);

        let quote_cmp1 = _mm256_cmpeq_epi8(byte_vector1, quote_mask);
        let quote_cmp2 = _mm256_cmpeq_epi8(byte_vector2, quote_mask);
        let quotes1 = _mm256_movemask_epi8(quote_cmp1) as u32;
        let quotes2 = _mm256_movemask_epi8(quote_cmp2) as u32;
        let quotes = (quotes1 as u64) | ((quotes2 as u64) << 32);
        //bin!("quotes", quotes);
        //bin!("quotes & !escaped", quotes & !escaped);

        let nonescaped_quotes = (quotes & !escaped) ^ prev_quote_bit;
        //bin!("nonescaped_quotes", nonescaped_quotes);

        let nonescaped_quotes_vector = _mm_set_epi64x(0, nonescaped_quotes as i64);
        let cumulative_xor = _mm_clmulepi64_si128::<0>(nonescaped_quotes_vector, all_ones128);

        let within_quotes = _mm_cvtsi128_si64(cumulative_xor) as u64;
        //bin!("within_quotes", within_quotes);
        prev_quote_bit = (within_quotes & (1 << 63)) >> 63;
        //bin!("prev_quote_bit", prev_quote_bit);

        let nonquoted_structural = structural & !within_quotes;
        //bin!("nonquoted_structural", nonquoted_structural);

        let mut block_event_source = BlockEventSource::new(block, nonquoted_structural);

        while let Some(event) = block_event_source.poll() {
            match state {
                0 => match event {
                    BlockEvent::Closing(_) => {
                        depth -= 1;
                    }
                    BlockEvent::Opening(_) => {
                        depth += 1;
                    }
                    BlockEvent::Colon(idx) => {
                        let len = labels[0].len();
                        if offset + idx >= len + 2 {
                            let opening_quote_idx = offset + idx - len - 2;
                            let slice = &bytes[opening_quote_idx..offset + idx];

                            if slice == labels[0].bytes_with_quotes() {
                                state = 1;
                                regs[0] = depth;
                            }
                        }
                    }
                },
                1 => match event {
                    BlockEvent::Closing(_) => {
                        depth -= 1;
                        if depth <= regs[0] {
                            state = 0;
                        }
                    }
                    BlockEvent::Opening(_) => {
                        depth += 1;
                    }
                    BlockEvent::Colon(idx) => {
                        let len = labels[1].len();
                        if offset + idx >= len + 2 {
                            let opening_quote_idx = offset + idx - len - 2;
                            let slice = &bytes[opening_quote_idx..offset + idx];

                            if slice == labels[1].bytes_with_quotes() {
                                state = 2;
                                regs[1] = depth;
                            }
                        }
                    }
                },
                2 => match event {
                    BlockEvent::Closing(_) => {
                        depth -= 1;
                        if depth <= regs[1] {
                            state = 1;
                        }
                    }
                    BlockEvent::Opening(_) => {
                        depth += 1;
                    }
                    BlockEvent::Colon(idx) => {
                        let len = labels[2].len();
                        if offset + idx >= len + 2 {
                            let opening_quote_idx = offset + idx - len - 2;
                            let slice = &bytes[opening_quote_idx..offset + idx];

                            if slice == labels[2].bytes_with_quotes() {
                                count += 1;
                            }
                        }
                    }
                },
                _ => unreachable!(),
            }
        }

        block = &block[2 * BLOCK_SIZE..];
        offset += 2 * BLOCK_SIZE;
        //log::debug!("Block end ({}).", count);
    }
    count
}
