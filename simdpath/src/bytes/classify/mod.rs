use cfg_if::cfg_if;

mod common;
pub use common::*;

cfg_if! {
    if #[cfg(all(
            any(target_arch = "x86_64", target_arch = "x86"),
            target_feature = "avx2"
    ))] {
        mod avx2;
    }
    else {
        mod nosimd;
    }
}

#[inline(always)]
pub fn classify_structural_characters<'a>(
    bytes: &'a [u8],
) -> impl Iterator<Item = Structural> + 'a {
    cfg_if! {
        if #[cfg(all(
                any(target_arch = "x86_64", target_arch = "x86"),
                target_feature = "avx2"
        ))] {
            avx2::Avx2Classifier::new(bytes)
        }
        else {
            todo!()
        }
    }
}
