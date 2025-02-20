use std::{
    io::{self, BufReader, Read, Write},
    path::Path,
    process::Command,
};

use crate::{engine::skip_tracker, lookup_table::LookUpTableImpl};

use super::lut_skip_evaluation::{
    get_filename, BESTBUY_JSON, GOOGLE_MAP_JSON, TWITTER_JSON, TWITTER_MINI_JSON, TWITTER_SHORT_JSON,
};

use crate::{
    engine::{Compiler, Engine, RsonpathEngine},
    input::OwnedBytes,
    lookup_table::LookUpTable,
};
use std::{error::Error, fs};

const COUNTER_FILE_PATH: &str = ".a_lut_tests/performance/skip_tracker/COUNTER.csv";

pub fn count_skips() {
    // count_bestbuy();

    let _ = track_skips(TWITTER_MINI_JSON, "q0", "$.search_metadata.count");

    // let _ = track_skips(TWITTER_SHORT_JSON, "q0", "$.search_metadata.count");
    // let _ = track_skips(TWITTER_SHORT_JSON, "q1", "$.created_at");
    // let _ = track_skips(TWITTER_SHORT_JSON, "q2", "$.user.name");
    // let _ = track_skips(TWITTER_SHORT_JSON, "q4", "$.retweeted_status.user.name");
    // let _ = track_skips(TWITTER_SHORT_JSON, "q5", "$.entities.urls[*].expanded_url");
    // let _ = track_skips(TWITTER_SHORT_JSON, "q6", "$.user.followers_count");
    // let _ = track_skips(TWITTER_SHORT_JSON, "q7", "$.user.friends_count");
    // let _ = track_skips(TWITTER_SHORT_JSON, "q8", "$.retweeted_status.retweet_count");
    // let _ = track_skips(TWITTER_SHORT_JSON, "q9", "$.retweeted_status.favorite_count");
    // let _ = track_skips(TWITTER_SHORT_JSON, "q10", "$.retweeted_status.entities.urls[*].expanded_url");

    // let _ = track_skips(POKEMON_JSON, "q0","$.cfgs[17].Moves[*]");
    // let _ = track_skips(POKEMON_JSON, "q1","$.cfgs[0].Name");
    // let _ = track_skips(POKEMON_JSON, "q2","$.cfgs[*].Name");
    // let _ = track_skips(POKEMON_JSON, "q3","$.cfgs[*].Moves[*].levelLearnedAt");
    // let _ = track_skips(POKEMON_JSON, "q4","$.cfgs[*].Moves[*]");

    // let _ = track_skips(WALMART_SHORT_JSON, "q0","$.items[*].name");
    // let _ = track_skips(WALMART_SHORT_JSON, "q1","$.items[50].stock");

    // let _ = track_skips(WALMART_JSON, "q0","$.items[50].stock");

    // let _ = track_skips(TWITTER_SHORT_JSON, "q0","$.search_metadata.count");
    // let _ = track_skips(TWITTER_JSON, "q0","$.search_metadata.count");
}

pub fn count_bestbuy() {
    let _ = track_skips(BESTBUY_JSON, "b0", "$.products[5].videoChapters");
    let _ = track_skips(BESTBUY_JSON, "b1", "$.products[*].videoChapters");
    let _ = track_skips(BESTBUY_JSON, "b2", "$.products[2].categoryPath[*].id");
    let _ = track_skips(BESTBUY_JSON, "b3", "$.products[5].categoryPath[1].id");
    let _ = track_skips(
        BESTBUY_JSON,
        "b4",
        "$.products[5].shippingLevelsOfService[1].serviceLevelName",
    );
    let _ = track_skips(
        BESTBUY_JSON,
        "b5",
        "$.products[10].shippingLevelsOfService[1].serviceLevelName",
    );
    let _ = track_skips(BESTBUY_JSON, "b6", "$.products[*].videoChapters[1].chapter");
    let _ = track_skips(BESTBUY_JSON, "b7", "$.products[20].monthlyRecurringChargeGrandTotal");
    let _ = track_skips(BESTBUY_JSON, "b8", "$.products[*].videoChapters[5].chapter");
    let _ = track_skips(BESTBUY_JSON, "b9", "$.products[*].monthlyRecurringChargeGrandTotal");

    let _ = track_skips(BESTBUY_JSON, "y0", "$.total");
    let _ = track_skips(BESTBUY_JSON, "y1", "$.products[*].shipping[*]");
    let _ = track_skips(
        BESTBUY_JSON,
        "y2",
        "$.products[*].shippingLevelsOfService[1].serviceLevelName",
    );
    let _ = track_skips(BESTBUY_JSON, "y3", "$.products[*].categoryPath[2]");
    let _ = track_skips(
        BESTBUY_JSON,
        "y4",
        "$.products[*].shippingLevelsOfService[*].serviceLevelName",
    );
}

pub fn count_google_map_large() {
    let _ = track_skips(GOOGLE_MAP_JSON, "b0", "$[*].available_travel_modes");
    let _ = track_skips(GOOGLE_MAP_JSON, "b1", "$[*].routes[*].legs[*].steps[*]");
    let _ = track_skips(GOOGLE_MAP_JSON, "b2", "$[*].routes[*].legs[*]");
    let _ = track_skips(GOOGLE_MAP_JSON, "b3", "$[1]");
    let _ = track_skips(GOOGLE_MAP_JSON, "b4", "$[200].routes[1].legs[5].steps[*].distance.text");
    let _ = track_skips(GOOGLE_MAP_JSON, "b5", "$[*].routes[*].legs[*].steps[1]");
    let _ = track_skips(GOOGLE_MAP_JSON, "b6", "$[500].routes[*].legs[5].steps[*].distance.text");
    let _ = track_skips(
        GOOGLE_MAP_JSON,
        "b7",
        "$[1000].routes[1].legs[5].steps[*].distance.text",
    );
    let _ = track_skips(
        GOOGLE_MAP_JSON,
        "b8",
        "$[10000].routes[1].legs[5].steps[*].distance.text",
    );
    let _ = track_skips(GOOGLE_MAP_JSON, "b9", "$[10000].routes[*]");
    let _ = track_skips(GOOGLE_MAP_JSON, "b10", "$[10000].routes[*].legs[*].steps[1]");
    let _ = track_skips(
        GOOGLE_MAP_JSON,
        "b11",
        "$[10000].routes[*].legs[1].steps[*].distance.text",
    );

    let _ = track_skips(GOOGLE_MAP_JSON, "y0", "$[*].routes[*].legs[*].steps[*].distance.text");
    let _ = track_skips(GOOGLE_MAP_JSON, "y1", "$[*].routes[*]");
    let _ = track_skips(GOOGLE_MAP_JSON, "y2", "$[*].routes[*].warnings");
    let _ = track_skips(GOOGLE_MAP_JSON, "y3", "$[*].routes[*].bounds[*]");
    let _ = track_skips(GOOGLE_MAP_JSON, "y4", "$[*].routes[*].legs[*].steps[1].distance.text");
    let _ = track_skips(GOOGLE_MAP_JSON, "y5", "$[*].routes[*].legs[1].steps[*].distance.text");
    let _ = track_skips(GOOGLE_MAP_JSON, "y6", "$[*].routes[1].legs[*].steps[*].distance.text");
    let _ = track_skips(GOOGLE_MAP_JSON, "y7", "$[1].routes[*].legs[*].steps[*].distance.text");
}

fn track_skips(json_path: &str, query_name: &str, query_text: &str) -> Result<(), Box<dyn Error>> {
    if !skip_tracker::is_off() {
        println!(
            "Mode={:?}: Process query: {} = {}",
            skip_tracker::MODE,
            query_name,
            query_text
        );
    } else {
        println!("No tracking set. Abort.");
        return Ok(());
    }

    // Build lut
    let lut = LookUpTableImpl::build(json_path, 0)?;

    // Build query
    let query = rsonpath_syntax::parse(query_text)?;
    let mut engine = RsonpathEngine::compile_query(&query)?;
    engine.add_lut(lut);

    // Get results
    let input = {
        let mut file = BufReader::new(fs::File::open(json_path)?);
        let mut buf = vec![];
        file.read_to_end(&mut buf)?;
        // Here you can define whether to use OwnedBytes (padding), Mmap (padding = 0)  or Borrowed (padding)
        OwnedBytes::new(buf)
    };
    let mut sink = vec![];
    engine.matches(&input, &mut sink)?;
    let result = sink.into_iter().map(|m| m.span().start_idx()).collect::<Vec<_>>();

    // for number in result {
    //     println!("Result: {}", number);
    // }

    let filename = get_filename(json_path);
    if skip_tracker::is_counting() {
        println!("File = {filename}, Query = {query_text} ");
        let _ = skip_tracker::print_count_results_and_save_in_csv(&COUNTER_FILE_PATH, filename, query_text);
    } else if skip_tracker::is_tracking() {
        // Save the tracked skips to a csv
        let file_path = format!(".a_lut_tests/performance/skip_tracker/{}_{}.csv", filename, query_name);
        let save_result = skip_tracker::save_track_results_to_csv(&file_path);
        if let Err(e) = save_result {
            eprintln!("Failed to save to CSV: {}", e);
        }
        plot_tracked_skips(&file_path);
    }

    Ok(())
}

fn plot_tracked_skips(csv_path: &str) {
    let msg = format!("Failed to open csv_path: {}", csv_path);
    let output = Command::new("python")
        .arg("crates/rsonpath-lib/src/lookup_table/python_statistic/skip_tracker_distribution.py")
        .arg(csv_path)
        .output()
        .expect(&msg);

    if output.status.success() {
        if let Err(e) = io::stdout().write_all(&output.stdout) {
            eprintln!("Failed to write stdout: {}", e);
        }
    } else {
        eprintln!("Error: {}", String::from_utf8_lossy(&output.stderr));
    }
}
