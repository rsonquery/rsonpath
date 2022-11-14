use rsonpath_benchmarks::prelude::*;

pub fn decl_name(c: &mut Criterion) -> Result<(), BenchmarkError> {
    let benchset = Benchset::new("decl_name", dataset::ast())?
        .add_all_targets_except_jsonski("$..decl.name")?
        .finish();

    benchset.run(c);

    Ok(())
}

pub fn included_from(c: &mut Criterion) -> Result<(), BenchmarkError> {
    let benchset = Benchset::new("included_from", dataset::ast())?
        .add_all_targets_except_jsonski("$..loc.includedFrom.file")?
        .finish();

    benchset.run(c);

    Ok(())
}

pub fn nested_inner(c: &mut Criterion) -> Result<(), BenchmarkError> {
    let benchset = Benchset::new("nested_inner", dataset::ast())?
        .add_all_targets_except_jsonski("$..inner..inner..type.qualType")?
        .finish();

    benchset.run(c);

    Ok(())
}

benchsets!(ast_benches, decl_name, included_from, nested_inner);
