use crate::files::Files;
use proc_macro2::TokenStream;
use quote::quote;
use std::{
    fmt::Display,
    io,
    path::{Path, PathBuf},
    time::{Duration, Instant},
};

mod compression;
mod files;
mod gen;
mod model;

/// Parsed TOML document declaration annotated with its name and path.
#[derive(Clone)]
pub(crate) struct DiscoveredDocument {
    /// Name of the file.
    pub name: String,
    /// Path relative to the source TOML directory.
    pub relative_path: PathBuf,
    /// Parsed TOML document.
    pub document: model::Document,
}

/// Generate the source of end-to-end tests based on the TOML configuration in `toml_directory_path`.
/// As a side-effect, JSON files are written to `output_json_directory_path`, and additional variants
/// with compressed inputs of TOML configs are generated.
pub fn generate_tests<P1, P2>(toml_directory_path: P1, output_json_directory_path: P2) -> Result<TokenStream, io::Error>
where
    P1: AsRef<Path>,
    P2: AsRef<Path>,
{
    println!("discovery...");

    let discovery_start = Instant::now();
    let mut files = Files::new(output_json_directory_path, toml_directory_path)?;

    println!("generating compressed variants...");

    compression::generate_compressed_documents(&mut files)?;

    let stats = files.stats();
    let discovery_elapsed = FormatDuration(discovery_start.elapsed());

    println!(
        "prepared {} documents with a total of {} queries; finished in {}",
        stats.number_of_documents(),
        stats.number_of_queries(),
        discovery_elapsed
    );

    println!("generating tests...");

    let imports = gen::generate_imports();
    let sources = gen::generate_test_fns(&mut files).into_iter();

    println!("writing files...");
    files.flush()?;

    Ok(quote! {
        #imports

        #(#sources)*
    })
}

/// Wrapper implementing [`Display`] for [`Duration`] which shows the duration in seconds.
struct FormatDuration(Duration);

impl Display for FormatDuration {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:.2}s", self.0.as_secs_f32())
    }
}
