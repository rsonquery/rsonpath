//! Determinization and minimization of an NFA into the final DFA used by the engines.

use std::sync::Arc;

// NOTE: Some comments in this module are outdated, because the minimizer doesn't
// actually produce minimal automata as of now - see #91.
use super::{
    array_transition_set::ArrayTransitionSet,
    error::CompilerError,
    nfa::{self, NfaState, NfaStateId},
    small_set::{SmallSet as _, SmallSet256},
    state::StateAttributesBuilder,
    Automaton, NondeterministicAutomaton, State as DfaStateId, StateAttributes, StateTable,
};
use crate::{automaton::ArrayTransition, debug, string_pattern::StringPattern};
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

pub(super) struct Minimizer {
    /// The NFA being minimized.
    nfa: NondeterministicAutomaton,
    /// All superstates created thus far mapping to their index in the DFA being constructed.
    superstates: VecMap<SmallSet256, DfaStateId>,
    /// Map from superstates to the furthest reachable checkpoint on a path leading to that superstate.
    checkpoints: VecMap<SmallSet256, NfaStateId>,
    /// Superstates that have not been processed and expanded yet.
    active_superstates: SmallVec<[SmallSet256; 2]>,
    /// All superstates created thus far, in order matching the `superstates` map.
    dfa_states: Vec<StateTable>,
    /// Set of activated DFA states that are accepting.
    accepting: SmallSet256,
}

#[derive(Debug)]
struct SuperstateTransitionTable {
    array: ArrayTransitionSet,
    member: VecMap<Arc<StringPattern>, SmallSet256>,
    wildcard: SmallSet256,
}

/**
 * Minimization proceeds by superset construction, made easier and ensuring minimality
 * due to *checkpoints*.
 *
 * Every state with a self-loop becomes a checkpoint. They have two crucial properties:
 *   1. Any path from the initial to the accepting state goes through each checkpoint.
 *   2. Each superstate containing
 *      a) a checkpoint and;
 *      b) some states on the path from the initial state to that checkpoint,
 *      is equivalent to a superstate without the b) states.
 *
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
impl Minimizer {
    /// Main loop of the algorithm. Initialize rejecting and initial states
    /// and perform expansion until we run out of active states.
    fn run(mut self) -> Result<Automaton, CompilerError> {
        // Rejecting state has no outgoing transitions except for a self-loop.
        self.dfa_states.push(StateTable {
            array_transitions: smallvec![],
            member_transitions: smallvec![],
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
        let array_transitions = transitions
            .array
            .into_iter()
            .map(|(label, state)| ArrayTransition::new(label, self.superstates[&state]))
            .collect::<SmallVec<_>>();
        let member_transitions = transitions
            .member
            .into_iter()
            .map(|(label, state)| (label, self.superstates[&state]))
            .collect::<SmallVec<_>>();
        debug!("Translated transitions (array): {array_transitions:?}");
        debug!("Translated transitions (member): {member_transitions:?}");

        // If a checkpoint was reached, its singleton superstate is this DFA state's fallback state.
        // Otherwise, we set the fallback to the rejecting state.
        let id = self.superstates[&current_superstate];
        let fallback_state = self.superstates[&transitions.wildcard];
        let attributes = self.build_attributes(id, &array_transitions, &member_transitions, fallback_state);
        let table = &mut self.dfa_states[id.0 as usize];
        table.array_transitions = array_transitions;
        table.member_transitions = member_transitions;
        table.fallback_state = fallback_state;
        table.attributes = attributes;

        Ok(())
    }

    /// Build attributes of a DFA state after all of its transitions have been
    /// determined.
    fn build_attributes(
        &self,
        id: DfaStateId,
        array_transitions: &[ArrayTransition],
        member_transitions: &[(Arc<StringPattern>, DfaStateId)],
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

        if self.accepting.contains(fallback.0)
            || array_transitions
                .iter()
                .any(|x| self.accepting.contains(x.target_state().0))
            || member_transitions.iter().any(|(_, s)| self.accepting.contains(s.0))
        {
            debug!("{id} has transitions to accepting");
            attrs = attrs.transitions_to_accepting();
        }
        if !array_transitions.is_empty() {
            debug!("{id} has an array index transition");
            attrs = attrs.has_array_transition();
        }
        if array_transitions
            .iter()
            .any(|x| self.accepting.contains(x.target_state().0))
        {
            debug!("{id} has an accepting array index transition");
            attrs = attrs.has_array_transition_to_accepting();
        }

        // Unitarity check:
        // 1. Fallback rejects.
        // 2. Only one transition that can match at most one element in a JSON, either:
        //   a) member transition; or
        //   b) array transition that matches only one index.
        let is_unitary = {
            fallback == Self::rejecting_state()
                && ((member_transitions.len() == 1 && array_transitions.is_empty())
                    || (array_transitions.len() == 1
                        && member_transitions.is_empty()
                        && array_transitions[0].label.matches_at_most_once()))
        };
        if is_unitary {
            debug!("{id} is unitary");
            attrs = attrs.unitary();
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
    ) -> Result<SuperstateTransitionTable, CompilerError> {
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
            array: ArrayTransitionSet::new(),
            member: VecMap::new(),
            wildcard: wildcard_targets,
        };

        for nfa_state in current_superstate.iter().map(NfaStateId) {
            match &self.nfa[nfa_state] {
                // Direct states simply have a single transition to the next state in the NFA.
                // Recursive transitions also have a self-loop, but that is handled by the
                // checkpoints mechanism - here we only handle the forward transition.
                NfaState::Direct(nfa::Transition::Member(label))
                | NfaState::Recursive(nfa::Transition::Member(label)) => {
                    debug!(
                        "Considering member transition {nfa_state} --{}-> {}",
                        std::str::from_utf8(label.unquoted()).unwrap_or("[invalid utf8]"),
                        nfa_state.next()?,
                    );
                    // Add the target NFA state to the target superstate, or create a singleton
                    // set if this is the first transition via this label encountered in the loop.
                    if let Some(target) = transitions.member.get_mut(label) {
                        target.insert(nfa_state.next()?.0);
                    } else {
                        let mut new_set = transitions.wildcard;
                        new_set.insert(nfa_state.next()?.0);
                        transitions.member.insert(label.clone(), new_set);
                    }
                }
                NfaState::Direct(nfa::Transition::Array(label))
                | NfaState::Recursive(nfa::Transition::Array(label)) => {
                    // Array transitions are trickier, as they can have overlap. For example,
                    // a transition over [5] overlaps with a transition over [3::2].
                    // If the incoming transition does not overlap with anything then it's easy and analogous
                    // to the member case - create a new singleton set with a single transition.
                    // Otherwise we need to solve conflicts with - potentially many! - existing transitions.
                    // Fortunately, the conflicts can be resolved one at a time.
                    // Assume we're processing --t1--> {s1} and there already is a --t2-->S2 (where S2 is a superstate),
                    // such that t1 overlaps with t2 (overlap(t1, t2) = t3).
                    // The resolution is to have the following transitions:
                    //   --t3--> S2+{s1}
                    //   --(t1-t3)--> {s1}
                    //   --(t2-t3)--> S2
                    // If t1 and t2 are slices then t3 is easy to compute and is also a slice.
                    // This is not the case for (t1-t3) or (t2-t3). Turns out this is actually a hard problem to solve.
                    // We can do away with a trick, however. As long as the engine always processes transitions in order
                    // and takes the first one that matches, it is enough for the procedure here to emit
                    //   --t3--> S2+{s1}
                    //   --t1--> {s1}
                    //   --t2--> S2
                    // and make sure the transition over t3 is put before the other two.
                    // The ArrayTransitionTable does that by assigning priorities to transitions and sorting them accordingly.
                    debug!(
                        "Considering array transition {nfa_state} --{}-> {}",
                        label,
                        nfa_state.next()?,
                    );
                    let mut new_set = transitions.wildcard;
                    new_set.insert(nfa_state.next()?.0);
                    transitions.array.add_transition(*label, new_set);
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
        for (_, state) in &mut transitions.member {
            normalize_one(self, state, current_checkpoint)?;
        }
        for state in &mut transitions.array.states_mut() {
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
    use pretty_assertions::assert_eq;
    use rsonpath_syntax::str::JsonString;
    use smallvec::smallvec;

    #[test]
    fn empty_query() {
        // Query = $
        let nfa = NondeterministicAutomaton {
            ordered_states: vec![NfaState::Accepting],
        };

        let mut result = minimize(nfa).unwrap();
        make_canonical(&mut result);
        let expected = Automaton {
            states: vec![
                StateTable {
                    array_transitions: smallvec![],
                    member_transitions: smallvec![],
                    fallback_state: State(0),
                    attributes: StateAttributes::REJECTING,
                },
                StateTable {
                    array_transitions: smallvec![],
                    member_transitions: smallvec![],
                    fallback_state: State(0),
                    attributes: StateAttributes::ACCEPTING,
                },
            ],
        };

        assert_eq!(result, expected);
    }

    #[test]
    fn simple_wildcard() {
        // Query = $.*
        let nfa = NondeterministicAutomaton {
            ordered_states: vec![NfaState::Direct(nfa::Transition::Wildcard), NfaState::Accepting],
        };

        let mut result = minimize(nfa).unwrap();
        make_canonical(&mut result);
        let expected = Automaton {
            states: vec![
                StateTable {
                    array_transitions: smallvec![],
                    member_transitions: smallvec![],
                    fallback_state: State(0),
                    attributes: StateAttributes::REJECTING,
                },
                StateTable {
                    array_transitions: smallvec![],
                    member_transitions: smallvec![],
                    fallback_state: State(2),
                    attributes: StateAttributes::TRANSITIONS_TO_ACCEPTING,
                },
                StateTable {
                    array_transitions: smallvec![],
                    member_transitions: smallvec![],
                    fallback_state: State(0),
                    attributes: StateAttributes::ACCEPTING,
                },
            ],
        };

        assert_eq!(result, expected);
    }

    #[test]
    fn simple_nonnegative_indexed() {
        // Query = $[0]
        let label = JsonUInt::ZERO;

        let nfa = NondeterministicAutomaton {
            ordered_states: vec![
                NfaState::Direct(nfa::Transition::Array(label.into())),
                NfaState::Accepting,
            ],
        };

        let mut result = minimize(nfa).unwrap();
        make_canonical(&mut result);
        let expected = Automaton {
            states: vec![
                StateTable {
                    array_transitions: smallvec![],
                    member_transitions: smallvec![],
                    fallback_state: State(0),
                    attributes: StateAttributes::REJECTING,
                },
                StateTable {
                    array_transitions: smallvec![ArrayTransition::new(ArrayTransitionLabel::Index(label), State(2))],
                    member_transitions: smallvec![],
                    fallback_state: State(0),
                    attributes: StateAttributes::UNITARY
                        | StateAttributes::TRANSITIONS_TO_ACCEPTING
                        | StateAttributes::HAS_ARRAY_TRANSITION
                        | StateAttributes::HAS_ARRAY_TRANSITION_TO_ACCEPTING,
                },
                StateTable {
                    array_transitions: smallvec![],
                    member_transitions: smallvec![],
                    fallback_state: State(0),
                    attributes: StateAttributes::ACCEPTING,
                },
            ],
        };

        assert_eq!(result, expected);
    }

    #[test]
    fn simple_descendant_wildcard() {
        // Query = $..*
        let nfa = NondeterministicAutomaton {
            ordered_states: vec![NfaState::Recursive(nfa::Transition::Wildcard), NfaState::Accepting],
        };

        let mut result = minimize(nfa).unwrap();
        make_canonical(&mut result);
        let expected = Automaton {
            states: vec![
                StateTable {
                    array_transitions: smallvec![],
                    member_transitions: smallvec![],
                    fallback_state: State(0),
                    attributes: StateAttributes::REJECTING,
                },
                StateTable {
                    array_transitions: smallvec![],
                    member_transitions: smallvec![],
                    fallback_state: State(2),
                    attributes: StateAttributes::TRANSITIONS_TO_ACCEPTING,
                },
                StateTable {
                    array_transitions: smallvec![],
                    member_transitions: smallvec![],
                    fallback_state: State(2),
                    attributes: StateAttributes::ACCEPTING | StateAttributes::TRANSITIONS_TO_ACCEPTING,
                },
            ],
        };

        assert_eq!(result, expected);
    }

    #[test]
    fn interstitial_descendant_wildcard() {
        // Query = $..a.b..*.a..b
        let label_a = Arc::new(StringPattern::new(&JsonString::new("a")));
        let label_b = Arc::new(StringPattern::new(&JsonString::new("b")));

        let nfa = NondeterministicAutomaton {
            ordered_states: vec![
                NfaState::Recursive(nfa::Transition::Member(label_a.clone())),
                NfaState::Direct(nfa::Transition::Member(label_b.clone())),
                NfaState::Recursive(nfa::Transition::Wildcard),
                NfaState::Direct(nfa::Transition::Member(label_a.clone())),
                NfaState::Recursive(nfa::Transition::Member(label_b.clone())),
                NfaState::Accepting,
            ],
        };

        let mut result = minimize(nfa).unwrap();
        make_canonical(&mut result);
        let expected = Automaton {
            states: vec![
                StateTable {
                    array_transitions: smallvec![],
                    member_transitions: smallvec![],
                    fallback_state: State(0),
                    attributes: StateAttributes::REJECTING,
                },
                StateTable {
                    array_transitions: smallvec![],
                    member_transitions: smallvec![(label_a.clone(), State(2))],
                    fallback_state: State(1),
                    attributes: StateAttributes::EMPTY,
                },
                StateTable {
                    array_transitions: smallvec![],
                    member_transitions: smallvec![(label_a.clone(), State(2)), (label_b.clone(), State(3))],
                    fallback_state: State(1),
                    attributes: StateAttributes::EMPTY,
                },
                StateTable {
                    array_transitions: smallvec![],
                    member_transitions: smallvec![],
                    fallback_state: State(4),
                    attributes: StateAttributes::EMPTY,
                },
                StateTable {
                    array_transitions: smallvec![],
                    member_transitions: smallvec![(label_a, State(5))],
                    fallback_state: State(4),
                    attributes: StateAttributes::EMPTY,
                },
                StateTable {
                    array_transitions: smallvec![],
                    member_transitions: smallvec![(label_b.clone(), State(6))],
                    fallback_state: State(5),
                    attributes: StateAttributes::TRANSITIONS_TO_ACCEPTING,
                },
                StateTable {
                    array_transitions: smallvec![],
                    member_transitions: smallvec![(label_b, State(6))],
                    fallback_state: State(5),
                    attributes: StateAttributes::ACCEPTING | StateAttributes::TRANSITIONS_TO_ACCEPTING,
                },
            ],
        };

        assert_eq!(result, expected);
    }

    #[test]
    fn interstitial_nondescendant_wildcard() {
        // Query = $..a.b.*.a..b
        let label_a = Arc::new(StringPattern::new(&JsonString::new("a")));
        let label_b = Arc::new(StringPattern::new(&JsonString::new("b")));

        let nfa = NondeterministicAutomaton {
            ordered_states: vec![
                NfaState::Recursive(nfa::Transition::Member(label_a.clone())),
                NfaState::Direct(nfa::Transition::Member(label_b.clone())),
                NfaState::Direct(nfa::Transition::Wildcard),
                NfaState::Direct(nfa::Transition::Member(label_a.clone())),
                NfaState::Recursive(nfa::Transition::Member(label_b.clone())),
                NfaState::Accepting,
            ],
        };

        let mut result = minimize(nfa).unwrap();
        make_canonical(&mut result);
        let expected = Automaton {
            states: vec![
                StateTable {
                    array_transitions: smallvec![],
                    member_transitions: smallvec![],
                    fallback_state: State(0),
                    attributes: StateAttributes::REJECTING,
                },
                StateTable {
                    array_transitions: smallvec![],
                    member_transitions: smallvec![(label_a.clone(), State(2))],
                    fallback_state: State(1),
                    attributes: StateAttributes::EMPTY,
                },
                StateTable {
                    array_transitions: smallvec![],
                    member_transitions: smallvec![(label_a.clone(), State(2)), (label_b.clone(), State(3))],
                    fallback_state: State(1),
                    attributes: StateAttributes::EMPTY,
                },
                StateTable {
                    array_transitions: smallvec![],
                    member_transitions: smallvec![(label_a.clone(), State(4))],
                    fallback_state: State(7),
                    attributes: StateAttributes::EMPTY,
                },
                StateTable {
                    array_transitions: smallvec![],
                    member_transitions: smallvec![(label_a.clone(), State(5)), (label_b.clone(), State(3))],
                    fallback_state: State(1),
                    attributes: StateAttributes::EMPTY,
                },
                StateTable {
                    array_transitions: smallvec![],
                    member_transitions: smallvec![(label_b.clone(), State(6))],
                    fallback_state: State(5),
                    attributes: StateAttributes::TRANSITIONS_TO_ACCEPTING,
                },
                StateTable {
                    array_transitions: smallvec![],
                    member_transitions: smallvec![(label_b, State(6))],
                    fallback_state: State(5),
                    attributes: StateAttributes::ACCEPTING | StateAttributes::TRANSITIONS_TO_ACCEPTING,
                },
                StateTable {
                    array_transitions: smallvec![],
                    member_transitions: smallvec![(label_a, State(5))],
                    fallback_state: State(1),
                    attributes: StateAttributes::EMPTY,
                },
            ],
        };

        assert_eq!(result, expected);
    }

    #[test]
    fn simple_multi_accepting() {
        // Query = $..a.*
        let label = Arc::new(StringPattern::new(&JsonString::new("a")));

        let nfa = NondeterministicAutomaton {
            ordered_states: vec![
                NfaState::Recursive(nfa::Transition::Member(label.clone())),
                NfaState::Direct(nfa::Transition::Wildcard),
                NfaState::Accepting,
            ],
        };

        let mut result = minimize(nfa).unwrap();
        make_canonical(&mut result);
        let expected = Automaton {
            states: vec![
                StateTable {
                    array_transitions: smallvec![],
                    member_transitions: smallvec![],
                    fallback_state: State(0),
                    attributes: StateAttributes::REJECTING,
                },
                StateTable {
                    array_transitions: smallvec![],
                    member_transitions: smallvec![(label.clone(), State(2)),],
                    fallback_state: State(1),
                    attributes: StateAttributes::EMPTY,
                },
                StateTable {
                    array_transitions: smallvec![],
                    member_transitions: smallvec![(label.clone(), State(3))],
                    fallback_state: State(4),
                    attributes: StateAttributes::TRANSITIONS_TO_ACCEPTING,
                },
                StateTable {
                    array_transitions: smallvec![],
                    member_transitions: smallvec![(label.clone(), State(3))],
                    fallback_state: State(4),
                    attributes: StateAttributes::ACCEPTING | StateAttributes::TRANSITIONS_TO_ACCEPTING,
                },
                StateTable {
                    array_transitions: smallvec![],
                    member_transitions: smallvec![(label, State(2))],
                    fallback_state: State(1),
                    attributes: StateAttributes::ACCEPTING,
                },
            ],
        };

        assert_eq!(result, expected);
    }

    #[test]
    fn simple_multi_accepting_nneg_index() {
        // Query = $..[3]
        let label = JsonUInt::ZERO;

        let nfa = NondeterministicAutomaton {
            ordered_states: vec![
                NfaState::Recursive(nfa::Transition::Array(label.into())),
                NfaState::Accepting,
            ],
        };

        let mut result = minimize(nfa).unwrap();
        make_canonical(&mut result);
        let expected = Automaton {
            states: vec![
                StateTable {
                    array_transitions: smallvec![],
                    member_transitions: smallvec![],
                    fallback_state: State(0),
                    attributes: StateAttributes::REJECTING,
                },
                StateTable {
                    array_transitions: smallvec![ArrayTransition::new(ArrayTransitionLabel::Index(label), State(2)),],
                    member_transitions: smallvec![],
                    fallback_state: State(1),
                    attributes: StateAttributes::TRANSITIONS_TO_ACCEPTING
                        | StateAttributes::HAS_ARRAY_TRANSITION
                        | StateAttributes::HAS_ARRAY_TRANSITION_TO_ACCEPTING,
                },
                StateTable {
                    array_transitions: smallvec![ArrayTransition::new(ArrayTransitionLabel::Index(label), State(2))],
                    member_transitions: smallvec![],
                    fallback_state: State(1),
                    attributes: StateAttributes::TRANSITIONS_TO_ACCEPTING
                        | StateAttributes::HAS_ARRAY_TRANSITION
                        | StateAttributes::HAS_ARRAY_TRANSITION_TO_ACCEPTING
                        | StateAttributes::ACCEPTING,
                },
            ],
        };

        assert_eq!(result, expected);
    }

    #[test]
    fn chained_wildcard_children() {
        // Query = $.a.*.*.*
        let label = Arc::new(StringPattern::new(&JsonString::new("a")));

        let nfa = NondeterministicAutomaton {
            ordered_states: vec![
                NfaState::Direct(nfa::Transition::Member(label.clone())),
                NfaState::Direct(nfa::Transition::Wildcard),
                NfaState::Direct(nfa::Transition::Wildcard),
                NfaState::Direct(nfa::Transition::Wildcard),
                NfaState::Accepting,
            ],
        };

        let mut result = minimize(nfa).unwrap();
        make_canonical(&mut result);
        let expected = Automaton {
            states: vec![
                StateTable {
                    array_transitions: smallvec![],
                    member_transitions: smallvec![],
                    fallback_state: State(0),
                    attributes: StateAttributes::REJECTING,
                },
                StateTable {
                    array_transitions: smallvec![],
                    member_transitions: smallvec![(label, State(2))],
                    fallback_state: State(0),
                    attributes: StateAttributes::UNITARY,
                },
                StateTable {
                    array_transitions: smallvec![],
                    member_transitions: smallvec![],
                    fallback_state: State(3),
                    attributes: StateAttributes::EMPTY,
                },
                StateTable {
                    array_transitions: smallvec![],
                    member_transitions: smallvec![],
                    fallback_state: State(4),
                    attributes: StateAttributes::EMPTY,
                },
                StateTable {
                    array_transitions: smallvec![],
                    member_transitions: smallvec![],
                    fallback_state: State(5),
                    attributes: StateAttributes::TRANSITIONS_TO_ACCEPTING,
                },
                StateTable {
                    array_transitions: smallvec![],
                    member_transitions: smallvec![],
                    fallback_state: State(0),
                    attributes: StateAttributes::ACCEPTING,
                },
            ],
        };

        assert_eq!(result, expected);
    }

    #[test]
    fn chained_wildcard_children_after_descendant() {
        // Query = $..a.*.*
        let label = Arc::new(StringPattern::new(&JsonString::new("a")));

        let nfa = NondeterministicAutomaton {
            ordered_states: vec![
                NfaState::Recursive(nfa::Transition::Member(label.clone())),
                NfaState::Direct(nfa::Transition::Wildcard),
                NfaState::Direct(nfa::Transition::Wildcard),
                NfaState::Accepting,
            ],
        };

        let mut result = minimize(nfa).unwrap();
        make_canonical(&mut result);
        let expected = Automaton {
            states: vec![
                StateTable {
                    array_transitions: smallvec![],
                    member_transitions: smallvec![],
                    fallback_state: State(0),
                    attributes: StateAttributes::REJECTING,
                },
                StateTable {
                    array_transitions: smallvec![],
                    member_transitions: smallvec![(label.clone(), State(2))],
                    fallback_state: State(1),
                    attributes: StateAttributes::EMPTY,
                },
                StateTable {
                    array_transitions: smallvec![],
                    member_transitions: smallvec![(label.clone(), State(3))],
                    fallback_state: State(7),
                    attributes: StateAttributes::EMPTY,
                },
                StateTable {
                    array_transitions: smallvec![],
                    member_transitions: smallvec![(label.clone(), State(4))],
                    fallback_state: State(5),
                    attributes: StateAttributes::TRANSITIONS_TO_ACCEPTING,
                },
                StateTable {
                    array_transitions: smallvec![],
                    member_transitions: smallvec![(label.clone(), State(4))],
                    fallback_state: State(5),
                    attributes: StateAttributes::ACCEPTING | StateAttributes::TRANSITIONS_TO_ACCEPTING,
                },
                StateTable {
                    array_transitions: smallvec![],
                    member_transitions: smallvec![(label.clone(), State(6))],
                    fallback_state: State(8),
                    attributes: StateAttributes::ACCEPTING | StateAttributes::TRANSITIONS_TO_ACCEPTING,
                },
                StateTable {
                    array_transitions: smallvec![],
                    member_transitions: smallvec![(label.clone(), State(3))],
                    fallback_state: State(7),
                    attributes: StateAttributes::ACCEPTING,
                },
                StateTable {
                    array_transitions: smallvec![],
                    member_transitions: smallvec![(label.clone(), State(6))],
                    fallback_state: State(8),
                    attributes: StateAttributes::TRANSITIONS_TO_ACCEPTING,
                },
                StateTable {
                    array_transitions: smallvec![],
                    member_transitions: smallvec![(label, State(2))],
                    fallback_state: State(1),
                    attributes: StateAttributes::ACCEPTING,
                },
            ],
        };

        assert_eq!(result, expected);
    }

    #[test]
    fn child_and_descendant() {
        // Query = $.x..a.b.a.b.c..d
        let label_a = Arc::new(StringPattern::new(&JsonString::new("a")));
        let label_b = Arc::new(StringPattern::new(&JsonString::new("b")));
        let label_c = Arc::new(StringPattern::new(&JsonString::new("c")));
        let label_d = Arc::new(StringPattern::new(&JsonString::new("d")));
        let label_x = Arc::new(StringPattern::new(&JsonString::new("x")));

        let nfa = NondeterministicAutomaton {
            ordered_states: vec![
                NfaState::Direct(nfa::Transition::Member(label_x.clone())),
                NfaState::Recursive(nfa::Transition::Member(label_a.clone())),
                NfaState::Direct(nfa::Transition::Member(label_b.clone())),
                NfaState::Direct(nfa::Transition::Member(label_a.clone())),
                NfaState::Direct(nfa::Transition::Member(label_b.clone())),
                NfaState::Direct(nfa::Transition::Member(label_c.clone())),
                NfaState::Recursive(nfa::Transition::Member(label_d.clone())),
                NfaState::Accepting,
            ],
        };

        let mut result = minimize(nfa).unwrap();
        make_canonical(&mut result);
        let expected = Automaton {
            states: vec![
                StateTable {
                    array_transitions: smallvec![],
                    member_transitions: smallvec![],
                    fallback_state: State(0),
                    attributes: StateAttributes::REJECTING,
                },
                StateTable {
                    array_transitions: smallvec![],
                    member_transitions: smallvec![(label_x, State(2))],
                    fallback_state: State(0),
                    attributes: StateAttributes::UNITARY,
                },
                StateTable {
                    array_transitions: smallvec![],
                    member_transitions: smallvec![(label_a.clone(), State(3))],
                    fallback_state: State(2),
                    attributes: StateAttributes::EMPTY,
                },
                StateTable {
                    array_transitions: smallvec![],
                    member_transitions: smallvec![(label_a.clone(), State(3)), (label_b.clone(), State(4))],
                    fallback_state: State(2),
                    attributes: StateAttributes::EMPTY,
                },
                StateTable {
                    array_transitions: smallvec![],
                    member_transitions: smallvec![(label_a.clone(), State(5))],
                    fallback_state: State(2),
                    attributes: StateAttributes::EMPTY,
                },
                StateTable {
                    array_transitions: smallvec![],
                    member_transitions: smallvec![(label_a.clone(), State(3)), (label_b, State(6))],
                    fallback_state: State(2),
                    attributes: StateAttributes::EMPTY,
                },
                StateTable {
                    array_transitions: smallvec![],
                    member_transitions: smallvec![(label_a, State(5)), (label_c, State(7))],
                    fallback_state: State(2),
                    attributes: StateAttributes::EMPTY,
                },
                StateTable {
                    array_transitions: smallvec![],
                    member_transitions: smallvec![(label_d.clone(), State(8))],
                    fallback_state: State(7),
                    attributes: StateAttributes::TRANSITIONS_TO_ACCEPTING,
                },
                StateTable {
                    array_transitions: smallvec![],
                    member_transitions: smallvec![(label_d, State(8))],
                    fallback_state: State(7),
                    attributes: StateAttributes::ACCEPTING | StateAttributes::TRANSITIONS_TO_ACCEPTING,
                },
            ],
        };

        assert_eq!(result, expected);
    }

    #[test]
    fn child_descendant_and_child_wildcard() {
        // Query = $.x.*..a.*.b
        let label_a = Arc::new(StringPattern::new(&JsonString::new("a")));
        let label_b = Arc::new(StringPattern::new(&JsonString::new("b")));
        let label_x = Arc::new(StringPattern::new(&JsonString::new("x")));

        let nfa = NondeterministicAutomaton {
            ordered_states: vec![
                NfaState::Direct(nfa::Transition::Member(label_x.clone())),
                NfaState::Direct(nfa::Transition::Wildcard),
                NfaState::Recursive(nfa::Transition::Member(label_a.clone())),
                NfaState::Direct(nfa::Transition::Wildcard),
                NfaState::Direct(nfa::Transition::Member(label_b.clone())),
                NfaState::Accepting,
            ],
        };

        let mut result = minimize(nfa).unwrap();
        make_canonical(&mut result);
        let expected = Automaton {
            states: vec![
                StateTable {
                    array_transitions: smallvec![],
                    member_transitions: smallvec![],
                    fallback_state: State(0),
                    attributes: StateAttributes::REJECTING,
                },
                StateTable {
                    array_transitions: smallvec![],
                    member_transitions: smallvec![(label_x, State(2))],
                    fallback_state: State(0),
                    attributes: StateAttributes::UNITARY,
                },
                StateTable {
                    array_transitions: smallvec![],
                    member_transitions: smallvec![],
                    fallback_state: State(3),
                    attributes: StateAttributes::EMPTY,
                },
                StateTable {
                    array_transitions: smallvec![],
                    member_transitions: smallvec![(label_a.clone(), State(4))],
                    fallback_state: State(3),
                    attributes: StateAttributes::EMPTY,
                },
                StateTable {
                    array_transitions: smallvec![],
                    member_transitions: smallvec![(label_a.clone(), State(5))],
                    fallback_state: State(8),
                    attributes: StateAttributes::EMPTY,
                },
                StateTable {
                    array_transitions: smallvec![],
                    member_transitions: smallvec![(label_a.clone(), State(5)), (label_b.clone(), State(6))],
                    fallback_state: State(8),
                    attributes: StateAttributes::TRANSITIONS_TO_ACCEPTING,
                },
                StateTable {
                    array_transitions: smallvec![],
                    member_transitions: smallvec![(label_a.clone(), State(4)), (label_b.clone(), State(7))],
                    fallback_state: State(3),
                    attributes: StateAttributes::ACCEPTING | StateAttributes::TRANSITIONS_TO_ACCEPTING,
                },
                StateTable {
                    array_transitions: smallvec![],
                    member_transitions: smallvec![(label_a.clone(), State(4))],
                    fallback_state: State(3),
                    attributes: StateAttributes::ACCEPTING,
                },
                StateTable {
                    array_transitions: smallvec![],
                    member_transitions: smallvec![(label_a, State(4)), (label_b, State(7))],
                    fallback_state: State(3),
                    attributes: StateAttributes::TRANSITIONS_TO_ACCEPTING,
                },
            ],
        };

        assert_eq!(result, expected);
    }

    #[test]
    fn all_name_and_wildcard_selectors() {
        // Query = $.a.b..c..d.*..*
        let label_a = Arc::new(StringPattern::new(&JsonString::new("a")));
        let label_b = Arc::new(StringPattern::new(&JsonString::new("b")));
        let label_c = Arc::new(StringPattern::new(&JsonString::new("c")));
        let label_d = Arc::new(StringPattern::new(&JsonString::new("d")));

        let nfa = NondeterministicAutomaton {
            ordered_states: vec![
                NfaState::Direct(nfa::Transition::Member(label_a.clone())),
                NfaState::Direct(nfa::Transition::Member(label_b.clone())),
                NfaState::Recursive(nfa::Transition::Member(label_c.clone())),
                NfaState::Recursive(nfa::Transition::Member(label_d.clone())),
                NfaState::Direct(nfa::Transition::Wildcard),
                NfaState::Recursive(nfa::Transition::Wildcard),
                NfaState::Accepting,
            ],
        };
        let expected = Automaton {
            states: vec![
                StateTable {
                    array_transitions: smallvec![],
                    member_transitions: smallvec![],
                    fallback_state: State(0),
                    attributes: StateAttributes::REJECTING,
                },
                StateTable {
                    array_transitions: smallvec![],
                    member_transitions: smallvec![(label_a, State(2)),],
                    fallback_state: State(0),
                    attributes: StateAttributes::UNITARY,
                },
                StateTable {
                    array_transitions: smallvec![],
                    member_transitions: smallvec![(label_b, State(3))],
                    fallback_state: State(0),
                    attributes: StateAttributes::UNITARY,
                },
                StateTable {
                    array_transitions: smallvec![],
                    member_transitions: smallvec![(label_c, State(4))],
                    fallback_state: State(3),
                    attributes: StateAttributes::EMPTY,
                },
                StateTable {
                    array_transitions: smallvec![],
                    member_transitions: smallvec![(label_d.clone(), State(5))],
                    fallback_state: State(4),
                    attributes: StateAttributes::EMPTY,
                },
                StateTable {
                    array_transitions: smallvec![],
                    member_transitions: smallvec![(label_d, State(6))],
                    fallback_state: State(6),
                    attributes: StateAttributes::EMPTY,
                },
                StateTable {
                    array_transitions: smallvec![],
                    member_transitions: smallvec![],
                    fallback_state: State(7),
                    attributes: StateAttributes::TRANSITIONS_TO_ACCEPTING,
                },
                StateTable {
                    array_transitions: smallvec![],
                    member_transitions: smallvec![],
                    fallback_state: State(7),
                    attributes: StateAttributes::ACCEPTING | StateAttributes::TRANSITIONS_TO_ACCEPTING,
                },
            ],
        };

        let mut result = minimize(nfa).unwrap();
        make_canonical(&mut result);

        assert_eq!(result, expected);
    }

    #[test]
    fn array_index_and_slice_combo() {
        // Query = $..[3][3::2][3:5:]
        // These overlap, but only on index 3.
        let label_3 = JsonUInt::from(3);
        let label_3_2 = SimpleSlice::new(3.into(), None, 2.into());
        let label_3_5 = SimpleSlice::new(3.into(), Some(5.into()), 1.into());

        let nfa = NondeterministicAutomaton {
            ordered_states: vec![
                NfaState::Recursive(nfa::Transition::Array(label_3.into())),
                NfaState::Direct(nfa::Transition::Array(label_3_2.into())),
                NfaState::Direct(nfa::Transition::Array(label_3_5.into())),
                NfaState::Accepting,
            ],
        };

        let mut result = minimize(nfa).unwrap();
        make_canonical(&mut result);
        let expected = Automaton {
            states: vec![
                StateTable {
                    array_transitions: smallvec![],
                    member_transitions: smallvec![],
                    fallback_state: State(0),
                    attributes: StateAttributes::REJECTING,
                },
                StateTable {
                    array_transitions: smallvec![ArrayTransition::new(ArrayTransitionLabel::Index(label_3), State(2)),],
                    member_transitions: smallvec![],
                    fallback_state: State(1),
                    attributes: StateAttributes::HAS_ARRAY_TRANSITION,
                },
                StateTable {
                    array_transitions: smallvec![
                        ArrayTransition::new(ArrayTransitionLabel::Index(label_3), State(6)),
                        ArrayTransition::new(ArrayTransitionLabel::Slice(label_3_2), State(3))
                    ],
                    member_transitions: smallvec![],
                    fallback_state: State(1),
                    attributes: StateAttributes::HAS_ARRAY_TRANSITION,
                },
                StateTable {
                    array_transitions: smallvec![
                        ArrayTransition::new(ArrayTransitionLabel::Index(label_3), State(5)),
                        ArrayTransition::new(ArrayTransitionLabel::Slice(label_3_5), State(4)),
                    ],
                    member_transitions: smallvec![],
                    fallback_state: State(1),
                    attributes: StateAttributes::HAS_ARRAY_TRANSITION
                        | StateAttributes::TRANSITIONS_TO_ACCEPTING
                        | StateAttributes::HAS_ARRAY_TRANSITION_TO_ACCEPTING,
                },
                StateTable {
                    array_transitions: smallvec![ArrayTransition::new(ArrayTransitionLabel::Index(label_3), State(2)),],
                    member_transitions: smallvec![],
                    fallback_state: State(1),
                    attributes: StateAttributes::HAS_ARRAY_TRANSITION | StateAttributes::ACCEPTING,
                },
                StateTable {
                    array_transitions: smallvec![
                        ArrayTransition::new(ArrayTransitionLabel::Index(label_3), State(6)),
                        ArrayTransition::new(ArrayTransitionLabel::Slice(label_3_2), State(3)),
                    ],
                    member_transitions: smallvec![],
                    fallback_state: State(1),
                    attributes: StateAttributes::HAS_ARRAY_TRANSITION | StateAttributes::ACCEPTING,
                },
                StateTable {
                    array_transitions: smallvec![
                        ArrayTransition::new(ArrayTransitionLabel::Index(label_3), State(7)),
                        ArrayTransition::new(ArrayTransitionLabel::Slice(label_3_2), State(3)),
                        ArrayTransition::new(ArrayTransitionLabel::Slice(label_3_5), State(4)),
                    ],
                    member_transitions: smallvec![],
                    fallback_state: State(1),
                    attributes: StateAttributes::HAS_ARRAY_TRANSITION
                        | StateAttributes::TRANSITIONS_TO_ACCEPTING
                        | StateAttributes::HAS_ARRAY_TRANSITION_TO_ACCEPTING,
                },
                StateTable {
                    array_transitions: smallvec![
                        ArrayTransition::new(ArrayTransitionLabel::Index(label_3), State(7)),
                        ArrayTransition::new(ArrayTransitionLabel::Slice(label_3_2), State(3)),
                        ArrayTransition::new(ArrayTransitionLabel::Slice(label_3_5), State(4)),
                    ],
                    member_transitions: smallvec![],
                    fallback_state: State(1),
                    attributes: StateAttributes::HAS_ARRAY_TRANSITION
                        | StateAttributes::TRANSITIONS_TO_ACCEPTING
                        | StateAttributes::HAS_ARRAY_TRANSITION_TO_ACCEPTING
                        | StateAttributes::ACCEPTING,
                },
            ],
        };

        assert_eq!(result, expected);
    }

    #[test]
    fn direct_slice() {
        // Query = $[1:3]
        let label = SimpleSlice::new(1.into(), Some(3.into()), 1.into());

        let nfa = NondeterministicAutomaton {
            ordered_states: vec![
                NfaState::Direct(nfa::Transition::Array(label.into())),
                NfaState::Accepting,
            ],
        };

        let mut result = minimize(nfa).unwrap();
        make_canonical(&mut result);
        let expected = Automaton {
            states: vec![
                StateTable {
                    array_transitions: smallvec![],
                    member_transitions: smallvec![],
                    fallback_state: State(0),
                    attributes: StateAttributes::REJECTING,
                },
                StateTable {
                    array_transitions: smallvec![ArrayTransition::new(ArrayTransitionLabel::Slice(label), State(2)),],
                    member_transitions: smallvec![],
                    fallback_state: State(0),
                    attributes: StateAttributes::TRANSITIONS_TO_ACCEPTING
                        | StateAttributes::HAS_ARRAY_TRANSITION
                        | StateAttributes::HAS_ARRAY_TRANSITION_TO_ACCEPTING,
                },
                StateTable {
                    array_transitions: smallvec![],
                    member_transitions: smallvec![],
                    fallback_state: State(0),
                    attributes: StateAttributes::ACCEPTING,
                },
            ],
        };

        assert_eq!(result, expected);
    }

    /// DFA creation is unstable - it can result in many different isomorphic automaton structures.
    /// This function relabels the states in a canonical way so that they can be compared for equality.
    fn make_canonical(dfa: &mut Automaton) {
        let mut translation = vec![0; dfa.states.len()];
        let mut stack = vec![1_u8];
        let mut i = 1_u8;

        while let Some(state) = stack.pop() {
            if state == 0 || translation[state as usize] != 0 {
                continue;
            }
            translation[state as usize] = i;
            i += 1;
            stack.push(dfa.states[state as usize].fallback_state.0);

            for trans in &dfa.states[state as usize].array_transitions {
                stack.push(trans.target.0);
            }
            for (_, target) in &dfa.states[state as usize].member_transitions {
                stack.push(target.0);
            }
        }

        let mut idx = 0_u8;
        let mut current_placement = translation.clone();
        while (idx as usize) < translation.len() {
            let c_idx = current_placement[idx as usize];
            if idx != c_idx {
                dfa.states.swap(idx as usize, c_idx as usize);
                current_placement.swap(idx as usize, c_idx as usize);
            } else {
                dfa.states[idx as usize].fallback_state.0 =
                    translation[dfa.states[idx as usize].fallback_state.0 as usize];
                for trans in &mut dfa.states[idx as usize].array_transitions {
                    trans.target.0 = translation[trans.target.0 as usize];
                }
                for (_, target) in &mut dfa.states[idx as usize].member_transitions {
                    target.0 = translation[target.0 as usize];
                }
                idx += 1;
            }
        }
    }
}
