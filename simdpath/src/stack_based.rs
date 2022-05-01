//! Stack based implementation of a JSONPath query engine.
//!
//! Baseline engine for processing of JSONPath queries executing the query in
//! the natural, recursive manner.
//!
//! The [`stackless`](super::stackless) implementation should be more performant than
//! this one.

use crate::bytes;
use crate::engine::result::CountResult;
use crate::engine::{Input, Runner};
use crate::query::{JsonPathQuery, JsonPathQueryNode, JsonPathQueryNodeType};

/// Stack based runner for a fixed JSONPath query.
///
/// The runner is stateless, meaning that it can be executed
/// on any number of separate inputs, even on separate threads.
pub struct StackBasedRunner<'a> {
    query: &'a JsonPathQuery,
}

/// Result of a recursive descent of the runner.
struct RunnerResult<'a> {
    /// Count of matched values during the recursive descent.
    pub(crate) count: usize,
    /// Bytes not processed during the recursive descent.
    pub(crate) remaining_bytes: &'a [u8],
}

impl<'a> StackBasedRunner<'a> {
    /// Compile a query into a [`StackBasedRunner`].
    pub fn compile_query(query: &'a JsonPathQuery) -> Self {
        StackBasedRunner { query }
    }
}

impl Runner for StackBasedRunner<'_> {
    fn count(&self, input: &Input) -> CountResult {
        let mut state = State::Initial(InitialState::new(self.query.root(), input));
        CountResult {
            count: state.run().count,
        }
    }
}

/// Trait implemented by states of the engine that can
/// be executed for results.
trait Runnable<'a> {
    fn run(&mut self) -> RunnerResult<'a>;
}

/// Created at the beginning of query execution to kick off the engine.
///
/// Corresponds to a [`JsonPathQueryNode::Root`](super::query::JsonPathQueryNode<_>::Root) query node.
struct InitialState<'a, 'b> {
    node: &'a JsonPathQueryNode,
    bytes: &'b [u8],
}

/// Created to seek for a label recursively in an object.
///
/// Corresponds to a [`JsonPathQueryNode::Descendant`](super::query::JsonPathQueryNode<_>::Descendant) query node.
struct RecursiveDescentState<'a, 'b> {
    node: &'a JsonPathQueryNode,
    bytes: &'b [u8],
}

/// Created to seek for a label recursively in a list.
///
/// Corresponds to a [`JsonPathQueryNode::Descendant`](super::query::JsonPathQueryNode<_>::Descendant) query node.
struct RecurseInListState<'a, 'b> {
    node: &'a JsonPathQueryNode,
    bytes: &'b [u8],
}

impl<'a, 'b> InitialState<'a, 'b> {
    fn new(node: &'a JsonPathQueryNode, bytes: &'b [u8]) -> Self {
        InitialState { node, bytes }
    }
}

impl<'a, 'b> RecursiveDescentState<'a, 'b> {
    fn new(node: &'a JsonPathQueryNode, bytes: &'b [u8]) -> Self {
        debug_assert! {node.is_descendant()}
        RecursiveDescentState { node, bytes }
    }
}

impl<'a, 'b> RecurseInListState<'a, 'b> {
    fn new(node: &'a JsonPathQueryNode, bytes: &'b [u8]) -> Self {
        debug_assert! {node.is_descendant()}
        RecurseInListState { node, bytes }
    }
}

enum State<'a, 'b> {
    Initial(InitialState<'a, 'b>),
    RecursivelyFindLabel(RecursiveDescentState<'a, 'b>),
}

impl<'b> Runnable<'b> for State<'_, 'b> {
    fn run(&mut self) -> RunnerResult<'b> {
        match self {
            State::Initial(state) => state.run(),
            State::RecursivelyFindLabel(state) => state.run(),
        }
    }
}

impl<'b> Runnable<'b> for InitialState<'_, 'b> {
    fn run(&mut self) -> RunnerResult<'b> {
        debug_assert! {self.node.is_root()};

        let first_brace = bytes::find_byte(b'{', self.bytes);
        match first_brace {
            None => RunnerResult {
                count: 0,
                remaining_bytes: &[],
            },
            Some(first_brace) => match self.node.child() {
                None => RunnerResult {
                    count: 1,
                    remaining_bytes: &self.bytes[first_brace + 1..],
                },
                Some(child_node) => match child_node {
                    JsonPathQueryNode::Descendant(_) => State::RecursivelyFindLabel(
                        RecursiveDescentState::new(child_node, &self.bytes[first_brace + 1..]),
                    )
                    .run(),
                    JsonPathQueryNode::Label(_, _) => {
                        panic! {"Currently a Label expression can only be used after a Descendant expression."}
                    }
                    JsonPathQueryNode::Root(_) => {
                        panic! {"Root expression should not be reachable."}
                    }
                    _ => todo!(),
                },
            },
        }
    }
}

impl<'b> Runnable<'b> for RecursiveDescentState<'_, 'b> {
    fn run(&mut self) -> RunnerResult<'b> {
        let label_node = self.node.child().unwrap();
        let label = match label_node {
            JsonPathQueryNode::Label(label, _) => label,
            _ => panic!("RecursiveDescentState must be run on a Label node."),
        };

        // Inbound contract: we are inside a JSON object after its opening brace
        // and zero or more keys and values passed. Therefore there is either another
        // label in front or the closing brace.
        let mut bytes = self.bytes;
        let mut count = 0;

        loop {
            let next = bytes::find_unescaped_byte2(b'"', b'}', bytes)
                .expect("JSON is malformed: closing brace missing.");
            let byte = bytes[next];
            bytes = &bytes[next + 1..];

            if byte == b'}' {
                break;
            }

            // Here byte == '"' and bytes[0] is exactly the first byte of the label.
            let end_of_label = bytes::find_unescaped_byte(b'"', bytes)
                .expect("JSON is malformed: closing quote missing.");
            let current_label = &bytes[..end_of_label];
            bytes = &bytes[end_of_label + 1..];

            let colon = bytes::find_byte(b':', bytes).expect("JSON is malformed: colon missing.");
            bytes = &bytes[colon + 1..];

            let object_start =
                bytes::find_non_whitespace(bytes).expect("JSON is malformed: value missing.");
            let object_start_byte = bytes[object_start];
            bytes = &bytes[object_start + 1..];

            if label == current_label {
                if label_node.child().is_none() {
                    count += 1;
                }
                match object_start_byte {
                    b'{' => {
                        let mut next_state = match label_node.child() {
                            // No child means that we keep recursively searching for the same label.
                            None => RecursiveDescentState::new(self.node, bytes),
                            Some(child_node) => match child_node {
                                JsonPathQueryNode::Descendant(_) => {
                                    RecursiveDescentState::new(child_node, bytes)
                                }
                                JsonPathQueryNode::Label(_, _) => {
                                    panic! {"Currently a Label expression can only be used after a Descendant expression."}
                                }
                                JsonPathQueryNode::Root(_) => {
                                    panic! {"Root expression should not be reachable."}
                                }
                                _ => todo!(),
                            },
                        };
                        let result = next_state.run();

                        bytes = result.remaining_bytes;
                        count += result.count;
                    }
                    b'[' => {
                        let mut next_state = match label_node.child() {
                            // No child means that we keep recursively searching for the same label.
                            None => RecurseInListState::new(self.node, bytes),
                            Some(child_node) => match child_node {
                                JsonPathQueryNode::Descendant(_) => {
                                    RecurseInListState::new(child_node, bytes)
                                }
                                JsonPathQueryNode::Label(_, _) => {
                                    panic! {"Currently a Label expression can only be used after a Descendant expression."}
                                }
                                JsonPathQueryNode::Root(_) => {
                                    panic! {"Root expression should not be reachable."}
                                }
                                _ => todo!(),
                            },
                        };
                        let result = next_state.run();

                        bytes = result.remaining_bytes;
                        count += result.count;
                    }
                    b'"' => {
                        let next = bytes::find_unescaped_byte(b'"', bytes)
                            .expect("JSON is malformed: closing quote missing.");
                        bytes = &bytes[next + 1..];
                    }
                    _ => {
                        let next = bytes::find_byte2(b',', b'}', bytes)
                            .expect("JSON is malformed: closing brace missing.");
                        bytes = &bytes[next..];
                    }
                }
            } else {
                match object_start_byte {
                    b'{' => {
                        let mut recursive_state = RecursiveDescentState::new(self.node, bytes);
                        let result = recursive_state.run();

                        bytes = result.remaining_bytes;
                        count += result.count;
                    }
                    b'[' => {
                        let mut state = RecurseInListState::new(self.node, bytes);
                        let result = state.run();

                        bytes = result.remaining_bytes;
                        count += result.count;
                    }
                    b'"' => {
                        let next = bytes::find_unescaped_byte(b'"', bytes)
                            .expect("JSON is malformed: closing quote missing.");
                        bytes = &bytes[next + 1..];
                    }
                    _ => {
                        let next = bytes::find_byte2(b',', b'}', bytes)
                            .expect("JSON is malformed: closing brace missing.");
                        bytes = &bytes[next..];
                    }
                }
            }
        }

        // At the end bytes are exactly the bytes after the closing brace of the JSON object
        // we started in.
        RunnerResult {
            count,
            remaining_bytes: bytes,
        }
    }
}

impl<'b> Runnable<'b> for RecurseInListState<'_, 'b> {
    fn run(&mut self) -> RunnerResult<'b> {
        // Inbound contract: we are inside a JSON list after its opening bracket
        // and zero or more values passed. Therefore there is either another
        // value in front or the closing bracket.
        let mut bytes = self.bytes;
        let mut count = 0;

        loop {
            let next = bytes::find_non_whitespace(bytes)
                .expect("JSON is malformed: closing bracket missing.");
            let byte = bytes[next];
            bytes = &bytes[next + 1..];

            if byte == b']' {
                break;
            }

            match byte {
                b'{' => {
                    let mut next_state = RecursiveDescentState::new(self.node, bytes);
                    let result = next_state.run();

                    bytes = result.remaining_bytes;
                    count += result.count;

                    let next = bytes::find_byte2(b',', b']', bytes)
                        .expect("JSON is malformed: closing bracket missing.");

                    if bytes[next] == b',' {
                        bytes = &bytes[next + 1..];
                    } else {
                        bytes = &bytes[next..];
                    }
                }
                b'[' => {
                    let mut next_state = RecurseInListState::new(self.node, bytes);
                    let result = next_state.run();
                    bytes = result.remaining_bytes;
                    count += result.count;
                }
                b'"' => {
                    let next = bytes::find_unescaped_byte(b'"', bytes)
                        .expect("JSON is malformed: closing quote missing.");
                    bytes = &bytes[next + 1..];
                    let comma = bytes::find_byte2(b',', b']', bytes)
                        .expect("JSON is malformed: closing bracket missing.");

                    if bytes[comma] == b',' {
                        bytes = &bytes[comma + 1..];
                    } else {
                        bytes = &bytes[comma..];
                    }
                }
                _ => {
                    let next = bytes::find_byte2(b',', b']', bytes)
                        .expect("JSON is malformed: closing bracket missing.");

                    if bytes[next] == b',' {
                        bytes = &bytes[next + 1..];
                    } else {
                        bytes = &bytes[next..];
                    }
                }
            }
        }

        // At the end bytes are exactly the bytes after the closing bracket of the JSON list
        // we started in.
        RunnerResult {
            count,
            remaining_bytes: bytes,
        }
    }
}
