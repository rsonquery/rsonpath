use color_eyre::{Help as _, SectionExt as _};
use eyre::eyre;
use rsonpath_lib::{automaton::error::CompilerError, engine::error::EngineError, error::UnsupportedFeatureError};
use rsonpath_syntax::{error::ParseError, JsonPathQuery};

const FEATURE_REQUEST_URL: &str = "https://github.com/V0ldek/rsonpath/issues/new?template=feature_request.md";

/// Turn a [`ParseError`] into a user-friendly eyre Report.
pub(super) fn report_parser_error(error: ParseError) -> eyre::Report {
    eyre!("One or more syntax errors occurred.").section(error.colored().header("Parse error:"))
}

/// Turn a [`CompilerError`] into a user-friendly eyre Report.
pub(super) fn report_compiler_error(query: &JsonPathQuery, error: CompilerError) -> eyre::Report {
    match error {
        CompilerError::NotSupported(unsupported) => report_unsupported_error(&unsupported),
        CompilerError::QueryTooComplex(_) => {
            let mut report = eyre::Report::new(error);
            if query.segments().iter().any(|segment| {
                segment
                    .selectors()
                    .iter()
                    .any(|selector| matches!(selector, rsonpath_syntax::Selector::Wildcard))
            }) {
                report = report.suggestion(
                    "Wildcard selectors are a common source of query complexity.\n            \
                    Consider reformulating the query using descendant selectors to replace sequences of wildcards.",
                );
            }
            add_unsupported_context(report, &UnsupportedFeatureError::large_automaton_queries())
        }
    }
}

/// Turn a [`EngineError`] into a user-friendly eyre Report.
pub(super) fn report_engine_error(error: EngineError) -> eyre::Report {
    match error {
        EngineError::DepthAboveLimit(_, _) => {
            add_unsupported_context(eyre::Report::new(error), &UnsupportedFeatureError::large_json_depths())
        }
        EngineError::NotSupported(unsupported) => report_unsupported_error(&unsupported),
        EngineError::DepthBelowZero(_, _)
        | EngineError::InputError(_)
        | EngineError::InternalError(_)
        | EngineError::MalformedStringQuotes(_)
        | EngineError::MissingClosingCharacter()
        | EngineError::MissingItem()
        | EngineError::MissingOpeningCharacter()
        | EngineError::SinkError(_) => eyre::Report::new(error),
    }
}

fn report_unsupported_error(unsupported: &UnsupportedFeatureError) -> eyre::Report {
    use color_eyre::owo_colors::OwoColorize as _;
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

fn add_unsupported_context(report: eyre::Report, unsupported: &UnsupportedFeatureError) -> eyre::Report {
    use color_eyre::owo_colors::OwoColorize as _;
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
