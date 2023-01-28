use crate::query::{
    error::CompilerError, JsonPathQuery, JsonPathQueryNode, JsonPathQueryNodeType, Label,
};
use std::{fmt::Display, ops::Index};

#[derive(Debug, PartialEq, Eq)]
pub(crate) struct NondeterministicAutomaton<'q> {
    pub(crate) ordered_states: Vec<NfaState<'q>>,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub(crate) enum NfaState<'q> {
    Direct(Transition<'q>),
    Recursive(Transition<'q>),
    Accepting,
}
use NfaState::*;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub(crate) enum Transition<'q> {
    Labelled(&'q Label),
    Wildcard,
}

impl<'q> Display for Transition<'q> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Transition::Labelled(label) => write!(f, "{}", label.display()),
            Transition::Wildcard => write!(f, "*"),
        }
    }
}

/// State of an [`NondeterministicAutomaton`]. Thin wrapper over a state's
/// identifier to distinguish NFA states from DFA states ([`State`]).
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Debug)]
pub(crate) struct NfaStateId(pub(crate) u8);

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
    pub(crate) fn new(query: &'q JsonPathQuery) -> Result<Self, CompilerError> {
        debug_assert!(query.root().is_root());

        let states_result: Result<Vec<NfaState>, CompilerError> = query
            .root()
            .iter()
            .filter_map(|node| match node {
                JsonPathQueryNode::Root(_) => None,
                JsonPathQueryNode::Descendant(label, _) => {
                    Some(Ok(Recursive(Transition::Labelled(label))))
                }
                JsonPathQueryNode::Child(label, _) => Some(Ok(Direct(Transition::Labelled(label)))),
                JsonPathQueryNode::AnyChild(_) => Some(Ok(Direct(Transition::Wildcard))),
            })
            .collect();
        let mut states = states_result?;

        states.push(Accepting);

        let accepting_state: Result<u8, _> = (states.len() - 1).try_into();
        if let Err(err) = accepting_state {
            Err(CompilerError::QueryTooComplex(err))
        } else {
            Ok(NondeterministicAutomaton {
                ordered_states: states,
            })
        }
    }

    pub(crate) fn accepting_state(&self) -> NfaStateId {
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

impl<'q> Display for NondeterministicAutomaton<'q> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        // This is the format for https://paperman.name/semigroup/
        // for easy debugging of minimisation.
        let all_labels: Vec<_> =
            self.ordered_states
                .iter()
                .filter_map(|s| match s {
                    Direct(Transition::Labelled(label))
                    | Recursive(Transition::Labelled(label)) => Some(*label),
                    _ => None,
                })
                .collect();

        for (i, state) in self.ordered_states.iter().enumerate() {
            match state {
                Direct(Transition::Labelled(label)) => {
                    writeln!(f, "s{i}.{} -> s{};", label.display(), i + 1)?;
                }
                Direct(Transition::Wildcard) => {
                    for label in &all_labels {
                        writeln!(f, "s{i}.{} -> s{};", label.display(), i + 1)?;
                    }
                    writeln!(f, "s{i}.X -> s{};", i + 1)?;
                }
                Recursive(Transition::Labelled(label)) => {
                    writeln!(f, "s{i}.{} -> s{i}, s{};", label.display(), i + 1)?;
                    for label in all_labels.iter().filter(|&l| l != label) {
                        writeln!(f, "s{i}.{} -> s{i};", label.display())?;
                    }
                    writeln!(f, "s{i}.X -> s{i};")?;
                }
                Recursive(Transition::Wildcard) => {
                    for label in &all_labels {
                        writeln!(f, "s{i}.{} -> s{i}, s{};", label.display(), i + 1)?;
                    }
                    writeln!(f, "s{i}.X -> s{i}, s{};", i + 1)?;
                }
                Accepting => (),
            }
        }
        Ok(())
    }
}
