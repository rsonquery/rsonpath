use std::error::Error;
use vergen::EmitBuilder;

const CODEGEN_ENV_KEY: &str = "RSONPATH_CODEGEN_FLAGS";

fn main() -> Result<(), Box<dyn Error>> {
    EmitBuilder::builder()
        .idempotent()
        .git_sha(false)
        .cargo_features()
        .cargo_opt_level()
        .cargo_target_triple()
        .all_rustc()
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
