//! Type definitions and serde support for the TOML configuration files.
use std::{cmp, error::Error, fmt::Display, path::PathBuf};

use quote::TokenStreamExt;
use serde::{Deserialize, Serialize};

/// Top-level test configuration.
#[derive(Debug, Deserialize, Serialize, Clone)]
pub(crate) struct Document {
    /// Input JSON for tests.
    pub(crate) input: Input,
    /// Query cases to run on the input.
    pub(crate) queries: Vec<Query>,
}

/// Configuration of the input JSON document.
#[derive(Debug, Deserialize, Serialize, Clone)]
pub(crate) struct Input {
    /// Human-friendly description of the structure of the JSON.
    pub(crate) description: String,
    /// Whether the JSON is already minified. If true, no compressed counterpart is automatically created.
    pub(crate) is_compressed: bool,
    /// Source of the JSON.
    pub(crate) source: InputSource,
}

/// Source of the input JSON, which can be either an inline string or a path to a file.
#[derive(Debug, Deserialize, Serialize, Clone)]
pub(crate) enum InputSource {
    /// Source from a file with the given path relative to the JSON output directory.
    #[serde(rename = "large_file")]
    LargeFile(PathBuf),
    /// Source from the inline string.
    #[serde(rename = "json_string")]
    JsonString(String),
}

/// Single query defined for the test document.
#[derive(Debug, Deserialize, Serialize, Clone)]
pub(crate) struct Query {
    /// Human-friendly description of what the query does.
    pub(crate) description: String,
    /// JSONPath string of the query.
    pub(crate) query: String,
    /// Results expected to be produced by the query.
    pub(crate) results: Results,
    /// Details about a disabled query. Omitted if query is not disabled.
    pub(crate) disabled: Option<Disabled>,
}

/// Expected results of a query.
#[derive(Debug, Deserialize, Serialize, Clone)]
pub(crate) struct Results {
    /// Expected number of matches.
    pub(crate) count: u64,
    /// Expected spans to match. May be omitted if the count is large.
    pub(crate) spans: Option<Vec<ResultSpan>>,
    /// Expected nodes to match. May be omitted if the count is large.
    pub(crate) nodes: Option<Vec<String>>,
}

/// Expected matched span of a single result.
#[derive(Debug, Clone, Copy)]
pub(crate) struct ResultSpan {
    /// Index of the first byte of the match.
    pub(crate) start: usize,
    /// Index of the one-past-last byte of the match.
    pub(crate) end: usize,
}

/// Allowed bounds of an approximate span of a single result.
#[derive(Debug, Clone, Copy)]
pub(crate) struct ResultApproximateSpan {
    /// Index of the first byte of the match.
    pub(crate) start: usize,
    /// Index of the last byte of the match.
    pub(crate) end_lower_bound: usize,
    /// Index of the first non-whitespace after the match.
    pub(crate) end_upper_bound: Option<usize>,
}

/// Details about a disabled query.
#[derive(Debug, Deserialize, Serialize, Clone)]
pub(crate) struct Disabled {
    /// Link to the GitHub issue whose resolution would fix the query.
    pub(crate) issue: String,
    /// Descriptive reason for the current rsonpath limitation.
    pub(crate) reason: String,
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

impl Serialize for ResultSpan {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        (self.start, self.end).serialize(serializer)
    }
}

impl<'de> Deserialize<'de> for ResultSpan {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let raw = <(usize, usize) as Deserialize>::deserialize(deserializer)?;
        Ok(Self {
            start: raw.0,
            end: raw.1,
        })
    }
}

impl quote::ToTokens for ResultApproximateSpan {
    fn to_tokens(&self, tokens: &mut proc_macro2::TokenStream) {
        use proc_macro2::{Ident, Punct, Spacing, Span};
        tokens.append(Punct::new('(', Spacing::Alone));
        self.start.to_tokens(tokens);
        tokens.append(Punct::new(',', Spacing::Alone));
        self.end_lower_bound.to_tokens(tokens);
        tokens.append(Punct::new(',', Spacing::Alone));

        match self.end_upper_bound {
            Some(x) => {
                tokens.append(Ident::new("Some", Span::call_site()));
                tokens.append(Punct::new('(', Spacing::Alone));
                x.to_tokens(tokens);
                tokens.append(Punct::new(')', Spacing::Alone));
            }
            None => tokens.append(Ident::new("None", Span::call_site())),
        }

        tokens.append(Punct::new(')', Spacing::Alone));
    }
}

impl quote::ToTokens for ResultSpan {
    fn to_tokens(&self, tokens: &mut proc_macro2::TokenStream) {
        use proc_macro2::{Punct, Spacing};
        tokens.append(Punct::new('(', Spacing::Alone));
        self.start.to_tokens(tokens);
        tokens.append(Punct::new(',', Spacing::Alone));
        self.end.to_tokens(tokens);
        tokens.append(Punct::new(')', Spacing::Alone));
    }
}

#[derive(Debug)]
pub(crate) enum ConfigurationError {
    DeserializationError(toml::de::Error),
    ValidationError(ValidationError),
}

#[derive(Debug)]
pub(crate) struct ValidationError {
    msgs: Vec<String>,
}

impl ValidationError {
    fn new() -> Self {
        Self { msgs: vec![] }
    }

    fn add(&mut self, err: String) {
        self.msgs.push(err);
    }

    fn is_empty(&self) -> bool {
        self.msgs.is_empty()
    }

    fn into_result(self) -> Result<(), Self> {
        if self.is_empty() {
            Ok(())
        } else {
            Err(self)
        }
    }
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
        for msg in &self.msgs {
            writeln!(f, "- {msg}")?
        }

        Ok(())
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
    let mut err = ValidationError::new();

    if doc.queries.is_empty() {
        err.add("no queries defined".to_owned());
        return Err(err);
    }

    let input = match &doc.input.source {
        InputSource::LargeFile(_) => None,
        InputSource::JsonString(c) => Some(c.clone()),
    };

    for query in &doc.queries {
        if let Some(spans) = query.results.spans.as_ref() {
            if spans.len() as u64 != query.results.count {
                err.add(format!(
                    "query {} defines a different number of expected spans ({}) than the result count ({})",
                    query.query,
                    spans.len(),
                    query.results.count
                ))
            }

            let mut max_start = 0;
            let mut max_end = 0;

            for span in spans {
                if span.start > span.end {
                    err.add(format!(
                        "query {} defines impossible expected spans, [{}, {}] has negative size",
                        query.query, span.start, span.end
                    ))
                }
                if span.start < max_start {
                    err.add(format!(
                        "query {} defines impossible expected spans, [{}, {}] starts before its predecessor",
                        query.query, span.start, span.end
                    ))
                }
                if span.start < max_end && span.end >= max_end {
                    err.add(format!(
                        "query {} defines impossible expected spans, [{}, {}] ends after the span it is nested in",
                        query.query, span.start, span.end
                    ))
                }
                max_start = cmp::max(span.start, max_start);
                max_end = cmp::max(span.end, max_end);
            }
        } else if query.results.count <= 64 {
            err.add(format!(
                "query {} with a small result count does not define expected bytes result; \
                             such tests are considered weak and should be updated to expect specific \
                             parts of the JSON to be matched",
                query.query
            ));
        }

        if let Some(nodes) = query.results.nodes.as_ref() {
            if nodes.len() as u64 != query.results.count {
                err.add(format!(
                    "query {} defines a different number of expected nodes ({}) than the result count ({})",
                    query.query,
                    nodes.len(),
                    query.results.count
                ))
            }

            if let Some(spans) = query.results.spans.as_ref() {
                assert_eq!(nodes.len(), spans.len()); // They are both equal to results.count.

                for i in 0..spans.len() {
                    let span = spans[i];
                    let node = &nodes[i];

                    if span.start + node.len() != span.end {
                        err.add(format!(
                            "query {} defines a span [{}, {}] which does not match the length of the corresponding node (expected {})",
                            query.query, span.start, span.end, node.len()
                        ))
                    }

                    if let Some(contents) = input.as_ref() {
                        if span.start <= span.end {
                            // This error is checked before, we don't want to panic here.
                            let actual_value = &contents[span.start..span.end];

                            if actual_value != node {
                                err.add(format!(
                                    "query {} defines a span [{}, {}] which does not select the corresponding node (expected '{}', selects '{}' instead)",
                                    query.query, span.start, span.end, node, actual_value
                                ))
                            }
                        }
                    }
                }
            }
        }
    }

    err.into_result()
}
