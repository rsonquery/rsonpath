//! Source code generation for `find_byte_sequenceN` functions without SIMD support.
//!
use proc_macro2::TokenStream;
use quote::quote;

const MAX_SEQUENCE_LENGTH_FOR_NOSIMD: usize = 32;

/// Get the source for the `simd_benchmarks::sequences::nosimd` module.
pub fn get_nosimd_source() -> TokenStream {
    let match_body = (1..MAX_SEQUENCE_LENGTH_FOR_NOSIMD).map(|i| {
        let i = i + 1;
        quote! {#i => bytes.windows(#i).position(|x| x == sequence)}
    });

    quote! {
        /// Find the first occurrence of a continuous byte sequence in the slice, if it exists.
        ///
        /// This is a sequential, no-SIMD version. For big slices it is recommended to enable
        /// the default `simd` flag and use the variant exported by [`sequences`](`super`):
        /// [`find_byte_sequence`](`super::find_byte_sequence`) variant for better performance.
        ///
        /// # Examples
        /// ```
        /// # use simd_benchmarks::sequences::nosimd::find_byte_sequence;
        /// let bytes = "abcdefgh".as_bytes();
        /// let result = find_byte_sequence("de".as_bytes(), bytes);
        ///
        /// assert_eq!(Some(3), result);
        /// ```
        ///
        /// ```
        /// # use simd_benchmarks::sequences::nosimd::find_byte_sequence;
        /// let bytes = "abcdefgh".as_bytes();
        /// let result = find_byte_sequence("ed".as_bytes(), bytes);
        ///
        /// assert_eq!(None, result);
        /// ```
        ///
        #[inline]
        pub fn find_byte_sequence(sequence: &[u8], bytes: &[u8]) -> Option<usize> {
            match sequence.len() {
                0 => panic!("Cannot look for an empty sequence."),
                1 => crate::find_byte::find_byte_rust_nosimd(sequence[0], bytes),
                #(#match_body,)*
                _ => bytes.windows(sequence.len()).position(|x| x == sequence)
            }
        }
    }
}
