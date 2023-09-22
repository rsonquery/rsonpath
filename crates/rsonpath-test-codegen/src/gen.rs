//! Main codegen logic, creating all test functions from TOML documents.
use crate::{files::Files, model};
use proc_macro2::{Ident, TokenStream};
use quote::{format_ident, quote};
use std::{fmt::Display, io, path::Path};

/// Generate the source file and register all required files.
pub(crate) fn generate_test_fns(files: &mut Files) -> Result<impl IntoIterator<Item = TokenStream>, io::Error> {
    let mut fns = vec![];
    // Clone the collection, since we need to mutate the files.
    let docs = files.documents().into_iter().cloned().collect::<Vec<_>>();
    for discovered_doc in docs {
        // The input JSON either already exists when large_file is used, or needs to be generated if it is inline.
        let input_json = match &discovered_doc.document.input.source {
            model::InputSource::LargeFile(f) => files.get_json_source_path(f),
            model::InputSource::JsonString(contents) => files.add_json_source(&discovered_doc, contents.clone()),
        };
        // For each query generate cases using each combination of input mode, result type, and engine type.
        for query in &discovered_doc.document.queries {
            for input_type in [InputTypeToTest::Owned, InputTypeToTest::Buffered, InputTypeToTest::Mmap] {
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
                        escape_format(&discovered_doc.name),
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

                    let r#fn = quote! {
                        #[test]
                        fn #fn_name() -> Result<(), Box<dyn Error>> {
                            #body
                        }
                    };

                    fns.push((fn_name, r#fn));
                }
            }
        }
    }

    fns.sort_by(|x, y| x.0.cmp(&y.0));

    return Ok(fns.into_iter().map(|x| x.1));

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
            let jsonpath_query = JsonPathQuery::parse(#query_string)?;

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
            InputTypeToTest::Owned => {
                quote! {
                    let raw_json = fs::read_to_string(#raw_input_path)?;
                    let #ident = OwnedBytes::new(&raw_json.as_bytes())?;
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

                    assert_eq!(result, vec![#(#indices,)*], "result != expected");
                }
            }
            ResultTypeToTest::ApproximateSpans(spans) => {
                quote! {
                    let mut result = vec![];
                    #engine_ident.approximate_spans(&#input_ident, &mut result)?;

                    let tups: Vec<(usize, usize)> = result.iter().map(|x| (x.start_idx(), x.end_idx())).collect();
                    let expected: Vec<(usize, usize, usize)> = vec![#(#spans,)*];

                    assert_eq!(tups.len(), expected.len(), "result.len() != expected.len()");

                    for i in 0..tups.len() {
                        let upper_bound = expected[i];
                        let actual = tups[i];

                        assert_eq!(actual.0, upper_bound.0, "result start_idx() != expected start_idx()");
                        assert!(actual.1 >= upper_bound.1, "result end_idx() < expected end_lower_bound ({} < {})", actual.1, upper_bound.1);
                        assert!(actual.1 <= upper_bound.2, "result end_idx() > expected end_upper_bound ({} > {}", actual.1, upper_bound.2);
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

                if end == b.len() {
                    end = usize::MAX;
                }

                model::ResultApproximateSpan {
                    start: span.start,
                    end_lower_bound: span.end,
                    end_upper_bound: end,
                }
            })
            .collect();

        Ok(ResultTypeToTest::ApproximateSpans(approx_spans))
    }
}

pub(crate) fn generate_imports() -> TokenStream {
    quote! {
        use rsonpath::engine::{Compiler, Engine, main::MainEngine};
        use rsonpath::input::*;
        use rsonpath::query::JsonPathQuery;
        use pretty_assertions::assert_eq;
        use std::error::Error;
        use std::fs;
        use std::str;
    }
}

#[derive(Clone, Copy)]
enum InputTypeToTest {
    Owned,
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
                Self::Owned => "OwnedBytes",
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
