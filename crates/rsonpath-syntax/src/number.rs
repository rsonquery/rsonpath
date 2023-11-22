//! JSON number types expressible in a JSONPath query.

use super::error::JsonFormatError;
use std::fmt::{self, Display, Formatter};

/// Unsigned interoperable JSON integer.
///
/// Provides an [IETF-conforming integer value](https://www.rfc-editor.org/rfc/rfc7493.html#section-2)
/// guaranteed to be non-negative. Values are \[0, (2<sup>53</sup>)-1].
/// # Examples
///
/// ```
/// # use rsonpath_syntax::number::JsonUInt;
///
/// let idx = JsonUInt::new(2);
///
/// assert_eq!(idx.as_u64(), 2);
/// ```
#[derive(Clone, Copy, PartialEq, Eq, Debug, PartialOrd, Ord)]
pub struct JsonUInt(u64);

/// The upper inclusive bound on JSON integers (2<sup>53</sup>-1).
const JSON_INT_ULIMIT: u64 = (1 << 53) - 1;

impl TryFrom<u64> for JsonUInt {
    type Error = JsonFormatError;

    #[inline]
    fn try_from(value: u64) -> Result<Self, JsonFormatError> {
        if value > JSON_INT_ULIMIT {
            Err(JsonFormatError::ExceedsUpperLimitError(value.to_string()))
        } else {
            Ok(Self::new(value))
        }
    }
}

impl JsonUInt {
    /// A constant index for the common and starting case of the first item.
    pub const ZERO: Self = Self::new(0);
    /// A constant index for the largest addressable index.
    pub const MAX: Self = Self::new(JSON_INT_ULIMIT);

    /// Create a new value from a [`u64`].
    #[must_use]
    const fn new(index: u64) -> Self {
        Self(index)
    }

    /// Create a new search index from a u64.
    /// # Errors
    /// Will return `Err` if the increment causes the [`JsonUInt`] to exceed the addressable IETF-conforming index value value.
    #[inline]
    pub fn try_increment(&mut self) -> Result<(), JsonFormatError> {
        let new_index = self.0 + 1;
        if new_index <= JSON_INT_ULIMIT {
            self.0 = new_index;
            Ok(())
        } else {
            Err(JsonFormatError::ExceedsUpperLimitError(new_index.to_string()))
        }
    }

    /// Return the value stored as a regular [`u64`].
    #[must_use]
    #[inline(always)]
    pub const fn as_u64(&self) -> u64 {
        self.0
    }
}

impl From<JsonUInt> for u64 {
    #[inline(always)]
    fn from(val: JsonUInt) -> Self {
        val.0
    }
}

impl Display for JsonUInt {
    #[inline]
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

#[cfg(feature = "arbitrary")]
#[cfg_attr(docsrs, doc(cfg(feature = "arbitrary")))]
impl<'a> arbitrary::Arbitrary<'a> for JsonUInt {
    #[inline]
    fn arbitrary(u: &mut arbitrary::Unstructured<'a>) -> arbitrary::Result<Self> {
        let val = u.int_in_range(0..=JSON_INT_ULIMIT)?;

        Ok(Self::new(val))
    }
}

#[cfg(test)]
mod tests {
    use super::{JsonUInt, JSON_INT_ULIMIT};

    #[test]
    fn index_ulimit_sanity_check() {
        assert_eq!(9_007_199_254_740_991, JSON_INT_ULIMIT);
    }

    #[test]
    fn index_ulimit_parse_check() {
        JsonUInt::try_from(JSON_INT_ULIMIT).expect("JSON int ulimit should be convertible.");

        JsonUInt::try_from(JSON_INT_ULIMIT + 1)
            .expect_err("Values in excess of JSON int ulimit should not be convertible.");
    }
}
