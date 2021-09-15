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

pub fn simdpath_stack_based_vs_stackless_combined_file(c: &mut Criterion) {
    let wikidata_combined = "wikidata/wikidata_combined.json";
    let wikidata_combined_query_string = "$..claims..references..hash";
    let wikidata_combined_contents = get_contents(wikidata_combined);
    let wikidata_combined_query = JsonPathQuery::parse(wikidata_combined_query_string).unwrap();

    let mut group = c.benchmark_group("simdpath_combined_file");
    group.warm_up_time(Duration::from_secs(10));
    group.measurement_time(Duration::from_secs(40));

    group.bench_with_input(
        BenchmarkId::new("stack-based", "wikidata_combined"),
        &(&wikidata_combined_query, &wikidata_combined_contents),
        |b, (q, c)| b.iter(|| StackBasedRunner::compile_query(q).count(c)),
    );
    group.bench_with_input(
        BenchmarkId::new("stackless", "wikidata_combined"),
        &(&wikidata_combined_query, &wikidata_combined_contents),
        |b, (q, c)| b.iter(|| StacklessRunner::compile_query(q).count(c)),
    );
    group.finish();
}

pub fn simdpath_stack_based_vs_stackless(c: &mut Criterion) {
    let wikidata_person = "wikidata/wikidata_person.json";
    let wikidata_person_query_string = "$..claims..references..hash";
    let wikidata_person_contents = get_contents(wikidata_person);
    let wikidata_person_query = JsonPathQuery::parse(wikidata_person_query_string).unwrap();

    let wikidata_profession = "wikidata/wikidata_profession.json";
    let wikidata_profession_query_string = "$..claims..mainsnak..value";
    let wikidata_profession_contents = get_contents(wikidata_profession);
    let wikidata_profession_query = JsonPathQuery::parse(wikidata_profession_query_string).unwrap();

    let wikidata_properties = "wikidata/wikidata_properties.json";
    let wikidata_properties_query_string = "$..qualifiers..datavalue..id";
    let wikidata_properties_contents = get_contents(wikidata_properties);
    let wikidata_properties_query = JsonPathQuery::parse(wikidata_properties_query_string).unwrap();

    let mut group = c.benchmark_group("simdpath");
    group.warm_up_time(Duration::from_secs(6));
    group.measurement_time(Duration::from_secs(10));

    group.bench_with_input(
        BenchmarkId::new("stack-based", "wikidata_person"),
        &(&wikidata_person_query, &wikidata_person_contents),
        |b, (q, c)| b.iter(|| StackBasedRunner::compile_query(q).count(c)),
    );
    group.bench_with_input(
        BenchmarkId::new("stackless", "wikidata_person"),
        &(&wikidata_person_query, &wikidata_person_contents),
        |b, (q, c)| b.iter(|| StacklessRunner::compile_query(q).count(c)),
    );
    group.bench_with_input(
        BenchmarkId::new("stack-based", "wikidata_profession"),
        &(&wikidata_profession_query, &wikidata_profession_contents),
        |b, (q, c)| b.iter(|| StackBasedRunner::compile_query(q).count(c)),
    );
    group.bench_with_input(
        BenchmarkId::new("stackless", "wikidata_profession"),
        &(&wikidata_profession_query, &wikidata_profession_contents),
        |b, (q, c)| b.iter(|| StacklessRunner::compile_query(q).count(c)),
    );
    group.bench_with_input(
        BenchmarkId::new("stack-based", "wikidata_properties"),
        &(&wikidata_properties_query, &wikidata_properties_contents),
        |b, (q, c)| b.iter(|| StackBasedRunner::compile_query(q).count(c)),
    );
    group.bench_with_input(
        BenchmarkId::new("stackless", "wikidata_properties"),
        &(&wikidata_properties_query, &wikidata_properties_contents),
        |b, (q, c)| b.iter(|| StacklessRunner::compile_query(q).count(c)),
    );
    group.finish();
}

criterion_group!(
    benches,
    simdpath_stack_based_vs_stackless,
    simdpath_stack_based_vs_stackless_combined_file
);
criterion_main!(benches);
