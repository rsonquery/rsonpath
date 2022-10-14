use core::time::Duration;
use criterion::{criterion_group, criterion_main, BenchmarkId, Criterion};
use rsonpath::engine::result::CountResult;
use rsonpath::engine::{Input, Runner};
use rsonpath::query::JsonPathQuery;
use rsonpath::stackless::StacklessRunner;
use rsonpath_benchmarks::rust_jsurfer;
use rsonpath_benchmarks::rust_jsonski;

use std::fs;

const ROOT_TEST_DIRECTORY: &str = "../data";
const AST_DATA_PATH : &str = "../data/ast/ast.json";
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

fn ast(c: &mut Criterion, options: BenchmarkOptions<'_>) {
    let context = rust_jsurfer::Jvm::attach().expect("failed to attach to Jvm");
    let jsurfer_file = context
        .load_file(&get_path(AST_DATA_PATH))
        .expect("failed to load file via jsurfer");
    let jsurfer_query = context
        .compile_query(options.query_string)
        .expect("failed to compile query via jsurfer");
    println!("rsonpath/jsurfer Query: {}", options.query_string);
    println!("JsonSki Query: {}", options.jsonski_query_string);
    let contents = get_contents(AST_DATA_PATH);
    let query = JsonPathQuery::parse(options.query_string).unwrap();

    let mut group = c.benchmark_group(format! {"ast_{}", options.id});
    group.warm_up_time(options.warm_up_time);
    group.measurement_time(options.measurement_time);
    group.throughput(criterion::Throughput::BytesDecimal(contents.len() as u64));

    let rsonpath = StacklessRunner::compile_query(&query);

    group.bench_with_input(
        BenchmarkId::new("rsonpath", options.query_string),
        &contents,
        |b, c| b.iter(|| rsonpath.run::<CountResult>(c)),
    );
    group.bench_with_input(
        BenchmarkId::new("jsurfer", options.query_string),
        &(&jsurfer_file, &jsurfer_query),
        |b, &(f, q)| {
            b.iter(|| q.run(f).unwrap());
        },
    );
    if !options.jsonski_query_string.is_empty() {
        let jsonski_query = rust_jsonski::create_jsonski_query(options.jsonski_query_string);
        let jsonski_record = get_jsonski_record(AST_DATA_PATH);
        group.bench_with_input(
            BenchmarkId::new("jsonski", options.jsonski_query_string),
            &(&jsonski_record, &jsonski_query),
            |b, &(r, q)| {
                b.iter(|| rust_jsonski::call_jsonski(q, r));
            },
        );
    }
    group.finish();
}
pub fn decl_name(c: &mut Criterion) {
    ast(
        c,
        BenchmarkOptions {
            query_string: "$..decl.name",
            jsonski_query_string: "",
            id: "decl_name",
            warm_up_time: Duration::from_secs(10),
            measurement_time: Duration::from_secs(40),
        },
    )
}
pub fn included_from(c: &mut Criterion) {
    ast(
        c,
        BenchmarkOptions {
            query_string: "$..loc.includedFrom.file",
            jsonski_query_string: "",
            id: "includedFrom",
            warm_up_time: Duration::from_secs(10),
            measurement_time: Duration::from_secs(40),
        },
    )
}

pub fn nested_inner(c: &mut Criterion) {
    ast(
        c,
        BenchmarkOptions {
            query_string: "$..inner..inner..type.qualType",
            jsonski_query_string: "",
            id: "includedFrom",
            warm_up_time: Duration::from_secs(10),
            measurement_time: Duration::from_secs(40),
        },
    )
}

criterion_group!(ast_benches, decl_name, nested_inner);
criterion_main!(ast_benches);
