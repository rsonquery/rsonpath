use super::*;

pub struct BorrowedBytes<'a> {
    bytes: &'a [u8],
}

pub struct BorrowedBytesBlockIterator<'a, const N: usize> {
    input: &'a [u8],
    idx: usize,
}

impl<'a> BorrowedBytes<'a> {
    #[must_use]
    #[inline(always)]
    pub fn new(bytes: &'a [u8]) -> Self {
        Self { bytes }
    }
}

impl<'a, const N: usize> BorrowedBytesBlockIterator<'a, N> {
    #[must_use]
    #[inline(always)]
    pub fn new(bytes: &'a [u8]) -> Self {
        Self {
            input: bytes,
            idx: 0,
        }
    }
}

impl<'a> Input for BorrowedBytes<'a> {
    type BlockIterator<'b, const N: usize> = BorrowedBytesBlockIterator<'b, N> where Self: 'b;

    #[inline(always)]
    fn iter_blocks<const N: usize>(&self) -> Self::BlockIterator<'_, N> {
        Self::BlockIterator {
            input: self.bytes,
            idx: 0,
        }
    }

    #[inline]
    fn seek_backward(&self, from: usize, needle: u8) -> Option<usize> {
        let mut idx = from;

        loop {
            if self.bytes[idx] == needle {
                return Some(idx);
            }
            if idx == 0 {
                return None;
            }
            idx -= 1;
        }
    }

    #[inline]
    fn seek_non_whitespace_forward(&self, from: usize) -> Option<(usize, u8)> {
        let mut idx = from;

        loop {
            let b = self.bytes[idx];
            if !b.is_ascii_whitespace() {
                return Some((idx, b));
            }
            idx += 1;
            if idx == self.bytes.len() {
                return None;
            }
        }
    }

    #[inline]
    fn seek_non_whitespace_backward(&self, from: usize) -> Option<(usize, u8)> {
        let mut idx = from;

        loop {
            let b = self.bytes[idx];
            if !b.is_ascii_whitespace() {
                return Some((idx, b));
            }
            if idx == 0 {
                return None;
            }
            idx -= 1;
        }
    }

    #[inline]
    #[cfg(feature = "head-skip")]
    fn find_label(&self, from: usize, label: &Label) -> Option<usize> {
        use memchr::memmem;

        let finder = memmem::Finder::new(label.bytes_with_quotes());
        let mut idx = from;

        loop {
            match finder.find(&self.bytes[idx..]) {
                Some(offset) => {
                    let starting_quote_idx = offset + idx;
                    if self.bytes[starting_quote_idx - 1] != b'\\' {
                        return Some(starting_quote_idx);
                    }
                    else {
                        idx = starting_quote_idx + label.bytes_with_quotes().len() + 1;
                    }
                },
                None => return None,
            }
        }
    }

    #[inline]
    fn is_label_match(&self, from: usize, to: usize, label: &Label) -> bool {
        let slice = &self.bytes[from..to];
        label.bytes_with_quotes() == slice && (from == 0 || self.bytes[from - 1] != b'\\')
    }
}

impl<'a, const N: usize> Iterator for BorrowedBytesBlockIterator<'a, N> {
    type Item = &'a [u8];

    #[inline]
    fn next(&mut self) -> Option<Self::Item> {
        if self.idx >= self.input.len() {
            None
        } else {
            let block = &self.input[self.idx..self.idx + N];
            self.idx += N;

            Some(block)
        }
    }
}

impl<'a, const N: usize> InputBlockIterator<'a, N> for BorrowedBytesBlockIterator<'a, N> {
    type Block = Self::Item;

    #[inline(always)]
    fn offset(&mut self, count: isize) {
        assert!(count >= 0);
        self.idx += count as usize * N;
    }
}
