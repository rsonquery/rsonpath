use std::{
    collections::{BTreeSet, HashMap, HashSet},
    fmt::Display,
};

use super::{JsonPathQuery, JsonPathQueryNode, JsonPathQueryNodeType, Label};
use crate::debug;
use smallvec::{smallvec, SmallVec};

pub struct Automaton<'q> {
    states: Vec<TransitionTable<'q>>,
}

pub struct TransitionTable<'q> {
    transitions: Vec<(&'q Label, u8)>,
    fallback_state: u8,
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

impl<'q> Automaton<'q> {
    pub fn new(query: &'q JsonPathQuery) -> Self {
        let nfa = NondeterministicAutomaton::new(query);
        debug!("NFA: {}", nfa);
        let dfa = Automaton::minimize(nfa);
        debug!("DFA:\n {}", dfa);
        dfa
    }

    pub fn states(&self) -> &Vec<TransitionTable<'q>> {
        &self.states
    }

    fn minimize(nfa: NondeterministicAutomaton<'q>) -> Self {
        let reject_state = nfa.ordered_states.len() as u8;
        let mut current_superstate: BTreeSet<u8> = [0].into();
        let mut superstates = HashMap::new();
        let mut tables = vec![];
        let mut recursive = reject_state;
        superstates.insert(current_superstate.clone(), 0);

        for (i, &state) in nfa.ordered_states.iter().enumerate() {
            let i = i as u8;
            debug_assert!(current_superstate.contains(&i));
            debug!("In superstate {:?}", current_superstate);
            match state {
                Recursive(label) => {
                    debug!("Recursive state {i}");
                    let table = TransitionTable {
                        transitions: [(label, i + 1)].into(),
                        fallback_state: i,
                    };
                    tables.push(table);
                    recursive = i;
                    current_superstate = [i, i + 1].into();
                    superstates.insert(current_superstate.clone(), i + 1);
                }
                _ => {
                    let mut transitions: HashMap<&Label, BTreeSet<u8>> = HashMap::new();

                    for &substate in current_superstate.iter() {
                        debug!("Expanding state {substate}");
                        match nfa.ordered_states[substate as usize] {
                            Recursive(label) | Direct(label) => {
                                if let Some(vec) = transitions.get_mut(label) {
                                    debug!("Hit");
                                    vec.insert(substate + 1);
                                } else if recursive != reject_state {
                                    transitions.insert(label, [recursive, substate + 1].into());
                                } else {
                                    transitions.insert(label, [substate + 1].into());
                                }
                                debug!(
                                    "Updated transition via {}, now to {:?}",
                                    std::str::from_utf8(label).unwrap_or("[invalid utf8]"),
                                    transitions[label]
                                );
                            }
                            _ => (),
                        }
                    }

                    debug!("Transitions: {:?}", transitions);

                    current_superstate = if let Direct(label) = state {
                        transitions[label].clone()
                    } else {
                        BTreeSet::default()
                    };
                    superstates.insert(current_superstate.clone(), i + 1);
                    let translated_transitions =
                        transitions.into_iter().map(|x| (x.0, superstates[&x.1]));
                    let table = TransitionTable {
                        transitions: translated_transitions.collect(),
                        fallback_state: recursive,
                    };
                    tables.push(table);
                }
            }
        }

        tables.push(TransitionTable {
            transitions: vec![],
            fallback_state: reject_state,
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
                    std::str::from_utf8(transition.0).unwrap_or("[invalid utf8]"),
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
                    write!(
                        f,
                        "d{dir} --{}-> ",
                        std::str::from_utf8(label).unwrap_or("[invalid utf8]")
                    )?;
                    dir += 1;
                }
                Recursive(label) => {
                    write!(
                        f,
                        "r{rec} --{}-> ",
                        std::str::from_utf8(label).unwrap_or("[invalid utf8]")
                    )?;
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
    pub fn fallback_state(&self) -> u8 {
        self.fallback_state
    }

    pub fn transitions(&self) -> &Vec<(&'q Label, u8)> {
        &self.transitions
    }
}
