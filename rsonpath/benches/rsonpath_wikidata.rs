use core::time::Duration;
use criterion::{criterion_group, criterion_main, BenchmarkId, Criterion};
use decimal_byte_measurement::DecimalByteMeasurement;
use rsonpath::engine::result::CountResult;
use rsonpath::engine::{Input, Runner};
use rsonpath::query::JsonPathQuery;
use rsonpath::stack_based::StackBasedRunner;
use rsonpath::stackless::StacklessRunner;
use std::fs;

const ROOT_TEST_DIRECTORY: &str = "./data";

type CriterionCtx = Criterion<DecimalByteMeasurement>;

struct BenchmarkOptions<'a> {
    pub path: &'a str,
    pub query_string: &'a str,
    pub id: &'a str,
    pub warm_up_time: Duration,
    pub measurement_time: Duration,
}

fn get_contents(test_path: &str) -> Input {
    let path = format!("{}/{}", ROOT_TEST_DIRECTORY, test_path);
    let raw = fs::read_to_string(path).unwrap();
    Input::new(raw)
}

fn rsonpath_stack_based_vs_stackless(c: &mut CriterionCtx, options: BenchmarkOptions<'_>) {
    let contents = get_contents(options.path);
    let query = JsonPathQuery::parse(options.query_string).unwrap();

    let mut group = c.benchmark_group(format! {"rsonpath_{}", options.id});
    group.warm_up_time(options.warm_up_time);
    group.measurement_time(options.measurement_time);
    group.throughput(criterion::Throughput::Bytes(contents.len() as u64));

    let stackless = StacklessRunner::compile_query(&query);
    let stack_based = StackBasedRunner::compile_query(&query);

    group.bench_with_input(
        BenchmarkId::new("stackless", options.id),
        &contents,
        |b, c| b.iter(|| stackless.run::<CountResult>(c)),
    );
    group.bench_with_input(
        BenchmarkId::new("stack-based", options.id),
        &contents,
        |b, c| b.iter(|| stack_based.run::<CountResult>(c)),
    );

    group.finish();
}

fn decimal_byte_measurement() -> CriterionCtx {
    Criterion::default().with_measurement(DecimalByteMeasurement::new())
}

pub fn wikidata_combined(c: &mut CriterionCtx) {
    rsonpath_stack_based_vs_stackless(
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

pub fn wikidata_combined_with_whitespace(c: &mut CriterionCtx) {
    rsonpath_stack_based_vs_stackless(
        c,
        BenchmarkOptions {
            path: "wikidata_prettified/wikidata_combined.json",
            query_string: "$..claims..references..hash",
            id: "wikidata_combined_with_whitespace",
            warm_up_time: Duration::from_secs(10),
            measurement_time: Duration::from_secs(40),
        },
    );
}

pub fn wikidata_person(c: &mut CriterionCtx) {
    rsonpath_stack_based_vs_stackless(
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

pub fn wikidata_person_en_value_recursive(c: &mut CriterionCtx) {
    rsonpath_stack_based_vs_stackless(
        c,
        BenchmarkOptions {
            path: "wikidata_compressed/wikidata_person.json",
            query_string: "$..en..value",
            id: "wikidata_person_en_value_recursive",
            warm_up_time: Duration::from_secs(3),
            measurement_time: Duration::from_secs(5),
        },
    );
}

pub fn wikidata_person_en_value_direct(c: &mut CriterionCtx) {
    rsonpath_stack_based_vs_stackless(
        c,
        BenchmarkOptions {
            path: "wikidata_compressed/wikidata_person.json",
            query_string: "$..en.value",
            id: "wikidata_person_en_value_direct",
            warm_up_time: Duration::from_secs(3),
            measurement_time: Duration::from_secs(5),
        },
    );
}

pub fn wikidata_profession(c: &mut CriterionCtx) {
    rsonpath_stack_based_vs_stackless(
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

pub fn wikidata_properties(c: &mut CriterionCtx) {
    rsonpath_stack_based_vs_stackless(
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
    name = wikidata_benches;
    config = decimal_byte_measurement();
    targets =
        wikidata_combined,
        wikidata_combined_with_whitespace,
        wikidata_person,
        wikidata_person_en_value_recursive,
        wikidata_person_en_value_direct,
        wikidata_profession,
        wikidata_properties
);

criterion_main!(wikidata_benches);
