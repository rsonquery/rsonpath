use core::time::Duration;
use criterion::{criterion_group, criterion_main, BenchmarkId, Criterion};
use simdpath::engine::runner::Runner;
use simdpath::query::JsonPathQuery;
use simdpath::stack_based::StackBasedRunner;
use simdpath::stackless::StacklessRunner;
use std::fs;

const ROOT_TEST_DIRECTORY: &str = "./data";

fn get_contents(test_path: &str) -> String {
    let path = format!("{}/{}", ROOT_TEST_DIRECTORY, test_path);
    fs::read_to_string(path).unwrap()
}

fn simdpath_stack_based_vs_stackless(c: &mut Criterion, path: &str, query_string: &str, id: &str) {
    let contents = get_contents(path);
    let query = JsonPathQuery::parse(query_string).unwrap();

    let mut group = c.benchmark_group(format! {"simdpath_{}", id});
    group.warm_up_time(Duration::from_secs(10));
    group.measurement_time(Duration::from_secs(40));

    group.bench_with_input(
        BenchmarkId::new("stack-based", id),
        &(&query, &contents),
        |b, (q, c)| b.iter(|| StackBasedRunner::compile_query(q).count(c)),
    );
    group.bench_with_input(
        BenchmarkId::new("stackless", id),
        &(&query, &contents),
        |b, (q, c)| b.iter(|| StacklessRunner::compile_query(q).count(c)),
    );
    group.finish();
}

pub fn wikidata_combined(c: &mut Criterion) {
    simdpath_stack_based_vs_stackless(
        c,
        "wikidata_compressed/wikidata_combined.json",
        "$..claims..references..hash",
        "wikidata_combined",
    );
}

pub fn wikidata_person(c: &mut Criterion) {
    simdpath_stack_based_vs_stackless(
        c,
        "wikidata_compressed/wikidata_person.json",
        "$..claims..references..hash",
        "wikidata_person",
    );
}

pub fn wikidata_profession(c: &mut Criterion) {
    simdpath_stack_based_vs_stackless(
        c,
        "wikidata_compressed/wikidata_profession.json",
        "$..claims..mainsnak..value",
        "wikidata_profession",
    );
}

pub fn wikidata_properties(c: &mut Criterion) {
    simdpath_stack_based_vs_stackless(
        c,
        "wikidata_compressed/wikidata_properties.json",
        "$..qualifiers..datavalue..id",
        "wikidata_properties",
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
