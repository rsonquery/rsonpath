//! Reference implementation of a JSONPath query engine with recursive descent.

use crate::bytes::classify::{classify_structural_characters, Structural, StructuralIterator};
use crate::debug;
use crate::engine::{result, Input, Runner};
use crate::query::{JsonPathQuery, JsonPathQueryNode, Label};
use aligners::{alignment, AlignedSlice};
use smallvec::{smallvec, SmallVec};
use std::iter::Peekable;
use std::rc::{Rc, Weak};

/*
 * The approach:
 * Including children requires us to carry more state than just the single
 * label that we want to match at any given point. Instead of that, many different
 * branches can follow a single label match.
 *
 * We represent each such state with a triple (
 *  label to match,
 *  list of states on match,
 *  list of states on mismatch
 * ).
 * The automaton state is now a list of all pattern matches that we're looking for. The transition tells us
 * with what states to go to from the current state if it's matched and what states to go to if it's not.
 *
 * Consider $..a..b.c..d
 *
 * We get the root and then we consider "..a". Our triples are:
 * recursive a = ("a", [recursive b], [recursive a])
 * recursive b = ("b", [direct c, recursive b], [recursive b]),
 * direct c = ("c", [recursive d], []),
 * recursive d = ("d", [accept, recursive d], [recursive d])
 *
 * And accept is a special state that adds one to the count and does nothing else.
 *
 * Let's run over an example document to see the transitions in action:
 *
 * {
 *    "a": {
 *       "y": {
 *          "b": {
 *            "c": {
 *              "d": "value1"
 *            },
 *            "z": {
 *              "b": {
 *                "c": {
 *                  "d": "value2"
 *                }
 *              }
 *            }
 *          }
 *       }
 *    }
 * }
 *
 * We match the root and our state is [recursive a].
 * We have "a", which matches, so recursive a is replaced by [recursive b].
 * Next we have "y", but it doesn't match, so we replace recursive b with itself.
 * State is still [recursive b], which is exactly what we want, we execute the same state
 * recursively.
 * Now we have "b" which does match, so we replace recursive b with [direct c, recursive b].
 * The idea is that while we might be able to find the full "b.c" match, there's also a chance
 * that a label won't match c. But in that case we want to recursively look for another "b", in case
 * "b.c" is nested deeper.
 * Inside "c" we find "d", which matches, and thus we get [accept, recursive d, recursive b]. This
 * adds 1 to our counter, but "d" is not an object or a list, so the reset of the state is discarded
 * as we return up the stack.
 * Next we exit "c", our state returning to [direct c, recursive b].
 * We see "z". It doesn't match direct c, so it is replaced by [] and disappears. It doesn't
 * match recursive b either, so it loops to [recursive b].
 * The rest of the story is pretty easy:
 * [recursive b] -> match "b" -> [direct c, recursive b] -> match "c" -> [recursive d, recursive b] -> match "d" -> [accept, recursive d, recursive b]
 *
 * All possible states can be constructed during compilation in a straightforward manner, then we only match the labels and follow transitions.
 */

enum State<'q> {
    Internal(Weak<InternalState<'q>>),
    Accepting,
}

#[cfg(debug_assertions)]
impl State<'_> {
    pub(crate) fn debug_name(&self) -> String {
        match self {
            State::Accepting => "Accepting".to_owned(),
            State::Internal(state) => state.upgrade().unwrap().debug_name.clone(),
        }
    }
}

#[cfg(debug_assertions)]
impl core::fmt::Debug for State<'_> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Accepting => write!(f, "Accepting"),
            Self::Internal(state) => write!(f, "{:?}", state),
        }
    }
}

struct InternalState<'q> {
    label: &'q Label,
    transition_on_match: State<'q>,
    recursive_transition: Option<Weak<InternalState<'q>>>,
    #[cfg(debug_assertions)]
    debug_name: String,
}

#[cfg(debug_assertions)]
impl core::fmt::Debug for InternalState<'_> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let label_string = std::str::from_utf8(self.label.bytes()).unwrap_or("[invalid utf8]");

        match &self.recursive_transition {
            Some(recursive_state) => write!(
                f,
                "{}: (\"{}\" -> {:?}, loop {:?})",
                self.debug_name,
                label_string,
                &self.transition_on_match.debug_name(),
                recursive_state.upgrade().unwrap().debug_name
            ),
            None => write!(
                f,
                "{}: (\"{}\" -> {:?})",
                self.debug_name,
                label_string,
                &self.transition_on_match.debug_name()
            ),
        }
    }
}

/// New version of [`StackBasedRunner`](`crate::stack_based::StackBasedRunner`).
pub struct StackBasedRunner<'q> {
    states: Vec<Rc<InternalState<'q>>>,
    initial_state: State<'q>,
}

#[cfg(debug_assertions)]
impl core::fmt::Debug for StackBasedRunner<'_> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        writeln!(f, "NewStackBasedRunner {{\n")?;

        for (i, s) in self.states.iter().rev().enumerate() {
            write!(f, "state.{}: ", i)?;
            writeln!(f, "{:?}\n", s)?;
        }

        writeln!(f, "}}")
    }
}

impl<'q> StackBasedRunner<'q> {
    /// Compile a query into a [`NewStackBasedRunner`].
    pub fn compile_query(query: &'q JsonPathQuery) -> Self {
        let mut this = StackBasedRunner {
            states: vec![],
            initial_state: State::Accepting,
        };

        if let Some(first_node) = query.root().child() {
            let first_state = this.compile_states(first_node);
            this.initial_state = first_state;
        }

        debug!("Created states:\n {:?}", this);

        this
    }

    fn compile_states(&mut self, node: &'q JsonPathQueryNode) -> State<'q> {
        let (is_recursive, label, next_node) = match node {
            JsonPathQueryNode::Descendant(child) => match child.as_ref() {
                JsonPathQueryNode::Label(label, next_node) => (true, label, next_node),
                _ => panic!("invalid query"),
            },
            JsonPathQueryNode::Child(child) => match child.as_ref() {
                JsonPathQueryNode::Label(label, next_node) => (false, label, next_node),
                _ => panic!("invalid query"),
            },
            _ => panic!("invalid query"),
        };

        let next_state = next_node.as_ref().map(|n| self.compile_states(n.as_ref()));

        let state = Rc::new_cyclic(|this| {
            let transition_on_match = next_state.unwrap_or(State::Accepting);
            let recursive_transition = if is_recursive {
                Some(this.clone())
            } else {
                None
            };

            self.create_state(
                is_recursive,
                label,
                transition_on_match,
                recursive_transition,
            )
        });

        self.states.push(state.clone());

        State::Internal(Rc::downgrade(&state))
    }

    #[inline(always)]
    #[cfg(not(debug_assertions))]
    fn create_state(
        &self,
        _is_recursive: bool,
        label: &'q Label,
        transition_on_match: State<'q>,
        recursive_transition: Option<Weak<InternalState<'q>>>,
    ) -> InternalState<'q> {
        InternalState {
            label,
            transition_on_match,
            recursive_transition,
        }
    }

    #[inline(always)]
    #[cfg(debug_assertions)]
    fn create_state(
        &self,
        is_recursive: bool,
        label: &'q Label,
        transition_on_match: State<'q>,
        recursive_transition: Option<Weak<InternalState<'q>>>,
    ) -> InternalState<'q> {
        let label_string = std::str::from_utf8(label.bytes()).unwrap_or("[invalid utf8]");
        let name = if is_recursive { "recursive" } else { "direct" };
        let debug_name = format!("{}.{name} {label_string}", self.states.len());

        InternalState {
            label,
            transition_on_match,
            recursive_transition,
            debug_name,
        }
    }
}

impl Runner for StackBasedRunner<'_> {
    fn count(&self, input: &Input) -> result::CountResult {
        let aligned_bytes: &AlignedSlice<alignment::Page> = input;
        let classifier = classify_structural_characters(aligned_bytes.relax_alignment());
        let mut execution_ctx = ExecutionContext::new(classifier, input);
        execution_ctx.run(&self.initial_state);
        result::CountResult {
            count: execution_ctx.count,
        }
    }
}

struct ExecutionContext<'b, I>
where
    I: StructuralIterator<'b>,
{
    classifier: Peekable<I>,
    count: usize,
    bytes: &'b [u8],
    #[cfg(debug_assertions)]
    depth: usize,
}

impl<'b, I> ExecutionContext<'b, I>
where
    I: StructuralIterator<'b>,
{
    #[cfg(debug_assertions)]
    pub(crate) fn new(classifier: I, bytes: &'b [u8]) -> Self {
        Self {
            classifier: classifier.peekable(),
            count: 0,
            bytes,
            depth: 0,
        }
    }

    #[cfg(not(debug_assertions))]
    pub(crate) fn new(classifier: I, bytes: &'b [u8]) -> Self {
        Self {
            classifier: classifier.peekable(),
            count: 0,
            bytes,
        }
    }

    pub(crate) fn run(&mut self, state: &State) {
        if let Some(Structural::Opening(_)) = self.classifier.next() {
            match state {
                State::Accepting => self.count += 1,
                State::Internal(state) => {
                    self.increase_depth();
                    let state_rc = state.upgrade().unwrap();

                    if state_rc.recursive_transition.is_some() {
                        self.run_internal(
                            smallvec![],
                            state.upgrade().unwrap().recursive_transition.clone(),
                        );
                    } else {
                        self.run_internal(smallvec![state.clone()], None);
                    }
                }
            }
        }
    }

    fn run_internal<'a>(
        &mut self,
        mut state: SmallVec<[Weak<InternalState<'a>>; 2]>,
        recursive_state: Option<Weak<InternalState<'a>>>,
    ) {
        self.increase_depth();

        if let Some(recursive_state) = &recursive_state {
            state.push(recursive_state.clone());
        }

        debug!(
            "[{}] Executing states: {:?}",
            self.depth,
            state
                .iter()
                .map(|s| s.upgrade().unwrap().debug_name.clone())
                .collect::<Vec<_>>()
        );
        debug!(
            "[{}] Recursive state: {:?}",
            self.depth,
            recursive_state
                .as_ref()
                .map(|s| s.upgrade().unwrap().debug_name.clone())
        );

        loop {
            match self.classifier.next() {
                Some(Structural::Colon(idx)) => {
                    let mut new_recursive_state = recursive_state.clone();
                    let states_iter = state.iter().filter_map(|s| {
                        self.match_and_transition(s, idx, &mut new_recursive_state)
                    });
                    let states = states_iter.collect::<SmallVec<_>>();

                    if let Some(Structural::Opening(_)) = self.classifier.peek() {
                        self.classifier.next();

                        if let Some(new_recursive_state) = &new_recursive_state {
                            if recursive_state
                                .as_ref()
                                .map_or(true, |p| !p.ptr_eq(new_recursive_state))
                            {
                                debug!(
                                    "[{}] New recursive checkpoint, flushing states.",
                                    self.depth,
                                );
                                self.run_internal(smallvec![], Some(new_recursive_state.clone()));
                            } else {
                                self.run_internal(states, recursive_state.clone());
                            }
                        } else {
                            self.run_internal(states, recursive_state.clone());
                        }
                    }
                }
                Some(Structural::Opening(_)) if recursive_state.is_some() => {
                    debug!("[{}] New object, recursing.", self.depth,);
                    self.run_internal(smallvec![], recursive_state.clone());
                }
                _ => {
                    debug!("[{}] Object or stream end, stack pop.", self.depth,);
                    self.decrease_depth();
                    return;
                }
            };
        }
    }

    fn match_and_transition<'a>(
        &mut self,
        state: &Weak<InternalState<'a>>,
        colon_idx: usize,
        recursive_state: &mut Option<Weak<InternalState<'a>>>,
    ) -> Option<Weak<InternalState<'a>>> {
        let state = state.upgrade().unwrap();
        debug!(
            "[{}] Attempting to transition from '{}'",
            self.depth,
            state.debug_name.clone()
        );

        let len = state.label.len();
        let is_match = if colon_idx >= len + 2 {
            let mut closing_quote_idx = colon_idx - 1;
            while self.bytes[closing_quote_idx] != b'"' {
                closing_quote_idx -= 1;
            }

            let opening_quote_idx = closing_quote_idx - len - 1;
            let slice = &self.bytes[opening_quote_idx..closing_quote_idx + 1];

            debug!(
                "[{}] Inspecting slice: {}",
                self.depth,
                std::str::from_utf8(slice).unwrap_or("[invalid utf8]")
            );

            slice == state.label.bytes_with_quotes()
        } else {
            false
        };

        if is_match {
            debug!("[{}] Matched label.", self.depth,);

            match &state.transition_on_match {
                State::Accepting => {
                    debug!("[{}] Accept.", self.depth,);
                    self.count += 1;
                    None
                }
                State::Internal(next_state) => {
                    if let Some(Structural::Opening(_)) = self.classifier.peek() {
                        debug!(
                            "[{}] Transitioning to {}",
                            self.depth,
                            next_state.upgrade().unwrap().debug_name.clone()
                        );
                        let recursive_transition =
                            &next_state.upgrade().unwrap().recursive_transition;
                        *recursive_state = recursive_transition.clone();
                        Some(next_state.clone())
                    } else {
                        None
                    }
                }
            }
        } else {
            None
        }
    }

    #[inline(always)]
    #[cfg(debug_assertions)]
    fn increase_depth(&mut self) {
        self.depth += 1;
    }

    #[inline(always)]
    #[cfg(not(debug_assertions))]
    fn increase_depth(&mut self) {}

    #[inline(always)]
    #[cfg(debug_assertions)]
    fn decrease_depth(&mut self) {
        self.depth -= 1;
    }

    #[inline(always)]
    #[cfg(not(debug_assertions))]
    fn decrease_depth(&mut self) {}
}
