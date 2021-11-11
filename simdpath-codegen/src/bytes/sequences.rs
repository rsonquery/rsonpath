//! Code generation for `simdpath::stackless::sequences`.
//!
//! Used by the `build.rs` script to generate the `find_byte_sequenceN` functions.

use proc_macro2::{Ident, TokenStream};
use quote::{format_ident, quote};
use std::collections::VecDeque;

const MAX_SEQUENCE_LENGTH_FOR_SIMD: usize = 32;
const MAX_SEQUENCE_LENGTH_FOR_NOSIMD: usize = 64;

/// Get the source for the `simdpath::stackless::automata` module.
pub fn get_mod_source() -> TokenStream {
    let find_byte_sequence_dispatch_source = get_find_byte_sequence_dispatch_source();
    let find_byte_sequence_sources = get_find_byte_sequence_sources();
    let find_long_byte_sequence_source = get_find_long_byte_sequence_source();
    let nosimd_find_byte_sequence_source = get_nosimd_find_byte_sequence_source();

    quote! {
        pub mod nosimd {
            #nosimd_find_byte_sequence_source
        }

        #[cfg(not(feature = "nosimd"))]
        pub mod simd {
            use super::nosimd;
            #[cfg(all(target_arch = "x86", target_feature = "avx2"))]
            use core::arch::x86::*;
            #[cfg(all(target_arch = "x86_64", target_feature = "avx2"))]
            use core::arch::x86_64::*;

            #[allow(dead_code)]
            const BYTES_IN_AVX2_REGISTER: usize = 256 / 8;

            #find_byte_sequence_dispatch_source
            #find_byte_sequence_sources
            #find_long_byte_sequence_source
        }
    }
}

fn get_find_byte_sequence_dispatch_source() -> TokenStream {
    let match_body = (1..MAX_SEQUENCE_LENGTH_FOR_SIMD).map(|i| {
        let i = i + 1;
        let ident = format_ident!("find_byte_sequence{}", i);
        quote! {#i => unsafe { #ident(sequence, bytes) }}
    });

    quote! {
        /// Find the first occurence of a continuous byte sequence in the slice, if it exists.
        ///
        /// This is a SIMD version, if the target CPU is not x86/x86_64 or does not
        /// support AVX2 this will fallback to [`nosimd::find_byte_sequence`].
        /// # Examples
        /// ```
        /// # use simdpath::bytes::simd::find_byte_sequence;
        /// let bytes = "abcdefgh".as_bytes();
        /// let result = find_byte_sequence("de".as_bytes(), bytes);
        ///
        /// assert_eq!(Some(3), result);
        /// ```
        ///
        /// ```
        /// # use simdpath::bytes::simd::find_byte_sequence;
        /// let bytes = "abcdefgh".as_bytes();
        /// let result = find_byte_sequence("ed".as_bytes(), bytes);
        ///
        /// assert_eq!(None, result);
        /// ```
        #[inline]
        pub fn find_byte_sequence(sequence: &[u8], bytes: &[u8]) -> Option<usize> {
            #[cfg(target_feature = "avx2")]
            {
                if bytes.len() < BYTES_IN_AVX2_REGISTER * 2 {
                    return nosimd::find_byte_sequence(sequence, bytes);
                }

                match sequence.len() {
                    0 => panic!("Cannot look for an empty sequence."),
                    1 => crate::bytes::find_byte(sequence[0], bytes),
                    #(#match_body,)*
                    _ => unsafe { find_long_byte_sequence(sequence, bytes) }
                }
            }

            #[cfg(not(target_feature = "avx2"))]
            nosimd::find_byte_sequence(sequence, bytes)
        }
    }
}

fn get_find_byte_sequence_sources() -> TokenStream {
    let sources = (1..MAX_SEQUENCE_LENGTH_FOR_SIMD).map(|i| get_find_byte_sequence_source(i + 1));

    quote! {
        #(#sources)*
    }
}

fn get_nosimd_find_byte_sequence_source() -> TokenStream {
    let match_body = (1..MAX_SEQUENCE_LENGTH_FOR_NOSIMD).map(|i| {
        let i = i + 1;
        quote! {#i => bytes.windows(#i).position(|x| x == sequence)}
    });

    quote! {
        ///  Find the first occurence of a continuous byte sequence in the slice, if it exists.
        ///
        /// This is a sequential, no-SIMD version. For big slices it is recommended to use
        /// the [`simd::find_byte_sequence`](`super::simd::find_byte_sequence`) variant for better performance.
        /// # Examples
        /// ```
        /// # use simdpath::bytes::nosimd::find_byte_sequence;
        /// let bytes = "abcdefgh".as_bytes();
        /// let result = find_byte_sequence("de".as_bytes(), bytes);
        ///
        /// assert_eq!(Some(3), result);
        /// ```
        ///
        /// ```
        /// # use simdpath::bytes::nosimd::find_byte_sequence;
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
                1 => crate::bytes::find_byte(sequence[0], bytes),
                #(#match_body,)*
                _ => bytes.windows(sequence.len()).position(|x| x == sequence)
            }
        }
    }
}

fn get_find_byte_sequence_source(n: usize) -> TokenStream {
    let ident = format_ident!("find_byte_sequence{}", n);
    let mask_idents: Vec<_> = (0..n).map(|i| format_ident! {"mask{}", i + 1}).collect();
    let cmp_mask_first_block_vector_idents: Vec<_> = (0..n)
        .map(|i| format_ident! {"cmp_mask{}_first_block_vector", i + 1})
        .collect();
    let cmp_mask_first_block_idents: Vec<_> = (0..n)
        .map(|i| format_ident! {"cmp_mask{}_first_block", i + 1})
        .collect();
    let cmp_mask_next_block_vector_idents: Vec<_> = (0..n)
        .map(|i| format_ident! {"cmp_mask{}_next_block_vector", i + 1})
        .collect();
    let cmp_mask_next_block_idents: Vec<_> = (0..n)
        .map(|i| format_ident! {"cmp_mask{}_next_block", i + 1})
        .collect();
    let cmp_mask_idents: Vec<_> = (0..n)
        .map(|i| format_ident! {"cmp_mask{}", i + 1})
        .collect();

    let declarations = (0..n).map(|i| {
        let mask_ident = &mask_idents[i];
        let cmp_mask_first_block_vector_ident = &cmp_mask_first_block_vector_idents[i];
        let cmp_mask_first_block_ident = &cmp_mask_first_block_idents[i];
        quote! {
            let #mask_ident = _mm256_set1_epi8(sequence[#i] as i8);
            let #cmp_mask_first_block_vector_ident = _mm256_cmpeq_epi8(first_block, #mask_ident);
            let mut #cmp_mask_first_block_ident = _mm256_movemask_epi8(#cmp_mask_first_block_vector_ident) as u32;
        }
    });

    let mask_calculations = (0..n).map(|i| {
        let mask_ident = &mask_idents[i];
        let cmp_mask_first_block_ident = &cmp_mask_first_block_idents[i];
        let cmp_mask_next_block_vector_ident = &cmp_mask_next_block_vector_idents[i];
        let cmp_mask_next_block_ident = &cmp_mask_next_block_idents[i];
        let cmp_mask_ident = &cmp_mask_idents[i];

        let mask_computation = if i > 0 {
            quote! {
                let #cmp_mask_ident = ((#cmp_mask_first_block_ident as u64) | ((#cmp_mask_next_block_ident as u64) << 32)) >> #i;
            }
        } else {
            quote! {
                let #cmp_mask_ident = (#cmp_mask_first_block_ident as u64) | ((#cmp_mask_next_block_ident as u64) << 32);
            }
        };

        quote! {
            let #cmp_mask_next_block_vector_ident = _mm256_cmpeq_epi8(next_block, #mask_ident);
            let #cmp_mask_next_block_ident = _mm256_movemask_epi8(#cmp_mask_next_block_vector_ident) as u32;
            #mask_computation
        }
    });

    let cmp_and_tree = CmpAndTree::build_tree(cmp_mask_idents.clone());

    let advance_block = (0..n).map(|i| {
        let cmp_mask_first_block_ident = &cmp_mask_first_block_idents[i];
        let cmp_mask_next_block_ident = &cmp_mask_next_block_idents[i];
        quote! {
            #cmp_mask_first_block_ident = #cmp_mask_next_block_ident;
        }
    });

    let root_cmp_node_ident = cmp_and_tree.root_node_ident();
    let cmp_and_tree_instructions = cmp_and_tree.instructions();

    quote! {
        #[target_feature(enable = "avx2")]
        #[cfg(target_feature = "avx2")]
        #[allow(dead_code)]
        #[inline]
        unsafe fn #ident(sequence: &[u8], bytes: &[u8]) -> Option<usize> {
            debug_assert!(sequence.len() == #n);

            let mut bytes = bytes;
            let mut i: usize = 0;
            let first_block = _mm256_loadu_si256(bytes.as_ptr() as *const __m256i);
            #(#declarations)*

            while bytes.len() >= BYTES_IN_AVX2_REGISTER * 2 {
                let ptr = bytes.as_ptr() as *const __m256i;
                let next_block = _mm256_loadu_si256(ptr.offset(1));

                #(#mask_calculations)*
                #(#cmp_and_tree_instructions)*

                if #root_cmp_node_ident != 0 {
                    return Some(i + (#root_cmp_node_ident.trailing_zeros() as usize));
                }

                #(#advance_block)*
                i += BYTES_IN_AVX2_REGISTER;
                bytes = &bytes[BYTES_IN_AVX2_REGISTER..];
            }

            nosimd::find_byte_sequence(sequence, bytes)
        }
    }
}

fn get_find_long_byte_sequence_source() -> TokenStream {
    quote! {
        #[target_feature(enable = "avx2")]
        #[cfg(target_feature = "avx2")]
        #[allow(dead_code)]
        #[inline]
        unsafe fn find_long_byte_sequence(sequence : &[u8], bytes: &[u8]) -> Option<usize> {
            let mut bytes = bytes;
            let mut i = 0;

            while bytes.len() >= sequence.len() {
                let heuristic_match = find_byte_sequence32(&sequence[..32], bytes);

                if let Some(j) = heuristic_match {
                    if (&bytes[j + 32..]).starts_with(&sequence[32..]) {
                        return Some(i + j);
                    }
                    bytes = &bytes[j + 1..];
                    i += j + 1;
                } else {
                    return None;
                }
            }

            None
        }
    }
}

struct CmpAndTree {
    instructions: Vec<TokenStream>,
    root_node_ident: Option<Ident>,
    next_node_id: usize,
    nodes: VecDeque<Ident>,
}

impl CmpAndTree {
    pub fn build_tree(leaves: Vec<Ident>) -> CmpAndTree {
        assert!(!leaves.is_empty());

        let mut tree = CmpAndTree {
            instructions: vec![],
            root_node_ident: None,
            next_node_id: 1,
            nodes: leaves.into(),
        };

        while tree.nodes.len() > 1 {
            tree.combine_nodes_once();
        }

        tree.root_node_ident = Some(tree.nodes[0].clone());

        tree
    }

    pub fn root_node_ident(&self) -> Ident {
        self.root_node_ident.clone().unwrap()
    }

    pub fn instructions(&self) -> &[TokenStream] {
        &self.instructions
    }

    fn combine_nodes_once(&mut self) {
        debug_assert!(self.nodes.len() > 1);

        let new_node = format_ident!("cmp{}", self.next_node_id);
        self.next_node_id += 1;

        let node1 = self.nodes.pop_front();
        let node2 = self.nodes.pop_front();

        let instruction = quote! {
            let #new_node = #node1 & #node2;
        };

        self.instructions.push(instruction);
        self.nodes.push_back(new_node);
    }
}
