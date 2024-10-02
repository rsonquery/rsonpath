use std::error::Error;
use vergen::{CargoBuilder, Emitter, RustcBuilder};
use vergen_git2::Git2Builder;

const CODEGEN_ENV_KEY: &str = "RSONPATH_CODEGEN_FLAGS";

fn main() -> Result<(), Box<dyn Error>> {
    let cargo = CargoBuilder::default()
        .features(true)
        .opt_level(true)
        .target_triple(true)
        .build()?;
    let git = Git2Builder::default().sha(false).build()?;
    let rustc = RustcBuilder::all_rustc()?;

    Emitter::new()
        .idempotent()
        .add_instructions(&cargo)?
        .add_instructions(&git)?
        .add_instructions(&rustc)?
        .emit()?;

    let codegen_flags = concat_codegen_flags();

    println!("cargo:rustc-env={CODEGEN_ENV_KEY}={codegen_flags}");

    Ok(())
}

fn concat_codegen_flags() -> String {
    rustflags::from_env()
        .filter_map(|x| match x {
            rustflags::Flag::Codegen { opt, value } => {
                Some(opt + &value.map_or(String::default(), |x| format!("={x}")))
            }
            _ => None,
        })
        .reduce(|s, x| format!("{s},{x}"))
        .unwrap_or_default()
}
