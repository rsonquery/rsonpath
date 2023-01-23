use crate::error::UnsupportedFeatureError;
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
    Direct(&'q Label),
    Recursive(&'q Label),
    Accepting,
}
use NfaState::*;

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
                JsonPathQueryNode::Descendant(label, _) => Some(Ok(Recursive(label))),
                JsonPathQueryNode::Child(label, _) => Some(Ok(Direct(label))),
                JsonPathQueryNode::AnyChild(_) => Some(Err(
                    UnsupportedFeatureError::wildcard_child_selector().into(),
                )),
            })
            .collect();
        let mut states = states_result?;

        states.push(Accepting);

        Ok(NondeterministicAutomaton {
            ordered_states: states,
        })
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
