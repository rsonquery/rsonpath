use core::time::Duration;
use criterion::{criterion_group, criterion_main, BenchmarkId, Criterion};
use simd_benchmarks::depth::{self, DepthBlock};
use std::fs;

const ROOT_TEST_DIRECTORY: &str = "./data";

fn get_contents(test_path: &str) -> String {
    let path = format!("{}/{}", ROOT_TEST_DIRECTORY, test_path);
    fs::read_to_string(path).unwrap()
}

fn do_sequential_bench(bytes: &[u8], depth_base: isize) -> usize {
    let mut count = 0;
    let mut depth = 0;

    for byte in bytes {
        match byte {
            b'{' => depth += 1,
            b'[' => depth += 1,
            b'}' => depth -= 1,
            b']' => depth -= 1,
            _ => (),
        }

        if depth >= depth_base {
            count += 1;
        }
    }

    count
}

fn do_bench<'a, F: Fn(&'a [u8]) -> (D, &'a [u8]), D: DepthBlock<'a>>(
    bytes: &'a [u8],
    depth_base: isize,
    build: F,
) -> usize {
    let mut bytes = bytes;
    let mut count = 0;
    let mut accumulated_depth = 0;

    while !bytes.is_empty() {
        let (mut vector, rem) = build(bytes);
        bytes = rem;

        let adjusted_depth = depth_base - accumulated_depth;
        loop {
            if vector.is_depth_greater_or_equal_to(adjusted_depth) {
                count += 1;
            }

            if !vector.advance() {
                break;
            }
        }

        accumulated_depth += vector.depth_at_end();
    }

    assert_eq!(69417863, count);
    count
}

fn wikidata_combined(c: &mut Criterion) {
    let mut group = c.benchmark_group("wikidata_combined");
    group.measurement_time(Duration::from_secs(30));

    let contents = get_contents("wikidata_compressed/wikidata_combined.json");

    group.bench_with_input(
        BenchmarkId::new("nosimd", "wikidata_combined"),
        &(5, &contents),
        |b, &(d, c)| b.iter(|| do_sequential_bench(c.as_bytes(), d)),
    );
    group.bench_with_input(
        BenchmarkId::new("simd", "wikidata_combined"),
        &(5, &contents),
        |b, &(d, c)| b.iter(|| do_bench(c.as_bytes(), d, depth::Avx2Vector::new)),
    );
    group.bench_with_input(
        BenchmarkId::new("simd_lazy", "wikidata_combined"),
        &(5, &contents),
        |b, &(d, c)| b.iter(|| do_bench(c.as_bytes(), d, depth::LazyAvx2Vector::new)),
    );
    group.finish();
}

criterion_group!(
    name = benches;
    config = Criterion::default();
    targets = wikidata_combined);
criterion_main!(benches);
