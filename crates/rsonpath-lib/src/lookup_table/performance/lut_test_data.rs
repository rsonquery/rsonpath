pub const TWITTER_MINI: &str = ".a_lut_tests/test_data/MB_1/twitter_(767kB).json";

pub const TWITTER_SHORT: &str = ".a_lut_tests/test_data/MB_100/twitter_short_(80MB).json";
pub const BESTBUY_SHORT: &str = ".a_lut_tests/test_data/MB_100/bestbuy_short_(103MB).json";
pub const GOOGLE_SHORT: &str = ".a_lut_tests/test_data/MB_100/google_map_short_(107MB).json";
pub const WALMART_SHORT: &str = ".a_lut_tests/test_data/MB_100/walmart_short_(95MB).json";

pub const BESTBUY: &str = ".a_lut_tests/test_data/GB_1/bestbuy_large_record_(1GB).json";
pub const WALMART: &str = ".a_lut_tests/test_data/GB_1/walmart_large_record_(995MB).json";
pub const TWITTER: &str = ".a_lut_tests/test_data/GB_1/twitter_large_record_(843MB).json";
pub const GOOGLE: &str = ".a_lut_tests/test_data/GB_1/google_map_large_record_(1.1GB).json";

// google_map_large
pub const TEST_GOOGLE: (&str, &[(&str, &str)]) = (
    GOOGLE,
    &[
        // ("0", "$[*].available_travel_modes"),
        // ("1", "$[*].routes[*]"),
        // ("2", "$[*].routes[*].bounds"),
        // ("3", "$[*].routes[*].bounds.northeast"),
        // ("4", "$[*].routes[*].bounds.northeast.lat"),
        // ("5", "$[*].routes[*].bounds.northeast.lng"),
        // ("6", "$[*].routes[*].copyrights"),
        // ("8", "$[*].routes[*].legs[*].start_location.lat"),
        // ("11", "$[*].routes[*].legs[*].steps[1]"),
        // ("12", "$[*].routes[*].legs[*].steps[1].distance.text"),
        // ("13", "$[*].routes[*].legs[*].traffic_speed_entry"),
        // ("16", "$[*].routes[*].overview_polyline"),
        // ("17", "$[*].routes[*].overview_polyline.points"),
        // ("18", "$[*].routes[*].summary"),
        // ("19", "$[*].routes[*].warnings"),
        // ("20", "$[*].routes[*].waypoint_order"),
        // ("21", "$[*].routes[*]..points"),
        // ("22", "$[*].routes[*]..steps[*]"),
        // ("23", "$[*].routes..legs"),
        // ("24", "$[*]..bounds"),
        // ("25", "$[*]..bounds.northeast"),
        // ("26", "$[*]..bounds.northeast.lat"),
        // ("27", "$[*]..bounds.northeast.lng"),
        // ("28", "$[*]..copyrights"),
        // ("29", "$[*]..summary"),
        // ("30", "$[*]..warnings"),
        // ("31", "$[*]..waypoint_order"),
        // ("32", "$[1]"),
        // ("33", "$[10].routes[*].bounds"),
        // ("34", "$[100].routes[*].bounds"),
        // ("35", "$[1000].routes[*].bounds"),
        // ("36", "$[2000].routes[*].bounds"),
        // ("37", "$[3000].routes[*].bounds"),
        // ("38", "$[4000].routes[*].bounds"),

        ("100", "$[*].routes[*].legs[*]"),
        ("101", "$[*].routes[*].legs[*].steps[*]"),
        ("102", "$[*].routes[*].legs[*].steps[*].distance"),
        ("103", "$[*].routes[*].legs[*].steps[*].distance.text"),
        ("104", "$[*].routes[*].legs[*].steps[*].distance.value"),
        ("108", "$[*].routes[*].legs[*].steps[*].duration"),
        ("109", "$[*].routes[*].legs[*].steps[*].polyline"),
        ("110", "$[*].routes[*].legs[*].steps[*].polyline.points"),
        ("111", "$[*].routes[*].legs[*].steps[*].end_location"),
        ("112", "$[*].routes[*].legs[*].steps[*].html_instructions"),
        ("113", "$[*].routes[*].legs[*].steps[*].travel_mode"),
        ("114", "$[*].routes[*].legs[*].steps[*].start_location"),
        ("115", "$[*].routes[*].legs[*].steps[*].start_location.lat"),
        ("116", "$[*].routes[*].legs[*].steps[*].start_location.lng"),
        ("117", "$[*].routes[*].legs[*].steps[*].maneuver"),
        ("118", "$[*].routes[*].legs[*]..lat"),
        ("119", "$[*].routes[*].legs[*]..lng"),
    ],
);


pub const TEST_BESTBUY: (&str, &[(&str, &str)]) = (
    BESTBUY,
    &[
        ("0", "$.products[5].videoChapters"),
        ("1", "$.products[*].videoChapters"),
        ("2", "$.products[2].categoryPath[*].id"),
        ("3", "$.products[5].categoryPath[1].id"),
        ("4", "$.products[5].shippingLevelsOfService[1].serviceLevelName"),
        ("5", "$.products[10].shippingLevelsOfService[1].serviceLevelName"),
        ("6", "$.products[*].videoChapters[1].chapter"),
        ("7", "$.products[20].monthlyRecurringChargeGrandTotal"),
        ("8", "$.products[*].videoChapters[5].chapter"),
        ("9", "$.products[*].monthlyRecurringChargeGrandTotal"),
        ("10", "$.total"),
        ("11", "$.products[*].shipping[*]"),
        ("12", "$.products[*].shippingLevelsOfService[1].serviceLevelName"),
        ("13", "$.products[*].categoryPath[2]"),
        ("14", "$.products[*].shippingLevelsOfService[*].serviceLevelName"),
    ],
);

pub const TEST_TWITTER: (&str, &[(&str, &str)]) = (
    TWITTER_MINI,
    &[("0", "$.search_metadata.count"), ("1", "$.search_metadata.count")],
);

// const TEST_DATA: &[(&str, &str, &str, &str)] = &[
//     ("Twitter b0", TWITTER_JSON, "$.search_metadata.count"),

//     ("Twitter y0", TWITTER_JSON, "$.search_metadata.count"),
// ];
