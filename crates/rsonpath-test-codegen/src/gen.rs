use crate::discovery::DiscoveredDocument;
use crate::model;
use proc_macro2::{Ident, TokenStream};
use quote::{format_ident, quote};
use std::{
    fmt::Display,
    path::{Path, PathBuf},
};

pub struct TestSet {
    documents: Vec<DiscoveredDocument>,
}

pub struct Stats {
    total_documents: usize,
    total_queries: usize,
}

impl Stats {
    pub fn number_of_documents(&self) -> usize {
        self.total_documents
    }

    pub fn number_of_queries(&self) -> usize {
        self.total_queries
    }
}

impl TestSet {
    pub(crate) fn new<I>(documents: I) -> TestSet
    where
        I: IntoIterator<Item = DiscoveredDocument>,
    {
        let documents: Vec<DiscoveredDocument> = documents.into_iter().collect();

        TestSet { documents }
    }

    pub fn stats(&self) -> Stats {
        let total_documents = self.documents.len();
        let total_queries = self.documents.iter().map(|x| x.document.queries.len()).sum();

        Stats {
            total_documents,
            total_queries,
        }
    }

    pub fn get_required_test_files<'a, P: AsRef<Path> + 'a>(
        &'a self,
        target_dir: P,
    ) -> impl IntoIterator<Item = (PathBuf, &'a str)> {
        self.documents.iter().map(move |d| {
            let new_path = Self::get_path_of_json_for_document(&target_dir, d);
            let contents: &'a str = &d.document.input.json;

            (new_path, contents)
        })
    }

    fn get_path_of_json_for_document<P: AsRef<Path>>(dir: P, document: &DiscoveredDocument) -> PathBuf {
        let file_name = document
            .path
            .file_name()
            .expect("all documents should have a file path");
        let mut new_path = dir.as_ref().to_path_buf();
        new_path.push(file_name);
        new_path.set_extension("json");

        new_path
    }

    pub fn generate_test_fns<P: AsRef<Path>>(&self, json_files_dir: P) -> Vec<TokenStream> {
        let mut fns = vec![];
        for discovered_doc in &self.documents {
            let input_json = Self::get_path_of_json_for_document(&json_files_dir, discovered_doc);
            for query in &discovered_doc.document.queries {
                for input_type in [InputTypeToTest::Owned, InputTypeToTest::Buffered, InputTypeToTest::Mmap] {
                    for result_type in [ResultTypeToTest::Count, ResultTypeToTest::Bytes] {
                        for engine_type in [EngineTypeToTest::Main, EngineTypeToTest::Recursive] {
                            let fn_name = format_ident!(
                                "{}",
                                heck::AsSnakeCase(format!(
                                    "{}_with_query_{}_with_{}_and_{}_using_{}",
                                    discovered_doc.document.input.description,
                                    query.description,
                                    input_type,
                                    result_type,
                                    engine_type
                                ))
                                .to_string()
                            );
                            let full_description = format!(
                                r#"on document {} running the query {} ({}) with Input impl {} and result mode {}"#,
                                discovered_doc.name, query.query, query.description, input_type, result_type
                            );
                            let body = generate_body(
                                full_description,
                                &input_json,
                                query,
                                input_type,
                                result_type,
                                engine_type,
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
        }

        return fns;

        fn generate_body<P: AsRef<Path>>(
            full_description: String,
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
                EngineTypeToTest::Recursive => {
                    quote! {
                        let #ident = RecursiveEngine::compile_query(&#query_ident)?;
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
                        let result = #engine_ident.run::<_, CountResult>(&#input_ident)?;

                        assert_eq!(result.get(), #count, "result != expected");
                    }
                }
                ResultTypeToTest::Bytes => {
                    let bytes = &query.results.bytes;
                    quote! {
                        let result = #engine_ident.run::<_, IndexResult>(&#input_ident)?;

                        assert_eq!(result.get(), vec![#(#bytes,)*], "result != expected");
                    }
                }
            }
        }
    }
}

pub fn generate_imports() -> TokenStream {
    quote! {
        use rsonpath::engine::{Compiler, Engine, main::MainEngine, recursive::RecursiveEngine};
        use rsonpath::input::*;
        use rsonpath::query::JsonPathQuery;
        use rsonpath::result::*;
        use pretty_assertions::assert_eq;
        use std::error::Error;
        use std::fs;
    }
}

#[derive(Clone, Copy)]
enum InputTypeToTest {
    Owned,
    Buffered,
    Mmap,
}

#[derive(Clone, Copy)]
enum ResultTypeToTest {
    Count,
    Bytes,
}

#[derive(Clone, Copy)]
enum EngineTypeToTest {
    Main,
    Recursive,
}

impl Display for InputTypeToTest {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match self {
                InputTypeToTest::Owned => "OwnedBytes",
                InputTypeToTest::Buffered => "BufferedInput",
                InputTypeToTest::Mmap => "MmapInput",
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

impl Display for EngineTypeToTest {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match self {
                EngineTypeToTest::Main => "MainEngine",
                EngineTypeToTest::Recursive => "RecursiveEngine",
            }
        )
    }
}
