use super::*;

/// Decorates a byte slice with JSON depth information.
///
/// This struct works on the entire slice and calculates the depth sequentially.
pub struct Vector<'a> {
    bytes: &'a [u8],
    depth: isize,
    idx: usize,
}

impl<'a> Vector<'a> {
    /// The remainder is guaranteed to be an empty slice,
    /// since this implementation works on the entire byte
    /// slice at once.
    #[inline]
    pub fn new(bytes: &'a [u8]) -> Self {
        let mut vector = Self {
            bytes,
            depth: 0,
            idx: 0,
        };
        vector.advance();
        vector
    }
}

impl<'a> DepthBlock<'a> for Vector<'a> {
    #[inline]
    fn len(&self) -> usize {
        self.bytes.len()
    }

    #[inline]
    fn advance(&mut self) -> bool {
        if self.idx >= self.bytes.len() {
            return false;
        }
        self.depth += match self.bytes[self.idx] {
            b'{' => 1,
            b'[' => 1,
            b'}' => -1,
            b']' => -1,
            _ => 0,
        };
        self.idx += 1;

        true
    }

    #[inline]
    fn is_depth_greater_or_equal_to(&self, depth: isize) -> bool {
        self.depth >= depth
    }

    #[inline]
    fn depth_at_end(mut self) -> isize {
        while self.advance() {}
        self.depth
    }
}
