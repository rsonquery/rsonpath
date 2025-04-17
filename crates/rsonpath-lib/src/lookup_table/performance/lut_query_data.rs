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

// MB_100
pub const APP: &str = ".a_lut_tests/test_data/MB_100/app_(97MB).json";
pub const BESTBUY_SHORT: &str = ".a_lut_tests/test_data/MB_100/bestbuy_short_(103MB).json";
pub const CROSSREF0: &str = ".a_lut_tests/test_data/MB_100/crossref0_(320MB).json";
pub const GOOGLE_SHORT: &str = ".a_lut_tests/test_data/MB_100/google_map_short_(107MB).json";
pub const POKEMON_SHORT: &str = ".a_lut_tests/test_data/MB_100/pokemon_(173MB).json";
pub const TWITTER_SHORT: &str = ".a_lut_tests/test_data/MB_100/twitter_short_(80MB).json";
pub const WALMART_SHORT: &str = ".a_lut_tests/test_data/MB_100/walmart_short_(95MB).json";

// GB_1
pub const BESTBUY: &str = ".a_lut_tests/test_data/GB_1/bestbuy_large_record_(1GB).json";
pub const CROSSREF1: &str = ".a_lut_tests/test_data/GB_1/crossref1_(551MB).json";
pub const CROSSREF2: &str = ".a_lut_tests/test_data/GB_1/crossref2_(1.1GB).json";
pub const CROSSREF4: &str = ".a_lut_tests/test_data/GB_1/crossref4_(2.1GB).json";
pub const GOOGLE: &str = ".a_lut_tests/test_data/GB_1/google_map_large_record_(1.1GB).json";
pub const NSPL: &str = ".a_lut_tests/test_data/GB_1/nspl_large_record_(1.2GB).json";
pub const TWITTER: &str = ".a_lut_tests/test_data/GB_1/twitter_large_record_(843MB).json";
pub const WALMART: &str = ".a_lut_tests/test_data/GB_1/walmart_large_record_(995MB).json";
pub const WIKI: &str = ".a_lut_tests/test_data/GB_1/wiki_large_record_(1.1GB).json";

// GB_25
pub const NESTED_COL: &str = ".a_lut_tests/test_data/GB_25/nested_col_(27.7GB).json";

pub const QUERY_BESTBUY: (&str, &[(&str, &str)]) = (
    BESTBUY,
    &[
        ("100", "$.products[5].videoChapters"),
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
        ("116", "$.products[*].earlyTerminationFees[*]"),
        ("117", "$.products[*].frequentlyPurchasedWith[*]"),
        ("118", "$.products[*].includedItemList[*]"),
        ("119", "$.products[*].accessories[*]"),
        ("120", "$.products[*].planFeatures[*]"),
        ("121", "$.products[*].homeDelivery"),
        ("122", "$.products[*].carrierPlans[*]"),
        ("123", "$.products[*].freeShipping"),
        ("124", "$.products[*].additionalFeatures[*]"),
        ("125", "$.products[*].additionalFeatures[*].feature"),
        ("126", "$.products[*].dollarSavings"),
        ("127", "$.products[*].lengthInMinutes"),
        ("128", "$.products[*].screenFormat"),
    ],
);

pub const QUERY_CROSSREF2: (&str, &[(&str, &str)]) = (CROSSREF2, &[("100", "$..")]);

pub const QUERY_CROSSREF4: (&str, &[(&str, &str)]) = (CROSSREF4, &[("100", "$..")]);

pub const QUERY_GOOGLE: (&str, &[(&str, &str)]) = (
    GOOGLE,
    &[
        // Random queries
        // ("0", "$[*]..bounds"),
        // ("1", "$[*]..bounds.northeast"),
        // ("2", "$[*]..bounds.northeast.lat"),
        // ("3", "$[*]..bounds.northeast.lng"),
        // ("4", "$[*]..copyrights"),
        // ("5", "$[*]..summary"),
        // ("6", "$[*]..warnings"),
        // ("7", "$[*]..waypoint_order"),
        // ("8", "$[*].routes[*]"),
        // ("9", "$[*].routes[*]..legs"),
        // ("10", "$[*].routes[*]..points"),
        // ("11", "$[*].routes[*]..steps[*]"),
        // ("12", "$[*].routes[*].bounds"),
        // ("13", "$[*].routes[*].bounds.northeast"),
        // ("14", "$[*].routes[*].bounds.northeast.lat"),
        // ("15", "$[*].routes[*].bounds.northeast.lng"),
        // ("16", "$[*].routes[*].legs[*].start_location.lat"),
        // ("17", "$[*].routes[*].legs[*].steps[1]"),
        // ("18", "$[*].routes[*].legs[*].steps[1].distance.text"),
        // ("19", "$[*].routes[*].legs[*].traffic_speed_entry"),
        // ("20", "$[*].routes[*].overview_polyline"),
        // ("21", "$[*].routes[*].overview_polyline.points"),
        // ("22", "$[*].routes[*].summary"),
        // ("23", "$[*].routes[*].warnings"),
        // ("24", "$[*].routes[*].waypoint_order"),
        // ("25", "$[1]"),
        // ("26", "$[10].routes[*].bounds"),
        // ("27", "$[100].routes[*].bounds"),
        // ("28", "$[1000].routes[*].bounds"),
        // ("29", "$[2000].routes[*].bounds"),
        // ("30", "$[3000].routes[*].bounds"),
        // // More queries
        ("100", "$[*].routes[*].legs[*]"),
        // ("101", "$[*].routes[*].legs[*].steps[*]"),
        // ("102", "$[*].routes[*].legs[*].steps[*].distance"),
        // ("103", "$[*].routes[*].legs[*].steps[*].distance.text"),
        // ("104", "$[*].routes[*].legs[*].steps[*].distance.value"),
        // ("108", "$[*].routes[*].legs[*].steps[*].duration"),
        // ("109", "$[*].routes[*].legs[*].steps[*].polyline"),
        // ("110", "$[*].routes[*].legs[*].steps[*].polyline.points"),
        // ("111", "$[*].routes[*].legs[*].steps[*].end_location"),
        // ("112", "$[*].routes[*].legs[*].steps[*].html_instructions"),
        // ("113", "$[*].routes[*].legs[*].steps[*].travel_mode"),
        // ("114", "$[*].routes[*].legs[*].steps[*].start_location"),
        // ("115", "$[*].routes[*].legs[*].steps[*].start_location.lat"),
        // ("116", "$[*].routes[*].legs[*].steps[*].start_location.lng"),
        // ("117", "$[*].routes[*].legs[*].steps[*].maneuver"),
        // ("118", "$[*].routes[*].legs[*]..lat"),
        // ("119", "$[*].routes[*].legs[*]..lng"),
        // ("200", "$[*].available_travel_modes"),
        // ("202", "$[*].routes[*]"),
        // ("203", "$[*].routes[*].legs[*]"),
        // ("204", "$[4000].routes[*].bounds"),
    ],
);

pub const QUERY_NSPL: (&str, &[(&str, &str)]) = (NSPL, &[("100", "$..")]);

pub const QUERY_TWITTER: (&str, &[(&str, &str)]) = (
    TWITTER,
    &[
        // Random Queries
        ("100", "$[*].geo"),
        ("101", "$[*].id"),
        ("102", "$[*].source"),
        ("103", "$[*].timestamp_ms"),
        ("104", "$[*].user.created_at"),
        ("105", "$[*].user.followers_count"),
        ("106", "$[*].user.lang"),
        ("107", "$[*].user.listed_count"),
        ("108", "$[*].user.profile_image_url_https"),
        ("109", "$[*].user.profile_sidebar_border_color"),
        ("110", "$[*].user.screen_name"),
        ("111", "$[*].user.verified"),
        ("112", "$[*].retweeted_status.id"),
        ("113", "$[*].retweeted_status.filter_level"),
        ("114", "$[*].retweeted_status.user.following"),
        ("115", "$[*].retweeted_status.user.name"),
        ("116", "$[*].retweeted_status[*]"),
        ("117", "$[*].retweeted_status[*]..entities..url"),
        ("118", "$[*].retweeted_status[*]..entities.symbols[*]"),
        ("119", "$[*].retweeted_status[*]..entities.user_mentions[*]"),
        ("120", "$[*].retweeted_status[*]..id"),
        ("121", "$[*].retweeted_status[*]..in_reply_to_screen_name"),
        ("122", "$[*].retweeted_status[*].user.lang"),
        ("123", "$[*].entities.hashtags[*]"),
        ("124", "$[*].entities.symbols[*]"),
        ("125", "$[*].entities.symbols[1]"),
        ("126", "$[*].entities.urls[*].display_url"),
        ("127", "$[*].entities..symbols[*]"),
        ("128", "$[*].entities..url"),
        ("129", "$[*]..id"),
        // Queries where LUT is faster than ITE
        ("200", "$[*].entities..symbols[*]"),
        ("201", "$[*].entities..url"),
        ("202", "$[*].entities.symbols[*]"),
        ("203", "$[*].entities.symbols[1]"),
        ("204", "$[*].entities.urls[*].display_url"),
        ("205", "$[*].timestamp_ms"),
    ],
);

pub const QUERY_WALMART: (&str, &[(&str, &str)]) = (WALMART, &[("100", "$..")]);

pub const QUERY_WIKI: (&str, &[(&str, &str)]) = (WIKI, &[("100", "$..")]);

pub const QUERY_POKEMON_SHORT: (&str, &[(&str, &str)]) = (
    POKEMON_SHORT,
    &[
        // Group 1: Tail data
        ("200", "$.taildata"),
        ("201", "$.taildata2"),
        // Group 2: cfg1 fields
        ("202", "$.cfg1[*].ID"),
        ("203", "$.cfg1[*].Name"),
        ("204", "$.cfg1[*].Height"),
        ("205", "$.cfg1[*].Weight"),
        ("206", "$.cfg1[*].Abilities[*]"),
        ("207", "$.cfg1[*].Moves[*].moveName"),
        ("208", "$.cfg1[*].Moves[*].levelLearnedAt"),
        ("209", "$.cfg1[*].Moves[*].moveLearnCondition"),
        // Group 3: Recursive search
        ("210", "$..ID"),
        ("211", "$..Name"),
        ("212", "$..Height"),
        ("213", "$..Weight"),
        ("214", "$..Abilities[*]"),
        ("215", "$..Moves[*].moveName"),
        ("216", "$..Moves[*].levelLearnedAt"),
        ("217", "$..Moves[*].moveLearnCondition"),
        // Group 4: cfg10 fields
        ("218", "$.cfg10[*].ID"),
        ("219", "$.cfg10[*].Abilities[*]"),
        ("220", "$.cfg10[*].Moves[*].levelLearnedAt"),
        // Group 5: cfg25 fields
        ("221", "$.cfg25[*].ID"),
        ("222", "$.cfg25[*].Abilities[*]"),
        ("223", "$.cfg25[*].Moves[*].levelLearnedAt"),
        // Group 6: cfg40 fields
        ("224", "$.cfg40[*].ID"),
        ("225", "$.cfg40[*].Abilities[*]"),
        ("226", "$.cfg40[*].Moves[*].levelLearnedAt"),
        // Group 7: cfg50 fields
        ("227", "$.cfg50[*].ID"),
        ("228", "$.cfg50[*].Abilities[*]"),
        ("229", "$.cfg50[*].Moves[*].levelLearnedAt"),
    ],
);
