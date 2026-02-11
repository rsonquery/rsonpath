//! Representation of linear sets of integers that capture JSONPath array index and slicing access.
//!
//! A _linear_ set is a set of integers in an arithmetic sequence: {a, a + k, a + 2k, ...}.
//! It can be either bounded or infinite. It has the general form of a:b:k, where b is the end bound.
//! These are exactly the sets that the slicing operator can express. The index selector is also a linear set
//! of form {a:a+1:1} (step doesn't matter, but 1 is chosen as canonical).
//!
//! These sets are closed under intersection, which is a crucial property.
//! This module allows manipulating a set of transitions from a single state labelled with linear sets
//! and automatic resolution of transition overlaps.
//!
//! ## Motivation
//!
//! Regular sets capture what happens during determinization of the query NFA, when multiple slice and/or index
//! selectors need to be combined.
//!
//! Consider a scenario where we have a complex transition labelled with a regular set X to some set of states S,
//! and in comes a slice selector Y supposed to transition to {t}. It is not contained within the regular set X, but
//! has a non-empty intersection. In that case we need the result to be transitions:
//! * over X-Y to S
//! * over Y-X to {t}
//! * over X intersect Y to S+{t}
//!
//! Linear sets are not closed under complementation or subtraction - the representation of X-Y might be
//! complex. Therefore, we rely on the engine to process the transitions from first to last and during compilation
//! maintain the correct order. The order is enforced via **priorities**. To represent the case above, we emit the
//! following prioritized transitions:
//! * {prio. 2} over X intersect Y to S+{t}
//! * {prio. 1} over X to S
//! * {prio. 1} over Y to {t}
//!
//! The semantics are correct as long as the transitions are taken in non-increasing order of priorities.
//!
//! Intersection of two linear sets is always a linear set. Finding such intersection is not trivial,
//! but doable. One needs to solve a linear congruence to find the smallest common element of the two sets,
//! if one exists, and then step by the least common multiple of the step values.
//!
//! ## Optimizations
//!
//! 1. This module automatically optimizes a few cases. A linear set is always represented canonically
//!    &ndash; empty sets are not relevant, so such transitions are not created; sets containing a single element
//!    are always represented as a singleton.
//!
//! 2. Engine runtime depends on the number of transitions it needs to process, so emitting superficial transitions
//!    is discouraged. In the case where the overlap of X and Y is X (or Y), it suffices to emit only X intersect Y and Y (or X)
//!    &ndash; the third transition would have never been taken anyway. Moreover, if X = Y then only one transition needs to be emitted.
//!    This check is not perfect, as a transition can be dominated by two other transitions piecewise ((X \cap Y) \cup (X \ cap Z) = X),
//!    but it does help reduce transitions, especially in cases where X is a singleton.

use super::{
    small_set::{SmallSet as _, SmallSet256},
    ArrayTransitionLabel, SimpleSlice,
};
use rsonpath_syntax::num::JsonUInt;
use std::collections::HashMap;

#[derive(Debug)]
pub(super) struct ArrayTransitionSet {
    transitions: HashMap<LinearSet, LinearSetTransition>,
}

#[derive(Debug)]
struct LinearSetTransition {
    priority: usize,
    target: SmallSet256,
}

#[derive(Debug, PartialEq, Eq, Clone, Copy, Hash)]
enum LinearSet {
    Singleton(JsonUInt),
    BoundedSlice(JsonUInt, JsonUInt, JsonUInt),
    OpenEndedSlice(JsonUInt, JsonUInt),
}

pub(super) struct ArrayTransitionSetIterator {
    transitions: std::vec::IntoIter<(LinearSet, LinearSetTransition)>,
}

impl ArrayTransitionSet {
    pub(super) fn new() -> Self {
        Self {
            transitions: HashMap::new(),
        }
    }

    pub(super) fn add_transition(&mut self, label: ArrayTransitionLabel, target: SmallSet256) {
        use std::collections::hash_map::Entry;
        let Some(label) = LinearSet::from_label(label) else {
            return;
        };
        let overlaps: Vec<_> = self
            .transitions
            .iter()
            .filter_map(|(other, trans)| {
                let overlap = other.overlap_with(&label)?;
                let priority = trans.priority + 1;
                let mut overlap_target = target;
                overlap_target.union(&trans.target);

                Some((overlap, LinearSetTransition { priority, target }))
            })
            .collect();

        for (label, trans) in overlaps {
            match self.transitions.entry(label) {
                Entry::Occupied(mut entry) => {
                    let entry = entry.get_mut();
                    entry.priority = std::cmp::max(entry.priority, trans.priority);
                    entry.target.union(&trans.target);
                }
                Entry::Vacant(entry) => {
                    entry.insert(trans);
                }
            }
        }

        match self.transitions.entry(label) {
            // Label overlapped (entirely) with some existing label, so it is already handled.
            Entry::Occupied(_) => (),
            Entry::Vacant(entry) => {
                entry.insert(LinearSetTransition { priority: 1, target });
            }
        }
    }

    pub(super) fn states_mut(&mut self) -> impl Iterator<Item = &mut SmallSet256> {
        self.transitions.iter_mut().map(|(_, trans)| &mut trans.target)
    }
}

impl ArrayTransitionSetIterator {
    fn new(mut transitions: Vec<(LinearSet, LinearSetTransition)>) -> Self {
        transitions.sort_by(|(_, x), (_, y)| x.priority.cmp(&y.priority).reverse());
        Self {
            transitions: transitions.into_iter(),
        }
    }
}

impl IntoIterator for ArrayTransitionSet {
    type Item = (ArrayTransitionLabel, SmallSet256);

    type IntoIter = ArrayTransitionSetIterator;

    fn into_iter(self) -> Self::IntoIter {
        ArrayTransitionSetIterator::new(self.transitions.into_iter().collect())
    }
}

impl Iterator for ArrayTransitionSetIterator {
    type Item = (ArrayTransitionLabel, SmallSet256);

    fn next(&mut self) -> Option<Self::Item> {
        let (label, transition) = self.transitions.next()?;
        Some(match label {
            LinearSet::Singleton(idx) => (ArrayTransitionLabel::Index(idx), transition.target),
            LinearSet::BoundedSlice(start, end, step) => (
                ArrayTransitionLabel::Slice(SimpleSlice::new(start, Some(end), step)),
                transition.target,
            ),
            LinearSet::OpenEndedSlice(start, step) => (
                ArrayTransitionLabel::Slice(SimpleSlice::new(start, None, step)),
                transition.target,
            ),
        })
    }
}

impl LinearSet {
    fn from_label(label: ArrayTransitionLabel) -> Option<Self> {
        match label {
            ArrayTransitionLabel::Index(idx) => Some(Self::Singleton(idx)),
            ArrayTransitionLabel::Slice(slice) => {
                if slice.step == JsonUInt::ZERO {
                    None
                } else if let Some(end) = slice.end {
                    if slice.start >= end {
                        None
                    } else if slice.start.as_u64().saturating_add(slice.step.as_u64()) >= end.as_u64() {
                        // Only one item within the slice.
                        Some(Self::Singleton(slice.start))
                    } else {
                        assert!(
                            end > JsonUInt::ZERO,
                            "end is a one-past-last index, must not be zero in a valid query"
                        );
                        Some(Self::BoundedSlice(slice.start, end, slice.step))
                    }
                } else {
                    Some(Self::OpenEndedSlice(slice.start, slice.step))
                }
            }
        }
    }

    fn from_slice(start: JsonUInt, end: Option<JsonUInt>, step: JsonUInt) -> Option<Self> {
        if step == JsonUInt::ZERO {
            None
        } else if let Some(end) = end {
            if start >= end {
                None
            } else if start.as_u64().saturating_add(step.as_u64()) >= end.as_u64() {
                // Only one item within the slice.
                Some(Self::Singleton(start))
            } else {
                assert!(
                    end > JsonUInt::ZERO,
                    "end is a one-past-last index, must not be zero in a valid query"
                );
                Some(Self::BoundedSlice(start, end, step))
            }
        } else {
            Some(Self::OpenEndedSlice(start, step))
        }
    }

    fn overlap_with(&self, other: &Self) -> Option<Self> {
        // Assume the first set starts not-later, otherwise flip.
        if self.start() > other.start() {
            return other.overlap_with(self);
        }
        assert_ne!(self.step().as_u64(), 0, "empty sets must be discarded on construction");
        assert_ne!(other.step().as_u64(), 0, "empty sets must be discarded on construction");

        // First we take both sets as if they are open-ended and linear.
        // We can take an overlap under that assumption and then simply apply the lower of the two end constraints,
        // if any, to obtain the ultimate result.
        //
        // If first_element is beyond the range of JsonUInt it will fail conversion at the end of this function,
        // and result in an empty set (empty transition = no transition). This is correct behavior - first element
        // out of bounds means there are no valid elements.
        let (first_element, gcd) = find_first_element(
            self.start().into(),
            self.step().into(),
            other.start().into(),
            other.step().into(),
        )?;
        // Perform the min of ends where None is treated as larger than everything.
        let end = match (self.end_exclusive(), other.end_exclusive()) {
            (None, Some(x)) | (Some(x), None) => Some(x),
            (None, None) => None,
            (Some(x), Some(y)) => Some(std::cmp::min(x, y)),
        };
        // This can also overflow both JsonUInt and u64. We saturate and then convert to JsonUInt.
        // A step that fails this conversion is essentially infinite, which means we need to emit a set containing only the
        // first_element.
        let common_step = (self.step().as_u64() / gcd).saturating_mul(other.step().as_u64());

        let start = JsonUInt::try_from(first_element).ok()?;

        return match JsonUInt::try_from(common_step).ok() {
            Some(step) => Self::from_slice(start, end, step),
            None if end.is_some_and(|end| end <= start) => None,
            None => Some(Self::Singleton(start)),
        };

        fn find_first_element(a: i64, k: i64, b: i64, l: i64) -> Option<(i64, u64)> {
            // Now we have two sets, S1=[a::k] and S2=[b::l], a <= b.
            // Clearly b \in S2 and every +l step is in S2.
            // Now the difference between b and the next element of S1 is given by:
            //     c = k - (b - a) mod k
            // (note that this can be zero if b-a is a multiple of k, which makes sense)
            //
            // To get a common element we need to apply +l steps until we land in S1.
            // We get the following equation:
            //     c + lx = 0 mod k
            // or
            //     lx = -c mod k
            //
            // This is a linear congruence which has a known algorithm using extended Euclid.
            let c = umod(k - (b - a), k);
            let (jumps, gcd) = solve_linear_congruence(l, c, k)?;
            Some((jumps.checked_mul(l)?.checked_add(b)?, gcd))
        }
    }

    fn start(&self) -> JsonUInt {
        match self {
            Self::Singleton(i) | Self::BoundedSlice(i, _, _) | Self::OpenEndedSlice(i, _) => *i,
        }
    }

    fn end_exclusive(&self) -> Option<JsonUInt> {
        match self {
            Self::Singleton(i) => JsonUInt::try_from(i.as_u64() + 1).ok(),
            Self::BoundedSlice(_, i, _) => Some(*i),
            Self::OpenEndedSlice(_, _) => None,
        }
    }

    fn step(&self) -> JsonUInt {
        match self {
            Self::Singleton(_) => JsonUInt::ONE,
            Self::BoundedSlice(_, _, s) | Self::OpenEndedSlice(_, s) => *s,
        }
    }
}

/// Unsigned modulo, a.k.a. proper mathematical modulo.
/// Returns the unique number k such that
/// x === k mod m AND 0 <= k < m
/// m must be positive.
fn umod(x: i64, m: i64) -> i64 {
    assert!(m > 0, "m must be positive");
    let k = x % m;
    if k < 0 {
        m + k
    } else {
        k
    }
}

/// Solve ax = b mod m.
/// If any solution exists, returns the smallest solution and the unique gcd(a, m).
fn solve_linear_congruence(a: i64, b: i64, m: i64) -> Option<(i64, u64)> {
    // If gcd(a, m) does not divide b mod m, then there are no solutions.
    // Otherwise, find the (x,y) that solve ax - my = gcd(a, m)
    // and take x*(b/gcd(a,m)) mod (m/gcd(a,m)) as the solution.
    //
    // Note that there may be multiple solutions if gcd(a, m) > 1,
    // but this always gives the smallest one.
    let b = umod(b, m);
    let (x, gcd) = extended_euclid(a, m);

    if b % gcd != 0 {
        None
    } else {
        Some((
            umod(x.checked_mul(b / gcd)?, m / gcd),
            u64::try_from(gcd).expect("negative gcd"),
        ))
    }
}

/// Only x and gcd is returned.
fn extended_euclid(a: i64, b: i64) -> (i64, i64) {
    let (mut old_r, mut r) = (a, b);
    let (mut old_x, mut x) = (1, 0);

    while r != 0 {
        let quotient = old_r / r;
        (old_r, r) = (r, old_r - quotient * r);
        (old_x, x) = (x, old_x - quotient * x);
    }

    (old_x, old_r)
}

#[cfg(test)]
mod tests {
    use test_case::test_case;

    use super::LinearSet;

    #[test_case(1, 1 => (0, 1))]
    #[test_case(4, 10 => (-2, 2))]
    #[test_case(7, 10 => (3, 1))]
    #[test_case(8, 10 => (-1, 2))]
    #[test_case(161, 28 => (-1, 7))]
    fn extended_euclid_tests(a: i64, b: i64) -> (i64, i64) {
        super::extended_euclid(a, b)
    }

    #[test_case(7, 3, 10 => Some((9, 1)))]
    #[test_case(7, 8, 10 => Some((4, 1)))]
    #[test_case(8, 3, 10 => None)]
    #[test_case(8, 2, 10 => Some((4, 2)))]
    #[test_case(94_253_004_627_829, 666_084_837_845, 888_777_666_555_119 => Some((2_412_193, 121_216_531)))]
    #[test_case(6_253_004_621, 2_156_208_490, 27_815_089_521 => Some((116, 215_620_849)))]
    fn linear_congruence_tests(a: i64, b: i64, m: i64) -> Option<(i64, u64)> {
        super::solve_linear_congruence(a, b, m)
    }

    #[test_case(LinearSet::Singleton(1.into()), LinearSet::Singleton(1.into()) => Some(LinearSet::Singleton(1.into())))]
    #[test_case(LinearSet::Singleton(1.into()), LinearSet::Singleton(2.into()) => None)]
    #[test_case(
        LinearSet::Singleton(3.into()),
        LinearSet::BoundedSlice(3.into(), 15.into(), 2.into())
        => Some(LinearSet::Singleton(3.into())))]
    #[test_case(
        LinearSet::Singleton(5.into()),
        LinearSet::BoundedSlice(3.into(), 15.into(), 2.into())
        => Some(LinearSet::Singleton(5.into())))]
    #[test_case(
        LinearSet::Singleton(15.into()),
        LinearSet::BoundedSlice(3.into(), 15.into(), 2.into())
        => None)]
    #[test_case(
        LinearSet::BoundedSlice(3.into(), 15.into(), 2.into()),
        LinearSet::BoundedSlice(3.into(), 15.into(), 2.into())
        => Some(LinearSet::BoundedSlice(3.into(), 15.into(), 2.into())))]
    #[test_case(
        LinearSet::BoundedSlice(5.into(), 1024.into(), 7.into()),
        LinearSet::BoundedSlice(3.into(), 911.into(), 10.into())
        => Some(LinearSet::BoundedSlice(33.into(), 911.into(), 70.into())))]
    #[test_case(
        LinearSet::OpenEndedSlice(5.into(), 7.into()),
        LinearSet::OpenEndedSlice(3.into(), 10.into())
        => Some(LinearSet::OpenEndedSlice(33.into(), 70.into())))]
    #[test_case(
        LinearSet::OpenEndedSlice(5.into(), 8.into()),
        LinearSet::OpenEndedSlice(3.into(), 10.into())
        => Some(LinearSet::OpenEndedSlice(13.into(), 40.into())))]
    #[test_case(
        LinearSet::OpenEndedSlice(156_208_490.try_into().unwrap(), 6_253_004_621_u64.try_into().unwrap()),
        LinearSet::OpenEndedSlice(4_253_004_621_u64.try_into().unwrap(), 27_815_089_521_u64.try_into().unwrap())
        => Some(LinearSet::OpenEndedSlice(87_698_273_184_u64.try_into().unwrap(), 806_637_596_109_u64.try_into().unwrap())))]
    #[test_case(
        LinearSet::OpenEndedSlice(666_123_456_789_u64.try_into().unwrap(), 888_777_666_555_119_u64.try_into().unwrap()),
        LinearSet::OpenEndedSlice(888_777_705_174_063_u64.try_into().unwrap(), 94_253_004_627_829_u64.try_into().unwrap())
        => None)]
    fn overlap_tests(a: LinearSet, b: LinearSet) -> Option<LinearSet> {
        a.overlap_with(&b)
    }
}
