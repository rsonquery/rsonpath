//! Automaton representations of a JSONPath query.

use std::{fmt::Display, ops::Index};

use super::{JsonPathQuery, JsonPathQueryNode, JsonPathQueryNodeType, Label};
use crate::debug;
use smallvec::SmallVec;

mod minimizer;
mod superstate;

#[derive(Debug, PartialEq, Eq)]
pub(crate) struct NondeterministicAutomaton<'q> {
    ordered_states: Vec<NfaState<'q>>,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub(crate) enum NfaState<'q> {
    Direct(&'q Label),
    Recursive(&'q Label),
    Accepting,
}
use NfaState::*;

/// State of an [`NondeterministicAutomaton`]. Thin wrapper over a state's
/// identifier to distinguish NFA states from DFA states ([`State`]).
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Debug)]
pub(crate) struct NfaStateId(u8);

impl NfaStateId {
    pub(crate) fn next(&self) -> Self {
        Self(self.0 + 1)
    }
}

impl Display for NfaStateId {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "NFA({})", self.0)
    }
}

impl From<u8> for NfaStateId {
    fn from(i: u8) -> Self {
        Self(i)
    }
}

impl<'q> NondeterministicAutomaton<'q> {
    fn new(query: &'q JsonPathQuery) -> Self {
        debug_assert!(query.root().is_root());

        let mut states: Vec<NfaState> = query
            .root()
            .iter()
            .filter_map(|node| match node {
                JsonPathQueryNode::Root(_) => None,
                JsonPathQueryNode::Descendant(label, _) => Some(Recursive(label)),
                JsonPathQueryNode::Child(label, _) => Some(Direct(label)),
            })
            .collect();

        states.push(Accepting);

        NondeterministicAutomaton {
            ordered_states: states,
        }
    }
}

impl<'q> Index<NfaStateId> for NondeterministicAutomaton<'q> {
    type Output = NfaState<'q>;

    fn index(&self, index: NfaStateId) -> &Self::Output {
        &self.ordered_states[index.0 as usize]
    }
}

/// State of an [`Automaton`]. Thin wrapper over a state's identifier.
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Debug)]
pub struct State(u8);

impl Display for State {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "DFA({})", self.0)
    }
}

impl From<u8> for State {
    fn from(i: u8) -> Self {
        Self(i)
    }
}

/// A minimal, deterministic automaton representing a JSONPath query.
#[derive(Debug, PartialEq, Eq)]
pub struct Automaton<'q> {
    states: Vec<TransitionTable<'q>>,
}

/// A transition table of a single [`State`] of an [`Automaton`].
///
/// Contains transitions triggered by matching labels, and a fallback transition
/// triggered when none of the label transitions match.
#[derive(Debug)]
pub struct TransitionTable<'q> {
    transitions: SmallVec<[(&'q Label, State); 2]>,
    fallback_state: State,
}

impl<'q> PartialEq for TransitionTable<'q> {
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

    fn index(&self, index: State) -> &Self::Output {
        &self.states[index.0 as usize]
    }
}

impl<'q> Automaton<'q> {
    /// Convert a [`JsonPathQuery`] into a minimal deterministic automaton.
    #[must_use]
    pub fn new(query: &'q JsonPathQuery) -> Self {
        let nfa = NondeterministicAutomaton::new(query);
        debug!("NFA: {}", nfa);
        let dfa = Automaton::minimize(nfa);
        debug!("DFA:\n {}", dfa);
        dfa
    }

    /// Returns whether this automaton represents an empty JSONPath query ('$').
    ///
    /// # Examples
    /// ```rust
    /// # use rsonpath::query::*;
    /// # use rsonpath::query::automaton::*;
    /// let query = JsonPathQuery::parse("$").unwrap();
    /// let automaton = Automaton::new(&query);
    ///
    /// assert!(automaton.is_empty_query());
    /// ```
    ///
    /// ```rust
    /// # use rsonpath::query::*;
    /// # use rsonpath::query::automaton::*;
    /// let query = JsonPathQuery::parse("$.a").unwrap();
    /// let automaton = Automaton::new(&query);
    ///
    /// assert!(!automaton.is_empty_query());
    /// ```
    #[must_use]
    pub fn is_empty_query(&self) -> bool {
        self.states.len() == 2
    }

    /// Returns the rejecting state of the automaton.
    ///
    /// The state is defined as the unique state from which there
    /// exists no accepting run. If the query automaton reaches this state,
    /// the current subtree is guaranteed to have no matches.
    #[must_use]
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
    pub fn accepting_state(&self) -> State {
        State((self.states.len() - 1) as u8)
    }

    /// Returns whether the given state is accepting.
    ///
    /// # Example
    /// ```rust
    /// # use rsonpath::query::*;
    /// # use rsonpath::query::automaton::*;
    /// let query = JsonPathQuery::parse("$.a").unwrap();
    /// let automaton = Automaton::new(&query);
    ///
    /// assert!(automaton.is_accepting(automaton.accepting_state()));
    /// ```
    #[must_use]
    pub fn is_accepting(&self, state: State) -> bool {
        state == self.accepting_state()
    }

    /// Returns whether the given state is rejecting, i.e.
    /// there exist no accepting runs from it.
    ///
    /// # Example
    /// ```rust
    /// # use rsonpath::query::*;
    /// # use rsonpath::query::automaton::*;
    /// let query = JsonPathQuery::parse("$.a").unwrap();
    /// let automaton = Automaton::new(&query);
    ///
    /// assert!(automaton.is_rejecting(automaton.rejecting_state()));
    /// ```
    #[must_use]
    pub fn is_rejecting(&self, state: State) -> bool {
        state == self.rejecting_state()
    }

    fn minimize(nfa: NondeterministicAutomaton<'q>) -> Self {
        minimizer::minimize(nfa)
    }
}

impl<'q> TransitionTable<'q> {
    /// Returns the state to which a fallback transition leads.
    ///
    /// A fallback transition is the catch-all transition triggered
    /// if none of the transitions were triggered.
    #[must_use]
    pub fn fallback_state(&self) -> State {
        self.fallback_state
    }

    /// Returns the collection of labelled transitions from this state.
    ///
    /// A transition is triggered if the [`Label`] is matched and leads
    /// to the contained [`State`].
    #[must_use]
    pub fn transitions(&self) -> &SmallVec<[(&'q Label, State); 2]> {
        &self.transitions
    }
}

impl<'q> Display for Automaton<'q> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        writeln!(f, "digraph {{")?;
        for (i, state) in self.states.iter().enumerate() {
            for transition in state.transitions.iter() {
                writeln!(
                    f,
                    "  {i} -> {} [label=\"{}\"]",
                    transition.1 .0,
                    transition.0.display(),
                )?;
            }
            writeln!(f, "  {i} -> {} [label=\"*\"]", state.fallback_state.0)?;
        }
        write!(f, "}}")?;
        Ok(())
    }
}

impl<'q> Display for NondeterministicAutomaton<'q> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut dir = 1;
        let mut rec = 1;
        for state in &self.ordered_states {
            match state {
                Direct(label) => {
                    write!(f, "d{dir} --{}-> ", label.display())?;
                    dir += 1;
                }
                Recursive(label) => {
                    write!(f, "r{rec} --{}-> ", label.display())?;
                    rec += 1;
                }
                Accepting => {
                    write!(f, "acc")?;
                }
            }
        }
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
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

        let result = Automaton::minimize(nfa);
        let expected = Automaton {
            states: vec![
                TransitionTable {
                    transitions: smallvec![],
                    fallback_state: State(0),
                },
                TransitionTable {
                    transitions: smallvec![(&label_x, State(2))],
                    fallback_state: State(0),
                },
                TransitionTable {
                    transitions: smallvec![(&label_a, State(3))],
                    fallback_state: State(2),
                },
                TransitionTable {
                    transitions: smallvec![(&label_a, State(3)), (&label_b, State(4))],
                    fallback_state: State(2),
                },
                TransitionTable {
                    transitions: smallvec![(&label_a, State(5))],
                    fallback_state: State(2),
                },
                TransitionTable {
                    transitions: smallvec![(&label_a, State(3)), (&label_b, State(6))],
                    fallback_state: State(2),
                },
                TransitionTable {
                    transitions: smallvec![(&label_a, State(5)), (&label_c, State(7))],
                    fallback_state: State(2),
                },
                TransitionTable {
                    transitions: smallvec![(&label_d, State(8))],
                    fallback_state: State(7),
                },
                TransitionTable {
                    transitions: smallvec![(&label_d, State(8))],
                    fallback_state: State(7),
                },
            ],
        };

        assert_eq!(result, expected);
    }
}
