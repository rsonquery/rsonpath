use std::error::Error;

fn main() -> Result<(), Box<dyn Error>> {
    println!("cargo:rerun-if-changed=build.rs");
    println!("cargo:rerun-if-changed=./tests/documents/toml/*");
    println!("cargo:rerun-if-changed=./tests/documents/json/large/*");
    println!("cargo:rerun-if-changed=./tests/end_to_end.rs");
    println!("cargo:rerun-if-changed=../rsonpath-test-codegen/**/*");

    #[cfg(feature = "gen-tests")]
    test_gen::generate()?;

    #[cfg(feature = "simd")]
    {
        #[cfg(any(target_arch = "x86", target_arch = "x86_64"))]
        {
            if is_x86_feature_detected!("avx2") {
                eprintln!("AVX2 support detected, using simd=avx2");
                println!(r#"cargo:rustc-cfg=simd="avx2""#);
                return Ok(());
            }
        }

        panic!(
            "Target architecture is not supported by SIMD features of this crate. Disable the default `simd` feature."
        )
    }
    #[cfg(not(feature = "simd"))]
    {
        println!("cargo:warning=Building rsonpath without SIMD support, expect lower performance.");
        Ok(())
    }
}
