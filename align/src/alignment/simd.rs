use super::Alignment;
use cfg_if::cfg_if;

/// Alignment to a SIMD block guarantee.
///
/// It is guaranteed that this alignment's [`size`](`Alignment::size`) is a multiplicity
/// of the size of a SIMD register of the target architecture.
///
/// # Alignments
///
/// The alignment size will be the first entry in the below table
/// that is supported by the target CPU.
///
/// | CPU feature     | Alignment (bytes) |
/// |-----------------|------------------:|
/// | AVX2            | 32                |
#[derive(Debug)]
pub struct SimdBlock {}

/// Alignment to two SIMD blocks guarantee.
///
/// This size is always equal to twice the size of [`SimdBlock`].
///
/// # Examples
/// ```rust
/// use align::alignment::{self, Alignment};
///
/// assert_eq!(2 * alignment::SimdBlock::size(), alignment::TwoSimdBlocks::size());
/// ```
#[derive(Debug)]
pub struct TwoSimdBlocks {}

// SAFETY:
// Always returning a const value that is a power of two.
unsafe impl Alignment for SimdBlock {
    fn size() -> usize {
        cfg_if! {
            if #[cfg(all(
                any(target_arch = "x86", target_arch = "x86_64"),
                target_feature = "avx2",
            ))] {
                32
            } else if #[cfg(doc)] {
                32
            }
            else {
                compile_error!("Target architecture is not supported by SIMD features of this crate. Disable the default `simd` feature.");
                unreachable!();
            }
        }
    }
}

// SAFETY:
// Safe as long as the impl for `SimdBlock` is safe, since we multiply by 2.
unsafe impl Alignment for TwoSimdBlocks {
    fn size() -> usize {
        SimdBlock::size() * 2
    }
}
