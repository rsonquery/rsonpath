use std::cmp;

use color_eyre::{
    eyre::{eyre, Result},
    section::Section,
    SectionExt,
};

pub(crate) struct ParseErrorReport {
    errors: Vec<ParseError>,
}

struct ParseError {
    start_idx: usize,
    len: usize,
}

impl ParseErrorReport {
    pub(crate) fn new() -> Self {
        Self { errors: vec![] }
    }

    pub(crate) fn record_at(&mut self, idx: usize) {
        match self.last_idx() {
            Some(last) if last + 1 == idx => self.extend_last(),
            _ => self.add_new(idx),
        }
    }

    fn add_new(&mut self, idx: usize) {
        self.errors.push(ParseError {
            start_idx: idx,
            len: 1,
        })
    }

    fn extend_last(&mut self) {
        let last = self.errors.last_mut().unwrap();
        last.len += 1;
    }

    fn last_idx(&self) -> Option<usize> {
        self.errors.last().map(|e| e.start_idx + e.len - 1)
    }

    pub(crate) fn error<T>(self, input: &str) -> Result<T> {
        use color_eyre::owo_colors::OwoColorize;

        let mut err = Err(eyre!("Unexpected tokens in the query string."));
        const SECTION_NAME: &str = "Parse error:";

        for parse_error in self.errors {
            let display_start_idx = if parse_error.start_idx > 80 {
                parse_error.start_idx - 80
            } else {
                0
            };
            let display_length = cmp::min(parse_error.len + 80, input.len() - display_start_idx);
            let error_slice =
                &input[parse_error.start_idx..parse_error.start_idx + parse_error.len];
            let slice = &input[display_start_idx..display_start_idx + display_length];
            let error_idx = parse_error.start_idx - display_start_idx;

            let underline: String = std::iter::repeat(' ')
                .take(error_idx)
                .chain(std::iter::repeat('^').take(parse_error.len))
                .collect();
            let display_string = format!(
                "{}\n{}",
                slice,
                (underline + " invalid tokens").bright_red()
            );

            err = err.section(display_string.header(SECTION_NAME));

            if parse_error.start_idx == 0 {
                err = err.suggestion("Queries should start with the root selector `$`.");
            }

            if error_slice.contains('$') {
                err = err.suggestion("The `$` character is reserved for the root selector and may appear only at the start.");
            }
        }

        err
    }
}
