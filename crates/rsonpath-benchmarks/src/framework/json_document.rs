use std::fs;

use super::BenchmarkError;

pub(crate) struct JsonDocument {
    pub(crate) file_path: String,
    pub(crate) size_in_bytes: u64,
}

impl JsonDocument {
    pub(crate) fn new(file_path: String) -> Result<Self, BenchmarkError> {
        let metadata = fs::metadata(&file_path)
            .map_err(|err| BenchmarkError::FileNotFound(file_path.clone().into(), err))?;

        Ok(Self {
            file_path,
            size_in_bytes: metadata.len(),
        })
    }
}
