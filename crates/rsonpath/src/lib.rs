#![forbid(unsafe_code)]

use color_eyre::{Help, SectionExt};
use eyre::eyre;
use rsonpath_lib::{
    engine::error::EngineError,
    error::UnsupportedFeatureError,
    query::{
        error::{CompilerError, ParseErrorReport, ParserError},
        JsonPathQuery,
    },
};

const FEATURE_REQUEST_URL: &str = "https://github.com/V0ldek/rsonpath/issues/new?template=feature_request.md";

/// Turn a [`ParserError`] into a user-friendly eyre Report.
pub fn report_parser_error(query_string: &str, error: ParserError) -> eyre::Report {
    match error {
        ParserError::SyntaxError { report } => report_query_syntax_error(query_string, report),
        ParserError::InternalNomError { .. } => eyre::Report::new(error),
        ParserError::ArrayIndexError(_) => eyre::Report::new(error),
    }
}

/// Turn a [`CompilerError`] into a user-friendly eyre Report.
pub fn report_compiler_error(query: &JsonPathQuery, error: CompilerError) -> eyre::Report {
    match error {
        CompilerError::NotSupported(unsupported) => report_unsupported_error(unsupported),
        CompilerError::QueryTooComplex(_) => {
            let mut report = eyre::Report::new(error);
            if query
                .root()
                .iter()
                .any(|node| matches!(node, rsonpath_lib::query::JsonPathQueryNode::AnyChild(_)))
            {
                report = report.suggestion(
                    "Wildcard selectors are a common source of query complexity.\n            \
                    Consider reformulating the query using descendant selectors to replace sequences of wildcards.",
                );
            }
            add_unsupported_context(report, UnsupportedFeatureError::large_automaton_queries())
        }
    }
}

/// Turn a [`EngineError`] into a user-friendly eyre Report.
pub fn report_engine_error(error: EngineError) -> eyre::Report {
    match error {
        EngineError::DepthBelowZero(_, _) => eyre::Report::new(error),
        EngineError::DepthAboveLimit(_, _) => {
            add_unsupported_context(eyre::Report::new(error), UnsupportedFeatureError::large_json_depths())
        }
        EngineError::MissingClosingCharacter() => eyre::Report::new(error),
        EngineError::MalformedStringQuotes(_) => eyre::Report::new(error),
        EngineError::NotSupported(unsupported) => report_unsupported_error(unsupported),
        EngineError::InternalError(_) => eyre::Report::new(error),
    }
}

fn report_query_syntax_error(query_string: &str, report: ParseErrorReport) -> eyre::Report {
    let mut eyre = eyre!("One or more syntax errors occurred.");

    for error in report.errors() {
        use color_eyre::owo_colors::OwoColorize;
        use std::{cmp, iter};
        const MAX_DISPLAY_LENGTH: usize = 80;

        let display_start_idx = if error.start_idx > MAX_DISPLAY_LENGTH {
            error.start_idx - MAX_DISPLAY_LENGTH
        } else {
            0
        };
        let display_length = cmp::min(error.len + MAX_DISPLAY_LENGTH, query_string.len() - display_start_idx);
        let error_slice = &query_string[error.start_idx..error.start_idx + error.len];
        let slice = &query_string[display_start_idx..display_start_idx + display_length];
        let error_idx = error.start_idx - display_start_idx;

        let underline: String = iter::repeat(' ')
            .take(error_idx)
            .chain(iter::repeat('^').take(error.len))
            .collect();
        let display_string = format!("{}\n{}", slice, (underline + " invalid tokens").bright_red());

        eyre = eyre.section(display_string.header("Parse error:"));

        if error.start_idx == 0 {
            eyre = eyre.suggestion(format!(
                "Queries should start with the root selector '{}'.",
                "$".dimmed()
            ));
        }

        if error_slice.contains('$') {
            eyre = eyre.suggestion(format!(
                "The '{}' character is reserved for the root selector and may appear only at the start.",
                "$".dimmed()
            ));
        }
    }

    eyre
}

fn report_unsupported_error(unsupported: UnsupportedFeatureError) -> eyre::Report {
    use color_eyre::owo_colors::OwoColorize;
    let feature = unsupported.feature();
    let base_report = if unsupported.is_planned() {
        let feature = feature.blue();
        eyre!("The feature {feature} {}", "is not supported yet.".bright_red())
    } else {
        let feature = feature.red();
        eyre!("The feature {feature} {}", "is not supported.".bright_red())
    };
    add_unsupported_context(base_report, unsupported)
}

fn add_unsupported_context(report: eyre::Report, unsupported: UnsupportedFeatureError) -> eyre::Report {
    use color_eyre::owo_colors::OwoColorize;
    let feature = unsupported.feature();
    if let Some(issue) = unsupported.issue() {
        let feature = feature.blue();
        report.note(format!(
            "The feature {feature} is planned for a future release of rsonpath.\n      \
            You can join the ongoing discussion at {}.",
            format!("https://github.com/V0ldek/rsonpath/issues/{issue}").bright_blue()
        ))
    } else {
        let feature = feature.red();
        report.note(format!(
            "The feature {feature} is not supported and is {} planned.\n      \
            If you would like to see it introduced to rsonpath, please raise a feature request at\n      \
            {} and describe your use case.",
            "not".italic(),
            FEATURE_REQUEST_URL.bright_blue()
        ))
    }
}
