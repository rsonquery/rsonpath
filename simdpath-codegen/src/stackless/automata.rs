//! Code generation for `simdpath::stackless::automata`.
//!
//! Used by the `build.rs` script to generate the `dispatch_automaton` and `descendant_only_automaton_N`
//! functions.

use proc_macro2::TokenStream;
use quote::{format_ident, quote};

const MAX_AUTOMATON_SIZE: u8 = 32;

/// Get the source for the `simdpath::stackless::automata` module.
pub fn get_mod_source() -> TokenStream {
    let assert_supported_size_macro_source = get_assert_supported_size_macro_source();
    let dispatch_automaton_source = get_dispatch_automaton_source();
    let automaton_source = get_all_descendant_only_automaton_sources();
    quote! {
        #assert_supported_size_macro_source
        #dispatch_automaton_source
        #automaton_source
    }
}

fn get_assert_supported_size_macro_source() -> TokenStream {
    quote! {
        pub const MAX_AUTOMATON_SIZE: u8 = #MAX_AUTOMATON_SIZE;

        #[macro_export]
        #[doc(hidden)]
        macro_rules! __assert_supported_size {
            ($value:expr) => {
                let value = $value;
                let value_u8 = ::std::convert::TryInto::<u8>::try_into(value);
                assert!(value_u8.map_or(false, |x| x <= crate::stackless::automata::MAX_AUTOMATON_SIZE),
                    "Max supported length of a query for StacklessRunner is currently {}. The supplied query has length {}.",
                    crate::stackless::automata::MAX_AUTOMATON_SIZE,
                    value);
            };
        }
        #[doc(inline)]
        pub use __assert_supported_size as assert_supported_size;
    }
}

fn get_dispatch_automaton_source() -> TokenStream {
    let match_body = (0..MAX_AUTOMATON_SIZE).map(|i| {
        let i = (i + 1) as usize;
        let ident = format_ident!("descendant_only_automaton_{}", i);
        quote! {#i => #ident(labels, bytes)}
    });

    let tokens = quote! {
        pub fn dispatch_automaton(labels : &[&[u8]], bytes: &[u8]) -> usize {
            match labels.len() {
                #(#match_body,)*
                0 => 1,
                _ => unimplemented!("Max number of labels supported by dispatch_automaton! is {}, requested {}", #MAX_AUTOMATON_SIZE, labels.len())
            }
        }
    };

    tokens
}

fn get_all_descendant_only_automaton_sources() -> TokenStream {
    let sources = (0..MAX_AUTOMATON_SIZE).map(|i| get_descendant_only_automaton_source(i + 1));

    quote! {#(#sources)*}
}

fn get_descendant_only_automaton_source(size: u8) -> TokenStream {
    assert!(size <= MAX_AUTOMATON_SIZE);

    let fn_ident = format_ident!("descendant_only_automaton_{}", size);
    let states = (0..size).map(|i| {
        let closing_code = if i == 0 {
            quote! {
                depth -= 1;
                bytes = &bytes[i + 1..];
            }
        } else {
            let reg = (i - 1) as usize;
            quote! {
                depth -= 1;
                bytes = &bytes[i + 1..];
                if depth == regs[#reg] {
                    state = #i - 1;
                }
            }
        };
        let found_if = if i == size - 1 {
            quote! {
                if label == labels[#i as usize] {
                    count += 1;
                    bytes = &bytes[next..];
                }
            }
        } else {
            quote! {
                if (bytes[next] == b'{' || bytes[next] == b'[') && label == labels[#i as usize] {
                    state = #i + 1;
                    regs[#i as usize] = depth;
                    depth += 1;
                    bytes = &bytes[next + 1..];
                }
            }
        };

        let quote_match_body = quote! {
            bytes = &bytes[i + 1..];
            let closing_quote = crate::bytes::find_unescaped_byte(b'"', bytes).unwrap();

            let label = &bytes[..closing_quote];
            bytes = &bytes[closing_quote + 1..];
            let next = crate::bytes::find_non_whitespace(bytes).unwrap();

            if bytes[next] == b':' {
                bytes = &bytes[next + 1..];
                let next = crate::bytes::find_non_whitespace(bytes).unwrap();
                #found_if else {
                    bytes = &bytes[next..];
                }
            } else {
                bytes = &bytes[next..];
            }
        };

        if size == 1 {
            quote! {
                #i => match bytes[i] {
                    b'\\' => {
                        bytes = &bytes[i + 2..];
                    }
                    b'"' => {
                        #quote_match_body
                    }
                    _ => {
                        bytes = &bytes[i + 1..];
                    }
                }
            }
        } else {
            quote! {
                #i => match bytes[i] {
                    b'{' => {
                        depth += 1;
                        bytes = &bytes[i + 1..];
                    }
                    b'}' => {
                        #closing_code
                    }
                    b'[' => {
                        depth += 1;
                        bytes = &bytes[i + 1..];
                    }
                    b']' => {
                        #closing_code
                    }
                    b'\\' => {
                        bytes = &bytes[i + 2..];
                    }
                    b'"' => {
                        #quote_match_body
                    }
                    _ => {
                        bytes = &bytes[i + 1..];
                    }
                }
            }
        }
    });

    let depth_decl = if size == 1 {
        quote! {}
    } else {
        quote! {
            let mut depth: usize = 0;
        }
    };
    let state_decl = if size == 1 {
        quote! {
            let state: u8 = 0;
        }
    } else {
        quote! {
            let mut state: u8 = 0;
        }
    };

    let reg_decl = if size == 1 {
        quote! {}
    } else {
        quote! {
            let mut regs = [0usize; #size as usize];
        }
    };

    let automaton_code = quote! {
        fn #fn_ident<'a>(labels: &[&'a [u8]], bytes: &'a [u8]) -> usize {
            debug_assert_eq!(labels.len(), #size as usize);

            let mut bytes = bytes;
            #depth_decl
            #state_decl
            let mut count: usize = 0;
            #reg_decl

            while let Some(i) = crate::bytes::find_non_whitespace(bytes) {
                match state {
                    #(#states,)*
                    _ => unreachable! {},
                }
            }

            count
        }
    };

    automaton_code
}
