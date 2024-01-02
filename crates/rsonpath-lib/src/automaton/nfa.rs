//! Definition of a nondeterministic automaton that can be directly
//! obtained from a JsonPath query. This is then turned into
//! a DFA with the minimizer.
use crate::error::UnsupportedFeatureError;

use super::{error::CompilerError, TransitionLabel};
use rsonpath_syntax::JsonPathQuery;
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
    ///
    /// Returns a [`CompilerError::NotSupported`] if the query contains a construct
    /// not currently supported by rsonpath.
    pub(super) fn new(query: &'q JsonPathQuery) -> Result<Self, CompilerError> {
        use rsonpath_syntax::{Index, Segment, Selector};

        let states_result: Result<Vec<NfaState>, CompilerError> = query
            .segments()
            .iter()
            .map(|segment| match segment {
                Segment::Child(selectors) if selectors.len() == 1 => match selectors.first() {
                    Selector::Name(name) => Ok(Direct(Transition::Labelled(name.into()))),
                    Selector::Wildcard => Ok(Direct(Transition::Wildcard)),
                    Selector::Index(Index::FromStart(index)) => Ok(Direct(Transition::Labelled((*index).into()))),
                    Selector::Index(Index::FromEnd(_)) => Err(UnsupportedFeatureError::indexing_from_end().into()),
                },
                Segment::Descendant(selectors) if selectors.len() == 1 => match selectors.first() {
                    Selector::Name(name) => Ok(Recursive(Transition::Labelled(name.into()))),
                    Selector::Wildcard => Ok(Recursive(Transition::Wildcard)),
                    Selector::Index(Index::FromStart(index)) => Ok(Recursive(Transition::Labelled((*index).into()))),
                    Selector::Index(Index::FromEnd(_)) => Err(UnsupportedFeatureError::indexing_from_end().into()),
                },
                _ => Err(UnsupportedFeatureError::multiple_selectors().into()),
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

#[cfg(test)]
mod tests {
    use super::*;
    use rsonpath_syntax::builder::JsonPathQueryBuilder;
    use rsonpath_syntax::str::JsonString;

    #[test]
    fn nfa_test() {
        let label_a = JsonString::new("a");
        let label_b = JsonString::new("b");
        let label_c = JsonString::new("c");
        let label_d = JsonString::new("d");

        let query = JsonPathQueryBuilder::new()
            .child_name(label_a.clone())
            .child_name(label_b.clone())
            .descendant_name(label_c.clone())
            .descendant_name(label_d.clone())
            .child_wildcard()
            .child_wildcard()
            .descendant_wildcard()
            .descendant_wildcard()
            .to_query();

        let expected_automaton = NondeterministicAutomaton {
            ordered_states: vec![
                NfaState::Direct(Transition::Labelled((&label_a).into())),
                NfaState::Direct(Transition::Labelled((&label_b).into())),
                NfaState::Recursive(Transition::Labelled((&label_c).into())),
                NfaState::Recursive(Transition::Labelled((&label_d).into())),
                NfaState::Direct(Transition::Wildcard),
                NfaState::Direct(Transition::Wildcard),
                NfaState::Recursive(Transition::Wildcard),
                NfaState::Recursive(Transition::Wildcard),
                NfaState::Accepting,
            ],
        };
        let automaton = NondeterministicAutomaton::new(&query).unwrap();

        assert_eq!(expected_automaton, automaton);
    }
}
