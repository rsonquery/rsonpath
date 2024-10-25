use rsonpath_benchmarks::prelude::*;

pub fn canada_second_coord_component(c: &mut Criterion) -> Result<(), BenchmarkError> {
    let benchset: rsonpath_benchmarks::framework::ConfiguredBenchset =
        Benchset::new("canada::second_coord_component", dataset::nativejson_canada())?
            .do_not_measure_file_load_time()
            .add_rsonpath_with_all_result_types("$.features[*].geometry.coordinates[*][*][1]")?
            .finish();

    benchset.run(c);

    Ok(())
}

pub fn canada_coord_476_1446_1(c: &mut Criterion) -> Result<(), BenchmarkError> {
    let benchset: rsonpath_benchmarks::framework::ConfiguredBenchset =
        Benchset::new("canada::coord_476_1446_1", dataset::nativejson_canada())?
            .do_not_measure_file_load_time()
            .add_rsonpath_with_all_result_types("$..coordinates[476][1446][1]")?
            .finish();

    benchset.run(c);

    Ok(())
}

pub fn canada_coord_slice_100_to_200(c: &mut Criterion) -> Result<(), BenchmarkError> {
    let benchset: rsonpath_benchmarks::framework::ConfiguredBenchset =
        Benchset::new("canada::coord_slice_100_to_200", dataset::nativejson_canada())?
            .do_not_measure_file_load_time()
            .add_rsonpath_with_all_result_types("$..coordinates[100:201][*][*]")?
            .finish();

    benchset.run(c);

    Ok(())
}

pub fn canada_coord_slice_overlapping(c: &mut Criterion) -> Result<(), BenchmarkError> {
    let benchset: rsonpath_benchmarks::framework::ConfiguredBenchset =
        Benchset::new("canada::coord_slice_overlapping", dataset::nativejson_canada())?
            .do_not_measure_file_load_time()
            .add_rsonpath_with_all_result_types("$..coordinates[5::7][3::10][*]")?
            .finish();

    benchset.run(c);

    Ok(())
}

pub fn citm_seat_category(c: &mut Criterion) -> Result<(), BenchmarkError> {
    let benchset: rsonpath_benchmarks::framework::ConfiguredBenchset =
        Benchset::new("citm::seatCategoryId", dataset::nativejson_citm())?
            .do_not_measure_file_load_time()
            .add_rsonpath_with_all_result_types("$..seatCategoryId")?
            .finish();

    benchset.run(c);

    Ok(())
}

pub fn ast_nested_inner(c: &mut Criterion) -> Result<(), BenchmarkError> {
    let benchset = Benchset::new("ast::nested_inner", dataset::ast())?
        .do_not_measure_file_load_time()
        .add_rsonpath_with_all_result_types("$..inner..inner..type.qualType")?
        .finish();

    benchset.run(c);

    Ok(())
}

pub fn ast_deepest(c: &mut Criterion) -> Result<(), BenchmarkError> {
    let benchset = Benchset::new("ast::deepest", dataset::ast())?
        .do_not_measure_file_load_time()
        .add_rsonpath_with_all_result_types("$..*..*..*..*..*..*..*..*..*..*..*..*..*..*..*..*..*..*..*..*..*..*..*..*..*..*..*..*..*..*..*..*..*..*..*..*..*..*..*..*..*..*..*..*..*..*..*..*..*..*..*..*..*..*..*..*..*..*..*..*..*..*..*..*..*..*..*..*..*..*..*..*..*..*..*..*..*..*..*..*..*..*..*..*..*..*..*..*..*..*..*..*..*..*..*..*..*..*..*..*..*")?
        .finish();

    benchset.run(c);

    Ok(())
}

pub fn bestbuy_products_category_slice(c: &mut Criterion) -> Result<(), BenchmarkError> {
    let benchset = Benchset::new("bestbuy::products_category", dataset::pison_bestbuy_short())?
        .do_not_measure_file_load_time()
        .add_rsonpath_with_all_result_types("$.products[*].categoryPath[1:3].id")?
        .finish();

    benchset.run(c);

    Ok(())
}

pub fn bestbuy_products_video_only(c: &mut Criterion) -> Result<(), BenchmarkError> {
    let benchset = Benchset::new("bestbuy::products_video_only", dataset::pison_bestbuy_short())?
        .do_not_measure_file_load_time()
        .add_target_with_id(
            BenchTarget::Rsonpath("$.products[*].videoChapters", ResultType::Count),
            "rsonpath_direct_count",
        )?
        .add_target_with_id(
            BenchTarget::Rsonpath("$..videoChapters", ResultType::Count),
            "rsonpath_descendant_count",
        )?
        .add_target_with_id(
            BenchTarget::Rsonpath("$.products[*].videoChapters", ResultType::Full),
            "rsonpath_direct_nodes",
        )?
        .add_target_with_id(
            BenchTarget::Rsonpath("$..videoChapters", ResultType::Full),
            "rsonpath_descendant_nodes",
        )?
        .finish();

    benchset.run(c);

    Ok(())
}

pub fn bestbuy_all_nodes(c: &mut Criterion) -> Result<(), BenchmarkError> {
    let benchset = Benchset::new("bestbuy::all_nodes", dataset::pison_bestbuy_short())?
        .do_not_measure_file_load_time()
        .add_rsonpath_with_all_result_types("$..*")?
        .finish();

    benchset.run(c);

    Ok(())
}

pub fn google_map_routes(c: &mut Criterion) -> Result<(), BenchmarkError> {
    let benchset = Benchset::new("google_map::routes", dataset::pison_google_map_short())?
        .do_not_measure_file_load_time()
        .add_rsonpath_with_all_result_types("$[*].routes[*].legs[*].steps[*].distance.text")?
        .finish();

    benchset.run(c);

    Ok(())
}

pub fn google_map_travel_modes(c: &mut Criterion) -> Result<(), BenchmarkError> {
    let benchset = Benchset::new("google_map::travel_modes", dataset::pison_google_map_short())?
        .do_not_measure_file_load_time()
        .add_target_with_id(
            BenchTarget::Rsonpath("$[*].available_travel_modes", ResultType::Count),
            "rsonpath_direct_count",
        )?
        .add_target_with_id(
            BenchTarget::Rsonpath("$..available_travel_modes", ResultType::Count),
            "rsonpath_descendant_count",
        )?
        .add_target_with_id(
            BenchTarget::Rsonpath("$[*].available_travel_modes", ResultType::Full),
            "rsonpath_direct_nodes",
        )?
        .add_target_with_id(
            BenchTarget::Rsonpath("$..available_travel_modes", ResultType::Full),
            "rsonpath_descendant_nodes",
        )?
        .finish();

    benchset.run(c);

    Ok(())
}

pub fn walmart_items_name(c: &mut Criterion) -> Result<(), BenchmarkError> {
    let benchset = Benchset::new("walmart::items_name", dataset::pison_walmart_short())?
        .do_not_measure_file_load_time()
        .add_target_with_id(
            BenchTarget::Rsonpath("$.items[*].name", ResultType::Count),
            "rsonpath_direct_count",
        )?
        .add_target_with_id(
            BenchTarget::Rsonpath("$..items_name", ResultType::Count),
            "rsonpath_descendant_count",
        )?
        .add_target_with_id(
            BenchTarget::Rsonpath("$.items[*].name", ResultType::Full),
            "rsonpath_direct_nodes",
        )?
        .add_target_with_id(
            BenchTarget::Rsonpath("$..items_name", ResultType::Full),
            "rsonpath_descendant_nodes",
        )?
        .finish();

    benchset.run(c);

    Ok(())
}

pub fn twitter_metadata(c: &mut Criterion) -> Result<(), BenchmarkError> {
    let benchset = Benchset::new("twitter::metadata", dataset::twitter())?
        .do_not_measure_file_load_time()
        .add_target_with_id(
            BenchTarget::Rsonpath("$.search_metadata.count", ResultType::Count),
            "rsonpath_direct_count",
        )?
        .add_target_with_id(
            BenchTarget::Rsonpath("$..count", ResultType::Count),
            "rsonpath_descendant_count",
        )?
        .add_target_with_id(
            BenchTarget::Rsonpath("$.search_metadata.count", ResultType::Full),
            "rsonpath_direct_nodes",
        )?
        .add_target_with_id(
            BenchTarget::Rsonpath("$..count", ResultType::Full),
            "rsonpath_descendant_nodes",
        )?
        .finish();

    benchset.run(c);

    Ok(())
}

pub fn inner_array(c: &mut Criterion) -> Result<(), BenchmarkError> {
    let benchset = Benchset::new("inner_array", dataset::ast())?
        .do_not_measure_file_load_time()
        .add_rsonpath_with_all_result_types("$..inner[0]")?
        .finish();

    benchset.run(c);

    Ok(())
}

pub fn user_second_mention_index(c: &mut Criterion) -> Result<(), BenchmarkError> {
    let benchset = Benchset::new("user_mentions_indices", dataset::twitter())?
        .do_not_measure_file_load_time()
        .add_rsonpath_with_all_result_types("$..entities.user_mentions[1]")?
        .finish();

    benchset.run(c);

    Ok(())
}

pub fn all_first_index(c: &mut Criterion) -> Result<(), BenchmarkError> {
    let benchset = Benchset::new("all_first_index", dataset::twitter())?
        .do_not_measure_file_load_time()
        .add_rsonpath_with_all_result_types("$..[0]")?
        .finish();

    benchset.run(c);

    Ok(())
}

benchsets!(
    main_benches,
    canada_second_coord_component,
    canada_coord_476_1446_1,
    canada_coord_slice_100_to_200,
    canada_second_coord_component,
    citm_seat_category,
    ast_nested_inner,
    ast_deepest,
    bestbuy_products_category_slice,
    bestbuy_products_video_only,
    bestbuy_all_nodes,
    google_map_routes,
    google_map_travel_modes,
    inner_array,
    user_second_mention_index,
    walmart_items_name,
    twitter_metadata,
    all_first_index
);
