use console::style;
use std::{
    error::Error,
    fmt::Display,
    path::Path,
    process::ExitCode,
    time::{Duration, Instant},
};

mod diff;
mod discovery;
mod model;
mod runner;

pub fn test<P: AsRef<Path>>(directory_path: P) -> Result<ExitCode, Box<dyn Error>> {
    println!("discovery...");

    let discovery_start = Instant::now();
    let all_documents = discovery::discover(directory_path)?;
    let discovery_elapsed = FormatDuration(discovery_start.elapsed());

    let test_set = runner::TestSet::new(all_documents);
    let stats = test_set.stats();

    println!(
        "discovered {} documents with a total of {} queries; finished in {}",
        stats.number_of_documents(),
        stats.number_of_queries(),
        discovery_elapsed
    );

    println!("running {} tests...", stats.number_of_test_runs());

    let run_start = Instant::now();
    let result = test_set.run();
    let run_elapsed = FormatDuration(run_start.elapsed());

    if result.failed().is_empty() {
        println!("test result: {}. finished in {}", style("ok").green(), run_elapsed);
        println!();
        Ok(ExitCode::SUCCESS)
    } else {
        println!("failures:");
        for failure in result.failed() {
            println!("\t{}", failure.case_name());
            println!("\t\t{}", failure.reason());
        }

        println!("test result: {}. finished in {}", style("FAILED").red(), run_elapsed);
        println!();
        Ok(ExitCode::from(2))
    }
}

struct FormatDuration(Duration);

impl Display for FormatDuration {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:.2}s", self.0.as_secs_f32())
    }
}
