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
}

#[cfg(all(
    not(feature = "nosimd"),
    any(target_arch = "x86", target_arch = "x86_64")
))]
struct BlockEventSource {
    opening_mask: u32,
    closing_mask: u32,
    event_mask: u32,
}

#[cfg(all(
    not(feature = "nosimd"),
    any(target_arch = "x86", target_arch = "x86_64")
))]
impl BlockEventSource {
    #[inline(always)]
    pub fn new(
        opening_mask: u32,
        closing_mask: u32,
        colon_mask: u32,
        depth_difference_threshold: isize,
    ) -> Self {
        let closing_count = closing_mask.count_ones() as isize;
        if depth_difference_threshold > closing_count {
            // Depth is guaranteed to not go below within the block.
            Self {
                opening_mask,
                closing_mask,
                event_mask: colon_mask,
            }
        } else {
            // Depth may go below within the block.
            Self {
                opening_mask,
                closing_mask,
                event_mask: closing_mask | colon_mask,
            }
        }
    }

    #[inline(always)]
    pub fn poll(&mut self) -> Option<BlockEvent> {
        use BlockEvent::*;
        let next_event_idx = self.event_mask.trailing_zeros();

        if next_event_idx == 32 {
            return None;
        }

        let bit_mask = 1 << next_event_idx;

        self.event_mask ^= bit_mask;

        let event = if self.closing_mask & bit_mask != 0 {
            Closing(next_event_idx as usize)
        } else {
            Colon(next_event_idx as usize)
        };

        Some(event)
    }

    #[inline(always)]
    pub fn depth_at_index(&self, idx: usize) -> isize {
        use crate::bytes::simd::BLOCK_SIZE;
        let rev_idx = BLOCK_SIZE - idx - 1;

        let depth = ((self.opening_mask << rev_idx).count_ones() as i32)
            - ((self.closing_mask << rev_idx).count_ones() as i32);
        depth as isize
    }

    #[inline(always)]
    pub fn depth_at_end(&self) -> isize {
        use crate::bytes::simd::BLOCK_SIZE;
        self.depth_at_index(BLOCK_SIZE - 1)
    }
}

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
    let opening_brace_byte_mask = _mm256_set1_epi8(b'{' as i8);
    let opening_bracket_byte_mask = _mm256_set1_epi8(b'[' as i8);
    let closing_brace_byte_mask = _mm256_set1_epi8(b'}' as i8);
    let closing_bracket_byte_mask = _mm256_set1_epi8(b']' as i8);
    let colon_byte_mask = _mm256_set1_epi8(b':' as i8);

    while !block.is_empty() {
        let byte_vector = _mm256_load_si256(block.as_ptr() as *const __m256i);
        let opening_brace_cmp = _mm256_cmpeq_epi8(byte_vector, opening_brace_byte_mask);
        let opening_bracket_cmp = _mm256_cmpeq_epi8(byte_vector, opening_bracket_byte_mask);
        let closing_brace_cmp = _mm256_cmpeq_epi8(byte_vector, closing_brace_byte_mask);
        let closing_bracket_cmp = _mm256_cmpeq_epi8(byte_vector, closing_bracket_byte_mask);
        let colon_cmp = _mm256_cmpeq_epi8(byte_vector, colon_byte_mask);
        let opening_vector = _mm256_or_si256(opening_brace_cmp, opening_bracket_cmp);
        let closing_vector = _mm256_or_si256(closing_brace_cmp, closing_bracket_cmp);
        let opening_mask = _mm256_movemask_epi8(opening_vector) as u32;
        let closing_mask = _mm256_movemask_epi8(closing_vector) as u32;
        let colon_mask = _mm256_movemask_epi8(colon_cmp) as u32;

        let depth_difference_threshold = match state {
            0 => isize::MIN,
            1 => depth - regs[0],
            2 => depth - regs[1],
            _ => unreachable!(),
        };
        let mut block_event_source = BlockEventSource::new(
            opening_mask,
            closing_mask,
            colon_mask,
            depth_difference_threshold,
        );

        while let Some(event) = block_event_source.poll() {
            match state {
                0 => {
                    // Depth is irrelevant.
                    if let BlockEvent::Colon(idx) = event {
                        let len = labels[0].len();
                        if offset + idx >= len + 2 {
                            let opening_quote_idx = offset + idx - len - 2;
                            let slice = &bytes[opening_quote_idx..offset + idx];

                            if slice == labels[0].bytes_with_quotes() {
                                state = 1;
                                regs[0] = depth + block_event_source.depth_at_index(idx);
                            }
                        }
                    }
                }
                1 => match event {
                    BlockEvent::Closing(idx) => {
                        let actual_depth = depth + block_event_source.depth_at_index(idx);
                        if actual_depth <= regs[0] {
                            state = 0;
                        }
                    }
                    BlockEvent::Colon(idx) => {
                        let len = labels[1].len();
                        if offset + idx >= len + 2 {
                            let opening_quote_idx = offset + idx - len - 2;
                            let slice = &bytes[opening_quote_idx..offset + idx];

                            if slice == labels[1].bytes_with_quotes() {
                                state = 2;
                                regs[1] = depth + block_event_source.depth_at_index(idx);
                            }
                        }
                    }
                },
                2 => match event {
                    BlockEvent::Closing(idx) => {
                        let actual_depth = depth + block_event_source.depth_at_index(idx);
                        if actual_depth <= regs[1] {
                            state = 1;
                        }
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

        depth += block_event_source.depth_at_end();
        block = &block[BLOCK_SIZE..];
        offset += BLOCK_SIZE;
    }
    count
}
