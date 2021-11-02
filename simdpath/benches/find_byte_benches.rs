use core::time::Duration;
use criterion::{criterion_group, criterion_main, BenchmarkId, Criterion};
use memchr;
use simdpath::bytes::nosimd;
use simdpath::bytes::simd;

const LENGTH: usize = 32 * 1024 * 1024;
const LETTERS: &str = "abcdefghijklmnopqrstuvwxyz";

fn setup_bytes() -> String {
    let mut contents = String::new();

    while contents.len() < LENGTH {
        contents += LETTERS;
    }

    contents += "X";
    contents += LETTERS;

    while contents.len() % 32 != 0 {
        contents += "X";
    }

    contents
}

#[target_feature(enable = "avx2")]
unsafe fn simd_find_byte(byte: u8, bytes: &[u8]) -> Option<usize> {
    #[cfg(all(target_arch = "x86", target_feature = "avx2"))]
    use core::arch::x86::*;
    #[cfg(all(target_arch = "x86_64", target_feature = "avx2"))]
    use core::arch::x86_64::*;
    const BYTES_IN_AVX2_REGISTER: usize = 256 / 8;

    let byte_mask = _mm256_set1_epi8(byte as i8);
    let mut bytes = bytes;
    let mut i = 0;

    while bytes.len() >= BYTES_IN_AVX2_REGISTER {
        let block = _mm256_loadu_si256(bytes.as_ptr() as *const __m256i);
        let cmp_vector = _mm256_cmpeq_epi8(block, byte_mask);
        let cmp_packed = _mm256_movemask_epi8(cmp_vector);

        if cmp_packed != 0 {
            return Some(i * 32 + (cmp_packed.trailing_zeros() as usize));
        }

        i += 1;
        bytes = &bytes[BYTES_IN_AVX2_REGISTER..];
    }

    nosimd::find_byte(byte, bytes)
}

fn bench_find_byte(c: &mut Criterion) {
    let mut group = c.benchmark_group("find_byte_bench");
    group.measurement_time(Duration::from_secs(30));

    let contents = setup_bytes();
    let bytes = contents.as_bytes();

    group.bench_with_input(
        BenchmarkId::new("memchr_bench", format!("bench_{}", contents.len())),
        &(b'X', &bytes),
        |bench, &(b, c)| bench.iter(|| memchr::memchr(b, c)),
    );
    group.bench_with_input(
        BenchmarkId::new("simd::find_byte_bench", format!("bench_{}", contents.len())),
        &(b'X', &bytes),
        |bench, &(b, c)| bench.iter(|| unsafe { simd_find_byte(b, c) }),
    );
    group.bench_with_input(
        BenchmarkId::new(
            "nosimd::find_byte_bench",
            format!("bench_{}", contents.len()),
        ),
        &(b'X', &bytes),
        |bench, &(b, c)| bench.iter(|| nosimd::find_byte(b, c)),
    );
    group.finish();
}

criterion_group!(benches, bench_find_byte);
criterion_main!(benches);
