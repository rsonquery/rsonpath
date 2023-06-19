use proc_macro2::TokenStream;
use quote::quote;
use std::{
    fmt::Display,
    io,
    path::Path,
    time::{Duration, Instant},
};

mod codegen;
mod discovery;
mod model;

pub fn test_source<P: AsRef<Path>>(directory_path: P) -> Result<TokenStream, io::Error> {
    println!("discovery...");

    let discovery_start = Instant::now();
    let all_documents = discovery::discover(directory_path)?;
    let discovery_elapsed = FormatDuration(discovery_start.elapsed());

    let test_set = codegen::TestSet::new(all_documents);
    let stats = test_set.stats();

    println!(
        "discovered {} documents with a total of {} queries; finished in {}",
        stats.number_of_documents(),
        stats.number_of_queries(),
        discovery_elapsed
    );

    let imports = codegen::generate_imports();
    let sources = test_set.generate_test_fns();

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
