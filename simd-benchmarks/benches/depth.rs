use aligners::{
    alignment::{Alignment, TwoTo},
    AlignedBytes, AlignedSlice,
};
use core::time::Duration;
use criterion::{criterion_group, criterion_main, BenchmarkId, Throughput};
use criterion_decimal_throughput::{decimal_byte_measurement, Criterion};
use simd_benchmarks::depth::{self, DepthBlock};
use std::fs;

const ROOT_TEST_DIRECTORY: &str = "../rsonpath/data";

fn get_contents(test_path: &str) -> String {
    let path = format!("{}/{}", ROOT_TEST_DIRECTORY, test_path);
    fs::read_to_string(path).unwrap()
}

fn do_bench<
    'a,
    A: Alignment,
    F: Fn(&'a AlignedSlice<A>) -> (D, &'a AlignedSlice<A>),
    D: DepthBlock<'a>,
>(
    bytes: &'a AlignedSlice<A>,
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

    count
}

fn wikidata_combined(c: &mut Criterion) {
    let mut group = c.benchmark_group("wikidata_combined");

    let contents = get_contents("wikidata_compressed/wikidata_combined.json");
    let bytes: AlignedBytes<TwoTo<6>> = contents.as_bytes().into();
    group.measurement_time(Duration::from_secs(30));
    group.throughput(Throughput::Bytes(bytes.len() as u64));

    group.bench_with_input(
        BenchmarkId::new("nosimd", "wikidata_combined"),
        &(5, &bytes),
        |b, &(d, c)| {
            b.iter(|| {
                do_bench(c, d, |x| {
                    (depth::nosimd::Vector::new(x), Default::default())
                })
            })
        },
    );
    group.bench_with_input(
        BenchmarkId::new("sse2", "wikidata_combined"),
        &(5, bytes.relax_alignment()),
        |b, &(d, c)| b.iter(|| do_bench(c, d, depth::sse2::Vector::new)),
    );
    group.bench_with_input(
        BenchmarkId::new("sse2_lazy", "wikidata_combined"),
        &(5, bytes.relax_alignment()),
        |b, &(d, c)| b.iter(|| do_bench(c, d, depth::sse2::LazyVector::new)),
    );
    group.bench_with_input(
        BenchmarkId::new("avx2", "wikidata_combined"),
        &(5, bytes.relax_alignment()),
        |b, &(d, c)| b.iter(|| do_bench(c, d, depth::avx2::Vector::new)),
    );
    group.bench_with_input(
        BenchmarkId::new("avx2_lazy", "wikidata_combined"),
        &(5, bytes.relax_alignment()),
        |b, &(d, c)| b.iter(|| do_bench(c, d, depth::avx2::LazyVector::new)),
    );
    #[cfg(feature = "avx512")]
    group.bench_with_input(
        BenchmarkId::new("avx512_lazy", "wikidata_combined"),
        &(5, &bytes),
        |b, &(d, c)| b.iter(|| do_bench(c, d, depth::avx512::LazyVector::new)),
    );
    group.finish();
}

criterion_group!(
    name = benches;
    config = decimal_byte_measurement();
    targets = wikidata_combined);
criterion_main!(benches);
