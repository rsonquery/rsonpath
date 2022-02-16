use super::common::*;
use len_trait::Empty;

pub struct SequentialClassifier<'a> {
    bytes: &'a [u8],
    idx: usize,
}

impl<'a> SequentialClassifier<'a> {
    #[inline(always)]
    #[allow(dead_code)]
    pub fn new(bytes: &'a [u8]) -> Self {
        Self { bytes, idx: 0 }
    }
}

impl<'a> Iterator for SequentialClassifier<'a> {
    type Item = Structural;

    #[inline(always)]
    fn next(&mut self) -> Option<Structural> {
        use Structural::*;
        let mut escaped = false;
        let mut in_quotes = false;

        while self.idx < self.bytes.len() {
            let character = self.bytes[self.idx];
            let ret_idx = self.idx;
            self.idx += 1;

            if !escaped && character == b'"' {
                in_quotes = !in_quotes;
            }

            if character == b'\\' {
                escaped = !escaped;
            } else {
                escaped = false;
            }

            if !in_quotes {
                match character {
                    b']' | b'}' => return Some(Closing(ret_idx)),
                    b'[' | b'{' => return Some(Opening(ret_idx)),
                    b':' => return Some(Colon(ret_idx)),
                    _ => (),
                };
            }
        }

        None
    }
}

impl<'a> std::iter::FusedIterator for SequentialClassifier<'a> {}

impl<'a> Empty for SequentialClassifier<'a> {
    fn is_empty(&self) -> bool {
        self.idx == self.bytes.len()
    }
}

impl<'a> StructuralIterator<'a> for SequentialClassifier<'a> {}
