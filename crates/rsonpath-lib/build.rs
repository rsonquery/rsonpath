fn main() -> eyre::Result<()> {
    println!("cargo:rerun-if-changed=build.rs");

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

        Err(eyre::eyre!(
            "Target architecture is not supported by SIMD features of this crate. Disable the default `simd` feature."
        ))
    }
    #[cfg(not(feature = "simd"))]
    {
        println!("cargo:warning=Building rsonpath without SIMD support, expect lower performance.");
        Ok(())
    }
}
