//! Common errors shared across the library.
use std::fmt::{self, Display};
use thiserror::Error;

pub(crate) const FEATURE_REQUEST_URL: &str =
    "https://github.com/V0ldek/rsonpath/issues/new?template=feature_request.md";
pub(crate) const BUG_REPORT_URL: &str = "https://github.com/V0ldek/rsonpath/issues/new?template=bug_report.md";

/// Internal irrecoverable error. These are caused solely
/// by bugs &ndash; broken invariants or assertions in internal logic &ndash;
/// but we return those instead of panicking.
#[derive(Error, Debug)]
pub struct InternalRsonpathError {
    details: &'static str,
    #[source]
    source: Option<InternalErrorSource>,
}

struct InternalErrorSource(Box<dyn std::error::Error + Send + Sync>);

impl fmt::Debug for InternalErrorSource {
    #[inline(always)]
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        fmt::Debug::fmt(&self.0, f)
    }
}

impl Display for InternalErrorSource {
    #[inline(always)]
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        fmt::Display::fmt(&self.0, f)
    }
}

impl std::error::Error for InternalErrorSource {
    #[inline(always)]
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        self.0.source()
    }
}

impl InternalRsonpathError {
    #[allow(unused)]
    pub(crate) fn from_expectation(details: &'static str) -> Self {
        Self { details, source: None }
    }

    #[allow(unused)]
    pub(crate) fn from_error<E: std::error::Error + Send + Sync + 'static>(err: E, details: &'static str) -> Self {
        Self {
            details,
            source: Some(InternalErrorSource(Box::new(err))),
        }
    }
}

impl Display for InternalRsonpathError {
    #[inline]
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let details_are_enabled = std::env::var("RUST_BACKTRACE").unwrap_or_default() == "0";
        write!(
            f,
            "an internal error has occurred; this is a bug, please report it at {BUG_REPORT_URL}"
        )?;

        if details_are_enabled {
            writeln!(f, "; the error details follow")?;
            write!(f, "{}", self.details)?;
            if let Some(source) = &self.source {
                write!(f, "; source: {}", source)?;
            }
        }

        Ok(())
    }
}

/// Error raised when rsonpath is asked to perform an operation that is currently
/// unsupported. This may be either because the feature is in the works, or
/// because it is not planned to ever be supported.
#[derive(Error, Debug)]
pub struct UnsupportedFeatureError {
    issue: Option<usize>,
    feature: &'static str,
}

impl UnsupportedFeatureError {
    #[must_use]
    #[allow(dead_code)]
    #[inline(always)]
    fn tracked(issue: usize, feature: &'static str) -> Self {
        Self {
            issue: Some(issue),
            feature,
        }
    }

    #[must_use]
    #[inline(always)]
    fn untracked(feature: &'static str) -> Self {
        Self { issue: None, feature }
    }

    /// Large JSON Depths feature &ndash; supporting JSON documents
    /// with nesting depth exceeding 255. Unsupported and not planned.
    #[must_use]
    #[inline(always)]
    pub fn large_json_depths() -> Self {
        Self::untracked("Large JSON Depths")
    }

    /// Large Automaton Queries feature &ndash; supporting queries that
    /// cause compiled DFAs to exceed 256 states. Unsupported and not planned.
    #[must_use]
    #[inline(always)]
    pub fn large_automaton_queries() -> Self {
        Self::untracked("Large Automaton Queries")
    }

    /// Temporary error for index functionality to be implemented in engine.
    #[must_use]
    #[inline(always)]
    pub fn array_index() -> Self {
        Self::tracked(64, "Array Index")
    }

    /// Returns the issue number on GitHub corresponding to the unsupported feature.
    /// Is [`None`] if the feature is not planned.
    #[must_use]
    #[inline(always)]
    pub fn issue(&self) -> Option<usize> {
        self.issue
    }

    /// Returns the descriptive name of the feature.
    #[must_use]
    #[inline(always)]
    pub fn feature(&self) -> &str {
        self.feature
    }

    /// Whether the issue is planned to ever be supported.
    #[must_use]
    #[inline(always)]
    pub fn is_planned(&self) -> bool {
        self.issue.is_some()
    }
}

impl Display for UnsupportedFeatureError {
    #[inline]
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self.issue {
            Some(issue) => {
                write!(
                    f,
                    "the feature {} (#{}) is not supported yet; it is being tracked and discussed at \
                    https://github.com/V0ldek/rsonpath/issues/{}",
                    self.feature, issue, issue
                )
            }
            None => {
                write!(
                    f,
                    "the feature {} is not supported, and is not planned; \
                    if you would like to see it introduced to rsonpath, please raise a feature request at \
                    {FEATURE_REQUEST_URL}",
                    self.feature
                )
            }
        }
    }
}
