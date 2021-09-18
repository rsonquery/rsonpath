use core::time::Duration;
use criterion::{criterion_group, criterion_main, BenchmarkId, Criterion};
use simdpath::engine::runner::Runner;
use simdpath::query::JsonPathQuery;
use simdpath::stack_based::StackBasedRunner;
use simdpath::stackless::StacklessRunner;
use std::fs;

const ROOT_TEST_DIRECTORY: &str = "./data";

struct BenchmarkOptions<'a> {
    pub path: &'a str,
    pub query_string: &'a str,
    pub id: &'a str,
    pub warm_up_time: Duration,
    pub measurement_time: Duration,
}

fn get_contents(test_path: &str) -> String {
    let path = format!("{}/{}", ROOT_TEST_DIRECTORY, test_path);
    fs::read_to_string(path).unwrap()
}

fn simdpath_stack_based_vs_stackless(c: &mut Criterion, options: BenchmarkOptions<'_>) {
    let contents = get_contents(options.path);
    let query = JsonPathQuery::parse(options.query_string).unwrap();

    let mut group = c.benchmark_group(format! {"simdpath_{}", options.id});
    group.warm_up_time(options.warm_up_time);
    group.measurement_time(options.measurement_time);

    group.bench_with_input(
        BenchmarkId::new("stack-based", options.id),
        &(&query, &contents),
        |b, (q, c)| b.iter(|| StackBasedRunner::compile_query(q).count(c)),
    );
    group.bench_with_input(
        BenchmarkId::new("stackless", options.id),
        &(&query, &contents),
        |b, (q, c)| b.iter(|| StacklessRunner::compile_query(q).count(c)),
    );
    group.finish();
}

pub fn wikidata_combined(c: &mut Criterion) {
    simdpath_stack_based_vs_stackless(
        c,
        BenchmarkOptions {
            path: "wikidata_compressed/wikidata_combined.json",
            query_string: "$..claims..references..hash",
            id: "wikidata_combined",
            warm_up_time: Duration::from_secs(10),
            measurement_time: Duration::from_secs(40),
        },
    );
}

pub fn wikidata_person(c: &mut Criterion) {
    simdpath_stack_based_vs_stackless(
        c,
        BenchmarkOptions {
            path: "wikidata_compressed/wikidata_person.json",
            query_string: "$..claims..references..hash",
            id: "wikidata_person",
            warm_up_time: Duration::from_secs(3),
            measurement_time: Duration::from_secs(5),
        },
    );
}

pub fn wikidata_profession(c: &mut Criterion) {
    simdpath_stack_based_vs_stackless(
        c,
        BenchmarkOptions {
            path: "wikidata_compressed/wikidata_profession.json",
            query_string: "$..claims..mainsnak..value",
            id: "wikidata_profession",
            warm_up_time: Duration::from_secs(3),
            measurement_time: Duration::from_secs(5),
        },
    );
}

pub fn wikidata_properties(c: &mut Criterion) {
    simdpath_stack_based_vs_stackless(
        c,
        BenchmarkOptions {
            path: "wikidata_compressed/wikidata_properties.json",
            query_string: "$..qualifiers..datavalue..id",
            id: "wikidata_properties",
            warm_up_time: Duration::from_secs(3),
            measurement_time: Duration::from_secs(5),
        },
    );
}

criterion_group!(
    benches,
    wikidata_combined,
    wikidata_person,
    wikidata_profession,
    wikidata_properties,
);
criterion_main!(benches);
