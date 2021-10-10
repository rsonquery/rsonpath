use core::time::Duration;
use criterion::{criterion_group, criterion_main, BenchmarkId, Criterion};
use simdpath::bytes::nosimd;
use simdpath::bytes::simd;

const BYTE1: u8 = b'y';
const BYTE2: u8 = b'x';
const LENGTH: usize = 32 * 1024 * 1024;
const LETTERS: &str = "abcdefghijklmnopqrstuvwxyz";

fn setup_bytes() -> String {
    let mut contents = String::new();

    while contents.len() < LENGTH {
        contents += LETTERS;
    }

    contents += "y";
    contents += "x";
    contents += LETTERS;

    while contents.len() % 32 != 0 {
        contents += "x";
    }

    contents
}

pub fn bench(c: &mut Criterion) {
    let mut group = c.benchmark_group("find_byte_sequence2_bench");
    group.measurement_time(Duration::from_secs(10));

    let contents = setup_bytes();
    let bytes = contents.as_bytes();

    group.bench_with_input(
        BenchmarkId::new(
            "simd::find_byte_sequence2",
            format!("bench_{}", contents.len()),
        ),
        &(BYTE1, BYTE2, &bytes),
        |bench, &(b1, b2, c)| bench.iter(|| simd::find_byte_sequence2(b1, b2, c)),
    );
    group.bench_with_input(
        BenchmarkId::new(
            "nosimd::find_byte_sequence2",
            format!("bench_{}", contents.len()),
        ),
        &(BYTE1, BYTE2, &bytes),
        |bench, &(b1, b2, c)| bench.iter(|| nosimd::find_byte_sequence2(b1, b2, c)),
    );
    group.finish();
}

criterion_group!(benches, bench);
criterion_main!(benches);
