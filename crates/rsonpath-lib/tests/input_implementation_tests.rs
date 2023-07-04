use pretty_assertions::assert_eq;
use rsonpath::{
    input::{error::InputError, *},
    FallibleIterator, recorder::EmptyRecorder,
};
use std::{cmp, fs, iter};
use std::{fs::File, io::Read};
use test_case::test_case;

const ROOT_TEST_DIRECTORY: &str = "tests/documents/json/large";
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
file_test_cases!(owned_bytes, FileTestInput::Owned);

mod in_memory_proptests {
    use proptest::{collection, num, prelude::*};

    use crate::InMemoryTestInput;

    proptest! {
        #[test]
        fn buffered_input_represents_the_same_bytes_padded(input in collection::vec(num::u8::ANY, collection::SizeRange::default())) {
            InMemoryTestInput::Buffered.test_on_bytes(&input)
        }

        #[test]
        fn owned_bytes_represents_the_same_bytes_padded(input in collection::vec(num::u8::ANY, collection::SizeRange::default())) {
            InMemoryTestInput::Owned.test_on_bytes(&input)
        }
    }
}

#[derive(Debug)]
enum FileTestInput {
    Buffered,
    Mmap,
    Owned,
}

#[derive(Debug)]
enum InMemoryTestInput {
    Buffered,
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
            FileTestInput::Owned => Self::test_owned(path),
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

    fn test_owned(path: &str) {
        let mut buf = vec![];
        let mut file = Self::get_file(path);
        file.read_to_end(&mut buf).unwrap();
        let input = OwnedBytes::new(&buf).unwrap();

        test_equivalence(&buf, input);
    }
}

impl InMemoryTestInput {
    fn test_on_bytes(&self, bytes: &[u8]) {
        match self {
            InMemoryTestInput::Buffered => Self::test_buffered(bytes),
            InMemoryTestInput::Owned => Self::test_owned(bytes),
        }
    }

    fn test_buffered(bytes: &[u8]) {
        let read = ReadBytes(bytes, 0);
        let input = BufferedInput::new(read);

        test_equivalence(bytes, input);
    }

    fn test_owned(bytes: &[u8]) {
        let input = OwnedBytes::new(&bytes).unwrap();

        test_equivalence(bytes, input);
    }
}

fn test_equivalence<I: Input>(original_contents: &[u8], input: I) {
    let original_length = original_contents.len();
    let mut input_contents = read_input_to_end(input).unwrap();

    assert_padding_is_correct(&input_contents, original_length);
    remove_padding(&mut input_contents, original_length);
    buffered_assert_eq(&input_contents, original_contents);
}

fn read_input_to_end<I: Input>(input: I) -> Result<Vec<u8>, InputError> {
    let mut result: Vec<u8> = vec![];
    let mut iter = input.iter_blocks::<_, BLOCK_SIZE>(&EmptyRecorder);

    while let Some(block) = iter.next()? {
        result.extend_from_slice(&block)
    }

    Ok(result)
}

fn assert_padding_is_correct(result: &[u8], original_length: usize) {
    assert_eq!(result.len() % BLOCK_SIZE, 0);
    assert!(
        result.len() >= original_length,
        "result len ({}) should be at least the original length ({})",
        result.len(),
        original_length
    );

    let padding_length = result.len() - original_length;
    let expected_padding: Vec<u8> = iter::repeat(0).take(padding_length).collect();

    assert_eq!(&result[original_length..], expected_padding);
}

fn remove_padding(result: &mut Vec<u8>, original_length: usize) {
    while result.len() > original_length {
        result.pop();
    }
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
