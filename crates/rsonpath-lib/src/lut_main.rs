use clap::{Parser, Subcommand};
use rsonpath::lookup_table::{
    count_distances::{self, DISTANCE_EVAL_DIR},
    performance::{self, HEAP_EVAL_DIR, TIME_EVAL_DIR},
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
    /// Setup the required folder structure
    Setup {
        /// Path to the folder holding the structure
        dir_path: String,
    },

    /// Measure distances for each JSON in the folder
    Distances {
        /// Path to the folder containing JSON files
        json_dir: String,

        /// Path to the output CSV folder
        csv_dir: String,
    },

    /// Run performance tests
    Performance {
        /// Path to the input JSON folder
        json_dir: String,

        /// Path to the output CSV folder
        csv_dir: String,

        /// Task to run: 0 for time eval, 1 for heap eval, 2 for both
        tasks: u16,
    },
}

fn main() -> Result<(), Box<dyn Error>> {
    let cli = Cli::parse();

    match &cli.command {
        Commands::Setup { dir_path } => {
            create_folder_setup(dir_path)?;
        }
        Commands::Distances { json_dir, csv_dir } => {
            count_distances::count_distances_in_dir(json_dir, csv_dir);
        }
        Commands::Performance {
            json_dir,
            csv_dir,
            tasks,
        } => {
            check_if_dir_exists(json_dir);
            check_if_dir_exists(csv_dir);
            performance::performance_test(json_dir, csv_dir, *tasks);
        }
    }

    Ok(())
}

/// Creates the required folder structure if it does not exist.
fn create_folder_setup(dir_name: &str) -> std::io::Result<()> {
    let dirs = [
        dir_name,
        &format!("{}/performance", dir_name),
        &format!("{}/performance/{}", dir_name, HEAP_EVAL_DIR),
        &format!("{}/performance/{}", dir_name, TIME_EVAL_DIR),
        &format!("{}/performance/{}", dir_name, DISTANCE_EVAL_DIR),
        &format!("{}/test_data", dir_name),
    ];

    for dir in &dirs {
        let path = Path::new(dir);
        if !path.exists() {
            fs::create_dir_all(path)?;
            println!("Created directory: {}", dir);
        } else {
            println!("Directory already exists: {}", dir);
        }
    }

    Ok(())
}

fn check_if_dir_exists(path: &str) {
    if fs::metadata(path).is_err() {
        eprintln!("Error: The provided folder '{}' does not exist.", path);
        std::process::exit(1);
    } else if !Path::new(path).is_dir() {
        eprintln!("Error: The provided folder '{}' is not a directory.", path);
        std::process::exit(1);
    }
}
