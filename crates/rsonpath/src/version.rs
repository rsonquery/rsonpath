use std::sync::OnceLock;

static LONG_VERSION: OnceLock<String> = OnceLock::new();

pub(super) fn get_long_version() -> &'static str {
    LONG_VERSION.get_or_init(|| {
        let mut res = env!("CARGO_PKG_VERSION").to_owned();
        let simd = rsonpath_lib::classification::describe_simd();
        let details = [
            ("Commit SHA:", env!("VERGEN_GIT_SHA")),
            ("Features:", env!("VERGEN_CARGO_FEATURES")),
            ("Opt level:", env!("VERGEN_CARGO_OPT_LEVEL")),
            ("Target triple:", env!("VERGEN_CARGO_TARGET_TRIPLE")),
            ("Codegen flags:", env!("RSONPATH_CODEGEN_FLAGS")),
            ("SIMD support:", &format!("{simd}")),
        ];

        res += "\n";
        for (k, v) in details {
            if v != "VERGEN_IDEMPOTENT_OUTPUT" {
                res += &format!("\n{: <16} {}", k, v);
            }
        }

        res
    })
}
