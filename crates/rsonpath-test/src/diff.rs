use rsonpath::result::{CountResult, IndexResult, QueryResult};
use std::fmt::Display;

pub struct ExpectCount {
    count: u64,
}

pub struct ExpectBytes {
    bytes: Vec<usize>,
}

pub trait Expect<R: QueryResult> {
    fn diff(&self, result: &R) -> Option<Diff>;
}

pub struct Diff {
    text: String
}

impl ExpectCount {
    pub fn new(count: u64) -> Self {
        ExpectCount { count }
    }
}

impl ExpectBytes {
    pub fn new(bytes: &[usize]) -> Self {
        ExpectBytes {
            bytes: bytes.to_vec(),
        }
    }
}

impl Expect<CountResult> for ExpectCount {
    fn diff(&self, result: &CountResult) -> Option<Diff> {
        let actual = result.get().try_into().unwrap();
        if self.count != actual {
            Some(Diff {
                text: format!("t'was different. expected {}, got {}", self.count, actual)
            })
        }
        else {
            None
        }
    }
}

impl Expect<IndexResult> for ExpectBytes {
    fn diff(&self, result: &IndexResult) -> Option<Diff> {
        let actual = result.get();
        if self.bytes != actual {
            Some(Diff {
                text: format!("t'was different. expected {:?}, got {:?}", self.bytes, actual)
            })
        }
        else {
            None
        }
    }
}

impl Display for Diff {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.text)
    }
}
