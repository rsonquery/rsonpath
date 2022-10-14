use super::NfaStateId;
use std::{collections::BTreeSet, fmt::Debug};

#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub(crate) struct Superstate {
    bitmask: u128,
}

impl Superstate {
    pub(crate) fn len(&self) -> usize {
        self.bitmask.count_ones() as usize
    }

    pub(crate) fn insert(&mut self, elem: NfaStateId) {
        self.bitmask |= 1 << elem.0;
    }

    pub(crate) fn iter(&self) -> SuperstateIter {
        SuperstateIter {
            bitmask: self.bitmask,
        }
    }

    /// If the set is a singleton, returns the only element.
    /// Otherwise, returns `None`.
    pub(crate) fn singleton(&self) -> Option<NfaStateId> {
        let elem = self.bitmask.trailing_zeros();
        let elem_mask = 1u128.wrapping_shl(elem);
        let remainder = self.bitmask ^ elem_mask;

        if remainder == 0 {
            Some(NfaStateId(elem as u8))
        } else {
            None
        }
    }

    /// Removes all elements smaller than `cutoff` from the set.
    pub(crate) fn remove_all_before(&mut self, cutoff: NfaStateId) {
        let mask: u128 = 0xFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFF << cutoff.0;
        self.bitmask &= mask;
    }
}

impl<const N: usize> From<[NfaStateId; N]> for Superstate {
    fn from(arr: [NfaStateId; N]) -> Self {
        Self::from_iter(arr.into_iter())
    }
}

impl From<&[NfaStateId]> for Superstate {
    fn from(arr: &[NfaStateId]) -> Self {
        Self::from_iter(arr.iter().copied())
    }
}

impl PartialEq<BTreeSet<NfaStateId>> for Superstate {
    fn eq(&self, other: &BTreeSet<NfaStateId>) -> bool {
        self.len() == other.len() && self.iter().all(|x| other.contains(&x))
    }
}

impl PartialEq<Superstate> for BTreeSet<NfaStateId> {
    fn eq(&self, other: &Superstate) -> bool {
        other.eq(self)
    }
}

impl FromIterator<NfaStateId> for Superstate {
    fn from_iter<T: IntoIterator<Item = NfaStateId>>(iter: T) -> Self {
        let mut result = Self::default();
        for elem in iter {
            result.insert(elem);
        }
        result
    }
}

impl Debug for Superstate {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_set().entries(self.iter()).finish()
    }
}

pub(crate) struct SuperstateIter {
    bitmask: u128,
}

impl Iterator for SuperstateIter {
    type Item = NfaStateId;

    fn next(&mut self) -> Option<Self::Item> {
        let next_elem = self.bitmask.trailing_zeros();

        if next_elem == 128 {
            return None;
        }

        let elem_mask = 1 << next_elem;
        self.bitmask ^= elem_mask;

        Some(NfaStateId(next_elem as u8))
    }
}

#[cfg(test)]
mod tests {
    use std::collections::BTreeSet;

    use super::*;
    use proptest::{collection, proptest, strategy::Strategy};

    fn any_state() -> impl proptest::strategy::Strategy<Value = NfaStateId> {
        (0u8..=127).prop_map_into()
    }

    proptest! {
        #[test]
        fn from_slice(btree_set in collection::btree_set(any_state(), 0..=128)) {
            let vec: Vec<NfaStateId> = btree_set.into_iter().collect();
            let slice: &[NfaStateId] = &vec;
            let state_set: Superstate = slice.into();

            let round_trip: Vec<NfaStateId> = state_set.iter().collect();

            assert_eq!(&round_trip, slice);
        }

        #[test]
        fn singleton_some(value in any_state()) {
            let state_set: Superstate = [value].into();

            assert_eq!(Some(value), state_set.singleton());
        }

        #[test]
        fn singleton_some_many(btree_set in collection::btree_set(any_state(), 2..=128)) {
            let vec: Vec<NfaStateId> = btree_set.into_iter().collect();
            let slice: &[NfaStateId] = &vec;
            let state_set: Superstate = slice.into();

            assert_eq!(None, state_set.singleton());
        }

        #[test]
        fn remove_all_below(btree_set in collection::btree_set(any_state(), 0..=128), state in any_state()) {
            let expected_btree_set = BTreeSet::from_iter(btree_set.iter().copied().filter(|&x| x >= state));
            let mut state_set = Superstate::from_iter(btree_set.into_iter());

            state_set.remove_all_before(state);

            assert_eq!(expected_btree_set, state_set);
        }
    }

    #[test]
    fn singleton_none_empty() {
        let state_set = Superstate::default();

        assert_eq!(None, state_set.singleton());
    }
}
