use rsonpath::engine::{Input, Runner};
use rsonpath::query::JsonPathQuery;
use rsonpath::stack_based::StackBasedRunner;
use rsonpath::stackless::StacklessRunner;
use std::fs;
use test_case::test_case;

const ROOT_TEST_DIRECTORY: &str = "./data";

fn get_contents(test_path: &str) -> Input {
    let path = format!("{}/{}", ROOT_TEST_DIRECTORY, test_path);
    let raw = fs::read_to_string(path).unwrap();
    Input::new(raw)
}

macro_rules! test_cases {
    ($test_name:ident, $impl:ident) => {
        #[test_case("basic/empty.json", "" => 0; "empty.json")]
        #[test_case("basic/empty.json", "$" => 0; "empty.json $")]
        #[test_case("basic/root.json", "$" => 1; "root.json $")]
        #[test_case("basic/root.json", "" => 1; "root.json")]
        #[test_case("basic/small_no_list.json", "$..person..phoneNumber..number" => 2; "small_no_list.json $..person..phoneNumber..number")]
        #[test_case("basic/small.json", "$..person..phoneNumber..number" => 4; "small.json $..person..phoneNumber..number")]
        #[test_case("basic/child.json", "$..a..b.c..d" => 3; "child.json $..a..b.c..d")]
        #[test_case("basic/child_hell.json", "$..x..a.b.a.b.c" => 6; "child_hell.json $..x..a.b.a.b.c")]
        #[test_case("basic/twitter.json", "$..user..entities..url" => 44; "twitter.json $..user..entities..url (recursive)")]
        #[test_case("basic/twitter.json", "$..user..entities.url" => 18; "twitter.json $..user..entities.url (child)")]
        #[test_case("basic/escapes.json", r#"$..a..b..['label\\']"# => 1; "escapes.json existing label")]
        #[test_case("basic/escapes.json", r#"$..a..b..['label\']"# => 0; "escapes.json nonexistent label")]
        #[test_case("basic/spaced_colon.json", r#"$..a..b..label"# => 2; "spaced colon")]
        #[test_case("basic_compressed/small_no_list.json", "$..person..phoneNumber..number" => 2; "compressed small_no_list.json $..person..phoneNumber..number")]
        #[test_case("basic_compressed/small.json", "$..person..phoneNumber..number" => 4; "compressed small.json $..person..phoneNumber..number")]
        #[test_case("basic_compressed/child.json", "$..a..b.c..d" => 3; "compressed child.json $..a..b.c..d")]
        #[test_case("basic_compressed/child_hell.json", "$..x..a.b.a.b.c" => 6; "compressed child_hell.json $..x..a.b.a.b.c")]
        #[test_case("basic_compressed/twitter.json", "$..user..entities..url" => 44; "compressed twitter.json $..user..entities..url (recursive)")]
        #[test_case("basic_compressed/twitter.json", "$..user..entities.url" => 18; "compressed twitter.json $..user..entities.url (child)")]
        #[test_case("basic_compressed/escapes.json", r#"$..a..b..['label\\']"# => 1; "compressed escapes.json existing label")]
        #[test_case("basic_compressed/escapes.json", r#"$..a..b..['label\']"# => 0; "compressed escapes.json nonexistent label")]
        #[test_case(
            "wikidata/wikidata_person.json", "$..claims..references..hash" => 37736;
            "wikidata_person.json $..claims..references..hash"
        )]
        #[test_case(
            "wikidata/wikidata_person.json", "$..references..snaks..datavalue" => 25118;
            "wikidata_person.json $..references..snaks..datavalue"
        )]
        #[test_case(
            "wikidata/wikidata_person.json", "$..references..snaks..datavalue..value" => 25118;
            "wikidata_person.json $..references..snaks..datavalue..value"
        )]
        #[test_case(
            "wikidata/wikidata_person.json", "$..references..snaks..datavalue..value..id" => 11113;
            "wikidata_person.json $..references..snaks..datavalue..value..id"
        )]
        #[test_case(
            "wikidata/wikidata_person.json", "$..snaks..datavalue..value" => 25118;
            "wikidata_person.json $..snaks..datavalue..value"
        )]
        #[test_case(
            "wikidata/wikidata_person.json", "$..datavalue..value..id" => 25093;
            "wikidata_person.json $..datavalue..value..id"
        )]
        #[test_case(
            "wikidata/wikidata_person.json", "$..mainsnak..datavalue..value" => 26115;
            "wikidata_person.json $..mainsnak..datavalue..value"
        )]
        #[test_case(
            "wikidata/wikidata_person.json", "$..mainsnak..datavalue..value..id" => 12958;
            "wikidata_person.json $..mainsnak..datavalue..value..id"
        )]
        #[test_case(
            "wikidata/wikidata_person.json", "$..en..value" => 2360;
            "wikidata_person.json $..en..value (recursive)"
        )]
        #[test_case(
            "wikidata/wikidata_person.json", "$..en.value" => 1753;
            "wikidata_person.json $..en.value (child)"
        )]
        #[test_case(
            "wikidata/wikidata_profession.json", "$..claims..mainsnak..value" => 59112;
            "wikidata_profession.json $..claims..mainsnak..value"
        )]
        #[test_case(
            "wikidata/wikidata_profession.json", "$..en..value" => 13634;
            "wikidata_profession.json $..en..value (recursive)"
        )]
        #[test_case(
            "wikidata/wikidata_profession.json", "$..en.value" => 9452;
            "wikidata_profession.json $..en.value (child)"
        )]
        #[test_case(
            "wikidata/wikidata_properties.json", "$..qualifiers..datavalue..id" => 18219;
            "wikidata_properties.json $..qualifiers..datavalue..id"
        )]
        #[test_case(
            "wikidata/wikidata_properties.json", "$..en..value" => 4504;
            "wikidata_properties.json $..en..value (recursive)"
        )]
        #[test_case(
            "wikidata/wikidata_properties.json", "$..en.value" => 1760;
            "wikidata_properties.json $..en.value (child)"
        )]
        fn $test_name(test_path: &str, query_string: &str) -> usize {
            let contents = get_contents(test_path);
            let query = JsonPathQuery::parse(query_string).unwrap();
            let result = $impl::compile_query(&query).count(&contents);

            result.count
        }
    };
}

test_cases!(rsonpath_stackless, StacklessRunner);
test_cases!(rsonpath_stack_based, StackBasedRunner);
