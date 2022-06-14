//! Code generation for `simd_benchmarks::sequences`.
//!
//! Used by the `build.rs` script to generate the `find_byte_sequenceN` functions.
//!
pub mod avx2;
mod cmp_and_tree;
pub mod nosimd;
pub mod sse2;
