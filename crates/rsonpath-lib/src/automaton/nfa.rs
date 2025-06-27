//! Definition of a nondeterministic automaton that can be directly
//! obtained from a JsonPath query. This is then turned into
//! a DFA with the minimizer.
use crate::{automaton::SimpleSlice, error::UnsupportedFeatureError, string_pattern::StringPattern};

use super::{error::CompilerError, ArrayTransitionLabel};
use rsonpath_syntax::{str::JsonString, JsonPathQuery, Step};
use std::{collections::HashMap, fmt::Display, ops::Index, sync::Arc};

/// An NFA representing a query. It is always a directed path
/// from an initial state to the unique accepting state at the end,
/// where transitions are either self-loops or go forward to the immediate
/// successor in the path.
#[derive(Debug, PartialEq, Eq)]
pub(super) struct NondeterministicAutomaton {
    pub(super) ordered_states: Vec<NfaState>,
}

/// Types of states allowed in an NFA directly mapped from a [`JsonPathQuery`].
#[derive(Clone, Debug, PartialEq, Eq)]
pub(super) enum NfaState {
    /// A state with a single forward [`Transition`] only.
    Direct(Transition),
    /// A state with a forward [`Transition`] and a wildcard self-loop.
    Recursive(Transition),
    /// The final state in the NFA with no outgoing transitions.
    Accepting,
}
use NfaState::*;

/// A transition in the NFA mapped from a [`JsonPathQuery`] selector.
#[derive(Clone, Debug, PartialEq, Eq)]
pub(super) enum Transition {
    /// A transition matching array indices.
    Array(ArrayTransitionLabel),
    /// A transition matching a specific member.
    Member(Arc<StringPattern>),
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

impl NondeterministicAutomaton {
    /// Translate a [`JsonPathQuery`] into an NFA.
    ///
    /// # Errors
    /// Returns a [`CompilerError::QueryTooComplex`] if the internal limit
    /// on the state number is exceeded.
    ///
    /// Returns a [`CompilerError::NotSupported`] if the query contains a construct
    /// not currently supported by rsonpath.
    pub(super) fn new<'q>(query: &'q JsonPathQuery) -> Result<Self, CompilerError> {
        use rsonpath_syntax::{Index, Selector};
        use std::collections::hash_map::Entry;

        let mut string_pattern_cache: HashMap<&'q JsonString, Arc<StringPattern>> = HashMap::new();

        let states_result: Result<Vec<NfaState>, CompilerError> = query
            .segments()
            .iter()
            .map(|segment| {
                let selectors = segment.selectors();

                if selectors.len() > 1 {
                    Err(UnsupportedFeatureError::multiple_selectors().into())
                } else {
                    let transition = match selectors.first() {
                        Selector::Name(name) => {
                            let pattern = match string_pattern_cache.entry(name) {
                                Entry::Occupied(pat) => pat.get().clone(),
                                Entry::Vacant(entry) => {
                                    let pat = Arc::new(StringPattern::new(name));
                                    entry.insert(pat.clone());
                                    pat
                                }
                            };
                            Ok::<_, CompilerError>(Transition::Member(pattern))
                        }
                        Selector::Wildcard => Ok(Transition::Wildcard),
                        Selector::Index(Index::FromStart(index)) => Ok(Transition::Array((*index).into())),
                        Selector::Index(Index::FromEnd(_)) => Err(UnsupportedFeatureError::indexing_from_end().into()),
                        Selector::Slice(slice) => {
                            let start = match slice.start() {
                                Index::FromStart(idx) => Ok::<_, CompilerError>(idx),
                                Index::FromEnd(_) => Err(UnsupportedFeatureError::indexing_from_end().into()),
                            }?;
                            let end = match slice.end() {
                                Some(Index::FromStart(idx)) => Ok::<_, CompilerError>(Some(idx)),
                                Some(Index::FromEnd(_)) => Err(UnsupportedFeatureError::indexing_from_end().into()),
                                None => Ok(None),
                            }?;
                            let step = match slice.step() {
                                Step::Forward(step) => Ok::<_, CompilerError>(step),
                                Step::Backward(_) => Err(UnsupportedFeatureError::slice_with_backward_step().into()),
                            }?;
                            let simple_slice = SimpleSlice::new(start, end, step);
                            Ok(Transition::Array(simple_slice.into()))
                        }
                        Selector::Filter(_) => Err(UnsupportedFeatureError::filter_selector().into()),
                    }?;
                    if segment.is_child() {
                        Ok(Direct(transition))
                    } else {
                        Ok(Recursive(transition))
                    }
                }
            })
            .collect();
        let mut states = states_result?;

        states.push(Accepting);

        let accepting_state: Result<u8, _> = (states.len() - 1).try_into();
        if let Err(err) = accepting_state {
            Err(CompilerError::QueryTooComplex(Some(err)))
        } else {
            Ok(Self { ordered_states: states })
        }
    }

    pub(super) fn accepting_state(&self) -> NfaStateId {
        // CAST: safe because of the check in `new`.
        NfaStateId((self.ordered_states.len() - 1) as u8)
    }
}

impl Index<NfaStateId> for NondeterministicAutomaton {
    type Output = NfaState;

    fn index(&self, index: NfaStateId) -> &Self::Output {
        &self.ordered_states[index.0 as usize]
    }
}

impl Display for NfaStateId {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "NFA({})", self.0)
    }
}

impl Display for NondeterministicAutomaton {
    // This is the format for https://paperman.name/semigroup/
    // for easy debugging of minimization.
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let all_labels: Vec<_> = self
            .ordered_states
            .iter()
            .filter_map(|s| match s {
                Direct(Transition::Member(label)) | Recursive(Transition::Member(label)) => {
                    Some(stringify_label(label).to_string())
                }
                Direct(Transition::Array(label)) | Recursive(Transition::Array(label)) => Some(label.to_string()),
                _ => None,
            })
            .collect();

        for (i, state) in self.ordered_states.iter().enumerate() {
            match state {
                Direct(Transition::Array(label)) => {
                    writeln!(f, "s{i}.{} -> s{};", label, i + 1)?;
                }
                Direct(Transition::Member(label)) => {
                    writeln!(f, "s{i}.{} -> s{};", stringify_label(label), i + 1)?;
                }
                Direct(Transition::Wildcard) => {
                    for label in &all_labels {
                        writeln!(f, "s{i}.{} -> s{};", label, i + 1)?;
                    }
                    writeln!(f, "s{i}.X -> s{};", i + 1)?;
                }
                Recursive(Transition::Member(label)) => {
                    writeln!(f, "s{i}.{} -> s{i}, s{};", stringify_label(label), i + 1)?;
                    for label in all_labels.iter().filter(|&l| l != stringify_label(label)) {
                        writeln!(f, "s{i}.{label} -> s{i};")?;
                    }
                    writeln!(f, "s{i}.X -> s{i};")?;
                }
                Recursive(Transition::Array(label)) => {
                    writeln!(f, "s{i}.{} -> s{i}, s{};", label, i + 1)?;
                    for label in all_labels.iter().filter(|&l| l != &label.to_string()) {
                        writeln!(f, "s{i}.{label} -> s{i};")?;
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
        return Ok(());

        fn stringify_label(label: &StringPattern) -> &str {
            std::str::from_utf8(label.unquoted()).expect("labels must be valid utf8")
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use rsonpath_syntax::{builder::JsonPathQueryBuilder, str::JsonString};

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
                NfaState::Direct(Transition::Member(Arc::new(StringPattern::new(&label_a)))),
                NfaState::Direct(Transition::Member(Arc::new(StringPattern::new(&label_b)))),
                NfaState::Recursive(Transition::Member(Arc::new(StringPattern::new(&label_c)))),
                NfaState::Recursive(Transition::Member(Arc::new(StringPattern::new(&label_d)))),
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
