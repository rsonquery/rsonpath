//! Error types for arithmetic, conversion, and parse operations
//! on [`JsonInt`] and [`JsonUInt`].

use crate::num::{JsonInt, JsonUInt};
use std::{
    fmt::{self, Display},
    num::IntErrorKind,
};
use thiserror::Error;

/// Errors raised when trying to convert between JSON integer types
/// or between a JSON int and a regular Rust int, or when performing
/// arithmetic on JSON ints that would over-/underflow.
#[derive(Debug, PartialEq, Error, Clone)]
pub struct JsonIntOverflowError {
    kind: JsonIntOverflowKind,
}

/// Errors raised when trying to parse JSON integer types from strings.
#[derive(Debug, PartialEq, Eq, Error, Clone)]
pub struct JsonIntParseError {
    kind: JsonIntParseErrorKind,
}

/// Errors raised when trying to convert between [`JsonFloat`](crate::num::JsonFloat) and
/// regular Rust floating point numbers.
#[derive(Debug, PartialEq, Error, Clone)]
pub struct JsonFloatConvertError {
    kind: JsonFloatConvertErrorKind,
}

/// Errors raised when trying to parse [`JsonFloat`](crate::num::JsonFloat) values from strings.
#[derive(Debug, PartialEq, Eq, Error, Clone)]
pub struct JsonFloatParseError {
    kind: JsonFloatParseErrorKind,
}

impl JsonIntOverflowError {
    pub(crate) fn int_pos_overflow(src: i64) -> Self {
        Self {
            kind: JsonIntOverflowKind::IntPos(src),
        }
    }

    pub(crate) fn int_pos_overflow_u(src: u64) -> Self {
        Self {
            kind: JsonIntOverflowKind::IntPosU(src),
        }
    }

    pub(crate) fn int_neg_overflow(src: i64) -> Self {
        Self {
            kind: JsonIntOverflowKind::IntNeg(src),
        }
    }

    pub(crate) fn uint_pos_overflow(src: u64) -> Self {
        Self {
            kind: JsonIntOverflowKind::UIntPos(src),
        }
    }

    pub(crate) fn negative_uint(src: i64) -> Self {
        Self {
            kind: JsonIntOverflowKind::UIntNeg(src),
        }
    }

    pub(crate) fn zero_non_zero_uint() -> Self {
        Self {
            kind: JsonIntOverflowKind::NonZeroUIntZero,
        }
    }

    pub(crate) fn fractional(src: f64) -> Self {
        Self {
            kind: JsonIntOverflowKind::Fractional(src),
        }
    }

    pub(crate) fn int_float_pos_overflow(src: f64) -> Self {
        Self {
            kind: JsonIntOverflowKind::FloatPos(src),
        }
    }

    pub(crate) fn int_float_neg_overflow(src: f64) -> Self {
        Self {
            kind: JsonIntOverflowKind::FloatNeg(src),
        }
    }
}

impl JsonIntParseError {
    pub(crate) fn int_parse_error(src: &str, err: &IntErrorKind) -> Self {
        Self {
            kind: match err {
                IntErrorKind::PosOverflow => JsonIntParseErrorKind::IntPosOverflow(src.to_string()),
                IntErrorKind::NegOverflow => JsonIntParseErrorKind::IntNegOverflow(src.to_string()),
                IntErrorKind::Zero => unreachable!(), // Zero is always a valid JsonInt value.
                _ => JsonIntParseErrorKind::InvalidFormat(src.to_string()),
            },
        }
    }

    pub(crate) fn parse_conversion_err(src: &str, err: &JsonIntOverflowError) -> Self {
        Self {
            kind: match err.kind {
                JsonIntOverflowKind::IntPosU(_) | JsonIntOverflowKind::IntPos(_) => {
                    JsonIntParseErrorKind::IntPosOverflow(src.to_string())
                }
                JsonIntOverflowKind::IntNeg(_) => JsonIntParseErrorKind::IntNegOverflow(src.to_string()),
                JsonIntOverflowKind::UIntPos(_) => JsonIntParseErrorKind::UIntPosOverflow(src.to_string()),
                JsonIntOverflowKind::UIntNeg(_) => JsonIntParseErrorKind::UIntNegOverflow(src.to_string()),
                JsonIntOverflowKind::NonZeroUIntZero => JsonIntParseErrorKind::NonZeroUIntZero(src.to_string()),
                JsonIntOverflowKind::Fractional(_)
                | JsonIntOverflowKind::FloatPos(_)
                | JsonIntOverflowKind::FloatNeg(_) => JsonIntParseErrorKind::InvalidFormat(src.to_string()),
            },
        }
    }

    pub(crate) fn uint_parse_error(src: &str, err: &IntErrorKind) -> Self {
        Self {
            kind: match err {
                IntErrorKind::PosOverflow => JsonIntParseErrorKind::UIntPosOverflow(src.to_string()),
                IntErrorKind::NegOverflow => JsonIntParseErrorKind::UIntNegOverflow(src.to_string()),
                IntErrorKind::Zero => unreachable!(), // Zero is always a valid JsonUInt value.
                _ => JsonIntParseErrorKind::InvalidFormat(src.to_string()),
            },
        }
    }

    pub(crate) fn non_zero_uint_parse_error(src: &str, err: &IntErrorKind) -> Self {
        Self {
            kind: match err {
                IntErrorKind::PosOverflow => JsonIntParseErrorKind::UIntPosOverflow(src.to_string()),
                IntErrorKind::NegOverflow => JsonIntParseErrorKind::UIntNegOverflow(src.to_string()),
                IntErrorKind::Zero => JsonIntParseErrorKind::NonZeroUIntZero(src.to_string()),
                _ => JsonIntParseErrorKind::InvalidFormat(src.to_string()),
            },
        }
    }
}

impl JsonFloatConvertError {
    pub(crate) fn infinite_or_nan(src: f64) -> Self {
        Self {
            kind: JsonFloatConvertErrorKind::InfinityOrNaN(src),
        }
    }
}

impl JsonFloatParseError {
    pub(crate) fn infinite_or_nan(src: &str) -> Self {
        Self {
            kind: JsonFloatParseErrorKind::InfinityOrNaN(src.to_string()),
        }
    }

    pub(crate) fn leading_plus_sign(src: &str) -> Self {
        Self {
            kind: JsonFloatParseErrorKind::LeadingPlusSign(src.to_string()),
        }
    }

    pub(crate) fn leading_zeros(src: &str) -> Self {
        Self {
            kind: JsonFloatParseErrorKind::LeadingZeros(src.to_string()),
        }
    }

    pub(crate) fn nothing_after_decimal_point(src: &str) -> Self {
        Self {
            kind: JsonFloatParseErrorKind::NothingAfterDecimalPoint(src.to_string()),
        }
    }

    pub(crate) fn nothing_before_decimal_point(src: &str) -> Self {
        Self {
            kind: JsonFloatParseErrorKind::NothingBeforeDecimalPoint(src.to_string()),
        }
    }

    pub(crate) fn f64_parse_error(src: &str) -> Self {
        Self {
            kind: JsonFloatParseErrorKind::InvalidFormat(src.to_string()),
        }
    }
}

#[derive(Debug, PartialEq, Clone)]
enum JsonIntOverflowKind {
    IntPos(i64),
    IntPosU(u64),
    IntNeg(i64),
    UIntPos(u64),
    UIntNeg(i64),
    NonZeroUIntZero,
    Fractional(f64),
    FloatPos(f64),
    FloatNeg(f64),
}

#[derive(Debug, PartialEq, Eq, Clone)]
enum JsonIntParseErrorKind {
    IntPosOverflow(String),
    IntNegOverflow(String),
    UIntPosOverflow(String),
    UIntNegOverflow(String),
    NonZeroUIntZero(String),
    InvalidFormat(String),
}

#[derive(Debug, PartialEq, Clone)]
enum JsonFloatConvertErrorKind {
    InfinityOrNaN(f64),
}

#[derive(Debug, PartialEq, Eq, Clone)]
enum JsonFloatParseErrorKind {
    InfinityOrNaN(String),
    NothingBeforeDecimalPoint(String),
    NothingAfterDecimalPoint(String),
    LeadingPlusSign(String),
    LeadingZeros(String),
    InvalidFormat(String),
}

impl Display for JsonIntOverflowError {
    #[inline(always)]
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        self.kind.fmt(f)
    }
}

impl Display for JsonIntParseError {
    #[inline(always)]
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        self.kind.fmt(f)
    }
}

impl Display for JsonFloatConvertError {
    #[inline(always)]
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        self.kind.fmt(f)
    }
}

impl Display for JsonFloatParseError {
    #[inline(always)]
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        self.kind.fmt(f)
    }
}

impl Display for JsonIntOverflowKind {
    #[inline]
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::IntPos(src) => write!(
                f,
                "value {src} is above the range of JsonInt values [{}..{}]",
                JsonInt::MIN,
                JsonInt::MAX
            ),
            Self::IntPosU(src) => write!(
                f,
                "value {src} is above the range of JsonInt values [{}..{}]",
                JsonInt::MIN,
                JsonInt::MAX
            ),
            Self::IntNeg(src) => write!(
                f,
                "value {src} is below the range of JsonInt values [{}..{}]",
                JsonInt::MIN,
                JsonInt::MAX
            ),
            Self::UIntPos(src) => write!(
                f,
                "value {src} is above the range of JsonUInt values [0..{}]",
                JsonUInt::MAX
            ),
            Self::UIntNeg(src) => {
                write!(f, "attempt to convert a negative value {src} into a JsonUInt",)
            }
            Self::NonZeroUIntZero => {
                write!(f, "attempt to convert a zero value into a JsonNonZeroUInt",)
            }
            Self::Fractional(src) => write!(f, "attempt to convert a fractional value {src} into a JsonInt"),
            Self::FloatPos(src) => write!(
                f,
                "floating-point value {src} is above the range of JsonInt values [{}..{}]",
                JsonInt::MIN,
                JsonInt::MAX
            ),
            Self::FloatNeg(src) => write!(
                f,
                "floating-point value {src} is below the range of JsonInt values [{}..{}]",
                JsonInt::MIN,
                JsonInt::MAX
            ),
        }
    }
}

impl Display for JsonIntParseErrorKind {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::IntPosOverflow(src) => write!(
                f,
                "string '{src}' represents a value above the range of JsonInt values [{}..{}]",
                JsonInt::MIN,
                JsonInt::MAX
            ),
            Self::IntNegOverflow(src) => write!(
                f,
                "string '{src}' represents a value below the range of JsonInt values [{}..{}]",
                JsonInt::MIN,
                JsonInt::MAX
            ),
            Self::UIntPosOverflow(src) => write!(
                f,
                "string '{src}' represents a value above the range of JsonUInt values [0..{}]",
                JsonUInt::MAX
            ),
            Self::UIntNegOverflow(src) => {
                write!(
                    f,
                    "string '{src}' represents a value below the range of JsonUInt values [0..{}]",
                    JsonUInt::MAX
                )
            }
            Self::NonZeroUIntZero(src) => write!(
                f,
                "string '{src}' represents a zero value, which is not a valid JsonNonZeroUInt"
            ),
            Self::InvalidFormat(src) => write!(f, "string '{src}' is not a valid representation of a JSON integer"),
        }
    }
}

impl Display for JsonFloatConvertErrorKind {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::InfinityOrNaN(src) => write!(f, "cannot convert from a non-finite float {src}"),
        }
    }
}

impl Display for JsonFloatParseErrorKind {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::InfinityOrNaN(src) => {
                write!(
                    f,
                    "string '{src}' is not a valid JsonFloat as it is not a finite number"
                )
            }
            Self::NothingBeforeDecimalPoint(src) => write!(f, "missing digits before the decimal point in '{src}'"),
            Self::NothingAfterDecimalPoint(src) => write!(f, "missing digits after the decimal point in '{src}'"),
            Self::LeadingPlusSign(src) => write!(f, "string '{src}' includes a leading plus sign"),
            Self::LeadingZeros(src) => write!(f, "string '{src}' includes leading zeros"),
            Self::InvalidFormat(src) => write!(f, "string '{src}' is not a valid representation of a float"),
        }
    }
}
