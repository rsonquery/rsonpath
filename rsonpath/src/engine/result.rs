//! Result types that can be returned by a JSONPath query engine.

use std::fmt::{self, Display};

/// Result informing on the number of values matching the executed query.
pub struct CountResult {
    /// Number of values matched by the executed query.
    pub count: usize,
}

impl Display for CountResult {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.count)
    }
}
