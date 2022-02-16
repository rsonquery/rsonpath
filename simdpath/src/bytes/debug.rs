//! Utilities for debugging SIMD algorithms.

/// Debug log the given u64 expression by its full 64-bit binary string representation.
#[macro_export]
macro_rules! bin {
    ($name: expr, $e:expr) => {
        log::debug!(
            "{: >24}: {:064b} ({})",
            $name,
            (|x| {
                let mut res = 0u64;
                for i in 0..64 {
                    let bit = (x & (1 << i)) >> i;
                    res |= bit << (63 - i);
                }
                res
            })($e),
            $e
        );
    };
}
