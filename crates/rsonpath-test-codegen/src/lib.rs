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

#[derive(Clone)]
pub(crate) struct DiscoveredDocument {
    pub name: String,
    pub relative_path: PathBuf,
    pub document: model::Document,
}

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

struct FormatDuration(Duration);

impl Display for FormatDuration {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:.2}s", self.0.as_secs_f32())
    }
}
