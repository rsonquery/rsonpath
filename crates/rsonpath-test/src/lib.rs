use std::{fs::File, io, path::Path};

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Tag {
    Basic,
    Filter,
    Function,
    MultipleSelectors,
    IndexingFromEnd,
    BackwardStep,
    ProperUnicode,
    StrictDescendantOrder,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TestSuite {
    tests: Vec<TestCaseDef>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TestCaseDef {
    pub name: String,
    pub selector: String,
    #[serde(default)]
    pub document: serde_json::Value,
    #[serde(default)]
    pub result: Option<Vec<serde_json::Value>>,
    #[serde(default)]
    pub results: Option<Vec<Vec<serde_json::Value>>>,
    #[serde(default)]
    pub invalid_selector: bool,
}

#[derive(Debug, Clone)]
pub struct TestCase {
    pub name: String,
    pub details: TestCaseDetails,
}

#[derive(Debug, Clone)]
pub enum TestCaseDetails {
    Invalid(InvalidTestCase),
    Valid(ValidTestCase),
}

#[derive(Debug, Clone)]
pub struct InvalidTestCase {
    pub selector: String,
}

#[derive(Debug, Clone)]
pub struct ValidTestCase {
    pub selector: String,
    pub document: serde_json::Value,
    pub results: Vec<Vec<serde_json::Value>>,
}

#[derive(Debug, Clone)]
pub struct TaggedTestCase {
    pub tags: Vec<Tag>,
    pub test_case: TestCase,
}

impl From<TestCaseDef> for TestCase {
    fn from(value: TestCaseDef) -> Self {
        let name = value.name;
        let details = if value.invalid_selector {
            TestCaseDetails::Invalid(InvalidTestCase {
                selector: value.selector,
            })
        } else {
            let results = match (value.result, value.results) {
                (Some(result), None) => vec![result],
                (None, Some(results)) => results,
                (Some(_), Some(_)) => panic!("{name}: both 'result' and 'results' defined"),
                (None, None) => panic!("{name}: neither 'result' nor 'results' defined"),
            };
            TestCaseDetails::Valid(ValidTestCase {
                selector: value.selector,
                document: value.document,
                results,
            })
        };
        Self { name, details }
    }
}

/// Read and tag test cases from the base jsonpath-compliance-test-suite path.
pub fn read_and_tag<P: AsRef<Path>>(path: P) -> Result<Vec<TaggedTestCase>, io::Error> {
    let tests = path.as_ref().join("tests");
    let functions_tests = tests.join("functions");
    let whitespace_tests = tests.join("whitespace");

    let basic = tests.join("basic.json");
    let filter = tests.join("filter.json");
    let index_selector = tests.join("index_selector.json");
    let name_selector = tests.join("name_selector.json");
    let slice_selector = tests.join("slice_selector.json");

    let functions_count = functions_tests.join("count.json");
    let functions_length = functions_tests.join("length.json");
    let functions_match = functions_tests.join("match.json");
    let functions_search = functions_tests.join("search.json");
    let functions_value = functions_tests.join("value.json");

    let whitespace_filter = whitespace_tests.join("filter.json");
    let whitespace_functions = whitespace_tests.join("functions.json");
    let whitespace_operators = whitespace_tests.join("operators.json");
    let whitespace_selectors = whitespace_tests.join("selectors.json");
    let whitespace_slice = whitespace_tests.join("slice.json");

    let mut collection = TaggedTestCollection::new();

    collection.read_file_and_tag(basic, Tag::Basic)?;
    collection.read_file_and_tag(filter, Tag::Filter)?;
    collection.read_file_and_tag(index_selector, Tag::Basic)?;
    collection.read_file_and_tag(name_selector, Tag::Basic)?;
    collection.read_file_and_tag(slice_selector, Tag::Basic)?;
    collection.read_file_and_tag(functions_count, Tag::Function)?;
    collection.read_file_and_tag(functions_length, Tag::Function)?;
    collection.read_file_and_tag(functions_match, Tag::Function)?;
    collection.read_file_and_tag(functions_search, Tag::Function)?;
    collection.read_file_and_tag(functions_value, Tag::Function)?;
    collection.read_file_and_tag(whitespace_filter, Tag::Filter)?;
    collection.read_file_and_tag(whitespace_functions, Tag::Function)?;
    collection.read_file_and_tag(whitespace_operators, Tag::Filter)?;
    collection.read_file_and_tag(whitespace_selectors, Tag::Basic)?;
    collection.read_file_and_tag(whitespace_slice, Tag::Basic)?;

    // This is included in /filter.json, but contains function calls.
    collection.add_special_case_tag("equals, special nothing", Tag::Function);

    // Tests with multiple selectors.
    let tests = [
        "multiple selectors",
        "multiple selectors, name and index, array data",
        "multiple selectors, name and index, object data",
        "multiple selectors, index and slice",
        "multiple selectors, index and slice, overlapping",
        "multiple selectors, duplicate index",
        "multiple selectors, wildcard and index",
        "multiple selectors, wildcard and name",
        "multiple selectors, wildcard and slice",
        "multiple selectors, multiple wildcards",
        "descendant segment, multiple selectors",
        "descendant segment, object traversal, multiple selectors",
        "space between selector and comma",
        "newline between selector and comma",
        "tab between selector and comma",
        "return between selector and comma",
        "space between comma and selector",
        "newline between comma and selector",
        "tab between comma and selector",
        "return between comma and selector",
    ];
    for test in tests {
        collection.add_special_case_tag(test, Tag::MultipleSelectors);
    }

    // Tests with indexing from end.
    let tests = [
        "negative",
        "more negative",
        "negative out of bound",
        "negative range with default step",
        "negative range with negative step",
        "negative range with larger negative step",
        "larger negative range with larger negative step",
        "negative from, positive to",
        "negative from",
        "positive from, negative to",
        "negative from, positive to, negative step",
        "positive from, negative to, negative step",
        "excessively small from value",
        "excessively large from value with negative step",
        "excessively small to value with negative step",
        "excessively small step",
    ];
    for test in tests {
        collection.add_special_case_tag(test, Tag::IndexingFromEnd);
    }

    // Tests with backwards step.
    let tests = [
        "negative step with default start and end",
        "negative step with default start",
        "negative step with default end",
        "larger negative step",
        "negative step with empty array",
        "maximal range with negative step",
    ];
    for test in tests {
        collection.add_special_case_tag(test, Tag::BackwardStep);
    }

    // Tests that require proper unicode support.
    let tests = [
        "double quotes, escaped double quote",
        "double quotes, escaped reverse solidus",
        "double quotes, escaped backspace",
        "double quotes, escaped form feed",
        "double quotes, escaped line feed",
        "double quotes, escaped carriage return",
        "double quotes, escaped tab",
        "single quotes, escaped reverse solidus",
        "single quotes, escaped backspace",
        "single quotes, escaped form feed",
        "single quotes, escaped line feed",
        "single quotes, escaped carriage return",
        "single quotes, escaped tab",
    ];
    for test in tests {
        collection.add_special_case_tag(test, Tag::ProperUnicode);
    }

    // Tests that require quite insane ordering semantics from descendant.
    let tests = [
        "descendant segment, wildcard selector, nested arrays",
        "descendant segment, wildcard selector, nested objects",
    ];
    for test in tests {
        collection.add_special_case_tag(test, Tag::StrictDescendantOrder);
    }

    Ok(collection.get())
}

struct TaggedTestCollection {
    cases: Vec<TaggedTestCase>,
}

impl TaggedTestCollection {
    fn new() -> Self {
        Self { cases: vec![] }
    }

    fn read_file_and_tag<P: AsRef<Path>>(&mut self, file: P, tag: Tag) -> Result<(), io::Error> {
        let file = File::open(file.as_ref())?;
        let deser: TestSuite = serde_json::from_reader(file)?;

        for test_case_def in deser.tests {
            let test_case = test_case_def.into();
            self.cases.push(TaggedTestCase {
                tags: vec![tag],
                test_case,
            })
        }

        Ok(())
    }

    fn add_special_case_tag(&mut self, name: &str, tag: Tag) {
        let case = self
            .cases
            .iter_mut()
            .find(|x| x.test_case.name == name)
            .expect("invalid special-case name");
        case.tags.push(tag);
    }

    fn get(self) -> Vec<TaggedTestCase> {
        self.cases
    }
}
