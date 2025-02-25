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
        ("0", "$[*].available_travel_modes"),
        ("1", "$[*].routes[*].legs[*].steps[*]"),
        ("2", "$[*].routes[*].legs[*]"),
        ("3", "$[1]"),
        ("4", "$[200].routes[1].legs[5].steps[*].distance.text"),
        ("5", "$[*].routes[*].legs[*].steps[1]"),
        ("6", "$[500].routes[*].legs[5].steps[*].distance.text"),
        ("7", "$[1000].routes[1].legs[5].steps[*].distance.text"),
        ("8", "$[10000].routes[1].legs[5].steps[*].distance.text"),
        ("9", "$[10000].routes[*]"),
        ("10", "$[10000].routes[*].legs[*].steps[1]"),
        ("11", "$[10000].routes[*].legs[1].steps[*].distance.text"),
        ("12", "$[*].routes[*].legs[*].steps[*].distance.text"),
        ("13", "$[*].routes[*]"),
        ("14", "$[*].routes[*].warnings"),
        ("15", "$[*].routes[*].bounds[*]"),
        ("16", "$[*].routes[*].legs[*].steps[1].distance.text"),
        ("17", "$[*].routes[*].legs[1].steps[*].distance.text"),
        ("18", "$[*].routes[1].legs[*].steps[*].distance.text"),
        ("19", "$[1].routes[*].legs[*].steps[*].distance.text"),
        ("20", "$[1500].routes[1].legs[5].steps[*].distance.text"),
        ("21", "$[2000].routes[1].legs[5].steps[*].distance.text"),
        ("22", "$[2500].routes[1].legs[5].steps[*].distance.text"),
        ("23", "$[3000].routes[1].legs[5].steps[*].distance.text"),
        ("24", "$[3500].routes[1].legs[5].steps[*].distance.text"),
        ("25", "$[4000].routes[1].legs[5].steps[*].distance.text"),
        // ("26", "$[*].routes[*]..steps[*]"),
        // ("26", "$.*.routes.*.legs.*.steps.*.distance.text"),
        // ("27", "$[*].routes[*].bounds[*].northeast.*"),
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
