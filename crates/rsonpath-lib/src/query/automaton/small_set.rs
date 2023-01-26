use std::{collections::BTreeSet, fmt::Debug};

pub(crate) trait SmallSet<T: Copy + PartialOrd + Ord>: IntoIterator<Item = T> {
    fn len(&self) -> usize;

    fn is_empty(&self) -> bool;

    fn insert(&mut self, elem: T);

    /// If the set is a singleton, returns the only element.
    /// Otherwise, returns `None`.
    fn singleton(&self) -> Option<T>;

    fn iter(&self) -> <Self as IntoIterator>::IntoIter;

    /// Remove all elements from the set.
    fn clear(&mut self);

    /// Removes all elements smaller than `cutoff` from the set.
    fn remove_all_before(&mut self, cutoff: T);
}

#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub(crate) struct SmallSet256 {
    half_1: SmallSet128,
    half_2: SmallSet128,
}

#[derive(Default, Clone, Copy, PartialEq, Eq)]
struct SmallSet128 {
    bitmask: u128,
}

impl SmallSet<u8> for SmallSet256 {
    fn len(&self) -> usize {
        self.half_1.len() + self.half_2.len()
    }

    fn is_empty(&self) -> bool {
        self.half_1.is_empty() && self.half_2.is_empty()
    }

    fn insert(&mut self, elem: u8) {
        if elem < 128 {
            self.half_1.insert(elem)
        } else {
            self.half_2.insert(elem - 128)
        }
    }

    fn iter(&self) -> SmallSet256Iter {
        SmallSet256Iter {
            half_1: self.half_1.iter(),
            half_2: self.half_2.iter(),
        }
    }

    fn singleton(&self) -> Option<u8> {
        if self.half_1.is_empty() {
            self.half_2.singleton().map(|x| x + 128)
        } else if self.half_2.is_empty() {
            self.half_1.singleton()
        } else {
            None
        }
    }

    fn clear(&mut self) {
        self.half_1.clear();
        self.half_2.clear();
    }

    fn remove_all_before(&mut self, cutoff: u8) {
        if cutoff < 128 {
            self.half_1.remove_all_before(cutoff)
        } else {
            self.half_1.clear();
            self.half_2.remove_all_before(cutoff - 128);
        }
    }
}

impl SmallSet<u8> for SmallSet128 {
    fn len(&self) -> usize {
        self.bitmask.count_ones() as usize
    }

    fn is_empty(&self) -> bool {
        self.bitmask == 0
    }

    fn insert(&mut self, elem: u8) {
        self.bitmask |= 1 << elem;
    }

    fn iter(&self) -> SmallSet128Iter {
        SmallSet128Iter {
            bitmask: self.bitmask,
        }
    }

    fn singleton(&self) -> Option<u8> {
        let elem = self.bitmask.trailing_zeros();
        let elem_mask = 1_u128.wrapping_shl(elem);
        let remainder = self.bitmask ^ elem_mask;

        // CAST: trivially safe as bitmask can have at most 128 zeroes.
        (remainder == 0).then_some(elem as u8)
    }

    fn clear(&mut self) {
        self.bitmask = 0;
    }

    fn remove_all_before(&mut self, cutoff: u8) {
        let mask: u128 = 0xFFFF_FFFF_FFFF_FFFF_FFFF_FFFF_FFFF_FFFF << cutoff;
        self.bitmask &= mask;
    }
}

impl IntoIterator for SmallSet128 {
    type Item = u8;
    type IntoIter = SmallSet128Iter;

    fn into_iter(self) -> Self::IntoIter {
        self.iter()
    }
}

impl<const N: usize> From<[u8; N]> for SmallSet256 {
    fn from(arr: [u8; N]) -> Self {
        Self::from_iter(arr.into_iter())
    }
}

impl From<&[u8]> for SmallSet256 {
    fn from(arr: &[u8]) -> Self {
        Self::from_iter(arr.iter().copied())
    }
}

impl PartialEq<BTreeSet<u8>> for SmallSet256 {
    fn eq(&self, other: &BTreeSet<u8>) -> bool {
        self.len() == other.len() && self.iter().all(|x| other.contains(&x))
    }
}

impl PartialEq<SmallSet256> for BTreeSet<u8> {
    #[inline(always)]
    fn eq(&self, other: &SmallSet256) -> bool {
        other.eq(self)
    }
}

impl IntoIterator for SmallSet256 {
    type Item = u8;
    type IntoIter = SmallSet256Iter;

    fn into_iter(self) -> Self::IntoIter {
        self.iter()
    }
}

impl FromIterator<u8> for SmallSet256 {
    fn from_iter<T: IntoIterator<Item = u8>>(iter: T) -> Self {
        let mut result = Self::default();
        for elem in iter {
            result.insert(elem);
        }
        result
    }
}

impl Debug for SmallSet256 {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_set().entries(self.iter()).finish()
    }
}

pub(crate) struct SmallSet256Iter {
    half_1: SmallSet128Iter,
    half_2: SmallSet128Iter,
}

impl Iterator for SmallSet256Iter {
    type Item = u8;

    fn next(&mut self) -> Option<Self::Item> {
        self.half_1
            .next()
            .or_else(|| self.half_2.next().map(|x| x + 128))
    }
}

pub(crate) struct SmallSet128Iter {
    bitmask: u128,
}

impl Iterator for SmallSet128Iter {
    type Item = u8;

    fn next(&mut self) -> Option<Self::Item> {
        let next_elem = self.bitmask.trailing_zeros();

        if next_elem == 128 {
            return None;
        }

        let elem_mask = 1 << next_elem;
        self.bitmask ^= elem_mask;

        // CAST: trivially safe as bitmask can have at most 128 zeroes.
        Some(next_elem as u8)
    }
}

#[cfg(test)]
mod tests256 {
    use super::*;
    use proptest::{collection, proptest, strategy::Strategy};
    use std::collections::BTreeSet;

    const MAX_SIZE: usize = 256;
    const MAX_ELEM: u8 = 255;

    fn any_state() -> impl proptest::strategy::Strategy<Value = u8> {
        (0_u8..=MAX_ELEM).prop_map_into()
    }

    proptest! {
        #[test]
        fn from_slice(btree_set in collection::btree_set(any_state(), 0..=MAX_SIZE)) {
            let vec: Vec<u8> = btree_set.into_iter().collect();
            let slice: &[u8] = &vec;
            let state_set: SmallSet256 = slice.into();

            let round_trip: Vec<u8> = state_set.iter().collect();

            assert_eq!(&round_trip, slice);
        }

        #[test]
        fn singleton_some(value in any_state()) {
            let state_set: SmallSet256 = [value].into();

            assert_eq!(Some(value), state_set.singleton());
        }

        #[test]
        fn singleton_some_many(btree_set in collection::btree_set(any_state(), 2..=MAX_SIZE)) {
            let vec: Vec<u8> = btree_set.into_iter().collect();
            let slice: &[u8] = &vec;
            let state_set: SmallSet256 = slice.into();

            assert_eq!(None, state_set.singleton());
        }

        #[test]
        fn remove_all_below(btree_set in collection::btree_set(any_state(), 0..=MAX_SIZE), state in any_state()) {
            let expected_btree_set = BTreeSet::from_iter(btree_set.iter().copied().filter(|&x| x >= state));
            let mut state_set = SmallSet256::from_iter(btree_set.into_iter());

            state_set.remove_all_before(state);

            assert_eq!(expected_btree_set, state_set);
        }
    }

    #[test]
    fn singleton_none_empty() {
        let state_set = SmallSet256::default();

        assert_eq!(None, state_set.singleton());
    }
}
