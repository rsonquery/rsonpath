use core::time::Duration;
use criterion::{criterion_group, criterion_main, BenchmarkId, Criterion};
use simdpath::bytes::sequences;

const LENGTH: usize = 32 * 1024 * 1024;
const LETTERS: &str = "abcdefghijklmnopqrstuvwxyz";
const SEQUENCE: &str = "umaxzlvhjkncfidewpyqrsbgotkfsniubghjlycmqxertwdzpvoa";

fn setup_bytes() -> String {
    let mut contents = String::new();

    while contents.len() < LENGTH {
        contents += LETTERS;
    }

    contents += SEQUENCE;
    contents += LETTERS;

    while contents.len() % 32 != 0 {
        contents += "x";
    }

    contents
}

fn bench_find_byte_sequencen(c: &mut Criterion, n: usize, measurement_time: Duration) {
    let mut group = c.benchmark_group(format!("find_byte_sequence{}_bench", n));
    group.measurement_time(measurement_time);

    let contents = setup_bytes();
    let bytes = contents.as_bytes();

    group.bench_with_input(
        BenchmarkId::new(
            format!("find_byte_sequence{}_bench", n),
            format!("bench_{}", contents.len()),
        ),
        &(&SEQUENCE[..n], &bytes),
        |bench, &(s, c)| bench.iter(|| sequences::find_byte_sequence(s.as_bytes(), c)),
    );
    group.finish();
}

pub fn bench_find_byte_sequence2(c: &mut Criterion) {
    bench_find_byte_sequencen(c, 2, Duration::from_secs(15))
}

pub fn bench_find_byte_sequence3(c: &mut Criterion) {
    bench_find_byte_sequencen(c, 3, Duration::from_secs(20))
}

pub fn bench_find_byte_sequence4(c: &mut Criterion) {
    bench_find_byte_sequencen(c, 4, Duration::from_secs(25))
}

pub fn bench_find_byte_sequence8(c: &mut Criterion) {
    bench_find_byte_sequencen(c, 8, Duration::from_secs(30))
}

pub fn bench_find_byte_sequence15(c: &mut Criterion) {
    bench_find_byte_sequencen(c, 15, Duration::from_secs(55))
}

pub fn bench_find_byte_sequence16(c: &mut Criterion) {
    bench_find_byte_sequencen(c, 16, Duration::from_secs(55))
}

pub fn bench_find_byte_sequence32(c: &mut Criterion) {
    bench_find_byte_sequencen(c, 32, Duration::from_secs(90))
}

pub fn bench_find_byte_sequence33(c: &mut Criterion) {
    bench_find_byte_sequencen(c, 33, Duration::from_secs(90))
}

pub fn bench_find_byte_sequence48(c: &mut Criterion) {
    bench_find_byte_sequencen(c, 48, Duration::from_secs(90))
}

criterion_group!(
    benches,
    bench_find_byte_sequence2,
    bench_find_byte_sequence3,
    bench_find_byte_sequence4,
    bench_find_byte_sequence8,
    bench_find_byte_sequence15,
    bench_find_byte_sequence16,
    bench_find_byte_sequence32,
    bench_find_byte_sequence33,
    bench_find_byte_sequence48
);
criterion_main!(benches);
