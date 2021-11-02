use core::time::Duration;
use criterion::{criterion_group, criterion_main, BenchmarkId, Criterion};
use simdpath::bytes::depth::nosimd;
use simdpath::bytes::depth::simd;
use simdpath::bytes::depth::{BytesWithDepth, DepthBlock};
use std::fs;

const ROOT_TEST_DIRECTORY: &str = "./data";

fn get_contents(test_path: &str) -> String {
    let path = format!("{}/{}", ROOT_TEST_DIRECTORY, test_path);
    fs::read_to_string(path).unwrap()
}

fn do_bench<'a, F: FnOnce(&'a [u8]) -> BytesWithDepth<'a, D>, D: DepthBlock>(
    bytes: &'a [u8],
    depth_base: isize,
    build: F,
) -> usize {
    let mut bytes_with_depth = build(bytes);
    let mut count = 0;

    loop {
        let res = bytes_with_depth.is_depth_greater_or_equal_to(depth_base);
        if res {
            count += 1;
        }

        if !bytes_with_depth.advance() {
            break;
        }
    }

    count
}

fn wikidata_combined(c: &mut Criterion) {
    let mut group = c.benchmark_group("wikidata_combined");
    group.measurement_time(Duration::from_secs(30));

    let contents = get_contents("wikidata_compressed/wikidata_combined.json");

    group.bench_with_input(
        BenchmarkId::new("nosimd", "wikidata_combined"),
        &(5, &contents),
        |b, &(d, c)| b.iter(|| do_bench(c.as_bytes(), d, nosimd::decorate_depth)),
    );
    group.bench_with_input(
        BenchmarkId::new("simd", "wikidata_combined"),
        &(5, &contents),
        |b, &(d, c)| b.iter(|| do_bench(c.as_bytes(), d, simd::decorate_depth)),
    );
    group.bench_with_input(
        BenchmarkId::new("simd_lazy", "wikidata_combined"),
        &(5, &contents),
        |b, &(d, c)| b.iter(|| do_bench(c.as_bytes(), d, simd::decorate_depth_lazy)),
    );
    group.finish();
}

criterion_group!(
    name = benches;
    config = Criterion::default();//.with_measurement(Perf::new(Builder::from_hardware_event(Hardware::RefCPUCycles)));
    targets = wikidata_combined);
criterion_main!(benches);
