use pretty_assertions::assert_eq;
use rsonpath::{
    input::{error::InputError, *},
    result::empty::EmptyRecorder,
    StringPattern,
};
use std::{cmp, fs, fs::File, io::Read, iter};
use test_case::test_case;

const ROOT_TEST_DIRECTORY: &str = "../rsonpath-test/documents/json/large";
const BLOCK_SIZE: usize = 64;

macro_rules! file_test_cases {
    ($test_name:ident, $impl:expr) => {
        #[test_case("twitter.json"; "twitter_json")]
        #[test_case("compressed/twitter.json"; "compressed_twitter_json")]
        #[test_case("wikidata_person.json"; "wikidata_person_json")]
        #[test_case("wikidata_profession.json"; "wikidata_profession_json")]
        #[test_case("wikidata_properties.json"; "wikidata_properties_json")]
        fn $test_name(test_path: &str) {
            $impl.test_on_file(test_path);
        }
    };
}

file_test_cases!(buffered_input, FileTestInput::Buffered);
file_test_cases!(mmap_input, FileTestInput::Mmap);
file_test_cases!(borrowed_bytes, FileTestInput::Borrowed);

#[derive(Debug)]
enum FileTestInput {
    Buffered,
    Mmap,
    Borrowed,
}

#[derive(Debug)]
enum InMemoryTestInput {
    Buffered,
    Borrowed,
    Owned,
}

impl FileTestInput {
    fn get_file(path: &str) -> File {
        let path = format!("{ROOT_TEST_DIRECTORY}/{path}");
        let act_path = fs::canonicalize(path).unwrap();
        fs::File::open(act_path).unwrap()
    }

    fn test_on_file(&self, path: &str) {
        match self {
            FileTestInput::Buffered => Self::test_buffered(path),
            FileTestInput::Mmap => Self::test_mmap(path),
            FileTestInput::Borrowed => Self::test_borrowed(path),
        }
    }

    fn test_buffered(path: &str) {
        let mut buf = vec![];
        let mut file = Self::get_file(path);
        file.read_to_end(&mut buf).unwrap();
        drop(file);

        let file = Self::get_file(path);
        let input = BufferedInput::new(file);

        test_equivalence(&buf, input);
    }

    fn test_mmap(path: &str) {
        let mut buf = vec![];
        let mut file = Self::get_file(path);
        file.read_to_end(&mut buf).unwrap();
        drop(file);

        let file = Self::get_file(path);
        let input = unsafe { MmapInput::map_file(&file) }.unwrap();

        test_equivalence(&buf, input);
    }

    fn test_borrowed(path: &str) {
        let mut buf = vec![];
        let mut file = Self::get_file(path);
        file.read_to_end(&mut buf).unwrap();
        let input = BorrowedBytes::new(&buf);

        test_equivalence(&buf, input);
    }
}

impl InMemoryTestInput {
    fn test_padding(&self, bytes: &[u8]) {
        match self {
            InMemoryTestInput::Buffered => Self::test_padding_buffered(bytes),
            InMemoryTestInput::Borrowed => Self::test_padding_borrowed(bytes),
            InMemoryTestInput::Owned => Self::test_padding_owned(bytes),
        }
    }

    fn test_seek_forward(&self, bytes: &[u8], from: usize, needle: u8, expected: usize) {
        match self {
            InMemoryTestInput::Buffered => Self::test_seek_forward_buffered(bytes, from, needle, expected),
            InMemoryTestInput::Borrowed => Self::test_seek_forward_borrowed(bytes, from, needle, expected),
            InMemoryTestInput::Owned => Self::test_seek_forward_owned(bytes, from, needle, expected),
        }
    }

    fn test_seek_non_whitespace_forward(&self, bytes: &[u8], from: usize, expected: usize, expected_byte: u8) {
        match self {
            InMemoryTestInput::Buffered => {
                Self::test_seek_non_whitespace_forward_buffered(bytes, from, expected, expected_byte)
            }
            InMemoryTestInput::Borrowed => {
                Self::test_seek_non_whitespace_forward_borrowed(bytes, from, expected, expected_byte)
            }
            InMemoryTestInput::Owned => {
                Self::test_seek_non_whitespace_forward_owned(bytes, from, expected, expected_byte)
            }
        }
    }

    fn test_seek_backward(&self, bytes: &[u8], from: usize, needle: u8, expected: usize) {
        match self {
            InMemoryTestInput::Buffered => Self::test_seek_backward_buffered(bytes, from, needle, expected),
            InMemoryTestInput::Borrowed => Self::test_seek_backward_borrowed(bytes, from, needle, expected),
            InMemoryTestInput::Owned => Self::test_seek_backward_owned(bytes, from, needle, expected),
        }
    }

    fn test_seek_non_whitespace_backward(&self, bytes: &[u8], from: usize, expected: usize, expected_byte: u8) {
        match self {
            InMemoryTestInput::Buffered => {
                Self::test_seek_non_whitespace_backward_buffered(bytes, from, expected, expected_byte)
            }
            InMemoryTestInput::Borrowed => {
                Self::test_seek_non_whitespace_backward_borrowed(bytes, from, expected, expected_byte)
            }
            InMemoryTestInput::Owned => {
                Self::test_seek_non_whitespace_backward_owned(bytes, from, expected, expected_byte)
            }
        }
    }

    /*fn test_positive_is_member_match(&self, bytes: &[u8], from: usize, to: usize, string_pattern: StringPattern) {
        match self {
            InMemoryTestInput::Buffered => {
                Self::test_positive_is_member_match_buffered(bytes, from, to, string_pattern)
            }
            InMemoryTestInput::Borrowed => {
                Self::test_positive_is_member_match_borrowed(bytes, from, to, string_pattern)
            }
            InMemoryTestInput::Owned => Self::test_positive_is_member_match_owned(bytes, from, to, string_pattern),
        }
    }*/

    fn test_seek_forward_buffered(bytes: &[u8], from: usize, needle: u8, expected: usize) {
        let input = create_buffered(bytes);
        let result = input.seek_forward(from, [needle]).expect("seek succeeds");
        // Buffered is never padded from the start.

        assert_eq!(result, Some((expected, needle)));
    }

    fn test_seek_forward_borrowed(bytes: &[u8], from: usize, needle: u8, expected: usize) {
        let input = BorrowedBytes::new(bytes);
        let result = input.seek_forward(from, [needle]).expect("seek succeeds");
        // Need to take padding into account.
        let expected = expected + input.leading_padding_len();

        assert_eq!(result, Some((expected, needle)));
    }

    fn test_seek_forward_owned(bytes: &[u8], from: usize, needle: u8, expected: usize) {
        let input = OwnedBytes::new(bytes);
        let result = input.seek_forward(from, [needle]).expect("seek succeeds");
        // Need to take padding into account.
        let expected = expected + input.leading_padding_len();

        assert_eq!(result, Some((expected, needle)));
    }

    fn test_seek_non_whitespace_forward_buffered(bytes: &[u8], from: usize, expected: usize, expected_byte: u8) {
        let input = create_buffered(bytes);
        let result = input.seek_non_whitespace_forward(from).expect("seek succeeds");
        // Buffered is never padded from the start.

        assert_eq!(result, Some((expected, expected_byte)));
    }

    fn test_seek_non_whitespace_forward_borrowed(bytes: &[u8], from: usize, expected: usize, expected_byte: u8) {
        let input = BorrowedBytes::new(bytes);
        let result = input.seek_non_whitespace_forward(from).expect("seek succeeds");
        // Need to take padding into account.
        let expected = expected + input.leading_padding_len();

        assert_eq!(result, Some((expected, expected_byte)));
    }

    fn test_seek_non_whitespace_forward_owned(bytes: &[u8], from: usize, expected: usize, expected_byte: u8) {
        let input = OwnedBytes::new(bytes);
        let result = input.seek_non_whitespace_forward(from).expect("seek succeeds");
        // Need to take padding into account.
        let expected = expected + input.leading_padding_len();

        assert_eq!(result, Some((expected, expected_byte)));
    }

    fn test_seek_backward_buffered(bytes: &[u8], from: usize, needle: u8, expected: usize) {
        let input = create_buffered(bytes);
        // A bit of a hack, make sure we read buffered until at least `from`.
        input.seek_forward(from, [bytes[from]]).expect("forwarding succeeds");

        let result = input.seek_backward(from, needle);
        // Buffered is never padded from the start.

        assert_eq!(result, Some(expected));
    }

    fn test_seek_backward_borrowed(bytes: &[u8], from: usize, needle: u8, expected: usize) {
        let input = BorrowedBytes::new(bytes);

        // Need to take padding into account.
        let from = from + input.leading_padding_len();
        let expected = expected + input.leading_padding_len();
        let result = input.seek_backward(from, needle);

        assert_eq!(result, Some(expected));
    }

    fn test_seek_backward_owned(bytes: &[u8], from: usize, needle: u8, expected: usize) {
        let input = OwnedBytes::new(bytes);

        // Need to take padding into account.
        let from = from + input.leading_padding_len();
        let expected = expected + input.leading_padding_len();
        let result = input.seek_backward(from, needle);

        assert_eq!(result, Some(expected));
    }

    fn test_seek_non_whitespace_backward_buffered(bytes: &[u8], from: usize, expected: usize, expected_byte: u8) {
        let input = create_buffered(bytes);
        // A bit of a hack, make sure we read buffered until at least `from`.
        input.seek_forward(from, [bytes[from]]).expect("forwarding succeeds");

        let result = input.seek_non_whitespace_backward(from);
        // Buffered is never padded from the start.

        assert_eq!(result, Some((expected, expected_byte)));
    }

    fn test_seek_non_whitespace_backward_borrowed(bytes: &[u8], from: usize, expected: usize, expected_byte: u8) {
        let input = BorrowedBytes::new(bytes);

        // Need to take padding into account.
        let from = from + input.leading_padding_len();
        let expected = expected + input.leading_padding_len();
        let result = input.seek_non_whitespace_backward(from);

        assert_eq!(result, Some((expected, expected_byte)));
    }

    fn test_seek_non_whitespace_backward_owned(bytes: &[u8], from: usize, expected: usize, expected_byte: u8) {
        let input = OwnedBytes::new(bytes);

        // Need to take padding into account.
        let from = from + input.leading_padding_len();
        let expected = expected + input.leading_padding_len();
        let result = input.seek_non_whitespace_backward(from);

        assert_eq!(result, Some((expected, expected_byte)));
    }

    /*fn test_positive_is_member_match_buffered(bytes: &[u8], from: usize, to: usize, string_pattern: StringPattern) {
        let input = create_buffered(bytes);

        let result = input
            .is_string_match(from, to, &string_pattern)
            .expect("match succeeds");
        // Buffered is never padded from the start.

        assert!(result);
    }

    fn test_positive_is_member_match_borrowed(bytes: &[u8], from: usize, to: usize, string_pattern: StringPattern) {
        let input = BorrowedBytes::new(bytes);

        // Need to take padding into account.
        let from = from + input.leading_padding_len();
        let to = to + input.leading_padding_len();
        let result = input
            .is_string_match(from, to, &string_pattern)
            .expect("match succeeds");

        assert!(result);
    }

    fn test_positive_is_member_match_owned(bytes: &[u8], from: usize, to: usize, string_pattern: StringPattern) {
        let input = OwnedBytes::new(bytes);

        // Need to take padding into account.
        let from = from + input.leading_padding_len();
        let to = to + input.leading_padding_len();
        let result = input
            .is_string_match(from, to, &string_pattern)
            .expect("match succeeds");

        assert!(result);
    }*/

    fn test_padding_buffered(bytes: &[u8]) {
        let input = create_buffered(bytes);
        test_equivalence(bytes, input);
    }

    fn test_padding_borrowed(bytes: &[u8]) {
        let input = BorrowedBytes::new(bytes);
        test_equivalence(bytes, input);
    }

    fn test_padding_owned(bytes: &[u8]) {
        let input = OwnedBytes::new(bytes);
        test_equivalence(bytes, input);
    }
}

fn create_buffered(bytes: &[u8]) -> BufferedInput<ReadBytes> {
    let read = ReadBytes(bytes, 0);
    BufferedInput::new(read)
}

fn test_equivalence<I: Input>(original_contents: &[u8], input: I) {
    let original_length = original_contents.len();
    let mut input_contents = read_input_to_end(input).unwrap();

    assert_padding_is_correct(&input_contents, original_length);
    remove_padding(&mut input_contents, original_length);
    buffered_assert_eq(&input_contents.data, original_contents);
}

fn read_input_to_end<I: Input>(input: I) -> Result<ResultInput, InputError> {
    let mut result: Vec<u8> = vec![];
    let mut iter = input.iter_blocks::<_, BLOCK_SIZE>(&EmptyRecorder);

    while let Some(block) = iter.next().map_err(|x| x.into())? {
        result.extend_from_slice(&block)
    }

    Ok(ResultInput {
        data: result,
        leading_padding_len: input.leading_padding_len(),
        trailing_padding_len: input.trailing_padding_len(),
    })
}

fn assert_padding_is_correct(result: &ResultInput, original_length: usize) {
    assert_eq!(result.data.len() % BLOCK_SIZE, 0);
    assert!(
        result.data.len() >= original_length,
        "result len ({}) should be at least the original length ({})",
        result.data.len(),
        original_length
    );

    let expected_leading_padding: Vec<u8> = iter::repeat(b' ').take(result.leading_padding_len).collect();
    let expected_trailing_padding: Vec<u8> = iter::repeat(b' ').take(result.trailing_padding_len).collect();

    assert_eq!(&result.data[..result.leading_padding_len], expected_leading_padding);
    assert_eq!(
        &result.data[original_length + result.leading_padding_len..],
        expected_trailing_padding
    );
}

fn remove_padding(result: &mut ResultInput, original_length: usize) {
    // Remove the leading padding by draining leading_padding_len elems...
    result.data.drain(..result.leading_padding_len);
    // Now remove the trailing padding by truncating to the original length.
    // This works since we removed leading padding first, so the entire difference
    // between data.len() and original_length is the trailing padding.
    result.data.truncate(original_length);
}

/// Assert eq on the entire contents results in way too long outputs on failure.
/// This function compares the results in block-sized chunks, printing the diff for first mismatched chunk.
fn buffered_assert_eq(left: &[u8], right: &[u8]) {
    let mut i = 0;

    while i < left.len() || i < right.len() {
        let left_chunk = &left[i..cmp::min(i + BLOCK_SIZE, left.len())];
        let right_chunk = &right[i..cmp::min(i + BLOCK_SIZE, right.len())];

        assert_eq!(
            left_chunk, right_chunk,
            "difference at {} byte long chunk starting at index {}",
            BLOCK_SIZE, i
        );

        i += BLOCK_SIZE;
    }
}

struct ResultInput {
    data: Vec<u8>,
    leading_padding_len: usize,
    trailing_padding_len: usize,
}

struct ReadBytes<'a>(&'a [u8], usize);

impl<'a> Read for ReadBytes<'a> {
    fn read(&mut self, buf: &mut [u8]) -> std::io::Result<usize> {
        let rem = self.0.len() - self.1;
        if rem > 0 {
            let size = cmp::min(1024, rem);
            buf[..size].copy_from_slice(&self.0[self.1..self.1 + size]);
            self.1 += size;
            Ok(size)
        } else {
            Ok(0)
        }
    }
}

mod in_memory_proptests {
    use crate::InMemoryTestInput;
    use proptest::prelude::*;
    use rsonpath::StringPattern;
    use rsonpath_syntax::str::JsonString;
    const JSON_WHITESPACE_BYTES: [u8; 4] = [b' ', b'\t', b'\n', b'\r'];

    fn decode_whitespace(ws_idx: Vec<usize>) -> Vec<u8> {
        ws_idx.into_iter().map(|x| JSON_WHITESPACE_BYTES[x]).collect()
    }

    proptest! {
        #[test]
        fn buffered_input_represents_the_same_bytes_padded(input in prop::collection::vec(prop::num::u8::ANY, 0..1024)) {
            InMemoryTestInput::Buffered.test_padding(&input)
        }

        #[test]
        fn borrowed_bytes_represents_the_same_bytes_padded(input in prop::collection::vec(prop::num::u8::ANY, 0..1024)) {
            InMemoryTestInput::Borrowed.test_padding(&input)
        }

        #[test]
        fn owned_bytes_represents_the_same_bytes_padded(input in prop::collection::vec(prop::num::u8::ANY, 0..1024)) {
            InMemoryTestInput::Owned.test_padding(&input)
        }

        #[test]
        fn buffered_input_seek_forward_is_correct((input, from, expected) in seek_forward_strategy()) {
            InMemoryTestInput::Buffered.test_seek_forward(&input, from, 255, expected)
        }

        #[test]
        fn borrowed_input_seek_forward_is_correct((input, from, expected) in seek_forward_strategy()) {
            InMemoryTestInput::Borrowed.test_seek_forward(&input, from, 255, expected)
        }

        #[test]
        fn owned_input_seek_forward_is_correct((input, from, expected) in seek_forward_strategy()) {
            InMemoryTestInput::Owned.test_seek_forward(&input, from, 255, expected)
        }

        #[test]
        fn buffered_input_seek_non_whitespace_forward_is_correct((input, from, expected) in seek_non_whitespace_forward_strategy()) {
            InMemoryTestInput::Buffered.test_seek_non_whitespace_forward(&input, from, expected, 255)
        }

        #[test]
        fn borrowed_input_seek_non_whitespace_forward_is_correct((input, from, expected) in seek_non_whitespace_forward_strategy()) {
            InMemoryTestInput::Borrowed.test_seek_non_whitespace_forward(&input, from, expected, 255)
        }

        #[test]
        fn owned_input_seek_non_whitespace_forward_is_correct((input, from, expected) in seek_non_whitespace_forward_strategy()) {
            InMemoryTestInput::Owned.test_seek_non_whitespace_forward(&input, from, expected, 255)
        }

        #[test]
        fn buffered_input_seek_backward_is_correct((input, from, expected) in seek_backward_strategy()) {
            InMemoryTestInput::Buffered.test_seek_backward(&input, from, 255, expected)
        }

        #[test]
        fn borrowed_input_seek_backward_is_correct((input, from, expected) in seek_backward_strategy()) {
            InMemoryTestInput::Borrowed.test_seek_backward(&input, from, 255, expected)
        }

        #[test]
        fn owned_input_seek_backward_is_correct((input, from, expected) in seek_backward_strategy()) {
            InMemoryTestInput::Owned.test_seek_backward(&input, from, 255, expected)
        }

        #[test]
        fn buffered_input_seek_non_whitespace_backward_is_correct((input, from, expected) in seek_non_whitespace_backward_strategy()) {
            InMemoryTestInput::Buffered.test_seek_non_whitespace_backward(&input, from, expected, 255)
        }

        #[test]
        fn borrowed_input_seek_non_whitespace_backward_is_correct((input, from, expected) in seek_non_whitespace_backward_strategy()) {
            InMemoryTestInput::Borrowed.test_seek_non_whitespace_backward(&input, from, expected, 255)
        }

        #[test]
        fn owned_input_seek_non_whitespace_backward_is_correct((input, from, expected) in seek_non_whitespace_backward_strategy()) {
            InMemoryTestInput::Owned.test_seek_non_whitespace_backward(&input, from, expected, 255)
        }

        /*#[test]
        fn buffered_input_is_member_match_should_be_true((input, from, to, pattern) in positive_is_member_match_strategy()) {
             InMemoryTestInput::Buffered.test_positive_is_member_match(&input, from, to, pattern)
        }

        #[test]
        fn borrowed_input_is_member_match_should_be_true((input, from, to, pattern) in positive_is_member_match_strategy()) {
            InMemoryTestInput::Borrowed.test_positive_is_member_match(&input, from, to, pattern)
        }

        #[test]
        fn owned_input_is_member_match_should_be_true((input, from, to, pattern) in positive_is_member_match_strategy()) {
            InMemoryTestInput::Owned.test_positive_is_member_match(&input, from, to, pattern)
        }*/
    }

    prop_compose! {
        fn seek_forward_strategy()
            (input in prop::collection::vec(0_u8..=254, 1..1024))
            (mut from in 0..input.len(), mut expected in 0..input.len(), mut input in Just(input)) -> (Vec<u8>, usize, usize)
        {
            if expected < from {
                std::mem::swap(&mut expected, &mut from);
            }
            input[expected] = 255;

            (input, from, expected)
        }
    }

    prop_compose! {
        fn seek_backward_strategy()
            (input in prop::collection::vec(0_u8..=254, 1..1024))
            (mut from in 0..input.len(), mut expected in 0..input.len(), mut input in Just(input)) -> (Vec<u8>, usize, usize)
        {
            if expected > from {
                std::mem::swap(&mut expected, &mut from);
            }
            input[expected] = 255;

            (input, from, expected)
        }
    }

    prop_compose! {
        fn seek_non_whitespace_forward_strategy()
            (ws_idx in prop::collection::vec(0..JSON_WHITESPACE_BYTES.len(), 1..1024))
            (mut from in 0..ws_idx.len(), mut expected in 0..ws_idx.len(), ws_idx in Just(ws_idx)) -> (Vec<u8>, usize, usize)
        {
            if expected < from {
                std::mem::swap(&mut expected, &mut from);
            }

            let mut input = decode_whitespace(ws_idx);
            input[expected] = 255;

            (input, from, expected)
        }
    }

    prop_compose! {
        fn seek_non_whitespace_backward_strategy()
            (ws_idx in prop::collection::vec(0..JSON_WHITESPACE_BYTES.len(), 1..1024))
            (mut from in 0..ws_idx.len(), mut expected in 0..ws_idx.len(), ws_idx in Just(ws_idx)) -> (Vec<u8>, usize, usize)
        {
            if expected > from {
                std::mem::swap(&mut expected, &mut from);
            }

            let mut input = decode_whitespace(ws_idx);
            input[expected] = 255;

            (input, from, expected)
        }
    }

    prop_compose! {
        fn positive_is_member_match_strategy()
            (input in prop::collection::vec(prop::num::u8::ANY, 2..1024))
            (mut from in 0..input.len(), mut to in 0..input.len(), mut input in Just(input)) -> (Vec<u8>, usize, usize, StringPattern)
        {
            if from > to {
                std::mem::swap(&mut from, &mut to);
            }

            if to - from == 0 {
                if from == 0 {
                    to += 2;
                }
                else if from == 1 {
                    from -= 1;
                    to += 1;
                }
                else {
                    from -= 2;
                }
            }

            if to - from == 1 {
                if from == 0 {
                    to += 1;
                }
                else {
                    from -= 1;
                }
            }

            let str = "x".repeat(to - from - 2);
            let json_string = JsonString::new(&str);
            let pattern = StringPattern::new(&json_string);
            let slice = &mut input[from..to];

            slice.copy_from_slice(json_string.quoted().as_bytes());

            if from != 0 && input[from - 1] == b'\\' {
                input[from - 1] = 255;
            }

            (input, from, to, pattern)
        }
    }
}
