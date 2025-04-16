use clap::{Parser, Subcommand};
use rsonpath::lookup_table::{
    analysis::{
        distance_distribution, json_size_distribution::create_json_size_csv,
        json_size_estimation_bits::print_estimation,
    },
    performance::{self, distance_cutoff_evaluation, lut_query_data, lut_skip_counter, lut_skip_evaluation, EVAL_DIR},
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
        json_dir: String, // Path to the folder containing JSON files
        result_dir: String,
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
    TestQuery {},
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
    Cutoff {},
    Analysis {
        json_folder_path: String,
    },
}

fn main() -> Result<(), Box<dyn Error>> {
    let cli = Cli::parse();

    match &cli.command {
        Commands::Analysis { json_folder_path } => {
            // create_json_size_csv(json_folder_path);
            print_estimation();
        }
        Commands::Query { json_query, json_path } => {
            query_with_lut(json_path, json_query);
        }
        Commands::Distances { json_dir, result_dir } => {
            check_if_dir_exists(json_dir);
            check_if_dir_exists(result_dir);

            distance_distribution::count_distances_in_dir(json_dir, &result_dir);
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
        Commands::TestQuery {} => {
            lut_query_data::test_build_and_queries();
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
        Commands::Cutoff {} => {
            distance_cutoff_evaluation::evaluate();
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
