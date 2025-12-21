use super::display::tweaked_width;
use std::iter::FusedIterator;

pub(super) struct IndexedInput<'a> {
    input: &'a str,
    char_data: Vec<IndexedInputCharData>,
    is_multiline: bool,
}

#[derive(Debug, PartialEq, Eq)]
struct IndexedInputCharData {
    idx: usize,
    acc_width: usize,
    c: char,
    line: usize,
}

impl<'a> IndexedInput<'a> {
    pub(super) fn new(input: &'a str) -> Self {
        let mut is_multiline = false;
        let char_data = input
            .char_indices()
            .scan((0, 0), |(acc_width, line), (i, c)| {
                if *line > 0 {
                    is_multiline = true;
                }
                let c_width = tweaked_width(c);
                *acc_width += c_width;
                let res = IndexedInputCharData::new(i, *acc_width, c, *line);
                if c == '\n' {
                    *line += 1;
                }
                Some(res)
            })
            .collect::<Vec<_>>();

        Self {
            input,
            char_data,
            is_multiline,
        }
    }

    pub(super) fn str(&self) -> &str {
        self.input
    }

    pub(super) fn len(&self) -> usize {
        self.input.len()
    }

    pub(super) fn is_empty(&self) -> bool {
        self.input.is_empty()
    }

    pub(super) fn is_multiline(&self) -> bool {
        self.is_multiline
    }

    pub(super) fn iter_useful_chars(
        &self,
        error_byte_start: usize,
        error_byte_end: usize,
        width_limit: usize,
    ) -> impl Iterator<Item = InputChar> + '_ {
        if self.is_empty() {
            return IndexedInputIter::empty();
        }
        let error_start_idx = self
            .char_data
            .binary_search_by_key(&error_byte_start, |c| c.idx)
            .unwrap_or_else(|idx| idx - 1);
        let error_end_idx = if error_byte_end == self.len() {
            self.char_data.len() - 1
        } else {
            self.char_data
                .binary_search_by_key(&error_byte_end, |c| c.idx)
                .unwrap_or_else(|idx| idx - 1)
        };
        let target_start_width = self.char_data[error_start_idx].acc_width.saturating_sub(width_limit);
        let target_end_width = self.char_data[error_end_idx].acc_width + width_limit;
        let start_idx = match self
            .char_data
            .binary_search_by_key(&target_start_width, |d| d.acc_width)
        {
            Ok(idx) | Err(idx) => idx,
        };
        let end_idx = self
            .char_data
            .binary_search_by_key(&target_end_width, |d| d.acc_width)
            .unwrap_or_else(|idx| idx - 1);
        IndexedInputIter::new(self.char_data[start_idx..=end_idx].iter())
    }
}

impl IndexedInputCharData {
    fn new(idx: usize, acc_width: usize, c: char, line: usize) -> Self {
        Self {
            idx,
            acc_width,
            c,
            line,
        }
    }
}

struct IndexedInputIter<'a> {
    iter: Option<std::slice::Iter<'a, IndexedInputCharData>>,
}

impl<'a> IndexedInputIter<'a> {
    fn new(iter: std::slice::Iter<'a, IndexedInputCharData>) -> Self {
        Self { iter: Some(iter) }
    }

    fn empty() -> Self {
        Self { iter: None }
    }
}

impl<'a> Iterator for IndexedInputIter<'a> {
    type Item = InputChar;

    fn next(&mut self) -> Option<Self::Item> {
        self.iter.as_mut().and_then(|i| {
            i.next().map(|d| InputChar {
                char: d.c,
                idx: d.idx,
                line: d.line,
            })
        })
    }
}

impl<'a> FusedIterator for IndexedInputIter<'a> {}

#[derive(Debug, Clone, Eq, PartialEq)]
pub(super) struct InputChar {
    pub char: char,
    pub idx: usize,
    pub line: usize,
}

#[cfg(test)]
mod tests {
    use super::*;
    use pretty_assertions::assert_eq;
    use IndexedInputCharData as Data;

    #[test]
    fn empty_input() {
        let s = "";
        let input = IndexedInput::new(s);
        assert_eq!(input.str(), "");
        assert_eq!(input.len(), 0);
        assert!(input.is_empty());
        assert!(input.char_data.is_empty());
        assert!(!input.is_multiline());
        let mut iter = input.iter_useful_chars(0, 0, 80);
        assert!(iter.next().is_none());
    }

    #[test]
    fn simple_ascii_input() {
        let s = "abc123...";
        let expected_data = vec![
            Data::new(0, 1, 'a', 0),
            Data::new(1, 2, 'b', 0),
            Data::new(2, 3, 'c', 0),
            Data::new(3, 4, '1', 0),
            Data::new(4, 5, '2', 0),
            Data::new(5, 6, '3', 0),
            Data::new(6, 7, '.', 0),
            Data::new(7, 8, '.', 0),
            Data::new(8, 9, '.', 0),
        ];
        let input = IndexedInput::new(s);
        assert_eq!(input.str(), s);
        assert_eq!(input.len(), s.len());
        assert!(!input.is_empty());
        assert!(!input.is_multiline());
        assert_eq!(input.char_data, expected_data);
        let iter = input.iter_useful_chars(0, 8, 80);
        let iter_data = iter.collect::<Vec<_>>();
        let str_data = s
            .char_indices()
            .map(|(idx, char)| InputChar { idx, char, line: 0 })
            .collect::<Vec<_>>();
        assert_eq!(iter_data, str_data);
        let iter = input.iter_useful_chars(5, 6, 80);
        let iter_data = iter.collect::<Vec<_>>();
        assert_eq!(iter_data, str_data);
    }

    #[test]
    fn variable_width_input() {
        const WIDTH_TO_TEST: usize = 80;
        let base_s = "🦀."; // This string has width 3 (2 for Ferris, 1 for the period) but byte-length of 5.
        assert_eq!(base_s.len(), 5);
        let s = base_s.repeat(100); // Total width 300, byte-length 500.
        let mut expected_data = vec![];
        for i in 0..100 {
            expected_data.push(Data::new(5 * i, 3 * i + 2, '🦀', 0));
            expected_data.push(Data::new(5 * i + 4, 3 * i + 3, '.', 0));
        }
        let input = IndexedInput::new(&s);
        assert_eq!(input.str(), s);
        assert_eq!(input.len(), s.len());
        assert!(!input.is_empty());
        assert!(!input.is_multiline());
        assert_eq!(input.char_data, expected_data);

        // We select the Ferris at byte index 50.
        // There is 10 copies of base_s to the left, with a total width of 30. All of them should be included.
        // There is a period and then 89 copies of base_s to the right, with a total width of 268.
        // We are limited by 80 width, so we can select the period and up to 26 copies of base_s, leaving us with
        // one width remaining; we cannot use that width, since one Ferris has width of 2.
        // This is a total of 20 + 1 + 1 + 52 = 74 characters.
        let iter = input.iter_useful_chars(50, 53, WIDTH_TO_TEST);
        let iter_data = iter.collect::<Vec<_>>();
        let str_data = s
            .char_indices()
            .map(|(idx, char)| InputChar { idx, char, line: 0 })
            .collect::<Vec<_>>();
        assert_eq!(iter_data, str_data[..74]);

        // Now select the Ferris at byte index 445.
        // There is 89 copies of base_s to the left and 10 copies to the right.
        // The situation is symmetrical, only that we take all 10 copies to the right and 26 copies to the left.
        // Now we can also include the one remaining width for the period.
        // We need to skip the first 89 - 26 = 63 groups without skipping the last period,
        // skipping 2 * 63 - 1 = 125 characters.
        let iter = input.iter_useful_chars(445, 448, WIDTH_TO_TEST);
        let iter_data = iter.collect::<Vec<_>>();
        let str_data = s
            .char_indices()
            .map(|(idx, char)| InputChar { idx, char, line: 0 })
            .collect::<Vec<_>>();
        assert_eq!(iter_data, str_data[125..]);

        // Now we select Ferris at byte index 270.
        // There is 54 copies of base_s to the left and 45 to the right.
        // We can include 26 copies to the left and a period, and 26 copies to the right.
        // We thus skip 28 groups to the left (minus the period) for a total of 2 * 28 - 1 = 55 characters;
        // take the period, 26 groups, and target Ferris, coming up to 1 + 2 * 26 + 1 = 54 characters;
        // and to the right we take the period and up to 26 copies of base_s for a total of 53 characters;
        // this produces the range 55..(55 + 54 + 53 = 162).
        let iter = input.iter_useful_chars(270, 273, WIDTH_TO_TEST);
        let iter_data = iter.collect::<Vec<_>>();
        let str_data = s
            .char_indices()
            .map(|(idx, char)| InputChar { idx, char, line: 0 })
            .collect::<Vec<_>>();
        assert_eq!(iter_data, str_data[55..162]);
    }
}
