use proc_macro2::TokenStream;
use quote::quote;
use std::{
    fmt::Display,
    fs,
    io::{self, Write},
    path::Path,
    time::{Duration, Instant},
};

mod discovery;
mod gen;
mod model;

pub fn generate_tests<P1, P2>(toml_directory_path: P1, output_json_directory_path: P2) -> Result<TokenStream, io::Error>
where
    P1: AsRef<Path>,
    P2: AsRef<Path>,
{
    println!("discovery...");

    let discovery_start = Instant::now();
    let all_documents = discovery::discover(toml_directory_path)?;
    let discovery_elapsed = FormatDuration(discovery_start.elapsed());

    let test_set = gen::TestSet::new(all_documents);
    let stats = test_set.stats();

    println!(
        "discovered {} documents with a total of {} queries; finished in {}",
        stats.number_of_documents(),
        stats.number_of_queries(),
        discovery_elapsed
    );

    println!("generating jsons...");

    fs::create_dir_all(&output_json_directory_path)?;
    for (path, contents) in test_set.get_required_test_files(&output_json_directory_path) {
        let mut file = fs::File::create(path)?;
        write!(file, "{}", contents)?;
    }

    let imports = gen::generate_imports();
    let sources = test_set.generate_test_fns(output_json_directory_path);

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
