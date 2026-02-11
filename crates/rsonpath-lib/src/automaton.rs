//! Automaton representations of a JSONPath query.
mod array_transition_set;
pub mod error;
mod minimizer;
mod nfa;
mod small_set;
mod state;

pub use state::{State, StateAttributes};

use crate::{automaton::error::CompilerError, debug, string_pattern::StringPattern};
use nfa::NondeterministicAutomaton;
use rsonpath_syntax::{num::JsonUInt, JsonPathQuery};
use smallvec::SmallVec;
use std::{fmt::Display, ops::Index, sync::Arc};

/// A minimal, deterministic automaton representing a JSONPath query.
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Automaton {
    states: Vec<StateTable>,
}

/// Transition when a JSON member name matches a [`StringPattern`].
pub type MemberTransition = (Arc<StringPattern>, State);

/// Transition on elements of an array with indices specified by either a single index
/// or a simple slice expression.
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct ArrayTransition {
    label: ArrayTransitionLabel,
    target: State,
}

/// Represent the distinct methods of moving on a match between states.
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[derive(Debug, Copy, PartialEq, Clone, Eq)]
pub(super) enum ArrayTransitionLabel {
    /// Transition on the n-th element of an array, with n specified by a [`JsonUInt`].
    Index(JsonUInt),
    /// Transition on elements of array matched by a slice expression - bounds and a step.
    Slice(SimpleSlice),
}

/// A transition table of a single [`State`] of an [`Automaton`].
///
/// Contains transitions triggered by matching member names or array indices, and a fallback transition
/// triggered when none of the labelled transitions match.
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[derive(Debug, Clone)]
pub struct StateTable {
    attributes: StateAttributes,
    member_transitions: SmallVec<[MemberTransition; 2]>,
    array_transitions: SmallVec<[ArrayTransition; 2]>,
    fallback_state: State,
}

#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[derive(Debug, Copy, PartialEq, Clone, Eq)]
pub(crate) struct SimpleSlice {
    start: JsonUInt,
    end: Option<JsonUInt>,
    step: JsonUInt,
}

impl Default for StateTable {
    #[inline]
    fn default() -> Self {
        Self {
            attributes: StateAttributes::default(),
            member_transitions: SmallVec::default(),
            array_transitions: SmallVec::default(),
            fallback_state: State(0),
        }
    }
}

impl PartialEq for StateTable {
    #[inline]
    fn eq(&self, other: &Self) -> bool {
        return self.fallback_state == other.fallback_state
            && set_eq(&self.array_transitions, &other.array_transitions)
            && set_eq(&self.member_transitions, &other.member_transitions)
            && self.attributes == other.attributes;

        #[inline(always)]
        fn set_eq<T: Eq, A: smallvec::Array<Item = T>>(left: &SmallVec<A>, right: &SmallVec<A>) -> bool {
            left.len() == right.len()
                && left.iter().all(|x| right.contains(x))
                && right.iter().all(|x| left.contains(x))
        }
    }
}

impl Eq for StateTable {}

impl Index<State> for Automaton {
    type Output = StateTable;

    #[inline(always)]
    fn index(&self, index: State) -> &Self::Output {
        &self.states[index.0 as usize]
    }
}

impl ArrayTransition {
    pub(crate) fn new(label: ArrayTransitionLabel, target: State) -> Self {
        Self { label, target }
    }

    #[inline(always)]
    pub(crate) fn target_state(&self) -> State {
        self.target
    }

    #[inline(always)]
    pub(crate) fn matches(&self, index: JsonUInt) -> bool {
        self.label.matches(index)
    }
}

impl ArrayTransitionLabel {
    pub(crate) fn matches(&self, index: JsonUInt) -> bool {
        match self {
            Self::Index(i) => index.eq(i),
            Self::Slice(s) => s.contains(index),
        }
    }

    fn matches_at_most_once(&self) -> bool {
        match self {
            Self::Index(_) => true,
            Self::Slice(slice) => {
                slice.step == JsonUInt::ZERO && slice.end.is_some_and(|end| slice.start.as_u64() + 1 >= end.as_u64())
            }
        }
    }
}

impl From<JsonUInt> for ArrayTransitionLabel {
    #[inline(always)]
    fn from(index: JsonUInt) -> Self {
        Self::Index(index)
    }
}

impl From<SimpleSlice> for ArrayTransitionLabel {
    #[inline(always)]
    fn from(slice: SimpleSlice) -> Self {
        Self::Slice(slice)
    }
}

impl Automaton {
    /// Convert a [`JsonPathQuery`] into a minimal deterministic automaton.
    ///
    /// # Errors
    /// - [`CompilerError::QueryTooComplex`] raised if the query is too complex
    ///   and the automaton size was exceeded.
    /// - [`CompilerError::NotSupported`] raised if the query contains elements
    ///   not yet supported by the compiler.
    #[inline]
    pub fn new(query: &JsonPathQuery) -> Result<Self, CompilerError> {
        let nfa = NondeterministicAutomaton::new(query)?;
        debug!("NFA: {}", nfa);
        Self::minimize(nfa)
    }

    /// Returns whether this automaton represents the select-root JSONPath query ('$').
    ///
    /// # Examples
    /// ```rust
    /// # use rsonpath::automaton::*;
    /// let query = rsonpath_syntax::parse("$").unwrap();
    /// let automaton = Automaton::new(&query).unwrap();
    ///
    /// assert!(automaton.is_select_root_query());
    /// ```
    ///
    /// ```rust
    /// # use rsonpath::automaton::*;
    /// let query = rsonpath_syntax::parse("$.a").unwrap();
    /// let automaton = Automaton::new(&query).unwrap();
    ///
    /// assert!(!automaton.is_select_root_query());
    /// ```
    #[must_use]
    #[inline(always)]
    pub fn is_select_root_query(&self) -> bool {
        self.states.len() == 2
            && self.states[1].array_transitions.is_empty()
            && self.states[1].member_transitions.is_empty()
            && self.states[1].fallback_state == State(0)
            && self.states[1].attributes.is_accepting()
    }

    /// Returns whether this automaton represents an empty JSONPath query that cannot accept anything.
    ///
    /// A query like this can be created by, for example, putting a trivially false filter
    /// or an empty slice into the query.
    ///
    /// # Examples
    /// ```rust
    /// # use rsonpath::automaton::*;
    /// let query = rsonpath_syntax::parse("$[::0]").unwrap();
    /// let automaton = Automaton::new(&query).unwrap();
    ///
    /// assert!(automaton.is_empty_query());
    /// ```
    ///
    /// ```rust
    /// # use rsonpath::automaton::*;
    /// let query = rsonpath_syntax::parse("$").unwrap();
    /// let automaton = Automaton::new(&query).unwrap();
    ///
    /// assert!(!automaton.is_empty_query());
    /// ```
    #[must_use]
    #[inline(always)]
    pub fn is_empty_query(&self) -> bool {
        self.states.len() == 2
            && self.states[1].array_transitions.is_empty()
            && self.states[1].member_transitions.is_empty()
            && self.states[1].fallback_state == State(0)
            && !self.states[1].attributes.is_accepting()
    }

    /// Returns the rejecting state of the automaton.
    ///
    /// The state is defined as the unique state from which there
    /// exists no accepting run. If the query automaton reaches this state,
    /// the current subtree is guaranteed to have no matches.
    #[must_use]
    #[inline(always)]
    #[allow(
        clippy::unused_self,
        reason = "This is for stability. If the implementation changes so that
                                   this is not always a 0 we don't want to have to change callsites."
    )]
    pub fn rejecting_state(&self) -> State {
        State(0)
    }

    /// Returns the initial state of the automaton.
    ///
    /// Query execution should start from this state.
    #[must_use]
    #[inline(always)]
    #[allow(
        clippy::unused_self,
        reason = "This is for stability. If the implementation changes so that
                                   this is not always a 1 we don't want to have to change callsites."
    )]
    pub fn initial_state(&self) -> State {
        State(1)
    }

    /// Returns whether the given state is accepting.
    ///
    /// # Example
    /// ```rust
    /// # use rsonpath::automaton::*;
    /// let query = rsonpath_syntax::parse("$.a").unwrap();
    /// let automaton = Automaton::new(&query).unwrap();
    /// let state_2 = automaton[automaton.initial_state()].member_transitions()[0].1;
    ///
    /// assert!(automaton.is_accepting(state_2));
    /// ```
    #[must_use]
    #[inline(always)]
    pub fn is_accepting(&self, state: State) -> bool {
        self[state].attributes.is_accepting()
    }

    /// Returns whether the given state transitions to any list.
    ///
    /// # Example
    /// ```rust
    /// # use rsonpath::automaton::*;
    /// let query = rsonpath_syntax::parse("$[2]").unwrap();
    /// let automaton = Automaton::new(&query).unwrap();
    /// let state = automaton.initial_state();
    ///
    /// assert!(automaton.has_any_array_item_transition(state));
    /// ```
    #[must_use]
    #[inline(always)]
    pub fn has_any_array_item_transition(&self, state: State) -> bool {
        self[state].attributes.has_array_transition()
    }

    /// Returns whether the given state is accepting the first item in a list.
    ///
    /// # Example
    /// ```rust
    /// # use rsonpath::automaton::*;
    /// let query = rsonpath_syntax::parse("$[0]").unwrap();
    /// let automaton = Automaton::new(&query).unwrap();
    /// let state = automaton.initial_state();
    ///
    /// assert!(automaton.has_first_array_index_transition_to_accepting(state));
    /// ```
    /// ```rust
    /// # use rsonpath::automaton::*;
    /// let query = rsonpath_syntax::parse("$[1]").unwrap();
    /// let automaton = Automaton::new(&query).unwrap();
    /// let state = automaton.initial_state();
    ///
    /// assert!(!automaton.has_first_array_index_transition_to_accepting(state));
    /// ```
    #[must_use]
    #[inline(always)]
    pub fn has_first_array_index_transition_to_accepting(&self, state: State) -> bool {
        self.has_array_index_transition_to_accepting(state, &JsonUInt::ZERO)
    }

    /// Returns whether the given state is accepting the item at a given index in a list.
    ///
    /// # Example
    /// ```rust
    /// # use rsonpath_syntax::num::JsonUInt;
    /// # use rsonpath::automaton::*;
    /// let query = rsonpath_syntax::parse("$[1]").unwrap();
    /// let automaton = Automaton::new(&query).unwrap();
    /// let state = automaton.initial_state();
    /// let match_index_1 = JsonUInt::try_from(1).unwrap();
    /// let match_index_2 = JsonUInt::try_from(2).unwrap();
    ///
    /// assert!(automaton.has_array_index_transition_to_accepting(state, &match_index_1));
    /// assert!(!automaton.has_array_index_transition_to_accepting(state, &match_index_2));
    /// ```
    #[must_use]
    #[inline(always)]
    pub fn has_array_index_transition_to_accepting(&self, state: State, match_index: &JsonUInt) -> bool {
        let state = &self[state];
        state.attributes.has_array_transition_to_accepting()
            && state
                .array_transitions()
                .iter()
                .any(|trans| self.is_accepting(trans.target) && trans.matches(*match_index))
    }

    /// Returns whether the given state has any transitions
    /// (labelled or fallback) to an accepting state.
    ///
    /// # Example
    /// ```rust
    /// # use rsonpath::automaton::*;
    /// let query = rsonpath_syntax::parse("$.a").unwrap();
    /// let automaton = Automaton::new(&query).unwrap();
    ///
    /// assert!(automaton.has_transition_to_accepting(automaton.initial_state()));
    /// ```
    #[must_use]
    #[inline(always)]
    pub fn has_transition_to_accepting(&self, state: State) -> bool {
        self[state].attributes.has_transition_to_accepting()
    }

    /// Returns whether the given state is rejecting, i.e.
    /// there exist no accepting runs from it.
    ///
    /// # Example
    /// ```rust
    /// # use rsonpath::automaton::*;
    /// let query = rsonpath_syntax::parse("$.a").unwrap();
    /// let automaton = Automaton::new(&query).unwrap();
    ///
    /// assert!(automaton.is_rejecting(automaton.rejecting_state()));
    /// ```
    #[must_use]
    #[inline(always)]
    pub fn is_rejecting(&self, state: State) -> bool {
        self[state].attributes.is_rejecting()
    }

    /// Returns whether the given state is unitary.
    /// A unitary state is one that has exactly one labelled transition
    /// and its fallback targets the rejecting state.
    ///
    /// Intuitively, there exists only one label that progresses towards
    /// acceptance from this state.
    ///
    /// # Example
    /// ```rust
    /// # use rsonpath::automaton::*;
    /// let query = rsonpath_syntax::parse("$.a").unwrap();
    /// let automaton = Automaton::new(&query).unwrap();
    ///
    /// assert!(automaton.is_unitary(automaton.initial_state()));
    /// ```
    #[must_use]
    #[inline(always)]
    pub fn is_unitary(&self, state: State) -> bool {
        self[state].attributes.is_unitary()
    }

    fn minimize(nfa: NondeterministicAutomaton) -> Result<Self, CompilerError> {
        minimizer::minimize(nfa)
    }
}

impl StateTable {
    /// Returns the state to which a fallback transition leads.
    ///
    /// A fallback transition is the catch-all transition triggered
    /// if none of the transitions were triggered.
    #[must_use]
    #[inline(always)]
    pub fn fallback_state(&self) -> State {
        self.fallback_state
    }

    /// Returns the collection of labelled array transitions from this state.
    ///
    /// A transition is triggered if the [`ArrayTransition`] is matched and leads
    /// to the contained [`State`].
    #[must_use]
    #[inline(always)]
    pub fn array_transitions(&self) -> &[ArrayTransition] {
        &self.array_transitions
    }

    /// Returns the collection of labelled member transitions from this state.
    ///
    /// A transition is triggered if the [`MemberTransition`] is matched and leads
    /// to the contained [`State`].
    #[must_use]
    #[inline(always)]
    pub fn member_transitions(&self) -> &[MemberTransition] {
        &self.member_transitions
    }
}

impl Display for ArrayTransitionLabel {
    #[inline(always)]
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Index(index) => write!(f, "{}", index.as_u64()),
            Self::Slice(slice) => {
                if let Some(end) = slice.end {
                    write!(f, "[{}:{}:{}]", slice.start, end, slice.step)
                } else {
                    write!(f, "[{}::{}]", slice.start, slice.step)
                }
            }
        }
    }
}

impl Display for Automaton {
    #[inline]
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        writeln!(f, "digraph {{")?;

        for (i, state) in self.states.iter().enumerate() {
            let mut color_one = "fillcolor=\"white;0.5";
            let mut color_two = ":white\"";
            let mut shape = "shape=circle";

            if state.attributes.is_accepting() {
                shape = "shape=doublecircle";
            }
            if state.attributes.is_unitary() {
                color_one = "fillcolor=\"darkgoldenrod2;0.5";
            }
            if state.attributes.has_transition_to_accepting() {
                color_two = ":dodgerblue\"";
            }
            if state.attributes.is_rejecting() {
                color_one = "fillcolor=gray";
                color_two = "";
            }

            let attrs = [shape, "style=filled", "gradientangle=45", color_one, color_two].join(" ");

            writeln!(f, "node [{attrs}]; {i}")?;
        }

        for (i, transitions) in self.states.iter().enumerate() {
            for array_transition in &transitions.array_transitions {
                match array_transition.label {
                    ArrayTransitionLabel::Index(label) => writeln!(
                        f,
                        "  {i} -> {} [label=\"[{}]\"]",
                        array_transition.target.0,
                        label.as_u64()
                    )?,
                    ArrayTransitionLabel::Slice(label) => {
                        if let Some(end) = label.end {
                            writeln!(
                                f,
                                "  {i} -> {} [label=\"[{}:{}:{}]\"]",
                                array_transition.target.0,
                                label.start.as_u64(),
                                end.as_u64(),
                                label.step.as_u64()
                            )?
                        } else {
                            writeln!(
                                f,
                                "  {i} -> {} [label=\"[{}::{}]\"]",
                                array_transition.target.0,
                                label.start.as_u64(),
                                label.step.as_u64()
                            )?
                        }
                    }
                }
            }
            for (label, state) in &transitions.member_transitions {
                writeln!(
                    f,
                    "  {i} -> {} [label=\"{}\"]",
                    state.0,
                    std::str::from_utf8(label.unquoted()).expect("labels to be valid utf8")
                )?
            }
            writeln!(f, "  {i} -> {} [label=\"*\"]", transitions.fallback_state.0)?;
        }
        write!(f, "}}")?;
        Ok(())
    }
}

impl SimpleSlice {
    fn new(start: JsonUInt, end: Option<JsonUInt>, step: JsonUInt) -> Self {
        Self { start, end, step }
    }

    #[inline(always)]
    #[must_use]
    fn contains(&self, index: JsonUInt) -> bool {
        if index < self.start {
            return false;
        }
        let offset = index.as_u64() - self.start.as_u64();
        if let Some(end) = self.end {
            index < end && offset.is_multiple_of(self.step.as_u64())
        } else {
            offset.is_multiple_of(self.step.as_u64())
        }
    }
}

#[cfg(test)]
mod tests {
    use super::SimpleSlice;
    use rsonpath_syntax::num::JsonUInt;
    use test_case::test_case;

    #[test_case(0.into(), None, 1.into(), 0.into() => true)]
    #[test_case(2.into(), None, 1.into(), 3.into() => true)]
    #[test_case(2.into(), None, 2.into(), 3.into() => false)]
    #[test_case(3.into(), None, 2.into(), 3.into() => true)]
    #[test_case(2.into(), None, 2.into(), 4.into() => true)]
    #[test_case(2.into(), Some(6.into()), 2.into(), 2.into() => true)]
    #[test_case(2.into(), Some(6.into()), 2.into(), 6.into() => false)]
    fn simple_slice_containment(start: JsonUInt, end: Option<JsonUInt>, step: JsonUInt, idx: JsonUInt) -> bool {
        let slice = SimpleSlice::new(start, end, step);
        slice.contains(idx)
    }
}
