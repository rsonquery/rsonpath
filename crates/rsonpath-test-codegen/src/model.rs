//! Type definitions and serde support for the TOML configuration files.
use std::{error::Error, fmt::Display, path::PathBuf};

use serde::{Deserialize, Serialize};

/// Top-level test configuration.
#[derive(Deserialize, Serialize, Clone)]
pub(crate) struct Document {
    /// Input JSON for tests.
    pub(crate) input: Input,
    /// Query cases to run on the input.
    pub(crate) queries: Vec<Query>,
}

/// Configuration of the input JSON document.
#[derive(Deserialize, Serialize, Clone)]
pub(crate) struct Input {
    /// Human-friendly description of the structure of the JSON.
    pub(crate) description: String,
    /// Whether the JSON is already minified. If true, no compressed counterpart is automatically created.
    pub(crate) is_compressed: bool,
    /// Source of the JSON.
    pub(crate) source: InputSource,
}

/// Source of the input JSON, which can be either an inline string or a path to a file.
#[derive(Deserialize, Serialize, Clone)]
pub(crate) enum InputSource {
    /// Source from a file with the given path relative to the JSON output directory.
    #[serde(rename = "large_file")]
    LargeFile(PathBuf),
    /// Source from the inline string.
    #[serde(rename = "json_string")]
    JsonString(String),
}

/// Single query defined for the test document.
#[derive(Deserialize, Serialize, Clone)]
pub(crate) struct Query {
    /// Human-friendly description of what the query does.
    pub(crate) description: String,
    /// JSONPath string of the query.
    pub(crate) query: String,
    /// Results expected to be produced by the query.
    pub(crate) results: Results,
}

/// Expected results of a query.
#[derive(Deserialize, Serialize, Clone)]
pub(crate) struct Results {
    /// Expected number of matches.
    pub(crate) count: u64,
    /// Expected indices to match. May be omitted if the count is large.
    pub(crate) bytes: Option<Vec<usize>>,
    /// Expected nodes to match. May be omitted if the count is large.
    pub(crate) nodes: Option<Vec<String>>,
}

/// Serialize a [`Document`] to [`String`].
pub(crate) fn serialize(doc: &Document) -> String {
    toml::to_string(doc).expect("generated toml must be valid")
}

/// Deserialize a [`Document`] from a [`str`]-like.
///
/// # Errors
/// Additional validation to ensure test quality are performed and may fail.
pub(crate) fn deserialize<S: AsRef<str>>(contents: S) -> Result<Document, ConfigurationError> {
    let doc = toml::from_str(contents.as_ref()).map_err(ConfigurationError::DeserializationError)?;
    validate(&doc).map_err(ConfigurationError::ValidationError)?;

    Ok(doc)
}

#[derive(Debug)]
pub(crate) enum ConfigurationError {
    DeserializationError(toml::de::Error),
    ValidationError(ValidationError),
}

#[derive(Debug)]
pub(crate) struct ValidationError {
    msg: String,
}

impl Display for ConfigurationError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::DeserializationError(err) => write!(f, "{err}"),
            Self::ValidationError(err) => write!(f, "{err}"),
        }
    }
}

impl Display for ValidationError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.msg)
    }
}

impl Error for ConfigurationError {
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        match self {
            Self::DeserializationError(err) => Some(err),
            Self::ValidationError(err) => Some(err),
        }
    }
}
impl Error for ValidationError {}

fn validate(doc: &Document) -> Result<(), ValidationError> {
    if doc.queries.is_empty() {
        return err(&"no queries defined for the doc");
    }

    for query in &doc.queries {
        if query.results.count <= 64 && query.results.bytes.is_none() {
            return err(&format!(
                "query {} with a small result count does not define expected bytes result; \
                             such tests are considered weak and should be updated to expect specific \
                             parts of the JSON to be matched",
                query.query
            ));
        }
    }

    Ok(())
}

fn err<S: ToString>(msg: &S) -> Result<(), ValidationError> {
    Err(ValidationError { msg: msg.to_string() })
}
