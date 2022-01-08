use crate::bytes::align::{alignment, AlignedBytes};

#[cfg(not(feature = "nosimd"))]
mod simd {
    use super::*;
    #[cfg(all(target_arch = "x86", target_feature = "avx2"))]
    use core::arch::x86::*;
    #[cfg(all(target_arch = "x86_64", target_feature = "avx2"))]
    use core::arch::x86_64::*;

    #[inline(always)]
    pub fn classify_nonescaped_quotes(bytes: &AlignedBytes<alignment::Block>) -> usize {
        #[cfg(target_feature = "avx2")]
        unsafe {
            avx2_classify_nonescaped_quotes(bytes)
        }
        #[cfg(not(target_feature = "avx2"))]
        nosimd::classify_nonescaped_quotes(bytes)
    }

    #[target_feature(enable = "avx2")]
    #[cfg(target_feature = "avx2")]
    #[allow(dead_code)]
    #[inline]
    unsafe fn avx2_classify_nonescaped_quotes(bytes: &AlignedBytes<alignment::Block>) -> usize {
        todo!()
    }
}

mod nosimd {
    use super::*;

    pub fn classify_nonescaped_quotes(bytes: &AlignedBytes<alignment::Block>) -> usize {
        let mut result = 0usize;
        let len = std::cmp::min(bytes.len(), 64);
        let mut even_number_of_escapes = true;

        for (i, &b) in bytes[..len].iter().enumerate() {
            match b {
                b'\\' => even_number_of_escapes = !even_number_of_escapes,
                b'"' => {
                    if even_number_of_escapes {
                        result |= 1 << i;
                    }
                    even_number_of_escapes = true;
                }
                _ => even_number_of_escapes = true,
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
            #[test_case(r#"\\\"""# => 0b10000usize; "when unescaped follows an escaped quote")]
            #[test_case(r#"\n""# => 0b100usize; "when unescaped follows an escaped char")]
            #[test_case(r#""label \"quotes\"\\\"\\\"\\""# => 0b1000000000000000000000000001usize; "when label contains escaped quotes")]
            #[test_case(r#""# => 0usize; "when string is empty")]
            fn $testname(source: &str) -> usize {
                let aligned = AlignedBytes::<alignment::Block>::from(source.as_bytes());
                $impl(&aligned)
            }
        }
    }

    test_cases!(nosimd_classify, nosimd::classify_nonescaped_quotes);

    #[cfg(not(feature = "nosimd"))]
    test_cases!(simd_classify, simd::classify_nonescaped_quotes);

    macro_rules! proptests {
        ($testname:ident, $impl:expr) => {
            proptest! {
                #[test]
                fn $testname(
                    bytes in prop::collection::vec(
                        prop_oneof![Just(b'\\'), Just(b'"'), Just(b'x')],
                        0..64
                    )
                ) {
                    let aligned = AlignedBytes::<alignment::Block>::from(bytes);
                    let result = $impl(&aligned);
                    let len = std::cmp::min(aligned.len(), 64);

                    for i in 0..len {
                        let bit = result & (1 << i);
                        let is_unescaped_quote = is_quote(i, &aligned) && !is_escaped(i, &aligned);

                        prop_assert_eq!(bit != 0, is_unescaped_quote, "At index {}", i);
                    }
                }
            }
        };
    }

    proptests!(
        nonsimd_classifier_correctly_classifies_unescaped_quotes,
        nosimd::classify_nonescaped_quotes
    );

    #[cfg(not(feature = "nosimd"))]
    proptests!(
        simd_classifier_correctly_classifies_unescaped_quotes,
        simd::classify_nonescaped_quotes
    );

    fn is_quote(idx: usize, slice: &[u8]) -> bool {
        slice[idx] == b'"'
    }

    fn is_escaped(idx: usize, slice: &[u8]) -> bool {
        if idx == 0 {
            return false;
        }
        slice[..idx]
            .iter()
            .rev()
            .take_while(|&&x| x == b'\\')
            .count()
            % 2
            != 0
    }
}
