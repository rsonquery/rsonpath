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
    fn count<T: AsRef<[u8]>>(&self, input: &Input<T>) -> CountResult {
        assert_eq!(input.as_ref().len() % crate::bytes::simd::BLOCK_SIZE, 0);

        /*if self.labels.len() == 3 {
            let count = custom_automaton3(&self.labels, input.as_ref());
            return CountResult { count };
        }*/

        let count = automata::dispatch_automaton(&self.labels, input.as_ref());

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
/*
impl<'a> Iterator for BlockIterator<'a> {
    type Item = &'a [u8];

    fn next(&mut self) -> Option<Self::Item> {
        let ret = match self.bytes {
            &[] => None,
            bytes if bytes.len() >= 32 => Some(&self.bytes[..32]),
            _ => {
                self.buffer.copy_into_prefix(self.bytes);
                let r: &'a [u8] = self.buffer.get_ref();
                Some(self.buffer.get_ref())
            }
        };
        self.bytes = &self.bytes[32..];
        ret
    }

    fn size_hint(&self) -> (usize, Option<usize>) {
        (self.block_count(), Some(self.block_count()))
    }
}*/

fn custom_automaton3(labels: &[&[u8]], bytes: &[u8]) -> usize {
    debug_assert_eq!(labels.len(), 3usize);

    let mut depth: usize = 0;
    let mut state: u8 = 0;
    let mut count: usize = 0;
    let mut regs = [0usize; 3u8 as usize];

    let mut bytes = bytes;

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
    count
}
