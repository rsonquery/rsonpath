//! Automaton representations of a JSONPath query.

mod minimizer;
mod nfa;
mod small_set;

use super::{error::CompilerError, JsonPathQuery, Label};
use crate::debug;
use nfa::NondeterministicAutomaton;
use smallvec::SmallVec;
use std::{fmt::Display, ops::Index};

/// State of an [`Automaton`]. Thin wrapper over a state's identifier.
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Debug)]
pub struct State(u8);

impl Display for State {
    #[inline]
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "DFA({})", self.0)
    }
}

impl From<u8> for State {
    #[inline(always)]
    fn from(i: u8) -> Self {
        Self(i)
    }
}

/// A minimal, deterministic automaton representing a JSONPath query.
#[derive(Debug, PartialEq, Eq)]
pub struct Automaton<'q> {
    states: Vec<TransitionTable<'q>>,
}

/// A single transition of an [`Automaton`].
#[derive(Debug, PartialEq, Eq)]
pub enum Transition<'q> {
    /// A labelled transition: go to state `.1` if label `.0` is matched.
    Labelled(&'q Label, State),
    /// A wildcard transition: go to state `.0` unconditionally.
    Wildcard(State),
}

impl<'q> Transition<'q> {
    /// Get the label of the transition, if applicable.
    /// Returns `None` for wildcard transitions.
    #[must_use]
    #[inline(always)]
    pub fn label(&self) -> Option<&'q Label> {
        match self {
            Transition::Labelled(label, _) => Some(label),
            Transition::Wildcard(_) => None,
        }
    }

    /// Get the target state of the transition.
    #[must_use]
    #[inline(always)]
    pub fn target(&self) -> State {
        match self {
            Transition::Labelled(_, state) | Transition::Wildcard(state) => *state,
        }
    }
}

/// A transition table of a single [`State`] of an [`Automaton`].
///
/// Contains transitions triggered by matching labels, and a fallback transition
/// triggered when none of the label transitions match.
#[derive(Debug)]
pub struct TransitionTable<'q> {
    transitions: SmallVec<[Transition<'q>; 2]>,
    fallback_state: State,
}

impl<'q> PartialEq for TransitionTable<'q> {
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

impl<'q> Eq for TransitionTable<'q> {}

impl<'q> Index<State> for Automaton<'q> {
    type Output = TransitionTable<'q>;

    #[inline(always)]
    fn index(&self, index: State) -> &Self::Output {
        &self.states[index.0 as usize]
    }
}

impl<'q> Automaton<'q> {
    /// Convert a [`JsonPathQuery`] into a minimal deterministic automaton.
    ///
    /// # Errors
    /// [`CompilerError::NotSupported`] raised if the query contains elements
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

    /// Returns the accepting state of the automaton.
    ///
    /// Query execution should treat transitioning into this state
    /// as a match.
    #[must_use]
    #[inline(always)]
    pub fn accepting_state(&self) -> State {
        State((self.states.len() - 1) as u8)
    }

    /// Returns whether the given state is accepting.
    ///
    /// # Example
    /// ```rust
    /// # use rsonpath_lib::query::*;
    /// # use rsonpath_lib::query::automaton::*;
    /// let query = JsonPathQuery::parse("$.a").unwrap();
    /// let automaton = Automaton::new(&query).unwrap();
    ///
    /// assert!(automaton.is_accepting(automaton.accepting_state()));
    /// ```
    #[must_use]
    #[inline(always)]
    pub fn is_accepting(&self, state: State) -> bool {
        state == self.accepting_state()
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
        state == self.rejecting_state()
    }

    fn minimize(nfa: NondeterministicAutomaton<'q>) -> Result<Self, CompilerError> {
        minimizer::minimize(nfa)
    }
}

impl<'q> TransitionTable<'q> {
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
    pub fn transitions(&self) -> &SmallVec<[Transition<'q>; 2]> {
        &self.transitions
    }
}

impl<'q> Display for Automaton<'q> {
    #[inline]
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        writeln!(f, "digraph {{")?;
        for (i, state) in self.states.iter().enumerate() {
            for transition in state.transitions.iter() {
                match transition {
                    Transition::Labelled(label, state) => {
                        writeln!(f, "  {i} -> {} [label=\"{}\"]", state.0, label.display(),)?
                    }
                    Transition::Wildcard(state) => {
                        writeln!(f, "  {i} -> {} [label=\"*\"]", state.0,)?
                    }
                }
            }
            writeln!(f, "  {i} -> {} [label=\"*\"]", state.fallback_state.0)?;
        }
        write!(f, "}}")?;
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use nfa::NfaState;
    use smallvec::smallvec;

    #[test]
    fn child_and_descendant_test() {
        // Query = $.x..a.b.a.b.c..d
        let label_a = Label::new("a");
        let label_b = Label::new("b");
        let label_c = Label::new("c");
        let label_d = Label::new("d");
        let label_x = Label::new("x");

        let nfa = NondeterministicAutomaton {
            ordered_states: vec![
                NfaState::Direct(&label_x),
                NfaState::Recursive(&label_a),
                NfaState::Direct(&label_b),
                NfaState::Direct(&label_a),
                NfaState::Direct(&label_b),
                NfaState::Direct(&label_c),
                NfaState::Recursive(&label_d),
                NfaState::Accepting,
            ],
        };

        let result = Automaton::minimize(nfa).unwrap();
        let expected = Automaton {
            states: vec![
                TransitionTable {
                    transitions: smallvec![],
                    fallback_state: State(0),
                },
                TransitionTable {
                    transitions: smallvec![Transition::Labelled(&label_x, State(2))],
                    fallback_state: State(0),
                },
                TransitionTable {
                    transitions: smallvec![Transition::Labelled(&label_a, State(3))],
                    fallback_state: State(2),
                },
                TransitionTable {
                    transitions: smallvec![
                        Transition::Labelled(&label_a, State(3)),
                        Transition::Labelled(&label_b, State(4))
                    ],
                    fallback_state: State(2),
                },
                TransitionTable {
                    transitions: smallvec![Transition::Labelled(&label_a, State(5))],
                    fallback_state: State(2),
                },
                TransitionTable {
                    transitions: smallvec![
                        Transition::Labelled(&label_a, State(3)),
                        Transition::Labelled(&label_b, State(6))
                    ],
                    fallback_state: State(2),
                },
                TransitionTable {
                    transitions: smallvec![
                        Transition::Labelled(&label_a, State(5)),
                        Transition::Labelled(&label_c, State(7))
                    ],
                    fallback_state: State(2),
                },
                TransitionTable {
                    transitions: smallvec![Transition::Labelled(&label_d, State(8))],
                    fallback_state: State(7),
                },
                TransitionTable {
                    transitions: smallvec![Transition::Labelled(&label_d, State(8))],
                    fallback_state: State(7),
                },
            ],
        };

        assert_eq!(result, expected);
    }
}
