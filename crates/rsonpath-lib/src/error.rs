//! Common errors shared across the library.
use std::fmt::Display;
use thiserror::Error;

pub(crate) const FEATURE_REQUEST_URL: &str =
    "https://github.com/V0ldek/rsonpath/issues/new?template=feature_request.md";
pub(crate) const BUG_REPORT_URL: &str =
    "https://github.com/V0ldek/rsonpath/issues/new?template=bug_report.md";

/// Error raised when rsonpath is asked to perform an operation that is currently
/// unsupported. This may be either because the feature is in the works, or
/// because it is not planned to ever be supported.
#[derive(Error, Debug)]
pub struct UnsupportedFeatureError {
    issue: Option<usize>,
    feature: &'static str,
}

impl UnsupportedFeatureError {
    #[inline(always)]
    fn tracked(issue: usize, feature: &'static str) -> Self {
        Self {
            issue: Some(issue),
            feature,
        }
    }

    #[allow(dead_code)]
    #[inline(always)]
    fn untracked(feature: &'static str) -> Self {
        Self {
            issue: None,
            feature,
        }
    }

    #[inline(always)]
    pub(crate) fn wildcard_child_selector() -> Self {
        Self::tracked(9, "Wildcard Child Selector")
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
                    "the feature {} is not supported yet, and is not planned; \
                    if you would like to see it introduced to rsonpath, please raise a feature request at \
                    {BUG_REPORT_URL}",
                    self.feature
                )
            }
        }
    }
}
