use crate::bytes::align::{alignment, AlignedBytes};

#[cfg(not(feature = "nosimd"))]
pub mod simd {
    use super::*;
    #[cfg(all(target_arch = "x86", target_feature = "avx2"))]
    use core::arch::x86::*;
    #[cfg(all(target_arch = "x86_64", target_feature = "avx2"))]
    use core::arch::x86_64::*;

    /// Calculate a bitmasks that marks all escaped, non-backslash characters
    /// with set bits.
    /// 
    /// # Examples
    /// ```
    /// # use simdpath::bytes::classify_escaped;
    /// # use simdpath::bytes::align::{alignment, AlignedBytes};
    /// let string = r#"abc \" \{ \} \\\" \\\\""#;
    /// let bytes = AlignedBytes::<alignment::Block>::from(string.as_bytes());
    /// let result = classify_escaped(&bytes);
    /// 
    /// assert_eq!(result, 0b10000100100100000);
    /// ```
    #[inline(always)]
    pub fn classify_escaped(bytes: &AlignedBytes<alignment::Block>) -> u32 {
        #[cfg(target_feature = "avx2")]
        unsafe {
            avx2_classify_escaped(bytes)
        }
        #[cfg(not(target_feature = "avx2"))]
        nosimd::classify_escaped(bytes)
    }

    #[target_feature(enable = "avx2")]
    #[cfg(target_feature = "avx2")]
    #[allow(dead_code)]
    #[inline]
    unsafe fn avx2_classify_escaped(bytes: &AlignedBytes<alignment::Block>) -> u32 {
        use crate::bytes::align::Aligned;

        assert!(bytes.len() <= 32);
        if bytes.len() < 32 {
            let mut padded_bytes = AlignedBytes::<alignment::Block>::new_zeroed(32);
            padded_bytes[..bytes.len()].copy_from_slice(bytes);
            return avx2_classify_escaped(&padded_bytes);
        }

        let vector = _mm256_load_si256(bytes.as_ptr() as *const __m256i);
        let slash_vector = _mm256_set1_epi8(b'\\' as i8);
        let slash_cmp = _mm256_cmpeq_epi8(vector, slash_vector);
        let slashes = _mm256_movemask_epi8(slash_cmp) as u32;

        let even = 0b01010101010101010101010101010101u32;
        let odd = 0b10101010101010101010101010101010u32;
        let starts = slashes & !(slashes << 1);
        let even_starts = even & starts;
        let odd_starts = odd & starts;

        let ends_of_even_starts = (even_starts + slashes) & !slashes;
        let ends_of_odd_starts = (odd_starts + slashes) & !slashes;

        (ends_of_even_starts & odd) | (ends_of_odd_starts & even)
    }
}

pub mod nosimd {
    use super::*;
    
    /// Calculate a bitmasks that marks all escaped, non-backslash characters
    /// with set bits.
    /// 
    /// # Examples
    /// ```
    /// # use simdpath::bytes::classify_escaped;
    /// # use simdpath::bytes::align::{alignment, AlignedBytes};
    /// let string = r#"abc \" \{ \} \\\" \\\\""#;
    /// let bytes = AlignedBytes::<alignment::Block>::from(string.as_bytes());
    /// let result = classify_escaped(&bytes);
    /// 
    /// assert_eq!(result, 0b10000100100100000);
    /// ```
    #[inline]
    pub fn classify_escaped(bytes: &AlignedBytes<alignment::Block>) -> u32 {
        let mut result = 0u32;
        let len = std::cmp::min(bytes.len(), 32);
        let mut escaped = false;

        for (i, &b) in bytes[..len].iter().enumerate() {
            match b {
                b'\\' => escaped = !escaped,
                _ => {
                    if escaped {
                        result |= 1 << i;
                    }
                    escaped = false;
                }
            };
        }
        return result;
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use proptest::prelude::*;
    use test_case::test_case;

    macro_rules! test_cases {
        ($testname:ident, $impl:expr) => {
            #[test_case(r#"\\\xx"# => 0b01000; "when unescaped follows an escaped")]
            #[test_case(r#""label \"quotes\"\\\"\\\"\\""# => 0b1000100010000000100000000; "when label contains escaped quotes")]
            #[test_case(r#""# => 0; "when string is empty")]
            fn $testname(source: &str) -> u32 {
                let aligned = AlignedBytes::<alignment::Block>::from(source.as_bytes());
                $impl(&aligned)
            }
        }
    }

    test_cases!(nosimd_classify, nosimd::classify_escaped);

    #[cfg(not(feature = "nosimd"))]
    test_cases!(simd_classify, simd::classify_escaped);

    macro_rules! proptests {
        ($testname:ident, $impl:expr) => {
            proptest! {
                #[test]
                fn $testname(
                    bytes in prop::collection::vec(
                        prop_oneof![Just(b'\\'), Just(b'x')],
                        0..32
                    )
                ) {
                    let aligned = AlignedBytes::<alignment::Block>::from(bytes);
                    let result = $impl(&aligned);
                    let len = std::cmp::min(aligned.len(), 32);

                    for i in 0..len {
                        let bit = result & (1 << i);
                        let expected = is_escaped(i, &aligned) && aligned[i] != b'\\';

                        prop_assert_eq!(bit != 0, expected, "At index {}", i);
                    }
                }
            }
        };
    }

    proptests!(
        nonsimd_classifier_correctly_classifies_unescaped_bytes,
        nosimd::classify_escaped
    );

    #[cfg(not(feature = "nosimd"))]
    proptests!(
        simd_classifier_correctly_classifies_unescaped_bytes,
        simd::classify_escaped
    );

    fn is_escaped(idx: usize, slice: &[u8]) -> bool {
        slice[..idx]
            .iter()
            .rev()
            .take_while(|&&x| x == b'\\')
            .count()
            % 2
            != 0
    }
}
