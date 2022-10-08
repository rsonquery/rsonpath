use criterion::{criterion_group, criterion_main, Criterion};
use itertools::Itertools;
use rsonpath::engine::result::CountResult;
use rsonpath::engine::{Input, Runner};
use rsonpath::query::JsonPathQuery;
use rsonpath::stack_based::StackBasedRunner;
use rsonpath::stackless::StacklessRunner;
use std::fs::{self, File};
use std::io::Read;
use std::path::{Path, PathBuf};

const ROOT_BENCHSET_DIRECTORY: &str = "./benches/benchset";
const QUERIES_FILE_NAME: &str = "queries.txt";

struct BenchmarkOptions<'a> {
    pub input: &'a Input,
    pub query_string: &'a str,
    pub id: &'a str,
}

fn get_contents(test_path: &Path) -> Input {
    let mut f = File::open(test_path).expect("no file found");
    let metadata = fs::metadata(&test_path).expect("unable to read metadata");
    let mut buffer = vec![0; metadata.len() as usize];
    f.read_to_end(&mut buffer).expect("buffer overflow");
    Input::new_bytes(buffer)
}

fn rsonpath_stack_based_vs_stackless(c: &mut Criterion, options: BenchmarkOptions<'_>) {
    let query = JsonPathQuery::parse(options.query_string).unwrap();

    let mut group = c.benchmark_group(format! {"rsonpath_{}", options.id});
    group.throughput(criterion::Throughput::BytesDecimal(
        options.input.len() as u64
    ));

    let stackless = StacklessRunner::compile_query(&query);
    let stack_based = StackBasedRunner::compile_query(&query);

    group.bench_with_input("stackless", &options.input, |b, c| {
        b.iter(|| stackless.run::<CountResult>(c))
    });
    group.bench_with_input("stack-based", &options.input, |b, c| {
        b.iter(|| stack_based.run::<CountResult>(c))
    });

    group.finish();
}

struct BenchmarkSet {
    pub json_file: PathBuf,
    pub queries_file: PathBuf,
}

fn run_benchset(c: &mut Criterion, benchset: &BenchmarkSet) {
    let input = get_contents(&benchset.json_file);
    let query_strings = std::fs::read_to_string(&benchset.queries_file).unwrap();
    let id_base = benchset.json_file.file_name().unwrap().to_string_lossy();

    for query_string in query_strings.lines() {
        let id = format!("{}__{}", id_base, query_string);
        rsonpath_stack_based_vs_stackless(
            c,
            BenchmarkOptions {
                input: &input,
                query_string,
                id: &id,
            },
        )
    }
}

fn benchset(c: &mut Criterion) {
    use std::fs::read_dir;

    let directories: Vec<_> = read_dir(ROOT_BENCHSET_DIRECTORY)
        .unwrap()
        .into_iter()
        .filter_map(Result::ok)
        .filter(|e| e.file_type().unwrap().is_dir())
        .map(|e| {
            read_dir(e.path())
                .unwrap()
                .filter_map(Result::ok)
                .collect::<Vec<_>>()
        })
        .collect();

    let benchsets: Vec<_> = directories
        .iter()
        .map(|e| BenchmarkSet {
            json_file: e
                .iter()
                .filter(|e| {
                    e.file_type().unwrap().is_file()
                        && e.path().extension().map_or(false, |ext| ext == "json")
                })
                .map(|e| e.path())
                .exactly_one()
                .unwrap(),
            queries_file: e
                .iter()
                .filter(|e| e.file_type().unwrap().is_file() && e.file_name() == QUERIES_FILE_NAME)
                .map(|e| e.path())
                .exactly_one()
                .unwrap(),
        })
        .collect();

    println!("Discovered benchsets:");

    for benchset in benchsets.iter() {
        println!(
            " - {}, {}",
            benchset.json_file.display(),
            benchset.queries_file.display()
        );
    }

    for benchset in benchsets {
        run_benchset(c, &benchset);
    }
}

criterion_group!(benchset_benches, benchset);

criterion_main!(benchset_benches);
