use rsonpath::engine::{Compiler, Engine};
use rsonpath_test::{Tag, TaggedTestCase, TestCaseDetails};
use serde_json::Value;
use std::io;

const CTS_PATH: &str = "jsonpath-compliance-test-suite";

#[test]
fn test_cts() -> Result<(), io::Error> {
    let collection = rsonpath_test::read_and_tag(CTS_PATH)?;
    let results: Vec<_> = collection
        .into_iter()
        .map(|t| (t.test_case.name.clone(), test_one(t)))
        .collect();
    let mut success = true;

    for (name, result) in results {
        match result {
            TestResult::Passed => eprintln!("v {name} passed"),
            TestResult::Ignored => eprintln!("? {name} ignored"),
            TestResult::Failed(err) => {
                success = false;
                eprintln!("x {name} failed\n{err}");
            }
        }
    }

    assert!(success);

    Ok(())
}

fn test_one(def: TaggedTestCase) -> TestResult {
    let (tags, test_case) = (def.tags, def.test_case);
    if !does_parser_support(&tags) {
        return TestResult::Ignored;
    }

    match test_case.details {
        TestCaseDetails::Invalid(test_details) => {
            let parser_result = rsonpath_syntax::parse(&test_details.selector);
            if parser_result.is_ok() {
                let err = format!(
                    "test case {} is supposed to fail, but parser accepted the query\nparse result: {:?}",
                    test_case.name,
                    parser_result.unwrap()
                );
                TestResult::Failed(err)
            } else {
                TestResult::Passed
            }
        }
        TestCaseDetails::Valid(test_details) => {
            let parser_result = rsonpath_syntax::parse(&test_details.selector);
            match parser_result {
                Ok(query) => {
                    if !does_engine_support(&tags) {
                        TestResult::Ignored
                    } else {
                        match rsonpath::engine::RsonpathEngine::compile_query(&query) {
                            Ok(engine) => {
                                let input_str = test_details.document.to_string();
                                let input = rsonpath::input::OwnedBytes::from(input_str);
                                let mut results = vec![];
                                match engine.matches(&input, &mut results) {
                                    Ok(()) => match compare_results(&results, &test_details.results) {
                                        Ok(()) => TestResult::Passed,
                                        Err(err) => {
                                            let err =
                                                format!("test case {} failed\ninvalid result: {}", test_case.name, err);
                                            TestResult::Failed(err)
                                        }
                                    },
                                    Err(engine_err) => {
                                        let err = format!(
                                            "test case {} failed\nexecution error: {}",
                                            test_case.name, engine_err
                                        );
                                        TestResult::Failed(err)
                                    }
                                }
                            }
                            Err(compile_err) => {
                                let err = format!(
                                    "test case {} failed to compile\ncompile error: {}",
                                    test_case.name, compile_err
                                );
                                TestResult::Failed(err)
                            }
                        }
                    }
                }
                Err(parse_err) => {
                    let err = format!(
                        "test case {} failed to parse\nparse error: {}",
                        test_case.name, parse_err
                    );
                    TestResult::Failed(err)
                }
            }
        }
    }
}

fn compare_results(matches: &[rsonpath::result::Match], variants: &Vec<Vec<Value>>) -> Result<(), String> {
    assert!(!variants.is_empty());
    let actual: Result<Vec<Value>, _> = matches.iter().map(|m| serde_json::from_slice(m.bytes())).collect();
    let actual = actual.map_err(|err| format!("matched value is not a valid JSON: {err}"))?;

    for variant in variants {
        if variant == &actual {
            return Ok(());
        }
    }

    let diff = pretty_assertions::Comparison::new(&variants[0], &actual);

    if variants.len() == 1 {
        Err(diff.to_string())
    } else {
        Err(format!("no result variants matched; diff with first:\n{diff}",))
    }
}

fn does_parser_support(tags: &[Tag]) -> bool {
    return tags.iter().all(single);

    fn single(tag: &Tag) -> bool {
        match tag {
            Tag::Basic
            | Tag::Filter
            | Tag::MultipleSelectors
            | Tag::IndexingFromEnd
            | Tag::BackwardStep
            | Tag::ProperUnicode => true,
            Tag::StrictDescendantOrder => true,
            Tag::Function => false,
        }
    }
}

fn does_engine_support(tags: &[Tag]) -> bool {
    return tags.iter().all(single);

    fn single(tag: &Tag) -> bool {
        match tag {
            Tag::Basic | Tag::ProperUnicode => true,
            Tag::Filter
            | Tag::Function
            | Tag::MultipleSelectors
            | Tag::IndexingFromEnd
            | Tag::BackwardStep
            | Tag::StrictDescendantOrder => false,
        }
    }
}

enum TestResult {
    Passed,
    Ignored,
    Failed(String),
}
