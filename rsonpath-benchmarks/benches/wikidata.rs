use core::time::Duration;
use criterion::{criterion_group, criterion_main, BenchmarkId, Criterion};
use rsonpath::engine::result::CountResult;
use rsonpath::engine::{Input, Runner};
use rsonpath::query::JsonPathQuery;
use rsonpath::stack_based::StackBasedRunner;
use rsonpath::stackless::StacklessRunner;
use std::fs;
use rsonpath_benchmarks::rust_jsurfer;
use rsonpath_benchmarks::rust_jsonski;

const ROOT_TEST_DIRECTORY: &str = "../data";

struct BenchmarkOptions<'a> {
    pub path: &'a str,
    pub query_string: &'a str,
    pub jsonski_query_string: &'a str,
    pub id: &'a str,
    pub warm_up_time: Duration,
    pub measurement_time: Duration,
}
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

fn wikipedia_bench(c: &mut Criterion, options: BenchmarkOptions<'_>) {
    
    
    println!("rsonpath/jsurfer Query: {}", options.query_string);
    println!("Jsonski Query: {}", options.jsonski_query_string);

    //rsonpath
    let contents = get_contents(options.path);
    let query = JsonPathQuery::parse(options.query_string).unwrap();
    let stackless = StacklessRunner::compile_query(&query);

    let mut group = c.benchmark_group(format! {"rsonpath_{}", options.id});
    group.warm_up_time(options.warm_up_time);
    group.measurement_time(options.measurement_time);
    group.throughput(criterion::Throughput::BytesDecimal(contents.len() as u64));    
    group.bench_with_input(
        BenchmarkId::new("stackless", options.id),
        &contents,
        |b, c| b.iter(|| stackless.run::<CountResult>(c)),
    );

    //jsonski
    if !options.jsonski_query_string.is_empty() {
        let jsonski_query = rust_jsonski::create_jsonski_query(options.jsonski_query_string);
        let jsonski_record = get_jsonski_record(&get_path(options.path));

        group.bench_with_input(
            BenchmarkId::new("jsonski", options.id),
            &(&jsonski_record, &jsonski_query),
            |b, &(r, q)| {
                b.iter(|| rust_jsonski::call_jsonski(q, r));
            },
        );
    }
    // jsurfer
    let context = rust_jsurfer::Jvm::attach().expect("failed to attach to Jvm");
    let jsurfer_file = context
        .load_file(&get_path(options.path))
        .expect("failed to load file via jsurfer");
    let jsurfer_query = context
        .compile_query(options.query_string)
        .expect("failed to compile query via jsurfer");
    group.bench_with_input(
        BenchmarkId::new("jsurfer", options.id),
        &(&jsurfer_file, &jsurfer_query),
        |b, &(f, q)| {
            b.iter(|| q.run(f).unwrap());
        },
    );
    group.finish();
}

pub fn wikidata_combined(c: &mut Criterion) {
    wikipedia_bench(
        c,
        BenchmarkOptions {
            path: "wikidata_compressed/wikidata_combined.json",
            query_string: "$..claims..references..hash",
            jsonski_query_string: "",
            id: "wikidata_combined",
            warm_up_time: Duration::from_secs(10),
            measurement_time: Duration::from_secs(40),
        },
    );
}

pub fn wikidata_combined_with_whitespace(c: &mut Criterion) {
    wikipedia_bench(
        c,
        BenchmarkOptions {
            path: "wikidata_prettified/wikidata_combined.json",
            query_string: "$..claims..references..hash",
            jsonski_query_string: "",
            id: "wikidata_combined_with_whitespace",
            warm_up_time: Duration::from_secs(10),
            measurement_time: Duration::from_secs(40),
        },
    );
}

pub fn wikidata_person(c: &mut Criterion) {
    wikipedia_bench(
        c,
        BenchmarkOptions {
            path: "wikidata_compressed/wikidata_person.json",
            query_string: "$..claims..references..hash",
            jsonski_query_string: "",
            id: "wikidata_person",
            warm_up_time: Duration::from_secs(3),
            measurement_time: Duration::from_secs(5),
        },
    );
}

pub fn wikidata_person_en_value_recursive(c: &mut Criterion) {
    wikipedia_bench(
        c,
        BenchmarkOptions {
            path: "wikidata_compressed/wikidata_person.json",
            query_string: "$..en..value",
            jsonski_query_string: "",
            id: "wikidata_person_en_value_recursive",
            warm_up_time: Duration::from_secs(3),
            measurement_time: Duration::from_secs(5),
        },
    );
}

pub fn wikidata_person_en_value_direct(c: &mut Criterion) {
    wikipedia_bench(
        c,
        BenchmarkOptions {
            path: "wikidata_compressed/wikidata_person.json",
            query_string: "$..en.value",
            jsonski_query_string: "",
            id: "wikidata_person_en_value_direct",
            warm_up_time: Duration::from_secs(3),
            measurement_time: Duration::from_secs(5),
        },
    );
}

pub fn wikidata_profession(c: &mut Criterion) {
    wikipedia_bench(
        c,
        BenchmarkOptions {
            path: "wikidata_compressed/wikidata_profession.json",
            query_string: "$..claims..mainsnak..value",
            jsonski_query_string: "",
            id: "wikidata_profession",
            warm_up_time: Duration::from_secs(3),
            measurement_time: Duration::from_secs(5),
        },
    );
}

pub fn wikidata_properties(c: &mut Criterion) {
    wikipedia_bench(
        c,
        BenchmarkOptions {
            path: "wikidata_compressed/wikidata_properties.json",
            query_string: "$..qualifiers..datavalue..id",
            jsonski_query_string: "",
            id: "wikidata_properties",
            warm_up_time: Duration::from_secs(3),
            measurement_time: Duration::from_secs(5),
        },
    );
}

criterion_group!(
    wikidata_benches,
    wikidata_combined,
    wikidata_combined_with_whitespace,
    wikidata_person,
    wikidata_person_en_value_recursive,
    wikidata_person_en_value_direct,
    wikidata_profession,
    wikidata_properties
);

criterion_main!(wikidata_benches);
