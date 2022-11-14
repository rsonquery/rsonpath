use rsonpath_benchmarks::prelude::*;

pub fn doi(c: &mut Criterion) -> Result<(), BenchmarkError> {
    let benchset = Benchset::new("DOI", dataset::crossref(2))?
        .add_all_targets_except_jsonski("$..DOI")?
        .finish();

    benchset.run(c);

    Ok(())
}

pub fn title(c: &mut Criterion) -> Result<(), BenchmarkError> {
    let benchset = Benchset::new("title", dataset::crossref(2))?
        .add_all_targets_except_jsonski("$..title")?
        .add_target(BenchTarget::JsonSki("$.items[*].title"))?
        .finish();

    benchset.run(c);

    Ok(())
}

pub fn orcid(c: &mut Criterion) -> Result<(), BenchmarkError> {
    let benchset = Benchset::new("orcid", dataset::crossref(2))?
        .add_all_targets_except_jsonski("$..author..ORCID")?
        .add_target(BenchTarget::JsonSki("$.items[*].author[*].ORCID"))?
        .finish();

    benchset.run(c);

    Ok(())
}

pub fn author_affiliation(c: &mut Criterion) -> Result<(), BenchmarkError> {
    let benchset = Benchset::new("author_affiliation", dataset::crossref(2))?
        .add_all_targets_except_jsonski("$..author..affiliation..name")?
        .add_target(BenchTarget::JsonSki(
            "$.items[*].author[*].affiliation[*].name",
        ))?
        .finish();

    benchset.run(c);

    Ok(())
}
pub fn editor_affiliation(c: &mut Criterion) -> Result<(), BenchmarkError> {
    let benchset = Benchset::new("editor", dataset::crossref(2))?
        .add_all_targets_except_jsonski("$..editor..affiliation..name")?
        .add_target(BenchTarget::JsonSki(
            "$.items[*].editor[*].affiliation[*].name",
        ))?
        .finish();
    benchset.run(c);
    Ok(())
}

fn scalability(c: &mut Criterion, size: u32) -> Result<(), BenchmarkError> {
    let benchset = Benchset::new(
        format!("scalability_affiliation{size}"),
        dataset::crossref(size),
    )?
    .add_target(BenchTarget::Rsonpath("$..affiliation..name"))?
    .finish();

    benchset.run(c);

    Ok(())
}

pub fn scalability0(c: &mut Criterion) -> Result<(), BenchmarkError> {
    scalability(c, 0)
}

pub fn scalability1(c: &mut Criterion) -> Result<(), BenchmarkError> {
    scalability(c, 1)
}

pub fn scalability2(c: &mut Criterion) -> Result<(), BenchmarkError> {
    scalability(c, 2)
}

pub fn scalability4(c: &mut Criterion) -> Result<(), BenchmarkError> {
    scalability(c, 4)
}

pub fn scalability8(c: &mut Criterion) -> Result<(), BenchmarkError> {
    scalability(c, 8)
}

pub fn scalability16(c: &mut Criterion) -> Result<(), BenchmarkError> {
    scalability(c, 16)
}

benchsets!(
    crossref_benches,
    author_affiliation,
    editor_affiliation,
    orcid,
    title,
    doi,
    scalability0,
    scalability1,
    scalability2,
    scalability4,
    scalability8,
    scalability16,
);
