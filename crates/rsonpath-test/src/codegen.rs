use crate::model;
use proc_macro2::{Ident, TokenStream};
use quote::{format_ident, quote};
use std::fmt::Display;

pub struct TestSet {
    documents: Vec<model::NamedDocument>,
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
        }
    }

    pub fn generate_test_fns(&self) -> Vec<TokenStream> {
        let mut fns = vec![];
        for named_doc in &self.documents {
            for query in &named_doc.document.queries {
                for input_type in [InputTypeToTest::Owned, InputTypeToTest::Buffered, InputTypeToTest::Mmap] {
                    for result_type in [ResultTypeToTest::Count, ResultTypeToTest::Bytes] {
                        for engine_type in [EngineTypeToTest::Main, EngineTypeToTest::Recursive] {
                            let fn_name = format_ident!(
                                "{}",
                                heck::AsSnakeCase(format!(
                                    "{}_with_query_{}_with_{}_and_{}_using_{}",
                                    named_doc.document.input.description,
                                    query.description,
                                    input_type,
                                    result_type,
                                    engine_type
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

        fn generate_body(
            full_description: String,
            input: &model::Input,
            query: &model::Query,
            input_type: InputTypeToTest,
            result_type: ResultTypeToTest,
            engine_type: EngineTypeToTest,
        ) -> TokenStream {
            let query_ident = format_ident!("jsonpath_query");
            let query_string = &query.query;
            let (input_ident, input_setup_code) = generate_input_setup(input, input_type);
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
                InputTypeToTest::Buffered => {
                    quote! {
                        let read_string = ReadString::new(#raw_input.to_string());
                        let #ident = BufferedInput::new(read_string);
                    }
                }
                InputTypeToTest::Mmap => {
                    quote! {
                        let tmp_file = mmap_tmp_file::create_with_contents(#raw_input)?;
                        let #ident = unsafe { MmapInput::map_file(&tmp_file)? };
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
        use std::cmp;
        use std::error::Error;
        use std::io::Read;
    }
}

pub fn generate_helper_types() -> TokenStream {
    quote! {        
        struct ReadString(String, usize);

        impl ReadString {
            fn new(string: String) -> Self {
                Self(string, 0)
            }
        }

        impl Read for ReadString {
            fn read(&mut self, buf: &mut [u8]) -> std::io::Result<usize> {
                let rem = self.0.as_bytes().len() - self.1;
                if rem > 0 {
                    let size = cmp::min(1024, rem);
                    buf[..size].copy_from_slice(&self.0.as_bytes()[self.1..self.1 + size]);
                    self.1 += size;
                    Ok(size)
                } else {
                    Ok(0)
                }
            }
        }

        mod mmap_tmp_file {
            use std::fs::File;
            use std::io::{Seek, SeekFrom, Write};

            pub(super) fn create_with_contents(contents: &str) -> std::io::Result<File> {
                let mut tmpfile = tempfile::tempfile()?;

                write!(tmpfile, "{}", contents)?;
                tmpfile.seek(SeekFrom::Start(0))?;

                Ok(tmpfile)       
            }
        }
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
