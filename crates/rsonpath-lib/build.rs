use std::error::Error;
const SIMD_ENVIRONMENT_VARIABLE: &str = "RSONPATH_UNSAFE_FORCE_SIMD";

fn main() -> Result<(), Box<dyn Error>> {
    println!("cargo:rerun-if-changed=build.rs");
    println!("cargo:rerun-if-env-changed={SIMD_ENVIRONMENT_VARIABLE}");

    if let Ok(simd) = std::env::var(SIMD_ENVIRONMENT_VARIABLE) {
        println!(
            r#"cargo:warning=OVERRIDING SIMD SUPPORT TO "{}". THIS IS UNSAFE."#,
            simd
        );
        println!(r#"cargo:rustc-cfg=simd="{}""#, simd);
        return Ok(());
    }

    #[cfg(feature = "simd")]
    {
        #[cfg(any(target_arch = "x86", target_arch = "x86_64"))]
        {
            if is_x86_feature_detected!("avx2") {
                #[cfg(target_arch = "x86_64")]
                {
                    eprintln!("AVX2 support detected on x86_64, using simd=avx2_64");
                    println!(r#"cargo:rustc-cfg=simd="avx2_64""#);
                    return Ok(());
                }

                #[cfg(target_arch = "x86")]
                {
                    eprintln!("AVX2 support detected on x86, using simd=avx2_32");
                    println!(r#"cargo:rustc-cfg=simd="avx2_32""#);
                    return Ok(());
                }
            }

            if is_x86_feature_detected!("ssse3") && is_x86_feature_detected!("pclmulqdq") {
                #[cfg(target_arch = "x86_64")]
                {
                    eprintln!("SSSE3 support detected on x86_64, using simd=ssse3_64");
                    println!(r#"cargo:rustc-cfg=simd="ssse3_64""#);
                    return Ok(());
                }

                #[cfg(target_arch = "x86")]
                {
                    eprintln!("SSSE3 support detected on x86, using simd=ssse3_32");
                    println!(r#"cargo:rustc-cfg=simd="ssse3_32""#);
                    return Ok(());
                }
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
