use super::display::UnicodeWidth as _;

#[derive(Debug, PartialEq, Eq)]
pub(crate) struct SyntaxErrorLine {
    pub(crate) line: String,
    pub(crate) line_number: usize,
    pub(crate) underline: Option<SyntaxErrorUnderline>,
    pub(crate) truncated_start: bool,
    pub(crate) truncated_end: bool,
}

#[derive(Debug, PartialEq, Eq)]
pub(crate) struct SyntaxErrorUnderline {
    pub(crate) start_pos: usize,
    pub(crate) len: usize,
    pub(crate) message: Option<String>,
}

pub(super) struct ErrorFormatter<'a> {
    input: &'a str,
    line_data: Vec<LineData>,
}

#[derive(Debug, PartialEq, Eq)]
struct LineData {
    start_idx: usize,
    one_past_end_idx: usize,
    char_data: Vec<CharData>,
}

#[derive(Debug, PartialEq, Eq)]
struct CharData {
    byte_idx: usize,
    acc_width: usize,
    c: char,
}

impl<'a> ErrorFormatter<'a> {
    pub(super) fn new(input: &'a str) -> Self {
        // Special case for empty inputs - pretend we do have a single line with a whitespace so that we can display
        // any diagnostics at all.
        if input.is_empty() {
            return Self {
                input,
                line_data: vec![LineData::new(0, 0, vec![CharData::new(0, 1, ' ')])],
            };
        }

        // Go through input lines and construct the index of all characters.
        // We keep the accumulated length in bytes to have accurate byte indices for all chars.
        let mut acc_len = 0;
        let line_data = input
            .split_inclusive('\n')
            .map(|line| {
                let start_idx = acc_len;
                let one_past_end_idx = acc_len + line.len();
                // For each line accumulate the total display width of each prefix.
                let mut char_data = line
                    .char_indices()
                    .scan(0, |acc_width, (i, c)| {
                        *acc_width += c.width();
                        Some(CharData::new(i + start_idx, *acc_width, c))
                    })
                    .collect::<Vec<_>>();
                // Insert a dummy whitespace at the end.
                // This helps for errors that sometimes display an underline one past the line end, e.g. because
                // something is missing (`$.` as an example).
                if let Some(last) = char_data.last() {
                    char_data.push(CharData::new(last.byte_idx + 1, last.acc_width + 1, ' '));
                } else {
                    char_data.push(CharData::new(start_idx + 1, 1, ' '));
                }
                acc_len += line.len();
                LineData {
                    start_idx,
                    one_past_end_idx,
                    char_data,
                }
            })
            .collect::<Vec<_>>();

        Self { input, line_data }
    }

    pub(super) fn str(&self) -> &str {
        self.input
    }

    pub(super) fn len(&self) -> usize {
        self.input.len()
    }

    pub(super) fn is_multiline(&self) -> bool {
        self.line_data.len() > 1
    }

    /// Create [`SyntaxErrorLines`](SyntaxErrorLine) for display of an error occurring between
    /// byte `error_byte_start` and `error_byte_end` (inclusive).
    ///
    /// The amount of context displayed for the error is controlled by `min_context_width` and `soft_width_limit`.
    /// First ensures that some amount of pre- and post-context is always displayed. Second limits the total width
    /// of every line; it is a soft limit, because the error is always displayed in full alongside 2 times
    /// `min_context_width` context.
    pub(super) fn build_error_lines(
        &self,
        error_byte_start: usize,
        error_byte_end: usize,
        min_context_width: usize,
        soft_width_limit: usize,
        underline_message: String,
    ) -> Vec<SyntaxErrorLine> {
        let start_line_num = self.find_line_containing(error_byte_start);
        let end_line_num = self.find_line_containing(error_byte_end);
        let start_line = &self.line_data[start_line_num];
        let end_line = &self.line_data[end_line_num];

        let error_start_char_idx = start_line.find_char_at_idx(error_byte_start);
        let error_end_char_idx = end_line.find_char_at_idx(error_byte_end);

        let full_pre_context_width = if error_start_char_idx == 0 {
            0
        } else {
            start_line.width_to_char(error_start_char_idx - 1)
        };
        let full_post_context_width = if error_end_char_idx == end_line.char_data.len() - 1 {
            0
        } else {
            end_line.width_from_char(error_end_char_idx + 1)
        };

        // There are two cases - either the error is fully contained within a single line or it spans multiple lines.
        // In the first case we need to balance the width of both contexts.
        // In the second case the pre-context is fully within the first line and post-context fully within the last
        // line, so calculations are independent. The lines in between are always fully displayed.
        if start_line_num == end_line_num {
            let only_line = start_line; // Just rename to avoid confusion.
            let line_error_width = only_line.width_of_char_span(error_start_char_idx, error_end_char_idx);
            let total_width = only_line.total_width();
            let (pre_width, post_width) = if total_width <= soft_width_limit {
                // All context fits, no truncating needed.
                (full_pre_context_width, full_post_context_width)
            } else {
                // Need to truncate at least one side. Try to do it in a balanced manner.
                // First, allocate half of the total allowed width to each side.
                let allowed_total_context_width = soft_width_limit.saturating_sub(line_error_width);
                let pre_allocation = allowed_total_context_width / 2;
                let post_allocation = allowed_total_context_width - pre_allocation;
                // This allocation might be too much if the full width is actually shorter.
                let pre_overallocation = pre_allocation.saturating_sub(full_pre_context_width);
                let post_overallocation = post_allocation.saturating_sub(full_post_context_width);
                // Give back the overallocation to the other side and enforce the min width requirement.
                let pre_width = (pre_allocation + post_overallocation)
                    .max(min_context_width)
                    .min(full_pre_context_width);
                let post_width = (post_allocation + pre_overallocation)
                    .max(min_context_width)
                    .min(full_post_context_width);
                (pre_width, post_width)
            };
            let pre_start_idx = only_line.find_start_of_pre_context(error_start_char_idx, pre_width);
            // Get the width from pre-context start to the error start. This is the effective offset of the underline.
            let underline_offset = if error_start_char_idx == 0 {
                0
            } else {
                only_line.width_of_char_span(pre_start_idx, error_start_char_idx - 1)
            };
            let post_end_idx = only_line.find_end_of_post_context(error_end_char_idx, post_width);
            let display_line = self.slice_line(only_line, pre_start_idx, post_end_idx).to_string();
            vec![SyntaxErrorLine {
                truncated_start: full_pre_context_width != pre_width,
                truncated_end: full_post_context_width != post_width,
                line: display_line,
                line_number: start_line_num,
                underline: if line_error_width == 0 {
                    None
                } else {
                    Some(SyntaxErrorUnderline {
                        len: line_error_width,
                        start_pos: underline_offset,
                        message: Some(underline_message),
                    })
                },
            }]
        } else {
            // Calculate the allowed width of pre-context and post-context independently on each of their lines.
            let first_line_error_width = start_line.width_from_char(error_start_char_idx);
            let last_line_error_width = end_line.width_to_char(error_end_char_idx);
            let pre_width = soft_width_limit
                .saturating_sub(first_line_error_width)
                .max(min_context_width)
                .min(full_pre_context_width);
            let post_width = soft_width_limit
                .saturating_sub(last_line_error_width)
                .max(min_context_width)
                .min(full_post_context_width);
            let pre_start_idx = start_line.find_start_of_pre_context(error_start_char_idx, pre_width);
            let post_end_idx = end_line.find_end_of_post_context(error_end_char_idx, post_width);
            // There are three types of lines - the first line, middle lines, and the last line.
            let mut lines = Vec::with_capacity(end_line_num - start_line_num + 1);
            let first_line_display =
                self.input[start_line.char_data[pre_start_idx].byte_idx..start_line.one_past_end_idx].to_string();
            lines.push(SyntaxErrorLine {
                truncated_start: full_pre_context_width != pre_width,
                truncated_end: false,
                line: first_line_display,
                line_number: start_line_num,
                underline: Some(SyntaxErrorUnderline {
                    len: start_line.width_from_char(error_start_char_idx),
                    start_pos: pre_width,
                    message: None,
                }),
            });
            for line_num in start_line_num + 1..end_line_num {
                let line = &self.line_data[line_num];
                let display_line = &self.input[line.start_idx..line.one_past_end_idx];
                lines.push(SyntaxErrorLine {
                    truncated_start: false,
                    truncated_end: false,
                    line: display_line.to_string(),
                    line_number: line_num,
                    underline: Some(SyntaxErrorUnderline {
                        len: line.total_width(),
                        start_pos: 0,
                        message: None,
                    }),
                });
            }
            let display_line = self.slice_line(end_line, 0, post_end_idx).to_string();
            lines.push(SyntaxErrorLine {
                truncated_start: false,
                truncated_end: full_post_context_width != post_width,
                line: display_line,
                line_number: end_line_num,
                underline: Some(SyntaxErrorUnderline {
                    len: end_line.width_to_char(error_end_char_idx),
                    start_pos: 0,
                    message: Some(underline_message),
                }),
            });
            lines
        }
    }

    /// Returns the line number which contains the given byte index.
    fn find_line_containing(&self, idx: usize) -> usize {
        self.line_data
            .binary_search_by_key(&idx, |l| l.start_idx)
            .unwrap_or_else(|idx| idx - 1)
    }

    /// Get the input slice for a given line between two *char* indices (inclusive).
    fn slice_line(&self, line: &LineData, start_char_idx: usize, end_char_idx: usize) -> &str {
        let start = line.char_data[start_char_idx].byte_idx;
        // This looks overcomplicated but is correct. Because the char at end_char_idx might have byte-width
        // greater than one we need to ask the next character, if any, for its index. If it is the last char,
        // then it is the dummy whitespace we inserted at the end that we don't want to display anyway.
        let end = if end_char_idx == line.char_data.len() - 1 {
            line.char_data[end_char_idx].byte_idx
        } else {
            line.char_data[end_char_idx + 1].byte_idx
        };
        &self.input[start..end]
    }
}

impl CharData {
    fn new(idx: usize, acc_width: usize, c: char) -> Self {
        Self {
            byte_idx: idx,
            acc_width,
            c,
        }
    }
}

impl LineData {
    fn new(start_idx: usize, one_past_end_idx: usize, char_data: Vec<CharData>) -> Self {
        Self {
            start_idx,
            one_past_end_idx,
            char_data,
        }
    }

    /// Get the char index of the char that contains the byte index.
    fn find_char_at_idx(&self, byte_idx: usize) -> usize {
        self.char_data
            .binary_search_by_key(&byte_idx, |c| c.byte_idx)
            .unwrap_or_else(|idx| idx - 1)
    }

    /// Get the char index at which pre-context should start if the error starts at the given
    /// index, and we are limited by a maximum `pre_width`.
    fn find_start_of_pre_context(&self, error_start_char_idx: usize, pre_width: usize) -> usize {
        let width_at_error_start = if error_start_char_idx == 0 {
            0
        } else {
            self.width_to_char(error_start_char_idx - 1)
        };
        let target_width = width_at_error_start.saturating_sub(pre_width);
        self.char_data
            .binary_search_by_key(&target_width, |d| d.acc_width - d.c.width())
            .unwrap_or_else(|idx| idx)
    }

    /// Get the char index at which post-context should end if the error ends at the given index,
    /// and we are limited by a maximum `post_width`.
    fn find_end_of_post_context(&self, error_end_char_idx: usize, post_width: usize) -> usize {
        let width_at_error_end = self.width_to_char(error_end_char_idx);
        let target_width = width_at_error_end + post_width;
        self.char_data
            .binary_search_by_key(&target_width, |c| c.acc_width)
            .unwrap_or_else(|idx| idx - 1)
    }

    /// Get the total width of characters between the two char indices (inclusive).
    fn width_of_char_span(&self, start_char_idx: usize, end_char_idx: usize) -> usize {
        self.char_data[end_char_idx].acc_width - self.char_data[start_char_idx].acc_width
            + self.char_data[start_char_idx].c.width()
    }

    /// Get the total width of the line suffix starting at the given char index.
    fn width_from_char(&self, start_char_idx: usize) -> usize {
        self.total_width() + self.char_data[start_char_idx].c.width() - self.char_data[start_char_idx].acc_width
    }

    /// Get the total width of the line prefix ending at the given char index (inclusive).
    fn width_to_char(&self, end_char_idx: usize) -> usize {
        self.char_data[end_char_idx].acc_width
    }

    /// Get the total width of all characters in this line.
    fn total_width(&self) -> usize {
        // Subtract one for the dummy whitespace we inserted.
        self.char_data.last().map_or(0, |c| c.acc_width) - 1
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use pretty_assertions::assert_eq;

    #[test]
    fn empty_input() {
        let s = "";
        let expected_lines = vec![SyntaxErrorLine {
            line: String::new(),
            truncated_start: false,
            truncated_end: false,
            line_number: 0,
            underline: Some(SyntaxErrorUnderline {
                start_pos: 0,
                len: 1,
                message: Some("message".to_string()),
            }),
        }];
        let input = ErrorFormatter::new(s);
        assert_eq!(input.str(), "");
        assert_eq!(input.len(), 0);
        assert!(!input.is_multiline());
        let lines = input.build_error_lines(0, 0, 30, 80, "message".to_string());
        assert_eq!(expected_lines, lines);
    }

    #[test]
    fn simple_ascii_input() {
        let s = "abc123...";
        let expected_char_data = vec![
            CharData::new(0, 1, 'a'),
            CharData::new(1, 2, 'b'),
            CharData::new(2, 3, 'c'),
            CharData::new(3, 4, '1'),
            CharData::new(4, 5, '2'),
            CharData::new(5, 6, '3'),
            CharData::new(6, 7, '.'),
            CharData::new(7, 8, '.'),
            CharData::new(8, 9, '.'),
            CharData::new(9, 10, ' '),
        ];
        let expected_line_data = vec![LineData::new(0, s.len(), expected_char_data)];
        let expected_error_data_1 = vec![SyntaxErrorLine {
            truncated_start: false,
            truncated_end: false,
            line: s.to_string(),
            underline: Some(SyntaxErrorUnderline {
                len: 9,
                start_pos: 0,
                message: Some("message".to_string()),
            }),
            line_number: 0,
        }];
        let expected_error_data_2 = vec![SyntaxErrorLine {
            truncated_start: false,
            truncated_end: false,
            line: s.to_string(),
            underline: Some(SyntaxErrorUnderline {
                len: 2,
                start_pos: 5,
                message: Some("message".to_string()),
            }),
            line_number: 0,
        }];
        let input = ErrorFormatter::new(s);
        assert_eq!(input.str(), s);
        assert_eq!(input.len(), s.len());
        assert!(!input.is_multiline());
        assert_eq!(input.line_data, expected_line_data);
        let data = input.build_error_lines(0, 8, 30, 80, "message".to_string());
        assert_eq!(expected_error_data_1, data);
        let data = input.build_error_lines(5, 6, 30, 80, "message".to_string());
        assert_eq!(expected_error_data_2, data);
    }

    #[test]
    fn variable_width_input() {
        const WIDTH_TO_TEST: usize = 80;
        let base_s = "🦀."; // This string has width 3 (2 for Ferris, 1 for the period) but byte-length of 5.
        assert_eq!(base_s.len(), 5);
        let s = base_s.repeat(100); // Total width 300, byte-length 500.
        let mut expected_char_data = vec![];
        for i in 0..100 {
            expected_char_data.push(CharData::new(5 * i, 3 * i + 2, '🦀'));
            expected_char_data.push(CharData::new(5 * i + 4, 3 * i + 3, '.'));
        }
        expected_char_data.push(CharData::new(500, 301, ' '));
        let expected_data = vec![LineData::new(0, s.len(), expected_char_data)];
        let input = ErrorFormatter::new(&s);
        assert_eq!(input.str(), s);
        assert_eq!(input.len(), s.len());
        assert!(!input.is_multiline());
        assert_eq!(input.line_data, expected_data);

        // We select the Ferris at byte index 50.
        // There is 10 copies of base_s to the left, with a total width of 30. All of them should be included.
        // Ferris has width 2 so we are left with 48 width for the post-context.
        // There is a period and then 89 copies of base_s to the right, with a total width of 268.
        // We are limited by 48 width, so we can select the period and up to 15 copies of base_s, leaving us with
        // two width remaining; we can use that width to select the next Ferris.
        // This is a total of 20 + 1 + 1 + 30 + 1 = 53 characters.
        let iter = input.build_error_lines(50, 53, 5, WIDTH_TO_TEST, "message".to_string());
        let expected_lines = vec![SyntaxErrorLine {
            truncated_start: false,
            truncated_end: true,
            line_number: 0,
            line: s.chars().take(53).collect::<String>(),
            underline: Some(SyntaxErrorUnderline {
                len: 2,
                start_pos: 30,
                message: Some("message".to_string()),
            }),
        }];
        assert_eq!(expected_lines, iter);

        // Now select the Ferris at byte index 445.
        // There is 89 copies of base_s to the left and 10 copies to the right.
        // The situation is symmetrical, only that we take the period and all 10 copies to the right
        // and 15 copies to the left.
        // We spent 45 + 2 + 31 = 78 width and we have 2 remaining.
        // Now we can spend one of the 2 remaining width for the period to the left, but we cannot
        // include another Ferris.
        // We need to skip the first 89 - 15 = 74 groups without skipping the last period,
        // skipping 2 * 74 - 1 = 147 characters.
        let iter = input.build_error_lines(445, 448, 5, WIDTH_TO_TEST, "message".to_string());
        let expected_lines = vec![SyntaxErrorLine {
            truncated_start: true,
            truncated_end: false,
            line_number: 0,
            line: s.chars().skip(147).collect::<String>(),
            underline: Some(SyntaxErrorUnderline {
                len: 2,
                start_pos: 46,
                message: Some("message".to_string()),
            }),
        }];
        assert_eq!(expected_lines, iter);

        // Now we select Ferris at byte index 270.
        // There is 54 copies of base_s to the left and 45 to the right.
        // We can include 13 copies to the left, the period, 12 copies, and then a Ferris
        // to the right.
        // We thus skip 28 groups to the left for a total of 2 * 28 = 56 characters;
        // take 13 groups and target Ferris, coming up to 2 * 13 + 1 = 27 characters;
        // and to the right we take 26 characters.
        let iter = input.build_error_lines(270, 273, 5, WIDTH_TO_TEST, "message".to_string());
        let expected_lines = vec![SyntaxErrorLine {
            truncated_start: true,
            truncated_end: true,
            line_number: 0,
            line: s.chars().skip(56).take(27 + 26).collect::<String>(),
            underline: Some(SyntaxErrorUnderline {
                len: 2,
                start_pos: 39,
                message: Some("message".to_string()),
            }),
        }];
        assert_eq!(expected_lines, iter);
    }
}
