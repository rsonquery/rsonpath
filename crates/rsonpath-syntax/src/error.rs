//! Error types for the crate.
//!
//! The main error type is [`ParseError`], which contains
//! all syntax errors encountered during parsing.
use crate::{
    num::error::JsonIntParseError,
    str::{self, EscapeMode},
};
#[cfg(feature = "color")]
use std::error::Error;
use std::fmt::{self, Display};
use thiserror::Error;

#[cfg(feature = "color")]
use colored::OwoColorsErrorStyle as ErrorStyleImpl;
#[cfg(not(feature = "color"))]
use plain::PlainErrorStyle as ErrorStyleImpl;

#[derive(Debug)]
pub(crate) struct ParseErrorBuilder {
    syntax_errors: Vec<SyntaxError>,
}

/// Errors raised by the query parser.
#[derive(Debug, Error)]
pub struct ParseError {
    input: String,
    syntax_errors: Vec<SyntaxError>,
}

impl ParseErrorBuilder {
    pub(crate) fn new() -> Self {
        Self { syntax_errors: vec![] }
    }

    pub(crate) fn add(&mut self, syntax_error: SyntaxError) {
        self.syntax_errors.push(syntax_error)
    }

    pub(crate) fn add_many(&mut self, mut syntax_errors: Vec<SyntaxError>) {
        self.syntax_errors.append(&mut syntax_errors)
    }

    pub(crate) fn is_empty(&self) -> bool {
        self.syntax_errors.is_empty()
    }

    pub(crate) fn build(self, str: String) -> ParseError {
        ParseError {
            input: str,
            syntax_errors: self.syntax_errors,
        }
    }
}

impl Display for ParseError {
    #[inline]
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        fmt_parse_error(self, &ErrorStyleImpl::empty(), f)
    }
}

#[cfg(feature = "color")]
impl ParseError {
    /// Turn the error into a version with colored display.
    #[inline(always)]
    #[must_use]
    #[cfg_attr(docsrs, doc(cfg(feature = "color")))]
    pub fn colored(self) -> impl Error {
        colored::ColoredParseError(self)
    }
}

#[inline(always)]
fn fmt_parse_error(error: &ParseError, style: &ErrorStyleImpl, f: &mut fmt::Formatter<'_>) -> fmt::Result {
    let mut suggestion = Suggestion::new();
    for syntax_error in &error.syntax_errors {
        writeln!(
            f,
            "{}",
            syntax_error.display(&error.input, &mut suggestion, style.clone())
        )?;
    }

    if let Some(suggestion) = suggestion.build(&error.input) {
        writeln!(
            f,
            "{} did you mean `{}` ?",
            style.note_prefix(&"suggestion:"),
            style.suggestion(&suggestion)
        )?;
    }

    Ok(())
}

#[derive(Debug, PartialEq, Eq, Clone)]
pub(crate) struct SyntaxError {
    /// Kind of the error.
    kind: SyntaxErrorKind,
    /// The byte index at which the error occurred, counting from the end of the input.
    rev_idx: usize,
    /// The number of characters that the parser recognized as invalid.
    len: usize,
}

#[derive(Debug, PartialEq, Eq, Clone)]
pub(crate) enum SyntaxErrorKind {
    DisallowedLeadingWhitespace,
    DisallowedTrailingWhitespace,
    InvalidUnescapedCharacter,
    InvalidEscapeSequence,
    UnpairedHighSurrogate,
    UnpairedLowSurrogate,
    InvalidHexDigitInUnicodeEscape,
    MissingClosingSingleQuote,
    MissingClosingDoubleQuote,
    MissingRootIdentifier,
    InvalidSegmentStart,
    InvalidSegmentAfterTwoPeriods,
    InvalidNameShorthandAfterOnePeriod,
    InvalidSelector,
    EmptySelector,
    MissingSelectorSeparator,
    MissingClosingBracket,
    NegativeZeroInteger,
    LeadingZeros,
    IndexParseError(JsonIntParseError),
    SliceStartParseError(JsonIntParseError),
    SliceEndParseError(JsonIntParseError),
    SliceStepParseError(JsonIntParseError),
}

impl SyntaxError {
    pub(crate) fn new(kind: SyntaxErrorKind, rev_idx: usize, len: usize) -> Self {
        Self { kind, rev_idx, len }
    }
    /*
        This creates friendly displayable errors.
        Every error displays the entire query up to the point of the error.
        An error consists of
        - The toplevel error name/message.
        - A list of lines of the input, each with an optional underline message.
        - A list of notes/suggestions at the end.
    */
    fn display(&self, input: &str, suggestion: &mut Suggestion, style: ErrorStyleImpl) -> DisplayableSyntaxError {
        let start_idx = input.len() - self.rev_idx;
        let end_idx = start_idx + self.len - 1;
        let mut builder = DisplayableSyntaxErrorBuilder::new();

        for (i, c) in input.char_indices() {
            let width = tweaked_width(c);
            if i < start_idx {
                builder.add_non_underline(width);
            } else if i <= end_idx {
                builder.add_underline(width);
            }
            builder.add_char(c);
        }
        if end_idx >= input.len() {
            builder.add_underline(1);
        }
        builder.add_underline_message(self.kind.underline_message());

        self.generate_notes(&mut builder, suggestion, input);

        return builder.finish(self.kind.toplevel_message(), start_idx, end_idx, style);

        fn tweaked_width(c: char) -> usize {
            use unicode_width::UnicodeWidthChar;
            match c {
                '\t' => 4,
                _ => c.width().unwrap_or(0),
            }
        }
    }

    fn generate_notes(&self, builder: &mut DisplayableSyntaxErrorBuilder, suggestion: &mut Suggestion, input: &str) {
        let start_idx = input.len() - self.rev_idx;
        let end_idx = start_idx + self.len - 1;
        let (prefix, error, suffix) = self.split_error(input);
        // Kind-specific notes and suggestion building.
        match self.kind {
            SyntaxErrorKind::DisallowedLeadingWhitespace | SyntaxErrorKind::DisallowedTrailingWhitespace => {
                suggestion.remove(start_idx, error.len());
            }
            SyntaxErrorKind::InvalidUnescapedCharacter => {
                if error == "\"" {
                    suggestion.replace(start_idx, 1, r#"\""#);
                } else if error == "'" {
                    suggestion.replace(start_idx, 1, r"\'");
                } else {
                    let escaped = str::escape(error, EscapeMode::DoubleQuoted);
                    suggestion.replace(start_idx, error.len(), escaped);
                }
            }
            SyntaxErrorKind::InvalidEscapeSequence => {
                if error == r"\U" && suffix.len() >= 4 && suffix[..4].chars().all(|x| x.is_ascii_hexdigit()) {
                    builder.add_note("unicode escape sequences must use a lowercase 'u'");
                    suggestion.replace(start_idx, 2, r"\u");
                } else if error == r#"\""# {
                    builder.add_note("double quotes may only be escaped within double-quoted name selectors");
                    suggestion.replace(start_idx, 2, r#"""#);
                } else if error == r"\'" {
                    builder.add_note("single quotes may only be escaped within single-quoted name selectors");
                    suggestion.replace(start_idx, 2, r#"'"#);
                } else {
                    builder.add_note(r#"the only valid escape sequences are \n, \r, \t, \f, \b, \\, \/, \' (in single quoted names), \" (in double quoted names), and \uXXXX where X are hex digits"#);
                    suggestion.invalidate()
                }
            }
            SyntaxErrorKind::UnpairedHighSurrogate => {
                builder.add_note(
                    "a UTF-16 high surrogate has to be followed by a low surrogate to encode a valid Unicode character",
                );
                builder.add_note("for more information about UTF-16 surrogate pairs see https://en.wikipedia.org/wiki/UTF-16#Code_points_from_U+010000_to_U+10FFFF");
                suggestion.invalidate();
            }
            SyntaxErrorKind::UnpairedLowSurrogate => {
                builder.add_note(
                    "a UTF-16 low surrogate has to be preceded by a high surrogate to encode a valid Unicode character",
                );
                builder.add_note("for more information about UTF-16 surrogate pairs see https://en.wikipedia.org/wiki/UTF-16#Code_points_from_U+010000_to_U+10FFFF");
                suggestion.invalidate();
            }
            SyntaxErrorKind::InvalidHexDigitInUnicodeEscape => {
                builder.add_note("valid hex digits are 0 through 9 and A through F (case-insensitive)");
                suggestion.invalidate();
            }
            SyntaxErrorKind::MissingClosingSingleQuote => suggestion.insert(end_idx, "'"),
            SyntaxErrorKind::MissingClosingDoubleQuote => suggestion.insert(end_idx, "\""),
            SyntaxErrorKind::MissingRootIdentifier => suggestion.insert(start_idx, "$"),
            SyntaxErrorKind::InvalidSegmentStart => {
                builder.add_note("valid segments are: member name shorthands like `.name`/`..name`; or child/descendant bracketed selections like `[<segments>]`/`..[<segments>]`");
                suggestion.invalidate();
            }
            SyntaxErrorKind::InvalidSegmentAfterTwoPeriods => {
                if error.starts_with('.') {
                    let nerror = error.trim_start_matches('.');
                    let number_of_periods = error.len() - nerror.len();
                    suggestion.remove(start_idx, number_of_periods);
                } else {
                    suggestion.invalidate();
                }
                builder.add_note("valid segments are either member name shorthands `name`, or bracketed selections like `['name']` or `[42]`");
            }
            SyntaxErrorKind::InvalidNameShorthandAfterOnePeriod => {
                if error.starts_with('[') {
                    suggestion.remove(start_idx - 1, 1);
                } else {
                    suggestion.invalidate();
                }
            }
            SyntaxErrorKind::MissingSelectorSeparator => {
                let prefix_whitespace_len = prefix.len() - prefix.trim_end_matches(' ').len(); // FIXME
                suggestion.insert(start_idx - prefix_whitespace_len, ",");
            }
            SyntaxErrorKind::MissingClosingBracket => suggestion.insert(end_idx, "]"),
            SyntaxErrorKind::NegativeZeroInteger => suggestion.replace(start_idx, error.len(), "0"),
            SyntaxErrorKind::LeadingZeros => {
                let is_negative = error.starts_with('-');
                let replacement = error.trim_start_matches(['-', '0']);
                let offset = if is_negative { 1 } else { 0 };

                if replacement.is_empty() {
                    suggestion.replace(start_idx, error.len(), "0");
                } else {
                    let remove_len = error.len() - replacement.len() - offset;
                    suggestion.remove(start_idx + offset, remove_len);
                }
            }
            SyntaxErrorKind::InvalidSelector
            | SyntaxErrorKind::IndexParseError(_)
            | SyntaxErrorKind::SliceStartParseError(_)
            | SyntaxErrorKind::SliceStepParseError(_)
            | SyntaxErrorKind::SliceEndParseError(_)
            | SyntaxErrorKind::EmptySelector => suggestion.invalidate(),
        }

        // Generic notes.
        if error.starts_with('$') {
            builder.add_note("the root identifier '$' must appear exactly once at the start of the query");
        }
    }

    fn split_error<'a>(&self, input: &'a str) -> (&'a str, &'a str, &'a str) {
        let start = input.len() - self.rev_idx;
        let (prefix, rest) = input.split_at(start);
        let (error, suffix) = if self.len >= rest.len() {
            (rest, "")
        } else {
            rest.split_at(self.len)
        };
        (prefix, error, suffix)
    }
}

struct DisplayableSyntaxErrorBuilder {
    current_line: String,
    current_underline_offset: usize,
    current_underline_len: usize,
    current_underline_message: Option<String>,
    lines: Vec<SyntaxErrorLine>,
    notes: Vec<SyntaxErrorNote>,
}

impl DisplayableSyntaxErrorBuilder {
    fn new() -> Self {
        Self {
            current_line: String::new(),
            lines: vec![],
            current_underline_offset: 0,
            current_underline_len: 0,
            current_underline_message: None,
            notes: vec![],
        }
    }

    fn add_non_underline(&mut self, width: usize) {
        if self.current_underline_len == 0 {
            self.current_underline_offset += width;
        }
    }

    fn add_underline(&mut self, width: usize) {
        self.current_underline_len += width;
    }

    fn add_underline_message<S: AsRef<str>>(&mut self, message: S) {
        self.current_underline_message = Some(message.as_ref().to_string());
    }

    fn add_note<S: AsRef<str>>(&mut self, message: S) {
        self.notes.push(SyntaxErrorNote {
            message: message.as_ref().to_string(),
        })
    }

    fn add_char(&mut self, c: char) {
        if c == '\n' {
            self.finish_line();
        } else {
            self.current_line.push(c);
        }
    }

    fn finish_line(&mut self) {
        let underline = self.finish_underline();
        let mut line = String::new();
        std::mem::swap(&mut line, &mut self.current_line);
        self.lines.push(SyntaxErrorLine { line, underline })
    }

    fn finish_underline(&mut self) -> Option<SyntaxErrorUnderline> {
        let res = (self.current_underline_len > 0).then(|| SyntaxErrorUnderline {
            start_pos: self.current_underline_offset,
            len: self.current_underline_len,
            message: self.current_underline_message.take(),
        });

        self.current_underline_offset = 0;
        self.current_underline_len = 0;
        res
    }

    fn finish(
        mut self,
        toplevel_message: String,
        start_idx: usize,
        end_idx: usize,
        style: ErrorStyleImpl,
    ) -> DisplayableSyntaxError {
        self.finish_line();
        DisplayableSyntaxError {
            toplevel_message,
            start_idx,
            end_idx,
            lines: self.lines,
            notes: self.notes,
            style,
        }
    }
}

#[derive(Debug)]
pub(crate) enum InternalParseError<'a> {
    SyntaxError(SyntaxError, &'a str),
    SyntaxErrors(Vec<SyntaxError>, &'a str),
    NomError(nom::error::Error<&'a str>),
}

impl<'a> nom::error::ParseError<&'a str> for InternalParseError<'a> {
    fn from_error_kind(input: &'a str, kind: nom::error::ErrorKind) -> Self {
        Self::NomError(nom::error::Error::from_error_kind(input, kind))
    }

    fn append(input: &'a str, kind: nom::error::ErrorKind, other: Self) -> Self {
        match other {
            Self::NomError(e) => Self::NomError(nom::error::Error::append(input, kind, e)),
            _ => other,
        }
    }
}

struct DisplayableSyntaxError {
    toplevel_message: String,
    start_idx: usize,
    end_idx: usize,
    lines: Vec<SyntaxErrorLine>,
    notes: Vec<SyntaxErrorNote>,
    style: ErrorStyleImpl,
}

struct SyntaxErrorNote {
    message: String,
}

struct SyntaxErrorLine {
    line: String,
    underline: Option<SyntaxErrorUnderline>,
}

struct SyntaxErrorUnderline {
    start_pos: usize,
    len: usize,
    message: Option<String>,
}

enum Suggestion {
    Valid(Vec<SuggestionDiff>),
    Invalid,
}

#[derive(Debug)]
enum SuggestionDiff {
    Insert(usize, String),
    Remove(usize, usize),
    Replace(usize, usize, String),
}

impl SuggestionDiff {
    fn start_idx(&self) -> usize {
        match self {
            Self::Remove(idx, _) | Self::Replace(idx, _, _) | Self::Insert(idx, _) => *idx,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn foo() {
        let input = "$..['abc' 'def']....abc..['\n']";
        let mut suggestion = Suggestion::new();
        suggestion.insert(9, ",");
        suggestion.remove(18, 2);
        suggestion.replace(27, 1, "\\n");

        let result = suggestion.build(input).unwrap();
        assert_eq!(result, "$..['abc', 'def']..abc..['\\n']");
    }
}

impl Suggestion {
    fn new() -> Self {
        Self::Valid(vec![])
    }

    fn insert<S: AsRef<str>>(&mut self, at: usize, str: S) {
        self.push(SuggestionDiff::Insert(at, str.as_ref().to_string()))
    }

    fn remove(&mut self, at: usize, len: usize) {
        self.push(SuggestionDiff::Remove(at, len))
    }

    fn replace<S: AsRef<str>>(&mut self, at: usize, remove_len: usize, str: S) {
        self.push(SuggestionDiff::Replace(at, remove_len, str.as_ref().to_string()))
    }

    fn push(&mut self, diff: SuggestionDiff) {
        match self {
            Self::Valid(diffs) => diffs.push(diff),
            Self::Invalid => (),
        }
    }

    fn invalidate(&mut self) {
        *self = Self::Invalid
    }

    fn build(self, input: &str) -> Option<String> {
        match self {
            Self::Invalid => None,
            Self::Valid(mut diffs) => {
                let mut result = String::new();
                let mut input_chars = input.char_indices();
                let mut next = input_chars.next();
                diffs.sort_by_key(SuggestionDiff::start_idx);
                diffs.reverse();

                while let Some((i, c)) = next {
                    if let Some(x) = diffs.last() {
                        if x.start_idx() == i {
                            let x = diffs.pop().expect("unreachable, last is Some");
                            match x {
                                SuggestionDiff::Insert(_, str) => {
                                    result.push_str(&str);
                                }
                                SuggestionDiff::Remove(_, len) => {
                                    let end_idx = i + len;
                                    while let Some((i, _)) = next {
                                        if i >= end_idx {
                                            break;
                                        }
                                        next = input_chars.next();
                                    }
                                }
                                SuggestionDiff::Replace(_, len, str) => {
                                    result.push_str(&str);
                                    let end_idx = i + len;
                                    while let Some((i, _)) = next {
                                        if i >= end_idx {
                                            break;
                                        }
                                        next = input_chars.next();
                                    }
                                }
                            }
                            continue;
                        }
                    }
                    next = input_chars.next();
                    result.push(c);
                }

                // Any diffs that remain should be inserts at the end.
                // Verify that and apply them.
                while let Some(diff) = diffs.pop() {
                    match diff {
                        SuggestionDiff::Insert(at, str) if at == input.len() => result.push_str(&str),
                        _ => panic!("invalid suggestion diff beyond bounds of input: {diff:?}"),
                    }
                }

                Some(result)
            }
        }
    }
}

impl Display for DisplayableSyntaxError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        writeln!(
            f,
            "{} {}",
            self.style.error_prefix(&"error:"),
            self.style.error_message(&self.toplevel_message)
        )?;
        writeln!(f)?;
        let multiline = self.lines.len() > 1;

        for (i, line) in self.lines.iter().enumerate() {
            if multiline {
                writeln!(
                    f,
                    " {: >3} {} {}",
                    self.style.line_numbers(&(i + 1)),
                    self.style.line_numbers(&"|"),
                    line.line
                )?;
            } else {
                writeln!(f, "  {}", line.line)?;
            }

            if let Some(underline) = &line.underline {
                if multiline {
                    write!(f, "     {} ", self.style.line_numbers(&"|"))?;
                } else {
                    write!(f, "  ")?;
                }

                for _ in 0..underline.start_pos {
                    write!(f, " ")?;
                }
                for _ in 0..underline.len {
                    write!(f, "{}", self.style.error_underline(&"^"))?;
                }
                if let Some(msg) = &underline.message {
                    writeln!(f, " {}", self.style.error_underline_message(msg))?;
                } else {
                    writeln!(f)?;
                }
            }
        }

        if multiline {
            write!(f, " ")?;
        }
        if self.start_idx == self.end_idx {
            writeln!(
                f,
                "  {} {}{}",
                self.style.error_position_hint(&"(byte"),
                self.style.error_position_hint(&self.start_idx),
                self.style.error_position_hint(&")")
            )?;
        } else {
            writeln!(
                f,
                "  {} {}{}{}{}",
                self.style.error_position_hint(&"(bytes"),
                self.style.error_position_hint(&self.start_idx),
                self.style.error_position_hint(&"-"),
                self.style.error_position_hint(&self.end_idx),
                self.style.error_position_hint(&")")
            )?;
        }

        writeln!(f)?;

        if !self.notes.is_empty() {
            let mut first = true;
            for note in &self.notes {
                if !first {
                    writeln!(f)?;
                };
                write!(f, "{} {note}", self.style.note_prefix(&"note:"))?;
                first = false;
            }
        }

        Ok(())
    }
}

impl Display for SyntaxErrorNote {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.message)
    }
}

impl SyntaxErrorKind {
    #[inline]
    fn toplevel_message(&self) -> String {
        match self {
            Self::DisallowedLeadingWhitespace => "query starting with whitespace".to_string(),
            Self::DisallowedTrailingWhitespace => "query ending with whitespace".to_string(),
            Self::InvalidUnescapedCharacter => "invalid unescaped control character".to_string(),
            Self::InvalidEscapeSequence => "invalid escape sequence".to_string(),
            Self::UnpairedHighSurrogate => "invalid unicode escape sequence - unpaired high surrogate".to_string(),
            Self::UnpairedLowSurrogate => "invalid unicode escape sequence - unpaired low surrogate".to_string(),
            Self::InvalidHexDigitInUnicodeEscape => "invalid unicode escape sequence - invalid hex digit".to_string(),
            Self::MissingClosingDoubleQuote => "double-quoted name selector is not closed".to_string(),
            Self::MissingClosingSingleQuote => "single-quoted name selector is not closed".to_string(),
            Self::MissingRootIdentifier => "query not starting with the root identifier '$'".to_string(),
            Self::InvalidSegmentStart => "invalid segment syntax".to_string(),
            Self::InvalidSegmentAfterTwoPeriods => "invalid descendant segment syntax".to_string(),
            Self::InvalidNameShorthandAfterOnePeriod => "invalid short member name syntax".to_string(),
            Self::InvalidSelector => "invalid selector syntax".to_string(),
            Self::EmptySelector => "invalid selector - empty".to_string(),
            Self::MissingSelectorSeparator => "selectors not separated with commas".to_string(),
            Self::MissingClosingBracket => "bracketed selection is not closed".to_string(),
            Self::NegativeZeroInteger => "negative zero used as an integer".to_string(),
            Self::LeadingZeros => "integer with leading zeros".to_string(),
            Self::IndexParseError(_) => "invalid index value".to_string(),
            Self::SliceStartParseError(_) => "invalid slice start".to_string(),
            Self::SliceEndParseError(_) => "invalid slice end".to_string(),
            Self::SliceStepParseError(_) => "invalid slice step value".to_string(),
        }
    }

    #[inline]
    fn underline_message(&self) -> String {
        match self {
            Self::DisallowedLeadingWhitespace => "leading whitespace is disallowed".to_string(),
            Self::DisallowedTrailingWhitespace => "trailing whitespace is disallowed".to_string(),
            Self::InvalidUnescapedCharacter => "this character must be escaped".to_string(),
            Self::InvalidEscapeSequence => "not a valid escape sequence".to_string(),
            Self::UnpairedHighSurrogate => "this high surrogate is unpaired".to_string(),
            Self::UnpairedLowSurrogate => "this low surrogate is unpaired".to_string(),
            Self::InvalidHexDigitInUnicodeEscape => "not a hex digit".to_string(),
            Self::MissingClosingDoubleQuote => "expected a double quote '\"'".to_string(),
            Self::MissingClosingSingleQuote => "expected a single quote `'`".to_string(),
            Self::MissingRootIdentifier => "the '$' character missing before here".to_string(),
            Self::InvalidSegmentStart => "not a valid segment syntax".to_string(),
            Self::InvalidSegmentAfterTwoPeriods => "not a valid descendant segment syntax".to_string(),
            Self::InvalidNameShorthandAfterOnePeriod => "not a valid name shorthand".to_string(),
            Self::InvalidSelector => "not a valid selector".to_string(),
            Self::EmptySelector => "expected a selector here, but found nothing".to_string(),
            Self::MissingSelectorSeparator => "expected a comma separator before this character".to_string(),
            Self::MissingClosingBracket => "expected a closing bracket ']'".to_string(),
            Self::NegativeZeroInteger => "negative zero is not allowed".to_string(),
            Self::LeadingZeros => "leading zeros are not allowed".to_string(),
            Self::IndexParseError(inner) => format!("this index value is invalid; {inner}"),
            Self::SliceStartParseError(inner) => format!("this start index is invalid; {inner}"),
            Self::SliceEndParseError(inner) => format!("this end index is invalid; {inner}"),
            Self::SliceStepParseError(inner) => format!("this step value is invalid; {inner}"),
        }
    }
}

#[cfg(feature = "color")]
mod colored {
    use super::{fmt_parse_error, ParseError};
    use std::fmt::{self, Display};
    use thiserror::Error;

    #[derive(Debug, Error)]
    pub(super) struct ColoredParseError(pub(super) ParseError);

    impl Display for ColoredParseError {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            fmt_parse_error(&self.0, &OwoColorsErrorStyle::colored(), f)
        }
    }

    #[derive(Clone)]
    pub(super) struct OwoColorsErrorStyle {
        error_prefix: owo_colors::Style,
        error_message: owo_colors::Style,
        error_position_hint: owo_colors::Style,
        error_underline: owo_colors::Style,
        error_underline_message: owo_colors::Style,
        line_numbers: owo_colors::Style,
        note_prefix: owo_colors::Style,
        suggestion: owo_colors::Style,
    }

    impl OwoColorsErrorStyle {
        pub(super) fn colored() -> Self {
            let error_color = owo_colors::Style::new().bright_red();
            let error_message = owo_colors::Style::new().bold();
            let error_position_hint = owo_colors::Style::new().dimmed();
            let line_color = owo_colors::Style::new().cyan();
            let note_color = owo_colors::Style::new().bright_cyan();
            let suggestion_color = owo_colors::Style::new().bright_cyan().bold();

            Self {
                error_prefix: error_color,
                error_message,
                error_position_hint,
                error_underline: error_color,
                error_underline_message: error_color,
                line_numbers: line_color,
                note_prefix: note_color,
                suggestion: suggestion_color,
            }
        }

        pub(crate) fn empty() -> Self {
            let empty_style = owo_colors::Style::new();
            Self {
                error_prefix: empty_style,
                error_message: empty_style,
                error_position_hint: empty_style,
                error_underline: empty_style,
                error_underline_message: empty_style,
                line_numbers: empty_style,
                note_prefix: empty_style,
                suggestion: empty_style,
            }
        }

        pub(crate) fn error_prefix<'a, D: Display>(&self, target: &'a D) -> impl Display + 'a {
            use owo_colors::OwoColorize;
            target.style(self.error_prefix)
        }

        pub(crate) fn error_message<'a, D: Display>(&self, target: &'a D) -> impl Display + 'a {
            use owo_colors::OwoColorize;
            target.style(self.error_message)
        }

        pub(crate) fn error_position_hint<'a, D: Display>(&self, target: &'a D) -> impl Display + 'a {
            use owo_colors::OwoColorize;
            target.style(self.error_position_hint)
        }

        pub(crate) fn error_underline<'a, D: Display>(&self, target: &'a D) -> impl Display + 'a {
            use owo_colors::OwoColorize;
            target.style(self.error_underline)
        }

        pub(crate) fn error_underline_message<'a, D: Display>(&self, target: &'a D) -> impl Display + 'a {
            use owo_colors::OwoColorize;
            target.style(self.error_underline_message)
        }

        pub(crate) fn line_numbers<'a, D: Display>(&self, target: &'a D) -> impl Display + 'a {
            use owo_colors::OwoColorize;
            target.style(self.line_numbers)
        }

        pub(crate) fn note_prefix<'a, D: Display>(&self, target: &'a D) -> impl Display + 'a {
            use owo_colors::OwoColorize;
            target.style(self.note_prefix)
        }

        pub(crate) fn suggestion<'a, D: Display>(&self, target: &'a D) -> impl Display + 'a {
            use owo_colors::OwoColorize;
            target.style(self.suggestion)
        }
    }
}

#[cfg(not(feature = "color"))]
mod plain {
    use std::fmt::Display;

    #[derive(Clone)]
    pub(super) struct PlainErrorStyle;

    impl PlainErrorStyle {
        pub(crate) fn empty() -> Self {
            Self
        }

        // We want to keep the same function signature as for the colored version, so `&self` must be here.
        // We could use a trait, but returning `impl trait` in traits would bump MSRV to 1.75.
        #[allow(clippy::unused_self)]
        pub(crate) fn error_prefix<'a, D: Display>(&self, target: &'a D) -> impl Display + 'a {
            target
        }

        #[allow(clippy::unused_self)]
        pub(crate) fn error_message<'a, D: Display>(&self, target: &'a D) -> impl Display + 'a {
            target
        }

        #[allow(clippy::unused_self)]
        pub(crate) fn error_position_hint<'a, D: Display>(&self, target: &'a D) -> impl Display + 'a {
            target
        }

        #[allow(clippy::unused_self)]
        pub(crate) fn error_underline<'a, D: Display>(&self, target: &'a D) -> impl Display + 'a {
            target
        }

        #[allow(clippy::unused_self)]
        pub(crate) fn error_underline_message<'a, D: Display>(&self, target: &'a D) -> impl Display + 'a {
            target
        }

        #[allow(clippy::unused_self)]
        pub(crate) fn line_numbers<'a, D: Display>(&self, target: &'a D) -> impl Display + 'a {
            target
        }

        #[allow(clippy::unused_self)]
        pub(crate) fn note_prefix<'a, D: Display>(&self, target: &'a D) -> impl Display + 'a {
            target
        }

        #[allow(clippy::unused_self)]
        pub(crate) fn suggestion<'a, D: Display>(&self, target: &'a D) -> impl Display + 'a {
            target
        }
    }
}
