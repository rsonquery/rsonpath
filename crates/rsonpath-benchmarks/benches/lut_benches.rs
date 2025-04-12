use paste::paste;
use rsonpath::lookup_table::performance::{lut_query_data::*, lut_skip_evaluation::get_filename};
use rsonpath_benchmarks::prelude::*;
use std::fmt::format;

// Macro to generate benchmarks for different datasets
macro_rules! generate_benchmarks {
    ($dataset_fn:ident, $prefix:literal, $($id:literal, $query:expr),*) => {
        $(
            paste! {
                pub fn [<$prefix _bench_ $id>](c: &mut Criterion) -> Result<(), BenchmarkError> {
                    let dataset = dataset::$dataset_fn();
                    let benchset = Benchset::new(concat!($prefix, "::q", stringify!($id)), dataset)?
                        .do_not_measure_file_load_time()
                        .add_rsonpath_with_lut($query)?
                        .finish();

                    benchset.run(c);
                    Ok(())
                }
            }
        )*
    };
}

// Generate Google Map benchmarks
generate_benchmarks!(
    pison_google_map_large,
    "google_map",
    100,
    "$[*].routes[*].legs[*]",
    101,
    "$[*].routes[*].legs[*].steps[*]",
    102,
    "$[*].routes[*].legs[*].steps[*].distance",
    103,
    "$[*].routes[*].legs[*].steps[*].distance.text",
    104,
    "$[*].routes[*].legs[*].steps[*].distance.value",
    108,
    "$[*].routes[*].legs[*].steps[*].duration",
    109,
    "$[*].routes[*].legs[*].steps[*].polyline",
    110,
    "$[*].routes[*].legs[*].steps[*].polyline.points",
    111,
    "$[*].routes[*].legs[*].steps[*].end_location",
    112,
    "$[*].routes[*].legs[*].steps[*].html_instructions",
    113,
    "$[*].routes[*].legs[*].steps[*].travel_mode",
    114,
    "$[*].routes[*].legs[*].steps[*].start_location",
    115,
    "$[*].routes[*].legs[*].steps[*].start_location.lat",
    116,
    "$[*].routes[*].legs[*].steps[*].start_location.lng",
    117,
    "$[*].routes[*].legs[*].steps[*].maneuver",
    118,
    "$[*].routes[*].legs[*]..lat",
    119,
    "$[*].routes[*].legs[*]..lng",
    200,
    "$[*].available_travel_modes",
    202,
    "$[*].routes[*]",
    203,
    "$[*].routes[*].legs[*]",
    204,
    "$[4000].routes[*].bounds"
);
// Generate BestBuy benchmarks
generate_benchmarks!(
    pison_bestbuy_short,
    "bestbuy",
    100,
    "$.products[5].videoChapters",
    101,
    "$.products[*].videoChapters",
    102,
    "$.products[*].videoChapters[1].chapter",
    103,
    "$.products[*].shipping[*]",
    104,
    "$.products[*].shipping[*].ground",
    105,
    "$.products[*].shipping[*].nextDay",
    106,
    "$.products[*].shipping[*].secondDay",
    107,
    "$.products[*].shipping[*].vendorDelivery",
    108,
    "$.products[*].shippingLevelsOfService[*]",
    109,
    "$.products[*].shippingLevelsOfService[*].serviceLevelId",
    110,
    "$.products[*].shippingLevelsOfService[*].serviceLevelName",
    111,
    "$.products[*].shippingLevelsOfService[*].unitShippingPrice",
    112,
    "$.products[*].categoryPath[2]"
);

// Generate Twitter benchmarks
generate_benchmarks!(
    pison_twitter_large,
    "twitter",
    200,
    "$[*].entities..symbols[*]",
    201,
    "$[*].entities..url",
    202,
    "$[*].entities.symbols[*]",
    203,
    "$[*].entities.symbols[1]",
    204,
    "$[*].entities.urls[*].display_url",
    205,
    "$[*].timestamp_ms"
);

// Generate Pokemon benchmarks
generate_benchmarks!(
    pokemon_short,
    "pokemon",
    200,
    "$.taildata",
    201,
    "$.taildata2",
    202,
    "$.cfg1[*].ID",
    203,
    "$.cfg1[*].Name",
    204,
    "$.cfg1[*].Height",
    205,
    "$.cfg1[*].Weight",
    206,
    "$.cfg1[*].Abilities[*]",
    207,
    "$.cfg1[*].Moves[*].moveName",
    208,
    "$.cfg1[*].Moves[*].levelLearnedAt",
    209,
    "$.cfg1[*].Moves[*].moveLearnCondition"
);

// Register all benchmarks
benchsets!(
    main_lut_benches, // Group name
    // Google
    google_map_bench_100,
    google_map_bench_101,
    google_map_bench_102,
    google_map_bench_103,
    google_map_bench_104,
    google_map_bench_108,
    google_map_bench_109,
    google_map_bench_110,
    google_map_bench_111,
    google_map_bench_112,
    google_map_bench_113,
    google_map_bench_114,
    google_map_bench_115,
    google_map_bench_116,
    google_map_bench_117,
    google_map_bench_118,
    google_map_bench_119,
    google_map_bench_200,
    google_map_bench_202,
    google_map_bench_203,
    google_map_bench_204,
    // Bestbuy
    // bestbuy_bench_100,
    // bestbuy_bench_101,
    // bestbuy_bench_102,
    // bestbuy_bench_103,
    // bestbuy_bench_104,
    // bestbuy_bench_105,
    // bestbuy_bench_106,
    // bestbuy_bench_107,
    // bestbuy_bench_108,
    // bestbuy_bench_109,
    // bestbuy_bench_110,
    // bestbuy_bench_111,
    // bestbuy_bench_112,
    // Twitter
    // twitter_bench_200,
    // twitter_bench_201,
    // twitter_bench_202,
    // twitter_bench_203,
    // twitter_bench_204,
    // twitter_bench_205,
    // Pokemon
    // pokemon_bench_200,
    // pokemon_bench_201,
    // pokemon_bench_202,
    // pokemon_bench_203,
    // pokemon_bench_204,
    // pokemon_bench_205,
    // pokemon_bench_206,
    // pokemon_bench_207,
    // pokemon_bench_208,
    // pokemon_bench_209
);
