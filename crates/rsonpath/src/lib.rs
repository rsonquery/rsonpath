use color_eyre::{Help, SectionExt};
use eyre::eyre;
use rsonpath_lib::query::error::ParseErrorReport;

pub fn report_query_syntax_error(query_string: &str, report: ParseErrorReport) -> eyre::Report {
    let mut eyre = eyre!("Could not parse JSONPath query.");

    for error in report.errors() {
        use color_eyre::owo_colors::OwoColorize;
        use std::{cmp, iter};
        const MAX_DISPLAY_LENGTH: usize = 80;

        let display_start_idx = if error.start_idx > MAX_DISPLAY_LENGTH {
            error.start_idx - MAX_DISPLAY_LENGTH
        } else {
            0
        };
        let display_length = cmp::min(
            error.len + MAX_DISPLAY_LENGTH,
            query_string.len() - display_start_idx,
        );
        let error_slice = &query_string[error.start_idx..error.start_idx + error.len];
        let slice = &query_string[display_start_idx..display_start_idx + display_length];
        let error_idx = error.start_idx - display_start_idx;

        let underline: String = iter::repeat(' ')
            .take(error_idx)
            .chain(iter::repeat('^').take(error.len))
            .collect();
        let display_string = format!(
            "{}\n{}",
            slice,
            (underline + " invalid tokens").bright_red()
        );

        eyre = eyre.section(display_string.header("Parse error:"));

        if error.start_idx == 0 {
            eyre = eyre.suggestion("Queries should start with the root selector `$`.");
        }

        if error_slice.contains('$') {
            eyre = eyre.suggestion("The `$` character is reserved for the root selector and may appear only at the start.");
        }
    }

    eyre
}
