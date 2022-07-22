use core::time::Duration;
use criterion::{criterion_group, criterion_main, BenchmarkId, Criterion};
use decimal_byte_measurement::DecimalByteMeasurement;
use rsonpath::engine::result::CountResult;
use rsonpath::engine::{Input, Runner};
use rsonpath::query::JsonPathQuery;
use rsonpath::stackless::StacklessRunner;
use rsonpath_benchmarks::rust_jsurfer;
use std::fs;

const ROOT_TEST_DIRECTORY: &str = "../data";

type CriterionCtx = Criterion<DecimalByteMeasurement>;

struct BenchmarkOptions<'a> {
    pub path: &'a str,
    pub query_string: &'a str,
    pub id: &'a str,
    pub warm_up_time: Duration,
    pub measurement_time: Duration,
}

fn get_path(test_path: &str) -> String {
    format!("{}/{}", ROOT_TEST_DIRECTORY, test_path)
}

fn get_contents(test_path: &str) -> Input {
    let raw = fs::read_to_string(get_path(test_path)).unwrap();
    Input::new(raw)
}

fn jsurfer_overhead(c: &mut CriterionCtx) {
    let context = rust_jsurfer::Jvm::attach().expect("failed to attach to Jvm");
    let jsurfer_file = context
        .load_file(&get_path("wikidata_compressed/wikidata_combined.json"))
        .expect("failed to load file via jsurfer");
    let jsurfer_overhead = context
        .create_overhead()
        .expect("failed to create overhead shim via jsurfer");

    let mut group = c.benchmark_group("jsurfer_overhead");

    group.bench_with_input(
        "jsurfer_overhead",
        &(&jsurfer_file, &jsurfer_overhead),
        |b, &(f, q)| {
            b.iter(|| q.run(f).unwrap());
        },
    );

    group.finish();
}

fn rsonpath_vs_jsurfer(c: &mut CriterionCtx, options: BenchmarkOptions<'_>) {
    let context = rust_jsurfer::Jvm::attach().expect("failed to attach to Jvm");
    let jsurfer_file = context
        .load_file(&get_path(options.path))
        .expect("failed to load file via jsurfer");
    let jsurfer_query = context
        .compile_query(options.query_string)
        .expect("failed to compile query via jsurfer");

    let contents = get_contents(options.path);
    let query = JsonPathQuery::parse(options.query_string).unwrap();

    let mut group = c.benchmark_group(format! {"rsonpath_vs_jsurfer_{}", options.id});
    group.warm_up_time(options.warm_up_time);
    group.measurement_time(options.measurement_time);
    group.throughput(criterion::Throughput::Bytes(contents.len() as u64));

    let rsonpath = StacklessRunner::compile_query(&query);

    group.bench_with_input(
        BenchmarkId::new("rsonpath", options.id),
        &contents,
        |b, c| b.iter(|| rsonpath.run::<CountResult>(c)),
    );
    group.bench_with_input(
        BenchmarkId::new("jsurfer_execution", options.id),
        &(&jsurfer_file, &jsurfer_query),
        |b, &(f, q)| {
            b.iter(|| q.run(f).unwrap());
        },
    );

    group.finish();
}

fn decimal_byte_measurement() -> CriterionCtx {
    Criterion::default().with_measurement(DecimalByteMeasurement::new())
}

pub fn wikidata_combined(c: &mut CriterionCtx) {
    rsonpath_vs_jsurfer(
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
    rsonpath_vs_jsurfer(
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

pub fn artificial(c: &mut CriterionCtx) {
    rsonpath_vs_jsurfer(
        c,
        BenchmarkOptions {
            path: "basic/fake1.json",
            query_string: "$..a.b.c.d",
            id: "charles_fake",
            warm_up_time: Duration::from_secs(10),
            measurement_time: Duration::from_secs(40),
        },
    )
}

criterion_group!(
    name = jsonski_benches;
    config = decimal_byte_measurement();
    targets =
        jsurfer_overhead,
        wikidata_combined,
        wikidata_combined_with_whitespace,
        artificial
);

criterion_main!(jsonski_benches);
