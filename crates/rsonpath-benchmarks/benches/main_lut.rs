use rsonpath_benchmarks::prelude::*;

pub fn first_lut_test(c: &mut Criterion) -> Result<(), BenchmarkError> {
    let benchset = Benchset::new("bestbuy::products_video_only", dataset::pison_bestbuy_short())?
        .do_not_measure_file_load_time()
        .add_rsonpath_with_lut("$.products[*].videoChapters")?
        .finish();

    benchset.run(c);

    Ok(())
}

benchsets!(
    main_lut_benches, // name of the group functions
    first_lut_test, // first parameter ...
);
