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
        use crate::bytes::align::{alignment, AlignedBytes};
        use crate::engine::{Input};
        use crate::query::{Label};

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
        quote! {#i => #ident(labels, input)}
    });

    let tokens = quote! {
        pub fn dispatch_automaton(labels : &[&Label], input: &Input) -> usize {
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

    let states = (0..size).map(|i| {
        let iusize = i as usize;
        if size == 1 {
            quote! {
                0 => match event {
                    Structural::Colon(idx) => {
                        let len = labels[0].len();
                        if idx >= len + 2 {
                            let opening_quote_idx = idx - len - 2;
                            let slice = &bytes[opening_quote_idx..idx];

                            if slice == labels[0].bytes_with_quotes() {
                                count += 1;
                            }
                        }
                    },
                    _ => ()
                }
            }
        } else {
            let closing_code = if i == 0 {
                quote! {
                    depth -= 1;
                }
            } else {
                let reg = (i - 1) as usize;
                let prev_state = i - 1;
                quote! {
                    depth -= 1;
                    if depth <= regs[#reg] {
                        state = #prev_state;
                    }
                }
            };
            let matching_code = if i == size - 1 {
                quote! {
                    let len = labels[#iusize].len();
                    if idx >= len + 2 {
                        let mut closing_quote_idx = idx - 1;
                        while bytes[closing_quote_idx] != b'"' {
                            closing_quote_idx -= 1;
                        }

                        let opening_quote_idx = closing_quote_idx - len - 1;
                        let slice = &bytes[opening_quote_idx..closing_quote_idx + 1];

                        if slice == labels[#iusize].bytes_with_quotes() {
                            count += 1;
                        }
                    }
                }
            } else {
                let next_state = i + 1;
                quote! {
                    //let next_event = block_event_source.peek();
                    //log::debug!("Next event: {:?}", next_event);
                    match block_event_source.peek() {
                        Some(Structural::Opening(_)) => {
                            let len = labels[#iusize].len();
                            if idx >= len + 2 {
                                let mut closing_quote_idx = idx - 1;
                                while bytes[closing_quote_idx] != b'"' {
                                    closing_quote_idx -= 1;
                                }

                                let opening_quote_idx = closing_quote_idx - len - 1;
                                let slice = &bytes[opening_quote_idx..closing_quote_idx + 1];

                                if slice == labels[#iusize].bytes_with_quotes() {
                                    state = #next_state;
                                    regs[#iusize] = depth;
                                }
                            }
                        }
                        _ => ()
                    }
                }
            };

            quote! {
                #i => match event {
                    Structural::Closing(_) => {
                        //log::debug!("Event: {:?}: Depth: {depth}, State: {state}, Count: {count} -> ", event);
                        #closing_code
                        //log::debug!("Depth: {depth}, State: {state}, Count: {count}");
                    }
                    Structural::Opening(_) => {
                        //log::debug!("Event: {:?}: Depth: {depth}, State: {state}, Count: {count} -> ", event);
                        depth += 1;
                        //log::debug!("Depth: {depth}, State: {state}, Count: {count}");
                    }
                    Structural::Colon(idx) => {
                        //log::debug!("Event: {:?}: Depth: {depth}, State: {state}, Count: {count} -> ", event);
                        #matching_code
                        //log::debug!("Depth: {depth}, State: {state}, Count: {count}");
                    }
                }
            }
        }
    });

    let automaton_code = quote! {
        fn #fn_ident(labels: &[&Label], bytes: &AlignedBytes<alignment::Page>) -> usize {
            use crate::bytes::{classify_structural_characters, Structural};

            debug_assert_eq!(labels.len(), #size as usize);

            #depth_decl
            #state_decl
            let mut count: usize = 0;
            #reg_decl

            let mut block_event_source = classify_structural_characters(bytes).peekable();

            while let Some(event) = block_event_source.next() {
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
