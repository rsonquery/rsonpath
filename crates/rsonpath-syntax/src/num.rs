//! JSON number types expressible in a JSONPath query.
//!
//! Exposes the [`JsonInt`] and [`JsonUInt`] types
//! that can represent any numbers in the range [-2<sup>53</sup>+1, 2<sup>53</sup>-1],
//! with the unsigned version additionally guaranteed to be non-negative. All operations
//! implemented are automatically checked for over- and underflow.
//!
//! This is governed by the [I-JSON IETF specification](https://www.rfc-editor.org/rfc/rfc7493.html#section-2).
//! All numbers appearing in a JSONPath query are required to be I-JSON conformant
//! (see [RFC 2.1-4.1](https://www.ietf.org/archive/id/draft-ietf-jsonpath-base-21.html#section-2.1-4.1)).
//! This includes index values, all values in slice selectors, and constants
//! in filter comparison expressions.
//!
//! # Examples
//! ```
//! # use rsonpath_syntax::num::{JsonInt, JsonUInt};
//! // An i32/u32 converts directly to JsonInt/JsonUInt.
//! let a = JsonInt::from(-42);
//! let b = JsonUInt::from(42);
//! // i64/u64 has to be checked for overflow.
//! let c = JsonInt::try_from(42_000_000_000_000_i64).expect("within range");
//! let d = JsonInt::try_from(42_000_000_000_000_000_i64).expect_err("too large");
//!
//! assert_eq!(a.as_i64(), -42);
//! assert_eq!(b.as_u64(), 42);
//! assert_eq!(c.as_i64(), 42_000_000_000_000_i64);
//! ```
pub mod error;

use crate::num::error::{JsonIntOverflowError, JsonIntParseError};
use std::{
    fmt::{self, Display, Formatter},
    num::{NonZeroU32, NonZeroU64},
    str::FromStr,
};

/// Signed interoperable JSON integer.
///
/// Provides an [IETF-conforming integer value](https://www.rfc-editor.org/rfc/rfc7493.html#section-2)
/// Values are \[-2<sup>53</sup>+1, 2<sup>53</sup>-1].
///
/// All values in a JSONPath query are limited to this range for interoperability
/// (see [RFC 2.1-4.1](https://www.ietf.org/archive/id/draft-ietf-jsonpath-base-21.html#section-2.1-4.1)).
///
/// The unsigned version is [`JsonUInt`].
///
/// # Examples
/// ```
/// # use rsonpath_syntax::num::JsonInt;
/// let two = JsonInt::from(2);
/// let zero = JsonInt::from(0);
/// let negative = JsonInt::from(-2);
///
/// assert_eq!(two.as_i64(), 2);
/// assert_eq!(zero.as_i64(), 0);
/// assert_eq!(negative.as_i64(), -2);
///
/// let too_big = JsonInt::try_from(1_i64 << 53).expect_err("out of range");
/// let too_small = JsonInt::try_from(-(1_i64 << 53)).expect_err("out of range");
/// ```
#[derive(Clone, Copy, Default, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct JsonInt(i64);

/// Unsigned interoperable JSON integer.
///
/// Provides an [IETF-conforming integer value](https://www.rfc-editor.org/rfc/rfc7493.html#section-2)
/// guaranteed to be non-negative. Values are \[0, (2<sup>53</sup>)-1].
///
/// All values in a JSONPath query are limited to the \[-2<sup>53</sup>+1, (2<sup>53</sup>)-1]
/// range for interoperability
/// (see [RFC 2.1-4.1](https://www.ietf.org/archive/id/draft-ietf-jsonpath-base-21.html#section-2.1-4.1)).
/// Some, like array indices, are additionally restricted to the non-negative part.
///
/// The signed version is [`JsonInt`].
///
/// # Examples
/// ```
/// # use rsonpath_syntax::num::JsonUInt;
/// let two = JsonUInt::from(2);
/// let zero = JsonUInt::from(0);
///
/// assert_eq!(two.as_u64(), 2);
/// assert_eq!(zero.as_u64(), 0);
///
/// let too_big = JsonUInt::try_from(1_u64 << 53).expect_err("out of range");
/// ```
#[derive(Clone, Copy, Default, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct JsonUInt(u64);

/// Unsigned interoperable JSON integer known to be non-zero.
///
/// Provides an [IETF-conforming integer value](https://www.rfc-editor.org/rfc/rfc7493.html#section-2)
/// guaranteed to be positive. Values are \(0, (2<sup>53</sup>)-1].
///
/// All values in a JSONPath query are limited to the \[-2<sup>53</sup>+1, (2<sup>53</sup>)-1]
/// range for interoperability
/// (see [RFC 2.1-4.1](https://www.ietf.org/archive/id/draft-ietf-jsonpath-base-21.html#section-2.1-4.1)).
/// Some, like array indices, are additionally restricted to the non-negative part, while
/// indexing from the end of an array requires a positive value.
///
/// The zero-compatible version is [`JsonUInt`].
///
/// # Examples
/// ```
/// # use rsonpath_syntax::num::JsonNonZeroUInt;
/// let two = JsonNonZeroUInt::try_from(2).expect("within range");
/// assert_eq!(two.as_u64(), 2);
///
/// let zero = JsonNonZeroUInt::try_from(0).expect_err("out of range");
/// let too_big = JsonNonZeroUInt::try_from(1_u64 << 53).expect_err("out of range");
/// ```
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct JsonNonZeroUInt(NonZeroU64);

/// The upper unsigned inclusive bound on JSON integers (2<sup>53</sup>-1).
const JSON_UINT_UPPER_LIMIT: u64 = (1 << 53) - 1;
/// The upper inclusive bound on JSON integers (2<sup>53</sup>-1).
const JSON_INT_UPPER_LIMIT: i64 = (1 << 53) - 1;
/// The lower inclusive bound on JSON integers (-2<sup>53</sup>+1).
const JSON_INT_LOWER_LIMIT: i64 = -(1 << 53) + 1;

impl JsonInt {
    /// A constant value of zero. Equivalent to [`JsonInt::default`](`Default::default`).
    ///
    /// # Examples
    /// ```
    /// # use rsonpath_syntax::num::JsonInt;
    /// assert_eq!(JsonInt::ZERO.as_i64(), 0);
    /// ```
    pub const ZERO: Self = Self::new(0);

    /// A constant value of one.
    ///
    /// # Examples
    /// ```
    /// # use rsonpath_syntax::num::JsonInt;
    /// assert_eq!(JsonInt::ONE.as_i64(), 1);
    /// ```
    pub const ONE: Self = Self::new(1);

    /// A constant for the smallest expressible value.
    ///
    /// # Examples
    /// ```
    /// # use rsonpath_syntax::num::JsonInt;
    /// let min_i64 = -(1 << 53) + 1;
    ///
    /// assert_eq!(JsonInt::MIN.as_i64(), min_i64);
    /// assert_eq!(JsonInt::try_from(min_i64).expect("within range"), JsonInt::MIN);
    /// ```
    pub const MIN: Self = Self::new(JSON_INT_LOWER_LIMIT);

    /// A constant for the largest expressible value.
    ///
    /// # Examples
    /// ```
    /// # use rsonpath_syntax::num::JsonInt;
    /// let max_i64 = (1 << 53) - 1;
    ///
    /// assert_eq!(JsonInt::MAX.as_i64(), max_i64);
    /// assert_eq!(JsonInt::try_from(max_i64).expect("within range"), JsonInt::MAX);
    /// ```
    pub const MAX: Self = Self::new(JSON_INT_UPPER_LIMIT);

    /// Create a new value from a [`i64`].
    #[must_use]
    const fn new(index: i64) -> Self {
        Self(index)
    }

    /// Increase the integer by one.
    ///
    /// # Errors
    /// Will return `Err` if the increment causes the [`JsonInt`] to exceed
    /// the upper limit of [`JsonInt::MAX`].
    ///
    /// # Examples
    /// ```
    /// # use rsonpath_syntax::num::JsonInt;
    /// let mut x = JsonInt::ZERO;
    /// x.try_increment().expect("within range");
    /// assert_eq!(x.as_i64(), 1);
    ///
    /// let mut y = JsonInt::MIN;
    /// y.try_increment().expect("within range");
    /// assert_eq!(y.as_i64(), -(1 << 53) + 2);
    ///
    /// JsonInt::MAX.try_increment().expect_err("out of range");
    /// ```
    #[inline]
    pub fn try_increment(&mut self) -> Result<(), JsonIntOverflowError> {
        let new_index = self.0 + 1;
        if new_index <= JSON_INT_UPPER_LIMIT {
            self.0 = new_index;
            Ok(())
        } else {
            Err(JsonIntOverflowError::int_neg_overflow(new_index))
        }
    }

    /// Return the value stored as a regular [`i64`].
    ///
    /// # Examples
    /// ```
    /// # use rsonpath_syntax::num::JsonInt;
    /// let val = JsonInt::from(42);
    /// assert_eq!(val.as_i64(), 42);
    /// ```
    #[must_use]
    #[inline(always)]
    pub const fn as_i64(&self) -> i64 {
        self.0
    }

    /// Return the negation of the value.
    ///
    /// This is guaranteed to succeed, as the valid range is symmetrical.
    /// ```
    /// # use rsonpath_syntax::num::JsonInt;
    /// let x = JsonInt::from(-42);
    /// assert_eq!(x.neg().as_i64(), 42);
    /// ```
    #[must_use]
    #[inline(always)]
    pub const fn neg(&self) -> Self {
        Self(-self.0)
    }

    /// Return the absolute value of this integer as a [`JsonUInt`].
    ///
    /// This is guaranteed to succeed, as the valid range is symmetrical.
    ///
    /// # Examples
    /// ```
    /// # use rsonpath_syntax::num::{JsonInt, JsonUInt};
    /// let pos = JsonInt::from(42);
    /// let neg = JsonInt::from(-42);
    /// assert_eq!(neg.abs().as_u64(), 42);
    /// assert_eq!(pos.abs().as_u64(), 42);
    /// ```
    #[inline(always)]
    #[must_use]
    pub const fn abs(&self) -> JsonUInt {
        JsonUInt(self.0.unsigned_abs())
    }
}

impl JsonUInt {
    /// A constant value of zero. Equivalent to [`JsonUInt::default`](`Default::default`).
    ///
    /// # Examples
    /// ```
    /// # use rsonpath_syntax::num::JsonUInt;
    /// assert_eq!(JsonUInt::ZERO.as_u64(), 0);
    /// ```
    pub const ZERO: Self = Self::new(0);

    /// A constant value of one.
    ///
    /// # Examples
    /// ```
    /// # use rsonpath_syntax::num::JsonUInt;
    /// assert_eq!(JsonUInt::ONE.as_u64(), 1);
    /// ```
    pub const ONE: Self = Self::new(1);

    /// A constant for the largest expressible value.
    ///
    /// # Examples
    /// ```
    /// # use rsonpath_syntax::num::JsonUInt;
    /// let max_u64 = (1 << 53) - 1;
    ///
    /// assert_eq!(JsonUInt::MAX.as_u64(), max_u64);
    /// assert_eq!(JsonUInt::try_from(max_u64).expect("within range"), JsonUInt::MAX);
    /// ```
    pub const MAX: Self = Self::new(JSON_UINT_UPPER_LIMIT);

    /// Create a new value from a [`u64`].
    #[must_use]
    const fn new(index: u64) -> Self {
        Self(index)
    }

    /// Increase the integer by one.
    ///
    /// # Errors
    /// Will return `Err` if the increment causes the [`JsonUInt`] to exceed
    /// the upper limit of [`JsonUInt::MAX`].
    ///
    /// # Examples
    /// ```
    /// # use rsonpath_syntax::num::JsonUInt;
    /// let mut x = JsonUInt::ZERO;
    /// x.try_increment().expect("within range");
    /// JsonUInt::MAX.try_increment().expect_err("out of range");
    ///
    /// assert_eq!(x.as_u64(), 1);
    /// ```
    #[inline]
    pub fn try_increment(&mut self) -> Result<(), JsonIntOverflowError> {
        let new_index = self.0 + 1;
        if new_index <= JSON_UINT_UPPER_LIMIT {
            self.0 = new_index;
            Ok(())
        } else {
            Err(JsonIntOverflowError::uint_pos_overflow(new_index))
        }
    }

    /// Return the negation of the value as a [`JsonInt`].
    ///
    /// This is guaranteed to succeed, as the valid range is symmetrical.
    /// ```
    /// # use rsonpath_syntax::num::{JsonInt, JsonUInt};
    /// let x = JsonUInt::from(42);
    /// let y = JsonInt::from(-42);
    /// assert_eq!(x.neg(), y);
    /// ```
    #[must_use]
    #[inline(always)]
    pub const fn neg(&self) -> JsonInt {
        JsonInt(-(self.0 as i64))
    }

    /// Return the value stored as a regular [`u64`].
    ///
    /// # Examples
    /// ```
    /// # use rsonpath_syntax::num::JsonUInt;
    /// let val = JsonUInt::from(42);
    /// assert_eq!(val.as_u64(), 42);
    /// ```
    #[must_use]
    #[inline(always)]
    pub const fn as_u64(&self) -> u64 {
        self.0
    }
}

impl JsonNonZeroUInt {
    #[must_use]
    const fn new(value: NonZeroU64) -> Self {
        Self(value)
    }

    /// Return the value stored as a [`NonZeroU64`].
    ///
    /// # Examples
    /// ```
    /// # use rsonpath_syntax::num::JsonNonZeroUInt;
    /// # use std::num::NonZeroU64;
    /// let val = JsonNonZeroUInt::try_from(42).unwrap();
    /// assert_eq!(val.as_non_zero_u64(), NonZeroU64::new(42).unwrap());
    /// ```
    #[must_use]
    #[inline(always)]
    pub const fn as_non_zero_u64(&self) -> NonZeroU64 {
        self.0
    }

    /// Return the value stored as a [`u64`].
    ///
    /// # Examples
    /// ```
    /// # use rsonpath_syntax::num::JsonNonZeroUInt;
    /// let val = JsonNonZeroUInt::try_from(42).unwrap();
    /// assert_eq!(val.as_u64(), 42);
    /// ```
    #[must_use]
    #[inline(always)]
    pub const fn as_u64(&self) -> u64 {
        self.0.get()
    }
}

impl TryFrom<i64> for JsonInt {
    type Error = JsonIntOverflowError;

    #[inline]
    fn try_from(value: i64) -> Result<Self, Self::Error> {
        if value > JSON_INT_UPPER_LIMIT {
            Err(JsonIntOverflowError::int_pos_overflow(value))
        } else if value < JSON_INT_LOWER_LIMIT {
            Err(JsonIntOverflowError::int_neg_overflow(value))
        } else {
            Ok(Self::new(value))
        }
    }
}

impl TryFrom<u64> for JsonInt {
    type Error = JsonIntOverflowError;

    #[inline]
    fn try_from(value: u64) -> Result<Self, Self::Error> {
        if value > i64::MAX as u64 {
            Err(JsonIntOverflowError::int_pos_overflow_u(value))
        } else {
            Self::try_from(value as i64)
        }
    }
}

impl From<i32> for JsonInt {
    // i32 is always in the range (-2^53, 2^53)
    #[inline]
    fn from(value: i32) -> Self {
        Self::new(i64::from(value))
    }
}

impl From<u32> for JsonInt {
    // u32 is always in the range (-2^53, 2^53)
    #[inline]
    fn from(value: u32) -> Self {
        Self::new(i64::from(value))
    }
}

impl From<JsonInt> for i64 {
    #[inline(always)]
    fn from(value: JsonInt) -> Self {
        value.0
    }
}

impl From<JsonUInt> for JsonInt {
    #[inline(always)]
    fn from(value: JsonUInt) -> Self {
        // This is always safe due to the type invariant bounds.
        Self::new(value.0 as i64)
    }
}

impl FromStr for JsonInt {
    type Err = JsonIntParseError;

    #[inline]
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match i64::from_str(s) {
            Ok(x) => x.try_into().map_err(|e| Self::Err::parse_conversion_err(s, &e)),
            Err(err) => Err(Self::Err::int_parse_error(s, err.kind())),
        }
    }
}

impl TryFrom<u64> for JsonUInt {
    type Error = JsonIntOverflowError;

    #[inline]
    fn try_from(value: u64) -> Result<Self, Self::Error> {
        if value > JSON_UINT_UPPER_LIMIT {
            Err(JsonIntOverflowError::uint_pos_overflow(value))
        } else {
            Ok(Self::new(value))
        }
    }
}

impl TryFrom<i64> for JsonUInt {
    type Error = JsonIntOverflowError;

    #[inline]
    fn try_from(value: i64) -> Result<Self, Self::Error> {
        if value < 0 {
            Err(JsonIntOverflowError::negative_uint(value))
        } else {
            Self::try_from(value as u64)
        }
    }
}

impl From<u32> for JsonUInt {
    // u32 is always in the range [0, 2^53)
    #[inline]
    fn from(value: u32) -> Self {
        Self::new(u64::from(value))
    }
}

impl TryFrom<i32> for JsonUInt {
    type Error = JsonIntOverflowError;

    #[inline]
    fn try_from(value: i32) -> Result<Self, Self::Error> {
        if value < 0 {
            Err(JsonIntOverflowError::negative_uint(i64::from(value)))
        } else {
            Ok(Self::from(value as u32))
        }
    }
}

impl From<JsonUInt> for u64 {
    #[inline(always)]
    fn from(value: JsonUInt) -> Self {
        value.0
    }
}

impl TryFrom<JsonInt> for JsonUInt {
    type Error = JsonIntOverflowError;

    #[inline]
    fn try_from(value: JsonInt) -> Result<Self, Self::Error> {
        if value.0 < 0 {
            Err(JsonIntOverflowError::negative_uint(value.0))
        } else {
            Ok(Self::new(value.0 as u64))
        }
    }
}

impl FromStr for JsonUInt {
    type Err = JsonIntParseError;

    #[inline]
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match i64::from_str(s) {
            // u64 would work but i64 gives us a better error message for negative values.
            Ok(x) => x.try_into().map_err(|e| Self::Err::parse_conversion_err(s, &e)),
            Err(err) => Err(Self::Err::uint_parse_error(s, err.kind())),
        }
    }
}

impl From<NonZeroU32> for JsonNonZeroUInt {
    // NonZeroU32 is always in the range (0, 2^53)
    #[inline]
    fn from(value: NonZeroU32) -> Self {
        Self::new(NonZeroU64::from(value))
    }
}

impl From<NonZeroU64> for JsonNonZeroUInt {
    // NonZeroU64 is always in the range (0, 2^53)
    #[inline]
    fn from(value: NonZeroU64) -> Self {
        Self::new(value)
    }
}

impl TryFrom<u32> for JsonNonZeroUInt {
    type Error = JsonIntOverflowError;

    #[inline]
    fn try_from(value: u32) -> Result<Self, Self::Error> {
        Self::try_from(u64::from(value))
    }
}

impl TryFrom<i32> for JsonNonZeroUInt {
    type Error = JsonIntOverflowError;

    #[inline]
    fn try_from(value: i32) -> Result<Self, Self::Error> {
        Self::try_from(i64::from(value))
    }
}

impl TryFrom<u64> for JsonNonZeroUInt {
    type Error = JsonIntOverflowError;

    #[inline]
    fn try_from(value: u64) -> Result<Self, Self::Error> {
        if value > JSON_UINT_UPPER_LIMIT {
            Err(JsonIntOverflowError::uint_pos_overflow(value))
        } else if let Some(x) = NonZeroU64::new(value) {
            Ok(Self(x))
        } else {
            Err(JsonIntOverflowError::zero_non_zero_uint())
        }
    }
}

impl TryFrom<i64> for JsonNonZeroUInt {
    type Error = JsonIntOverflowError;

    #[inline]
    fn try_from(value: i64) -> Result<Self, Self::Error> {
        if value < 0 {
            Err(JsonIntOverflowError::negative_uint(value))
        } else {
            Self::try_from(value as u64)
        }
    }
}

impl TryFrom<JsonUInt> for JsonNonZeroUInt {
    type Error = JsonIntOverflowError;

    #[inline]
    fn try_from(value: JsonUInt) -> Result<Self, Self::Error> {
        Self::try_from(value.0)
    }
}

impl From<JsonNonZeroUInt> for JsonUInt {
    #[inline]
    fn from(value: JsonNonZeroUInt) -> Self {
        Self::new(value.0.get())
    }
}

impl FromStr for JsonNonZeroUInt {
    type Err = JsonIntParseError;

    #[inline]
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match i64::from_str(s) {
            // u64 would work but i64 gives us a better error message for negative values.
            Ok(x) => x.try_into().map_err(|e| Self::Err::parse_conversion_err(s, &e)),
            Err(err) => Err(Self::Err::non_zero_uint_parse_error(s, err.kind())),
        }
    }
}

impl Display for JsonInt {
    #[inline]
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl Display for JsonUInt {
    #[inline]
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl Display for JsonNonZeroUInt {
    #[inline]
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

#[cfg(feature = "arbitrary")]
#[cfg_attr(docsrs, doc(cfg(feature = "arbitrary")))]
impl<'a> arbitrary::Arbitrary<'a> for JsonInt {
    #[inline]
    fn arbitrary(u: &mut arbitrary::Unstructured<'a>) -> arbitrary::Result<Self> {
        let val = u.int_in_range(JSON_INT_LOWER_LIMIT..=JSON_INT_UPPER_LIMIT)?;

        Ok(Self::new(val))
    }
}

#[cfg(feature = "arbitrary")]
#[cfg_attr(docsrs, doc(cfg(feature = "arbitrary")))]
impl<'a> arbitrary::Arbitrary<'a> for JsonUInt {
    #[inline]
    fn arbitrary(u: &mut arbitrary::Unstructured<'a>) -> arbitrary::Result<Self> {
        let val = u.int_in_range(0..=JSON_UINT_UPPER_LIMIT)?;

        Ok(Self::new(val))
    }
}

#[cfg(feature = "arbitrary")]
#[cfg_attr(docsrs, doc(cfg(feature = "arbitrary")))]
impl<'a> arbitrary::Arbitrary<'a> for JsonNonZeroUInt {
    #[inline]
    fn arbitrary(u: &mut arbitrary::Unstructured<'a>) -> arbitrary::Result<Self> {
        let val = u.int_in_range(1..=JSON_UINT_UPPER_LIMIT)?;

        Ok(Self::new(NonZeroU64::new(val).expect("range starts at 1")))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use pretty_assertions::assert_eq;

    #[test]
    fn int_upper_limit_sanity_check() {
        assert_eq!(JSON_INT_UPPER_LIMIT, (1 << 53) - 1);
        assert_eq!(JSON_INT_UPPER_LIMIT, 9_007_199_254_740_991);
    }

    #[test]
    fn int_lower_limit_sanity_check() {
        assert_eq!(JSON_INT_LOWER_LIMIT, -(1 << 53) + 1);
        assert_eq!(JSON_INT_LOWER_LIMIT, -9_007_199_254_740_991);
        assert_eq!(JSON_INT_LOWER_LIMIT, -JSON_INT_UPPER_LIMIT);
    }

    #[test]
    fn uint_upper_limit_sanity_check() {
        assert_eq!(JSON_UINT_UPPER_LIMIT, (1 << 53) - 1);
        assert_eq!(JSON_UINT_UPPER_LIMIT, 9_007_199_254_740_991);
        assert_eq!(JSON_INT_UPPER_LIMIT, JSON_UINT_UPPER_LIMIT as i64);
    }

    #[test]
    fn int_lower_limit_try_from_check() {
        let min = JsonInt::try_from(JSON_INT_LOWER_LIMIT).expect("JSON int lower_limit should be convertible.");
        let err = JsonInt::try_from(JSON_INT_LOWER_LIMIT - 1)
            .expect_err("Values below JSON int lower_limit should not be convertible.");
        assert_eq!(min.as_i64(), JSON_INT_LOWER_LIMIT);
        assert_eq!(
            err.to_string(),
            "value -9007199254740992 is below the range of JsonInt values [-9007199254740991..9007199254740991]"
        );
    }

    #[test]
    fn int_upper_limit_try_from_check() {
        let max = JsonInt::try_from(JSON_INT_UPPER_LIMIT).expect("JSON int upper_limit should be convertible.");
        let err = JsonInt::try_from(JSON_INT_UPPER_LIMIT + 1)
            .expect_err("Values in excess of JSON int upper_limit should not be convertible.");
        assert_eq!(max.as_i64(), JSON_INT_UPPER_LIMIT);
        assert_eq!(
            err.to_string(),
            "value 9007199254740992 is above the range of JsonInt values [-9007199254740991..9007199254740991]"
        );
    }

    #[test]
    fn uint_upper_limit_try_from_check() {
        let max = JsonUInt::try_from(JSON_UINT_UPPER_LIMIT).expect("JSON uint upper_limit should be convertible.");
        let err = JsonUInt::try_from(JSON_UINT_UPPER_LIMIT + 1)
            .expect_err("Values in excess of JSON uint upper_limit should not be convertible.");
        assert_eq!(max.as_u64(), JSON_UINT_UPPER_LIMIT);
        assert_eq!(
            err.to_string(),
            "value 9007199254740992 is above the range of JsonUInt values [0..9007199254740991]"
        );
    }

    #[test]
    fn non_zero_uint_try_from_zero_check() {
        let err_i32 = JsonNonZeroUInt::try_from(0_i32).expect_err("zero should not be convertible");
        let err_u32 = JsonNonZeroUInt::try_from(0_u32).expect_err("zero should not be convertible");
        let err_i64 = JsonNonZeroUInt::try_from(0_i64).expect_err("zero should not be convertible");
        let err_u64 = JsonNonZeroUInt::try_from(0_u64).expect_err("zero should not be convertible");
        assert_eq!(
            err_i32.to_string(),
            "attempt to convert a zero value into a JsonNonZeroUInt"
        );
        assert_eq!(
            err_u32.to_string(),
            "attempt to convert a zero value into a JsonNonZeroUInt"
        );
        assert_eq!(
            err_i64.to_string(),
            "attempt to convert a zero value into a JsonNonZeroUInt"
        );
        assert_eq!(
            err_u64.to_string(),
            "attempt to convert a zero value into a JsonNonZeroUInt"
        );
    }

    #[test]
    fn parse_int_from_empty() {
        let err = JsonInt::from_str("").expect_err("empty string is not valid");
        assert_eq!(
            err.to_string(),
            "string '' is not a valid representation of a JSON integer"
        );
    }

    #[test]
    fn parse_int_underflow() {
        let err = JsonInt::from_str("-9007199254740992").expect_err("out of range");
        assert_eq!(
            err.to_string(),
            "string '-9007199254740992' represents a value below the range of JsonInt values [-9007199254740991..9007199254740991]"
        );
    }

    #[test]
    fn parse_int_overflow() {
        let err = JsonInt::from_str("9007199254740992").expect_err("out of range");
        assert_eq!(
            err.to_string(),
            "string '9007199254740992' represents a value above the range of JsonInt values [-9007199254740991..9007199254740991]"
        );
    }

    #[test]
    fn parse_int_from_invalid_characters() {
        let err = JsonInt::from_str("42+7").expect_err("not a valid integer");
        assert_eq!(
            err.to_string(),
            "string '42+7' is not a valid representation of a JSON integer"
        );
    }

    #[test]
    fn parse_uint_from_empty() {
        let err = JsonUInt::from_str("").expect_err("empty string is not valid");
        assert_eq!(
            err.to_string(),
            "string '' is not a valid representation of a JSON integer"
        );
    }

    #[test]
    fn parse_uint_from_negative() {
        let err = JsonUInt::from_str("-42").expect_err("out of range");
        assert_eq!(
            err.to_string(),
            "string '-42' represents a value below the range of JsonUInt values [0..9007199254740991]"
        );
    }

    #[test]
    fn parse_uint_overflow() {
        let err = JsonUInt::from_str("9007199254740992").expect_err("out of range");
        assert_eq!(
            err.to_string(),
            "string '9007199254740992' represents a value above the range of JsonUInt values [0..9007199254740991]"
        );
    }

    #[test]
    fn parse_uint_from_invalid_characters() {
        let err = JsonUInt::from_str("42+7").expect_err("not a valid integer");
        assert_eq!(
            err.to_string(),
            "string '42+7' is not a valid representation of a JSON integer"
        );
    }

    #[test]
    fn parse_non_zero_uint_from_zero() {
        let err = JsonNonZeroUInt::from_str("0").expect_err("not a non-zero integer");
        assert_eq!(
            err.to_string(),
            "string '0' represents a zero value, which is not a valid JsonNonZeroUInt"
        )
    }

    mod proptests {
        use super::super::*;
        use proptest::prelude::*;

        proptest! {
            #[test]
            fn int_roundtrip(value in JSON_INT_LOWER_LIMIT..JSON_INT_UPPER_LIMIT) {
                let json_int = JsonInt::try_from(value).expect("within range");
                assert_eq!(json_int.as_i64(), value);
            }

            #[test]
            fn uint_roundtrip(value in 0..JSON_UINT_UPPER_LIMIT) {
                let json_uint = JsonUInt::try_from(value).expect("within range");
                assert_eq!(json_uint.as_u64(), value);
            }

            #[test]
            fn int_string_roundtrip(value in JSON_INT_LOWER_LIMIT..JSON_INT_UPPER_LIMIT) {
                let string = value.to_string();
                let json_int = JsonInt::from_str(&string).expect("valid string");
                assert_eq!(string, json_int.to_string())
            }

            #[test]
            fn uint_string_roundtrip(value in 0..JSON_UINT_UPPER_LIMIT) {
                let string = value.to_string();
                let json_int = JsonUInt::from_str(&string).expect("valid string");
                assert_eq!(string, json_int.to_string())
            }

            #[test]
            fn int_increment(value in JSON_INT_LOWER_LIMIT..(JSON_INT_UPPER_LIMIT - 1)) {
                let mut json_int = JsonInt::try_from(value).expect("within range");
                json_int.try_increment().expect("at most one below limit");
                assert_eq!(json_int.as_i64(), value + 1);
            }

            #[test]
            fn uint_increment(value in 0..(JSON_UINT_UPPER_LIMIT - 1)) {
                let mut json_uint = JsonUInt::try_from(value).expect("within range");
                json_uint.try_increment().expect("at most one below limit");
                assert_eq!(json_uint.as_u64(), value + 1);
            }
        }
    }
}
