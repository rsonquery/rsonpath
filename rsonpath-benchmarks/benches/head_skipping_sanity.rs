use criterion::{criterion_group, criterion_main, Criterion};
use memchr::memmem;
use regex::bytes::Regex;
use rsonpath::engine::{result::CountResult, Input, Runner};
use rsonpath::query::JsonPathQuery;
use rsonpath::stackless::StacklessRunner;
use std::fs;

const ROOT_TEST_DIRECTORY: &str = "../data";

fn get_contents(test_path: &str) -> Input {
    let path = format!("{}/{}", ROOT_TEST_DIRECTORY, test_path);
    let raw = fs::read_to_string(path).unwrap();
    Input::new(raw)
}

fn classifier_benches(c: &mut Criterion, path: &str, label: &str, id: &str) {
    let contents = get_contents(path);
    let query = format!(".{label}");
    let regex = Regex::new(&format!(r#"[^\\]"{label}"[\s]*:"#)).unwrap();
    let needle = format!(r#""{label}""#);
    let needle_bytes = &needle.as_bytes();

    let rsonpath_query = JsonPathQuery::parse(&query).unwrap();
    let stackless = StacklessRunner::compile_query(&rsonpath_query);

    let mut group = c.benchmark_group(id);
    group.throughput(criterion::Throughput::BytesDecimal(contents.len() as u64));

    group.bench_with_input("rsonpath", &contents, |b, c| {
        b.iter(|| stackless.run::<CountResult>(c))
    });
    group.bench_with_input("regex", &contents, |b, c| b.iter(|| regex.find(c)));
    group.bench_with_input("memmem", &(&contents, needle_bytes), |b, &(c, n)| {
        b.iter(|| match memmem::find(c, n) {
            Some(idx) => {
                if c[idx - 1] == b'\\' {
                    return false;
                }
                let mut ending = idx + needle_bytes.len();
                while c[ending].is_ascii_whitespace() {
                    ending += 1;
                }
                c[ending] == b':'
            }
            None => false,
        })
    });

    group.finish();
}

pub fn twitter_prettified(c: &mut Criterion) {
    classifier_benches(c, "basic/twitter.json", "search_metadata", "prettified");
}
pub fn twitter_compressed(c: &mut Criterion) {
    classifier_benches(
        c,
        "basic_compressed/twitter.json",
        "search_metadata",
        "compressed",
    );
}

criterion_group!(twitter_benches, twitter_compressed, twitter_prettified);

criterion_main!(twitter_benches);
