use std::fmt::{self, Display};
use thiserror::Error;

#[derive(Debug, Error)]
pub enum QueryError {
    #[error("one or more parsing errors occurred:\n{}", .report)]
    ParseError { report: ParseErrorReport },
    #[error("unexpected error in the parser; please report this issue at https://github.com/V0ldek/rsonpath/issues/new/choose")]
    InternalNomError {
        #[from]
        #[source]
        source: nom::error::Error<String>,
    },
}

#[derive(Debug)]
pub struct ParseErrorReport {
    errors: Vec<ParseError>,
}

impl Display for ParseErrorReport {
    #[inline]
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        for error in self.errors() {
            writeln!(f, "{}\n", error)?;
        }

        Ok(())
    }
}

#[derive(Debug)]
pub struct ParseError {
    pub start_idx: usize,
    pub len: usize,
}

impl Display for ParseError {
    #[inline]
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "invalid tokens of length {} at position {} ",
            self.len, self.start_idx
        )
    }
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

    #[inline]
    pub fn errors(&self) -> impl Iterator<Item = &ParseError> {
        self.errors.iter()
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
}
