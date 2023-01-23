use super::nfa::{NfaState, NfaStateId};
use super::small_set::{SmallSet, SmallSet256};
use super::Label;
use super::{Automaton, NondeterministicAutomaton, State as DfaStateId, TransitionTable};
use crate::debug;
use crate::query::automaton::Transition;
use crate::query::error::CompilerError;
use smallvec::smallvec;
use vector_map::VecMap;

/// Turn the [`NondeterministicAutomaton`] to an equivalent minimal deterministic [`Automaton`].
pub(crate) fn minimize(nfa: NondeterministicAutomaton) -> Result<Automaton, CompilerError> {
    let minimizer = Minimizer {
        nfa,
        superstates: VecMap::new(),
        checkpoints: VecMap::new(),
        active_superstates: vec![],
        dfa_states: vec![],
    };

    minimizer.run()
}

pub(crate) struct Minimizer<'q> {
    /// The NFA being minimized.
    nfa: NondeterministicAutomaton<'q>,
    /// All superstates created thus far mapping to their index in the DFA being constructed.
    superstates: VecMap<SmallSet256, DfaStateId>,
    /// Map from superstates to the furthest reachable checkpoint on a path leading to that superstate.
    checkpoints: VecMap<SmallSet256, NfaStateId>,
    /// Superstates that have not been processed and expanded yet.
    active_superstates: Vec<SmallSet256>,
    /// All superstates created thus far, in order matching the `superstates` map.
    dfa_states: Vec<TransitionTable<'q>>,
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
        self.dfa_states.push(TransitionTable {
            transitions: smallvec![],
            fallback_state: Self::rejecting_state(),
        });

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
            let identifier = (self.superstates.len() + 1)
                .try_into()
                .map(DfaStateId)
                .map_err(CompilerError::QueryTooComplex)?;
            self.superstates.insert(superstate, identifier);
            self.active_superstates.push(superstate);
            debug!("New superstate created: {superstate:?} {identifier}");
        }

        Ok(())
    }

    /// Create the superstate's [`TransitionTable`] by processing all transitions
    /// of NFA states within the superstate.
    fn process_superstate(&mut self, current_superstate: SmallSet256) -> Result<(), CompilerError> {
        let current_checkpoint = self.determine_checkpoint(current_superstate);
        debug!(
            "Expanding superstate: {current_superstate:?}, last checkpoint is {current_checkpoint:?}"
        );

        let mut transitions = self.process_nfa_transitions(current_superstate);
        debug!("Raw transitions: {:?}", transitions);

        self.normalize_superstate_transitions(&mut transitions, current_checkpoint)?;
        debug!("Normalized transitions: {:?}", transitions);

        // Translate the transitions to the data model expected by TransitionTable.
        let translated_transitions = transitions
            .into_iter()
            .map(|(label, state)| Transition::Labelled(label, self.superstates[&state]))
            .collect();
        debug!("Translated transitions: {translated_transitions:?}");

        // If a checkpoint was reached, its singleton superstate is this DFA state's fallback state.
        // Otherwise, we set the fallback to the rejecting state.
        self.dfa_states.push(TransitionTable {
            transitions: translated_transitions,
            fallback_state: current_checkpoint
                .map(|x| [x.0].into())
                .map_or(Self::rejecting_state(), |x: SmallSet256| {
                    self.superstates[&x]
                }),
        });

        Ok(())
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
    ) -> VecMap<&'q Label, SmallSet256> {
        let mut transitions: VecMap<&Label, SmallSet256> = VecMap::new();

        for nfa_state in current_superstate.iter().map(NfaStateId) {
            match self.nfa[nfa_state] {
                // Direct states simply have a single transition to the next state in the NFA.
                // Recursive transitions also have a self-loop, but that is handled by the
                // checkpoints mechanism - here we only handle the forward transition.
                NfaState::Direct(label) | NfaState::Recursive(label) => {
                    debug!(
                        "Considering transition {nfa_state} --{label:?}-> {}",
                        nfa_state.next()
                    );
                    // Add the target NFA state to the target superstate, or create a singleton
                    // set if this is the first transition via this label encountered in the loop.
                    if let Some(target) = transitions.get_mut(&label) {
                        target.insert(nfa_state.next().0);
                    } else {
                        transitions.insert(label, [nfa_state.next().0].into());
                    }
                }
                NfaState::Accepting => (),
            }
        }

        transitions
    }

    /// Use the checkpoints to perform normalization of superstates
    /// and activate them if needed.
    fn normalize_superstate_transitions(
        &mut self,
        transitions: &mut VecMap<&Label, SmallSet256>,
        current_checkpoint: Option<NfaStateId>,
    ) -> Result<(), CompilerError> {
        for (_, state) in transitions.iter_mut() {
            if let Some(checkpoint) = current_checkpoint {
                state.insert(checkpoint.0);
            }

            self.normalize(state);
            self.activate_if_new(*state)?;

            if let Some(checkpoint) = current_checkpoint {
                self.checkpoints.insert(*state, checkpoint);
            }
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
