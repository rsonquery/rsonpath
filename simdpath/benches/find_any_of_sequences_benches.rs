use core::time::Duration;
use criterion::{criterion_group, criterion_main, BenchmarkId, Criterion};
use simdpath::bytes::sequences;

const LENGTH: usize = 32 * 1024 * 1024;

struct BenchmarkOptions<'a> {
    pub sequences: &'a [&'a [u8]],
    pub contents: &'a [u8],
    pub group_name: &'a str,
    pub bench_name: &'a str,
    pub measurement_time: Duration,
}

fn bench(c: &mut Criterion, options: BenchmarkOptions) {
    let mut group = c.benchmark_group(options.group_name);
    group.measurement_time(options.measurement_time);

    group.bench_with_input(
        BenchmarkId::new(
            "find_any_of_sequences",
            format!("{}_{}", options.bench_name, options.contents.len()),
        ),
        &(options.sequences, options.contents),
        |bench, &(s, c)| bench.iter(|| sequences::find_any_of_sequences(s, c).unwrap()),
    );
    group.finish();
}

fn bench_find_any_of_sequences_no_partial_matches(c: &mut Criterion) {
    let sequences = [
        "aaaaaaaa".as_bytes(),
        "bbbbbbbb".as_bytes(),
        "cccccccc".as_bytes(),
        "dddddddd".as_bytes(),
        "eeeeeeee".as_bytes(),
        "ffffffff".as_bytes(),
        "gggggggg".as_bytes(),
        "hhhhhhhh".as_bytes(),
    ];
    let contents = std::iter::repeat(b'x')
        .take(LENGTH - 8)
        .chain(sequences[7].iter().copied())
        .collect::<Vec<_>>();

    bench(
        c,
        BenchmarkOptions {
            sequences: &sequences,
            contents: &contents,
            group_name: "find_any_of_sequences_no_partial_matches_bench",
            bench_name: "bench_no_partial_matches",
            measurement_time: Duration::from_secs(65),
        },
    );
}

fn bench_find_any_of_sequences_ten_percent_partial_matches(c: &mut Criterion) {
    let sequences = [
        "aaaaaaab".as_bytes(),
        "aaaaaaac".as_bytes(),
        "aaaaaaad".as_bytes(),
        "aaaaaaae".as_bytes(),
        "aaaaaaaf".as_bytes(),
        "aaaaaaag".as_bytes(),
        "aaaaaaah".as_bytes(),
        "aaaaaaai".as_bytes(),
    ];
    let contents = std::iter::repeat("aaaaxxxxxx".as_bytes())
        .flatten()
        .take(LENGTH - 8)
        .chain(sequences[7].iter())
        .copied()
        .collect::<Vec<_>>();

    bench(
        c,
        BenchmarkOptions {
            sequences: &sequences,
            contents: &contents,
            group_name: "find_any_of_sequences_ten_percent_partial_matches_bench",
            bench_name: "bench_ten_percent_partial_matches",
            measurement_time: Duration::from_secs(65),
        },
    );
}

fn bench_find_any_of_sequences_twenty_percent_partial_matches(c: &mut Criterion) {
    let sequences = [
        "aaaaaaab".as_bytes(),
        "aaaaaaac".as_bytes(),
        "aaaaaaad".as_bytes(),
        "aaaaaaae".as_bytes(),
        "aaaaaaaf".as_bytes(),
        "aaaaaaag".as_bytes(),
        "aaaaaaah".as_bytes(),
        "aaaaaaai".as_bytes(),
    ];
    let contents = std::iter::repeat("aaaax".as_bytes())
        .flatten()
        .take(LENGTH - 8)
        .chain(sequences[7].iter())
        .copied()
        .collect::<Vec<_>>();

    bench(
        c,
        BenchmarkOptions {
            sequences: &sequences,
            contents: &contents,
            group_name: "find_any_of_sequences_twenty_percent_partial_matches_bench",
            bench_name: "bench_twenty_percent_partial_matches",
            measurement_time: Duration::from_secs(65),
        },
    );
}

criterion_group!(
    benches,
    bench_find_any_of_sequences_no_partial_matches,
    bench_find_any_of_sequences_ten_percent_partial_matches,
    bench_find_any_of_sequences_twenty_percent_partial_matches
);

criterion_main!(benches);
