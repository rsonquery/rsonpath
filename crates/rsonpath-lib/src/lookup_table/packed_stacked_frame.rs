// use rsonpath_syntax::num::JsonUInt;

use log::debug;

const MAX_VALUE_47_BITS: usize = 0x7FFFFFFFFFFF;

#[derive(Clone, Copy)]
pub struct PackedStackFrame {
    frame: [u8; 16], // Frame is exactly 16 bytes
}

type JsonUInt = u64;

/// PackedStackFrame has a size of 16 bytes and following structure:
/// - Byte 0: depth: u8
/// - Byte 1: state: u8
/// - Bytes 2-9: JsonUInt (u64) array_count: JsonUInt
/// - Bytes 10-15 minus the last bit: idx_of_last_opening: 48 - 1 = 47 bits
/// - Last bit of byte 15: is_list: bool
impl PackedStackFrame {
    /// Creates a new `PackedStackFrame` instance.
    pub fn new(depth: u8, state: u8, is_list: bool, array_count: JsonUInt, idx_of_last_opening: usize) -> Self {
        let mut frame = [0u8; 16];

        // Byte 0: depth
        frame[0] = depth;

        // Byte 1: state
        frame[1] = state;

        // Bytes 2-9: JsonUInt (u64) array_count
        frame[2..10].copy_from_slice(&array_count.to_le_bytes());

        // Bytes 10-15 minus the last bit: idx_of_last_opening
        // Mask and truncate to fit in 47 bits (6 bytes - 1 bit)
        if idx_of_last_opening > MAX_VALUE_47_BITS {
            println!("Err: While building PackedStackFrame the passed value of idx_of_last_opening can not fit into 47 bits: {}", idx_of_last_opening);
        }
        let idx_masked = (idx_of_last_opening & ((1 << 47) - 1)) as u64;
        let idx_bytes = idx_masked.to_le_bytes();
        frame[10..16].copy_from_slice(&idx_bytes[..6]);

        // Last bit of byte 15: is_list
        if is_list {
            frame[15] |= 0b10000000; // Mask to set the most significant bit
        }

        Self { frame }
    }

    /// Extracts the depth field (Byte 0).
    pub fn depth(&self) -> u8 {
        self.frame[0]
    }

    /// Extracts the state field (Byte 1).
    pub fn state(&self) -> u8 {
        self.frame[1]
    }

    /// Extracts the `is_list` field (last bit of byte 15).
    pub fn is_list(&self) -> bool {
        self.frame[15] & 0b10000000 != 0
    }

    /// Extracts the `array_count` field (Bytes 2-9).
    pub fn array_count(&self) -> JsonUInt {
        JsonUInt::from_le_bytes(self.frame[2..10].try_into().unwrap())
    }

    /// Extracts the `idx_of_last_opening` field (Bytes 10-15 minus the last bit).
    pub fn idx_of_last_opening(&self) -> usize {
        // 8 bytes buffer for u64 but we are only interested in the first 47 bits
        let mut bytes = [0u8; 8];
        // Copy 6 bytes
        bytes[..6].copy_from_slice(&self.frame[10..16]);
        // Mask out the most significant bit of the 6th byte because that is reserved for is_list
        bytes[5] &= 0b01111111;
        // Set the last two bytes to 0 because we do not hold them in the struct
        bytes[6] = 0;
        bytes[7] = 0;
        u64::from_le_bytes(bytes) as usize
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
            .field("depth", &self.depth())
            .field("state", &self.state())
            .field("is_list", &self.is_list())
            .field("array_count", &self.array_count())
            .field("idx_of_last_opening", &self.idx_of_last_opening())
            .finish()
    }
}

pub fn test_packed_stacked_frame() {
    let frame = PackedStackFrame::new(10, 20, true, 123456789, 987654);
    println!("{:?}", frame);

    println!("Result vs. Expected:");
    println!("- depth:               ({}, {})", frame.depth(), 10);
    println!("- state:               ({}, {})", frame.state(), 20);
    println!("- is_list:             ({}, {})", frame.is_list(), true);
    println!("- array_count:         ({}, {})", frame.array_count(), 123456789);
    println!("- idx_of_last_opening: ({}, {})", frame.idx_of_last_opening(), 987654);
}

#[cfg(test)]
mod tests {
    use super::*;

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
        let max_array_count = u64::MAX; // 64-bit max value
        let max_idx_of_last_opening = (1 << 47) - 1; // 47 bits set to 1

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
        // Value exceeding 47 bits
        let invalid_idx_of_last_opening = (1 << 48) - 1; // 48 bits set to 1
        let truncated_idx_of_last_opening = (1 << 47) - 1; // Expected after truncation

        let frame = PackedStackFrame::new(10, 20, false, 0, invalid_idx_of_last_opening);
        println!("{:?}", frame);

        assert_eq!(frame.idx_of_last_opening(), truncated_idx_of_last_opening);
    }

    #[test]
    fn test_packed_stack_frame_boundary_values() {
        // Check transitions for depth and state at boundaries
        let frame = PackedStackFrame::new(255, 0, true, 0, 0);
        assert_eq!(frame.depth(), 255);
        assert_eq!(frame.state(), 0);

        let frame = PackedStackFrame::new(0, 255, false, 0, 0);
        assert_eq!(frame.depth(), 0);
        assert_eq!(frame.state(), 255);
    }
}
