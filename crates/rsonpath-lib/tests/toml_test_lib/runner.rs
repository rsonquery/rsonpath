use crate::diff::Diff;
use crate::model;
use rsonpath::engine::Compiler;
use std::{
    error::Error,
    fmt::{Debug, Display},
};

pub struct TestSet {
    cases: Vec<model::NamedCase>,
}

pub struct Stats {
    total_cases: usize,
    total_queries: usize,
    result_types: usize,
    input_types: usize,
    engine_types: usize,
}

impl Stats {
    pub fn number_of_cases(&self) -> usize {
        self.total_cases
    }

    pub fn number_of_queries(&self) -> usize {
        self.total_queries
    }

    pub fn number_of_test_runs(&self) -> usize {
        self.total_queries * self.result_types * self.input_types * self.engine_types
    }
}

impl TestSet {
    pub fn new<I: IntoIterator<Item = model::NamedCase>>(cases: I) -> TestSet {
        let cases: Vec<model::NamedCase> = cases.into_iter().collect();

        TestSet { cases }
    }

    pub fn stats(&self) -> Stats {
        let total_cases = self.cases.len();
        let total_queries = self.cases.iter().map(|x| x.case.queries.len()).sum();

        Stats {
            total_cases,
            total_queries,
            result_types: 2,
            input_types: 1,
            engine_types: 1,
        }
    }

    pub fn run(&self) -> SuiteResult {
        let mut failed = vec![];
        for test in &self.cases {
            for query in &test.case.queries {
                for input_type in [InputTypeToTest::Owned] {
                    for result_type in [ResultTypeToTest::Count, ResultTypeToTest::Bytes] {
                        let name = format!("");
                        let res =
                            std::panic::catch_unwind(|| run_one(&test.case.input, query, input_type, result_type));

                        match res {
                            Err(panic) => {
                                let panic_msg = format_panic(panic);
                                let failure = TestCaseFailure {
                                    name,
                                    reason: FailedReason::Panic(panic_msg),
                                };
                                failed.push(failure)
                            }
                            Ok(Err(err)) => {
                                let failure = TestCaseFailure {
                                    name,
                                    reason: FailedReason::Error(err),
                                };
                                failed.push(failure)
                            }
                            Ok(Ok(Some(diff))) => {
                                let failure = TestCaseFailure {
                                    name,
                                    reason: FailedReason::IncorrectResult(diff),
                                };
                                failed.push(failure)
                            }
                            Ok(Ok(None)) => (),
                        }
                    }
                }
            }
        }

        return SuiteResult { failed };

        fn run_one(
            input: &model::Input,
            query: &model::Query,
            input_type: InputTypeToTest,
            result_type: ResultTypeToTest,
        ) -> Result<Option<Diff>, Box<dyn Error>> {
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
        }
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

enum FailedReason {
    Error(Box<dyn Error>),
    Panic(String),
    IncorrectResult(Diff),
}

pub struct TestCaseFailure {
    name: String,
    reason: FailedReason,
}

pub struct SuiteResult {
    failed: Vec<TestCaseFailure>,
}

impl SuiteResult {
    pub fn failed(&self) -> &Vec<TestCaseFailure> {
        &self.failed
    }
}

fn format_panic(panic: Box<dyn std::any::Any + Send + 'static>) -> String {
    if let Some(string) = panic.downcast_ref::<String>() {
        string.clone()
    } else if let Some(&str) = panic.downcast_ref::<&'static str>() {
        str.to_owned()
    } else if let Some(display) = panic.downcast_ref::<&dyn Display>() {
        format!("{}", display)
    } else if let Some(debug) = panic.downcast_ref::<&dyn Debug>() {
        format!("{:?}", debug)
    } else {
        "[opaque panic payload]".to_owned()
    }
}

mod tests {
    use super::format_panic;

    #[test]
    fn format_string_panic() {
        let panic = std::panic::catch_unwind(|| {
            let string = "Expected string.".to_owned();            
            panic!(string)
        }).unwrap_err();

        let result = format_panic(panic);

        assert_eq!(result, "Expected string.");
    }
}