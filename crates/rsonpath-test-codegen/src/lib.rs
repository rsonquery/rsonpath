// Generic pedantic lints.
#![warn(
    explicit_outlives_requirements,
    semicolon_in_expressions_from_macros,
    unreachable_pub,
    unused_import_braces,
    unused_lifetimes
)]
// Clippy pedantic lints.
#![warn(
    clippy::allow_attributes_without_reason,
    clippy::cast_lossless,
    clippy::cloned_instead_of_copied,
    clippy::empty_drop,
    clippy::empty_line_after_outer_attr,
    clippy::equatable_if_let,
    clippy::expl_impl_clone_on_copy,
    clippy::explicit_deref_methods,
    clippy::explicit_into_iter_loop,
    clippy::explicit_iter_loop,
    clippy::fallible_impl_from,
    clippy::flat_map_option,
    clippy::if_then_some_else_none,
    clippy::inconsistent_struct_constructor,
    clippy::large_digit_groups,
    clippy::let_underscore_must_use,
    clippy::manual_ok_or,
    clippy::map_err_ignore,
    clippy::map_unwrap_or,
    clippy::match_same_arms,
    clippy::match_wildcard_for_single_variants,
    clippy::mod_module_files,
    clippy::must_use_candidate,
    clippy::needless_continue,
    clippy::needless_for_each,
    clippy::needless_pass_by_value,
    clippy::ptr_as_ptr,
    clippy::redundant_closure_for_method_calls,
    clippy::ref_binding_to_reference,
    clippy::ref_option_ref,
    clippy::rest_pat_in_fully_bound_structs,
    clippy::undocumented_unsafe_blocks,
    clippy::unneeded_field_pattern,
    clippy::unseparated_literal_suffix,
    clippy::unreadable_literal,
    clippy::unused_self,
    clippy::use_self
)]

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
    pub(crate) name: String,
    /// Path relative to the source TOML directory.
    pub(crate) relative_path: PathBuf,
    /// Parsed TOML document.
    pub(crate) document: model::Document,
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
