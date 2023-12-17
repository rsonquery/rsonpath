//! Error types for the crate.
//!
//! The main error type is [`ParseError`], which contains
//! all [`SyntaxErrors`](`SyntaxError`) encountered during parsing.
use std::{
    error::Error,
    fmt::{self, Display},
};
use thiserror::Error;

use crate::num::{self, error::JsonIntParseError};

#[derive(Debug)]
pub struct ParseErrorBuilder {
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
        fmt_parse_error(self, &ErrorStyle::empty(), f)
    }
}

impl ParseError {
    #[inline(always)]
    #[must_use]
    pub fn colored(self) -> impl Display + Error {
        ColoredParseError(self)
    }
}

#[derive(Debug, Error)]
struct ColoredParseError(ParseError);

impl Display for ColoredParseError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        fmt_parse_error(&self.0, &ErrorStyle::colored(), f)
    }
}

#[inline(always)]
fn fmt_parse_error(error: &ParseError, style: &ErrorStyle, f: &mut fmt::Formatter<'_>) -> fmt::Result {
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
        );
    }

    Ok(())
}

#[derive(Debug, PartialEq, Eq, Clone)]
pub struct SyntaxError {
    /// Kind of the error.
    kind: SyntaxErrorKind,
    /// The byte index at which the error occurred, counting from the end of the input.
    rev_idx: usize,
    /// The number of characters that the parser recognized as invalid.
    len: usize,
}

#[derive(Debug, PartialEq, Eq, Clone)]
pub enum SyntaxErrorKind {
    InvalidUnescapedCharacter,
    InvalidEscapeSequence,
    InvalidUnicodeEscapeSequence,
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
    Unknown,
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
    fn display(&self, input: &str, suggestion: &mut Suggestion, style: ErrorStyle) -> DisplayableSyntaxError {
        use unicode_width::UnicodeWidthChar;
        let start_idx = input.len() - self.rev_idx;
        let end_idx = start_idx + self.len - 1;
        let mut builder = DisplayableSyntaxErrorBuilder::new();

        for (i, c) in input.char_indices() {
            let width = c.width().unwrap_or(0);
            if i < start_idx {
                builder.add_non_underline(width);
            } else if i <= end_idx {
                builder.add_underline(width);
                if i == end_idx {
                    builder.add_underline_message(self.kind.underline_message());
                }
            }
            builder.add_char(c);
        }
        if end_idx >= input.len() {
            builder.add_underline(1);
            builder.add_underline_message(self.kind.underline_message());
        }

        let (prefix, error, suffix) = self.split_error(input);
        if self.kind == SyntaxErrorKind::MissingRootIdentifier {
            suggestion.insert(start_idx, "$");
        }
        if error.starts_with('$') {
            builder.add_note("the root identifier '$' must appear exactly once at the start of the query".to_string());
        }
        if self.kind == SyntaxErrorKind::InvalidNameShorthandAfterOnePeriod && error.starts_with('[') {
            suggestion.remove(start_idx - 1, 1);
        }
        if self.kind == SyntaxErrorKind::InvalidSegmentAfterTwoPeriods {
            if error.starts_with('.') {
                let nerror = error.trim_start_matches('.');
                let number_of_periods = error.len() - nerror.len();
                suggestion.remove(start_idx, number_of_periods);
            }
            builder.add_note("valid segments are either member name shorthands `name`, or bracketed selections like `['name']` or `[42]`".to_string());
        }
        if self.kind == SyntaxErrorKind::InvalidSegmentStart {
            builder.add_note("valid segments are: member name shorthands like `.name`/`..name`; or child/descendant bracketed selections like `[<segments>]`/`..[<segments>]`".to_string());
        }
        if self.kind == SyntaxErrorKind::MissingSelectorSeparator {
            suggestion.insert(start_idx, ",");
        }

        builder.finish(self.kind.toplevel_message(), start_idx, end_idx, style)
    }

    fn split_error<'a>(&self, input: &'a str) -> (&'a str, &'a str, &'a str) {
        let start = input.len() - self.rev_idx;
        let (prefix, rest) = input.split_at(start);
        let (error, suffix) = rest.split_at(self.len);
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

    fn add_underline_message(&mut self, message: String) {
        self.current_underline_message = Some(message);
    }

    fn add_note(&mut self, message: String) {
        self.notes.push(SyntaxErrorNote { message })
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
        style: ErrorStyle,
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
    style: ErrorStyle,
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
        suggestion.insert(9, ",".to_string());
        suggestion.remove(18, 2);
        suggestion.replace(27, 1, "\\n".to_string());

        let result = suggestion.build(input).unwrap();
        assert_eq!(result, "$..['abc', 'def']..abc..['\\n']");
    }
}

impl Suggestion {
    fn new() -> Self {
        Self::Valid(vec![])
    }

    fn insert<S: ToString>(&mut self, at: usize, str: S) {
        self.push(SuggestionDiff::Insert(at, str.to_string()))
    }

    fn remove(&mut self, at: usize, len: usize) {
        self.push(SuggestionDiff::Remove(at, len))
    }

    fn replace<S: ToString>(&mut self, at: usize, remove_len: usize, str: S) {
        self.push(SuggestionDiff::Replace(at, remove_len, str.to_string()))
    }

    fn push(&mut self, diff: SuggestionDiff) {
        match self {
            Self::Valid(diffs) => diffs.push(diff),
            Self::Invalid => (),
        }
    }

    fn build(mut self, input: &str) -> Option<String> {
        match self {
            Self::Invalid => None,
            Self::Valid(mut diffs) => {
                let mut result = String::new();
                let mut input = input.char_indices();
                let mut next = input.next();
                diffs.sort_by(|x, y| x.start_idx().cmp(&y.start_idx()).reverse());

                while let Some((i, c)) = next {
                    if let Some(x) = diffs.last() {
                        if x.start_idx() == i {
                            let x = diffs.pop().unwrap();
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
                                        next = input.next();
                                    }
                                }
                                SuggestionDiff::Replace(_, len, str) => {
                                    result.push_str(&str);
                                    let end_idx = i + len;
                                    while let Some((i, _)) = next {
                                        if i >= end_idx {
                                            break;
                                        }
                                        next = input.next();
                                    }
                                }
                            }
                            continue;
                        }
                    }
                    next = input.next();
                    result.push(c);
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

        if !self.notes.is_empty() {
            writeln!(f)?;
            for note in &self.notes {
                writeln!(f, "{} {note}", self.style.note_prefix(&"note:"))?;
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
            Self::InvalidUnescapedCharacter => "invalid unescaped control character".to_string(),
            Self::InvalidEscapeSequence => "invalid escape sequence".to_string(),
            Self::InvalidUnicodeEscapeSequence => "invalid unicode escape sequence".to_string(),
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
            Self::Unknown => "unknown error".to_string(),
        }
    }

    #[inline]
    fn underline_message(&self) -> String {
        match self {
            Self::InvalidUnescapedCharacter => "this character must be escaped".to_string(),
            Self::InvalidEscapeSequence => "not a valid escape sequence".to_string(),
            Self::InvalidUnicodeEscapeSequence => "not a valid unicode escape sequence".to_string(),
            Self::UnpairedHighSurrogate => "this high surrogate is unpaired".to_string(),
            Self::UnpairedLowSurrogate => "this low surrogate is unpaired".to_string(),
            Self::InvalidHexDigitInUnicodeEscape => "not a hex digit".to_string(),
            Self::MissingClosingDoubleQuote => "expected a double quote '\"'".to_string(),
            Self::MissingClosingSingleQuote => "expected a single quote `'`".to_string(),
            Self::MissingRootIdentifier => "the '$' character missing here".to_string(),
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
            Self::Unknown => "unknown error".to_string(),
        }
    }
}

#[derive(Clone)]
struct ErrorStyle {
    error_prefix: owo_colors::Style,
    error_message: owo_colors::Style,
    error_position_hint: owo_colors::Style,
    error_underline: owo_colors::Style,
    error_underline_message: owo_colors::Style,
    line_numbers: owo_colors::Style,
    note_prefix: owo_colors::Style,
    suggestion: owo_colors::Style,
}

impl ErrorStyle {
    fn empty() -> Self {
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

    fn colored() -> Self {
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

    fn error_prefix<'a, D: Display>(&self, target: &'a D) -> impl Display + 'a {
        use owo_colors::OwoColorize;
        target.style(self.error_prefix)
    }

    fn error_message<'a, D: Display>(&self, target: &'a D) -> impl Display + 'a {
        use owo_colors::OwoColorize;
        target.style(self.error_message)
    }

    fn error_position_hint<'a, D: Display>(&self, target: &'a D) -> impl Display + 'a {
        use owo_colors::OwoColorize;
        target.style(self.error_position_hint)
    }

    fn error_underline<'a, D: Display>(&self, target: &'a D) -> impl Display + 'a {
        use owo_colors::OwoColorize;
        target.style(self.error_underline)
    }

    fn error_underline_message<'a, D: Display>(&self, target: &'a D) -> impl Display + 'a {
        use owo_colors::OwoColorize;
        target.style(self.error_underline_message)
    }

    fn line_numbers<'a, D: Display>(&self, target: &'a D) -> impl Display + 'a {
        use owo_colors::OwoColorize;
        target.style(self.line_numbers)
    }

    fn note_prefix<'a, D: Display>(&self, target: &'a D) -> impl Display + 'a {
        use owo_colors::OwoColorize;
        target.style(self.note_prefix)
    }

    fn suggestion<'a, D: Display>(&self, target: &'a D) -> impl Display + 'a {
        use owo_colors::OwoColorize;
        target.style(self.suggestion)
    }
}
