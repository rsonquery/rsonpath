//! Automaton representations of a JSONPath query.

use std::{fmt::Display, ops::Index};

use super::{JsonPathQuery, JsonPathQueryNode, JsonPathQueryNodeType, Label};
use crate::debug;
use smallvec::{smallvec, SmallVec};
use vector_map::VecMap;

/// A state of an [`Automaton`].
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Debug)]
pub struct State(u8);

impl Display for State {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "({})", self.0)
    }
}

/// A minimal, deterministic automaton representing a JSONPath query.
pub struct Automaton<'q> {
    states: Vec<TransitionTable<'q>>,
}

/// A transition table of a single [`State`] of an [`Automaton`].
///
/// Contains transitions triggered by matching labels, and a fallback transition
/// triggered when none of the label transitions match.
pub struct TransitionTable<'q> {
    transitions: SmallVec<[(&'q Label, State); 2]>,
    fallback_state: State,
}

struct NondeterministicAutomaton<'q> {
    ordered_states: Vec<NfaState<'q>>,
}

#[derive(Clone, Copy)]
enum NfaState<'q> {
    Direct(&'q Label),
    Recursive(&'q Label),
    Accepting,
}
use NfaState::*;

impl<'q> Index<State> for Automaton<'q> {
    type Output = TransitionTable<'q>;

    fn index(&self, index: State) -> &Self::Output {
        &self.states[index.0 as usize]
    }
}

impl<'q> Automaton<'q> {
    /// Convert a [`JsonPathQuery`] into a minimal deterministic automaton.
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
    pub fn is_empty_query(&self) -> bool {
        self.states.len() == 2
    }

    /// Returns the initial state of the automaton.
    ///
    /// Query execution should start from this state.
    pub fn initial_state(&self) -> State {
        State(0)
    }

    /// Returns the accepting state of the automaton.
    ///
    /// Query execution should treat transitioning into this state
    /// as a match.
    pub fn accepting_state(&self) -> State {
        State((self.states.len() - 2) as u8)
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
    pub fn is_accepting(&self, state: State) -> bool {
        state == self.accepting_state()
    }

    fn minimize(nfa: NondeterministicAutomaton<'q>) -> Self {
        let reject_state = nfa.ordered_states.len() as u8;
        let mut current_superstate: StateSet = [0].into();
        let mut superstates = VecMap::new();
        let mut tables = vec![];
        let mut recursive = reject_state;
        superstates.insert(current_superstate, 0);

        for (i, &state) in nfa.ordered_states.iter().enumerate() {
            let i = i as u8;
            debug_assert!(current_superstate.contains(i));
            debug!("In superstate {:?}", current_superstate);
            match state {
                Recursive(label) => {
                    debug!("Recursive state {i}");
                    let table = TransitionTable {
                        transitions: smallvec![(label, State(i + 1))],
                        fallback_state: State(i),
                    };
                    tables.push(table);
                    recursive = i;
                    current_superstate = [i, i + 1].into();
                    superstates.insert(current_superstate, i + 1);
                }
                _ => {
                    let mut transitions: VecMap<&Label, StateSet> = VecMap::new();

                    for substate in current_superstate.iter() {
                        debug!("Expanding state {substate}");
                        match nfa.ordered_states[substate as usize] {
                            Recursive(label) | Direct(label) => {
                                if let Some(set) = transitions.get_mut(&label) {
                                    debug!("Hit");
                                    set.insert(substate + 1);
                                } else if recursive != reject_state {
                                    transitions.insert(label, [recursive, substate + 1].into());
                                } else {
                                    transitions.insert(label, [substate + 1].into());
                                }
                                debug!(
                                    "Updated transition via {}, now to {:?}",
                                    label.display(),
                                    transitions[&label]
                                );
                            }
                            _ => (),
                        }
                    }

                    debug!("Transitions: {:?}", transitions);

                    current_superstate = if let Direct(label) = state {
                        transitions[&label]
                    } else {
                        StateSet::default()
                    };
                    superstates.insert(current_superstate, i + 1);
                    let translated_transitions = transitions
                        .into_iter()
                        .map(|x| (x.0, State(superstates[&x.1])));
                    let table = TransitionTable {
                        transitions: translated_transitions.collect(),
                        fallback_state: State(recursive),
                    };
                    tables.push(table);
                }
            }
        }

        tables.push(TransitionTable {
            transitions: smallvec![],
            fallback_state: State(reject_state),
        });

        Automaton { states: tables }
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
                    transition.1,
                    transition.0.display(),
                )?;
            }
            writeln!(f, "  {i} -> {} [label=\"*\"]", state.fallback_state)?;
        }
        write!(f, "}}")?;
        Ok(())
    }
}

impl<'q> NondeterministicAutomaton<'q> {
    fn new(query: &'q JsonPathQuery) -> Self {
        debug_assert!(query.root().is_root());
        let mut node_opt = query.root().child();
        let mut ordered_states = vec![];

        while let Some(node) = node_opt {
            match node {
                JsonPathQueryNode::Descendant(label, next_node) => {
                    ordered_states.push(Recursive(label));
                    node_opt = next_node.as_deref();
                }
                JsonPathQueryNode::Child(label, next_node) => {
                    ordered_states.push(Direct(label));
                    node_opt = next_node.as_deref();
                }
                _ => panic! {"Unexpected type of node, expected Descendant or Child."},
            }
        }
        ordered_states.push(Accepting);

        NondeterministicAutomaton { ordered_states }
    }
}

impl<'q> Display for NondeterministicAutomaton<'q> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut dir = 1;
        let mut rec = 1;
        for state in self.ordered_states.iter() {
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

impl<'q> TransitionTable<'q> {
    /// Returns the state to which a fallback transition leads.
    ///
    /// A fallback transition is the catch-all transition triggered
    /// if none of the [`TransitionTable::transitions`] were triggered.
    pub fn fallback_state(&self) -> State {
        self.fallback_state
    }

    /// Returns the collection of labelled transitions from this state.
    ///
    /// A transition is triggered if the [`Label`] is matched and leads
    /// to the contained [`State`].
    pub fn transitions(&self) -> &SmallVec<[(&'q Label, State); 2]> {
        &self.transitions
    }
}

#[derive(Default, Clone, Copy, PartialEq, Eq, Debug)]
struct StateSet {
    bitmask: u64,
}

impl StateSet {
    fn insert(&mut self, elem: u8) {
        self.bitmask |= 1 << elem;
    }

    fn contains(&self, elem: u8) -> bool {
        (self.bitmask & (1 << elem)) != 0
    }

    fn iter(&self) -> StateSetIter {
        StateSetIter {
            bitmask: self.bitmask,
        }
    }
}

impl<const N: usize> From<[u8; N]> for StateSet {
    fn from(arr: [u8; N]) -> Self {
        let mut result = Self::default();
        for elem in arr {
            result.insert(elem);
        }
        result
    }
}

struct StateSetIter {
    bitmask: u64,
}

impl Iterator for StateSetIter {
    type Item = u8;

    fn next(&mut self) -> Option<Self::Item> {
        let next_elem = self.bitmask.trailing_zeros();

        if next_elem == 64 {
            return None;
        }

        let elem_mask = 1 << next_elem;
        self.bitmask ^= elem_mask;

        Some(next_elem as u8)
    }
}
