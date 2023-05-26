//! Determinization and minimization of an NFA into the final DFA used by the engines.

// NOTE: Some comments in this module are outdated, because the minimizer doesn't
// actually produce minimal automata as of now - see #91.
use super::nfa::{self, NfaState, NfaStateId};
use super::small_set::{SmallSet, SmallSet256};
use super::state::StateAttributesBuilder;
use super::{Automaton, NondeterministicAutomaton, State as DfaStateId, StateAttributes, StateTable, TransitionLabel};
use crate::debug;
use crate::query::error::CompilerError;
use smallvec::{smallvec, SmallVec};
use vector_map::VecMap;

/// Turn the [`NondeterministicAutomaton`] to an equivalent minimal* deterministic [`Automaton`].
///
/// *Not actually minimal. See #91
pub(super) fn minimize(nfa: NondeterministicAutomaton) -> Result<Automaton, CompilerError> {
    let minimizer = Minimizer {
        nfa,
        superstates: VecMap::new(),
        checkpoints: VecMap::new(),
        active_superstates: smallvec![],
        dfa_states: vec![],
        accepting: SmallSet256::default(),
    };

    minimizer.run()
}

pub(super) struct Minimizer<'q> {
    /// The NFA being minimized.
    nfa: NondeterministicAutomaton<'q>,
    /// All superstates created thus far mapping to their index in the DFA being constructed.
    superstates: VecMap<SmallSet256, DfaStateId>,
    /// Map from superstates to the furthest reachable checkpoint on a path leading to that superstate.
    checkpoints: VecMap<SmallSet256, NfaStateId>,
    /// Superstates that have not been processed and expanded yet.
    active_superstates: SmallVec<[SmallSet256; 2]>,
    /// All superstates created thus far, in order matching the `superstates` map.
    dfa_states: Vec<StateTable<'q>>,
    /// Set of activated DFA states that are accepting.
    accepting: SmallSet256,
}

#[derive(Debug)]
struct SuperstateTransitionTable<'q> {
    labelled: VecMap<TransitionLabel<'q>, SmallSet256>,
    wildcard: SmallSet256,
}

/**
 * Minimization proceeds by superset construction, made easier and ensuring minimality
 * due to *checkpoints*.
 *
 * Every state with a self-loop becomes a checkpoint. They have two crucial properties:
 *   1. Any path from the initial to the accepting state goes through each checkpoint.
 *   2. Each superstate containing
 *        a) a checkpoint and;
 *        b) some states on the path from the initial state to that checkpoint,
 *      is equivalent to a superstate without the b) states.
 * This allows on-the-fly minimization with the `normalize` function, vastly reducing
 * the number of superstates to consider.
 *
 * Identifying checkpoints is easy - these are exactly the singleton sets of Recursive
 * NFA states.
 *
 * We expand each superstate by examining all transitions originating from NFA states
 * in the superstate. The targets of those transitions are consolidated into superstates.
 * If a superstate is encountered for the first time, it becomes active and will be expanded later.
 * The algorithm continues while any states are active.
 *
 * Superstate number 0 is specifically designated as the rejecting state,
 * which is used when there is no available checkpoint to return to.
 **/
impl<'q> Minimizer<'q> {
    /// Main loop of the algorithm. Initialize rejecting and initial states
    /// and perform expansion until we run out of active states.
    fn run(mut self) -> Result<Automaton<'q>, CompilerError> {
        // Rejecting state has no outgoing transitions except for a self-loop.
        self.dfa_states.push(StateTable {
            transitions: smallvec![],
            fallback_state: Self::rejecting_state(),
            attributes: StateAttributesBuilder::new().rejecting().into(),
        });
        self.superstates.insert(SmallSet256::default(), Self::rejecting_state());

        // Initial superstate is {0}.
        let initial_superstate = [0].into();
        self.activate_if_new(initial_superstate)?;

        while let Some(superstate) = self.active_superstates.pop() {
            self.process_superstate(superstate)?;
        }

        Ok(Automaton {
            states: self.dfa_states,
        })
    }

    fn rejecting_state() -> DfaStateId {
        DfaStateId(0)
    }

    /// Every time a transition to a superstate is created, we need to check if it is
    /// discovered for the first time. If so, we need to initialize and activate it.
    fn activate_if_new(&mut self, superstate: SmallSet256) -> Result<(), CompilerError> {
        if !self.superstates.contains_key(&superstate) {
            let identifier = self
                .superstates
                .len()
                .try_into()
                .map(DfaStateId)
                .map_err(|err| CompilerError::QueryTooComplex(Some(err)))?;
            self.superstates.insert(superstate, identifier);
            self.active_superstates.push(superstate);
            self.dfa_states.push(StateTable::default());
            debug!("New superstate created: {superstate:?} {identifier}");
            if superstate.contains(self.nfa.accepting_state().0) {
                self.accepting.insert(identifier.0);
            }
        }

        Ok(())
    }

    /// Create the superstate's [`TransitionTable`] by processing all transitions
    /// of NFA states within the superstate.
    fn process_superstate(&mut self, current_superstate: SmallSet256) -> Result<(), CompilerError> {
        let current_checkpoint = self.determine_checkpoint(current_superstate);
        debug!("Expanding superstate: {current_superstate:?}, last checkpoint is {current_checkpoint:?}");

        let mut transitions = self.process_nfa_transitions(current_superstate, current_checkpoint)?;
        debug!("Raw transitions: {:?}", transitions);

        self.normalize_superstate_transitions(&mut transitions, current_checkpoint)?;
        debug!("Normalized transitions: {:?}", transitions);

        // Translate the transitions to the data model expected by TransitionTable.
        let translated_transitions: SmallVec<_> = transitions
            .labelled
            .into_iter()
            .map(|(label, state)| (label, self.superstates[&state]))
            .collect();
        debug!("Translated transitions: {translated_transitions:?}");

        // If a checkpoint was reached, its singleton superstate is this DFA state's fallback state.
        // Otherwise, we set the fallback to the rejecting state.
        let id = self.superstates[&current_superstate];
        let fallback_state = self.superstates[&transitions.wildcard];
        let attributes = self.build_attributes(id, &translated_transitions, fallback_state);
        let table = &mut self.dfa_states[id.0 as usize];
        table.transitions = translated_transitions;
        table.fallback_state = fallback_state;
        table.attributes = attributes;

        Ok(())
    }

    /// Build attributes of a DFA state after all of its transitions have been
    /// determined.
    fn build_attributes(
        &self,
        id: DfaStateId,
        transitions: &[(TransitionLabel, DfaStateId)],
        fallback: DfaStateId,
    ) -> StateAttributes {
        let mut attrs = StateAttributesBuilder::new();

        if self.accepting.contains(id.0) {
            debug!("{id} is accepting");
            attrs = attrs.accepting();
        }
        if id == Self::rejecting_state() {
            debug!("{id} is rejecting");
            attrs = attrs.rejecting();
        }
        if transitions.len() == 1 && fallback == Self::rejecting_state() {
            debug!("{id} is unitary");
            attrs = attrs.unitary();
        }
        if self.accepting.contains(fallback.0) || transitions.iter().any(|(_, s)| self.accepting.contains(s.0)) {
            debug!("{id} has transitions to accepting");
            attrs = attrs.transitions_to_accepting();
        }

        attrs.into()
    }

    /// Determine what is the furthest reachable checkpoint on the path to this
    /// superstate. This is either the superstate itself, if it is a checkpoint,
    /// or the one flowed into from a previous superstate via the `checkpoints` map.
    fn determine_checkpoint(&mut self, superstate: SmallSet256) -> Option<NfaStateId> {
        if let Some(nfa_state) = self.as_checkpoint(superstate) {
            self.checkpoints.insert(superstate, nfa_state);
            Some(nfa_state)
        } else {
            self.checkpoints.get(&superstate).copied()
        }
    }

    /// Determine whether the `superstate` is a checkpoint, and if yes
    /// return the Recursive NFA state it represents. Otherwise, return `None`.
    fn as_checkpoint(&self, superstate: SmallSet256) -> Option<NfaStateId> {
        if let Some(single_state) = superstate.singleton().map(NfaStateId) {
            if matches!(self.nfa[single_state], NfaState::Recursive(_)) {
                return Some(single_state);
            }
        }

        None
    }

    /// Create the transition table for a superstate by traversing all NFA transitions
    /// from states within it.
    fn process_nfa_transitions(
        &self,
        current_superstate: SmallSet256,
        current_checkpoint: Option<NfaStateId>,
    ) -> Result<SuperstateTransitionTable<'q>, CompilerError> {
        let mut wildcard_targets = current_superstate
            .iter()
            .map(NfaStateId)
            .filter_map(|id| match self.nfa[id] {
                NfaState::Recursive(nfa::Transition::Wildcard) | NfaState::Direct(nfa::Transition::Wildcard) => {
                    Some(id.next().map(|x| x.0))
                }
                _ => None,
            })
            .collect::<Result<SmallSet256, _>>()?;
        if let Some(checkpoint) = current_checkpoint {
            wildcard_targets.insert(checkpoint.0);
        }

        debug!("Wildcard target: {wildcard_targets:?}");

        let mut transitions = SuperstateTransitionTable {
            labelled: VecMap::new(),
            wildcard: wildcard_targets,
        };

        for nfa_state in current_superstate.iter().map(NfaStateId) {
            match self.nfa[nfa_state] {
                // Direct states simply have a single transition to the next state in the NFA.
                // Recursive transitions also have a self-loop, but that is handled by the
                // checkpoints mechanism - here we only handle the forward transition.
                NfaState::Direct(nfa::Transition::Labelled(label))
                | NfaState::Recursive(nfa::Transition::Labelled(label)) => {
                    debug!("Considering transition {nfa_state} --{}-> {}", label, nfa_state.next()?,);
                    // Add the target NFA state to the target superstate, or create a singleton
                    // set if this is the first transition via this label encountered in the loop.
                    if let Some(target) = transitions.labelled.get_mut(&label) {
                        target.insert(nfa_state.next()?.0);
                    } else {
                        let mut new_set = transitions.wildcard;
                        new_set.insert(nfa_state.next()?.0);
                        transitions.labelled.insert(label, new_set);
                    }
                }
                NfaState::Direct(nfa::Transition::Wildcard)
                | NfaState::Recursive(nfa::Transition::Wildcard)
                | NfaState::Accepting => (),
            }
        }

        Ok(transitions)
    }

    /// Use the checkpoints to perform normalization of superstates
    /// and activate them if needed.
    fn normalize_superstate_transitions(
        &mut self,
        transitions: &mut SuperstateTransitionTable,
        current_checkpoint: Option<NfaStateId>,
    ) -> Result<(), CompilerError> {
        fn normalize_one(
            this: &mut Minimizer,
            state: &mut SmallSet256,
            current_checkpoint: Option<NfaStateId>,
        ) -> Result<(), CompilerError> {
            if let Some(checkpoint) = current_checkpoint {
                state.insert(checkpoint.0);
            }

            this.normalize(state);
            this.activate_if_new(*state)?;

            if let Some(checkpoint) = current_checkpoint {
                this.checkpoints.insert(*state, checkpoint);
            }

            Ok(())
        }

        normalize_one(self, &mut transitions.wildcard, current_checkpoint)?;
        for (_, state) in transitions.labelled.iter_mut() {
            normalize_one(self, state, current_checkpoint)?;
        }

        Ok(())
    }

    /// If a superstate contains a Recursive NFA state, then all the NFA states
    /// prior to that Recursive state can be removed, equalizing many possible
    /// combinations.
    fn normalize(&self, superstate: &mut SmallSet256) {
        let furthest_checkpoint = superstate
            .iter()
            .map(NfaStateId)
            .filter(|&x| matches!(self.nfa[x], NfaState::Recursive(_)))
            .max();

        if let Some(cutoff) = furthest_checkpoint {
            superstate.remove_all_before(cutoff.0);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::super::*;
    use super::*;
    use nfa::NfaState;
    use pretty_assertions::assert_eq;
    use smallvec::smallvec;

    #[test]
    fn empty_query_test() {
        // Query = $
        let nfa = NondeterministicAutomaton {
            ordered_states: vec![NfaState::Accepting],
        };

        let result = minimize(nfa).unwrap();
        let expected = Automaton {
            states: vec![
                StateTable {
                    transitions: smallvec![],
                    fallback_state: State(0),
                    attributes: StateAttributes::REJECTING,
                },
                StateTable {
                    transitions: smallvec![],
                    fallback_state: State(0),
                    attributes: StateAttributes::ACCEPTING,
                },
            ],
        };

        assert_eq!(result, expected);
    }

    #[test]
    fn simple_wildcard_test() {
        // Query = $.*
        let nfa = NondeterministicAutomaton {
            ordered_states: vec![NfaState::Direct(nfa::Transition::Wildcard), NfaState::Accepting],
        };

        let result = minimize(nfa).unwrap();
        let expected = Automaton {
            states: vec![
                StateTable {
                    transitions: smallvec![],
                    fallback_state: State(0),
                    attributes: StateAttributes::REJECTING,
                },
                StateTable {
                    transitions: smallvec![],
                    fallback_state: State(2),
                    attributes: StateAttributes::TRANSITIONS_TO_ACCEPTING,
                },
                StateTable {
                    transitions: smallvec![],
                    fallback_state: State(0),
                    attributes: StateAttributes::ACCEPTING,
                },
            ],
        };

        assert_eq!(result, expected);
    }

    #[test]
    fn simple_nonnegative_indexed_test() {
        // Query = $[0]
        let label = TransitionLabel::ArrayIndex(0.try_into().unwrap());

        let nfa = NondeterministicAutomaton {
            ordered_states: vec![NfaState::Direct(nfa::Transition::Labelled(label)), NfaState::Accepting],
        };

        let result = minimize(nfa).unwrap();
        let expected = Automaton {
            states: vec![
                StateTable {
                    transitions: smallvec![],
                    fallback_state: State(0),
                    attributes: StateAttributes::REJECTING,
                },
                StateTable {
                    transitions: smallvec![(label, State(2))],
                    fallback_state: State(0),
                    attributes: StateAttributes::UNITARY | StateAttributes::TRANSITIONS_TO_ACCEPTING,
                },
                StateTable {
                    transitions: smallvec![],
                    fallback_state: State(0),
                    attributes: StateAttributes::ACCEPTING,
                },
            ],
        };

        assert_eq!(result, expected);
    }

    #[test]
    fn simple_descendant_wildcard_test() {
        // Query = $..*
        let nfa = NondeterministicAutomaton {
            ordered_states: vec![NfaState::Recursive(nfa::Transition::Wildcard), NfaState::Accepting],
        };

        let result = minimize(nfa).unwrap();
        let expected = Automaton {
            states: vec![
                StateTable {
                    transitions: smallvec![],
                    fallback_state: State(0),
                    attributes: StateAttributes::REJECTING,
                },
                StateTable {
                    transitions: smallvec![],
                    fallback_state: State(2),
                    attributes: StateAttributes::TRANSITIONS_TO_ACCEPTING,
                },
                StateTable {
                    transitions: smallvec![],
                    fallback_state: State(2),
                    attributes: StateAttributes::ACCEPTING | StateAttributes::TRANSITIONS_TO_ACCEPTING,
                },
            ],
        };

        assert_eq!(result, expected);
    }

    #[test]
    fn interstitial_descendant_wildcard_test() {
        // Query = $..a.b..*.a..b
        let label_a = JsonString::new("a");
        let label_a = (&label_a).into();

        let label_b = JsonString::new("b");
        let label_b = (&label_b).into();

        let nfa = NondeterministicAutomaton {
            ordered_states: vec![
                NfaState::Recursive(nfa::Transition::Labelled(label_a)),
                NfaState::Direct(nfa::Transition::Labelled(label_b)),
                NfaState::Recursive(nfa::Transition::Wildcard),
                NfaState::Direct(nfa::Transition::Labelled(label_a)),
                NfaState::Recursive(nfa::Transition::Labelled(label_b)),
                NfaState::Accepting,
            ],
        };

        let result = minimize(nfa).unwrap();
        let expected = Automaton {
            states: vec![
                StateTable {
                    transitions: smallvec![],
                    fallback_state: State(0),
                    attributes: StateAttributes::REJECTING,
                },
                StateTable {
                    transitions: smallvec![(label_a, State(2))],
                    fallback_state: State(1),
                    attributes: StateAttributes::EMPTY,
                },
                StateTable {
                    transitions: smallvec![(label_a, State(2)), (label_b, State(3))],
                    fallback_state: State(1),
                    attributes: StateAttributes::EMPTY,
                },
                StateTable {
                    transitions: smallvec![],
                    fallback_state: State(4),
                    attributes: StateAttributes::EMPTY,
                },
                StateTable {
                    transitions: smallvec![(label_a, State(5))],
                    fallback_state: State(4),
                    attributes: StateAttributes::EMPTY,
                },
                StateTable {
                    transitions: smallvec![(label_b, State(6))],
                    fallback_state: State(5),
                    attributes: StateAttributes::TRANSITIONS_TO_ACCEPTING,
                },
                StateTable {
                    transitions: smallvec![(label_b, State(6))],
                    fallback_state: State(5),
                    attributes: StateAttributes::ACCEPTING | StateAttributes::TRANSITIONS_TO_ACCEPTING,
                },
            ],
        };

        assert_eq!(result, expected);
    }

    #[test]
    fn interstitial_nondescendant_wildcard_test() {
        // Query = $..a.b.*.a..b
        let label_a = JsonString::new("a");
        let label_a = (&label_a).into();

        let label_b = JsonString::new("b");
        let label_b = (&label_b).into();

        let nfa = NondeterministicAutomaton {
            ordered_states: vec![
                NfaState::Recursive(nfa::Transition::Labelled(label_a)),
                NfaState::Direct(nfa::Transition::Labelled(label_b)),
                NfaState::Direct(nfa::Transition::Wildcard),
                NfaState::Direct(nfa::Transition::Labelled(label_a)),
                NfaState::Recursive(nfa::Transition::Labelled(label_b)),
                NfaState::Accepting,
            ],
        };

        let result = minimize(nfa).unwrap();
        let expected = Automaton {
            states: vec![
                StateTable {
                    transitions: smallvec![],
                    fallback_state: State(0),
                    attributes: StateAttributes::REJECTING,
                },
                StateTable {
                    transitions: smallvec![(label_a, State(2))],
                    fallback_state: State(1),
                    attributes: StateAttributes::EMPTY,
                },
                StateTable {
                    transitions: smallvec![(label_a, State(2)), (label_b, State(3))],
                    fallback_state: State(1),
                    attributes: StateAttributes::EMPTY,
                },
                StateTable {
                    transitions: smallvec![(label_a, State(5))],
                    fallback_state: State(4),
                    attributes: StateAttributes::EMPTY,
                },
                StateTable {
                    transitions: smallvec![(label_a, State(6))],
                    fallback_state: State(1),
                    attributes: StateAttributes::EMPTY,
                },
                StateTable {
                    transitions: smallvec![(label_a, State(6)), (label_b, State(3))],
                    fallback_state: State(1),
                    attributes: StateAttributes::EMPTY,
                },
                StateTable {
                    transitions: smallvec![(label_b, State(7))],
                    fallback_state: State(6),
                    attributes: StateAttributes::TRANSITIONS_TO_ACCEPTING,
                },
                StateTable {
                    transitions: smallvec![(label_b, State(7))],
                    fallback_state: State(6),
                    attributes: StateAttributes::ACCEPTING | StateAttributes::TRANSITIONS_TO_ACCEPTING,
                },
            ],
        };

        assert_eq!(result, expected);
    }

    #[test]
    fn simple_multi_accepting_test() {
        // Query = $..a.*
        let label = JsonString::new("a");
        let label = (&label).into();

        let nfa = NondeterministicAutomaton {
            ordered_states: vec![
                NfaState::Recursive(nfa::Transition::Labelled(label)),
                NfaState::Direct(nfa::Transition::Wildcard),
                NfaState::Accepting,
            ],
        };

        let result = minimize(nfa).unwrap();
        let expected = Automaton {
            states: vec![
                StateTable {
                    transitions: smallvec![],
                    fallback_state: State(0),
                    attributes: StateAttributes::REJECTING,
                },
                StateTable {
                    transitions: smallvec![(label, State(2)),],
                    fallback_state: State(1),
                    attributes: StateAttributes::EMPTY,
                },
                StateTable {
                    transitions: smallvec![(label, State(4))],
                    fallback_state: State(3),
                    attributes: StateAttributes::TRANSITIONS_TO_ACCEPTING,
                },
                StateTable {
                    transitions: smallvec![(label, State(2))],
                    fallback_state: State(1),
                    attributes: StateAttributes::ACCEPTING,
                },
                StateTable {
                    transitions: smallvec![(label, State(4))],
                    fallback_state: State(3),
                    attributes: StateAttributes::ACCEPTING | StateAttributes::TRANSITIONS_TO_ACCEPTING,
                },
            ],
        };

        assert_eq!(result, expected);
    }

    #[test]
    fn simple_multi_accepting_nneg_index_test() {
        // Query = $..[3]
        let label = TransitionLabel::ArrayIndex(0.try_into().unwrap());

        let nfa = NondeterministicAutomaton {
            ordered_states: vec![
                NfaState::Recursive(nfa::Transition::Labelled(label)),
                NfaState::Accepting,
            ],
        };

        let result = minimize(nfa).unwrap();
        let expected = Automaton {
            states: vec![
                StateTable {
                    transitions: smallvec![],
                    fallback_state: State(0),
                    attributes: StateAttributes::REJECTING,
                },
                StateTable {
                    transitions: smallvec![(label, State(2)),],
                    fallback_state: State(1),
                    attributes: StateAttributes::TRANSITIONS_TO_ACCEPTING,
                },
                StateTable {
                    transitions: smallvec![(label, State(2))],
                    fallback_state: State(1),
                    attributes: StateAttributes::TRANSITIONS_TO_ACCEPTING | StateAttributes::ACCEPTING,
                },
            ],
        };

        assert_eq!(result, expected);
    }

    #[test]
    fn chained_wildcard_children_test() {
        // Query = $.a.*.*.*
        let label = JsonString::new("a");
        let label = (&label).into();

        let nfa = NondeterministicAutomaton {
            ordered_states: vec![
                NfaState::Direct(nfa::Transition::Labelled(label)),
                NfaState::Direct(nfa::Transition::Wildcard),
                NfaState::Direct(nfa::Transition::Wildcard),
                NfaState::Direct(nfa::Transition::Wildcard),
                NfaState::Accepting,
            ],
        };

        let result = minimize(nfa).unwrap();
        let expected = Automaton {
            states: vec![
                StateTable {
                    transitions: smallvec![],
                    fallback_state: State(0),
                    attributes: StateAttributes::REJECTING,
                },
                StateTable {
                    transitions: smallvec![(label, State(2))],
                    fallback_state: State(0),
                    attributes: StateAttributes::UNITARY,
                },
                StateTable {
                    transitions: smallvec![],
                    fallback_state: State(3),
                    attributes: StateAttributes::EMPTY,
                },
                StateTable {
                    transitions: smallvec![],
                    fallback_state: State(4),
                    attributes: StateAttributes::EMPTY,
                },
                StateTable {
                    transitions: smallvec![],
                    fallback_state: State(5),
                    attributes: StateAttributes::TRANSITIONS_TO_ACCEPTING,
                },
                StateTable {
                    transitions: smallvec![],
                    fallback_state: State(0),
                    attributes: StateAttributes::ACCEPTING,
                },
            ],
        };

        assert_eq!(result, expected);
    }

    #[test]
    fn chained_wildcard_children_after_descendant_test() {
        // Query = $..a.*.*
        let label = JsonString::new("a");
        let label = (&label).into();

        let nfa = NondeterministicAutomaton {
            ordered_states: vec![
                NfaState::Recursive(nfa::Transition::Labelled(label)),
                NfaState::Direct(nfa::Transition::Wildcard),
                NfaState::Direct(nfa::Transition::Wildcard),
                NfaState::Accepting,
            ],
        };

        let result = minimize(nfa).unwrap();
        let expected = Automaton {
            states: vec![
                StateTable {
                    transitions: smallvec![],
                    fallback_state: State(0),
                    attributes: StateAttributes::REJECTING,
                },
                StateTable {
                    transitions: smallvec![(label, State(2))],
                    fallback_state: State(1),
                    attributes: StateAttributes::EMPTY,
                },
                StateTable {
                    transitions: smallvec![(label, State(4))],
                    fallback_state: State(3),
                    attributes: StateAttributes::EMPTY,
                },
                StateTable {
                    transitions: smallvec![(label, State(8))],
                    fallback_state: State(7),
                    attributes: StateAttributes::EMPTY | StateAttributes::TRANSITIONS_TO_ACCEPTING,
                },
                StateTable {
                    transitions: smallvec![(label, State(6))],
                    fallback_state: State(5),
                    attributes: StateAttributes::EMPTY | StateAttributes::TRANSITIONS_TO_ACCEPTING,
                },
                StateTable {
                    transitions: smallvec![(label, State(8))],
                    fallback_state: State(7),
                    attributes: StateAttributes::ACCEPTING | StateAttributes::TRANSITIONS_TO_ACCEPTING,
                },
                StateTable {
                    transitions: smallvec![(label, State(6))],
                    fallback_state: State(5),
                    attributes: StateAttributes::ACCEPTING | StateAttributes::TRANSITIONS_TO_ACCEPTING,
                },
                StateTable {
                    transitions: smallvec![(label, State(2))],
                    fallback_state: State(1),
                    attributes: StateAttributes::ACCEPTING,
                },
                StateTable {
                    transitions: smallvec![(label, State(4))],
                    fallback_state: State(3),
                    attributes: StateAttributes::ACCEPTING,
                },
            ],
        };

        assert_eq!(result, expected);
    }

    #[test]
    fn child_and_descendant_test() {
        // Query = $.x..a.b.a.b.c..d
        let label_a = JsonString::new("a");
        let label_a = (&label_a).into();

        let label_b = JsonString::new("b");
        let label_b = (&label_b).into();

        let label_c = JsonString::new("c");
        let label_c = (&label_c).into();

        let label_d = JsonString::new("d");
        let label_d = (&label_d).into();

        let label_x = JsonString::new("x");
        let label_x = (&label_x).into();

        let nfa = NondeterministicAutomaton {
            ordered_states: vec![
                NfaState::Direct(nfa::Transition::Labelled(label_x)),
                NfaState::Recursive(nfa::Transition::Labelled(label_a)),
                NfaState::Direct(nfa::Transition::Labelled(label_b)),
                NfaState::Direct(nfa::Transition::Labelled(label_a)),
                NfaState::Direct(nfa::Transition::Labelled(label_b)),
                NfaState::Direct(nfa::Transition::Labelled(label_c)),
                NfaState::Recursive(nfa::Transition::Labelled(label_d)),
                NfaState::Accepting,
            ],
        };

        let result = minimize(nfa).unwrap();
        let expected = Automaton {
            states: vec![
                StateTable {
                    transitions: smallvec![],
                    fallback_state: State(0),
                    attributes: StateAttributes::REJECTING,
                },
                StateTable {
                    transitions: smallvec![(label_x, State(2))],
                    fallback_state: State(0),
                    attributes: StateAttributes::UNITARY,
                },
                StateTable {
                    transitions: smallvec![(label_a, State(3))],
                    fallback_state: State(2),
                    attributes: StateAttributes::EMPTY,
                },
                StateTable {
                    transitions: smallvec![(label_a, State(3)), (label_b, State(4))],
                    fallback_state: State(2),
                    attributes: StateAttributes::EMPTY,
                },
                StateTable {
                    transitions: smallvec![(label_a, State(5))],
                    fallback_state: State(2),
                    attributes: StateAttributes::EMPTY,
                },
                StateTable {
                    transitions: smallvec![(label_a, State(3)), (label_b, State(6))],
                    fallback_state: State(2),
                    attributes: StateAttributes::EMPTY,
                },
                StateTable {
                    transitions: smallvec![(label_a, State(5)), (label_c, State(7))],
                    fallback_state: State(2),
                    attributes: StateAttributes::EMPTY,
                },
                StateTable {
                    transitions: smallvec![(label_d, State(8))],
                    fallback_state: State(7),
                    attributes: StateAttributes::TRANSITIONS_TO_ACCEPTING,
                },
                StateTable {
                    transitions: smallvec![(label_d, State(8))],
                    fallback_state: State(7),
                    attributes: StateAttributes::ACCEPTING | StateAttributes::TRANSITIONS_TO_ACCEPTING,
                },
            ],
        };

        assert_eq!(result, expected);
    }

    #[test]
    fn child_descendant_and_child_wildcard_test() {
        // Query = $.x.*..a.*.b
        let label_a = JsonString::new("a");
        let label_a = (&label_a).into();

        let label_b = JsonString::new("b");
        let label_b = (&label_b).into();

        let label_x = JsonString::new("x");
        let label_x = (&label_x).into();

        let nfa = NondeterministicAutomaton {
            ordered_states: vec![
                NfaState::Direct(nfa::Transition::Labelled(label_x)),
                NfaState::Direct(nfa::Transition::Wildcard),
                NfaState::Recursive(nfa::Transition::Labelled(label_a)),
                NfaState::Direct(nfa::Transition::Wildcard),
                NfaState::Direct(nfa::Transition::Labelled(label_b)),
                NfaState::Accepting,
            ],
        };

        let result = minimize(nfa).unwrap();
        let expected = Automaton {
            states: vec![
                StateTable {
                    transitions: smallvec![],
                    fallback_state: State(0),
                    attributes: StateAttributes::REJECTING,
                },
                StateTable {
                    transitions: smallvec![(label_x, State(2))],
                    fallback_state: State(0),
                    attributes: StateAttributes::UNITARY,
                },
                StateTable {
                    transitions: smallvec![],
                    fallback_state: State(3),
                    attributes: StateAttributes::EMPTY,
                },
                StateTable {
                    transitions: smallvec![(label_a, State(4))],
                    fallback_state: State(3),
                    attributes: StateAttributes::EMPTY,
                },
                StateTable {
                    transitions: smallvec![(label_a, State(6))],
                    fallback_state: State(5),
                    attributes: StateAttributes::EMPTY,
                },
                StateTable {
                    transitions: smallvec![(label_a, State(4)), (label_b, State(8))],
                    fallback_state: State(3),
                    attributes: StateAttributes::TRANSITIONS_TO_ACCEPTING,
                },
                StateTable {
                    transitions: smallvec![(label_a, State(6)), (label_b, State(7))],
                    fallback_state: State(5),
                    attributes: StateAttributes::TRANSITIONS_TO_ACCEPTING,
                },
                StateTable {
                    transitions: smallvec![(label_a, State(4)), (label_b, State(8))],
                    fallback_state: State(3),
                    attributes: StateAttributes::ACCEPTING | StateAttributes::TRANSITIONS_TO_ACCEPTING,
                },
                StateTable {
                    transitions: smallvec![(label_a, State(4))],
                    fallback_state: State(3),
                    attributes: StateAttributes::ACCEPTING,
                },
            ],
        };

        assert_eq!(result, expected);
    }
}
