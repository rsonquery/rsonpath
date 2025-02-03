use rsonpath_syntax::num::JsonUInt;

const MASK_55_BITS: usize = (1 << 55) - 1; // Max value for 55 bits
const MASK_56_BITS: usize = (1 << 56) - 1; // Max value for 56 bits

#[derive(Clone, Copy)]
pub struct PackedStackFrame {
    frame: [u8; 16], // Frame is exactly 16 bytes
}

/// PackedStackFrame has a size of 16 bytes and the following structure:
/// - Bytes 0-6: JsonUInt (u64) array_count (56 bits)
/// - Byte 7: depth: u8 (8 bit)
/// - Bytes 8-14 (minus the last bit): idx_of_last_opening: 55 bits
/// - Byte 14 (last bit): is_list: 1 bit
/// - Byte 15: state: u8 (8 bit)
impl PackedStackFrame {
    /// Creates a new `PackedStackFrame` instance.
    #[inline]
    #[must_use]
    pub fn new(depth: u8, state: u8, is_list: bool, array_count: JsonUInt, idx_of_last_opening: usize) -> Self {
        let mut frame = [0_u8; 16];

        // Bytes 0-6: array_count (56 bits)
        debug_assert!(
            array_count.as_u64() <= (MASK_56_BITS as u64),
            "array_count exceeds 56-bit limit: {}",
            array_count
        );
        frame[0..7].copy_from_slice(&array_count.as_u64().to_le_bytes()[..7]);

        // Byte 7: depth
        frame[7] = depth;

        // Bytes 8-14 minus the last bit: idx_of_last_opening (55 bits)
        debug_assert!(
            idx_of_last_opening <= MASK_55_BITS,
            "idx_of_last_opening exceeds 55-bit limit: {}",
            idx_of_last_opening
        );
        let idx_masked = (idx_of_last_opening & MASK_55_BITS) as u64;
        let idx_bytes = idx_masked.to_le_bytes();
        frame[8..15].copy_from_slice(&idx_bytes[..7]);

        // Byte 14: is_list (1 bit in the most significant position)
        if is_list {
            frame[14] |= 0b1000_0000; // Set the most significant bit
        }

        // Byte 15: state
        frame[15] = state;

        Self { frame }
    }

    /// Extracts the array_count field (Bytes 0-6).
    #[inline]
    #[must_use]
    pub fn array_count(&self) -> JsonUInt {
        let mut bytes = [0_u8; 8];
        bytes[..7].copy_from_slice(&self.frame[0..7]);

        JsonUInt::try_from(u64::from_le_bytes(bytes)).expect("Unable to unwrap Option")
    }

    /// Extracts the depth field (Byte 7)
    #[inline]
    #[must_use]
    pub fn depth(&self) -> u8 {
        self.frame[7]
    }

    /// Extracts the idx_of_last_opening field (Bytes 8-14 minus the last bit).
    #[inline]
    #[must_use]
    pub fn idx_of_last_opening(&self) -> usize {
        let mut bytes = [0_u8; 8];
        bytes[0..7].copy_from_slice(&self.frame[8..15]);
        bytes[6] &= 0b0111_1111; // Mask out the most significant bit
        u64::from_le_bytes(bytes) as usize
    }

    /// Extracts the is_list field (most significant bit of Byte 14).
    #[inline]
    #[must_use]
    pub fn is_list(&self) -> bool {
        self.frame[14] & 0b1000_0000 != 0
    }

    /// Extracts the state field (Byte 15).
    #[inline]
    #[must_use]
    pub fn state(&self) -> u8 {
        self.frame[15]
    }
}

// Implement Eq, PartialEq, and Debug traits
impl PartialEq for PackedStackFrame {
    #[inline]
    fn eq(&self, other: &Self) -> bool {
        self.frame == other.frame
    }
}

impl Eq for PackedStackFrame {}

impl std::fmt::Debug for PackedStackFrame {
    #[inline]
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("PackedStackFrame")
            .field("array_count", &self.array_count())
            .field("depth", &self.depth())
            .field("idx_of_last_opening", &self.idx_of_last_opening())
            .field("is_list", &self.is_list())
            .field("state", &self.state())
            .finish()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_packed_stack_frame_normal_values() {
        let depth = 10;
        let state = 20;
        let is_list = true;
        let array_count = JsonUInt::from(123456789);
        let idx_of_last_opening = 987654321;

        test_build_frame(depth, state, is_list, array_count, idx_of_last_opening);
    }

    #[test]
    fn test_packed_stack_frame_max_values() {
        let depth = u8::MAX; // 255
        let state = u8::MAX; // 255
        let is_list = true;
        let array_count = JsonUInt::try_from(MASK_56_BITS as u64).unwrap(); // 56-bit max value
        let idx_of_last_opening = MASK_55_BITS; // 55-bit max value

        test_build_frame(depth, state, is_list, array_count, idx_of_last_opening);
    }

    #[test]
    fn test_packed_stack_frame_min_values() {
        let depth = 0;
        let state = 0;
        let is_list = false;
        let array_count = JsonUInt::from(0);
        let idx_of_last_opening = 0;

        test_build_frame(depth, state, is_list, array_count, idx_of_last_opening);
    }

    #[test]
    #[should_panic(expected = "idx_of_last_opening exceeds 55-bit limit")]
    fn test_packed_stack_frame_invalid_idx_of_last_opening() {
        let invalid_idx_of_last_opening = MASK_55_BITS + 1; // 55 bits + 1 bit

        // This should panic due to `debug_assert!`
        let _frame = PackedStackFrame::new(10, 20, false, JsonUInt::from(0), invalid_idx_of_last_opening);
    }

    #[test]
    #[should_panic(expected = "array_count exceeds 56-bit limit")]
    fn test_packed_stack_frame_invalid_array_count() {
        let invalid_array_count: usize = MASK_56_BITS + 1; // 56 bits + 1 bit

        // This should panic due to `debug_assert!`
        let _frame = PackedStackFrame::new(10, 20, false, JsonUInt::from(0), invalid_array_count);
    }

    fn test_build_frame(depth: u8, state: u8, is_list: bool, array_count: JsonUInt, idx_of_last_opening: usize) {
        let frame = PackedStackFrame::new(depth, state, is_list, array_count, idx_of_last_opening);
        println!("{:?}", frame);

        assert_eq!(frame.depth(), depth);
        assert_eq!(frame.state(), state);
        assert_eq!(frame.is_list(), is_list);
        assert_eq!(frame.array_count(), array_count);
        assert_eq!(frame.idx_of_last_opening(), idx_of_last_opening);
    }
}
