use std::fmt::{self, Display};

pub struct CountResult {
    pub count: usize,
}

impl Display for CountResult {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.count)
    }
}
