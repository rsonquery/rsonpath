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

use crate::num::error::{JsonFloatConvertError, JsonFloatParseError, JsonIntOverflowError, JsonIntParseError};
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
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
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
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
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
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct JsonNonZeroUInt(NonZeroU64);

/// IEEE 754 conformant floating-point number expressible in JSON.
///
/// These numbers behave as standard binary64 (double precision) numbers
/// restricted as in [the JSON specification](https://www.rfc-editor.org/rfc/rfc7159#section-6),
/// i.e. they cannot be NaN, +Inf, or -Inf.
///
/// These restrictions allow some "nice" properties - [`JsonFloat`] implements
/// [`Eq`] and [`Ord`], as well as [`Hash`](std::hash::Hash), and its binary representation
/// is the same as a regular [`f64`].
///
/// ## Integer conversions
///
/// Because of interoperability restrictions on [`JsonInt`], any [`JsonInt`] losslessly converts
/// to a [`JsonFloat`] and back. Therefore, [`JsonInt`] is [`Into<JsonFloat>`](`Into`), and
/// [`JsonFloat`] is [`TryInto<JsonInt>`], where the conversion succeeds if and only if
/// the float is an exactly representable integer in the range \[-2<sup>53</sup>+1, (2<sup>53</sup>)-1].
#[derive(Clone, Copy, Debug, PartialEq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct JsonFloat(f64);

// This is correct since the allowed values for `JsonFloat` don't include NaNs or infinities.
impl Eq for JsonFloat {}
impl PartialOrd for JsonFloat {
    #[inline(always)]
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}
impl Ord for JsonFloat {
    #[inline(always)]
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.0.partial_cmp(&other.0).expect("JsonFloat never NaN")
    }
}

impl std::hash::Hash for JsonFloat {
    #[inline(always)]
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        self.0.to_bits().hash(state);
    }
}

/// JSONPath numeric type - either a [`JsonInt`] or a [`JsonFloat`].
///
/// Note that this type is not normalized and an integer in the range
/// \[-2<sup>53</sup>+1, (2<sup>53</sup>)-1] can be represented both as
/// a [`JsonNumber::Int`] and as a [`JsonNumber::Float`].
///
/// Which type is produced when is a parser implementation detail.
/// If you need to rely on integers always being represented as [`JsonNumber::Int`]
/// you can use [`JsonNumber::normalize`], or manually inspect the underlying
/// [`JsonFloat`] using [`JsonFloat::is_int`] and its [`TryInto<JsonInt>`] conversion.
///
/// ## Examples
///
/// ```
/// # use rsonpath_syntax::num::{JsonNumber, JsonInt, JsonFloat};
///
/// let int = JsonInt::from(42);
/// let float = JsonFloat::try_from(42.01).unwrap();
///
/// let num_int = JsonNumber::from(int);
/// let num_float = JsonNumber::from(float);
///
/// assert_eq!(num_int, JsonNumber::Int(int));
/// assert_eq!(num_float, JsonNumber::Float(float));
/// assert_eq!("42", num_int.to_string());
/// assert_eq!("42.01", num_float.to_string());
/// ```
#[derive(Clone, Copy, Debug)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum JsonNumber {
    /// A [`JsonInt`] number.
    Int(JsonInt),
    /// A [`JsonFloat`] number.
    Float(JsonFloat),
}

impl PartialEq for JsonNumber {
    #[inline]
    fn eq(&self, other: &Self) -> bool {
        match (self.normalize(), other.normalize()) {
            (Self::Int(l0), Self::Int(r0)) => l0 == r0,
            (Self::Float(l0), Self::Float(r0)) => l0 == r0,
            _ => false,
        }
    }
}

impl Eq for JsonNumber {}

impl std::hash::Hash for JsonNumber {
    #[inline]
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        match self.normalize() {
            Self::Int(i) => (0, i).hash(state),
            Self::Float(f) => (1, f).hash(state),
        }
    }
}

impl PartialOrd for JsonNumber {
    #[inline(always)]
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for JsonNumber {
    #[inline]
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        match (self, other) {
            (Self::Int(i1), Self::Int(i2)) => i1.cmp(i2),
            (Self::Int(i), Self::Float(f)) => JsonFloat::from(*i).cmp(f),
            (Self::Float(f), Self::Int(i)) => f.cmp(&JsonFloat::from(*i)),
            (Self::Float(f1), Self::Float(f2)) => f1.cmp(f2),
        }
    }
}

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

impl JsonFloat {
    /// A constant value of zero. Equivalent to [`JsonFloat::default`](`Default::default`).
    ///
    /// # Examples
    /// ```
    /// # use rsonpath_syntax::num::JsonFloat;
    /// assert_eq!(JsonFloat::ZERO.as_f64(), 0.0);
    /// ```
    pub const ZERO: Self = Self(0.0);

    fn new(x: f64) -> Self {
        debug_assert!(x.is_finite());
        Self(x)
    }

    /// Return the value stored as a [`f64`].
    ///
    /// # Examples
    /// ```
    /// # use rsonpath_syntax::num::JsonFloat;
    /// let val = JsonFloat::try_from(3.14).unwrap();
    /// assert_eq!(val.as_f64(), 3.14);
    /// ```
    #[inline]
    #[must_use]
    pub fn as_f64(&self) -> f64 {
        self.0
    }

    /// Check if this float is an equivalent of some [`JsonInt`].
    ///
    /// The range of valid [`JsonInt`] is exactly representable as [`JsonFloat`] values.
    /// This function returns true if the float is one of those valid values, i.e. an
    /// integer and in the [`JsonInt`] bounds.
    ///
    /// ## Examples
    /// ```
    /// # use rsonpath_syntax::num::JsonFloat;
    ///
    /// let f1 = JsonFloat::try_from(3.0).unwrap();
    /// let f2 = JsonFloat::try_from(3.14).unwrap();
    /// let f3 = JsonFloat::try_from(1e54).unwrap();
    ///
    /// assert!(f1.is_int());
    /// assert!(!f2.is_int());
    /// assert!(!f3.is_int());
    /// ```
    #[inline]
    #[must_use]
    pub fn is_int(&self) -> bool {
        JsonInt::try_from(*self).is_ok()
    }
}

impl JsonNumber {
    /// A constant value of zero, as an integer.
    /// Equivalent to [`JsonNumber::default`](`Default::default`) and
    /// [`JsonNumber::from(JsonInt::ZERO)`](`JsonNumber::Int`).
    ///
    /// # Examples
    /// ```
    /// # use rsonpath_syntax::num::{JsonNumber, JsonInt};
    /// assert_eq!(JsonNumber::ZERO, JsonNumber::Int(JsonInt::ZERO));
    /// ```
    pub const ZERO: Self = Self::Int(JsonInt(0));

    /// Normalize a [`JsonNumber`] so that valid [`JsonInt`] value is represented
    /// by [`JsonNumber::Int`].
    ///
    /// The parser is allowed to represent a normal JSON integer (e.g. 17) as an
    /// equivalent JSON float (17.0). Calling `normalize` ensures all values
    /// representable by a [`JsonInt`] are indeed represented as such.
    ///
    /// ## Examples
    ///
    /// ```
    /// # use rsonpath_syntax::num::{JsonNumber, JsonInt, JsonFloat};
    ///
    /// // Creating a JsonNumber from a JsonFloat always gives JsonNumber::Float.
    /// let f1 = JsonFloat::try_from(17.0).unwrap();
    /// let nf1 = JsonNumber::from(f1);
    /// assert_eq!(nf1, JsonNumber::Float(f1));
    /// // Normalizing will give us an integer representation, when possible.
    /// assert_eq!(nf1.normalize(), JsonNumber::Int(17.into()));
    ///
    /// // If the float is an integer within range normalization will succeed.
    /// let f2 = JsonFloat::try_from(1e15).unwrap();
    /// let nf2 = JsonNumber::from(f2);
    /// assert_eq!(nf2, JsonNumber::Float(f2));
    /// assert_eq!(nf2.normalize(), JsonNumber::Int(1_000_000_000_000_000_i64.try_into().unwrap()));
    ///
    /// // An int is an int, and remains so under normalization.
    /// let i1 = JsonInt::from(42);
    /// let ni1 = JsonNumber::from(i1);
    /// assert_eq!(ni1, JsonNumber::Int(i1));
    /// assert_eq!(ni1.normalize(), JsonNumber::Int(i1));
    ///
    /// // A float that is not an int remains the same when normalized.
    /// let f3 = JsonFloat::try_from(3.14).unwrap();
    /// let nf3 = JsonNumber::from(f3);
    /// assert_eq!(nf3, JsonNumber::Float(f3));
    /// assert_eq!(nf3.normalize(), JsonNumber::Float(f3));
    ///
    /// // A float that is an int, but outside of the interoperable JsonInt range,
    /// // is not normalized.
    /// let f4 = JsonFloat::try_from(1e120).unwrap();
    /// let nf4 = JsonNumber::from(f4);
    /// assert_eq!(nf4, JsonNumber::Float(f4));
    /// assert_eq!(nf4.normalize(), JsonNumber::Float(f4));
    /// ```
    #[inline]
    #[must_use]
    pub fn normalize(&self) -> Self {
        match *self {
            Self::Int(x) => Self::Int(x),
            Self::Float(x) => match JsonInt::try_from(x) {
                Ok(int) => Self::Int(int),
                Err(_) => Self::Float(x),
            },
        }
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

impl From<JsonUInt> for i64 {
    #[inline(always)]
    fn from(value: JsonUInt) -> Self {
        // Safe cast since JsonUInt::MAX is lower than i64::MAX.
        value.0 as Self
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

impl TryFrom<JsonFloat> for JsonInt {
    type Error = JsonIntOverflowError;

    #[inline]
    fn try_from(value: JsonFloat) -> Result<Self, Self::Error> {
        if value.0.fract() != 0.0 {
            return Err(JsonIntOverflowError::fractional(value.0));
        }
        // At this point the fractional part must be 0.0, so the value is *an* integer.
        // We need to check that it is within bounds of JsonInt. This is correct
        // only because JsonInt bounds are guaranteed to be interoperable with f64,
        // so every value within is exactly representable as a f64.
        let int_value = value.0.trunc();
        if int_value < JSON_INT_LOWER_LIMIT as f64 {
            return Err(JsonIntOverflowError::int_float_neg_overflow(value.0));
        }
        if int_value > JSON_INT_UPPER_LIMIT as f64 {
            return Err(JsonIntOverflowError::int_float_pos_overflow(value.0));
        }

        // This conversion is now guaranteed to be lossless.
        Ok(Self(int_value as i64))
    }
}

impl From<JsonInt> for JsonFloat {
    #[inline]
    fn from(value: JsonInt) -> Self {
        Self::new(value.0 as f64)
    }
}

impl TryFrom<f32> for JsonFloat {
    type Error = JsonFloatConvertError;

    #[inline]
    fn try_from(value: f32) -> Result<Self, Self::Error> {
        if value.is_finite() {
            Ok(Self::new(f64::from(value)))
        } else {
            Err(JsonFloatConvertError::infinite_or_nan(f64::from(value)))
        }
    }
}

impl TryFrom<f64> for JsonFloat {
    type Error = JsonFloatConvertError;

    #[inline]
    fn try_from(value: f64) -> Result<Self, Self::Error> {
        if value.is_finite() {
            Ok(Self::new(value))
        } else {
            Err(JsonFloatConvertError::infinite_or_nan(value))
        }
    }
}

impl FromStr for JsonFloat {
    type Err = JsonFloatParseError;

    /* Fact #1: parsing floats is hard.
    * Fact #2: grammar accepted by `f64::from_str` is slightly different from the
    *          JSONPath grammar for floats.
    * Fact #3: I have little interest in rewriting the close to 2,000 lines of code
    *          of Rust's `dec2flt` to incorporate those differences.
    *
    * The grammars accepted by Rust and JSON are, respectively:
      ; Rust f64::from_str
      Float  ::= Sign? ( 'inf' | 'infinity' | 'nan' | Number )
      Number ::= ( Digit+ |
                   Digit+ '.' Digit* |
                   Digit* '.' Digit+ ) Exp?
      Exp    ::= 'e' Sign? Digit+
      Sign   ::= [+-]
      Digit  ::= [0-9]

      ; JSON
      Number ::= (Int | "-0") Frac? Exp?
      Int    ::= "0" |
                 ("-"? Digit1 Digit*)
      Frac   ::= "." Digit+
      Exp    ::= "e" Sign? Digit+
      Sign   ::= [+-]
      Digit  ::= [0-9]
      Digit1 ::= [1-9]

    * Here are all the differences:
    * 1) 'inf', 'infinity', and 'nan' are acceptable only in Rust.
    * 2) Rust allows an explicit leading `+`, JSON does not.
    * 3) Rust allows an empty integer part, e.g. '.14' as equivalent to '0.14'; JSON does not.
    * 4) Rust allows an empty decimal part, e.g. '3.' as equivalent to '3.0'; JSON does not.
    * 5) Leading zeroes of the integral part are accepted only in Rust.
    *
    * Both accept the exponent char as either lower or uppercase.
    * Since Rust's grammar is more general than JSON (L(JSON) \subset L(Rust))
    * we can use Rust's parser and enforce stricter JSON rules independently.
    *
    * To satisfy all restrictions, we parse with Rust first and then:
    * - enforce the result is not Inf or NaN (rule 1);
    * - enforce the string does not begin with '+' (rule 2);
    * - check if the decimal period exists (Rust guarantees there is at most one),
    *   and enforce it is both preceded and followed by a digit (rules 3 and 4);
    * - enforce there are no leading zeroes (rule 5).
    *
    * Performance-wise this is not ideal - we're effectively inspecting the string twice.
    * But without access into the `f64::from_str` black-box the only solution would be
    * to rewrite the routine here and add the restrictions, and we rejected that at the start.
    * If Rust ever exposes an API to create an f64 out of the mantissa and exponent then it might
    * be possible - the hardest bits of the parsing routine happen after these are actually extracted
    * from the string. See: https://github.com/rust-lang/rust/blob/master/library/core/src/num/dec2flt/mod.rs
    */

    #[inline]
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match f64::from_str(s) {
            Ok(x) => {
                assert!(!s.is_empty()); // Empty strings are not accepted by f64::from_str.
                                        // Rule 1.
                if x.is_nan() || x.is_infinite() {
                    return Err(Self::Err::infinite_or_nan(s));
                }
                if let Some((before, after)) = s.split_once('.') {
                    // Rule 3. The case `before == "+"` is checked later.
                    if before.is_empty() || before == "-" {
                        return Err(Self::Err::nothing_before_decimal_point(s));
                    }
                    // Rule 4.
                    if after.is_empty() || after.starts_with(['e', 'E']) {
                        return Err(Self::Err::nothing_after_decimal_point(s));
                    }
                }
                let mut chars = s.chars();
                let first_c = chars.next().expect("s is non-empty");
                // Rule 2.
                if first_c == '+' {
                    return Err(Self::Err::leading_plus_sign(s));
                }
                // Skip the leading minus if it exists.
                let s_no_sign = if first_c == '-' { chars.as_str() } else { s };
                // Rule 5.
                // Check for leading zeroes. We strip the first zero from the front and check what's left.
                // The only acceptable case is that the next character is not a digit.
                if let Some(rest) = s_no_sign.strip_prefix('0') {
                    if matches!(rest.chars().next(), Some('0'..='9')) {
                        return Err(Self::Err::leading_zeros(s));
                    }
                }
                Ok(Self(x))
            }
            // Remember that all floats valid in JSON are also accepted by Rust,
            // so this is *definitely* not a valid JSON float.
            Err(_) => Err(Self::Err::f64_parse_error(s)),
        }
    }
}

impl From<JsonInt> for JsonNumber {
    #[inline]
    fn from(value: JsonInt) -> Self {
        Self::Int(value)
    }
}

impl From<JsonFloat> for JsonNumber {
    #[inline]
    fn from(value: JsonFloat) -> Self {
        Self::Float(value)
    }
}

// Not the smartest implementation, but a working one.
// Every valid JsonInt is a valid JsonFloat, so parse a JsonFloat first and then try to canonicalize
// the JsonNumber.
impl FromStr for JsonNumber {
    type Err = JsonFloatParseError;

    #[inline]
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(Self::Float(JsonFloat::from_str(s)?).normalize())
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

impl Display for JsonFloat {
    #[inline]
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl Display for JsonNumber {
    #[inline]
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        match self {
            Self::Int(int) => int.fmt(f),
            Self::Float(flt) => flt.fmt(f),
        }
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

    #[test]
    fn convert_large_float_to_int() {
        let float = JsonFloat::try_from(1e15).unwrap();
        let int = JsonInt::try_from(float).expect("should succeed");
        assert_eq!(int.as_i64(), 1_000_000_000_000_000);
    }

    mod json_float_parse {
        use super::*;
        use pretty_assertions::assert_eq;
        use test_case::test_case;

        #[allow(clippy::approx_constant)] // Detects 3.14 as PI, that's not we want for tests.
        #[test_case("0.0", 0.0; "0d0")]
        #[test_case("0.0e+000000000000000000000", 0.0; "0d0ep000000000000000000000")]
        #[test_case("0.0E+000000000000000000000", 0.0; "0d0Uep000000000000000000000")]
        #[test_case("-0.0", -0.0; "m0d0")]
        #[test_case("3.14", 3.14; "3d142")]
        #[test_case("-3.14", -3.14; "m3d142")]
        #[test_case("3.14159265358979323846264338327950288", std::f64::consts::PI; "pi")]
        #[test_case("-3.00000000000000000000000000000000000000000000000", -3.0; "m3d00000000000000000000000000000000000000000000000")]
        #[test_case("-3.14e53", -3.14e53; "m3d14e53")]
        #[test_case("-3.14e+53", -3.14e53; "m3d14ep53")]
        #[test_case("-3.14e-53", -3.14e-53; "m3d14em53")]
        #[test_case("-3.14e-153", -3.14e-153; "m3d14em153")]
        #[test_case("42", 42.0; "42")]
        fn valid_float_string(str: &str, expected: f64) {
            let float = JsonFloat::from_str(str).expect("should parse");
            assert_eq!(float.as_f64(), expected);
        }

        #[test_case("abc")]
        #[test_case("0xFF")]
        #[test_case("3,14")]
        #[test_case("3.14F-20")]
        #[test_case("3.3.3")]
        #[test_case(".")]
        #[test_case(".e30"; "de30")]
        #[test_case("e30")]
        fn invalid_float_strings_that_even_rust_rejects(str: &str) {
            let err = JsonFloat::from_str(str).expect_err("should not parse");
            let expected = format!("string '{str}' is not a valid representation of a float");
            assert_eq!(err.to_string(), expected);
        }

        #[test_case("nan"; "nan lowercase")]
        #[test_case("NaN"; "NaN mixed case")]
        #[test_case("NAN"; "NAN uppercase")]
        #[test_case("-nan"; "minus nan lowercase")]
        #[test_case("-NaN"; "minus nan mixed case")]
        #[test_case("-NAN"; "minus nan uppercase")]
        #[test_case("inf"; "inf")]
        #[test_case("Inf"; "inf mixed case")]
        #[test_case("INF"; "inf uppercase")]
        #[test_case("-inf"; "minus inf")]
        #[test_case("-Inf"; "minus inf mixed case")]
        #[test_case("-INF"; "minus inf uppercase")]
        #[test_case("infinity"; "infinity mixed case")]
        #[test_case("Infinity"; "infinity")]
        #[test_case("INFINITY"; "infinity uppercase")]
        #[test_case("-infinity"; "minus infinity")]
        #[test_case("-Infinity"; "minus infinity mixed case")]
        #[test_case("-INFINITY"; "minus infinity uppercase")]
        fn invalid_float_strings_infinity_or_nan(str: &str) {
            let err = JsonFloat::from_str(str).expect_err("should not parse");
            let expected = format!("string '{str}' is not a valid JsonFloat as it is not a finite number");
            assert_eq!(err.to_string(), expected);
        }

        #[test_case(".14"; "d14")]
        #[test_case("-.14"; "md14")]
        #[test_case(".0")]
        #[test_case(".14e53")]
        #[test_case(".00000e53")]
        fn invalid_float_strings_nothing_before_decimal_point(str: &str) {
            let err = JsonFloat::from_str(str).expect_err("should not parse");
            let expected = format!("missing digits before the decimal point in '{str}'");
            assert_eq!(err.to_string(), expected);
        }

        #[test_case("14."; "14d")]
        #[test_case("-14."; "m14d")]
        #[test_case("-0.")]
        #[test_case("14.e53")]
        #[test_case("0.e53")]
        fn invalid_float_strings_nothing_after_decimal_point(str: &str) {
            let err = JsonFloat::from_str(str).expect_err("should not parse");
            let expected = format!("missing digits after the decimal point in '{str}'");
            assert_eq!(err.to_string(), expected);
        }

        #[test_case("+3.14")]
        #[test_case("+3.14e53")]
        fn invalid_float_strings_leading_plus_sign(str: &str) {
            let err = JsonFloat::from_str(str).expect_err("should not parse");
            let expected = format!("string '{str}' includes a leading plus sign");
            assert_eq!(err.to_string(), expected);
        }

        #[test_case("00.0"; "00d0")]
        #[test_case("-00.0"; "m00d0")]
        #[test_case("00"; "00")]
        #[test_case("00000000000")]
        #[test_case("-00"; "m00")]
        #[test_case("-00000000000"; "m00000000000")]
        #[test_case("03.14"; "03d14")]
        #[test_case("-03.14"; "m03d14")]
        #[test_case("03e14"; "03e14")]
        #[test_case("-03e14"; "m03e14")]
        #[test_case("00e14"; "00e14")]
        #[test_case("-00e14"; "m00e14")]
        fn invalid_float_strings_leading_zeros(str: &str) {
            let err = JsonFloat::from_str(str).expect_err("should not parse");
            let expected = format!("string '{str}' includes leading zeros");
            assert_eq!(err.to_string(), expected);
        }
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
