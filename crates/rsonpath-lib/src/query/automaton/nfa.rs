//! Definition of a nondeterministic automaton that can be directly
//! obtained from a JsonPath query. This is then turned into
//! a DFA with the minimizer.
use super::TransitionLabel;
use crate::query::{error::CompilerError, JsonPathQuery, JsonPathQueryNode, JsonPathQueryNodeType};
use std::{fmt::Display, ops::Index};

/// An NFA representing a query. It is always a directed path
/// from an initial state to the unique accepting state at the end,
/// where transitions are either self-loops or go forward to the immediate
/// successor in the path.
#[derive(Debug, PartialEq, Eq)]
pub(super) struct NondeterministicAutomaton<'q> {
    pub(super) ordered_states: Vec<NfaState<'q>>,
}

/// Types of states allowed in an NFA directly mapped from a [`JsonPathQuery`].
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub(super) enum NfaState<'q> {
    /// A state with a single forward [`Transition`] only.
    Direct(Transition<'q>),
    /// A state with a forward [`Transition`] and a wildcard self-loop.
    Recursive(Transition<'q>),
    /// The final state in the NFA with no outgoing transitions.
    Accepting,
}
use NfaState::*;

/// A transition in the NFA mapped from a [`JsonPathQuery`] selector.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub(super) enum Transition<'q> {
    /// A transition matching a specific label.
    Labelled(TransitionLabel<'q>),
    /// A transition matching anything.
    Wildcard,
}

/// State of an [`NondeterministicAutomaton`]. Thin wrapper over a state's
/// identifier to distinguish NFA states from DFA states ([`State`](`super::state::State`)).
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Debug)]
pub(super) struct NfaStateId(pub(super) u8);

impl NfaStateId {
    /// Return the next state in the query NFA ordering.
    ///
    /// # Errors
    /// Returns a [`CompilerError::QueryTooComplex`] if the internal limit
    /// on the state number is exceeded.
    pub(super) fn next(&self) -> Result<Self, CompilerError> {
        self.0
            .checked_add(1)
            .ok_or(CompilerError::QueryTooComplex(None))
            .map(Self)
    }
}

impl<'q> NondeterministicAutomaton<'q> {
    /// Translate a [`JsonPathQuery`] into an NFA.
    ///
    /// # Errors
    /// Returns a [`CompilerError::QueryTooComplex`] if the internal limit
    /// on the state number is exceeded.
    pub(super) fn new(query: &'q JsonPathQuery) -> Result<Self, CompilerError> {
        debug_assert!(query.root().is_root());

        let states_result: Result<Vec<NfaState>, CompilerError> = query
            .root()
            .iter()
            .filter_map(|node| match node {
                JsonPathQueryNode::Root(_) => None,
                JsonPathQueryNode::Descendant(name, _) => Some(Ok(Recursive(Transition::Labelled(name.into())))),
                JsonPathQueryNode::Child(name, _) => Some(Ok(Direct(Transition::Labelled(name.into())))),
                JsonPathQueryNode::AnyChild(_) => Some(Ok(Direct(Transition::Wildcard))),
                JsonPathQueryNode::AnyDescendant(_) => Some(Ok(Recursive(Transition::Wildcard))),
                JsonPathQueryNode::ArrayIndexChild(index, _) => Some(Ok(Direct(Transition::Labelled((*index).into())))),
                JsonPathQueryNode::ArrayIndexDescendant(index, _) => {
                    Some(Ok(Recursive(Transition::Labelled((*index).into()))))
                }
            })
            .collect();
        let mut states = states_result?;

        states.push(Accepting);

        let accepting_state: Result<u8, _> = (states.len() - 1).try_into();
        if let Err(err) = accepting_state {
            Err(CompilerError::QueryTooComplex(Some(err)))
        } else {
            Ok(NondeterministicAutomaton { ordered_states: states })
        }
    }

    pub(super) fn accepting_state(&self) -> NfaStateId {
        // CAST: safe because of the check in `new`.
        NfaStateId((self.ordered_states.len() - 1) as u8)
    }
}

impl<'q> Index<NfaStateId> for NondeterministicAutomaton<'q> {
    type Output = NfaState<'q>;

    fn index(&self, index: NfaStateId) -> &Self::Output {
        &self.ordered_states[index.0 as usize]
    }
}

impl Display for NfaStateId {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "NFA({})", self.0)
    }
}

impl<'q> Display for NondeterministicAutomaton<'q> {
    // This is the format for https://paperman.name/semigroup/
    // for easy debugging of minimization.
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let all_labels: Vec<_> = self
            .ordered_states
            .iter()
            .filter_map(|s| match s {
                Direct(Transition::Labelled(label)) | Recursive(Transition::Labelled(label)) => Some(*label),
                _ => None,
            })
            .collect();

        for (i, state) in self.ordered_states.iter().enumerate() {
            match state {
                Direct(Transition::Labelled(label)) => {
                    writeln!(f, "s{i}.{} -> s{};", label, i + 1)?;
                }
                Direct(Transition::Wildcard) => {
                    for label in &all_labels {
                        writeln!(f, "s{i}.{} -> s{};", label, i + 1)?;
                    }
                    writeln!(f, "s{i}.X -> s{};", i + 1)?;
                }
                Recursive(Transition::Labelled(label)) => {
                    writeln!(f, "s{i}.{} -> s{i}, s{};", label, i + 1)?;
                    for label in all_labels.iter().filter(|&l| l != label) {
                        writeln!(f, "s{i}.{} -> s{i};", label)?;
                    }
                    writeln!(f, "s{i}.X -> s{i};")?;
                }
                Recursive(Transition::Wildcard) => {
                    for label in &all_labels {
                        writeln!(f, "s{i}.{} -> s{i}, s{};", label, i + 1)?;
                    }
                    writeln!(f, "s{i}.X -> s{i}, s{};", i + 1)?;
                }
                Accepting => (),
            }
        }
        Ok(())
    }
}
