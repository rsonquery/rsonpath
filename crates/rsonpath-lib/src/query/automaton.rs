//! Automaton representations of a JSONPath query.
mod minimizer;
mod nfa;
mod small_set;
mod state;

pub use state::{State, StateAttributes};

use super::{error::CompilerError, JsonPathQuery, JsonString, NonNegativeArrayIndex};
use crate::debug;
use nfa::NondeterministicAutomaton;
use smallvec::SmallVec;
use std::{borrow::Borrow, fmt::Display, ops::Index};

/// A minimal, deterministic automaton representing a JSONPath query.
#[derive(Debug, PartialEq, Eq)]
pub struct Automaton<'q> {
    states: Vec<StateTable<'q>>,
}

/// Represent the distinct methods of moving on a match between states.
#[derive(Debug, Copy, PartialEq, Clone, Eq)]
pub enum TransitionLabel<'q> {
    /// Transition when a JSON member name matches a [`JsonString`]i.
    ObjectMember(&'q JsonString),
    /// Transition on the n-th element of an array, with n specified by a [`NonNegativeArrayIndex`].
    ArrayIndex(NonNegativeArrayIndex),
}

impl<'q> TransitionLabel<'q> {
    ///Return the textual [`JsonString`] being wrapped if so. Returns [`None`] otherwise.
    #[must_use]
    #[inline(always)]
    pub fn get_member_name(&self) -> Option<&'q JsonString> {
        match self {
            TransitionLabel::ObjectMember(name) => Some(name),
            TransitionLabel::ArrayIndex(_) => None,
        }
    }

    ///Return the [`NonNegativeArrayIndex`] being wrapped if so. Returns [`None`] otherwise.
    #[must_use]
    #[inline(always)]
    pub fn get_array_index(&'q self) -> Option<&'q NonNegativeArrayIndex> {
        match self {
            TransitionLabel::ArrayIndex(name) => Some(name),
            TransitionLabel::ObjectMember(_) => None,
        }
    }

    /// Wraps a [`JsonString`] in a [`TransitionLabel`].
    #[must_use]
    #[inline(always)]
    pub fn new_object_member(member_name: &'q JsonString) -> Self {
        TransitionLabel::ObjectMember(member_name)
    }

    /// Wraps a [`NonNegativeArrayIndex`] in a [`TransitionLabel`].
    #[must_use]
    #[inline(always)]
    pub fn new_array_index(index: NonNegativeArrayIndex) -> Self {
        TransitionLabel::ArrayIndex(index)
    }
}

impl<'q> From<&'q JsonString> for TransitionLabel<'q> {
    #[must_use]
    #[inline(always)]
    fn from(member_name: &'q JsonString) -> Self {
        TransitionLabel::new_object_member(member_name)
    }
}

impl Display for TransitionLabel<'_> {
    #[inline(always)]
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            TransitionLabel::ObjectMember(name) => write!(f, "{}", name.display()),
            TransitionLabel::ArrayIndex(index) => write!(f, "{}", index.get_index()),
        }
    }
}

impl<T: Borrow<NonNegativeArrayIndex>> From<T> for TransitionLabel<'_> {
    #[must_use]
    #[inline(always)]
    fn from(index: T) -> Self {
        TransitionLabel::new_array_index(*index.borrow())
    }
}

/// A single transition of an [`Automaton`].
type Transition<'q> = (TransitionLabel<'q>, State);

/// A transition table of a single [`State`] of an [`Automaton`].
///
/// Contains transitions triggered by matching member names or array indices, and a fallback transition
/// triggered when none of the labelled transitions match.
#[derive(Debug)]
pub struct StateTable<'q> {
    attributes: StateAttributes,
    transitions: SmallVec<[Transition<'q>; 2]>,
    fallback_state: State,
}

impl<'q> Default for StateTable<'q> {
    #[inline]
    fn default() -> Self {
        Self {
            attributes: StateAttributes::default(),
            transitions: Default::default(),
            fallback_state: State(0),
        }
    }
}

impl<'q> PartialEq for StateTable<'q> {
    #[inline]
    fn eq(&self, other: &Self) -> bool {
        self.fallback_state == other.fallback_state
            && self.transitions.len() == other.transitions.len()
            && self.transitions.iter().all(|x| other.transitions.contains(x))
            && other.transitions.iter().all(|x| self.transitions.contains(x))
            && self.attributes == other.attributes
    }
}

impl<'q> Eq for StateTable<'q> {}

impl<'q> Index<State> for Automaton<'q> {
    type Output = StateTable<'q>;

    #[inline(always)]
    fn index(&self, index: State) -> &Self::Output {
        &self.states[index.0 as usize]
    }
}

impl<'q> Automaton<'q> {
    /// Convert a [`JsonPathQuery`] into a minimal deterministic automaton.
    ///
    /// # Errors
    /// - [`CompilerError::QueryTooComplex`] raised if the query is too complex
    /// and the automaton size was exceeded.
    /// - [`CompilerError::NotSupported`] raised if the query contains elements
    /// not yet supported by the compiler.
    #[inline]
    pub fn new(query: &'q JsonPathQuery) -> Result<Self, CompilerError> {
        let nfa = NondeterministicAutomaton::new(query)?;
        debug!("NFA: {}", nfa);
        Automaton::minimize(nfa)
    }

    /// Returns whether this automaton represents an empty JSONPath query ('$').
    ///
    /// # Examples
    /// ```rust
    /// # use rsonpath_lib::query::*;
    /// # use rsonpath_lib::query::automaton::*;
    /// let query = JsonPathQuery::parse("$").unwrap();
    /// let automaton = Automaton::new(&query).unwrap();
    ///
    /// assert!(automaton.is_empty_query());
    /// ```
    ///
    /// ```rust
    /// # use rsonpath_lib::query::*;
    /// # use rsonpath_lib::query::automaton::*;
    /// let query = JsonPathQuery::parse("$.a").unwrap();
    /// let automaton = Automaton::new(&query).unwrap();
    ///
    /// assert!(!automaton.is_empty_query());
    /// ```
    #[must_use]
    #[inline(always)]
    pub fn is_empty_query(&self) -> bool {
        self.states.len() == 2
    }

    /// Returns the rejecting state of the automaton.
    ///
    /// The state is defined as the unique state from which there
    /// exists no accepting run. If the query automaton reaches this state,
    /// the current subtree is guaranteed to have no matches.
    #[must_use]
    #[inline(always)]
    #[allow(clippy::unused_self)] /* This is for stability. If the implementation changes so that
                                   * this is not always a 0 we don't want to have to change callsites.
                                   */
    pub fn rejecting_state(&self) -> State {
        State(0)
    }

    /// Returns the initial state of the automaton.
    ///
    /// Query execution should start from this state.
    #[must_use]
    #[inline(always)]
    #[allow(clippy::unused_self)] /* This is for stability. If the implementation changes so that
                                   * this is not always a 1 we don't want to have to change callsites.
                                   */
    pub fn initial_state(&self) -> State {
        State(1)
    }

    /// Returns whether the given state is accepting.
    ///
    /// # Example
    /// ```rust
    /// # use rsonpath_lib::query::*;
    /// # use rsonpath_lib::query::automaton::*;
    /// let query = JsonPathQuery::parse("$.a").unwrap();
    /// let automaton = Automaton::new(&query).unwrap();
    /// let state_2 = automaton[automaton.initial_state()].transitions()[0].1;
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
    /// # use rsonpath_lib::query::*;
    /// # use rsonpath_lib::query::automaton::*;
    /// let query = JsonPathQuery::parse("$[2]").unwrap();
    /// let automaton = Automaton::new(&query).unwrap();
    /// let state = automaton.initial_state();
    ///
    /// assert!(automaton.has_any_array_item_transition(state));
    /// ```
    #[must_use]
    #[inline(always)]
    pub fn has_any_array_item_transition(&self, state: State) -> bool {
        self[state]
            .transitions()
            .iter()
            .any(|t| matches!(t, (TransitionLabel::ArrayIndex(_), _)))
    }

    /// Returns whether the given state is accepting an item in a list.
    ///
    /// # Example
    /// ```rust
    /// # use rsonpath_lib::query::*;
    /// # use rsonpath_lib::query::automaton::*;
    /// let query = JsonPathQuery::parse("$[2]").unwrap();
    /// let automaton = Automaton::new(&query).unwrap();
    /// let state = automaton.initial_state();
    ///
    /// assert!(automaton.has_any_array_item_transition_to_accepting(state));
    /// ```
    #[must_use]
    #[inline(always)]
    pub fn has_any_array_item_transition_to_accepting(&self, state: State) -> bool {
        self[state].transitions().iter().any(|t| match t {
            (TransitionLabel::ArrayIndex(_), s) => self.is_accepting(*s),
            _ => false,
        })
    }

    /// Returns whether the given state is accepting the first item in a list.
    ///
    /// # Example
    /// ```rust
    /// # use rsonpath_lib::query::*;
    /// # use rsonpath_lib::query::automaton::*;
    /// let query = JsonPathQuery::parse("$[0]").unwrap();
    /// let automaton = Automaton::new(&query).unwrap();
    /// let state = automaton.initial_state();
    ///
    /// assert!(automaton.has_first_array_index_transition_to_accepting(state));
    /// ```
    /// ```rust
    /// # use rsonpath_lib::query::*;
    /// # use rsonpath_lib::query::automaton::*;
    /// let query = JsonPathQuery::parse("$[1]").unwrap();
    /// let automaton = Automaton::new(&query).unwrap();
    /// let state = automaton.initial_state();
    ///
    /// assert!(!automaton.has_first_array_index_transition_to_accepting(state));
    /// ```
    #[must_use]
    #[inline(always)]
    pub fn has_first_array_index_transition_to_accepting(&self, state: State) -> bool {
        self.has_array_index_transition_to_accepting(state, &NonNegativeArrayIndex::ZERO)
    }

    /// Returns whether the given state is accepting the item at a given index in a list.
    ///
    /// # Example
    /// ```rust
    /// # use rsonpath_lib::query::*;
    /// # use rsonpath_lib::query::automaton::*;
    /// let query = JsonPathQuery::parse("$[1]").unwrap();
    /// let automaton = Automaton::new(&query).unwrap();
    /// let state = automaton.initial_state();
    /// let match_index_1 = NonNegativeArrayIndex::new(1);
    /// let match_index_2 = NonNegativeArrayIndex::new(2);
    ///
    /// assert!(automaton.has_array_index_transition_to_accepting(state, &match_index_1));
    /// assert!(!automaton.has_array_index_transition_to_accepting(state, &match_index_2));
    /// ```
    #[must_use]
    #[inline(always)]
    pub fn has_array_index_transition_to_accepting(&self, state: State, match_index: &NonNegativeArrayIndex) -> bool {
        self[state].transitions().iter().any(|t| match t {
            (TransitionLabel::ArrayIndex(i), s) => i.eq(match_index) && self.is_accepting(*s),
            _ => false,
        })
    }

    /// Returns whether the given state has any transitions
    /// (labelled or fallback) to an accepting state.
    ///
    /// # Example
    /// ```rust
    /// # use rsonpath_lib::query::*;
    /// # use rsonpath_lib::query::automaton::*;
    /// let query = JsonPathQuery::parse("$.a").unwrap();
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
    /// # use rsonpath_lib::query::*;
    /// # use rsonpath_lib::query::automaton::*;
    /// let query = JsonPathQuery::parse("$.a").unwrap();
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
    /// # use rsonpath_lib::query::*;
    /// # use rsonpath_lib::query::automaton::*;
    /// let query = JsonPathQuery::parse("$.a").unwrap();
    /// let automaton = Automaton::new(&query).unwrap();
    ///
    /// assert!(automaton.is_unitary(automaton.initial_state()));
    /// ```
    #[must_use]
    #[inline(always)]
    pub fn is_unitary(&self, state: State) -> bool {
        self[state].attributes.is_unitary()
    }

    fn minimize(nfa: NondeterministicAutomaton<'q>) -> Result<Self, CompilerError> {
        minimizer::minimize(nfa)
    }
}

impl<'q> StateTable<'q> {
    /// Returns the state to which a fallback transition leads.
    ///
    /// A fallback transition is the catch-all transition triggered
    /// if none of the transitions were triggered.
    #[must_use]
    #[inline(always)]
    pub fn fallback_state(&self) -> State {
        self.fallback_state
    }

    /// Returns the collection of labelled transitions from this state.
    ///
    /// A transition is triggered if the [`TransitionLabel`] is matched and leads
    /// to the contained [`State`].
    #[must_use]
    #[inline(always)]
    pub fn transitions(&self) -> &[Transition<'q>] {
        &self.transitions
    }
}

impl<'q> Display for Automaton<'q> {
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

            let attrs = vec![shape, "style=filled", "gradientangle=45", color_one, color_two].join(" ");

            writeln!(f, "node [{attrs}]; {i}")?;
        }

        for (i, transitions) in self.states.iter().enumerate() {
            for (label, state) in transitions.transitions.iter() {
                writeln!(f, "  {i} -> {} [label=\"{}\"]", state.0, label,)?
            }
            writeln!(f, "  {i} -> {} [label=\"*\"]", transitions.fallback_state.0)?;
        }
        write!(f, "}}")?;
        Ok(())
    }
}
