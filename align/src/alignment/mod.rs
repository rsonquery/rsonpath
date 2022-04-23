//! Types of possible alignment type arguments for [`AlignedBytes`](`super::AlignedBytes`).
use cfg_if::cfg_if;

/// Trait for all alignment types that provides its size.
///
/// # Safety
/// The `size` returned must satisfy the following conditions:
/// - it is constant between calls, i.e. two calls to `size` for the same alignment *MUST* return the same value;
/// - the value returned is a power of two.
///
/// Violating any of these constraints will cause undefined behavior when the alignment is used
/// for [`AlignedBytes`](`super::AlignedBytes`).
pub unsafe trait Alignment {
    /// Size of the alignment.
    fn size() -> usize;
}

/// Alignment to 1 byte, so no special alignment &ndash; every slice is always one-byte-aligned.
///
/// # Examples
/// ```rust
/// use align::alignment::{self, Alignment};
///
/// assert_eq!(1, alignment::One::size());
/// ```
#[derive(Debug)]
pub struct One {}

/// Alignment to page boundary.
///
/// Size is the size of a single page in the OS as returned by the
/// [`page_size`] crate.
///
/// # Examples
/// ```rust
/// use page_size;
/// use align::alignment::{self, Alignment};
///
/// assert_eq!(page_size::get(), alignment::Page::size());
/// ```
#[derive(Debug)]
pub struct Page {}

// SAFETY:
// One is a constant power of two.
unsafe impl Alignment for One {
    fn size() -> usize {
        1
    }
}

// SAFETY:
// We check whether the size is power of two. The [`page_size`] crate caches the result
// of its call, so it will not change, but I prefer not to rely on an external crate not changing
// its implementation for safety.
//
// No sane platform would have a page size that is not a power of two, but better not to take chances.
// This assertion will only be called once anyway.
unsafe impl Alignment for Page {
    fn size() -> usize {
        use lazy_static::lazy_static;

        lazy_static! {
            static ref PAGE_SIZE: usize = {
                let size = page_size::get();

                if size.next_power_of_two() != size {
                    panic!("detected page size that is not a power of two, this is unsupported");
                }

                size
            };
        }

        *PAGE_SIZE
    }
}

cfg_if! {
    if #[cfg(doc)] {
        #[cfg_attr(docsrs, doc(cfg(feature = "simd")))]
        mod simd;

        #[cfg_attr(docsrs, doc(cfg(feature = "simd")))]
        #[doc(inline)]
        pub use simd::*;
    }
    else if #[cfg(feature = "simd")] {
        mod simd;
        pub use simd::*;
    }
}
