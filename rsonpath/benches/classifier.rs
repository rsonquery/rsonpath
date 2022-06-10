use criterion::{black_box, criterion_group, criterion_main, Criterion};
use decimal_byte_measurement::DecimalByteMeasurement;
use rsonpath::classify::{self, Structural};
use rsonpath::engine::Input;
use std::fs;

const ROOT_TEST_DIRECTORY: &str = "./data";

type CriterionCtx = Criterion<DecimalByteMeasurement>;

fn get_contents(test_path: &str) -> Input {
    let path = format!("{}/{}", ROOT_TEST_DIRECTORY, test_path);
    let raw = fs::read_to_string(path).unwrap();
    Input::new(raw)
}

fn classifier_benches(c: &mut CriterionCtx, path: &str, id: &str) {
    let contents = get_contents(path);

    let mut group = c.benchmark_group(id);
    group.throughput(criterion::Throughput::Bytes(contents.len() as u64));

    group.bench_function("classifier", |b| {
        b.iter_batched(
            || {
                let iter =
                    classify::classify_structural_characters(contents.as_ref().relax_alignment());
                iter
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

fn decimal_byte_measurement() -> CriterionCtx {
    Criterion::default().with_measurement(DecimalByteMeasurement::new())
}

pub fn wikidata_compressed(c: &mut CriterionCtx) {
    classifier_benches(
        c,
        "wikidata_compressed/wikidata_combined.json",
        "compressed",
    );
}
pub fn wikidata_prettified(c: &mut CriterionCtx) {
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
