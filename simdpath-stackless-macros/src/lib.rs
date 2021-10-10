use proc_macro::TokenStream;
use quote::{format_ident, quote};
use std::result::Result;
use syn::{parse::*, *};

const MAX_AUTOMATON_SIZE: u8 = 32;

#[proc_macro]
pub fn assert_supported_size(tokens: TokenStream) -> TokenStream {
    let value = parse_macro_input!(tokens as Expr);
    let assertion = quote! {
        let value = #value;
        let value_u8 = std::convert::TryInto::<u8>::try_into(value);
        assert!(value_u8.map_or(false, |x| x <= #MAX_AUTOMATON_SIZE),
            "Max supported length of a query for StacklessRunner is currently {}. The supplied query has length {}.",
            #MAX_AUTOMATON_SIZE,
            value);
    };

    assertion.into()
}

struct Arguments {
    labels_expr: Expr,
    bytes_expr: Expr,
}

impl Parse for Arguments {
    fn parse(tokens: ParseStream) -> Result<Arguments, syn::Error> {
        let labels_expr: Expr = tokens.parse()?;
        let _comma: Token![,] = tokens.parse()?;
        let bytes_expr: Expr = tokens.parse()?;
        Ok(Arguments {
            labels_expr,
            bytes_expr,
        })
    }
}

#[proc_macro]
pub fn create_descendant_only_automaton(tokens: TokenStream) -> TokenStream {
    let size_lit = parse_macro_input!(tokens as LitInt);
    let size: u8 = size_lit.base10_parse().unwrap();

    let fn_ident = format_ident!("descendant_only_automaton_{}", size);
    let reg_idents: Vec<_> = (1..size).map(|i| format_ident!("reg_{}", i)).collect();
    let reg_decls = reg_idents
        .iter()
        .map(|reg| quote! {let mut #reg : usize = 0;});
    let states = (0..size).map(|i| {
        let closing_code = if i == 0 {
            quote! {
                depth -= 1;
                bytes = &bytes[i + 1..];
            }
        } else {
            let reg = &reg_idents[(i - 1) as usize];
            quote! {
                depth -= 1;
                bytes = &bytes[i + 1..];
                if depth == #reg {
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
            let reg = &reg_idents[i as usize];
            quote! {
                if (bytes[next] == b'{' || bytes[next] == b'[') && label == labels[#i as usize] {
                    state = #i + 1;
                    #reg = depth;
                    depth += 1;
                    bytes = &bytes[next + 1..];
                }
            }
        };

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
                }
                _ => {
                    bytes = &bytes[i + 1..];
                }
            }
        }
    });

    let automaton_code = quote! {
        pub fn #fn_ident<'a>(labels: &[&'a [u8]], bytes: &'a [u8]) -> usize {
            debug_assert_eq!(labels.len(), #size as usize);

            let mut bytes = bytes;
            let mut depth: usize = 0;
            let mut state: u8 = 0;
            let mut count: usize = 0;
            #(#reg_decls)*

            while let Some(i) = crate::bytes::find_non_whitespace(bytes) {
                match state {
                    #(#states,)*
                    _ => unreachable! {},
                }
            }

            count
        }
    };

    automaton_code.into()
}

#[proc_macro]
pub fn initialize(empty_input: TokenStream) -> TokenStream {
    parse_macro_input!(empty_input as Nothing);
    let tokens = (0..MAX_AUTOMATON_SIZE).map(|i| {
        let size = i + 1;
        quote! { create_descendant_only_automaton!(#size) }
    });

    (quote! {#(#tokens;)*}).into()
}

#[proc_macro]
pub fn dispatch_automaton(tokens: TokenStream) -> TokenStream {
    let arguments = parse_macro_input!(tokens as Arguments);
    let labels_expr = arguments.labels_expr;
    let bytes_expr = arguments.bytes_expr;
    let match_body = (0..MAX_AUTOMATON_SIZE)
        .map(|i| {
            (
                (i + 1) as usize,
                format_ident!("descendant_only_automaton_{}", i + 1),
            )
        })
        .map(|(i, ident)| quote! {#i => #ident(labels, bytes)});

    let tokens = quote! {
        {
            let labels = #labels_expr;
            let bytes = #bytes_expr;
            match labels.len() {
                #(#match_body,)*
                0 => 1,
                _ => unimplemented!("Max number of labels supported by dispatch_automaton! is {}, requested {}", #MAX_AUTOMATON_SIZE, labels.len())
            }
        }
    };

    tokens.into()
}
