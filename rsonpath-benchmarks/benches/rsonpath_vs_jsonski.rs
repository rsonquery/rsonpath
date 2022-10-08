use core::time::Duration;
use criterion::{criterion_group, criterion_main, BenchmarkId, Criterion};
use rsonpath::engine::result::CountResult;
use rsonpath::engine::{Input, Runner};
use rsonpath::query::JsonPathQuery;
use rsonpath::stackless::StacklessRunner;
use rsonpath_benchmarks::rust_jsonski;
use std::fs;

const ROOT_TEST_DIRECTORY: &str = "../data";

struct BenchmarkOptions<'a> {
    pub path: &'a str,
    pub query_string: &'a str,
    pub jsonski_query_string: &'a str,
    pub id: &'a str,
    pub warm_up_time: Duration,
    pub measurement_time: Duration,
}

fn get_contents(test_path: &str) -> Input {
    let path = format!("{}/{}", ROOT_TEST_DIRECTORY, test_path);
    let raw = fs::read_to_string(path).unwrap();
    Input::new(raw)
}

fn get_jsonski_record(test_path: &str) -> rust_jsonski::JsonSkiRecord {
    let path = format!("{}/{}", ROOT_TEST_DIRECTORY, test_path);
    rust_jsonski::load_jsonski_record(&path)
}

fn rsonpath_vs_jsonski(c: &mut Criterion, options: BenchmarkOptions<'_>) {
    let contents = get_contents(options.path);
    let mut group = c.benchmark_group(format! {"rsonpath_vs_jsonski_{}", options.id});
    group.warm_up_time(options.warm_up_time);
    group.measurement_time(options.measurement_time);
    group.throughput(criterion::Throughput::BytesDecimal(contents.len() as u64));

    let rsonpath_query = JsonPathQuery::parse(options.query_string).unwrap();
    let rsonpath = StacklessRunner::compile_query(&rsonpath_query);

    let jsonski_query = rust_jsonski::create_jsonski_query(options.jsonski_query_string);
    let jsonski_record = get_jsonski_record(options.path);

    group.bench_with_input(
        BenchmarkId::new("rsonpath", options.id),
        &contents,
        |b, c| b.iter(|| rsonpath.run::<CountResult>(c)),
    );
    group.bench_with_input(
        BenchmarkId::new("jsonski", options.id),
        &(&jsonski_record, &jsonski_query),
        |b, &(r, q)| {
            b.iter(|| rust_jsonski::call_jsonski(q, r));
        },
    );

    group.finish();
}

pub fn the_twitter_query(c: &mut Criterion) {
    rsonpath_vs_jsonski(
        c,
        BenchmarkOptions {
            path: "basic/twitter.json",
            query_string: "$.search_metadata.count",
            jsonski_query_string: "$.search_metadata.count",
            id: "the_twitter_query",
            warm_up_time: Duration::from_secs(5),
            measurement_time: Duration::from_secs(10),
        },
    );
}

pub fn the_twitter_query_compressed(c: &mut Criterion) {
    rsonpath_vs_jsonski(
        c,
        BenchmarkOptions {
            path: "basic_compressed/twitter.json",
            query_string: "$.search_metadata.count",
            jsonski_query_string: "$.search_metadata.count",
            id: "the_twitter_query_compressed",
            warm_up_time: Duration::from_secs(5),
            measurement_time: Duration::from_secs(10),
        },
    );
}

pub fn artificial1(c: &mut Criterion) {
    rsonpath_vs_jsonski(
        c,
        BenchmarkOptions {
            path: "basic_compressed/fake1.json",
            query_string: "$.a.b.c.d",
            jsonski_query_string: "$[*].a.b.c.d",
            id: "charles_fake1",
            warm_up_time: Duration::from_secs(10),
            measurement_time: Duration::from_secs(40),
        },
    )
}

pub fn artificial2(c: &mut Criterion) {
    rsonpath_vs_jsonski(
        c,
        BenchmarkOptions {
            path: "basic_compressed/fake2.json",
            query_string: "$.a999999.b.c.d",
            jsonski_query_string: "$.a999999.b.c.d",
            id: "charles_fake2",
            warm_up_time: Duration::from_secs(10),
            measurement_time: Duration::from_secs(40),
        },
    )
}

criterion_group!(
    jsonski_benches,
    the_twitter_query,
    the_twitter_query_compressed,
    artificial1,
    artificial2,
);

criterion_main!(jsonski_benches);
