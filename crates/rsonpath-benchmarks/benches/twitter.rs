use rsonpath_benchmarks::prelude::*;

pub fn metadata_1(c: &mut Criterion) -> Result<(), BenchmarkError> {
    let benchset = Benchset::new("metadata_1", dataset::twitter())?
        .add_all_targets("$.search_metadata.count")?
        .finish();

    benchset.run(c);

    Ok(())
}

pub fn metadata_2(c: &mut Criterion) -> Result<(), BenchmarkError> {
    let benchset = Benchset::new("metadata_2", dataset::twitter())?
        .add_all_targets_except_jsonski("$..search_metadata.count")?
        .add_target(BenchTarget::JsonSki("$.search_metadata.count"))?
        .finish();

    benchset.run(c);

    Ok(())
}

pub fn metadata_3(c: &mut Criterion) -> Result<(), BenchmarkError> {
    let benchset = Benchset::new("metadata_3", dataset::twitter())?
        .add_all_targets_except_jsonski("$..count")?
        .add_target(BenchTarget::JsonSki("$.search_metadata.count"))?
        .finish();

    benchset.run(c);

    Ok(())
}

pub fn all_hashtags(c: &mut Criterion) -> Result<(), BenchmarkError> {
    let benchset = Benchset::new("all_hashtags", dataset::twitter())?
        .add_all_targets_except_jsonski("$..hashtags..text")?
        .finish();

    benchset.run(c);

    Ok(())
}

pub fn hashtags_of_retweets(c: &mut Criterion) -> Result<(), BenchmarkError> {
    let benchset = Benchset::new("hashtags_of_retweets", dataset::twitter())?
        .add_all_targets_except_jsonski("$..retweeted_status..hashtags..text")?
        .add_target(BenchTarget::JsonSki(
            "$.statuses[*].retweeted_status.entities.hashtags[*].text",
        ))?
        .finish();

    benchset.run(c);

    Ok(())
}

benchsets!(
    twitter_benches,
    metadata_1,
    metadata_2,
    metadata_3,
    all_hashtags,
    hashtags_of_retweets
);
