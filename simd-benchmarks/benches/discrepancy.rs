use aligners::{alignment::TwoTo, AlignedBytes};
use core::time::Duration;
use criterion::{criterion_group, criterion_main, BenchmarkId, Criterion, Throughput};
use rand::prelude::*;
use simd_benchmarks::discrepancy;

const LENGTH: usize = 4 * 1024 * 1024;

fn setup_aligned_bytes() -> AlignedBytes<TwoTo<6>> {
    let mut rng = StdRng::seed_from_u64(213769420);
    AlignedBytes::new_initialize(LENGTH, move |_| rng.gen())
}

fn discrepancy_benches(c: &mut Criterion) {
    let mut group = c.benchmark_group("discrepancy");
    group.throughput(Throughput::Bytes(LENGTH as u64));
    group.measurement_time(Duration::from_secs(60));

    let bytes1 = setup_aligned_bytes();
    let mut bytes2 = bytes1.clone();
    let last_byte = bytes2.last_mut().unwrap();
    *last_byte = if bytes1.last().copied().unwrap() == 0 {
        1
    } else {
        0
    };

    group.bench_with_input(
        BenchmarkId::new("size8", LENGTH),
        &(&bytes1, &bytes2),
        |bench, &(a, b)| bench.iter(|| discrepancy::discrepancy_size8(a, b)),
    );
    group.bench_with_input(
        BenchmarkId::new("size64", LENGTH),
        &(&bytes1, &bytes2),
        |bench, &(a, b)| {
            bench.iter(|| discrepancy::discrepancy_size64(a.relax_alignment(), b.relax_alignment()))
        },
    );
    group.bench_with_input(
        BenchmarkId::new("size128", LENGTH),
        &(&bytes1, &bytes2),
        |bench, &(a, b)| {
            bench
                .iter(|| discrepancy::discrepancy_size128(a.relax_alignment(), b.relax_alignment()))
        },
    );
    group.bench_with_input(
        BenchmarkId::new("size256", LENGTH),
        &(&bytes1, &bytes2),
        |bench, &(a, b)| {
            bench
                .iter(|| discrepancy::discrepancy_size256(a.relax_alignment(), b.relax_alignment()))
        },
    );
    #[cfg(feature = "avx512")]
    group.bench_with_input(
        BenchmarkId::new("size512", LENGTH),
        &(&bytes1, &bytes2),
        |bench, &(a, b)| bench.iter(|| discrepancy::discrepancy_size512(a, b)),
    );

    group.finish();
}

fn decimal_byte_measurement() -> Criterion<DecimalByteMeasurement> {
    Criterion::default().with_measurement(DecimalByteMeasurement(WallTime))
}

criterion_group!(
    name = benches;
    config = decimal_byte_measurement();
    targets = discrepancy_benches
);
criterion_main!(benches);
