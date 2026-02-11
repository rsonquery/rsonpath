//! Logic for pretty-printing syntax errors.
//! This includes displaying the error and underline, fix suggestions, and other user-comfort notes.
//! Managing the style of displayed messages: coloring, emphasis, etc. - is done by the [`style`]
//! submodule, while this submodule deals with generating all the underlines, suggestions, notes,
//! and printing it to screen with an injected style.
use super::{formatter, style};
use crate::error::{InnerParseError, ParseError, SyntaxErrorKind};
use crate::str::EscapeMode;
use formatter::SyntaxErrorLine;
#[cfg(feature = "color")]
use std::error::Error;
use std::fmt;
use std::fmt::Display;

// Resolve the default style depending on whether the optional color dependencies are available.
use crate::JSONPATH_WHITESPACE_BYTES;
#[cfg(feature = "color")]
use style::colored::OwoColorsErrorStyle as ErrorStyleImpl;
#[cfg(not(feature = "color"))]
use style::plain::PlainErrorStyle as ErrorStyleImpl;

/// Controls the default width of tabulation for calculating the width of underlines.
/// It seems impossible to know how wide they will be displayed on the end user's device during construction,
/// so we default to 4. This hopefully shouldn't be too annoying: if you're non-ironically using tabs to format
/// a JSONPath query you're asking for trouble yourself.
const TAB_DISPLAY_WIDTH: usize = 4;
/// Controls the maximum allowed width of displayed line, including the underlined error and the context to the left
/// and right. It has to be limited to _some_ constant, as otherwise every individual error in a single long line
/// would cause the entire line to be written.
pub(super) const MAX_ERROR_LINE_WIDTH: usize = 100;
/// Controls the minimum allowed width of the original query to be displayed to the left and right of the underlined
/// error. If the error part is excessively long, the truncation could remove all the context. With this we force
/// it to print at least a few characters.
pub(super) const MIN_CONTEXT_WIDTH: usize = 5;

/// Allows querying for display width of a character.
pub(super) trait UnicodeWidth {
    /// Width of a character to consider for calculating underline offsets and lengths.
    fn width(&self) -> usize;
}

impl UnicodeWidth for char {
    /// Width of a character to consider for calculating underline offsets and lengths.
    ///
    /// This is the Unicode width of the character, except for `\t`: it has a Unicode width of 1, which is dumb.
    /// We use [`TAB_DISPLAY_WIDTH`] to control it instead.
    fn width(&self) -> usize {
        // Display tabs with a fixed width.
        // How this looks depends on the user's terminal settings, but we use a reasonable default.
        match self {
            '\t' => TAB_DISPLAY_WIDTH,
            _ => unicode_width::UnicodeWidthChar::width(*self).unwrap_or(0),
        }
    }
}

/// Get an empty, non-colored style. This should be used when the error is displayed not on-demand
/// by the end application, but via the default [`Display`] of the error (e.g. during a panic).
pub(super) fn empty_style() -> ErrorStyleImpl {
    ErrorStyleImpl::empty()
}

#[cfg(feature = "color")]
impl ParseError {
    /// Turn the error into a version with colored display.
    #[inline(always)]
    #[must_use]
    #[cfg_attr(docsrs, doc(cfg(feature = "color")))]
    pub fn colored(self) -> impl Error {
        style::colored::ColoredParseError(self)
    }
}

impl SyntaxErrorKind {
    /// Defines the main error message displayed to the user as the first line of the error.
    ///
    /// All must start with lowercase, not end with a period, and ideally be a single short sentence.
    /// It should avoid including any contextful information, for example a number parsing error should
    /// not include the input that failed in the message &ndash; instead, the error will be underlined
    /// and the user should see the relevant [`underline_message`] next to it.
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
            Self::NumberParseError(_) => "invalid number format".to_string(),
            Self::MissingClosingParenthesis => "missing closing parenthesis in filter expression".to_string(),
            Self::InvalidNegation => "invalid use of logical negation".to_string(),
            Self::MissingComparisonOperator => "missing comparison operator".to_string(),
            Self::InvalidComparisonOperator => "invalid comparison operator".to_string(),
            Self::InvalidComparable => "invalid right-hand side of comparison".to_string(),
            Self::NonSingularQueryInComparison => "non-singular query used in comparison".to_string(),
            Self::InvalidFilter => "invalid filter expression syntax".to_string(),
        }
    }

    /// Defines the error message displayed to the user right underneath the highlighted invalid
    /// portion of the query string.
    ///
    /// All must start with lowercase, not end with a period, and ideally be a single short sentence.
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
            Self::NumberParseError(inner) => format!("this number is invalid; {inner}"),
            Self::MissingClosingParenthesis => "expected a closing parenthesis `(`".to_string(),
            Self::InvalidNegation => "this negation is ambiguous".to_string(),
            Self::InvalidComparable => "expected a literal or a filter query here".to_string(),
            Self::NonSingularQueryInComparison => "this query is not singular".to_string(),
            Self::MissingComparisonOperator => "expected a comparison operator here".to_string(),
            Self::InvalidComparisonOperator => "not a valid comparison operator".to_string(),
            Self::InvalidFilter => "not a valid filter expression".to_string(),
        }
    }
}
impl super::SyntaxError {
    /// This creates friendly displayable errors.
    ///
    /// An error consists of
    /// - The toplevel error name/message.
    /// - A list of lines of the input, each with an optional underline message.
    /// - A list of notes/suggestions at the end.
    ///
    /// Every error displays the entire error as well as some context before and after the error.
    /// These are called the _pre-context_ and _post-context_, respectively. Ideally, we display the entire
    /// line with the error. However, if the line is very long it would kill performance if many separate errors
    /// were to print all of it to the output. Instead, we use the [`DisplayableSyntaxErrorBuilder`] to maintain
    /// a manageable pre- and post-context (controlled by [`MAX_ERROR_LINE_WIDTH`]).
    ///
    /// Controlling the width requires computing byte index offsets and widths of all characters. To avoid quadratic
    /// blowup, we compute this information once for the input via [`IndexedInput`](indexed_input::IndexedInput)
    /// and use it in every [`display`] invocation.
    fn display(
        &self,
        input: &formatter::ErrorFormatter,
        suggestion: &mut Suggestion,
        style: ErrorStyleImpl,
    ) -> DisplayableSyntaxError {
        let start_idx = input.len() - self.rev_idx;
        let end_idx = start_idx + self.len - 1;

        let lines = input.build_error_lines(
            start_idx,
            end_idx,
            MIN_CONTEXT_WIDTH,
            MAX_ERROR_LINE_WIDTH,
            self.kind.underline_message(),
        );
        let notes = self.generate_notes(suggestion, input.str());

        DisplayableSyntaxError {
            toplevel_message: self.kind.toplevel_message(),
            start_idx,
            end_idx,
            lines,
            notes,
            is_multiline: input.is_multiline(),
            style,
        }
    }

    /// Add suggestions and notes to the error message based on the error kind.
    fn generate_notes(&self, suggestion: &mut Suggestion, input: &str) -> Vec<SyntaxErrorNote> {
        // Figure out the first and last byte of the highlighted error. Errors always respect UTF-8 boundaries.
        let start_idx = input.len() - self.rev_idx;
        let end_idx = start_idx + self.len - 1;
        let (prefix, error, suffix) = self.split_error(input);
        // Kind-specific notes and suggestion building.
        let mut notes = vec![];
        match self.kind {
            SyntaxErrorKind::DisallowedLeadingWhitespace | SyntaxErrorKind::DisallowedTrailingWhitespace => {
                // Suggestion is to just remove the whitespace.
                suggestion.remove(start_idx, error.len());
            }
            SyntaxErrorKind::InvalidUnescapedCharacter => {
                // Escaping is context-sensitive (depends on surrounding quotes) for single and double quotes.
                // For everything else we can use the existing machinery and pass an arbitrary EscapeMode.
                if error == "\"" {
                    suggestion.replace(start_idx, 1, r#"\""#);
                } else if error == "'" {
                    suggestion.replace(start_idx, 1, r"\'");
                } else {
                    let escaped = crate::str::escape(error, EscapeMode::DoubleQuoted);
                    suggestion.replace(start_idx, error.len(), escaped);
                }
            }
            SyntaxErrorKind::InvalidEscapeSequence => {
                if error == r"\U" && suffix.len() >= 4 && suffix[..4].chars().all(|x| x.is_ascii_hexdigit()) {
                    // The user probably tried to use a Unicode escape but is unaware the `u` is case-sensitive.
                    notes.push("unicode escape sequences must use a lowercase 'u'".into());
                    suggestion.replace(start_idx, 2, r"\u");
                } else if error == r#"\""# {
                    // We were in a string but escaping `"` was an error.
                    // Thus, the string must be single-quote delimited and the double quote should be unescaped.
                    notes.push("double quotes may only be escaped within double-quoted name selectors".into());
                    suggestion.replace(start_idx, 2, r#"""#);
                } else if error == r"\'" {
                    // Analogous to above, but for single quotes in double-quote delimited strings.
                    notes.push("single quotes may only be escaped within single-quoted name selectors".into());
                    suggestion.replace(start_idx, 2, "'");
                } else {
                    // Try to suggest escaping the backslash. This might not be accurate, as the user might've tried to
                    // use some unsupported escape sequence like \v. It might be useful to add some common escape
                    // sequences not valid for JSONPath and suggest to replace them with the corresponding character
                    // or full Unicode escape. This is "good enough" though, it's just a suggestion after all.
                    notes.push(r#"the only valid escape sequences are \n, \r, \t, \f, \b, \\, \/, \' (in single quoted names), \" (in double quoted names), and \uXXXX where X are hex digits"#.into());
                    notes.push(r"if you meant to match a literal backslash, you need to escape it with \\".into());
                    suggestion.insert(start_idx, r"\");
                }
            }
            SyntaxErrorKind::UnpairedHighSurrogate => {
                notes.push(
                    "a UTF-16 high surrogate has to be followed by a low surrogate to encode a valid Unicode character".into(),
                );
                notes.push("for more information about UTF-16 surrogate pairs see https://en.wikipedia.org/wiki/UTF-16#Code_points_from_U+010000_to_U+10FFFF".into());
                // No way to guess what the user wanted here.
                suggestion.invalidate();
            }
            SyntaxErrorKind::UnpairedLowSurrogate => {
                notes.push(
                    "a UTF-16 low surrogate has to be preceded by a high surrogate to encode a valid Unicode character".into(),
                );
                notes.push("for more information about UTF-16 surrogate pairs see https://en.wikipedia.org/wiki/UTF-16#Code_points_from_U+010000_to_U+10FFFF".into());
                // No way to guess what the user wanted here.
                suggestion.invalidate();
            }
            SyntaxErrorKind::InvalidHexDigitInUnicodeEscape => {
                notes.push("valid hex digits are 0 through 9 and A through F (case-insensitive)".into());
                // We can't possibly guess what the user got wrong here. Most likely they forgot one of the digits
                // and the next character was picked up as a hex digit, but we can't resolve that.
                suggestion.invalidate();
            }
            // These three are straightforward.
            SyntaxErrorKind::MissingClosingSingleQuote => suggestion.insert(end_idx, "'"),
            SyntaxErrorKind::MissingClosingDoubleQuote => suggestion.insert(end_idx, "\""),
            SyntaxErrorKind::MissingRootIdentifier => suggestion.insert(start_idx, "$"),
            SyntaxErrorKind::InvalidSegmentStart => {
                notes.push("valid segments are: member name shorthands like `.name`/`..name`; or child/descendant bracketed selections like `[<segments>]`/`..[<segments>]`".into());
                // We can't possibly guess what segment the user wanted here.
                suggestion.invalidate();
            }
            SyntaxErrorKind::InvalidSegmentAfterTwoPeriods => {
                if error.starts_with('.') {
                    // The user probably put too many periods, try to trim to two.
                    let nerror = error.trim_start_matches('.');
                    let number_of_periods = error.len() - nerror.len();
                    suggestion.remove(start_idx, number_of_periods);
                } else {
                    // Otherwise, who knows?! E.g. `$..5` might've been an attempt to use an index selector `$..[5]` or
                    // a name selector for the string "5", i.e. `$..['5']`. Both suggestions seem equally plausible.
                    suggestion.invalidate();
                }
                notes.push("valid segments are either member name shorthands `name`, or bracketed selections like `['name']` or `[42]`".into());
            }
            SyntaxErrorKind::InvalidNameShorthandAfterOnePeriod => {
                // Detects using periods in conjunction with bracketed selectors - it's a very common mistake, so it's
                // important to have good suggestions here!
                if error.starts_with('[') && error.ends_with(']') {
                    // This means someone input .[a] or .['a']. The suggestion is to first remove the period.
                    suggestion.remove(start_idx - 1, 1);
                    // Now, if someone input .[a] then the quotes are also missing. We do our best to figure out which
                    // quotes could work and insert them. If this fails then we need to manually escape single quotes.
                    let looks_valid = (error.starts_with("['") && error.ends_with("']"))
                        || (error.starts_with("[\"") && error.ends_with("\"]"));
                    if !looks_valid {
                        fix_unquoted_bracketed_selector(suggestion, error.as_bytes(), start_idx);
                    }
                } else {
                    // Otherwise it's not clear what to suggest. As in the descendant case above, a pattern like
                    // `$.5` is ambiguous.
                    suggestion.invalidate();
                }
            }
            SyntaxErrorKind::MissingSelectorSeparator => {
                // This is always resolvable by just adding the separator. We do that while respecting sensible
                // whitespacing, i.e. `$['a' 'b']` becomes `$['a`, 'b']` and not `$['a' ,'b']`.
                let prefix_whitespace_len = prefix.len() - prefix.trim_end_matches(' ').len();
                suggestion.insert(start_idx - prefix_whitespace_len, ",");
            }
            // These two are straightforward.
            SyntaxErrorKind::MissingClosingBracket => suggestion.insert(end_idx, "]"),
            SyntaxErrorKind::MissingClosingParenthesis => suggestion.insert(end_idx, ")"),
            // Also straightforward, just use a plain zero instead.
            SyntaxErrorKind::NegativeZeroInteger => suggestion.replace(start_idx, error.len(), "0"),
            SyntaxErrorKind::LeadingZeros => {
                // Leading zeroes are always resolvable by simply removing the zeroes,
                // but we need to take care to handle negative numbers correctly.
                // The error highlights the entire integer with the minus when it fails, so detection is easy.
                let is_negative = error.starts_with('-');
                // We find the meaningful part of the number, ignoring the sign and all leading zeroes.
                // This works because:
                //  - we remember the sign and offset the removal index to preserve it if needed;
                //  - the minus sign is always tightly attached to the number, i.e. inputting `- 01` is invalid
                //    and would result in a different error altogether;
                //  - we separately ensure we don't replace a zero (e.g. `00`) with nothing.
                let replacement = error.trim_start_matches(['-', '0']);
                let offset = if is_negative { 1 } else { 0 };

                if replacement.is_empty() {
                    // Special case where there is nothing left after the trim.
                    // Note that a negative zero would be an error, so we forget if the sign was there and suggest
                    // a "normal" zero.
                    suggestion.replace(start_idx, error.len(), "0");
                } else {
                    // Remove the number of trimmed characters, perhaps except the minus sign.
                    let remove_len = error.len() - replacement.len() - offset;
                    suggestion.remove(start_idx + offset, remove_len);
                }
            }
            SyntaxErrorKind::NonSingularQueryInComparison => {
                notes.push("singular queries use only child segments with single name or index selectors".into());
                // There is no way to fix it, this is simply unsupported by JSONPath.
                suggestion.invalidate();
            }
            // This one is hard, as it's kind of a catch-all "user input is nonsense" error kind.
            // However, there are some special cases that are useful to match against:
            //  - a selector like `[a]` is invalid, but the user probably wanted to search for the key `a`, so we should
            //    suggest inserting quotes;
            //  - whitespace between sign and number is disallowed (e.g. `$[- 1]` is illegal), but the user probably
            //    just wants the version without whitespace.
            //
            // If any other cases are reasonable and fall into this kind, suggestion generation should be added here.
            SyntaxErrorKind::InvalidSelector => 'handler: {
                let input_bytes = input.as_bytes();
                // Handle the minus-with-whitespace case first.
                if error.starts_with('-') {
                    use std::str::FromStr as _;
                    let white_space_len = error
                        .as_bytes()
                        .iter()
                        .skip(1)
                        .take_while(|c| JSONPATH_WHITESPACE_BYTES.contains(c))
                        .count();
                    // Make sure the suggestion makes sense, i.e. after removing the whitespace we are left with an actual number.
                    // This requires us to also trim leading zeroes and handle the case when all digits were zero.
                    let leading_zero_len = error
                        .as_bytes()
                        .iter()
                        .skip(1 + white_space_len)
                        .take_while(|c| **c == b'0')
                        .count();
                    if 1 + white_space_len + leading_zero_len == error.len() {
                        // This was just a very elaborate negative zero.
                        suggestion.replace(start_idx, error.len(), "0");
                        break 'handler;
                    }
                    // Now make sure the rest is a sensible number. Slicing is allowed since we checked all characters
                    // we skipped are just ASCII.
                    let rest = &error[1 + white_space_len + leading_zero_len..];
                    if crate::num::JsonNumber::from_str(rest).is_ok() {
                        // We're okay, just remove all the nonsense.
                        suggestion.remove(start_idx, 1 + white_space_len + leading_zero_len);
                        break 'handler;
                    }
                    // Otherwise we can't handle this, but maybe something below will.
                }

                // Try to handle the case where we are delimited by (brackets or commas).
                if start_idx == 0 || end_idx == input_bytes.len() {
                    // The error is not delimited by anything.
                    suggestion.invalidate();
                } else {
                    // We need to respect whitespace, so find the delimiters.
                    let mut start_boundary = start_idx - 1;
                    let mut end_boundary = end_idx + 1;
                    while start_boundary > 0 && input_bytes[start_boundary].is_ascii_whitespace() {
                        start_boundary -= 1;
                    }
                    while end_boundary < input.len() - 1 && input_bytes[end_boundary].is_ascii_whitespace() {
                        end_boundary += 1;
                    }

                    // If it's brackets or commas then we can try to fix the selector.
                    if [b'[', b','].contains(&input_bytes[start_boundary])
                        && [b']', b','].contains(&input_bytes[end_boundary])
                    {
                        // The invalid selector is bracketed, so the user might've meant to search for the string inside
                        // but forgot the quotes. Try to fix it if possible.
                        fix_unquoted_bracketed_selector(
                            suggestion,
                            &input_bytes[start_idx - 1..=end_idx + 1],
                            start_idx - 1,
                        );
                    } else {
                        // Otherwise we can't do anything.
                        suggestion.invalidate()
                    }
                }
            }
            SyntaxErrorKind::EmptySelector => {
                // An empty selector like `$[]`. Maybe the user wants to select everything with no particular filter?
                suggestion.insert(start_idx + 1, "*");
                notes.push("if you meant to match any value, you should use the wildcard selector `*`".into());
            }
            SyntaxErrorKind::InvalidNegation => {
                // This is an ambiguous logical negation. We cannot resolve it for the user since
                // we don't know which version they meant, so we signal to disambiguate.
                notes.push("add parenthesis around the expression you want to negate".into());
            }
            // These are number-parsing errors other than the JSONPath-specific leading-zero and negative-zero ones.
            // Can't think of a good suggestion algorithm for those.
            SyntaxErrorKind::IndexParseError(_)
            | SyntaxErrorKind::SliceStartParseError(_)
            | SyntaxErrorKind::SliceStepParseError(_)
            | SyntaxErrorKind::SliceEndParseError(_)
            | SyntaxErrorKind::NumberParseError(_)
            // There might be some sensible cases here, but I can't think of any at the moment.
            | SyntaxErrorKind::InvalidComparisonOperator
            // We cannot possibly guess what operator the user meant.
            | SyntaxErrorKind::MissingComparisonOperator
            // There might be some useful cases here like with the InvalidSelector. Feel free to suggest.
            | SyntaxErrorKind::InvalidFilter
            | SyntaxErrorKind::InvalidComparable => suggestion.invalidate(),
        }

        // Generic notes.
        if error.starts_with('$') {
            notes.push("the root identifier '$' must appear exactly once at the start of the query".into());
        }

        return notes;

        fn fix_unquoted_bracketed_selector(suggestion: &mut Suggestion, selector_bytes: &[u8], idx_offset: usize) {
            // Try to fix a selector of the form `[somestr]` that is missing quotes.
            // There are three possible way of fixing it - `['somestr']`, `["somestr"]`, and also sometimes simplifying
            // to the shorthand selector `somestr`. We ignore the shorthand to simplify and try to suggest one of the
            // canonical forms. We prefer single quotes over double quotes, unless `somestr` contains unescaped single
            // quotes already. If `somestr` contains both kinds of quotes we will need to find all unescaped single
            // quotes and escape them before inserting the delimiting ones.
            let mut escaped = false;
            let mut unescaped_single = false;
            let mut unescaped_double = false;
            for &b in selector_bytes.iter().skip(1).take(selector_bytes.len() - 1) {
                if !escaped && b == b'\'' {
                    unescaped_single = true;
                }
                if !escaped && b == b'"' {
                    unescaped_double = true;
                }
                if b == b'\\' {
                    escaped = !escaped;
                } else {
                    escaped = false;
                }
            }
            if !unescaped_single {
                suggestion.insert(idx_offset + 1, "'");
                suggestion.insert(idx_offset + selector_bytes.len() - 1, "'");
            } else if !unescaped_double {
                suggestion.insert(idx_offset + 1, "\"");
                suggestion.insert(idx_offset + selector_bytes.len() - 1, "\"");
            } else {
                // Go again and escape all unescaped quotes.
                let mut escaped = false;
                for (i, &b) in selector_bytes.iter().enumerate() {
                    if !escaped && b == b'\'' {
                        suggestion.insert(idx_offset + i, "\\");
                    }
                    if b == b'\\' {
                        escaped = !escaped;
                    } else {
                        escaped = false;
                    }
                }
                // Now inserting single quotes is valid.
                suggestion.insert(idx_offset + 1, "'");
                suggestion.insert(idx_offset + selector_bytes.len() - 1, "'");
            }
        }
    }

    /// Locate the error within the input and split it into three parts, (prefix, error, suffix).
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

/// Format a [`ParseError`] into a [`Formatter`](fmt::Formatter) using the specified [`ErrorStyleImpl`].
#[inline(always)]
pub(super) fn fmt_parse_error(error: &ParseError, style: &ErrorStyleImpl, f: &mut fmt::Formatter<'_>) -> fmt::Result {
    match &error.inner {
        InnerParseError::Syntax(syntax_errors) => {
            // We display all the errors separately and accumulate the fixes to show one suggestion at the end.
            // First, index the input to avoid repeating work between consecutive errors.
            let indexed_input = formatter::ErrorFormatter::new(&error.input);
            let mut suggestion = Suggestion::new();
            for syntax_error in syntax_errors {
                writeln!(
                    f,
                    "{}",
                    syntax_error.display(&indexed_input, &mut suggestion, style.clone())
                )?;
            }

            if let Some(suggestion) = suggestion.apply(&error.input) {
                writeln!(
                    f,
                    "{} did you mean `{}` ?",
                    style.note_prefix(&"suggestion:"),
                    style.suggestion(&suggestion)
                )?;
            }
        }
        InnerParseError::RecursionLimit(limit) => {
            writeln!(
                f,
                "{} {}",
                style.error_prefix(&"error:"),
                style.error_message(&"nesting level exceeded")
            )?;
            writeln!(f)?;
            writeln!(f, "  {}", error.input)?;
            writeln!(
                f,
                "{} the parser limits nesting to {}; this applies to filter logical expressions",
                style.note_prefix(&"note:"),
                limit
            )?;
        }
    }

    Ok(())
}

/// Syntax error that can be pretty-printed.
///
/// This is not a publicly accessible type and exists only as an intermediary between the actual [`ParserError`]
/// and the output for its display.
struct DisplayableSyntaxError {
    toplevel_message: String,
    start_idx: usize,
    end_idx: usize,
    is_multiline: bool,
    lines: Vec<SyntaxErrorLine>,
    notes: Vec<SyntaxErrorNote>,
    style: ErrorStyleImpl,
}

struct SyntaxErrorNote {
    message: String,
}

impl From<&str> for SyntaxErrorNote {
    #[inline]
    fn from(value: &str) -> Self {
        Self {
            message: value.to_string(),
        }
    }
}

/// Suggestion for correcting the erroneous input, displayed to the user.
///
/// The suggestion is either a sequence of diff operations that can be applied to transform the input into a correct
/// one, or an [`Invalid`](Suggestion::Invalid) state which disables the suggestion &ndash; sometimes it's impossible
/// to make a sensible one.
enum Suggestion {
    Valid(Vec<SuggestionDiff>),
    Invalid,
}

#[derive(Debug)]
enum SuggestionDiff {
    /// At a given byte index of the original input, insert the given string.
    Insert(usize, String),
    /// Starting at a given byte index of the original input, remove this many bytes.
    Remove(usize, usize),
    /// Starting at a given byte index of the original input, remove this many bytes
    /// and replace them with the given string.
    Replace(usize, usize, String),
}

impl SuggestionDiff {
    fn start_idx(&self) -> usize {
        match self {
            Self::Remove(idx, _) | Self::Replace(idx, _, _) | Self::Insert(idx, _) => *idx,
        }
    }
}

impl Suggestion {
    fn new() -> Self {
        Self::Valid(vec![])
    }

    /// At a given byte index of the original input, insert the given string.
    fn insert<S: AsRef<str>>(&mut self, at: usize, str: S) {
        self.push(SuggestionDiff::Insert(at, str.as_ref().to_string()))
    }

    /// Starting at a given byte index of the original input, remove this many bytes.
    fn remove(&mut self, at: usize, len: usize) {
        self.push(SuggestionDiff::Remove(at, len))
    }

    /// Starting at a given byte index of the original input, remove this many bytes
    /// and replace them with the given string.
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

    /// Apply the suggestion to the given input (if possible and not [`Invalid`](Suggestion::Invalid)).
    fn apply(self, input: &str) -> Option<String> {
        match self {
            Self::Invalid => None,
            Self::Valid(mut diffs) => {
                // Treat the `diffs` as a stack of suggestions with the nearest start_idx at the top.
                // Then go through each character in the input and perform an action if the char idx matches the top
                // of the stack. This relies on the suggestions being sensible and respecting UTF-8 boundaries.
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
                    // else when no diff is applied
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

// Actually display the error.
// This is straightforward - all hard logic was performed above, now we just read the instructions and follow them
// while applying the internal style.
impl Display for DisplayableSyntaxError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        // Top-level error message.
        writeln!(
            f,
            "{} {}",
            self.style.error_prefix(&"error:"),
            self.style.error_message(&self.toplevel_message)
        )?;
        writeln!(f)?;

        // Annotated lines of input.
        for line in &self.lines {
            // Only print line numbers if required.
            if self.is_multiline {
                write!(
                    f,
                    " {: >3} {} ",
                    self.style.line_numbers(&(line.line_number + 1)),
                    self.style.line_numbers(&"|"),
                )?;
            } else {
                write!(f, "  ")?;
            }
            if line.truncated_start {
                write!(f, "{}", self.style.truncation_marks(&"(...) "))?;
            }
            write!(f, "{}", line.line)?;
            if line.truncated_end {
                write!(f, "{}", self.style.truncation_marks(&" (...)"))?;
            }
            if !line.line.ends_with('\n') {
                writeln!(f)?;
            }

            // Print the underline if it exists in this line.
            if let Some(underline) = &line.underline {
                if underline.len > 0 {
                    // If the input is multiline then we extend the vertical line to look nicer.
                    if self.is_multiline {
                        write!(f, "     {} ", self.style.line_numbers(&"|"))?;
                    } else {
                        write!(f, "  ")?;
                    }

                    for _ in 0..underline.start_pos {
                        write!(f, " ")?;
                    }
                    if line.truncated_start {
                        write!(f, "      ")?;
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
        }

        // If the input is multiline then we offset the bytes indices so that they visually start directly below
        // the vertical bar. Purely aesthetical choice.
        if self.is_multiline {
            write!(f, "   ")?;
        }
        // Print the byte indices, differently if there's only one or if it's a range.
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

        // Print all the notes at the end.
        if !self.notes.is_empty() {
            // Track if it's the first line to avoid a trailing newline.
            let mut first = true;
            for note in &self.notes {
                if !first {
                    writeln!(f)?;
                }
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn artificial_suggestion_test() {
        let input = "$..['abc' 'def']....abc..['\n']";
        let mut suggestion = Suggestion::new();
        suggestion.insert(9, ",");
        suggestion.remove(18, 2);
        suggestion.replace(27, 1, "\\n");

        let result = suggestion.apply(input).unwrap();
        assert_eq!(result, "$..['abc', 'def']..abc..['\\n']");
    }
}
