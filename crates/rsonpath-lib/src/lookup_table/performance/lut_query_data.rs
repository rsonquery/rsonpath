// All listed queries here have a result with count > 0.

// ##########
// kB_1
// ##########
pub const QUERY_ALPHABET: (&str, &[(&str, &str)]) = (
    ".a_lut_tests/test_data/kB_1/alphabet_(2kB).json",
    &[
        ("1", "$.alphabet[0]"),
        ("2", "$.alphabet[*]"),
        ("3", "$.alphabet[*].ID"),
        ("4", "$.alphabet[*].Letter"),
        ("5", "$.alphabet[*].Pronunciation"),
        ("6", "$.alphabet[10].Letter"),
        ("7", "$.alphabet[25].Pronunciation"),
    ],
);

pub const QUERY_BUGS: (&str, &[(&str, &str)]) = (
    ".a_lut_tests/test_data/kB_1/bugs.json",
    &[
        ("1", "$.a..b"),
        ("2", "$.a"),
        ("3", "$.a[0]"),
        ("4", "$.a[0].c"),
        ("5", "$.a[0].c.d"),
        ("6", "$.a[0].c.b"),
    ],
);

pub const QUERY_BUGS_2: (&str, &[(&str, &str)]) = (
    ".a_lut_tests/test_data/kB_1/bugs_2.json",
    &[
        ("1", "$.b[0]"),
        ("2", "$.a"),
        ("3", "$.b"),
        ("4", "$.b[0]"),
        ("5", "$.b[0].b"),
    ],
);

pub const QUERY_JOHN: (&str, &[(&str, &str)]) = (
    ".a_lut_tests/test_data/kB_1/john_119.json",
    &[
        ("1", "$.name"),
        ("2", "$.age"),
        ("3", "$.isMale"),
        ("4", "$.phones"),
        ("5", "$.phones[0]"),
        ("6", "$.phones[1]"),
        ("7", "$.address"),
    ],
);

pub const QUERY_JOHN_BIG: (&str, &[(&str, &str)]) = (
    ".a_lut_tests/test_data/kB_1/john_big.json",
    &[
        ("1", "$.person.firstName"),
        ("2", "$.person.lastName"),
        ("3", "$.person.phoneNumber[1].type"),
        ("4", "$.person.spouse.person.phoneNumber.*"),
        ("5", "$.person.spouse.person.phoneNumber[0]"),
        ("6", "$.person.spouse.person.phoneNumber[1]"),
        ("7", "$[1]"),
    ],
);

pub const QUERY_NUMBERS: (&str, &[(&str, &str)]) = (
    ".a_lut_tests/test_data/kB_1/numbers_117.json",
    &[
        ("1", "$.numbers[0]"),
        ("2", "$.numbers[*]"),
        ("3", "$.numbers[25]"),
        ("4", "$.numbers[10]"),
    ],
);

pub const QUERY_SMALL: (&str, &[(&str, &str)]) = (".a_lut_tests/test_data/kB_1/small_14.json", &[("1", "$.x")]);

// ##########
// MB_1
// ##########
pub const QUERY_CANADA: (&str, &[(&str, &str)]) = (
    ".a_lut_tests/test_data/MB_1/canada_(3MB).json",
    &[
        ("1", "$.features[0].properties.name"),
        ("2", "$.features[*].properties.name"),
        ("3", "$.features[0].geometry.coordinates[0]"),
        ("4", "$.features[0].geometry.coordinates[1]"),
        ("5", "$.features[0].geometry.coordinates[*]"),
        ("6", "$.features[*].geometry.coordinates[*]"),
        ("7", "$.features[0].geometry.type"),
        ("8", "$.features[*].geometry.type"),
    ],
);

pub const QUERY_OPENFOOD: (&str, &[(&str, &str)]) = (
    ".a_lut_tests/test_data/MB_1/openfood_(867kB).json",
    &[
        ("1", "$.products[0].brands"),
        ("2", "$.products[*].brands"),
        ("3", "$.products[0].categories"),
        ("4", "$.products[0].categories_hierarchy"),
        ("5", "$.products[0].ingredients_text"),
        ("6", "$.products[0].allergens"),
        ("7", "$.products[0].ecoscore_grade"),
        ("8", "$.products[*].ecoscore_grade"),
        ("9", "$.products[0].packaging"),
        ("10", "$.products[0].countries"),
        ("13", "$.products[0].ecoscore_score"),
        ("14", "$.products[0].categories_properties"),
        ("15", "$.products[0].ciqual_food_name_tags"),
        ("16", "$.products[0].ingredients_n"),
    ],
);

pub const QUERY_PEOPLE: (&str, &[(&str, &str)]) = (
    ".a_lut_tests/test_data/MB_1/people_(1.2MB).json",
    &[
        ("1", "$[0].name"),
        ("2", "$[*].name"),
        ("3", "$[0].email"),
        ("4", "$[*].email"),
        ("5", "$[0].address"),
        ("6", "$[*].address"),
        ("7", "$[0].phone"),
        ("8", "$[*].phone"),
        ("9", "$[0].website"),
        ("10", "$[*].website"),
    ],
);

pub const QUERY_PRETTY_PEOPLE: (&str, &[(&str, &str)]) = (
    ".a_lut_tests/test_data/MB_1/pretty_people_(1.8MB).json",
    &[
        ("1", "$.ctRoot[0].name"),
        ("2", "$.ctRoot[*].name"),
        ("3", "$.ctRoot[0].dob"),
        ("4", "$.ctRoot[*].dob"),
        ("5", "$.ctRoot[0].address.street"),
        ("6", "$.ctRoot[*].address.street"),
        ("7", "$.ctRoot[0].address.town"),
        ("8", "$.ctRoot[*].address.town"),
        ("9", "$.ctRoot[0].address.postode"),
        ("10", "$.ctRoot[*].address.postode"),
        ("11", "$.ctRoot[0].telephone"),
        ("12", "$.ctRoot[*].telephone"),
        ("13", "$.ctRoot[0].pets"),
        ("14", "$.ctRoot[*].pets"),
        ("15", "$.ctRoot[0].score"),
        ("16", "$.ctRoot[*].score"),
        ("17", "$.ctRoot[0].email"),
        ("18", "$.ctRoot[*].email"),
        ("19", "$.ctRoot[0].url"),
        ("20", "$.ctRoot[*].url"),
        ("21", "$.ctRoot[0].description"),
        ("22", "$.ctRoot[*].description"),
        ("23", "$.ctRoot[0].verified"),
        ("24", "$.ctRoot[*].verified"),
        ("25", "$.ctRoot[0].salary"),
        ("26", "$.ctRoot[*].salary"),
    ],
);

pub const QUERY_TWITTER_MINI: (&str, &[(&str, &str)]) = (
    ".a_lut_tests/test_data/MB_1/twitter_(767kB).json",
    &[
        ("1", "$.statuses[0].metadata.result_type"),
        ("2", "$.statuses[0].metadata.iso_language_code"),
        ("3", "$.statuses[0].created_at"),
        ("4", "$.statuses[*].id"),
        ("5", "$.statuses[*].text"),
        ("6", "$.statuses[0].source"),
        ("7", "$.statuses[0].user.id"),
        ("8", "$.statuses[*].user.name"),
        ("9", "$.statuses[0].user.screen_name"),
        ("10", "$.statuses[*].user.followers_count"),
        ("11", "$.statuses[*].user.friends_count"),
        ("12", "$.statuses[0].retweet_count"),
        ("13", "$.statuses[*].favorite_count"),
        ("14", "$.statuses[0].entities.user_mentions"),
        ("15", "$.statuses[*].lang"),
    ],
);

// ##########
// MB_15
// ##########
pub const QUERY_AST: (&str, &[(&str, &str)]) = (
    ".a_lut_tests/test_data/MB_15/ast_(26MB).json",
    &[
        ("1", "$.inner[0].name"),
        ("2", "$.inner[1].name"),
        ("3", "$.inner[*].type.qualType"),
        ("4", "$.inner[2].type.qualType"),
        ("5", "$.inner[0].inner[0].kind"),
        ("6", "$.inner[1].inner[0].kind"),
        ("7", "$..inner[0].kind"),
        ("10", "$.inner[0].isImplicit"),
        ("11", "$.inner[1].isImplicit"),
        ("12", "$..range.begin.offset"),
        ("14", "$..range.begin.col"),
        ("17", "$..isReferenced"),
    ],
);

pub const QUERY_DUMMY_10: (&str, &[(&str, &str)]) = (
    ".a_lut_tests/test_data/MB_15/dummy_(10MB).json",
    &[
        ("1", "$[0].name"),
        ("2", "$[*].name"),
        ("3", "$[0].email"),
        ("4", "$[*].email"),
        ("5", "$[0].address"),
        ("6", "$[*].address"),
        ("7", "$[0].phone"),
        ("8", "$[*].phone"),
        ("9", "$[0].website"),
        ("10", "$[*].website"),
        ("11", "$[1].name"),
        ("12", "$[1].email"),
        ("13", "$..address"),
        ("14", "$..phone"),
        ("15", "$..website"),
        ("16", "$..name"),
    ],
);

pub const QUERY_DUMMY_20: (&str, &[(&str, &str)]) = (
    ".a_lut_tests/test_data/MB_15/dummy_(20MB).json",
    &[
        ("1", "$[*].name"),
        ("2", "$[1].email"),
        ("3", "$[2].address"),
        ("4", "$[3].phone"),
        ("5", "$[4].website"),
        ("6", "$[5].name"),
        ("7", "$[6].email"),
        ("8", "$[7].address"),
        ("9", "$[8].phone"),
        ("10", "$[9].website"),
        ("11", "$[10].name"),
        ("12", "$[11].email"),
        ("13", "$[12].address"),
        ("14", "$[13].phone"),
        ("15", "$[14].website"),
        ("16", "$..name"),
        ("17", "$..email"),
        ("18", "$..address"),
        ("19", "$..phone"),
        ("20", "$..website"),
    ],
);

pub const QUERY_POKEMON_MINI: (&str, &[(&str, &str)]) = (
    ".a_lut_tests/test_data/MB_15/pokemon_(6MB).json",
    &[
        ("1", "$.cfgs[*].Name"),
        ("2", "$.cfgs[1].ID"),
        ("6", "$.cfgs[*].Color"),
        ("7", "$.cfgs[*].Habitat"),
        ("8", "$.cfgs[6].Shape"),
        ("10", "$.cfgs[*].BaseStats"),
        ("11", "$.cfgs[*].Abilities"),
        ("12", "$.cfgs[9].Moves"),
        ("13", "$.cfgs[*].Moves[*].moveName"),
        ("14", "$.cfgs[*].Moves[*].levelLearnedAt"),
        ("16", "$..Genus"),
        ("18", "$..Height"),
        ("19", "$..Weight"),
    ],
);

// ##########
// MB_100
// ##########
pub const QUERY_APP: (&str, &[(&str, &str)]) = (
    ".a_lut_tests/test_data/MB_100/app_(97MB).json",
    &[
        // ("1", "$.['All ASCII'].here"),
        ("2", "$..here"),
        ("3", "$..sqsk"),
        ("4", "$..xddt"),
    ],
);

pub const QUERY_BESTBUY_SHORT: (&str, &[(&str, &str)]) = (
    ".a_lut_tests/test_data/MB_100/bestbuy_short_(103MB).json",
    &[
        ("101", "$.products[*].videoChapters"),
        ("102", "$.products[*].videoChapters[1].chapter"),
        ("103", "$.products[*].shipping[*]"),
        ("104", "$.products[*].shipping[*].ground"),
        ("105", "$.products[*].shipping[*].nextDay"),
        ("106", "$.products[*].shipping[*].secondDay"),
        ("107", "$.products[*].shipping[*].vendorDelivery"),
        ("108", "$.products[*].shippingLevelsOfService[*]"),
        ("109", "$.products[*].shippingLevelsOfService[*].serviceLevelId"),
        ("110", "$.products[*].shippingLevelsOfService[*].serviceLevelName"),
        ("111", "$.products[*].shippingLevelsOfService[*].unitShippingPrice"),
        ("112", "$.products[*].categoryPath[2]"),
        ("113", "$.products[*].categoryPath[*].id"),
        ("114", "$.products[*].categoryPath[*].name"),
        ("115", "$.products[*].quantityLimit"),
        ("117", "$.products[*].frequentlyPurchasedWith[*]"),
        ("118", "$.products[*].includedItemList[*]"),
        ("121", "$.products[*].homeDelivery"),
        ("123", "$.products[*].freeShipping"),
        ("124", "$.products[*].additionalFeatures[*]"),
        ("125", "$.products[*].additionalFeatures[*].feature"),
        ("126", "$.products[*].dollarSavings"),
        ("127", "$.products[*].lengthInMinutes"),
        ("128", "$.products[*].screenFormat"),
        ("200", "$..freeShipping"),
        ("201", "$..additionalFeatures[*]"),
        ("202", "$..additionalFeatures[*].feature"),
        ("203", "$..dollarSavings"),
        ("204", "$..lengthInMinutes"),
        ("205", "$..screenFormat"),
        ("300", "$.products[4].categoryPath[2]"),
        ("301", "$.products[4].categoryPath[*].id"),
        ("302", "$.products[4].categoryPath[*].name"),
        ("303", "$.products[4].quantityLimit"),
    ],
);

pub const QUERY_CROSSREF0: (&str, &[(&str, &str)]) = (
    ".a_lut_tests/test_data/MB_100/crossref0_(320MB).json",
    &[
        ("1", "$.items[*].URL"),
        ("2", "$.items[*].resource.primary.URL"),
        ("3", "$.items[*].member"),
        ("4", "$.items[*].author[*].given"),
        ("5", "$.items[*].author[*].family"),
        ("6", "$.items[*].author[*].sequence"),
        ("7", "$.items[*].score"),
        ("8", "$.items[0].prefix"),
        ("9", "$.items[0].DOI"),
        ("10", "$.items[1].URL"),
        ("11", "$.items[1].author[*].given"),
        ("12", "$.items[2].URL"),
        ("13", "$.items[2].resource.primary.URL"),
        ("14", "$..URL"),
        ("15", "$..author[*].given"),
        ("16", "$..author[*].family"),
        ("17", "$..author[*].affiliation[0].name"),
        ("18", "$..title[*]"),
    ],
);

pub const QUERY_GOOGLE_SHORT: (&str, &[(&str, &str)]) = (
    ".a_lut_tests/test_data/MB_100/google_map_short_(107MB).json",
    &[
        ("0", "$[*]..bounds"),
        ("1", "$[*]..bounds.northeast"),
        ("2", "$[*]..bounds.northeast.lat"),
        ("3", "$[*]..bounds.northeast.lng"),
        ("4", "$[*]..copyrights"),
        ("5", "$[*]..summary"),
        ("6", "$[*]..warnings"),
        ("7", "$[*]..waypoint_order"),
        ("8", "$[*].routes[*]"),
        ("9", "$[*].routes[*]..legs"),
        ("10", "$[*].routes[*]..points"),
        ("11", "$[*].routes[*]..steps[*]"),
        ("12", "$[*].routes[*].bounds"),
        ("13", "$[*].routes[*].bounds.northeast"),
        ("14", "$[*].routes[*].bounds.northeast.lat"),
        ("15", "$[*].routes[*].bounds.northeast.lng"),
        ("16", "$[*].routes[*].legs[*].start_location.lat"),
        ("17", "$[*].routes[*].legs[*].steps[1]"),
        ("18", "$[*].routes[*].legs[*].steps[1].distance.text"),
        ("19", "$[*].routes[*].legs[*].traffic_speed_entry"),
        ("20", "$[*].routes[*].overview_polyline"),
        ("21", "$[*].routes[*].overview_polyline.points"),
        ("22", "$[*].routes[*].summary"),
        ("23", "$[*].routes[*].warnings"),
        ("24", "$[*].routes[*].waypoint_order"),
        ("25", "$[1]"),
        ("26", "$[10].routes[*].bounds"),
        ("27", "$[100].routes[*].bounds"),
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
        ("200", "$[*].available_travel_modes"),
        ("202", "$[*].routes[*]"),
        ("203", "$[*].routes[*].legs[*]"),
    ],
);

pub const QUERY_POKEMON: (&str, &[(&str, &str)]) = (
    ".a_lut_tests/test_data/MB_100/pokemon_(173MB).json",
    &[
        ("1", "$.cfg1[0].Name"),
        ("5", "$.cfg1[*].Abilities[0]"),
        ("6", "$.cfg1[*].Moves[1].moveName"),
        ("7", "$.cfg1[*].Name"),
        ("11", "$.cfg1[*].Abilities[1]"),
        ("12", "$..Moves[*].moveName"),
        ("13", "$..Name"),
        ("17", "$.cfg6[*].Abilities[*]"),
        ("18", "$.cfg7[*].Moves[*].moveName"),
    ],
);

pub const QUERY_TWITTER_SHORT: (&str, &[(&str, &str)]) = (
    ".a_lut_tests/test_data/MB_100/twitter_short_(80MB).json",
    &[
        ("1", "$[*].geo"),
        ("2", "$[*].id"),
        ("3", "$[*].source"),
        ("4", "$[*].timestamp_ms"),
        ("5", "$[*].user.created_at"),
        ("6", "$[*].retweeted_status.id"),
        ("7", "$[*].retweeted_status.filter_level"),
        ("8", "$[1].retweeted_status.user.following"),
        ("9", "$[0].retweeted_status.user.name"),
        ("10", "$[1].retweeted_status[*]"),
        ("14", "$[1].retweeted_status[*]..id"),
        ("16", "$[1].retweeted_status[*].user.lang"),
        ("17", "$..entities.hashtags[*]"),
        ("18", "$..entities.symbols[*]"),
        ("19", "$..entities.symbols[1]"),
        ("20", "$..urls[*].display_url"),
        ("21", "$[*].entities..symbols[*]"),
        ("22", "$[*].entities..url"),
        ("23", "$[*]..id"),
        ("24", "$[*].entities..symbols[*]"),
        ("25", "$[*].entities..url"),
        ("26", "$[*].entities.symbols[*]"),
        ("27", "$[*].entities.symbols[1]"),
        ("28", "$[*].entities.urls[*].display_url"),
        ("29", "$[*].timestamp_ms"),
    ],
);

pub const QUERY_WALMART_SHORT: (&str, &[(&str, &str)]) = (
    ".a_lut_tests/test_data/MB_100/walmart_short_(95MB).json",
    &[
        ("1", "$.items[*].itemId"),
        ("2", "$.items[*].name"),
        ("3", "$.items[*].msrp"),
        ("4", "$.items[*].salePrice"),
        ("5", "$.items[*].upc"),
        ("6", "$.items[*].categoryPath"),
        ("7", "$.items[*].shortDescription"),
        ("8", "$.items[*].longDescription"),
        ("9", "$.items[0].thumbnailImage"),
        ("10", "$.items[0].productTrackingUrl"),
        ("11", "$.items[0].freeShipToStore"),
        ("12", "$.items[0].stock"),
        ("13", "$..addToCartUrl"),
        ("14", "$..isbn"),
        ("15", "$..availableOnline"),
        ("16", "$..freeShippingOver50Dollars"),
        ("17", "$..categoryNode"),
        ("18", "$..marketplace"),
        ("19", "$.category"),
        ("20", "$.format"),
    ],
);

// ##########
// GB_1
// ##########
pub const QUERY_BESTBUY: (&str, &[(&str, &str)]) = (
    ".a_lut_tests/test_data/GB_1/bestbuy_large_record_(1GB).json",
    &[
        ("1", "$.products[4].categoryPath[2]"),                             // 0.999999
        ("2", "$.products[*].categoryPath[2]"),                             // 0.827555
        ("3", "$.products[*].quantityLimit"),                               // 0.803025
        ("4", "$.products[*].frequentlyPurchasedWith[*]"),                  // 0.775238
        ("5", "$.products[*].includedItemList[*]"),                         // 0.765115
        ("6", "$.products[*].homeDelivery"),                                // 0.598374
        ("7", "$.products[*].freeShipping"),                                // 0.510142
        ("8", "$.products[*].shipping[*]"),                                 // 0.492032
        ("9", "$.products[*].shippingLevelsOfService[*].serviceLevelName"), // 0.306224
        ("10", "$.products[*].shippingLevelsOfService[*]"),                 // 0.295264
        ("11", "$.products[*].dollarSavings"),                              // 0.162506
        ("12", "$.products[*].lengthInMinutes"),                            // 0.127193
        ("13", "$.products[*].screenFormat"),                               // 0.101771
        ("14", "$.products[*].additionalFeatures[*]"),                      // 0.097107
        ("15", "$.products[*].videoChapters"),                              // 0.088070
        ("16", "$..freeShipping"),                                          // 0.000000
    ],
);

pub const QUERY_CROSSREF1: (&str, &[(&str, &str)]) = (
    ".a_lut_tests/test_data/GB_1/crossref1_(551MB).json",
    &[
        ("1", "$.items[2].resource.primary.URL"), // 1.000000
        ("2", "$.items[*].URL"),                  // 0.963846
        ("3", "$.items[*].member"),               // 0.950990
        ("4", "$.items[*].score"),                // 0.948115
        ("5", "$.items[*].resource.primary.URL"), // 0.937938
        ("6", "$.items[*].issued"),               // 0.922995
        ("7", "$.items[*].author[*].given"),      // 0.881082
        ("8", "$.items[*].author[*].family"),     // 0.865040
        ("9", "$.items[*].author[*].sequence"),   // 0.852936
        ("10", "$.items[*].author[*]"),           // 0.849213
        ("11", "$.items[*].institution[*].name"), // 0.808941
        ("12", "$.items[*].link[*]..URL"),        // 0.804825
        ("13", "$.items[*].assertion[*]..name"),  // 0.796434
        ("14", "$.items[*].reference[*].key"),    // 0.732995
        ("15", "$.items[*].reference[*].author"), // 0.545019
        ("16", "$.items[*].reference[*].DOI"),    // 0.490577
        ("17", "$.items[*].reference[*].year"),   // 0.374631
        ("18", "$.items[*].reference[*].issue"),  // 0.338667
        ("19", "$.items[*].reference[*].volume"), // 0.313539
        ("20", "$.items[*].reference[*]"),        // 0.256538
        ("21", "$..URL"),                         // 0.000000
    ],
);

pub const QUERY_CROSSREF2: (&str, &[(&str, &str)]) = (
    ".a_lut_tests/test_data/GB_1/crossref2_(1.1GB).json",
    &[
        ("1", "$.items[2].resource.primary.URL"), // 1.000000
        ("2", "$.items[*].URL"),                  // 0.963846
        ("3", "$.items[*].member"),               // 0.950990
        ("4", "$.items[*].score"),                // 0.948115
        ("5", "$.items[*].resource.primary.URL"), // 0.937938
        ("6", "$.items[*].issued"),               // 0.922995
        ("7", "$.items[*].author[*].given"),      // 0.881082
        ("8", "$.items[*].author[*].family"),     // 0.865040
        ("9", "$.items[*].author[*].sequence"),   // 0.852936
        ("10", "$.items[*].author[*]"),           // 0.849213
        ("11", "$.items[*].institution[*].name"), // 0.808941
        ("12", "$.items[*].link[*]..URL"),        // 0.804825
        ("13", "$.items[*].assertion[*]..name"),  // 0.796434
        ("14", "$.items[*].reference[*].key"),    // 0.732995
        ("15", "$.items[*].reference[*].author"), // 0.545019
        ("16", "$.items[*].reference[*].DOI"),    // 0.490577
        ("17", "$.items[*].reference[*].year"),   // 0.374631
        ("18", "$.items[*].reference[*].issue"),  // 0.338667
        ("19", "$.items[*].reference[*].volume"), // 0.313539
        ("20", "$.items[*].reference[*]"),        // 0.256538
        ("21", "$..URL"),                         // 0.000000
    ],
);

pub const QUERY_CROSSREF4: (&str, &[(&str, &str)]) = (
    ".a_lut_tests/test_data/GB_1/crossref4_(2.1GB).json",
    &[
        ("1", "$.items[2].resource.primary.URL"), // 1.000000
        ("2", "$.items[*].URL"),                  // 0.963846
        ("3", "$.items[*].member"),               // 0.950990
        ("4", "$.items[*].score"),                // 0.948115
        ("5", "$.items[*].resource.primary.URL"), // 0.937938
        ("6", "$.items[*].issued"),               // 0.922995
        ("7", "$.items[*].author[*].given"),      // 0.881082
        ("8", "$.items[*].author[*].family"),     // 0.865040
        ("9", "$.items[*].author[*].sequence"),   // 0.852936
        ("10", "$.items[*].author[*]"),           // 0.849213
        ("11", "$.items[*].institution[*].name"), // 0.808941
        ("12", "$.items[*].link[*]..URL"),        // 0.804825
        ("13", "$.items[*].assertion[*]..name"),  // 0.796434
        ("14", "$.items[*].reference[*].key"),    // 0.732995
        ("15", "$.items[*].reference[*].author"), // 0.545019
        ("16", "$.items[*].reference[*].DOI"),    // 0.490577
        ("17", "$.items[*].reference[*].year"),   // 0.374631
        ("18", "$.items[*].reference[*].issue"),  // 0.338667
        ("19", "$.items[*].reference[*].volume"), // 0.313539
        ("20", "$.items[*].reference[*]"),        // 0.256538
        ("21", "$..URL"),                         // 0.000000
    ],
);

pub const QUERY_GOOGLE: (&str, &[(&str, &str)]) = (
    ".a_lut_tests/test_data/GB_1/google_map_large_record_(1.1GB).json",
    &[
        ("0", "$[4000].routes[*].bounds"), // 0.999989
                                           // ("1", "$[*].routes[*].legs[*].steps[*].html_instructions"),     // 0.965067
                                           // ("2", "$[*].routes[*].legs[*].steps[*].distance.text"),         // 0.821380
                                           // ("3", "$[*].routes[*].legs[*].steps[*].distance"),              // 0.819461
                                           // ("4", "$[*].routes[*].legs[*].steps[*].maneuver"),              // 0.745787
                                           // ("5", "$[*].routes[*].legs[*].steps[*].start_location"),        // 0.673776
                                           // ("6", "$[*].routes[*].legs[*].steps[*].start_location.lat"),    // 0.665473
                                           // ("7", "$[1:2000].routes[*].legs[*].steps[*].polyline.points"),  // 0.654228
                                           // ("8", "$[*].routes[*].legs[*].steps[*].duration"),              // 0.594071
                                           // ("9", "$[*].routes[*].legs[*].steps[*].end_location"),          // 0.532257
                                           // ("10", "$[1:3000].routes[*].legs[*].steps[*].polyline.points"), // 0.467309
                                           // ("11", "$[*].routes[*].legs[*].steps[*]"),                      // 0.432387
                                           // ("12", "$[1:4000].routes[*].legs[*].steps[*].polyline.points"), // 0.269475
                                           // ("13", "$[*].routes[*].legs[*].steps[*].polyline"),             // 0.225161
                                           // ("14", "$[*].routes[*].legs[*].steps[*].polyline.points"),      // 0.173328
                                           // ("15", "$[*].routes[*].legs[*]..lat"),                          // 0.006689
                                           // ("16", "$[*]..bounds"),                                         // 0.000000
    ],
);

pub const QUERY_NSPL: (&str, &[(&str, &str)]) = (
    ".a_lut_tests/test_data/GB_1/nspl_large_record_(1.2GB).json",
    &[
        ("0", "$.meta.view.id"),        // 1.000000
        ("1", "$.data[0:100000][*]"),   // 0.938919
        ("2", "$.data[0:200000][*]"),   // 0.882185
        ("3", "$.data[0:300000][*]"),   // 0.825455
        ("4", "$.data[0:400000][*]"),   // 0.768723
        ("5", "$.data[0:500000][*]"),   // 0.711985
        ("6", "$.data[0:600000][*]"),   // 0.655252
        ("7", "$.data[0:700000][*]"),   // 0.598512
        ("8", "$.data[0:800000][*]"),   // 0.541783
        ("9", "$.data[0:900000][*]"),   // 0.485039
        ("10", "$.data[0:1000000][*]"), // 0.428310
        ("11", "$.data[0:1100000][*]"), // 0.371576
        ("12", "$.data[0:1200000][*]"), // 0.314850
        ("13", "$.data[0:1300000][*]"), // 0.258127
        ("14", "$.data[0:1400000][*]"), // 0.201401
        ("15", "$.data[0:1500000][*]"), // 0.144680
        ("16", "$.data[0:1600000][*]"), // 0.087951
        ("17", "$.data[*]"),            // 0.093469
        ("18", "$..id"),                // 0.000000
    ],
);

pub const QUERY_TWITTER: (&str, &[(&str, &str)]) = (
    ".a_lut_tests/test_data/GB_1/twitter_large_record_(843MB).json",
    &[
        // Picked
        // ("1", "$[*].user..url.*"),                        // 0.000000
        // ("2", "$[1:150000]..user"),                       // 0.035640
        // ("3", "$[1:140000]..user"),                       // 0.101083
        // ("4", "$[1:130000]..user"),                       // 0.165078
        // ("5", "$[1:120000]..user"),                       // 0.228623
        // ("6", "$[1:110000]..user"),                       // 0.292380
        // ("7", "$[1:100000]..user"),                       // 0.356660
        // ("8", "$[1:90000]..user"),                        // 0.417997
        // ("9", "$[1:80000]..user"),                        // 0.481244
        // ("10", "$[1:70000]..user"),                       // 0.545231
        // ("11", "$[1:60000]..user"),                       // 0.572430
        // ("12", "$[1:50000]..user"),                       // 0.609424
        // ("13", "$[1:40000]..user"),                       // 0.661039
        // ("14", "$[1:30000]..user"),                       // 0.675647
        // ("15", "$[1:20000]..user"),                       // 0.708100
        // ("16", "$[1:10000]..user"),                       // 0.741561
        // ("17", "$[*].user.profile_sidebar_border_color"), // 0.751322
        // ("18", "$[*].user.profile_image_url_https"),      // 0.780070
        // ("19", "$[*].retweeted_status.filter_level"),     // 0.805593
        // ("20", "$[*].retweeted_status.user.name"),        // 0.807400
        // ("21", "$[*].user.created_at"),                   // 0.834220
        // ("22", "$[*].retweeted_status.id"),               // 0.870497
        // ("23", "$[*].user.screen_name"),                  // 0.872362
        // ("24", "$[*].source"),                            // 0.889281
        // ("25", "$[*].id"),                                // 0.901908
        // ("26", "$[*].geo"),                               // 0.936339
        // ("27", "$[*].retweeted_status[*]"),               // 0.954362
        // ("28", "$[*]..id"),                               // 0.989802
        // Queries where LUT is faster than ITE
        ("200", "$[*].entities..symbols[*]"),
        ("201", "$[*].entities..url"),
        ("202", "$[*].entities.symbols[*]"),
        ("203", "$[*].entities.symbols[1]"),
        ("204", "$[*].entities.urls[*].display_url"),
        ("205", "$[*].timestamp_ms"),
    ],
);

pub const QUERY_WALMART: (&str, &[(&str, &str)]) = (
    ".a_lut_tests/test_data/GB_1/walmart_large_record_(995MB).json",
    &[
        ("0", "$.category"),                     // 1.000000
        ("1", "$.items[*].itemId"),              // 0.996712
        ("2", "$.items[*].name"),                // 0.985764
        ("3", "$.items[*].upc"),                 // 0.955816
        ("4", "$.items[*].categoryPath"),        // 0.949076
        ("5", "$.items[*].salePrice"),           // 0.938876
        ("6", "$.items[*].longDescription"),     // 0.878879
        ("7", "$.items[*].msrp"),                // 0.680515
        ("8", "$.items[*].thumbnailImage"),      // 0.564013
        ("9", "$.items[*].mediumImage"),         // 0.520963
        ("10", "$.items[*].largeImage"),         // 0.478252
        ("11", "$.items[*].productTrackingUrl"), // 0.433276
        ("12", "$.items[*].productUrl"),         // 0.301231
        ("13", "$.items[*].addToCartUrl"),       // 0.187381
        ("14", "$.items[*].isbn"),               // 0.019278
        ("15", "$..category"),                   // 0.000000
    ],
);

pub const QUERY_WIKI: (&str, &[(&str, &str)]) = (
    ".a_lut_tests/test_data/GB_1/wiki_large_record_(1.1GB).json",
    &[
        ("1", "$[0].id"),                // 1.000000
        ("2", "$[1:1000]..fr"),          // 0.981716
        ("3", "$[*].labels.*.language"), // 0.911067
        ("4", "$[*].labels.*.value"),    // 0.861710
        ("5", "$[1:10000]..fr"),         // 0.882321
        ("6", "$[*].*.fr.*"),            // 0.813313
        ("7", "$[1:20000]..fr"),         // 0.781792
        ("8", "$[1:30000]..fr"),         // 0.702920
        ("9", "$[1:40000]..fr"),         // 0.624433
        ("10", "$[1:50000]..fr"),        // 0.541105
        ("11", "$[1:60000]..fr"),        // 0.472672
        ("12", "$[1:70000]..fr"),        // 0.408434
        ("13", "$[1:80000]..fr"),        // 0.338641
        ("14", "$[1:90000]..fr"),        // 0.279816
        ("15", "$[1:100000]..fr"),       // 0.226452
        ("16", "$[1:110000]..fr"),       // 0.161277
        ("17", "$[1:120000]..fr"),       // 0.097605
        ("18", "$[1:130000]..fr"),       // 0.039856
        ("19", "$[1:140000]..fr"),       // 0.000021
        ("20", "$..language"),           // 0.000000
    ],
);

// GB_25
pub const QUERY_NESTED_COL: (&str, &[(&str, &str)]) = (
    ".a_lut_tests/test_data/GB_25/nested_col_(27.7GB).json",
    &[
        ("1", "$[*].c_custkey"),
        ("2", "$[*].c_name"),
        ("9", "$[*].c_orders[*].o_orderkey"),
        ("10", "$[*].c_orders[*].o_orderstatus"),
        ("11", "$[*].c_orders[*].o_totalprice"),
        ("12", "$[*].c_orders[*].o_orderdate"),
        ("13", "$[*].c_orders[*].o_orderpriority"),
        ("14", "$[*].c_orders[*].o_clerk"),
        ("15", "$[*].c_orders[*].o_shippriority"),
        ("16", "$[*].c_orders[*].o_comment"),
        ("17", "$[*].c_orders[*].o_lineitems[*].l_partkey"),
        ("18", "$[*].c_orders[*].o_lineitems[*].l_suppkey"),
        ("19", "$[*].c_orders[*].o_lineitems[*].l_linenumber"),
        ("101", "$[0].c_custkey"),
        ("102", "$[0].c_name"),
        ("103", "$[0].c_address"),
        ("104", "$[0].c_nationkey"),
        ("200", "$..l_receiptdate"),
        ("201", "$..l_shipinstruct"),
        ("202", "$..l_shipmode"),
        ("203", "$..l_comment"),
    ],
);
