use aligners::{alignment::TwoTo, AlignedBytes};
use core::time::Duration;
use criterion::{criterion_group, criterion_main, BenchmarkId, Criterion};
use decimal_byte_measurement::DecimalByteMeasurement;
use simd_benchmarks::find_byte;

const LENGTH: usize = 32 * 1024 * 1024;
const LETTERS: &str = "abcdefghijklmnopqrstuvwxyz";

fn setup_aligned_bytes() -> AlignedBytes<TwoTo<6>> {
    let mut contents = String::new();

    while contents.len() < LENGTH {
        contents += LETTERS;
    }

    contents += "X";
    contents += LETTERS;

    while contents.len() % 32 != 0 {
        contents += "X";
    }

    AlignedBytes::new_padded(contents.as_bytes())
}

fn bench_find_byte(c: &mut Criterion<DecimalByteMeasurement>) {
    let mut group = c.benchmark_group("find_byte");
    group.measurement_time(Duration::from_secs(30));
    group.throughput(criterion::Throughput::Bytes(LENGTH as u64));

    let bytes = setup_aligned_bytes();

    group.bench_with_input(
        BenchmarkId::new("nosimd", LENGTH),
        &(b'X', &bytes),
        |bench, &(b, c)| bench.iter(|| find_byte::find_byte_nosimd(b, c)),
    );
    group.bench_with_input(
        BenchmarkId::new("rust_nosimd", LENGTH),
        &(b'X', &bytes),
        |bench, &(b, c)| bench.iter(|| find_byte::find_byte_rust_nosimd(b, c)),
    );
    group.bench_with_input(
        BenchmarkId::new("size128", LENGTH),
        &(b'X', &bytes),
        |bench, &(b, c)| bench.iter(|| find_byte::find_byte_size128(b, c.relax_alignment())),
    );
    group.bench_with_input(
        BenchmarkId::new("size256", LENGTH),
        &(b'X', &bytes),
        |bench, &(b, c)| bench.iter(|| find_byte::find_byte_size256(b, c.relax_alignment())),
    );
    #[cfg(feature = "avx512")]
    group.bench_with_input(
        BenchmarkId::new("size512", LENGTH),
        &(b'X', &bytes),
        |bench, &(b, c)| bench.iter(|| find_byte::find_byte_size512(b, c.relax_alignment())),
    );
    group.bench_with_input(
        BenchmarkId::new("memchr", LENGTH),
        &(b'X', &bytes),
        |bench, &(b, c)| bench.iter(|| memchr::memchr(b, c)),
    );
    group.finish();
}

fn decimal_byte_measurement() -> Criterion<DecimalByteMeasurement> {
    Criterion::default().with_measurement(DecimalByteMeasurement::new())
}

criterion_group!(
    name = benches;
    config = decimal_byte_measurement();
    targets = bench_find_byte
);
criterion_main!(benches);
