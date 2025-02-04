use rsonpath_benchmarks::prelude::*;

pub fn bench_0(c: &mut Criterion) -> Result<(), BenchmarkError> {
    let id = "bestbuy::products_video_only";
    let dataset = dataset::pison_bestbuy_short();
    let query = "$.products[*].videoChapters";
    let benchset = Benchset::new(id, dataset)?
        .do_not_measure_file_load_time()
        .add_rsonpath_with_lut(query)?
        .finish();

    benchset.run(c);

    Ok(())
}

pub fn bench_1(c: &mut Criterion) -> Result<(), BenchmarkError> {
    let id = "twitter:test_0";
    let dataset = dataset::twitter();
    let query = "$.search_metadata.count";
    let benchset = Benchset::new(id, dataset)?
        .do_not_measure_file_load_time()
        .add_rsonpath_with_lut(query)?
        .finish();

    benchset.run(c);

    Ok(())
}

// TODO Ricardo add:
// Queries that skip nothing
// Queries that skip everything
// Queries that skip 10% to 90% in 10% steps

benchsets!(
    main_lut_benches, // name of the group functions
    bench_0,          // first parameter ...
                      // bench_1,
);
