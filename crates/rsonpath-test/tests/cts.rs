use std::io;

use rsonpath_test::{Tag, TaggedTestCase};

const CTS_PATH: &str = "jsonpath-compliance-test-suite";

#[test]
fn test_cts() -> Result<(), io::Error> {
    let collection = rsonpath_test::read_and_tag(CTS_PATH)?;
    let results: Vec<_> = collection.into_iter().map(test_one).collect();
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

fn test_one(t: TaggedTestCase) -> (String, TestResult) {
    let (tag, test_case) = (t.tag, t.test_case);
    if !does_parser_support(tag) {
        return (test_case.name, TestResult::Ignored);
    }

    let parser_result = rsonpath_syntax::parse(&test_case.selector);

    if test_case.invalid_selector {
        if parser_result.is_ok() {
            let err = format!(
                "test case {} is supposed to fail, but parser accepted the query\nparse result: {:?}",
                test_case.name,
                parser_result.unwrap()
            );
            return (test_case.name, TestResult::Failed(err));
        }
        return (test_case.name, TestResult::Passed);
    }

    if parser_result.is_err() {
        let err = format!(
            "test case {} failed to parse\nparse error: {}",
            test_case.name,
            parser_result.unwrap_err()
        );
        return (test_case.name, TestResult::Failed(err));
    }

    (test_case.name, TestResult::Passed)
}

fn does_parser_support(tag: Tag) -> bool {
    match tag {
        Tag::Basic => true,
        Tag::Filter | Tag::Function => false,
    }
}

enum TestResult {
    Passed,
    Ignored,
    Failed(String),
}
