use rsonpath::engine::main::MainEngine;
use rsonpath::engine::recursive::RecursiveEngine;
use rsonpath::engine::{Compiler, Engine};
use rsonpath::input::BufferedInput;
use rsonpath::query::JsonPathQuery;
use rsonpath::result::{CountResult};
use std::io::Read;
use std::{cmp, fs};
use test_case::test_case;

const ROOT_TEST_DIRECTORY: &str = "./tests/data";

struct ReadString(String, usize);

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

fn get_contents(test_path: &str) -> BufferedInput<ReadString> {
    let path = format!("{ROOT_TEST_DIRECTORY}/{test_path}");
    let act_path = fs::canonicalize(path).unwrap();
    let raw = fs::read_to_string(act_path).unwrap();
    BufferedInput::new(ReadString(raw, 0))
}

macro_rules! count_test_cases {
    ($test_name:ident, $impl:ident) => {
        #[test_case("basic/atomic_after_complex.json", "$.a..b" => 1; "atomic_after_complex.json $.a..b")]
        #[test_case("basic/atomic_after_complex.json", "$.a[0]" => 1; "atomic_after_complex.json nneg array index")]
        #[test_case("basic/atomic_after_complex.json", "$.a[0].c.d[2]" => 1; "atomic_after_complex.json nneg last array index")]
        #[test_case("basic/atomic_after_complex.json", "$.a[0].c.d[3]" => 0; "atomic_after_complex.json nneg nonexistent array index")]
        #[test_case("basic/atomic_after_complex.json", "$..*[2]" => 1; "atomic_after_complex.json any desc nneg nonexistent array index")]
        #[test_case("basic/array_root_nested.json", "$[0]" => 1; "array_root_nested.json nneg array top")]
        #[test_case("basic/array_root_nested.json", "$[0].*" => 1; "array_root_nested.json nneg array inner any child")]
        #[test_case("basic/array_root_nested.json", "$.*[0]" => 2; "array_root_nested.json any nneg array inner")]
        #[test_case("basic/array_root_nested.json", "$[2][1][0][1]" => 1; "array_root_nested.json any nneg array inner inner inner")]
        #[test_case("basic/array_root_nested.json", "$[2][1].*" => 3; "array_root_nested.json any nneg array inner inner child")]
        #[test_case("basic/array_root_nested.json", "$[2].*[1]" => 1; "array_root_nested.json nneg array child inner")]
        #[test_case("basic/array_root_nested.json", "$[2][1]..*[1]" => 3; "array_root_nested.json nneg nneg anydesc nneg")]
        #[test_case("basic/array_root_nested.json", "$[2]..*" => 11; "array_root_nested.json nneg array anydesc")]
        #[test_case("basic/array_root_nested.json", "$..*[0]" => 7; "array_root_nested.json anydesc nneg array")]
        #[test_case("basic/array_root_nested.json", "$[2][0]" => 1; "array_root_nested.json nneg array direct first")]
        #[test_case("basic/array_root_nested.json", "$[2][1]" => 1; "array_root_nested.json nneg array direct second")]
        #[test_case("basic/child.json", "$..a..b.c..d" => 3; "child.json $..a..b.c..d")]
        #[test_case("basic/child_hell.json", "$..x..a.b.a.b.c" => 6; "child_hell.json $..x..a.b.a.b.c")]
        #[test_case("basic/escapes.json", r#"$..a..b..['label\\']"# => 1; "escapes.json existing label")]
        #[test_case("basic/escapes.json", r#"$..a..b..['label\\\\']"# => 0; "escapes.json nonexistent label")]
        #[test_case("basic/heterogeneous_list.json", r#"$.a.*"# => 3; "heterogeneous_list.json $.a.*")]
        #[test_case("basic/memchr_trap.json", "$..b" => 1; "memchr_trap.json $..b")]
        #[test_case("basic/memchr_trap.json", r#"$..['"b']"# => 1; r#"memchr_trap.json $..['"b']"#)]
        #[test_case("basic/quote_escape.json", r#"$['x']"# => 1; "quote_escape.json without quote")]
        #[test_case("basic/quote_escape.json", r#"$['"x']"# => 1; "quote_escape.json with quote")]
        #[test_case("basic/singletons_and_empties.json", r#"$.*.*"# => 2; "singletons_and_empties.json $.*.*")]
        #[test_case("basic/singletons_and_empties.json", r#"$..*.*"# => 2; "singletons_and_empties.json any descendant $..*.*")]
        #[test_case("basic/skipping.json", r#"$.a.b"# => 1; "skipping")]
        #[test_case("basic/small_no_list.json", "$..person..phoneNumber..number" => 2; "small_no_list.json $..person..phoneNumber..number")]
        #[test_case("basic/small.json", "$..person..phoneNumber..number" => 4; "small.json $..person..phoneNumber..number")]
        #[test_case("basic/small.json", "$..person..*..type" => 4; "small.json $..person..*..type")]
        #[test_case("basic/spaced_colon.json", r#"$..a..b..label"# => 2; "spaced colon")]
        #[test_case("basic/wildcard_list.json", r#"$..a.*"# => 6; "wildcard_list.json $..a.*")]
        #[test_case("basic/wildcard_list2.json", r#"$..a.*..b.*"# => 8; "wildcard_list2.json $..a.*..b.*")]
        #[test_case("basic/wildcard_list2.json", r#"$..a..*..b..*"# => 9; "wildcard_list2.json any descendant $..a..*..b..*")]
        #[test_case("basic/wildcard_object.json", r#"$..a.*"# => 7; "wildcard_object.json $..a.*")]
        #[test_case("basic/wildcard_object2.json", r#"$..a.*.*..b.*.*"# => 9; "wildcard_object2.json $..a.*.*..b.*.*")]
        #[test_case("twitter/twitter.json", "$..user..entities..url" => 44; "twitter.json $..user..entities..url (recursive)")]
        #[test_case("twitter/twitter.json", "$..user..entities.url" => 18; "twitter.json $..user..entities.url (child)")]
        #[test_case("twitter/twitter.json", "$.search_metadata.count" => 1; "twitter.json $.search_metadata.count (child-child)")]
        #[test_case("twitter/twitter.json", "$..search_metadata.count" => 1; "twitter.json $..search_metadata.count (recursive-child)")]
        #[test_case("twitter/twitter.json", "$.search_metadata..count" => 1; "twitter.json $.search_metadata..count (child-recursive)")]
        #[test_case("twitter/twitter.json", "$..search_metadata..count" => 1; "twitter.json $..search_metadata..count (recursive-recursive)")]
        #[test_case("twitter/twitter.json", "$..count" => 1; "twitter.json $..count")]
        #[test_case("twitter/twitter_urls.json", "$..entities..urls..url" => 2; "twitter_urls.json $..entities..urls..url")]
        #[test_case("twitter/twitter_urls.json", "$..entities.urls..url" => 2; "twitter_urls.json $..entities.urls..url (child)")]
        #[test_case("basic/compressed/child.json", "$..a..b.c..d" => 3; "compressed child.json $..a..b.c..d")]
        #[test_case("basic/compressed/child_hell.json", "$..x..a.b.a.b.c" => 6; "compressed child_hell.json $..x..a.b.a.b.c")]
        #[test_case("basic/compressed/escapes.json", r#"$..a..b..['label\\']"# => 1; "compressed escapes.json existing label")]
        #[test_case("basic/compressed/escapes.json", r#"$..a..b..['label\\\\']"# => 0; "compressed escapes.json nonexistent label")]
        #[test_case("basic/compressed/fake2.json", r#"$.a999999.b.c.d"# => 1; "compressed fake2.json")]
        #[test_case("basic/compressed/memchr_trap.json", "$..b" => 1; "compressed memchr_trap.json $..b")]
        #[test_case("basic/compressed/memchr_trap.json", r#"$..['"b']"# => 1; r#"compressed memchr_trap.json $..['"b']"#)]
        #[test_case("basic/compressed/quote_escape.json", r#"$['x']"# => 1; "compressed quote_escape.json without quote")]
        #[test_case("basic/compressed/quote_escape.json", r#"$['"x']"# => 1; "compressed quote_escape.json with quote")]
        #[test_case("basic/compressed/singletons_and_empties.json", r#"$.*.*"# => 2; "compressed singletons_and_empties.json")]
        #[test_case("basic/compressed/skipping.json", r#"$.a.b"# => 1; "compressed skipping")]
        #[test_case("basic/compressed/small_no_list.json", "$..person..phoneNumber..number" => 2; "compressed small_no_list.json $..person..phoneNumber..number")]
        #[test_case("basic/compressed/small.json", "$..person..phoneNumber..number" => 4; "compressed small.json $..person..phoneNumber..number")]
        #[test_case("twitter/compressed/twitter.json", "$..user..entities..url" => 44; "compressed twitter.json $..user..entities..url (recursive)")]
        #[test_case("twitter/compressed/twitter.json", "$..user..entities.url" => 18; "compressed twitter.json $..user..entities.url (child)")]
        #[test_case("twitter/compressed/twitter_urls.json", "$..entities..urls..url" => 2; "compressed twitter_urls.json $..entities..urls..url")]
        #[test_case("twitter/compressed/twitter_urls.json", "$..entities.urls..url" => 2; "compressed twitter_urls.json $..entities.urls..url (child)")]
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
            "wikidata/wikidata_person.json", "$..snaks..*.id" => 11113;
            "wikidata_person.json $..snaks..*.id any descendant"
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
            "wikidata/wikidata_person.json", "$..*" => 970442;
            "wikidata_person.json $..* any descendant"
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
            "wikidata/wikidata_profession.json", "$..*" => 1702482;
            "wikidata_profession.json $..* any descendant"
        )]
        #[test_case(
            "wikidata/wikidata_profession.json", "$..en..value" => 13634;
            "wikidata_profession.json $..en..value (recursive)"
        )]
        #[test_case(
            "wikidata/wikidata_profession.json", "$..*.id" => 98805;
            "wikidata_profession.json $..*.id any descendant"
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
        #[test_case(
            "wikidata/wikidata_properties.json", "$..*.value" => 132188;
            "wikidata_properties.json $..*.value (child) any desc"
        )]
        #[test_case(
            "wikidata/wikidata_properties.json", "$..*[5]" => 2511;
            "wikidata_properties.json $..*[5] (child) any desc nneg array index"
        )]
        #[test_case(
            "wikidata/wikidata_properties.json", "$..P7103.claims.P31..references..snaks.P4656..hash" => 1;
            "wikidata_properties.json $..P7103.claims.P31..references..snaks.P4656..hash"
        )]
        fn $test_name(test_path: &str, query_string: &str) -> u64 {
            let contents = get_contents(test_path);
            let query = JsonPathQuery::parse(query_string).unwrap();
            let result = $impl::compile_query(&query).unwrap().run::<_, CountResult>(&contents).unwrap();

            result.get()
        }
    };
}

count_test_cases!(rsonpath_count_main, MainEngine);
count_test_cases!(rsonpath_count_recursive, RecursiveEngine);
