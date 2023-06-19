use crate::model;
use proc_macro2::{Ident, TokenStream};
use quote::{format_ident, quote};
use std::{error::Error, fmt::Display};

pub fn generate_imports() -> TokenStream {
    quote! {
        use rsonpath::engine::{Compiler, main::MainEngine, Engine};
        use rsonpath::input::*;
        use rsonpath::query::JsonPathQuery;
        use rsonpath::result::*;
        use pretty_assertions::assert_eq;
        use std::error::Error;
    }
}

pub struct TestSet {
    documents: Vec<model::NamedDocument>,
}

pub struct Stats {
    total_documents: usize,
    total_queries: usize,
    result_types: usize,
    input_types: usize,
    engine_types: usize,
}

impl Stats {
    pub fn number_of_documents(&self) -> usize {
        self.total_documents
    }

    pub fn number_of_queries(&self) -> usize {
        self.total_queries
    }

    pub fn number_of_test_runs(&self) -> usize {
        self.total_queries * self.result_types * self.input_types * self.engine_types
    }
}

impl TestSet {
    pub fn new<I>(documents: I) -> TestSet
    where
        I: IntoIterator<Item = model::NamedDocument>,
    {
        let documents: Vec<model::NamedDocument> = documents.into_iter().collect();

        TestSet { documents }
    }

    pub fn stats(&self) -> Stats {
        let total_documents = self.documents.len();
        let total_queries = self.documents.iter().map(|x| x.document.queries.len()).sum();

        Stats {
            total_documents,
            total_queries,
            result_types: 2,
            input_types: 1,
            engine_types: 1,
        }
    }

    pub fn generate_test_fns(&self) -> Vec<TokenStream> {
        let mut fns = vec![];
        for named_doc in &self.documents {
            for query in &named_doc.document.queries {
                for input_type in [InputTypeToTest::Owned] {
                    for result_type in [ResultTypeToTest::Count, ResultTypeToTest::Bytes] {
                        let fn_name = format_ident!(
                            "{}",
                            heck::AsSnakeCase(format!(
                                "document_{}_query_{}_input_{}_result_{}",
                                named_doc.document.input.description, query.description, input_type, result_type
                            ))
                            .to_string()
                        );
                        let full_description = format!(
                            r#"on document {} running the query {} ({}) with Input impl {} and result mode {}"#,
                            named_doc.name, query.query, query.description, input_type, result_type
                        );
                        let body = generate_body(
                            full_description,
                            &named_doc.document.input,
                            query,
                            input_type,
                            result_type,
                        );

                        let r#fn = quote! {
                            #[test]
                            fn #fn_name() -> Result<(), Box<dyn Error>> {
                                #body
                            }
                        };

                        fns.push(r#fn);
                    }
                }
            }
        }

        return fns;

        fn generate_body(
            full_description: String,
            input: &model::Input,
            query: &model::Query,
            input_type: InputTypeToTest,
            result_type: ResultTypeToTest,
        ) -> TokenStream {
            let query_ident = format_ident!("jsonpath_query");
            let query_string = &query.query;
            let (input_ident, input_setup_code) = generate_input_setup(input, input_type);
            let run_and_diff_code = generate_run_and_diff_code(query, result_type, query_ident, input_ident);

            quote! {
                println!(#full_description);
                let jsonpath_query = JsonPathQuery::parse(#query_string)?;

                #input_setup_code

                #run_and_diff_code

                Ok(())
            }
        }

        fn generate_input_setup(input: &model::Input, input_type: InputTypeToTest) -> (Ident, TokenStream) {
            let ident = format_ident!("input");
            let raw_input = &input.json;

            let code = match input_type {
                InputTypeToTest::Owned => {
                    quote! {
                        let raw_json = #raw_input;
                        let #ident = OwnedBytes::new(&raw_json.as_bytes())?;
                    }
                }
            };

            (ident, code)
        }

        fn generate_run_and_diff_code(
            query: &model::Query,
            result_type: ResultTypeToTest,
            query_ident: Ident,
            input_ident: Ident,
        ) -> TokenStream {
            let run_code = match result_type {
                ResultTypeToTest::Count => {
                    let count = query.results.count as usize;
                    quote! {
                        let result = engine.run::<_, CountResult>(&#input_ident)?;

                        assert_eq!(result.get(), #count);
                    }
                }
                ResultTypeToTest::Bytes => {
                    let bytes = &query.results.bytes;
                    quote! {
                        let result = engine.run::<_, IndexResult>(&#input_ident)?;

                        assert_eq!(result.get(), vec![#(#bytes,)*]);
                    }
                }
            };

            quote! {
                let engine = MainEngine::compile_query(&#query_ident)?;

                #run_code
            }
        }

        /*
        fn generate_body(
            input: &model::Input,
            query: &model::Query,
            input_type: InputTypeToTest,
            result_type: ResultTypeToTest,
        ) -> TokenStream {
            use crate::diff::*;
            use rsonpath::engine::{main::MainEngine, Engine};
            use rsonpath::input::*;
            use rsonpath::query::JsonPathQuery;
            use rsonpath::result::*;

            let jsonpath_query = JsonPathQuery::parse(&query.query)?;

            return match input_type {
                InputTypeToTest::Owned => {
                    let owned_input = OwnedBytes::new(&input.json)?;
                    run_with_input(query, &jsonpath_query, owned_input, result_type)
                }
            };

            fn run_with_input<I: Input>(
                query: &model::Query,
                parsed_query: &JsonPathQuery,
                input: I,
                result_type: ResultTypeToTest,
            ) -> Result<Option<Diff>, Box<dyn Error>> {
                match result_type {
                    ResultTypeToTest::Count => {
                        let expected_result = ExpectCount::new(query.results.count);
                        run_with_input_and_result::<_, CountResult, _>(parsed_query, input, expected_result)
                    }
                    ResultTypeToTest::Bytes => {
                        let expected_result = ExpectBytes::new(&query.results.bytes);
                        run_with_input_and_result::<_, IndexResult, _>(parsed_query, input, expected_result)
                    }
                }
            }

            fn run_with_input_and_result<I: Input, R: QueryResult, E: Expect<R>>(
                query: &JsonPathQuery,
                input: I,
                expected: E,
            ) -> Result<Option<Diff>, Box<dyn Error>> {
                let engine = MainEngine::compile_query(query)?;

                let result = engine.run::<I, R>(&input)?;

                Ok(expected.diff(&result))
            }
        }*/
    }
}

#[derive(Clone, Copy)]
enum InputTypeToTest {
    Owned,
}

enum ResultTypeToTest {
    Count,
    Bytes,
}

impl Display for InputTypeToTest {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match self {
                InputTypeToTest::Owned => "OwnedInput",
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
                ResultTypeToTest::Count => "CountResult",
                ResultTypeToTest::Bytes => "IndexResult",
            }
        )
    }
}
