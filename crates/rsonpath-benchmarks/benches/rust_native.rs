use rsonpath_benchmarks::prelude::*;

pub fn ast_decl_inner(c: &mut Criterion) -> Result<(), BenchmarkError> {
    let benchset = Benchset::new("rust_native::ast::decl_inner", dataset::ast())?
        .measure_compilation_time()
        .add_rust_native_targets("$..decl.name")?
        .finish();

    benchset.run(c);

    Ok(())
}

pub fn twitter_metadata(c: &mut Criterion) -> Result<(), BenchmarkError> {
    let benchset = Benchset::new("rust_native::twitter::metadata", dataset::twitter())?
        .measure_compilation_time()
        .add_target_with_id(
            BenchTarget::RsonpathMmap("$.search_metadata.count", ResultType::Full),
            "rsonpath_direct",
        )?
        .add_target_with_id(
            BenchTarget::RsonpathMmap("$..count", ResultType::Full),
            "rsonpath_descendant",
        )?
        .add_target_with_id(
            BenchTarget::JsonpathRust("$.search_metadata.count"),
            "jsonpath-rust_direct",
        )?
        .add_target_with_id(BenchTarget::JsonpathRust("$..count"), "jsonpath-rust_descendant")?
        .add_target_with_id(
            BenchTarget::SerdeJsonPath("$.search_metadata.count"),
            "serde_json_path_direct",
        )?
        .add_target_with_id(BenchTarget::SerdeJsonPath("$..count"), "serde_json_path_descendant")?
        .finish();

    benchset.run(c);

    Ok(())
}

fn az_tenant_last(c: &mut Criterion) -> Result<(), BenchmarkError> {
    let benchset = Benchset::new("rust_native::az_tenants::tenant_last", dataset::az_tenants())?
        .measure_compilation_time()
        .add_rust_native_targets("$[83]")?
        .finish();

    benchset.run(c);

    Ok(())
}

fn az_tenant_ids(c: &mut Criterion) -> Result<(), BenchmarkError> {
    let benchset = Benchset::new("rust_native::az_tenant::tenant_ids", dataset::az_tenants())?
        .measure_compilation_time()
        .add_target_with_id(
            BenchTarget::RsonpathMmap("$[*].tenantId", ResultType::Full),
            "rsonpath_direct",
        )?
        .add_target_with_id(
            BenchTarget::RsonpathMmap("$..tenantId", ResultType::Full),
            "rsonpath_descendant",
        )?
        .add_target_with_id(BenchTarget::JsonpathRust("$[*].tenantId"), "jsonpath-rust_direct")?
        .add_target_with_id(BenchTarget::JsonpathRust("$..tenantId"), "jsonpath-rust_descendant")?
        .add_target_with_id(BenchTarget::SerdeJsonPath("$[*].tenantId"), "serde_json_path_direct")?
        .add_target_with_id(BenchTarget::SerdeJsonPath("$..tenantId"), "serde_json_path_descendant")?
        .finish();

    benchset.run(c);

    Ok(())
}

fn az_every_other_tenant(c: &mut Criterion) -> Result<(), BenchmarkError> {
    let benchset = Benchset::new("rust_native::az_tenant:every_other_tenant", dataset::az_tenants())?
        .measure_compilation_time()
        .add_rust_native_targets("$[::2]")?
        .finish();

    benchset.run(c);

    Ok(())
}

fn az_first_ten_tenant_ids(c: &mut Criterion) -> Result<(), BenchmarkError> {
    let benchset = Benchset::new("rust_native::az_tenant::first_ten_tenant_ids", dataset::az_tenants())?
        .measure_compilation_time()
        .add_rust_native_targets("$[:10].tenantId")?
        .finish();

    benchset.run(c);

    Ok(())
}

benchsets!(
    main_benches,
    ast_decl_inner,
    az_tenant_last,
    az_tenant_ids,
    az_every_other_tenant,
    az_first_ten_tenant_ids,
    twitter_metadata
);
