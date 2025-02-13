use rsonpath_benchmarks::prelude::*;

pub fn bestbuy_q0(c: &mut Criterion) -> Result<(), BenchmarkError> {
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

// TODO Ricardo add:
// Queries that skip nothing
// Queries that skip everything
// Queries that skip 10% to 90% in 10% steps

pub fn bestbuy_short_q0(c: &mut Criterion) -> Result<(), BenchmarkError> {
    let id = "bestbuy_short::q0";
    let dataset = dataset::pison_bestbuy_short();
    let query = "$.products[*].videoChapters";
    let benchset = Benchset::new(id, dataset)?
        .do_not_measure_file_load_time()
        .add_rsonpath_with_lut(query)?
        .finish();

    benchset.run(c);

    Ok(())
}

pub fn bestbuy_short_q1(c: &mut Criterion) -> Result<(), BenchmarkError> {
    let id = "bestbuy_short::q1";
    let dataset = dataset::pison_bestbuy_short();
    let query = "$.products[*].categoryPath[2]";
    let benchset = Benchset::new(id, dataset)?
        .do_not_measure_file_load_time()
        .add_rsonpath_with_lut(query)?
        .finish();

    benchset.run(c);

    Ok(())
}

pub fn google_map_short_q0(c: &mut Criterion) -> Result<(), BenchmarkError> {
    let id = "google_map_short::q0";
    let dataset = dataset::pison_google_map_short();
    let query = "$[*].routes[*].legs[*].steps[*].distance.text";
    let benchset = Benchset::new(id, dataset)?
        .do_not_measure_file_load_time()
        .add_rsonpath_with_lut(query)?
        .finish();

    benchset.run(c);

    Ok(())
}

pub fn google_map_q0(c: &mut Criterion) -> Result<(), BenchmarkError> {
    let id = "google_map::q0";
    let dataset = dataset::pison_google_map_large();
    let query = "$[*].available_travel_modes";
    let benchset = Benchset::new(id, dataset)?
        .do_not_measure_file_load_time()
        .add_rsonpath_with_lut(query)?
        .finish();

    benchset.run(c);

    Ok(())
}

pub fn twitter_large_q0(c: &mut Criterion) -> Result<(), BenchmarkError> {
    let id = "twitter_large::q0";
    let dataset = dataset::pison_twitter_large();
    let query = "$.search_metadata.count";
    let benchset = Benchset::new(id, dataset)?
        .do_not_measure_file_load_time()
        .add_rsonpath_with_lut(query)?
        .finish();

    benchset.run(c);

    Ok(())
}

benchsets!(
    main_lut_benches, // name of the group functions
    // bestbuy_short_q0,
    // bestbuy_short_q1,
    // google_map_short_q0,
    // twitter_large_q0,
    google_map_q0,
    // bestbuy_q0,
    // twitter_q0,
);
