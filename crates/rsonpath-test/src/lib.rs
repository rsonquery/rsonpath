use std::{fs::File, io, path::Path};

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Tag {
    Basic,
    Filter,
    Function,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TestSuite {
    tests: Vec<TestCase>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TestCase {
    pub name: String,
    pub selector: String,
    #[serde(default)]
    pub document: serde_json::Value,
    #[serde(default)]
    pub result: Vec<serde_json::Value>,
    #[serde(default)]
    pub invalid_selector: bool,
}

#[derive(Debug, Clone)]
pub struct TaggedTestCase {
    pub tag: Tag,
    pub test_case: TestCase,
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

        for test_case in deser.tests {
            self.cases.push(TaggedTestCase { tag, test_case })
        }

        Ok(())
    }

    fn get(self) -> Vec<TaggedTestCase> {
        self.cases
    }
}
