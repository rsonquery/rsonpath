use proc_macro2::TokenStream;
use quote::quote;
use std::{
    fmt::Display,
    fs,
    io::{self, Write},
    path::Path,
    time::{Duration, Instant},
};

mod compression;
mod discovery;
mod gen;
mod model;
mod paths;

pub fn generate_tests<P1, P2>(toml_directory_path: P1, output_json_directory_path: P2) -> Result<TokenStream, io::Error>
where
    P1: AsRef<Path>,
    P2: AsRef<Path>,
{
    println!("discovery...");

    let discovery_start = Instant::now();
    let all_documents: Vec<_> = discovery::discover(&toml_directory_path)?.into_iter().collect();

    for compressed in compression::get_compressed_toml_files(&all_documents) {
        write_file(
            &toml_directory_path,
            compressed.relative_path,
            model::serialize(&compressed.document),
        )?;
    }

    println!("generating compressed variants...");

    let test_set = gen::TestSet::new(all_documents);
    let stats = test_set.stats();
    let discovery_elapsed = FormatDuration(discovery_start.elapsed());

    println!(
        "discovered {} documents with a total of {} queries; finished in {}",
        stats.number_of_documents(),
        stats.number_of_queries(),
        discovery_elapsed
    );

    println!("generating jsons...");

    for (relative_path, contents) in test_set.get_required_test_files() {
        write_file(&output_json_directory_path, relative_path, contents)?;
    }

    let imports = gen::generate_imports();
    let sources = test_set.generate_test_fns(output_json_directory_path).into_iter();

    Ok(quote! {
        #imports

        #(#sources)*
    })
}

fn write_file<P1: AsRef<Path>, P2: AsRef<Path>, D: Display>(
    dir: P1,
    relative_path: P2,
    contents: D,
) -> Result<(), io::Error> {
    let full_path = Path::join(dir.as_ref(), relative_path);

    let dir = full_path.parent().expect("generated json files must have a parent");
    fs::create_dir_all(dir)?;
    let mut file = fs::File::create(full_path)?;
    write!(file, "{}", contents)
}

struct FormatDuration(Duration);

impl Display for FormatDuration {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:.2}s", self.0.as_secs_f32())
    }
}
