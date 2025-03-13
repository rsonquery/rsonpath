use clap::{Parser, Subcommand};
use rsonpath::lookup_table::{
    distance_counter::{self, DISTANCE_EVAL_DIR},
    lut_sichash,
    performance::{self, lut_skip_counter, lut_skip_evaluation, EVAL_DIR},
    pokemon_test_data_generator,
    query_with_lut::query_with_lut,
    sichash_test_data_generator::{self, SICHASH_DATA_DIR},
};
use std::{error::Error, fs, path::Path};

#[derive(Parser)]
#[command(
    name = "LUT Performance Tool",
    about = "A tool for evaluating performance and distances."
)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// Apply a query on the a given JSON file
    Query {
        /// Query to be applied
        json_query: String,
        /// Path to the JSON file
        json_path: String,
    },
    /// Measure distances of each parenthesis pair for each JSON in the folder and plot the distribution
    Distances {
        /// Path to the folder containing JSON files
        json_dir: String,
        /// Path to the output directory where the results are saved
        out_dir: String,
    },
    /// Run performance tests
    Performance {
        /// Path to the input JSON folder
        json_dir: String,
        /// Path to the output directory
        out_dir: String,
    },
    Skip {},
    SkipCount {},
    TestSicHash {},
    /// Create the test data used in this project: https://github.com/KraftRicardo/test-SicHash
    Sichash {
        /// Path to the folder containing JSON files
        json_dir: String,
        /// Path to the output directory where the results are saved
        out_dir: String,
    },
    Pokemon {
        json_path: String,
    },
}

fn main() -> Result<(), Box<dyn Error>> {
    let cli = Cli::parse();

    match &cli.command {
        Commands::Query { json_query, json_path } => {
            query_with_lut(json_path, json_query);
        }
        Commands::Distances { json_dir, out_dir } => {
            check_if_dir_exists(json_dir);
            create_folder_setup(out_dir)?;
            let csv_dir = format!("{}/{}", out_dir, "performance");

            distance_counter::count_distances_in_dir(json_dir, &csv_dir);
        }
        Commands::Performance { json_dir, out_dir } => {
            check_if_dir_exists(json_dir);
            create_folder_setup(out_dir)?;
            let csv_dir = format!("{}/{}", out_dir, "performance");

            performance::performance_test(json_dir, &csv_dir);
        }
        Commands::Skip {} => {
            lut_skip_evaluation::skip_evaluation();
        }
        Commands::SkipCount {} => {
            lut_skip_counter::track_skips();
        }
        Commands::TestSicHash {} => {
            lut_sichash::test_sichash_lut();
        }
        Commands::Sichash { json_dir, out_dir } => {
            check_if_dir_exists(json_dir);
            create_folder_setup(out_dir)?;
            let csv_dir = format!("{}/{}", out_dir, "performance");

            sichash_test_data_generator::generate_test_data_for_sichash(json_dir, &csv_dir);
        }
        Commands::Pokemon { json_path } => {
            pokemon_test_data_generator::generate_bigger_version(json_path);
        }
    }

    Ok(())
}

/// Creates the required folder structure if it does not exist.
fn create_folder_setup(dir_name: &str) -> std::io::Result<()> {
    let dirs = [
        dir_name,
        &format!("{}/performance", dir_name),
        &format!("{}/performance/{}", dir_name, EVAL_DIR),
        &format!("{}/performance/{}", dir_name, DISTANCE_EVAL_DIR),
        &format!("{}/performance/{}", dir_name, SICHASH_DATA_DIR),
        &format!("{}/test_data", dir_name),
    ];

    for dir in &dirs {
        let path = Path::new(dir);
        if !path.exists() {
            fs::create_dir_all(path)?;
            println!("Created directory: {}", dir);
        }
    }

    Ok(())
}

fn check_if_dir_exists(path: &str) {
    if fs::metadata(path).is_err() {
        panic!("Error: The provided folder '{}' does not exist.", path);
    } else if !Path::new(path).is_dir() {
        panic!("Error: The provided folder '{}' is not a directory.", path);
    }
}
