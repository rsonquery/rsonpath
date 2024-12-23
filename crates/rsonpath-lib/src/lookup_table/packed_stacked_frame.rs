use log::debug;

const MAX_VALUE_55_BITS: usize = (1 << 55) - 1; // Max value for 55 bits
const MAX_VALUE_56_BITS: u64 = (1 << 56) - 1; // Max value for 56 bits

#[derive(Clone, Copy)]
pub struct PackedStackFrame {
    frame: [u8; 16], // Frame is exactly 16 bytes
}

type JsonUInt = u64;

/// PackedStackFrame has a size of 16 bytes and the following structure:
/// - Bytes 0-6: JsonUInt (u64) array_count (56 bits)
/// - Byte 7: depth: u8
/// - Bytes 8-14 (minus the last bit): idx_of_last_opening: 55 bits
/// - Byte 14 (last bit): is_list: 1 bit
/// - Byte 15: state: u8
impl PackedStackFrame {
    /// Creates a new `PackedStackFrame` instance.
    pub fn new(depth: u8, state: u8, is_list: bool, array_count: JsonUInt, idx_of_last_opening: usize) -> Self {
        let mut frame = [0u8; 16];

        // Bytes 0-6: array_count (56 bits)
        // debug_assert!(
        //     array_count <= MAX_VALUE_56_BITS,
        //     "array_count exceeds 56-bit limit: {}",
        //     array_count
        // );
        frame[0..7].copy_from_slice(&array_count.to_le_bytes()[..7]);

        // Byte 7: depth
        frame[7] = depth;

        // Bytes 8-14 minus the last bit: idx_of_last_opening (55 bits)
        // debug_assert!(
        //     idx_of_last_opening <= MAX_VALUE_55_BITS,
        //     "idx_of_last_opening exceeds 55-bit limit: {}",
        //     idx_of_last_opening
        // );
        let idx_masked = (idx_of_last_opening & MAX_VALUE_55_BITS) as u64;
        let mut idx_bytes = idx_masked.to_le_bytes();
        idx_bytes[6] &= 0b01111111; // Mask off the most significant bit for is_list
        frame[8..14].copy_from_slice(&idx_bytes[..6]);

        // Byte 14: is_list (1 bit in the most significant position)
        if is_list {
            frame[14] |= 0b10000000; // Set the most significant bit
        }

        // Byte 15: state
        frame[15] = state;

        Self { frame }
    }

    /// Extracts the array_count field (Bytes 0-6).
    pub fn array_count(&self) -> JsonUInt {
        let mut bytes = [0u8; 8];
        bytes[..7].copy_from_slice(&self.frame[0..7]);
        JsonUInt::from_le_bytes(bytes)
    }

    /// Extracts the depth field (Byte 7)
    pub fn depth(&self) -> u8 {
        self.frame[7]
    }

    /// Extracts the idx_of_last_opening field (Bytes 8-14 minus the last bit).
    pub fn idx_of_last_opening(&self) -> usize {
        let mut bytes = [0u8; 8];
        bytes[..7].copy_from_slice(&self.frame[8..15]);
        bytes[6] &= 0b01111111; // Mask out the most significant bit
        u64::from_le_bytes(bytes) as usize
    }

    /// Extracts the is_list field (most significant bit of Byte 14).
    pub fn is_list(&self) -> bool {
        self.frame[14] & 0b10000000 != 0
    }

    /// Extracts the state field (Byte 15).
    pub fn state(&self) -> u8 {
        self.frame[15]
    }
}

// Implement Eq, PartialEq, and Debug traits
impl PartialEq for PackedStackFrame {
    fn eq(&self, other: &Self) -> bool {
        self.frame == other.frame
    }
}

impl Eq for PackedStackFrame {}

impl std::fmt::Debug for PackedStackFrame {
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

    const MAX_VALUE_55_BITS: usize = (1 << 55) - 1; // Max value for 55 bits
    const MAX_VALUE_56_BITS: u64 = (1 << 56) - 1; // Max value for 56 bits

    #[test]
    fn test_packed_stack_frame_normal_values() {
        let frame = PackedStackFrame::new(10, 20, true, 123456789, 987654);
        println!("{:?}", frame);

        assert_eq!(frame.depth(), 10);
        assert_eq!(frame.state(), 20);
        assert_eq!(frame.is_list(), true);
        assert_eq!(frame.array_count(), 123456789);
        assert_eq!(frame.idx_of_last_opening(), 987654);
    }

    #[test]
    fn test_packed_stack_frame_max_values() {
        let max_depth = u8::MAX; // 255
        let max_state = u8::MAX; // 255
        let max_array_count = MAX_VALUE_56_BITS; // 56-bit max value
        let max_idx_of_last_opening = MAX_VALUE_55_BITS; // 55-bit max value

        let frame = PackedStackFrame::new(max_depth, max_state, true, max_array_count, max_idx_of_last_opening);
        println!("{:?}", frame);

        assert_eq!(frame.depth(), max_depth);
        assert_eq!(frame.state(), max_state);
        assert_eq!(frame.is_list(), true);
        assert_eq!(frame.array_count(), max_array_count);
        assert_eq!(frame.idx_of_last_opening(), max_idx_of_last_opening);
    }

    #[test]
    fn test_packed_stack_frame_min_values() {
        let frame = PackedStackFrame::new(0, 0, false, 0, 0);
        println!("{:?}", frame);

        assert_eq!(frame.depth(), 0);
        assert_eq!(frame.state(), 0);
        assert_eq!(frame.is_list(), false);
        assert_eq!(frame.array_count(), 0);
        assert_eq!(frame.idx_of_last_opening(), 0);
    }

    #[test]
    fn test_packed_stack_frame_truncated_idx_of_last_opening() {
        // Value exceeding 55 bits
        let invalid_idx_of_last_opening = MAX_VALUE_55_BITS + 1; // 55 bits + 1 bit
        let truncated_idx_of_last_opening = MAX_VALUE_55_BITS; // Expected after truncation

        let frame = PackedStackFrame::new(10, 20, false, 0, invalid_idx_of_last_opening);
        println!("{:?}", frame);

        assert_eq!(frame.idx_of_last_opening(), truncated_idx_of_last_opening);
    }

    #[test]
    fn test_packed_stack_frame_boundary_values() {
        // Check transitions for depth and state at boundaries
        let frame = PackedStackFrame::new(u8::MAX, 0, true, 0, 0);
        assert_eq!(frame.depth(), u8::MAX);
        assert_eq!(frame.state(), 0);

        let frame = PackedStackFrame::new(0, u8::MAX, false, 0, 0);
        assert_eq!(frame.depth(), 0);
        assert_eq!(frame.state(), u8::MAX);
    }
}
