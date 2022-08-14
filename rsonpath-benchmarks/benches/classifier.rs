use criterion::{black_box, criterion_group, criterion_main};
use criterion_decimal_throughput::{decimal_byte_measurement, Criterion};
use rsonpath::classify::{self};
use rsonpath::engine::Input;
use rsonpath::quotes;
use std::fs;

const ROOT_TEST_DIRECTORY: &str = "../data";

fn get_contents(test_path: &str) -> Input {
    let path = format!("{}/{}", ROOT_TEST_DIRECTORY, test_path);
    let raw = fs::read_to_string(path).unwrap();
    Input::new(raw)
}

fn classifier_benches(c: &mut Criterion, path: &str, id: &str) {
    let contents = get_contents(path);

    let mut group = c.benchmark_group(id);
    group.throughput(criterion::Throughput::Bytes(contents.len() as u64));

    group.bench_function("classifier", |b| {
        b.iter_batched(
            || {
                let quote_iter =
                    quotes::classify_quoted_sequences(contents.as_ref().relax_alignment());
                classify::classify_structural_characters(quote_iter)
            },
            |iter| {
                for elem in iter {
                    black_box(elem);
                }
            },
            criterion::BatchSize::SmallInput,
        )
    });

    group.finish();
}

pub fn wikidata_compressed(c: &mut Criterion) {
    classifier_benches(
        c,
        "wikidata_compressed/wikidata_combined.json",
        "compressed",
    );
}
pub fn wikidata_prettified(c: &mut Criterion) {
    classifier_benches(
        c,
        "wikidata_prettified/wikidata_combined.json",
        "prettified",
    );
}

criterion_group!(
    name = wikidata_benches;
    config = decimal_byte_measurement();
    targets = wikidata_compressed, wikidata_prettified
);

criterion_main!(wikidata_benches);
