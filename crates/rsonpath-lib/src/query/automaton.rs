//! Automaton representations of a JSONPath query.

mod minimizer;
mod nfa;
mod small_set;
mod state;

pub use state::{State, StateAttributes};

use super::{error::CompilerError, JsonPathQuery, Label};
use crate::debug;
use nfa::NondeterministicAutomaton;
use smallvec::SmallVec;
use std::{fmt::Display, ops::Index};

/// A minimal, deterministic automaton representing a JSONPath query.
#[derive(Debug, PartialEq, Eq)]
pub struct Automaton<'q> {
    states: Vec<StateTable<'q>>,
}

/// A single transition of an [`Automaton`].
type Transition<'q> = (&'q Label, State);

/// A transition table of a single [`State`] of an [`Automaton`].
///
/// Contains transitions triggered by matching labels, and a fallback transition
/// triggered when none of the label transitions match.
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
            && self
                .transitions
                .iter()
                .all(|x| other.transitions.contains(x))
            && other
                .transitions
                .iter()
                .all(|x| self.transitions.contains(x))
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
    /// A transition is triggered if the [`Label`] is matched and leads
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
                writeln!(f, "  {i} -> {} [label=\"{}\"]", state.0, label.display(),)?
            }
            writeln!(f, "  {i} -> {} [label=\"*\"]", transitions.fallback_state.0)?;
        }
        write!(f, "}}")?;
        Ok(())
    }
}
