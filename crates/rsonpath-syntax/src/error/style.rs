#[cfg(feature = "color")]
pub(super) mod colored {
    use super::super::{fmt_parse_error, ParseError};
    use std::fmt::{self, Display};
    use thiserror::Error;

    #[derive(Debug, Error)]
    pub(crate) struct ColoredParseError(pub(crate) ParseError);

    impl Display for ColoredParseError {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            fmt_parse_error(&self.0, &OwoColorsErrorStyle::colored(), f)
        }
    }

    #[derive(Clone)]
    pub(crate) struct OwoColorsErrorStyle {
        error_prefix: owo_colors::Style,
        error_message: owo_colors::Style,
        error_position_hint: owo_colors::Style,
        error_underline: owo_colors::Style,
        error_underline_message: owo_colors::Style,
        line_numbers: owo_colors::Style,
        note_prefix: owo_colors::Style,
        suggestion: owo_colors::Style,
        truncation_marks: owo_colors::Style,
    }

    impl OwoColorsErrorStyle {
        pub(super) fn colored() -> Self {
            let error_color = owo_colors::Style::new().bright_red();
            let error_message = owo_colors::Style::new().bold();
            let error_position_hint = owo_colors::Style::new().dimmed();
            let line_color = owo_colors::Style::new().cyan();
            let note_color = owo_colors::Style::new().bright_cyan();
            let suggestion_color = owo_colors::Style::new().bright_cyan().bold();
            let truncation_marks = owo_colors::Style::new().dimmed();

            Self {
                error_prefix: error_color,
                error_message,
                error_position_hint,
                error_underline: error_color,
                error_underline_message: error_color,
                line_numbers: line_color,
                note_prefix: note_color,
                suggestion: suggestion_color,
                truncation_marks,
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
                truncation_marks: empty_style,
            }
        }

        pub(crate) fn error_prefix<'a, D: Display>(&self, target: &'a D) -> impl Display + 'a {
            use owo_colors::OwoColorize as _;
            target.style(self.error_prefix)
        }

        pub(crate) fn error_message<'a, D: Display>(&self, target: &'a D) -> impl Display + 'a {
            use owo_colors::OwoColorize as _;
            target.style(self.error_message)
        }

        pub(crate) fn error_position_hint<'a, D: Display>(&self, target: &'a D) -> impl Display + 'a {
            use owo_colors::OwoColorize as _;
            target.style(self.error_position_hint)
        }

        pub(crate) fn error_underline<'a, D: Display>(&self, target: &'a D) -> impl Display + 'a {
            use owo_colors::OwoColorize as _;
            target.style(self.error_underline)
        }

        pub(crate) fn error_underline_message<'a, D: Display>(&self, target: &'a D) -> impl Display + 'a {
            use owo_colors::OwoColorize as _;
            target.style(self.error_underline_message)
        }

        pub(crate) fn line_numbers<'a, D: Display>(&self, target: &'a D) -> impl Display + 'a {
            use owo_colors::OwoColorize as _;
            target.style(self.line_numbers)
        }

        pub(crate) fn note_prefix<'a, D: Display>(&self, target: &'a D) -> impl Display + 'a {
            use owo_colors::OwoColorize as _;
            target.style(self.note_prefix)
        }

        pub(crate) fn suggestion<'a, D: Display>(&self, target: &'a D) -> impl Display + 'a {
            use owo_colors::OwoColorize as _;
            target.style(self.suggestion)
        }

        pub(crate) fn truncation_marks<'a, D: Display>(&self, target: &'a D) -> impl Display + 'a {
            use owo_colors::OwoColorize as _;
            target.style(self.truncation_marks)
        }
    }
}

#[cfg(not(feature = "color"))]
pub(super) mod plain {
    use std::fmt::Display;

    #[derive(Clone)]
    pub(crate) struct PlainErrorStyle;

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

        pub(crate) fn truncation_marks<'a, D: Display>(&self, target: &'a D) -> impl Display + 'a {
            target
        }
    }
}
