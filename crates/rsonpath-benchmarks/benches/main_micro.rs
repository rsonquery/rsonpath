use rsonpath_benchmarks::prelude::*;

fn az_shallow_tenant_ids(c: &mut Criterion) -> Result<(), BenchmarkError> {
    let benchset = Benchset::new("az_tenant::shallow_ids", dataset::az_tenants())?
        .do_not_measure_file_load_time()
        .measure_compilation_time()
        .add_rsonpath_with_all_result_types("$[*].tenantId")?
        .finish();

    benchset.run(c);

    Ok(())
}

fn az_recursive_tenant_ids(c: &mut Criterion) -> Result<(), BenchmarkError> {
    let benchset = Benchset::new("az_tenants::recursive_ids", dataset::az_tenants())?
        .do_not_measure_file_load_time()
        .measure_compilation_time()
        .add_rsonpath_with_all_result_types("$..tenantId")?
        .finish();

    benchset.run(c);

    Ok(())
}

fn az_first_ten_tenant_ids(c: &mut Criterion) -> Result<(), BenchmarkError> {
    let benchset = Benchset::new("az_tenants::first_ten_tenant_ids", dataset::az_tenants())?
        .do_not_measure_file_load_time()
        .measure_compilation_time()
        .add_rsonpath_with_all_result_types("$[:10].tenantId")?
        .finish();

    benchset.run(c);

    Ok(())
}

fn az_tenant_17(c: &mut Criterion) -> Result<(), BenchmarkError> {
    let benchset = Benchset::new("az_tenants::tenant_17", dataset::az_tenants())?
        .do_not_measure_file_load_time()
        .measure_compilation_time()
        .add_rsonpath_with_all_result_types("$[17]")?
        .finish();

    benchset.run(c);

    Ok(())
}

fn az_tenant_last(c: &mut Criterion) -> Result<(), BenchmarkError> {
    let benchset = Benchset::new("az_tenants::tenant_last", dataset::az_tenants())?
        .do_not_measure_file_load_time()
        .measure_compilation_time()
        .add_rsonpath_with_all_result_types("$[83]")?
        .finish();

    benchset.run(c);

    Ok(())
}

fn az_every_other_tenant(c: &mut Criterion) -> Result<(), BenchmarkError> {
    let benchset = Benchset::new("az_tenants::every_other_tenant", dataset::az_tenants())?
        .do_not_measure_file_load_time()
        .measure_compilation_time()
        .add_rsonpath_with_all_result_types("$[::2]")?
        .finish();

    benchset.run(c);

    Ok(())
}

benchsets!(
    main_micro_benches,
    az_shallow_tenant_ids,
    az_recursive_tenant_ids,
    az_first_ten_tenant_ids,
    az_tenant_17,
    az_tenant_last,
    az_every_other_tenant
);
