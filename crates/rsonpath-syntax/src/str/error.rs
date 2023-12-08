use std::fmt::Display;
use thiserror::Error;

#[derive(Debug, Error)]
pub struct JsonStringParseError<'a> {
    kind: JsonStringParseErrorKind,
    char_idx: usize,
    len: usize,
    input: &'a str,
}

impl<'a> JsonStringParseError<'a> {
    pub fn err_at(kind: JsonStringParseErrorKind, input: &'a str, char_idx: usize) -> Self {
        Self {
            char_idx,
            len: kind.diagnostic_len(),
            kind,
            input,
        }
    }

    pub fn err_at_with_len(kind: JsonStringParseErrorKind, input: &'a str, char_idx: usize, len: usize) -> Self {
        Self {
            kind,
            char_idx,
            len,
            input,
        }
    }
}

impl<'a> nom::error::ParseError<&'a str> for JsonStringParseError<'a> {
    fn from_error_kind(input: &'a str, kind: nom::error::ErrorKind) -> Self {
        Self {
            kind: JsonStringParseErrorKind::Unknown,
            char_idx: 0,
            len: 1,
            input,
        }
    }

    fn append(input: &'a str, kind: nom::error::ErrorKind, other: Self) -> Self {
        other
    }
}

#[derive(Debug)]
pub enum JsonStringParseErrorKind {
    InvalidUnescapedCharacter,
    InvalidEscapeSequence,
    InvalidUnicodeEscapeSequence,
    UnpairedHighSurrogate,
    UnpairedLowSurrogate,
    InvalidHexDigitInUnicodeEscape,
    Unknown,
}

impl JsonStringParseErrorKind {
    fn diagnostic_len(&self) -> usize {
        match self {
            Self::InvalidUnescapedCharacter => 1,
            Self::InvalidEscapeSequence => 2,
            Self::InvalidUnicodeEscapeSequence => 6,
            Self::UnpairedHighSurrogate => 6,
            Self::UnpairedLowSurrogate => 6,
            Self::InvalidHexDigitInUnicodeEscape => 1,
            Self::Unknown => 0,
        }
    }
}

impl Display for JsonStringParseErrorKind {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::InvalidUnescapedCharacter => write!(f, "this character must be escaped"),
            Self::InvalidEscapeSequence => write!(f, "not a valid escape sequence"),
            Self::InvalidUnicodeEscapeSequence => write!(f, "not a valid unicode escape sequence"),
            Self::UnpairedHighSurrogate => write!(f, "this high surrogate is unpaired"),
            Self::UnpairedLowSurrogate => write!(f, "this low surrogate is unpaired"),
            Self::InvalidHexDigitInUnicodeEscape => write!(f, "not a hex digit"),
            Self::Unknown => write!(f, "unknown error"),
        }
    }
}

impl<'a> Display for JsonStringParseError<'a> {
    #[inline]
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        use unicode_width::UnicodeWidthChar;
        let end_idx = self.char_idx + self.len - 1;
        writeln!(f, "{}", self.kind)?;
        writeln!(f)?;
        writeln!(f, "  {}", self.input)?;
        write!(f, "  ")?;
        for (i, c) in self.input.chars().enumerate() {
            if i < self.char_idx {
                let width = c.width().unwrap_or(0);
                for _ in 0..width {
                    write!(f, " ")?;
                }
            } else if i <= end_idx {
                let width = c.width().unwrap_or(0);
                for _ in 0..width {
                    write!(f, "^")?;
                }
            } else {
                break;
            }
        }
        writeln!(f)?;
        writeln!(f)?;
        if self.char_idx == end_idx {
            write!(f, "at position {end_idx}")
        } else {
            write!(f, "at positions {}-{end_idx}", self.char_idx)
        }
    }
}
