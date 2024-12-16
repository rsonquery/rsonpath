//! Main codegen logic, creating all test functions from TOML documents.
use crate::{files::Files, model, DocumentName};
use proc_macro2::{Ident, TokenStream};
use quote::{format_ident, quote};
use std::{fmt::Display, io, path::Path};

pub(crate) struct DocumentTestGroup {
    pub(crate) name: DocumentName,
    pub(crate) query_test_groups: Vec<QueryTestGroup>,
}

pub(crate) struct QueryTestGroup {
    pub(crate) name: String,
    pub(crate) source: TokenStream,
}

/// Generate the source file and register all required files.
pub(crate) fn generate_test_fns(files: &mut Files) -> Result<(), io::Error> {
    // Clone the collection, since we need to mutate the files.
    let docs = files.documents().into_iter().cloned().collect::<Vec<_>>();
    let imports = generate_imports();
    let mut compressed_mod = vec![];
    let mut tests_mod = vec![];
    for discovered_doc in docs {
        // The input JSON either already exists when large_file is used, or needs to be generated if it is inline.
        let input_json = match &discovered_doc.document.input.source {
            model::InputSource::LargeFile(f) => files.get_json_source_path(f),
            model::InputSource::JsonString(contents) => files.add_json_source(&discovered_doc, contents.clone()),
        };
        // For each query generate cases using each combination of input mode, result type, and engine type.
        let mut query_test_groups = vec![];
        let mut group_mod = vec![];
        for query in &discovered_doc.document.queries {
            let mut fns = vec![];
            for input_type in [
                InputTypeToTest::Borrowed,
                InputTypeToTest::Buffered,
                InputTypeToTest::Mmap,
            ] {
                for result_type in get_available_results(&discovered_doc.document.input.source, query)? {
                    let fn_name = format_ident!(
                        "{}",
                        heck::AsSnakeCase(format!(
                            "{}_with_query_{}_with_{}_and_{}_using_{}",
                            discovered_doc.document.input.description,
                            query.description,
                            input_type,
                            result_type,
                            EngineTypeToTest::Main,
                        ))
                        .to_string()
                    );
                    let full_description = format!(
                        r#"on document {} running the query {} ({}) with Input impl {} and result mode {}"#,
                        escape_format(&discovered_doc.name.simple_name()),
                        escape_format(&query.query),
                        escape_format(&query.description),
                        escape_format(&input_type),
                        escape_format(&result_type)
                    );
                    let body = generate_body(
                        &full_description,
                        &input_json,
                        query,
                        input_type,
                        result_type,
                        EngineTypeToTest::Main,
                    );
                    let mb_ignore = if let Some(disabled) = &query.disabled {
                        let reason = format!("{} (see {})", disabled.reason, disabled.issue);
                        quote! {
                            #[doc = #reason]
                            #[ignore]
                        }
                    } else {
                        quote! {}
                    };

                    let r#fn = quote! {
                        #mb_ignore
                        #[test]
                        fn #fn_name() -> Result<(), Box<dyn Error>> {
                            #body
                        }
                    };

                    fns.push((fn_name, r#fn));
                }
            }
            fns.sort_by(|x, y| x.0.cmp(&y.0));
            let sources = fns.into_iter().map(|x| x.1);
            let src = quote! {
                #imports

                #(#sources)*
            };
            let name = format_ident!("{}", escape_test_name(&query.description).to_string());
            group_mod.push(quote! {
                mod #name;
            });
            query_test_groups.push(QueryTestGroup {
                name: escape_test_name(&query.description).to_string(),
                source: src,
            });
        }

        let doc_name = &discovered_doc.name;
        let mod_name = format_ident!("{}", doc_name.simple_name());
        let mod_tokens = quote! {
            mod #mod_name {
                #(#group_mod)*
            }
        };

        if doc_name.is_compressed() {
            compressed_mod.push(mod_tokens);
        } else {
            tests_mod.push(mod_tokens);
        }

        files.add_test_group(&DocumentTestGroup {
            name: discovered_doc.name,
            query_test_groups,
        });
    }

    tests_mod.push(quote! {
        mod compressed {
            #(#compressed_mod)*
        }
    });
    let tests_source = quote! {
        #![allow(non_snake_case)]
        #(#tests_mod)*
    };

    files.add_rust_file("mod.rs", &tests_source);

    return Ok(());

    fn generate_body<P: AsRef<Path>>(
        full_description: &str,
        input_json_path: P,
        query: &model::Query,
        input_type: InputTypeToTest,
        result_type: ResultTypeToTest,
        engine_type: EngineTypeToTest,
    ) -> TokenStream {
        let query_ident = format_ident!("jsonpath_query");
        let query_string = &query.query;
        let (input_ident, input_setup_code) = generate_input_setup(input_json_path, input_type);
        let (engine_ident, engine_setup_code) = generate_engine_setup(engine_type, &query_ident);
        let run_and_diff_code = generate_run_and_diff_code(query, result_type, &engine_ident, &input_ident);

        quote! {
            println!(#full_description);
            let jsonpath_query = rsonpath_syntax::parse(#query_string)?;

            #input_setup_code
            #engine_setup_code

            #run_and_diff_code

            Ok(())
        }
    }

    fn generate_input_setup<P: AsRef<Path>>(input_path: P, input_type: InputTypeToTest) -> (Ident, TokenStream) {
        let ident = format_ident!("input");
        let raw_input_path = input_path.as_ref().to_str().expect("supported unicode path");

        let code = match input_type {
            InputTypeToTest::Borrowed => {
                quote! {
                    let raw_json = fs::read_to_string(#raw_input_path)?;
                    let #ident = BorrowedBytes::new(raw_json.as_bytes());
                }
            }
            InputTypeToTest::Buffered => {
                quote! {
                    let json_file = fs::File::open(#raw_input_path)?;
                    let #ident = BufferedInput::new(json_file);
                }
            }
            InputTypeToTest::Mmap => {
                quote! {
                    let json_file = fs::File::open(#raw_input_path)?;
                    let #ident = unsafe { MmapInput::map_file(&json_file)? };
                }
            }
        };

        (ident, code)
    }

    fn generate_engine_setup(engine_type: EngineTypeToTest, query_ident: &Ident) -> (Ident, TokenStream) {
        let ident = format_ident!("engine");

        let code = match engine_type {
            EngineTypeToTest::Main => {
                quote! {
                    let #ident = MainEngine::compile_query(&#query_ident)?;
                }
            }
        };

        (ident, code)
    }

    fn generate_run_and_diff_code(
        query: &model::Query,
        result_type: ResultTypeToTest,
        engine_ident: &Ident,
        input_ident: &Ident,
    ) -> TokenStream {
        match result_type {
            ResultTypeToTest::Count => {
                let count = query.results.count;
                quote! {
                    let result = #engine_ident.count(&#input_ident)?;

                    assert_eq!(result, #count, "result != expected");
                }
            }
            ResultTypeToTest::Indices => {
                let indices = query
                    .results
                    .spans
                    .as_ref()
                    .expect("result without data in toml should be filtered out in get_available_results")
                    .iter()
                    .map(|x| x.start);
                quote! {
                    let mut result = vec![];
                    #engine_ident.indices(&#input_ident, &mut result)?;

                    let expected: Vec<usize> = vec![#(#indices,)*];
                    assert_eq!(result, expected, "result != expected");
                }
            }
            ResultTypeToTest::ApproximateSpans(spans) => {
                quote! {
                    let mut result = vec![];
                    #engine_ident.approximate_spans(&#input_ident, &mut result)?;

                    let tups: Vec<(usize, usize)> = result.iter().map(|x| (x.start_idx(), x.end_idx())).collect();
                    let expected: Vec<(usize, usize, Option<usize>)> = vec![#(#spans,)*];

                    assert_eq!(tups.len(), expected.len(), "result.len() != expected.len()");

                    for i in 0..tups.len() {
                        let upper_bound = expected[i];
                        let actual = tups[i];

                        assert_eq!(actual.0, upper_bound.0, "result start_idx() != expected start_idx()");
                        assert!(actual.1 >= upper_bound.1, "result end_idx() < expected end_lower_bound ({} < {})", actual.1, upper_bound.1);

                        if let Some(end_upper_bound) = upper_bound.2 {
                            assert!(actual.1 <= end_upper_bound, "result end_idx() > expected end_upper_bound ({} > {}", actual.1, end_upper_bound);
                        }
                    }
                }
            }
            ResultTypeToTest::Spans => {
                let spans = query
                    .results
                    .spans
                    .as_ref()
                    .expect("result without data in toml should be filtered out in get_available_results");
                quote! {
                    let mut result = vec![];
                    #engine_ident.matches(&#input_ident, &mut result)?;

                    let tups: Vec<(usize, usize)> = result.iter().map(|x| (x.span().start_idx(), x.span().end_idx())).collect();
                    let expected: Vec<(usize, usize)> = vec![#(#spans,)*];

                    assert_eq!(tups, expected, "result != expected");
                }
            }
            ResultTypeToTest::Nodes => {
                let node_strings = query
                    .results
                    .nodes
                    .as_ref()
                    .expect("result without data in toml should be filtered out in get_available_results");
                quote! {
                    let mut result = vec![];
                    #engine_ident.matches(&#input_ident, &mut result)?;

                    let utf8: Result<Vec<&str>, _> = result.iter().map(|x| str::from_utf8(x.bytes())).collect();
                    let utf8 = utf8.expect("valid utf8");
                    let expected: Vec<&str> = vec![#(#node_strings,)*];

                    assert_eq!(utf8, expected, "result != expected");
                }
            }
        }
    }
}

fn get_available_results(input: &model::InputSource, query: &model::Query) -> Result<Vec<ResultTypeToTest>, io::Error> {
    let mut res = vec![ResultTypeToTest::Count];

    if let Some(spans) = &query.results.spans {
        res.push(ResultTypeToTest::Indices);
        res.push(ResultTypeToTest::Spans);
        match input {
            model::InputSource::LargeFile(_) => (),
            model::InputSource::JsonString(s) => res.push(generate_approximate_spans_result(s, spans)?),
        }
    }

    if query.results.nodes.is_some() {
        res.push(ResultTypeToTest::Nodes);
    }

    return Ok(res);

    fn generate_approximate_spans_result(
        contents: &str,
        spans: &[model::ResultSpan],
    ) -> Result<ResultTypeToTest, io::Error> {
        let b = contents.as_bytes();

        let approx_spans = spans
            .iter()
            .map(|span| {
                let mut end = span.end;

                while end < b.len() && (b[end] == b' ' || b[end] == b'\t' || b[end] == b'\n' || b[end] == b'\r') {
                    end += 1
                }

                let upper_bound = if end == b.len() { None } else { Some(end) };

                model::ResultApproximateSpan {
                    start: span.start,
                    end_lower_bound: span.end,
                    end_upper_bound: upper_bound,
                }
            })
            .collect();

        Ok(ResultTypeToTest::ApproximateSpans(approx_spans))
    }
}

pub(crate) fn generate_imports() -> TokenStream {
    quote! {
        use rsonpath_lib::engine::{Compiler, Engine, main::MainEngine};
        use rsonpath_lib::input::*;
        use pretty_assertions::assert_eq;
        use std::error::Error;
        use std::fs;
        #[allow(unused_imports)]
        use std::str;
    }
}

#[derive(Clone, Copy)]
enum InputTypeToTest {
    Borrowed,
    Buffered,
    Mmap,
}

#[derive(Clone)]
enum ResultTypeToTest {
    Indices,
    Spans,
    ApproximateSpans(Vec<model::ResultApproximateSpan>),
    Count,
    Nodes,
}

#[derive(Clone, Copy)]
enum EngineTypeToTest {
    Main,
}

impl Display for InputTypeToTest {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match self {
                Self::Borrowed => "BorrowedBytes",
                Self::Buffered => "BufferedInput",
                Self::Mmap => "MmapInput",
            }
        )
    }
}

impl Display for ResultTypeToTest {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match self {
                Self::Count => "CountResult",
                Self::Indices => "IndexResult",
                Self::ApproximateSpans(_) => "ApproxSpanResult",
                Self::Spans => "NodesResult(Span)",
                Self::Nodes => "NodesResult",
            }
        )
    }
}

impl Display for EngineTypeToTest {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match self {
                Self::Main => "MainEngine",
            }
        )
    }
}

fn escape_format<D>(val: &D) -> impl Display
where
    D: Display,
{
    let s = val.to_string();
    s.replace('{', "{{").replace('}', "}}")
}

fn escape_test_name<D>(val: &D) -> impl Display
where
    D: Display,
{
    let s = val.to_string();
    let mut res = String::new();
    let mut was_prev_underscore = false;

    for c in s.chars() {
        let c = if c == '_' || c.is_alphanumeric() { c } else { '_' };

        if c != '_' || !was_prev_underscore {
            res.push(c);
        }
        was_prev_underscore = c == '_';
    }

    res
}
