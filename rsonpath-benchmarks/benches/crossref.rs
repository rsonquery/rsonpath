use core::time::Duration;
use criterion::{criterion_group, criterion_main, BenchmarkId};
use criterion_decimal_throughput::{decimal_byte_measurement, Criterion};
use rsonpath::engine::result::CountResult;
use rsonpath::engine::{Input, Runner};
use rsonpath::query::JsonPathQuery;
use rsonpath::stackless::StacklessRunner;
use rsonpath_benchmarks::rust_jsurfer;
use rsonpath_benchmarks::rust_jsonski;

use std::fs;

const ROOT_TEST_DIRECTORY: &str = "../data";
const CROSSREF_DATA_PATH : &str = "../data/crossref/crossref.json";
fn get_jsonski_record(test_path: &str) -> rust_jsonski::JsonSkiRecord {
    let path = format!("{}/{}", ROOT_TEST_DIRECTORY, test_path);
    rust_jsonski::load_jsonski_record(&path)
}
fn get_path(test_path: &str) -> String {
    format!("{}/{}", ROOT_TEST_DIRECTORY, test_path)
}

fn get_contents(test_path: &str) -> Input {
    let raw = fs::read_to_string(get_path(test_path)).unwrap();
    Input::new(raw)
}

struct BenchmarkOptions<'a> {
    pub query_string: &'a str,
    pub jsonski_query_string: &'a str,
    pub id: &'a str,
    pub warm_up_time: Duration,
    pub measurement_time: Duration,
}

fn crossref(c: &mut Criterion, options: BenchmarkOptions<'_>) {
    let context = rust_jsurfer::Jvm::attach().expect("failed to attach to Jvm");
    let jsurfer_file = context
        .load_file(&get_path(CROSSREF_DATA_PATH))
        .expect("failed to load file via jsurfer");
    let jsurfer_query = context
        .compile_query(options.query_string)
        .expect("failed to compile query via jsurfer");

    let contents = get_contents(CROSSREF_DATA_PATH);
    let query = JsonPathQuery::parse(options.query_string).unwrap();

    let mut group = c.benchmark_group(format! {"crossref_{}", options.id});
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
    if !options.jsonski_query_string.is_empty() {
        let jsonski_query = rust_jsonski::create_jsonski_query(options.jsonski_query_string);
        let jsonski_record = get_jsonski_record(CROSSREF_DATA_PATH); 
        group.bench_with_input(
            BenchmarkId::new("jsonski", options.id),
            &(&jsonski_record, &jsonski_query),
                |b, &(r, q)| {
                    b.iter(|| rust_jsonski::call_jsonski(q, r));
                },
        );
    }
    group.finish();
}
pub fn doi(c: &mut Criterion) {
    crossref(
        c,
        BenchmarkOptions {
            query_string: "$..DOI",
            jsonski_query_string: "",
            id: "DOI",
            warm_up_time: Duration::from_secs(10),
            measurement_time: Duration::from_secs(40),
        },
    )
}
pub fn title(c: &mut Criterion) {
    crossref(
        c,
        BenchmarkOptions {
            query_string: "$..title",
            jsonski_query_string: "",
            id: "title",
            warm_up_time: Duration::from_secs(10),
            measurement_time: Duration::from_secs(40),
        },
    )
}
criterion_group!(
    name = crossref_benches;
    config = decimal_byte_measurement();
    targets =
        title,
        doi,
);

criterion_main!(crossref_benches);
