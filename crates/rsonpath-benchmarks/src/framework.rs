use crate::{
    rsonpath::{Rsonpath, RsonpathError},
    rust_jsonski::{JsonSki, JsonSkiError},
    rust_jsurfer::{JSurfer, JSurferError},
};

use self::benchmark_options::BenchmarkOptions;
use self::implementation::prepare;
use self::json_document::JsonDocument;
use criterion::{Criterion, Throughput};
use implementation::{Implementation, PreparedQuery};
use std::{
    path::{Path, PathBuf},
    time::Duration,
};
use thiserror::Error;

pub mod benchmark_options;
pub mod implementation;
pub mod json_document;

#[derive(Clone, Copy, Debug)]
pub enum BenchTarget<'q> {
    Rsonpath(&'q str),
    JsonSki(&'q str),
    JSurfer(&'q str),
}

pub struct Benchset {
    id: String,
    options: BenchmarkOptions,
    json_document: JsonDocument,
    implementations: Vec<Box<dyn BenchFn>>,
}

pub struct ConfiguredBenchset {
    source: Benchset,
}

impl ConfiguredBenchset {
    pub fn run(&self, c: &mut Criterion) {
        let bench = &self.source;
        let mut group = c.benchmark_group(&bench.id);

        bench.options.apply_to(&mut group);
        group.throughput(Throughput::BytesDecimal(bench.json_document.size_in_bytes));

        for implementation in bench.implementations.iter() {
            let id = format!("{}_{}", &bench.id, implementation.id());
            group.bench_function(id, |b| b.iter(move || implementation.run()));
        }

        group.finish();
    }
}

impl Benchset {
    pub fn new<S: Into<String>, P: AsRef<Path>>(
        id: S,
        file_path: P,
    ) -> Result<Self, BenchmarkError> {
        let file_path_str = file_path
            .as_ref()
            .to_str()
            .ok_or_else(|| BenchmarkError::InvalidFilePath(file_path.as_ref().to_owned()))?;
        let json_document = JsonDocument::new(file_path_str.to_owned())?;

        let warm_up_time = if json_document.size_in_bytes < 10_000_000 {
            None
        } else if json_document.size_in_bytes < 100_000_000 {
            Some(Duration::from_secs(5))
        } else {
            Some(Duration::from_secs(10))
        };
        let measurement_time = if json_document.size_in_bytes < 1_000_000 {
            None
        } else if json_document.size_in_bytes < 10_000_000 {
            Some(Duration::from_secs(10))
        } else if json_document.size_in_bytes < 100_000_000 {
            Some(Duration::from_secs(25))
        } else {
            Some(Duration::from_secs(45))
        };
        let sample_count = if json_document.size_in_bytes < 100_000_000 {
            None
        } else {
            Some(10)
        };

        Ok(Self {
            id: format!("{}_{}", json_document.file_path, id.into()),
            options: BenchmarkOptions {
                warm_up_time,
                measurement_time,
                sample_count,
            },
            json_document,
            implementations: vec![],
        })
    }

    pub fn add_target(mut self, target: BenchTarget<'_>) -> Result<Self, BenchmarkError> {
        let bench_fn = target.to_bench_fn(&self.json_document.file_path)?;
        self.implementations.push(bench_fn);
        Ok(self)
    }

    pub fn add_all_targets_except_jsonski(self, query: &str) -> Result<Self, BenchmarkError> {
        self.add_target(BenchTarget::Rsonpath(query))?
            .add_target(BenchTarget::JSurfer(query))
    }

    pub fn add_all_targets(self, query: &str) -> Result<Self, BenchmarkError> {
        self.add_target(BenchTarget::Rsonpath(query))?
            .add_target(BenchTarget::JsonSki(query))?
            .add_target(BenchTarget::JSurfer(query))
    }

    pub fn finish(self) -> ConfiguredBenchset {
        ConfiguredBenchset { source: self }
    }
}

trait Target {
    fn to_bench_fn(self, file_path: &str) -> Result<Box<dyn BenchFn>, BenchmarkError>;
}

impl<'a> Target for BenchTarget<'a> {
    fn to_bench_fn(self, file_path: &str) -> Result<Box<dyn BenchFn>, BenchmarkError> {
        match self {
            BenchTarget::Rsonpath(q) => {
                let rsonpath = Rsonpath::new()?;
                let prepared = prepare(rsonpath, file_path, q)?;
                Ok(Box::new(prepared))
            }
            BenchTarget::JsonSki(q) => {
                let jsonski = JsonSki::new()?;
                let prepared = prepare(jsonski, file_path, q)?;
                Ok(Box::new(prepared))
            }
            BenchTarget::JSurfer(q) => {
                let jsurfer = JSurfer::new()?;
                let prepared = prepare(jsurfer, file_path, q)?;
                Ok(Box::new(prepared))
            }
        }
    }
}

trait BenchFn {
    fn id(&self) -> &str;

    fn run(&self) -> u64;
}

impl<I: Implementation> BenchFn for PreparedQuery<I> {
    fn id(&self) -> &str {
        I::id()
    }

    fn run(&self) -> u64 {
        self.implementation.run(&self.query, &self.file).unwrap()
    }
}

#[derive(Error, Debug)]
pub enum BenchmarkError {
    #[error("invalid dataset file path, has to be valid UTF-8: '{0}'")]
    InvalidFilePath(PathBuf),
    #[error(r#"dataset not found, either the path is invalid or the bench was started in an unexpected way
    
    Here's what can help:
    1. Ensure the dataset was downloaded with dl.sh and exists at the path '{0}' relative to the root of the rsonpath-benchmarks crate.
    2. Ensure the benchmarks is run with `cargo bench --bench <name>`. See the Usage section of the README."#)]
    FileNotFound(PathBuf, #[source] std::io::Error),
    #[error("error preparing Rsonpath bench")]
    RsonpathError(#[from] RsonpathError),
    #[error("error preparing JsonSki bench")]
    JsonSkiError(#[from] JsonSkiError),
    #[error("error preparing JSurfer bench")]
    JSurferError(#[from] JSurferError),
}
