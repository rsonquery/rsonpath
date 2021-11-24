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

use crate::engine::result::CountResult;
use crate::engine::{Input, Runner};
use crate::query::{JsonPathQuery, JsonPathQueryNode, JsonPathQueryNodeType};

/// Stackless runner for a fixed JSONPath query.
///
/// The runner is stateless, meaning that it can be executed
/// on any number of separate inputs, even on separate threads.
pub struct StacklessRunner<'a> {
    labels: Vec<&'a [u8]>,
}

impl<'a> StacklessRunner<'a> {
    /// Compile a query into a [`StacklessRunner`].
    ///
    /// Compilation time is proportional to the length of the query.
    pub fn compile_query(query: &JsonPathQuery<'a>) -> StacklessRunner<'a> {
        let labels = query_to_descendant_pattern_labels(query);

        automata::assert_supported_size!(labels.len());

        StacklessRunner { labels }
    }
}

impl<'a> Runner for StacklessRunner<'a> {
    fn count_bytes(&self, input: &Input<&[u8]>) -> CountResult {
        if self.labels.len() == 3 {
            let count = unsafe { custom_automaton3(&self.labels, input) };
            return CountResult { count };
        }

        let count = automata::dispatch_automaton(&self.labels, input);

        CountResult { count }
    }
}

fn query_to_descendant_pattern_labels<'a>(query: &JsonPathQuery<'a>) -> Vec<&'a [u8]> {
    debug_assert!(query.root().is_root());
    let mut node_opt = query.root().child();
    let mut result = vec![];

    while let Some(node) = node_opt {
        match node {
            JsonPathQueryNode::Descendant(label_node) => match label_node.as_ref() {
                JsonPathQueryNode::Label(label, next_node) => {
                    result.push(*label);
                    node_opt = next_node.as_deref();
                }
                _ => panic! {"Unexpected type of node, expected Label."},
            },
            _ => panic! {"Unexpected type of node, expected Descendant."},
        }
    }

    result
}

enum BlockEvent {
    Closing(usize),
    Quote(usize),
}

struct BlockEventSource {
    opening_mask: u32,
    closing_mask: u32,
    event_mask: u32,
}

impl BlockEventSource {
    #[inline(always)]
    pub fn new(opening_mask: u32, closing_mask: u32, quote_mask: u32) -> Self {
        Self {
            opening_mask,
            closing_mask,
            event_mask: closing_mask | quote_mask,
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
            Quote(next_event_idx as usize)
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
unsafe fn custom_automaton3(labels: &[&[u8]], contents: &[u8]) -> usize {
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
    let mut bytes = contents;
    let opening_brace_byte_mask = _mm256_set1_epi8(b'{' as i8);
    let opening_bracket_byte_mask = _mm256_set1_epi8(b'[' as i8);
    let closing_brace_byte_mask = _mm256_set1_epi8(b'}' as i8);
    let closing_bracket_byte_mask = _mm256_set1_epi8(b']' as i8);
    let quote_byte_mask = _mm256_set1_epi8(b'"' as i8);

    while !bytes.is_empty() {
        let byte_vector = _mm256_loadu_si256(bytes.as_ptr() as *const __m256i);
        let opening_brace_cmp = _mm256_cmpeq_epi8(byte_vector, opening_brace_byte_mask);
        let opening_bracket_cmp = _mm256_cmpeq_epi8(byte_vector, opening_bracket_byte_mask);
        let closing_brace_cmp = _mm256_cmpeq_epi8(byte_vector, closing_brace_byte_mask);
        let closing_bracket_cmp = _mm256_cmpeq_epi8(byte_vector, closing_bracket_byte_mask);
        let quote_cmp = _mm256_cmpeq_epi8(byte_vector, quote_byte_mask);
        let opening_vector = _mm256_or_si256(opening_brace_cmp, opening_bracket_cmp);
        let closing_vector = _mm256_or_si256(closing_brace_cmp, closing_bracket_cmp);
        let opening_mask = _mm256_movemask_epi8(opening_vector) as u32;
        let closing_mask = _mm256_movemask_epi8(closing_vector) as u32;
        let quote_mask = _mm256_movemask_epi8(quote_cmp) as u32;

        let mut block_event_source = BlockEventSource::new(opening_mask, closing_mask, quote_mask);

        while let Some(event) = block_event_source.poll() {
            match state {
                0 => {
                    // Depth is irrelevant.
                    if let BlockEvent::Quote(idx) = event {
                        let len = labels[0].len();
                        let closing_quote_position = idx + len + 1;

                        if bytes[closing_quote_position..closing_quote_position + 2] == [b'"', b':']
                            && bytes[idx + 1..].starts_with(labels[0])
                        {
                            state = 1;
                            regs[0] = depth + block_event_source.depth_at_index(idx);
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
                    BlockEvent::Quote(idx) => {
                        let len = labels[1].len();
                        let closing_quote_position = idx + len + 1;

                        if bytes[closing_quote_position..closing_quote_position + 2] == [b'"', b':']
                            && bytes[idx + 1..].starts_with(labels[1])
                        {
                            state = 2;
                            regs[1] = depth + block_event_source.depth_at_index(idx);
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
                    BlockEvent::Quote(idx) => {
                        let len = labels[2].len();
                        let closing_quote_position = idx + len + 1;

                        if bytes[closing_quote_position..closing_quote_position + 2] == [b'"', b':']
                            && bytes[idx + 1..].starts_with(labels[2])
                        {
                            count += 1;
                        }
                    }
                },
                _ => unreachable!(),
            }
        }

        depth += block_event_source.depth_at_end();
        bytes = &bytes[BLOCK_SIZE..];
        /*if depth <= -closing_count {
            // Depth is guaranteed to not go below within the block.
        } else {
            // Depth may go below within the block.
        }

        depth += opening_count - closing_count;*/
    }
    count
    /*
    while let Some(i) = crate::bytes::find_non_whitespace(bytes) {
        match state {
            0u8 => match bytes[i] {
                b'{' => {
                    depth += 1;
                    bytes = &bytes[i + 1..];
                }
                b'}' => {
                    depth -= 1;
                    bytes = &bytes[i + 1..];
                }
                b'[' => {
                    depth += 1;
                    bytes = &bytes[i + 1..];
                }
                b']' => {
                    depth -= 1;
                    bytes = &bytes[i + 1..];
                }
                b'\\' => {
                    bytes = &bytes[i + 2..];
                }
                b'"' => {
                    bytes = &bytes[i + 1..];
                    let closing_quote = crate::bytes::find_unescaped_byte(b'"', bytes).unwrap();
                    let label = &bytes[..closing_quote];
                    bytes = &bytes[closing_quote + 1..];
                    let next = crate::bytes::find_non_whitespace(bytes).unwrap();
                    if bytes[next] == b':' {
                        bytes = &bytes[next + 1..];
                        let next = crate::bytes::find_non_whitespace(bytes).unwrap();
                        if (bytes[next] == b'{' || bytes[next] == b'[')
                            && label == labels[0u8 as usize]
                        {
                            state = 0u8 + 1;
                            regs[0u8 as usize] = depth;
                            depth += 1;
                            bytes = &bytes[next + 1..];
                        } else {
                            bytes = &bytes[next..];
                        }
                    } else {
                        bytes = &bytes[next..];
                    }
                }
                _ => {
                    bytes = &bytes[i + 1..];
                }
            },
            1u8 => match bytes[i] {
                b'{' => {
                    depth += 1;
                    bytes = &bytes[i + 1..];
                }
                b'}' => {
                    depth -= 1;
                    bytes = &bytes[i + 1..];
                    if depth == regs[0usize] {
                        state = 1u8 - 1;
                    }
                }
                b'[' => {
                    depth += 1;
                    bytes = &bytes[i + 1..];
                }
                b']' => {
                    depth -= 1;
                    bytes = &bytes[i + 1..];
                    if depth == regs[0usize] {
                        state = 1u8 - 1;
                    }
                }
                b'\\' => {
                    bytes = &bytes[i + 2..];
                }
                b'"' => {
                    bytes = &bytes[i + 1..];
                    let closing_quote = crate::bytes::find_unescaped_byte(b'"', bytes).unwrap();
                    let label = &bytes[..closing_quote];
                    bytes = &bytes[closing_quote + 1..];
                    let next = crate::bytes::find_non_whitespace(bytes).unwrap();
                    if bytes[next] == b':' {
                        bytes = &bytes[next + 1..];
                        let next = crate::bytes::find_non_whitespace(bytes).unwrap();
                        if (bytes[next] == b'{' || bytes[next] == b'[')
                            && label == labels[1u8 as usize]
                        {
                            state = 1u8 + 1;
                            regs[1u8 as usize] = depth;
                            depth += 1;
                            bytes = &bytes[next + 1..];
                        } else {
                            bytes = &bytes[next..];
                        }
                    } else {
                        bytes = &bytes[next..];
                    }
                }
                _ => {
                    bytes = &bytes[i + 1..];
                }
            },
            2u8 => match bytes[i] {
                b'{' => {
                    depth += 1;
                    bytes = &bytes[i + 1..];
                }
                b'}' => {
                    depth -= 1;
                    bytes = &bytes[i + 1..];
                    if depth == regs[1usize] {
                        state = 2u8 - 1;
                    }
                }
                b'[' => {
                    depth += 1;
                    bytes = &bytes[i + 1..];
                }
                b']' => {
                    depth -= 1;
                    bytes = &bytes[i + 1..];
                    if depth == regs[1usize] {
                        state = 2u8 - 1;
                    }
                }
                b'\\' => {
                    bytes = &bytes[i + 2..];
                }
                b'"' => {
                    bytes = &bytes[i + 1..];
                    let closing_quote = crate::bytes::find_unescaped_byte(b'"', bytes).unwrap();
                    let label = &bytes[..closing_quote];
                    bytes = &bytes[closing_quote + 1..];
                    let next = crate::bytes::find_non_whitespace(bytes).unwrap();
                    if bytes[next] == b':' {
                        bytes = &bytes[next + 1..];
                        let next = crate::bytes::find_non_whitespace(bytes).unwrap();
                        if label == labels[2u8 as usize] {
                            count += 1;
                            bytes = &bytes[next..];
                        } else {
                            bytes = &bytes[next..];
                        }
                    } else {
                        bytes = &bytes[next..];
                    }
                }
                _ => {
                    bytes = &bytes[i + 1..];
                }
            },
            _ => unreachable! {},
        }
    }
    count*/
}

struct BlockIterator<'a> {
    bytes: &'a [u8],
}

impl<'a> BlockIterator<'a> {
    #[inline(always)]
    fn new(bytes: &'a [u8]) -> Self {
        debug_assert_eq!(bytes.len() / crate::bytes::simd::BLOCK_SIZE, 0);
        Self { bytes }
    }

    #[inline(always)]
    fn block_count(&self) -> usize {
        use crate::bytes::simd::BLOCK_SIZE;

        (self.bytes.len() + BLOCK_SIZE - 1) / BLOCK_SIZE
    }
}

impl<'a> Iterator for BlockIterator<'a> {
    type Item = &'a [u8];

    #[inline(always)]
    fn next(&mut self) -> Option<Self::Item> {
        use crate::bytes::simd::BLOCK_SIZE;

        let block = &self.bytes[..BLOCK_SIZE];
        debug_assert_eq!(block.len(), BLOCK_SIZE);
        self.bytes = &self.bytes[BLOCK_SIZE..];

        if block.is_empty() {
            None
        } else {
            Some(block)
        }
    }

    #[inline(always)]
    fn size_hint(&self) -> (usize, Option<usize>) {
        (self.block_count(), Some(self.block_count()))
    }
}

impl<'a> ExactSizeIterator for BlockIterator<'a> {}

struct TwoBlockWindowIterator<'a> {
    block_iterator: BlockIterator<'a>,
    current_block: &'a [u8],
}

impl<'a> TwoBlockWindowIterator<'a> {
    #[inline(always)]
    pub fn new(mut block_iterator: BlockIterator<'a>) -> Self {
        let first_block = block_iterator
            .next()
            .expect("empty BlockIterator provided for TwoBlockWindowIterator");
        Self {
            block_iterator,
            current_block: first_block,
        }
    }
}

impl<'a> Iterator for TwoBlockWindowIterator<'a> {
    type Item = (&'a [u8], &'a [u8]);

    #[inline(always)]
    fn next(&mut self) -> Option<Self::Item> {
        let next_block = self.block_iterator.next();

        match next_block {
            None => None,
            Some(block) => {
                let result = (self.current_block, block);
                self.current_block = block;
                Some(result)
            }
        }
    }

    #[inline(always)]
    fn size_hint(&self) -> (usize, Option<usize>) {
        self.block_iterator.size_hint()
    }
}
