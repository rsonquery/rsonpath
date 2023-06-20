use pretty_assertions::assert_eq;
use rsonpath::engine::{main::MainEngine, recursive::RecursiveEngine, Compiler, Engine};
use rsonpath::input::*;
use rsonpath::query::JsonPathQuery;
use rsonpath::result::*;
use std::cmp;
use std::error::Error;
use std::io::Read;
struct ReadString(String, usize);
impl ReadString {
    fn new(string: String) -> Self {
        Self(string, 0)
    }
}
impl Read for ReadString {
    fn read(&mut self, buf: &mut [u8]) -> std::io::Result<usize> {
        let rem = self.0.as_bytes().len() - self.1;
        if rem > 0 {
            let size = cmp::min(1024, rem);
            buf[..size].copy_from_slice(&self.0.as_bytes()[self.1..self.1 + size]);
            self.1 += size;
            Ok(size)
        } else {
            Ok(0)
        }
    }
}
mod mmap_tmp_file {
    use std::fs::File;
    use std::io::{Seek, SeekFrom, Write};
    pub(super) fn create_with_contents(contents: &str) -> std::io::Result<File> {
        let mut tmpfile = tempfile::tempfile()?;
        write!(tmpfile, "{}", contents)?;
        tmpfile.seek(SeekFrom::Start(0))?;
        Ok(tmpfile)
    }
}
#[test]
fn whitespace_separators_between_structurals_to_test_correctness_of_index_result_handling_with_query_select_root_with_owned_bytes_and_count_result_using_main_engine(
) -> Result<(), Box<dyn Error>> {
    println ! ("on document index_result.toml running the query $ (select root) with Input impl OwnedBytes and result mode CountResult");
    let jsonpath_query = JsonPathQuery::parse("$")?;
    let raw_json = "{\"a\"  :  [0  ,  1  ,  2]}\n";
    let input = OwnedBytes::new(&raw_json.as_bytes())?;
    let engine = MainEngine::compile_query(&jsonpath_query)?;
    let result = engine.run::<_, CountResult>(&input)?;
    assert_eq!(result.get(), 1u64, "result != expected");
    Ok(())
}
#[test]
fn whitespace_separators_between_structurals_to_test_correctness_of_index_result_handling_with_query_select_root_with_owned_bytes_and_count_result_using_recursive_engine(
) -> Result<(), Box<dyn Error>> {
    println ! ("on document index_result.toml running the query $ (select root) with Input impl OwnedBytes and result mode CountResult");
    let jsonpath_query = JsonPathQuery::parse("$")?;
    let raw_json = "{\"a\"  :  [0  ,  1  ,  2]}\n";
    let input = OwnedBytes::new(&raw_json.as_bytes())?;
    let engine = RecursiveEngine::compile_query(&jsonpath_query)?;
    let result = engine.run::<_, CountResult>(&input)?;
    assert_eq!(result.get(), 1u64, "result != expected");
    Ok(())
}
#[test]
fn whitespace_separators_between_structurals_to_test_correctness_of_index_result_handling_with_query_select_root_with_owned_bytes_and_index_result_using_main_engine(
) -> Result<(), Box<dyn Error>> {
    println ! ("on document index_result.toml running the query $ (select root) with Input impl OwnedBytes and result mode IndexResult");
    let jsonpath_query = JsonPathQuery::parse("$")?;
    let raw_json = "{\"a\"  :  [0  ,  1  ,  2]}\n";
    let input = OwnedBytes::new(&raw_json.as_bytes())?;
    let engine = MainEngine::compile_query(&jsonpath_query)?;
    let result = engine.run::<_, IndexResult>(&input)?;
    assert_eq!(result.get(), vec![0usize,], "result != expected");
    Ok(())
}
#[test]
fn whitespace_separators_between_structurals_to_test_correctness_of_index_result_handling_with_query_select_root_with_owned_bytes_and_index_result_using_recursive_engine(
) -> Result<(), Box<dyn Error>> {
    println ! ("on document index_result.toml running the query $ (select root) with Input impl OwnedBytes and result mode IndexResult");
    let jsonpath_query = JsonPathQuery::parse("$")?;
    let raw_json = "{\"a\"  :  [0  ,  1  ,  2]}\n";
    let input = OwnedBytes::new(&raw_json.as_bytes())?;
    let engine = RecursiveEngine::compile_query(&jsonpath_query)?;
    let result = engine.run::<_, IndexResult>(&input)?;
    assert_eq!(result.get(), vec![0usize,], "result != expected");
    Ok(())
}
#[test]
fn whitespace_separators_between_structurals_to_test_correctness_of_index_result_handling_with_query_select_root_with_buffered_input_and_count_result_using_main_engine(
) -> Result<(), Box<dyn Error>> {
    println ! ("on document index_result.toml running the query $ (select root) with Input impl BufferedInput and result mode CountResult");
    let jsonpath_query = JsonPathQuery::parse("$")?;
    let read_string = ReadString::new("{\"a\"  :  [0  ,  1  ,  2]}\n".to_string());
    let input = BufferedInput::new(read_string);
    let engine = MainEngine::compile_query(&jsonpath_query)?;
    let result = engine.run::<_, CountResult>(&input)?;
    assert_eq!(result.get(), 1u64, "result != expected");
    Ok(())
}
#[test]
fn whitespace_separators_between_structurals_to_test_correctness_of_index_result_handling_with_query_select_root_with_buffered_input_and_count_result_using_recursive_engine(
) -> Result<(), Box<dyn Error>> {
    println ! ("on document index_result.toml running the query $ (select root) with Input impl BufferedInput and result mode CountResult");
    let jsonpath_query = JsonPathQuery::parse("$")?;
    let read_string = ReadString::new("{\"a\"  :  [0  ,  1  ,  2]}\n".to_string());
    let input = BufferedInput::new(read_string);
    let engine = RecursiveEngine::compile_query(&jsonpath_query)?;
    let result = engine.run::<_, CountResult>(&input)?;
    assert_eq!(result.get(), 1u64, "result != expected");
    Ok(())
}
#[test]
fn whitespace_separators_between_structurals_to_test_correctness_of_index_result_handling_with_query_select_root_with_buffered_input_and_index_result_using_main_engine(
) -> Result<(), Box<dyn Error>> {
    println ! ("on document index_result.toml running the query $ (select root) with Input impl BufferedInput and result mode IndexResult");
    let jsonpath_query = JsonPathQuery::parse("$")?;
    let read_string = ReadString::new("{\"a\"  :  [0  ,  1  ,  2]}\n".to_string());
    let input = BufferedInput::new(read_string);
    let engine = MainEngine::compile_query(&jsonpath_query)?;
    let result = engine.run::<_, IndexResult>(&input)?;
    assert_eq!(result.get(), vec![0usize,], "result != expected");
    Ok(())
}
#[test]
fn whitespace_separators_between_structurals_to_test_correctness_of_index_result_handling_with_query_select_root_with_buffered_input_and_index_result_using_recursive_engine(
) -> Result<(), Box<dyn Error>> {
    println ! ("on document index_result.toml running the query $ (select root) with Input impl BufferedInput and result mode IndexResult");
    let jsonpath_query = JsonPathQuery::parse("$")?;
    let read_string = ReadString::new("{\"a\"  :  [0  ,  1  ,  2]}\n".to_string());
    let input = BufferedInput::new(read_string);
    let engine = RecursiveEngine::compile_query(&jsonpath_query)?;
    let result = engine.run::<_, IndexResult>(&input)?;
    assert_eq!(result.get(), vec![0usize,], "result != expected");
    Ok(())
}
#[test]
fn whitespace_separators_between_structurals_to_test_correctness_of_index_result_handling_with_query_select_root_with_mmap_input_and_count_result_using_main_engine(
) -> Result<(), Box<dyn Error>> {
    println ! ("on document index_result.toml running the query $ (select root) with Input impl MmapInput and result mode CountResult");
    let jsonpath_query = JsonPathQuery::parse("$")?;
    let tmp_file = mmap_tmp_file::create_with_contents("{\"a\"  :  [0  ,  1  ,  2]}\n")?;
    let input = unsafe { MmapInput::map_file(&tmp_file)? };
    let engine = MainEngine::compile_query(&jsonpath_query)?;
    let result = engine.run::<_, CountResult>(&input)?;
    assert_eq!(result.get(), 1u64, "result != expected");
    Ok(())
}
#[test]
fn whitespace_separators_between_structurals_to_test_correctness_of_index_result_handling_with_query_select_root_with_mmap_input_and_count_result_using_recursive_engine(
) -> Result<(), Box<dyn Error>> {
    println ! ("on document index_result.toml running the query $ (select root) with Input impl MmapInput and result mode CountResult");
    let jsonpath_query = JsonPathQuery::parse("$")?;
    let tmp_file = mmap_tmp_file::create_with_contents("{\"a\"  :  [0  ,  1  ,  2]}\n")?;
    let input = unsafe { MmapInput::map_file(&tmp_file)? };
    let engine = RecursiveEngine::compile_query(&jsonpath_query)?;
    let result = engine.run::<_, CountResult>(&input)?;
    assert_eq!(result.get(), 1u64, "result != expected");
    Ok(())
}
#[test]
fn whitespace_separators_between_structurals_to_test_correctness_of_index_result_handling_with_query_select_root_with_mmap_input_and_index_result_using_main_engine(
) -> Result<(), Box<dyn Error>> {
    println ! ("on document index_result.toml running the query $ (select root) with Input impl MmapInput and result mode IndexResult");
    let jsonpath_query = JsonPathQuery::parse("$")?;
    let tmp_file = mmap_tmp_file::create_with_contents("{\"a\"  :  [0  ,  1  ,  2]}\n")?;
    let input = unsafe { MmapInput::map_file(&tmp_file)? };
    let engine = MainEngine::compile_query(&jsonpath_query)?;
    let result = engine.run::<_, IndexResult>(&input)?;
    assert_eq!(result.get(), vec![0usize,], "result != expected");
    Ok(())
}
#[test]
fn whitespace_separators_between_structurals_to_test_correctness_of_index_result_handling_with_query_select_root_with_mmap_input_and_index_result_using_recursive_engine(
) -> Result<(), Box<dyn Error>> {
    println ! ("on document index_result.toml running the query $ (select root) with Input impl MmapInput and result mode IndexResult");
    let jsonpath_query = JsonPathQuery::parse("$")?;
    let tmp_file = mmap_tmp_file::create_with_contents("{\"a\"  :  [0  ,  1  ,  2]}\n")?;
    let input = unsafe { MmapInput::map_file(&tmp_file)? };
    let engine = RecursiveEngine::compile_query(&jsonpath_query)?;
    let result = engine.run::<_, IndexResult>(&input)?;
    assert_eq!(result.get(), vec![0usize,], "result != expected");
    Ok(())
}
#[test]
fn whitespace_separators_between_structurals_to_test_correctness_of_index_result_handling_with_query_select_a_by_child_with_owned_bytes_and_count_result_using_main_engine(
) -> Result<(), Box<dyn Error>> {
    println ! ("on document index_result.toml running the query $.a (select a by child) with Input impl OwnedBytes and result mode CountResult");
    let jsonpath_query = JsonPathQuery::parse("$.a")?;
    let raw_json = "{\"a\"  :  [0  ,  1  ,  2]}\n";
    let input = OwnedBytes::new(&raw_json.as_bytes())?;
    let engine = MainEngine::compile_query(&jsonpath_query)?;
    let result = engine.run::<_, CountResult>(&input)?;
    assert_eq!(result.get(), 1u64, "result != expected");
    Ok(())
}
#[test]
fn whitespace_separators_between_structurals_to_test_correctness_of_index_result_handling_with_query_select_a_by_child_with_owned_bytes_and_count_result_using_recursive_engine(
) -> Result<(), Box<dyn Error>> {
    println ! ("on document index_result.toml running the query $.a (select a by child) with Input impl OwnedBytes and result mode CountResult");
    let jsonpath_query = JsonPathQuery::parse("$.a")?;
    let raw_json = "{\"a\"  :  [0  ,  1  ,  2]}\n";
    let input = OwnedBytes::new(&raw_json.as_bytes())?;
    let engine = RecursiveEngine::compile_query(&jsonpath_query)?;
    let result = engine.run::<_, CountResult>(&input)?;
    assert_eq!(result.get(), 1u64, "result != expected");
    Ok(())
}
#[test]
fn whitespace_separators_between_structurals_to_test_correctness_of_index_result_handling_with_query_select_a_by_child_with_owned_bytes_and_index_result_using_main_engine(
) -> Result<(), Box<dyn Error>> {
    println ! ("on document index_result.toml running the query $.a (select a by child) with Input impl OwnedBytes and result mode IndexResult");
    let jsonpath_query = JsonPathQuery::parse("$.a")?;
    let raw_json = "{\"a\"  :  [0  ,  1  ,  2]}\n";
    let input = OwnedBytes::new(&raw_json.as_bytes())?;
    let engine = MainEngine::compile_query(&jsonpath_query)?;
    let result = engine.run::<_, IndexResult>(&input)?;
    assert_eq!(result.get(), vec![9usize,], "result != expected");
    Ok(())
}
#[test]
fn whitespace_separators_between_structurals_to_test_correctness_of_index_result_handling_with_query_select_a_by_child_with_owned_bytes_and_index_result_using_recursive_engine(
) -> Result<(), Box<dyn Error>> {
    println ! ("on document index_result.toml running the query $.a (select a by child) with Input impl OwnedBytes and result mode IndexResult");
    let jsonpath_query = JsonPathQuery::parse("$.a")?;
    let raw_json = "{\"a\"  :  [0  ,  1  ,  2]}\n";
    let input = OwnedBytes::new(&raw_json.as_bytes())?;
    let engine = RecursiveEngine::compile_query(&jsonpath_query)?;
    let result = engine.run::<_, IndexResult>(&input)?;
    assert_eq!(result.get(), vec![9usize,], "result != expected");
    Ok(())
}
#[test]
fn whitespace_separators_between_structurals_to_test_correctness_of_index_result_handling_with_query_select_a_by_child_with_buffered_input_and_count_result_using_main_engine(
) -> Result<(), Box<dyn Error>> {
    println ! ("on document index_result.toml running the query $.a (select a by child) with Input impl BufferedInput and result mode CountResult");
    let jsonpath_query = JsonPathQuery::parse("$.a")?;
    let read_string = ReadString::new("{\"a\"  :  [0  ,  1  ,  2]}\n".to_string());
    let input = BufferedInput::new(read_string);
    let engine = MainEngine::compile_query(&jsonpath_query)?;
    let result = engine.run::<_, CountResult>(&input)?;
    assert_eq!(result.get(), 1u64, "result != expected");
    Ok(())
}
#[test]
fn whitespace_separators_between_structurals_to_test_correctness_of_index_result_handling_with_query_select_a_by_child_with_buffered_input_and_count_result_using_recursive_engine(
) -> Result<(), Box<dyn Error>> {
    println ! ("on document index_result.toml running the query $.a (select a by child) with Input impl BufferedInput and result mode CountResult");
    let jsonpath_query = JsonPathQuery::parse("$.a")?;
    let read_string = ReadString::new("{\"a\"  :  [0  ,  1  ,  2]}\n".to_string());
    let input = BufferedInput::new(read_string);
    let engine = RecursiveEngine::compile_query(&jsonpath_query)?;
    let result = engine.run::<_, CountResult>(&input)?;
    assert_eq!(result.get(), 1u64, "result != expected");
    Ok(())
}
#[test]
fn whitespace_separators_between_structurals_to_test_correctness_of_index_result_handling_with_query_select_a_by_child_with_buffered_input_and_index_result_using_main_engine(
) -> Result<(), Box<dyn Error>> {
    println ! ("on document index_result.toml running the query $.a (select a by child) with Input impl BufferedInput and result mode IndexResult");
    let jsonpath_query = JsonPathQuery::parse("$.a")?;
    let read_string = ReadString::new("{\"a\"  :  [0  ,  1  ,  2]}\n".to_string());
    let input = BufferedInput::new(read_string);
    let engine = MainEngine::compile_query(&jsonpath_query)?;
    let result = engine.run::<_, IndexResult>(&input)?;
    assert_eq!(result.get(), vec![9usize,], "result != expected");
    Ok(())
}
#[test]
fn whitespace_separators_between_structurals_to_test_correctness_of_index_result_handling_with_query_select_a_by_child_with_buffered_input_and_index_result_using_recursive_engine(
) -> Result<(), Box<dyn Error>> {
    println ! ("on document index_result.toml running the query $.a (select a by child) with Input impl BufferedInput and result mode IndexResult");
    let jsonpath_query = JsonPathQuery::parse("$.a")?;
    let read_string = ReadString::new("{\"a\"  :  [0  ,  1  ,  2]}\n".to_string());
    let input = BufferedInput::new(read_string);
    let engine = RecursiveEngine::compile_query(&jsonpath_query)?;
    let result = engine.run::<_, IndexResult>(&input)?;
    assert_eq!(result.get(), vec![9usize,], "result != expected");
    Ok(())
}
#[test]
fn whitespace_separators_between_structurals_to_test_correctness_of_index_result_handling_with_query_select_a_by_child_with_mmap_input_and_count_result_using_main_engine(
) -> Result<(), Box<dyn Error>> {
    println ! ("on document index_result.toml running the query $.a (select a by child) with Input impl MmapInput and result mode CountResult");
    let jsonpath_query = JsonPathQuery::parse("$.a")?;
    let tmp_file = mmap_tmp_file::create_with_contents("{\"a\"  :  [0  ,  1  ,  2]}\n")?;
    let input = unsafe { MmapInput::map_file(&tmp_file)? };
    let engine = MainEngine::compile_query(&jsonpath_query)?;
    let result = engine.run::<_, CountResult>(&input)?;
    assert_eq!(result.get(), 1u64, "result != expected");
    Ok(())
}
#[test]
fn whitespace_separators_between_structurals_to_test_correctness_of_index_result_handling_with_query_select_a_by_child_with_mmap_input_and_count_result_using_recursive_engine(
) -> Result<(), Box<dyn Error>> {
    println ! ("on document index_result.toml running the query $.a (select a by child) with Input impl MmapInput and result mode CountResult");
    let jsonpath_query = JsonPathQuery::parse("$.a")?;
    let tmp_file = mmap_tmp_file::create_with_contents("{\"a\"  :  [0  ,  1  ,  2]}\n")?;
    let input = unsafe { MmapInput::map_file(&tmp_file)? };
    let engine = RecursiveEngine::compile_query(&jsonpath_query)?;
    let result = engine.run::<_, CountResult>(&input)?;
    assert_eq!(result.get(), 1u64, "result != expected");
    Ok(())
}
#[test]
fn whitespace_separators_between_structurals_to_test_correctness_of_index_result_handling_with_query_select_a_by_child_with_mmap_input_and_index_result_using_main_engine(
) -> Result<(), Box<dyn Error>> {
    println ! ("on document index_result.toml running the query $.a (select a by child) with Input impl MmapInput and result mode IndexResult");
    let jsonpath_query = JsonPathQuery::parse("$.a")?;
    let tmp_file = mmap_tmp_file::create_with_contents("{\"a\"  :  [0  ,  1  ,  2]}\n")?;
    let input = unsafe { MmapInput::map_file(&tmp_file)? };
    let engine = MainEngine::compile_query(&jsonpath_query)?;
    let result = engine.run::<_, IndexResult>(&input)?;
    assert_eq!(result.get(), vec![9usize,], "result != expected");
    Ok(())
}
#[test]
fn whitespace_separators_between_structurals_to_test_correctness_of_index_result_handling_with_query_select_a_by_child_with_mmap_input_and_index_result_using_recursive_engine(
) -> Result<(), Box<dyn Error>> {
    println ! ("on document index_result.toml running the query $.a (select a by child) with Input impl MmapInput and result mode IndexResult");
    let jsonpath_query = JsonPathQuery::parse("$.a")?;
    let tmp_file = mmap_tmp_file::create_with_contents("{\"a\"  :  [0  ,  1  ,  2]}\n")?;
    let input = unsafe { MmapInput::map_file(&tmp_file)? };
    let engine = RecursiveEngine::compile_query(&jsonpath_query)?;
    let result = engine.run::<_, IndexResult>(&input)?;
    assert_eq!(result.get(), vec![9usize,], "result != expected");
    Ok(())
}
#[test]
fn whitespace_separators_between_structurals_to_test_correctness_of_index_result_handling_with_query_select_a_by_descendant_with_owned_bytes_and_count_result_using_main_engine(
) -> Result<(), Box<dyn Error>> {
    println ! ("on document index_result.toml running the query $..a (select a by descendant) with Input impl OwnedBytes and result mode CountResult");
    let jsonpath_query = JsonPathQuery::parse("$..a")?;
    let raw_json = "{\"a\"  :  [0  ,  1  ,  2]}\n";
    let input = OwnedBytes::new(&raw_json.as_bytes())?;
    let engine = MainEngine::compile_query(&jsonpath_query)?;
    let result = engine.run::<_, CountResult>(&input)?;
    assert_eq!(result.get(), 1u64, "result != expected");
    Ok(())
}
#[test]
fn whitespace_separators_between_structurals_to_test_correctness_of_index_result_handling_with_query_select_a_by_descendant_with_owned_bytes_and_count_result_using_recursive_engine(
) -> Result<(), Box<dyn Error>> {
    println ! ("on document index_result.toml running the query $..a (select a by descendant) with Input impl OwnedBytes and result mode CountResult");
    let jsonpath_query = JsonPathQuery::parse("$..a")?;
    let raw_json = "{\"a\"  :  [0  ,  1  ,  2]}\n";
    let input = OwnedBytes::new(&raw_json.as_bytes())?;
    let engine = RecursiveEngine::compile_query(&jsonpath_query)?;
    let result = engine.run::<_, CountResult>(&input)?;
    assert_eq!(result.get(), 1u64, "result != expected");
    Ok(())
}
#[test]
fn whitespace_separators_between_structurals_to_test_correctness_of_index_result_handling_with_query_select_a_by_descendant_with_owned_bytes_and_index_result_using_main_engine(
) -> Result<(), Box<dyn Error>> {
    println ! ("on document index_result.toml running the query $..a (select a by descendant) with Input impl OwnedBytes and result mode IndexResult");
    let jsonpath_query = JsonPathQuery::parse("$..a")?;
    let raw_json = "{\"a\"  :  [0  ,  1  ,  2]}\n";
    let input = OwnedBytes::new(&raw_json.as_bytes())?;
    let engine = MainEngine::compile_query(&jsonpath_query)?;
    let result = engine.run::<_, IndexResult>(&input)?;
    assert_eq!(result.get(), vec![9usize,], "result != expected");
    Ok(())
}
#[test]
fn whitespace_separators_between_structurals_to_test_correctness_of_index_result_handling_with_query_select_a_by_descendant_with_owned_bytes_and_index_result_using_recursive_engine(
) -> Result<(), Box<dyn Error>> {
    println ! ("on document index_result.toml running the query $..a (select a by descendant) with Input impl OwnedBytes and result mode IndexResult");
    let jsonpath_query = JsonPathQuery::parse("$..a")?;
    let raw_json = "{\"a\"  :  [0  ,  1  ,  2]}\n";
    let input = OwnedBytes::new(&raw_json.as_bytes())?;
    let engine = RecursiveEngine::compile_query(&jsonpath_query)?;
    let result = engine.run::<_, IndexResult>(&input)?;
    assert_eq!(result.get(), vec![9usize,], "result != expected");
    Ok(())
}
#[test]
fn whitespace_separators_between_structurals_to_test_correctness_of_index_result_handling_with_query_select_a_by_descendant_with_buffered_input_and_count_result_using_main_engine(
) -> Result<(), Box<dyn Error>> {
    println ! ("on document index_result.toml running the query $..a (select a by descendant) with Input impl BufferedInput and result mode CountResult");
    let jsonpath_query = JsonPathQuery::parse("$..a")?;
    let read_string = ReadString::new("{\"a\"  :  [0  ,  1  ,  2]}\n".to_string());
    let input = BufferedInput::new(read_string);
    let engine = MainEngine::compile_query(&jsonpath_query)?;
    let result = engine.run::<_, CountResult>(&input)?;
    assert_eq!(result.get(), 1u64, "result != expected");
    Ok(())
}
#[test]
fn whitespace_separators_between_structurals_to_test_correctness_of_index_result_handling_with_query_select_a_by_descendant_with_buffered_input_and_count_result_using_recursive_engine(
) -> Result<(), Box<dyn Error>> {
    println ! ("on document index_result.toml running the query $..a (select a by descendant) with Input impl BufferedInput and result mode CountResult");
    let jsonpath_query = JsonPathQuery::parse("$..a")?;
    let read_string = ReadString::new("{\"a\"  :  [0  ,  1  ,  2]}\n".to_string());
    let input = BufferedInput::new(read_string);
    let engine = RecursiveEngine::compile_query(&jsonpath_query)?;
    let result = engine.run::<_, CountResult>(&input)?;
    assert_eq!(result.get(), 1u64, "result != expected");
    Ok(())
}
#[test]
fn whitespace_separators_between_structurals_to_test_correctness_of_index_result_handling_with_query_select_a_by_descendant_with_buffered_input_and_index_result_using_main_engine(
) -> Result<(), Box<dyn Error>> {
    println ! ("on document index_result.toml running the query $..a (select a by descendant) with Input impl BufferedInput and result mode IndexResult");
    let jsonpath_query = JsonPathQuery::parse("$..a")?;
    let read_string = ReadString::new("{\"a\"  :  [0  ,  1  ,  2]}\n".to_string());
    let input = BufferedInput::new(read_string);
    let engine = MainEngine::compile_query(&jsonpath_query)?;
    let result = engine.run::<_, IndexResult>(&input)?;
    assert_eq!(result.get(), vec![9usize,], "result != expected");
    Ok(())
}
#[test]
fn whitespace_separators_between_structurals_to_test_correctness_of_index_result_handling_with_query_select_a_by_descendant_with_buffered_input_and_index_result_using_recursive_engine(
) -> Result<(), Box<dyn Error>> {
    println ! ("on document index_result.toml running the query $..a (select a by descendant) with Input impl BufferedInput and result mode IndexResult");
    let jsonpath_query = JsonPathQuery::parse("$..a")?;
    let read_string = ReadString::new("{\"a\"  :  [0  ,  1  ,  2]}\n".to_string());
    let input = BufferedInput::new(read_string);
    let engine = RecursiveEngine::compile_query(&jsonpath_query)?;
    let result = engine.run::<_, IndexResult>(&input)?;
    assert_eq!(result.get(), vec![9usize,], "result != expected");
    Ok(())
}
#[test]
fn whitespace_separators_between_structurals_to_test_correctness_of_index_result_handling_with_query_select_a_by_descendant_with_mmap_input_and_count_result_using_main_engine(
) -> Result<(), Box<dyn Error>> {
    println ! ("on document index_result.toml running the query $..a (select a by descendant) with Input impl MmapInput and result mode CountResult");
    let jsonpath_query = JsonPathQuery::parse("$..a")?;
    let tmp_file = mmap_tmp_file::create_with_contents("{\"a\"  :  [0  ,  1  ,  2]}\n")?;
    let input = unsafe { MmapInput::map_file(&tmp_file)? };
    let engine = MainEngine::compile_query(&jsonpath_query)?;
    let result = engine.run::<_, CountResult>(&input)?;
    assert_eq!(result.get(), 1u64, "result != expected");
    Ok(())
}
#[test]
fn whitespace_separators_between_structurals_to_test_correctness_of_index_result_handling_with_query_select_a_by_descendant_with_mmap_input_and_count_result_using_recursive_engine(
) -> Result<(), Box<dyn Error>> {
    println ! ("on document index_result.toml running the query $..a (select a by descendant) with Input impl MmapInput and result mode CountResult");
    let jsonpath_query = JsonPathQuery::parse("$..a")?;
    let tmp_file = mmap_tmp_file::create_with_contents("{\"a\"  :  [0  ,  1  ,  2]}\n")?;
    let input = unsafe { MmapInput::map_file(&tmp_file)? };
    let engine = RecursiveEngine::compile_query(&jsonpath_query)?;
    let result = engine.run::<_, CountResult>(&input)?;
    assert_eq!(result.get(), 1u64, "result != expected");
    Ok(())
}
#[test]
fn whitespace_separators_between_structurals_to_test_correctness_of_index_result_handling_with_query_select_a_by_descendant_with_mmap_input_and_index_result_using_main_engine(
) -> Result<(), Box<dyn Error>> {
    println ! ("on document index_result.toml running the query $..a (select a by descendant) with Input impl MmapInput and result mode IndexResult");
    let jsonpath_query = JsonPathQuery::parse("$..a")?;
    let tmp_file = mmap_tmp_file::create_with_contents("{\"a\"  :  [0  ,  1  ,  2]}\n")?;
    let input = unsafe { MmapInput::map_file(&tmp_file)? };
    let engine = MainEngine::compile_query(&jsonpath_query)?;
    let result = engine.run::<_, IndexResult>(&input)?;
    assert_eq!(result.get(), vec![9usize,], "result != expected");
    Ok(())
}
#[test]
fn whitespace_separators_between_structurals_to_test_correctness_of_index_result_handling_with_query_select_a_by_descendant_with_mmap_input_and_index_result_using_recursive_engine(
) -> Result<(), Box<dyn Error>> {
    println ! ("on document index_result.toml running the query $..a (select a by descendant) with Input impl MmapInput and result mode IndexResult");
    let jsonpath_query = JsonPathQuery::parse("$..a")?;
    let tmp_file = mmap_tmp_file::create_with_contents("{\"a\"  :  [0  ,  1  ,  2]}\n")?;
    let input = unsafe { MmapInput::map_file(&tmp_file)? };
    let engine = RecursiveEngine::compile_query(&jsonpath_query)?;
    let result = engine.run::<_, IndexResult>(&input)?;
    assert_eq!(result.get(), vec![9usize,], "result != expected");
    Ok(())
}
#[test]
fn whitespace_separators_between_structurals_to_test_correctness_of_index_result_handling_with_query_select_each_item_on_the_list_with_wildcard_with_owned_bytes_and_count_result_using_main_engine(
) -> Result<(), Box<dyn Error>> {
    println ! ("on document index_result.toml running the query $.a[*] (select each item on the list with wildcard) with Input impl OwnedBytes and result mode CountResult");
    let jsonpath_query = JsonPathQuery::parse("$.a[*]")?;
    let raw_json = "{\"a\"  :  [0  ,  1  ,  2]}\n";
    let input = OwnedBytes::new(&raw_json.as_bytes())?;
    let engine = MainEngine::compile_query(&jsonpath_query)?;
    let result = engine.run::<_, CountResult>(&input)?;
    assert_eq!(result.get(), 3u64, "result != expected");
    Ok(())
}
#[test]
fn whitespace_separators_between_structurals_to_test_correctness_of_index_result_handling_with_query_select_each_item_on_the_list_with_wildcard_with_owned_bytes_and_count_result_using_recursive_engine(
) -> Result<(), Box<dyn Error>> {
    println ! ("on document index_result.toml running the query $.a[*] (select each item on the list with wildcard) with Input impl OwnedBytes and result mode CountResult");
    let jsonpath_query = JsonPathQuery::parse("$.a[*]")?;
    let raw_json = "{\"a\"  :  [0  ,  1  ,  2]}\n";
    let input = OwnedBytes::new(&raw_json.as_bytes())?;
    let engine = RecursiveEngine::compile_query(&jsonpath_query)?;
    let result = engine.run::<_, CountResult>(&input)?;
    assert_eq!(result.get(), 3u64, "result != expected");
    Ok(())
}
#[test]
fn whitespace_separators_between_structurals_to_test_correctness_of_index_result_handling_with_query_select_each_item_on_the_list_with_wildcard_with_owned_bytes_and_index_result_using_main_engine(
) -> Result<(), Box<dyn Error>> {
    println ! ("on document index_result.toml running the query $.a[*] (select each item on the list with wildcard) with Input impl OwnedBytes and result mode IndexResult");
    let jsonpath_query = JsonPathQuery::parse("$.a[*]")?;
    let raw_json = "{\"a\"  :  [0  ,  1  ,  2]}\n";
    let input = OwnedBytes::new(&raw_json.as_bytes())?;
    let engine = MainEngine::compile_query(&jsonpath_query)?;
    let result = engine.run::<_, IndexResult>(&input)?;
    assert_eq!(result.get(), vec![10usize, 16usize, 22usize,], "result != expected");
    Ok(())
}
#[test]
fn whitespace_separators_between_structurals_to_test_correctness_of_index_result_handling_with_query_select_each_item_on_the_list_with_wildcard_with_owned_bytes_and_index_result_using_recursive_engine(
) -> Result<(), Box<dyn Error>> {
    println ! ("on document index_result.toml running the query $.a[*] (select each item on the list with wildcard) with Input impl OwnedBytes and result mode IndexResult");
    let jsonpath_query = JsonPathQuery::parse("$.a[*]")?;
    let raw_json = "{\"a\"  :  [0  ,  1  ,  2]}\n";
    let input = OwnedBytes::new(&raw_json.as_bytes())?;
    let engine = RecursiveEngine::compile_query(&jsonpath_query)?;
    let result = engine.run::<_, IndexResult>(&input)?;
    assert_eq!(result.get(), vec![10usize, 16usize, 22usize,], "result != expected");
    Ok(())
}
#[test]
fn whitespace_separators_between_structurals_to_test_correctness_of_index_result_handling_with_query_select_each_item_on_the_list_with_wildcard_with_buffered_input_and_count_result_using_main_engine(
) -> Result<(), Box<dyn Error>> {
    println ! ("on document index_result.toml running the query $.a[*] (select each item on the list with wildcard) with Input impl BufferedInput and result mode CountResult");
    let jsonpath_query = JsonPathQuery::parse("$.a[*]")?;
    let read_string = ReadString::new("{\"a\"  :  [0  ,  1  ,  2]}\n".to_string());
    let input = BufferedInput::new(read_string);
    let engine = MainEngine::compile_query(&jsonpath_query)?;
    let result = engine.run::<_, CountResult>(&input)?;
    assert_eq!(result.get(), 3u64, "result != expected");
    Ok(())
}
#[test]
fn whitespace_separators_between_structurals_to_test_correctness_of_index_result_handling_with_query_select_each_item_on_the_list_with_wildcard_with_buffered_input_and_count_result_using_recursive_engine(
) -> Result<(), Box<dyn Error>> {
    println ! ("on document index_result.toml running the query $.a[*] (select each item on the list with wildcard) with Input impl BufferedInput and result mode CountResult");
    let jsonpath_query = JsonPathQuery::parse("$.a[*]")?;
    let read_string = ReadString::new("{\"a\"  :  [0  ,  1  ,  2]}\n".to_string());
    let input = BufferedInput::new(read_string);
    let engine = RecursiveEngine::compile_query(&jsonpath_query)?;
    let result = engine.run::<_, CountResult>(&input)?;
    assert_eq!(result.get(), 3u64, "result != expected");
    Ok(())
}
#[test]
fn whitespace_separators_between_structurals_to_test_correctness_of_index_result_handling_with_query_select_each_item_on_the_list_with_wildcard_with_buffered_input_and_index_result_using_main_engine(
) -> Result<(), Box<dyn Error>> {
    println ! ("on document index_result.toml running the query $.a[*] (select each item on the list with wildcard) with Input impl BufferedInput and result mode IndexResult");
    let jsonpath_query = JsonPathQuery::parse("$.a[*]")?;
    let read_string = ReadString::new("{\"a\"  :  [0  ,  1  ,  2]}\n".to_string());
    let input = BufferedInput::new(read_string);
    let engine = MainEngine::compile_query(&jsonpath_query)?;
    let result = engine.run::<_, IndexResult>(&input)?;
    assert_eq!(result.get(), vec![10usize, 16usize, 22usize,], "result != expected");
    Ok(())
}
#[test]
fn whitespace_separators_between_structurals_to_test_correctness_of_index_result_handling_with_query_select_each_item_on_the_list_with_wildcard_with_buffered_input_and_index_result_using_recursive_engine(
) -> Result<(), Box<dyn Error>> {
    println ! ("on document index_result.toml running the query $.a[*] (select each item on the list with wildcard) with Input impl BufferedInput and result mode IndexResult");
    let jsonpath_query = JsonPathQuery::parse("$.a[*]")?;
    let read_string = ReadString::new("{\"a\"  :  [0  ,  1  ,  2]}\n".to_string());
    let input = BufferedInput::new(read_string);
    let engine = RecursiveEngine::compile_query(&jsonpath_query)?;
    let result = engine.run::<_, IndexResult>(&input)?;
    assert_eq!(result.get(), vec![10usize, 16usize, 22usize,], "result != expected");
    Ok(())
}
#[test]
fn whitespace_separators_between_structurals_to_test_correctness_of_index_result_handling_with_query_select_each_item_on_the_list_with_wildcard_with_mmap_input_and_count_result_using_main_engine(
) -> Result<(), Box<dyn Error>> {
    println ! ("on document index_result.toml running the query $.a[*] (select each item on the list with wildcard) with Input impl MmapInput and result mode CountResult");
    let jsonpath_query = JsonPathQuery::parse("$.a[*]")?;
    let tmp_file = mmap_tmp_file::create_with_contents("{\"a\"  :  [0  ,  1  ,  2]}\n")?;
    let input = unsafe { MmapInput::map_file(&tmp_file)? };
    let engine = MainEngine::compile_query(&jsonpath_query)?;
    let result = engine.run::<_, CountResult>(&input)?;
    assert_eq!(result.get(), 3u64, "result != expected");
    Ok(())
}
#[test]
fn whitespace_separators_between_structurals_to_test_correctness_of_index_result_handling_with_query_select_each_item_on_the_list_with_wildcard_with_mmap_input_and_count_result_using_recursive_engine(
) -> Result<(), Box<dyn Error>> {
    println ! ("on document index_result.toml running the query $.a[*] (select each item on the list with wildcard) with Input impl MmapInput and result mode CountResult");
    let jsonpath_query = JsonPathQuery::parse("$.a[*]")?;
    let tmp_file = mmap_tmp_file::create_with_contents("{\"a\"  :  [0  ,  1  ,  2]}\n")?;
    let input = unsafe { MmapInput::map_file(&tmp_file)? };
    let engine = RecursiveEngine::compile_query(&jsonpath_query)?;
    let result = engine.run::<_, CountResult>(&input)?;
    assert_eq!(result.get(), 3u64, "result != expected");
    Ok(())
}
#[test]
fn whitespace_separators_between_structurals_to_test_correctness_of_index_result_handling_with_query_select_each_item_on_the_list_with_wildcard_with_mmap_input_and_index_result_using_main_engine(
) -> Result<(), Box<dyn Error>> {
    println ! ("on document index_result.toml running the query $.a[*] (select each item on the list with wildcard) with Input impl MmapInput and result mode IndexResult");
    let jsonpath_query = JsonPathQuery::parse("$.a[*]")?;
    let tmp_file = mmap_tmp_file::create_with_contents("{\"a\"  :  [0  ,  1  ,  2]}\n")?;
    let input = unsafe { MmapInput::map_file(&tmp_file)? };
    let engine = MainEngine::compile_query(&jsonpath_query)?;
    let result = engine.run::<_, IndexResult>(&input)?;
    assert_eq!(result.get(), vec![10usize, 16usize, 22usize,], "result != expected");
    Ok(())
}
#[test]
fn whitespace_separators_between_structurals_to_test_correctness_of_index_result_handling_with_query_select_each_item_on_the_list_with_wildcard_with_mmap_input_and_index_result_using_recursive_engine(
) -> Result<(), Box<dyn Error>> {
    println ! ("on document index_result.toml running the query $.a[*] (select each item on the list with wildcard) with Input impl MmapInput and result mode IndexResult");
    let jsonpath_query = JsonPathQuery::parse("$.a[*]")?;
    let tmp_file = mmap_tmp_file::create_with_contents("{\"a\"  :  [0  ,  1  ,  2]}\n")?;
    let input = unsafe { MmapInput::map_file(&tmp_file)? };
    let engine = RecursiveEngine::compile_query(&jsonpath_query)?;
    let result = engine.run::<_, IndexResult>(&input)?;
    assert_eq!(result.get(), vec![10usize, 16usize, 22usize,], "result != expected");
    Ok(())
}
#[test]
fn short_json_with_objects_and_lists_given_as_an_example_on_jsonpath_com_with_query_select_exact_path_with_name_and_index_selectors_with_owned_bytes_and_count_result_using_main_engine(
) -> Result<(), Box<dyn Error>> {
    println ! ("on document jsonpath_com_example.toml running the query $.phoneNumbers[0].type (select exact path with name and index selectors) with Input impl OwnedBytes and result mode CountResult");
    let jsonpath_query = JsonPathQuery::parse("$.phoneNumbers[0].type")?;
    let raw_json = "{\n    \"firstName\": \"John\",\n    \"lastName\": \"doe\",\n    \"age\": 26,\n    \"address\": {\n        \"streetAddress\": \"naist street\",\n        \"city\": \"Nara\",\n        \"postalCode\": \"630-0192\"\n    },\n    \"phoneNumbers\": [\n        {\n            \"type\": \"iPhone\",\n            \"number\": \"0123-4567-8888\"\n        },\n        {\n            \"type\": \"home\",\n            \"number\": \"0123-4567-8910\"\n        }\n    ]\n}\n" ;
    let input = OwnedBytes::new(&raw_json.as_bytes())?;
    let engine = MainEngine::compile_query(&jsonpath_query)?;
    let result = engine.run::<_, CountResult>(&input)?;
    assert_eq!(result.get(), 1u64, "result != expected");
    Ok(())
}
#[test]
fn short_json_with_objects_and_lists_given_as_an_example_on_jsonpath_com_with_query_select_exact_path_with_name_and_index_selectors_with_owned_bytes_and_count_result_using_recursive_engine(
) -> Result<(), Box<dyn Error>> {
    println ! ("on document jsonpath_com_example.toml running the query $.phoneNumbers[0].type (select exact path with name and index selectors) with Input impl OwnedBytes and result mode CountResult");
    let jsonpath_query = JsonPathQuery::parse("$.phoneNumbers[0].type")?;
    let raw_json = "{\n    \"firstName\": \"John\",\n    \"lastName\": \"doe\",\n    \"age\": 26,\n    \"address\": {\n        \"streetAddress\": \"naist street\",\n        \"city\": \"Nara\",\n        \"postalCode\": \"630-0192\"\n    },\n    \"phoneNumbers\": [\n        {\n            \"type\": \"iPhone\",\n            \"number\": \"0123-4567-8888\"\n        },\n        {\n            \"type\": \"home\",\n            \"number\": \"0123-4567-8910\"\n        }\n    ]\n}\n" ;
    let input = OwnedBytes::new(&raw_json.as_bytes())?;
    let engine = RecursiveEngine::compile_query(&jsonpath_query)?;
    let result = engine.run::<_, CountResult>(&input)?;
    assert_eq!(result.get(), 1u64, "result != expected");
    Ok(())
}
#[test]
fn short_json_with_objects_and_lists_given_as_an_example_on_jsonpath_com_with_query_select_exact_path_with_name_and_index_selectors_with_owned_bytes_and_index_result_using_main_engine(
) -> Result<(), Box<dyn Error>> {
    println ! ("on document jsonpath_com_example.toml running the query $.phoneNumbers[0].type (select exact path with name and index selectors) with Input impl OwnedBytes and result mode IndexResult");
    let jsonpath_query = JsonPathQuery::parse("$.phoneNumbers[0].type")?;
    let raw_json = "{\n    \"firstName\": \"John\",\n    \"lastName\": \"doe\",\n    \"age\": 26,\n    \"address\": {\n        \"streetAddress\": \"naist street\",\n        \"city\": \"Nara\",\n        \"postalCode\": \"630-0192\"\n    },\n    \"phoneNumbers\": [\n        {\n            \"type\": \"iPhone\",\n            \"number\": \"0123-4567-8888\"\n        },\n        {\n            \"type\": \"home\",\n            \"number\": \"0123-4567-8910\"\n        }\n    ]\n}\n" ;
    let input = OwnedBytes::new(&raw_json.as_bytes())?;
    let engine = MainEngine::compile_query(&jsonpath_query)?;
    let result = engine.run::<_, IndexResult>(&input)?;
    assert_eq!(result.get(), vec![239usize,], "result != expected");
    Ok(())
}
#[test]
fn short_json_with_objects_and_lists_given_as_an_example_on_jsonpath_com_with_query_select_exact_path_with_name_and_index_selectors_with_owned_bytes_and_index_result_using_recursive_engine(
) -> Result<(), Box<dyn Error>> {
    println ! ("on document jsonpath_com_example.toml running the query $.phoneNumbers[0].type (select exact path with name and index selectors) with Input impl OwnedBytes and result mode IndexResult");
    let jsonpath_query = JsonPathQuery::parse("$.phoneNumbers[0].type")?;
    let raw_json = "{\n    \"firstName\": \"John\",\n    \"lastName\": \"doe\",\n    \"age\": 26,\n    \"address\": {\n        \"streetAddress\": \"naist street\",\n        \"city\": \"Nara\",\n        \"postalCode\": \"630-0192\"\n    },\n    \"phoneNumbers\": [\n        {\n            \"type\": \"iPhone\",\n            \"number\": \"0123-4567-8888\"\n        },\n        {\n            \"type\": \"home\",\n            \"number\": \"0123-4567-8910\"\n        }\n    ]\n}\n" ;
    let input = OwnedBytes::new(&raw_json.as_bytes())?;
    let engine = RecursiveEngine::compile_query(&jsonpath_query)?;
    let result = engine.run::<_, IndexResult>(&input)?;
    assert_eq!(result.get(), vec![239usize,], "result != expected");
    Ok(())
}
#[test]
fn short_json_with_objects_and_lists_given_as_an_example_on_jsonpath_com_with_query_select_exact_path_with_name_and_index_selectors_with_buffered_input_and_count_result_using_main_engine(
) -> Result<(), Box<dyn Error>> {
    println ! ("on document jsonpath_com_example.toml running the query $.phoneNumbers[0].type (select exact path with name and index selectors) with Input impl BufferedInput and result mode CountResult");
    let jsonpath_query = JsonPathQuery::parse("$.phoneNumbers[0].type")?;
    let read_string = ReadString :: new ("{\n    \"firstName\": \"John\",\n    \"lastName\": \"doe\",\n    \"age\": 26,\n    \"address\": {\n        \"streetAddress\": \"naist street\",\n        \"city\": \"Nara\",\n        \"postalCode\": \"630-0192\"\n    },\n    \"phoneNumbers\": [\n        {\n            \"type\": \"iPhone\",\n            \"number\": \"0123-4567-8888\"\n        },\n        {\n            \"type\": \"home\",\n            \"number\": \"0123-4567-8910\"\n        }\n    ]\n}\n" . to_string ()) ;
    let input = BufferedInput::new(read_string);
    let engine = MainEngine::compile_query(&jsonpath_query)?;
    let result = engine.run::<_, CountResult>(&input)?;
    assert_eq!(result.get(), 1u64, "result != expected");
    Ok(())
}
#[test]
fn short_json_with_objects_and_lists_given_as_an_example_on_jsonpath_com_with_query_select_exact_path_with_name_and_index_selectors_with_buffered_input_and_count_result_using_recursive_engine(
) -> Result<(), Box<dyn Error>> {
    println ! ("on document jsonpath_com_example.toml running the query $.phoneNumbers[0].type (select exact path with name and index selectors) with Input impl BufferedInput and result mode CountResult");
    let jsonpath_query = JsonPathQuery::parse("$.phoneNumbers[0].type")?;
    let read_string = ReadString :: new ("{\n    \"firstName\": \"John\",\n    \"lastName\": \"doe\",\n    \"age\": 26,\n    \"address\": {\n        \"streetAddress\": \"naist street\",\n        \"city\": \"Nara\",\n        \"postalCode\": \"630-0192\"\n    },\n    \"phoneNumbers\": [\n        {\n            \"type\": \"iPhone\",\n            \"number\": \"0123-4567-8888\"\n        },\n        {\n            \"type\": \"home\",\n            \"number\": \"0123-4567-8910\"\n        }\n    ]\n}\n" . to_string ()) ;
    let input = BufferedInput::new(read_string);
    let engine = RecursiveEngine::compile_query(&jsonpath_query)?;
    let result = engine.run::<_, CountResult>(&input)?;
    assert_eq!(result.get(), 1u64, "result != expected");
    Ok(())
}
#[test]
fn short_json_with_objects_and_lists_given_as_an_example_on_jsonpath_com_with_query_select_exact_path_with_name_and_index_selectors_with_buffered_input_and_index_result_using_main_engine(
) -> Result<(), Box<dyn Error>> {
    println ! ("on document jsonpath_com_example.toml running the query $.phoneNumbers[0].type (select exact path with name and index selectors) with Input impl BufferedInput and result mode IndexResult");
    let jsonpath_query = JsonPathQuery::parse("$.phoneNumbers[0].type")?;
    let read_string = ReadString :: new ("{\n    \"firstName\": \"John\",\n    \"lastName\": \"doe\",\n    \"age\": 26,\n    \"address\": {\n        \"streetAddress\": \"naist street\",\n        \"city\": \"Nara\",\n        \"postalCode\": \"630-0192\"\n    },\n    \"phoneNumbers\": [\n        {\n            \"type\": \"iPhone\",\n            \"number\": \"0123-4567-8888\"\n        },\n        {\n            \"type\": \"home\",\n            \"number\": \"0123-4567-8910\"\n        }\n    ]\n}\n" . to_string ()) ;
    let input = BufferedInput::new(read_string);
    let engine = MainEngine::compile_query(&jsonpath_query)?;
    let result = engine.run::<_, IndexResult>(&input)?;
    assert_eq!(result.get(), vec![239usize,], "result != expected");
    Ok(())
}
#[test]
fn short_json_with_objects_and_lists_given_as_an_example_on_jsonpath_com_with_query_select_exact_path_with_name_and_index_selectors_with_buffered_input_and_index_result_using_recursive_engine(
) -> Result<(), Box<dyn Error>> {
    println ! ("on document jsonpath_com_example.toml running the query $.phoneNumbers[0].type (select exact path with name and index selectors) with Input impl BufferedInput and result mode IndexResult");
    let jsonpath_query = JsonPathQuery::parse("$.phoneNumbers[0].type")?;
    let read_string = ReadString :: new ("{\n    \"firstName\": \"John\",\n    \"lastName\": \"doe\",\n    \"age\": 26,\n    \"address\": {\n        \"streetAddress\": \"naist street\",\n        \"city\": \"Nara\",\n        \"postalCode\": \"630-0192\"\n    },\n    \"phoneNumbers\": [\n        {\n            \"type\": \"iPhone\",\n            \"number\": \"0123-4567-8888\"\n        },\n        {\n            \"type\": \"home\",\n            \"number\": \"0123-4567-8910\"\n        }\n    ]\n}\n" . to_string ()) ;
    let input = BufferedInput::new(read_string);
    let engine = RecursiveEngine::compile_query(&jsonpath_query)?;
    let result = engine.run::<_, IndexResult>(&input)?;
    assert_eq!(result.get(), vec![239usize,], "result != expected");
    Ok(())
}
#[test]
fn short_json_with_objects_and_lists_given_as_an_example_on_jsonpath_com_with_query_select_exact_path_with_name_and_index_selectors_with_mmap_input_and_count_result_using_main_engine(
) -> Result<(), Box<dyn Error>> {
    println ! ("on document jsonpath_com_example.toml running the query $.phoneNumbers[0].type (select exact path with name and index selectors) with Input impl MmapInput and result mode CountResult");
    let jsonpath_query = JsonPathQuery::parse("$.phoneNumbers[0].type")?;
    let tmp_file = mmap_tmp_file :: create_with_contents ("{\n    \"firstName\": \"John\",\n    \"lastName\": \"doe\",\n    \"age\": 26,\n    \"address\": {\n        \"streetAddress\": \"naist street\",\n        \"city\": \"Nara\",\n        \"postalCode\": \"630-0192\"\n    },\n    \"phoneNumbers\": [\n        {\n            \"type\": \"iPhone\",\n            \"number\": \"0123-4567-8888\"\n        },\n        {\n            \"type\": \"home\",\n            \"number\": \"0123-4567-8910\"\n        }\n    ]\n}\n") ? ;
    let input = unsafe { MmapInput::map_file(&tmp_file)? };
    let engine = MainEngine::compile_query(&jsonpath_query)?;
    let result = engine.run::<_, CountResult>(&input)?;
    assert_eq!(result.get(), 1u64, "result != expected");
    Ok(())
}
#[test]
fn short_json_with_objects_and_lists_given_as_an_example_on_jsonpath_com_with_query_select_exact_path_with_name_and_index_selectors_with_mmap_input_and_count_result_using_recursive_engine(
) -> Result<(), Box<dyn Error>> {
    println ! ("on document jsonpath_com_example.toml running the query $.phoneNumbers[0].type (select exact path with name and index selectors) with Input impl MmapInput and result mode CountResult");
    let jsonpath_query = JsonPathQuery::parse("$.phoneNumbers[0].type")?;
    let tmp_file = mmap_tmp_file :: create_with_contents ("{\n    \"firstName\": \"John\",\n    \"lastName\": \"doe\",\n    \"age\": 26,\n    \"address\": {\n        \"streetAddress\": \"naist street\",\n        \"city\": \"Nara\",\n        \"postalCode\": \"630-0192\"\n    },\n    \"phoneNumbers\": [\n        {\n            \"type\": \"iPhone\",\n            \"number\": \"0123-4567-8888\"\n        },\n        {\n            \"type\": \"home\",\n            \"number\": \"0123-4567-8910\"\n        }\n    ]\n}\n") ? ;
    let input = unsafe { MmapInput::map_file(&tmp_file)? };
    let engine = RecursiveEngine::compile_query(&jsonpath_query)?;
    let result = engine.run::<_, CountResult>(&input)?;
    assert_eq!(result.get(), 1u64, "result != expected");
    Ok(())
}
#[test]
fn short_json_with_objects_and_lists_given_as_an_example_on_jsonpath_com_with_query_select_exact_path_with_name_and_index_selectors_with_mmap_input_and_index_result_using_main_engine(
) -> Result<(), Box<dyn Error>> {
    println ! ("on document jsonpath_com_example.toml running the query $.phoneNumbers[0].type (select exact path with name and index selectors) with Input impl MmapInput and result mode IndexResult");
    let jsonpath_query = JsonPathQuery::parse("$.phoneNumbers[0].type")?;
    let tmp_file = mmap_tmp_file :: create_with_contents ("{\n    \"firstName\": \"John\",\n    \"lastName\": \"doe\",\n    \"age\": 26,\n    \"address\": {\n        \"streetAddress\": \"naist street\",\n        \"city\": \"Nara\",\n        \"postalCode\": \"630-0192\"\n    },\n    \"phoneNumbers\": [\n        {\n            \"type\": \"iPhone\",\n            \"number\": \"0123-4567-8888\"\n        },\n        {\n            \"type\": \"home\",\n            \"number\": \"0123-4567-8910\"\n        }\n    ]\n}\n") ? ;
    let input = unsafe { MmapInput::map_file(&tmp_file)? };
    let engine = MainEngine::compile_query(&jsonpath_query)?;
    let result = engine.run::<_, IndexResult>(&input)?;
    assert_eq!(result.get(), vec![239usize,], "result != expected");
    Ok(())
}
#[test]
fn short_json_with_objects_and_lists_given_as_an_example_on_jsonpath_com_with_query_select_exact_path_with_name_and_index_selectors_with_mmap_input_and_index_result_using_recursive_engine(
) -> Result<(), Box<dyn Error>> {
    println ! ("on document jsonpath_com_example.toml running the query $.phoneNumbers[0].type (select exact path with name and index selectors) with Input impl MmapInput and result mode IndexResult");
    let jsonpath_query = JsonPathQuery::parse("$.phoneNumbers[0].type")?;
    let tmp_file = mmap_tmp_file :: create_with_contents ("{\n    \"firstName\": \"John\",\n    \"lastName\": \"doe\",\n    \"age\": 26,\n    \"address\": {\n        \"streetAddress\": \"naist street\",\n        \"city\": \"Nara\",\n        \"postalCode\": \"630-0192\"\n    },\n    \"phoneNumbers\": [\n        {\n            \"type\": \"iPhone\",\n            \"number\": \"0123-4567-8888\"\n        },\n        {\n            \"type\": \"home\",\n            \"number\": \"0123-4567-8910\"\n        }\n    ]\n}\n") ? ;
    let input = unsafe { MmapInput::map_file(&tmp_file)? };
    let engine = RecursiveEngine::compile_query(&jsonpath_query)?;
    let result = engine.run::<_, IndexResult>(&input)?;
    assert_eq!(result.get(), vec![239usize,], "result != expected");
    Ok(())
}
#[test]
fn short_json_with_objects_and_lists_given_as_an_example_on_jsonpath_com_with_query_descendant_search_for_number_with_owned_bytes_and_count_result_using_main_engine(
) -> Result<(), Box<dyn Error>> {
    println ! ("on document jsonpath_com_example.toml running the query $..number (descendant search for 'number') with Input impl OwnedBytes and result mode CountResult");
    let jsonpath_query = JsonPathQuery::parse("$..number")?;
    let raw_json = "{\n    \"firstName\": \"John\",\n    \"lastName\": \"doe\",\n    \"age\": 26,\n    \"address\": {\n        \"streetAddress\": \"naist street\",\n        \"city\": \"Nara\",\n        \"postalCode\": \"630-0192\"\n    },\n    \"phoneNumbers\": [\n        {\n            \"type\": \"iPhone\",\n            \"number\": \"0123-4567-8888\"\n        },\n        {\n            \"type\": \"home\",\n            \"number\": \"0123-4567-8910\"\n        }\n    ]\n}\n" ;
    let input = OwnedBytes::new(&raw_json.as_bytes())?;
    let engine = MainEngine::compile_query(&jsonpath_query)?;
    let result = engine.run::<_, CountResult>(&input)?;
    assert_eq!(result.get(), 2u64, "result != expected");
    Ok(())
}
#[test]
fn short_json_with_objects_and_lists_given_as_an_example_on_jsonpath_com_with_query_descendant_search_for_number_with_owned_bytes_and_count_result_using_recursive_engine(
) -> Result<(), Box<dyn Error>> {
    println ! ("on document jsonpath_com_example.toml running the query $..number (descendant search for 'number') with Input impl OwnedBytes and result mode CountResult");
    let jsonpath_query = JsonPathQuery::parse("$..number")?;
    let raw_json = "{\n    \"firstName\": \"John\",\n    \"lastName\": \"doe\",\n    \"age\": 26,\n    \"address\": {\n        \"streetAddress\": \"naist street\",\n        \"city\": \"Nara\",\n        \"postalCode\": \"630-0192\"\n    },\n    \"phoneNumbers\": [\n        {\n            \"type\": \"iPhone\",\n            \"number\": \"0123-4567-8888\"\n        },\n        {\n            \"type\": \"home\",\n            \"number\": \"0123-4567-8910\"\n        }\n    ]\n}\n" ;
    let input = OwnedBytes::new(&raw_json.as_bytes())?;
    let engine = RecursiveEngine::compile_query(&jsonpath_query)?;
    let result = engine.run::<_, CountResult>(&input)?;
    assert_eq!(result.get(), 2u64, "result != expected");
    Ok(())
}
#[test]
fn short_json_with_objects_and_lists_given_as_an_example_on_jsonpath_com_with_query_descendant_search_for_number_with_owned_bytes_and_index_result_using_main_engine(
) -> Result<(), Box<dyn Error>> {
    println ! ("on document jsonpath_com_example.toml running the query $..number (descendant search for 'number') with Input impl OwnedBytes and result mode IndexResult");
    let jsonpath_query = JsonPathQuery::parse("$..number")?;
    let raw_json = "{\n    \"firstName\": \"John\",\n    \"lastName\": \"doe\",\n    \"age\": 26,\n    \"address\": {\n        \"streetAddress\": \"naist street\",\n        \"city\": \"Nara\",\n        \"postalCode\": \"630-0192\"\n    },\n    \"phoneNumbers\": [\n        {\n            \"type\": \"iPhone\",\n            \"number\": \"0123-4567-8888\"\n        },\n        {\n            \"type\": \"home\",\n            \"number\": \"0123-4567-8910\"\n        }\n    ]\n}\n" ;
    let input = OwnedBytes::new(&raw_json.as_bytes())?;
    let engine = MainEngine::compile_query(&jsonpath_query)?;
    let result = engine.run::<_, IndexResult>(&input)?;
    assert_eq!(result.get(), vec![271usize, 359usize,], "result != expected");
    Ok(())
}
#[test]
fn short_json_with_objects_and_lists_given_as_an_example_on_jsonpath_com_with_query_descendant_search_for_number_with_owned_bytes_and_index_result_using_recursive_engine(
) -> Result<(), Box<dyn Error>> {
    println ! ("on document jsonpath_com_example.toml running the query $..number (descendant search for 'number') with Input impl OwnedBytes and result mode IndexResult");
    let jsonpath_query = JsonPathQuery::parse("$..number")?;
    let raw_json = "{\n    \"firstName\": \"John\",\n    \"lastName\": \"doe\",\n    \"age\": 26,\n    \"address\": {\n        \"streetAddress\": \"naist street\",\n        \"city\": \"Nara\",\n        \"postalCode\": \"630-0192\"\n    },\n    \"phoneNumbers\": [\n        {\n            \"type\": \"iPhone\",\n            \"number\": \"0123-4567-8888\"\n        },\n        {\n            \"type\": \"home\",\n            \"number\": \"0123-4567-8910\"\n        }\n    ]\n}\n" ;
    let input = OwnedBytes::new(&raw_json.as_bytes())?;
    let engine = RecursiveEngine::compile_query(&jsonpath_query)?;
    let result = engine.run::<_, IndexResult>(&input)?;
    assert_eq!(result.get(), vec![271usize, 359usize,], "result != expected");
    Ok(())
}
#[test]
fn short_json_with_objects_and_lists_given_as_an_example_on_jsonpath_com_with_query_descendant_search_for_number_with_buffered_input_and_count_result_using_main_engine(
) -> Result<(), Box<dyn Error>> {
    println ! ("on document jsonpath_com_example.toml running the query $..number (descendant search for 'number') with Input impl BufferedInput and result mode CountResult");
    let jsonpath_query = JsonPathQuery::parse("$..number")?;
    let read_string = ReadString :: new ("{\n    \"firstName\": \"John\",\n    \"lastName\": \"doe\",\n    \"age\": 26,\n    \"address\": {\n        \"streetAddress\": \"naist street\",\n        \"city\": \"Nara\",\n        \"postalCode\": \"630-0192\"\n    },\n    \"phoneNumbers\": [\n        {\n            \"type\": \"iPhone\",\n            \"number\": \"0123-4567-8888\"\n        },\n        {\n            \"type\": \"home\",\n            \"number\": \"0123-4567-8910\"\n        }\n    ]\n}\n" . to_string ()) ;
    let input = BufferedInput::new(read_string);
    let engine = MainEngine::compile_query(&jsonpath_query)?;
    let result = engine.run::<_, CountResult>(&input)?;
    assert_eq!(result.get(), 2u64, "result != expected");
    Ok(())
}
#[test]
fn short_json_with_objects_and_lists_given_as_an_example_on_jsonpath_com_with_query_descendant_search_for_number_with_buffered_input_and_count_result_using_recursive_engine(
) -> Result<(), Box<dyn Error>> {
    println ! ("on document jsonpath_com_example.toml running the query $..number (descendant search for 'number') with Input impl BufferedInput and result mode CountResult");
    let jsonpath_query = JsonPathQuery::parse("$..number")?;
    let read_string = ReadString :: new ("{\n    \"firstName\": \"John\",\n    \"lastName\": \"doe\",\n    \"age\": 26,\n    \"address\": {\n        \"streetAddress\": \"naist street\",\n        \"city\": \"Nara\",\n        \"postalCode\": \"630-0192\"\n    },\n    \"phoneNumbers\": [\n        {\n            \"type\": \"iPhone\",\n            \"number\": \"0123-4567-8888\"\n        },\n        {\n            \"type\": \"home\",\n            \"number\": \"0123-4567-8910\"\n        }\n    ]\n}\n" . to_string ()) ;
    let input = BufferedInput::new(read_string);
    let engine = RecursiveEngine::compile_query(&jsonpath_query)?;
    let result = engine.run::<_, CountResult>(&input)?;
    assert_eq!(result.get(), 2u64, "result != expected");
    Ok(())
}
#[test]
fn short_json_with_objects_and_lists_given_as_an_example_on_jsonpath_com_with_query_descendant_search_for_number_with_buffered_input_and_index_result_using_main_engine(
) -> Result<(), Box<dyn Error>> {
    println ! ("on document jsonpath_com_example.toml running the query $..number (descendant search for 'number') with Input impl BufferedInput and result mode IndexResult");
    let jsonpath_query = JsonPathQuery::parse("$..number")?;
    let read_string = ReadString :: new ("{\n    \"firstName\": \"John\",\n    \"lastName\": \"doe\",\n    \"age\": 26,\n    \"address\": {\n        \"streetAddress\": \"naist street\",\n        \"city\": \"Nara\",\n        \"postalCode\": \"630-0192\"\n    },\n    \"phoneNumbers\": [\n        {\n            \"type\": \"iPhone\",\n            \"number\": \"0123-4567-8888\"\n        },\n        {\n            \"type\": \"home\",\n            \"number\": \"0123-4567-8910\"\n        }\n    ]\n}\n" . to_string ()) ;
    let input = BufferedInput::new(read_string);
    let engine = MainEngine::compile_query(&jsonpath_query)?;
    let result = engine.run::<_, IndexResult>(&input)?;
    assert_eq!(result.get(), vec![271usize, 359usize,], "result != expected");
    Ok(())
}
#[test]
fn short_json_with_objects_and_lists_given_as_an_example_on_jsonpath_com_with_query_descendant_search_for_number_with_buffered_input_and_index_result_using_recursive_engine(
) -> Result<(), Box<dyn Error>> {
    println ! ("on document jsonpath_com_example.toml running the query $..number (descendant search for 'number') with Input impl BufferedInput and result mode IndexResult");
    let jsonpath_query = JsonPathQuery::parse("$..number")?;
    let read_string = ReadString :: new ("{\n    \"firstName\": \"John\",\n    \"lastName\": \"doe\",\n    \"age\": 26,\n    \"address\": {\n        \"streetAddress\": \"naist street\",\n        \"city\": \"Nara\",\n        \"postalCode\": \"630-0192\"\n    },\n    \"phoneNumbers\": [\n        {\n            \"type\": \"iPhone\",\n            \"number\": \"0123-4567-8888\"\n        },\n        {\n            \"type\": \"home\",\n            \"number\": \"0123-4567-8910\"\n        }\n    ]\n}\n" . to_string ()) ;
    let input = BufferedInput::new(read_string);
    let engine = RecursiveEngine::compile_query(&jsonpath_query)?;
    let result = engine.run::<_, IndexResult>(&input)?;
    assert_eq!(result.get(), vec![271usize, 359usize,], "result != expected");
    Ok(())
}
#[test]
fn short_json_with_objects_and_lists_given_as_an_example_on_jsonpath_com_with_query_descendant_search_for_number_with_mmap_input_and_count_result_using_main_engine(
) -> Result<(), Box<dyn Error>> {
    println ! ("on document jsonpath_com_example.toml running the query $..number (descendant search for 'number') with Input impl MmapInput and result mode CountResult");
    let jsonpath_query = JsonPathQuery::parse("$..number")?;
    let tmp_file = mmap_tmp_file :: create_with_contents ("{\n    \"firstName\": \"John\",\n    \"lastName\": \"doe\",\n    \"age\": 26,\n    \"address\": {\n        \"streetAddress\": \"naist street\",\n        \"city\": \"Nara\",\n        \"postalCode\": \"630-0192\"\n    },\n    \"phoneNumbers\": [\n        {\n            \"type\": \"iPhone\",\n            \"number\": \"0123-4567-8888\"\n        },\n        {\n            \"type\": \"home\",\n            \"number\": \"0123-4567-8910\"\n        }\n    ]\n}\n") ? ;
    let input = unsafe { MmapInput::map_file(&tmp_file)? };
    let engine = MainEngine::compile_query(&jsonpath_query)?;
    let result = engine.run::<_, CountResult>(&input)?;
    assert_eq!(result.get(), 2u64, "result != expected");
    Ok(())
}
#[test]
fn short_json_with_objects_and_lists_given_as_an_example_on_jsonpath_com_with_query_descendant_search_for_number_with_mmap_input_and_count_result_using_recursive_engine(
) -> Result<(), Box<dyn Error>> {
    println ! ("on document jsonpath_com_example.toml running the query $..number (descendant search for 'number') with Input impl MmapInput and result mode CountResult");
    let jsonpath_query = JsonPathQuery::parse("$..number")?;
    let tmp_file = mmap_tmp_file :: create_with_contents ("{\n    \"firstName\": \"John\",\n    \"lastName\": \"doe\",\n    \"age\": 26,\n    \"address\": {\n        \"streetAddress\": \"naist street\",\n        \"city\": \"Nara\",\n        \"postalCode\": \"630-0192\"\n    },\n    \"phoneNumbers\": [\n        {\n            \"type\": \"iPhone\",\n            \"number\": \"0123-4567-8888\"\n        },\n        {\n            \"type\": \"home\",\n            \"number\": \"0123-4567-8910\"\n        }\n    ]\n}\n") ? ;
    let input = unsafe { MmapInput::map_file(&tmp_file)? };
    let engine = RecursiveEngine::compile_query(&jsonpath_query)?;
    let result = engine.run::<_, CountResult>(&input)?;
    assert_eq!(result.get(), 2u64, "result != expected");
    Ok(())
}
#[test]
fn short_json_with_objects_and_lists_given_as_an_example_on_jsonpath_com_with_query_descendant_search_for_number_with_mmap_input_and_index_result_using_main_engine(
) -> Result<(), Box<dyn Error>> {
    println ! ("on document jsonpath_com_example.toml running the query $..number (descendant search for 'number') with Input impl MmapInput and result mode IndexResult");
    let jsonpath_query = JsonPathQuery::parse("$..number")?;
    let tmp_file = mmap_tmp_file :: create_with_contents ("{\n    \"firstName\": \"John\",\n    \"lastName\": \"doe\",\n    \"age\": 26,\n    \"address\": {\n        \"streetAddress\": \"naist street\",\n        \"city\": \"Nara\",\n        \"postalCode\": \"630-0192\"\n    },\n    \"phoneNumbers\": [\n        {\n            \"type\": \"iPhone\",\n            \"number\": \"0123-4567-8888\"\n        },\n        {\n            \"type\": \"home\",\n            \"number\": \"0123-4567-8910\"\n        }\n    ]\n}\n") ? ;
    let input = unsafe { MmapInput::map_file(&tmp_file)? };
    let engine = MainEngine::compile_query(&jsonpath_query)?;
    let result = engine.run::<_, IndexResult>(&input)?;
    assert_eq!(result.get(), vec![271usize, 359usize,], "result != expected");
    Ok(())
}
#[test]
fn short_json_with_objects_and_lists_given_as_an_example_on_jsonpath_com_with_query_descendant_search_for_number_with_mmap_input_and_index_result_using_recursive_engine(
) -> Result<(), Box<dyn Error>> {
    println ! ("on document jsonpath_com_example.toml running the query $..number (descendant search for 'number') with Input impl MmapInput and result mode IndexResult");
    let jsonpath_query = JsonPathQuery::parse("$..number")?;
    let tmp_file = mmap_tmp_file :: create_with_contents ("{\n    \"firstName\": \"John\",\n    \"lastName\": \"doe\",\n    \"age\": 26,\n    \"address\": {\n        \"streetAddress\": \"naist street\",\n        \"city\": \"Nara\",\n        \"postalCode\": \"630-0192\"\n    },\n    \"phoneNumbers\": [\n        {\n            \"type\": \"iPhone\",\n            \"number\": \"0123-4567-8888\"\n        },\n        {\n            \"type\": \"home\",\n            \"number\": \"0123-4567-8910\"\n        }\n    ]\n}\n") ? ;
    let input = unsafe { MmapInput::map_file(&tmp_file)? };
    let engine = RecursiveEngine::compile_query(&jsonpath_query)?;
    let result = engine.run::<_, IndexResult>(&input)?;
    assert_eq!(result.get(), vec![271usize, 359usize,], "result != expected");
    Ok(())
}
#[test]
fn empty_array_root_with_query_select_the_root_empty_query_with_owned_bytes_and_count_result_using_main_engine(
) -> Result<(), Box<dyn Error>> {
    println ! ("on document empty_array.toml running the query  (select the root (empty query)) with Input impl OwnedBytes and result mode CountResult");
    let jsonpath_query = JsonPathQuery::parse("")?;
    let raw_json = "[]\n";
    let input = OwnedBytes::new(&raw_json.as_bytes())?;
    let engine = MainEngine::compile_query(&jsonpath_query)?;
    let result = engine.run::<_, CountResult>(&input)?;
    assert_eq!(result.get(), 1u64, "result != expected");
    Ok(())
}
#[test]
fn empty_array_root_with_query_select_the_root_empty_query_with_owned_bytes_and_count_result_using_recursive_engine(
) -> Result<(), Box<dyn Error>> {
    println ! ("on document empty_array.toml running the query  (select the root (empty query)) with Input impl OwnedBytes and result mode CountResult");
    let jsonpath_query = JsonPathQuery::parse("")?;
    let raw_json = "[]\n";
    let input = OwnedBytes::new(&raw_json.as_bytes())?;
    let engine = RecursiveEngine::compile_query(&jsonpath_query)?;
    let result = engine.run::<_, CountResult>(&input)?;
    assert_eq!(result.get(), 1u64, "result != expected");
    Ok(())
}
#[test]
fn empty_array_root_with_query_select_the_root_empty_query_with_owned_bytes_and_index_result_using_main_engine(
) -> Result<(), Box<dyn Error>> {
    println ! ("on document empty_array.toml running the query  (select the root (empty query)) with Input impl OwnedBytes and result mode IndexResult");
    let jsonpath_query = JsonPathQuery::parse("")?;
    let raw_json = "[]\n";
    let input = OwnedBytes::new(&raw_json.as_bytes())?;
    let engine = MainEngine::compile_query(&jsonpath_query)?;
    let result = engine.run::<_, IndexResult>(&input)?;
    assert_eq!(result.get(), vec![0usize,], "result != expected");
    Ok(())
}
#[test]
fn empty_array_root_with_query_select_the_root_empty_query_with_owned_bytes_and_index_result_using_recursive_engine(
) -> Result<(), Box<dyn Error>> {
    println ! ("on document empty_array.toml running the query  (select the root (empty query)) with Input impl OwnedBytes and result mode IndexResult");
    let jsonpath_query = JsonPathQuery::parse("")?;
    let raw_json = "[]\n";
    let input = OwnedBytes::new(&raw_json.as_bytes())?;
    let engine = RecursiveEngine::compile_query(&jsonpath_query)?;
    let result = engine.run::<_, IndexResult>(&input)?;
    assert_eq!(result.get(), vec![0usize,], "result != expected");
    Ok(())
}
#[test]
fn empty_array_root_with_query_select_the_root_empty_query_with_buffered_input_and_count_result_using_main_engine(
) -> Result<(), Box<dyn Error>> {
    println ! ("on document empty_array.toml running the query  (select the root (empty query)) with Input impl BufferedInput and result mode CountResult");
    let jsonpath_query = JsonPathQuery::parse("")?;
    let read_string = ReadString::new("[]\n".to_string());
    let input = BufferedInput::new(read_string);
    let engine = MainEngine::compile_query(&jsonpath_query)?;
    let result = engine.run::<_, CountResult>(&input)?;
    assert_eq!(result.get(), 1u64, "result != expected");
    Ok(())
}
#[test]
fn empty_array_root_with_query_select_the_root_empty_query_with_buffered_input_and_count_result_using_recursive_engine(
) -> Result<(), Box<dyn Error>> {
    println ! ("on document empty_array.toml running the query  (select the root (empty query)) with Input impl BufferedInput and result mode CountResult");
    let jsonpath_query = JsonPathQuery::parse("")?;
    let read_string = ReadString::new("[]\n".to_string());
    let input = BufferedInput::new(read_string);
    let engine = RecursiveEngine::compile_query(&jsonpath_query)?;
    let result = engine.run::<_, CountResult>(&input)?;
    assert_eq!(result.get(), 1u64, "result != expected");
    Ok(())
}
#[test]
fn empty_array_root_with_query_select_the_root_empty_query_with_buffered_input_and_index_result_using_main_engine(
) -> Result<(), Box<dyn Error>> {
    println ! ("on document empty_array.toml running the query  (select the root (empty query)) with Input impl BufferedInput and result mode IndexResult");
    let jsonpath_query = JsonPathQuery::parse("")?;
    let read_string = ReadString::new("[]\n".to_string());
    let input = BufferedInput::new(read_string);
    let engine = MainEngine::compile_query(&jsonpath_query)?;
    let result = engine.run::<_, IndexResult>(&input)?;
    assert_eq!(result.get(), vec![0usize,], "result != expected");
    Ok(())
}
#[test]
fn empty_array_root_with_query_select_the_root_empty_query_with_buffered_input_and_index_result_using_recursive_engine(
) -> Result<(), Box<dyn Error>> {
    println ! ("on document empty_array.toml running the query  (select the root (empty query)) with Input impl BufferedInput and result mode IndexResult");
    let jsonpath_query = JsonPathQuery::parse("")?;
    let read_string = ReadString::new("[]\n".to_string());
    let input = BufferedInput::new(read_string);
    let engine = RecursiveEngine::compile_query(&jsonpath_query)?;
    let result = engine.run::<_, IndexResult>(&input)?;
    assert_eq!(result.get(), vec![0usize,], "result != expected");
    Ok(())
}
#[test]
fn empty_array_root_with_query_select_the_root_empty_query_with_mmap_input_and_count_result_using_main_engine(
) -> Result<(), Box<dyn Error>> {
    println ! ("on document empty_array.toml running the query  (select the root (empty query)) with Input impl MmapInput and result mode CountResult");
    let jsonpath_query = JsonPathQuery::parse("")?;
    let tmp_file = mmap_tmp_file::create_with_contents("[]\n")?;
    let input = unsafe { MmapInput::map_file(&tmp_file)? };
    let engine = MainEngine::compile_query(&jsonpath_query)?;
    let result = engine.run::<_, CountResult>(&input)?;
    assert_eq!(result.get(), 1u64, "result != expected");
    Ok(())
}
#[test]
fn empty_array_root_with_query_select_the_root_empty_query_with_mmap_input_and_count_result_using_recursive_engine(
) -> Result<(), Box<dyn Error>> {
    println ! ("on document empty_array.toml running the query  (select the root (empty query)) with Input impl MmapInput and result mode CountResult");
    let jsonpath_query = JsonPathQuery::parse("")?;
    let tmp_file = mmap_tmp_file::create_with_contents("[]\n")?;
    let input = unsafe { MmapInput::map_file(&tmp_file)? };
    let engine = RecursiveEngine::compile_query(&jsonpath_query)?;
    let result = engine.run::<_, CountResult>(&input)?;
    assert_eq!(result.get(), 1u64, "result != expected");
    Ok(())
}
#[test]
fn empty_array_root_with_query_select_the_root_empty_query_with_mmap_input_and_index_result_using_main_engine(
) -> Result<(), Box<dyn Error>> {
    println ! ("on document empty_array.toml running the query  (select the root (empty query)) with Input impl MmapInput and result mode IndexResult");
    let jsonpath_query = JsonPathQuery::parse("")?;
    let tmp_file = mmap_tmp_file::create_with_contents("[]\n")?;
    let input = unsafe { MmapInput::map_file(&tmp_file)? };
    let engine = MainEngine::compile_query(&jsonpath_query)?;
    let result = engine.run::<_, IndexResult>(&input)?;
    assert_eq!(result.get(), vec![0usize,], "result != expected");
    Ok(())
}
#[test]
fn empty_array_root_with_query_select_the_root_empty_query_with_mmap_input_and_index_result_using_recursive_engine(
) -> Result<(), Box<dyn Error>> {
    println ! ("on document empty_array.toml running the query  (select the root (empty query)) with Input impl MmapInput and result mode IndexResult");
    let jsonpath_query = JsonPathQuery::parse("")?;
    let tmp_file = mmap_tmp_file::create_with_contents("[]\n")?;
    let input = unsafe { MmapInput::map_file(&tmp_file)? };
    let engine = RecursiveEngine::compile_query(&jsonpath_query)?;
    let result = engine.run::<_, IndexResult>(&input)?;
    assert_eq!(result.get(), vec![0usize,], "result != expected");
    Ok(())
}
#[test]
fn empty_array_root_with_query_select_the_root_with_owned_bytes_and_count_result_using_main_engine(
) -> Result<(), Box<dyn Error>> {
    println ! ("on document empty_array.toml running the query $ (select the root) with Input impl OwnedBytes and result mode CountResult");
    let jsonpath_query = JsonPathQuery::parse("$")?;
    let raw_json = "[]\n";
    let input = OwnedBytes::new(&raw_json.as_bytes())?;
    let engine = MainEngine::compile_query(&jsonpath_query)?;
    let result = engine.run::<_, CountResult>(&input)?;
    assert_eq!(result.get(), 1u64, "result != expected");
    Ok(())
}
#[test]
fn empty_array_root_with_query_select_the_root_with_owned_bytes_and_count_result_using_recursive_engine(
) -> Result<(), Box<dyn Error>> {
    println ! ("on document empty_array.toml running the query $ (select the root) with Input impl OwnedBytes and result mode CountResult");
    let jsonpath_query = JsonPathQuery::parse("$")?;
    let raw_json = "[]\n";
    let input = OwnedBytes::new(&raw_json.as_bytes())?;
    let engine = RecursiveEngine::compile_query(&jsonpath_query)?;
    let result = engine.run::<_, CountResult>(&input)?;
    assert_eq!(result.get(), 1u64, "result != expected");
    Ok(())
}
#[test]
fn empty_array_root_with_query_select_the_root_with_owned_bytes_and_index_result_using_main_engine(
) -> Result<(), Box<dyn Error>> {
    println ! ("on document empty_array.toml running the query $ (select the root) with Input impl OwnedBytes and result mode IndexResult");
    let jsonpath_query = JsonPathQuery::parse("$")?;
    let raw_json = "[]\n";
    let input = OwnedBytes::new(&raw_json.as_bytes())?;
    let engine = MainEngine::compile_query(&jsonpath_query)?;
    let result = engine.run::<_, IndexResult>(&input)?;
    assert_eq!(result.get(), vec![0usize,], "result != expected");
    Ok(())
}
#[test]
fn empty_array_root_with_query_select_the_root_with_owned_bytes_and_index_result_using_recursive_engine(
) -> Result<(), Box<dyn Error>> {
    println ! ("on document empty_array.toml running the query $ (select the root) with Input impl OwnedBytes and result mode IndexResult");
    let jsonpath_query = JsonPathQuery::parse("$")?;
    let raw_json = "[]\n";
    let input = OwnedBytes::new(&raw_json.as_bytes())?;
    let engine = RecursiveEngine::compile_query(&jsonpath_query)?;
    let result = engine.run::<_, IndexResult>(&input)?;
    assert_eq!(result.get(), vec![0usize,], "result != expected");
    Ok(())
}
#[test]
fn empty_array_root_with_query_select_the_root_with_buffered_input_and_count_result_using_main_engine(
) -> Result<(), Box<dyn Error>> {
    println ! ("on document empty_array.toml running the query $ (select the root) with Input impl BufferedInput and result mode CountResult");
    let jsonpath_query = JsonPathQuery::parse("$")?;
    let read_string = ReadString::new("[]\n".to_string());
    let input = BufferedInput::new(read_string);
    let engine = MainEngine::compile_query(&jsonpath_query)?;
    let result = engine.run::<_, CountResult>(&input)?;
    assert_eq!(result.get(), 1u64, "result != expected");
    Ok(())
}
#[test]
fn empty_array_root_with_query_select_the_root_with_buffered_input_and_count_result_using_recursive_engine(
) -> Result<(), Box<dyn Error>> {
    println ! ("on document empty_array.toml running the query $ (select the root) with Input impl BufferedInput and result mode CountResult");
    let jsonpath_query = JsonPathQuery::parse("$")?;
    let read_string = ReadString::new("[]\n".to_string());
    let input = BufferedInput::new(read_string);
    let engine = RecursiveEngine::compile_query(&jsonpath_query)?;
    let result = engine.run::<_, CountResult>(&input)?;
    assert_eq!(result.get(), 1u64, "result != expected");
    Ok(())
}
#[test]
fn empty_array_root_with_query_select_the_root_with_buffered_input_and_index_result_using_main_engine(
) -> Result<(), Box<dyn Error>> {
    println ! ("on document empty_array.toml running the query $ (select the root) with Input impl BufferedInput and result mode IndexResult");
    let jsonpath_query = JsonPathQuery::parse("$")?;
    let read_string = ReadString::new("[]\n".to_string());
    let input = BufferedInput::new(read_string);
    let engine = MainEngine::compile_query(&jsonpath_query)?;
    let result = engine.run::<_, IndexResult>(&input)?;
    assert_eq!(result.get(), vec![0usize,], "result != expected");
    Ok(())
}
#[test]
fn empty_array_root_with_query_select_the_root_with_buffered_input_and_index_result_using_recursive_engine(
) -> Result<(), Box<dyn Error>> {
    println ! ("on document empty_array.toml running the query $ (select the root) with Input impl BufferedInput and result mode IndexResult");
    let jsonpath_query = JsonPathQuery::parse("$")?;
    let read_string = ReadString::new("[]\n".to_string());
    let input = BufferedInput::new(read_string);
    let engine = RecursiveEngine::compile_query(&jsonpath_query)?;
    let result = engine.run::<_, IndexResult>(&input)?;
    assert_eq!(result.get(), vec![0usize,], "result != expected");
    Ok(())
}
#[test]
fn empty_array_root_with_query_select_the_root_with_mmap_input_and_count_result_using_main_engine(
) -> Result<(), Box<dyn Error>> {
    println ! ("on document empty_array.toml running the query $ (select the root) with Input impl MmapInput and result mode CountResult");
    let jsonpath_query = JsonPathQuery::parse("$")?;
    let tmp_file = mmap_tmp_file::create_with_contents("[]\n")?;
    let input = unsafe { MmapInput::map_file(&tmp_file)? };
    let engine = MainEngine::compile_query(&jsonpath_query)?;
    let result = engine.run::<_, CountResult>(&input)?;
    assert_eq!(result.get(), 1u64, "result != expected");
    Ok(())
}
#[test]
fn empty_array_root_with_query_select_the_root_with_mmap_input_and_count_result_using_recursive_engine(
) -> Result<(), Box<dyn Error>> {
    println ! ("on document empty_array.toml running the query $ (select the root) with Input impl MmapInput and result mode CountResult");
    let jsonpath_query = JsonPathQuery::parse("$")?;
    let tmp_file = mmap_tmp_file::create_with_contents("[]\n")?;
    let input = unsafe { MmapInput::map_file(&tmp_file)? };
    let engine = RecursiveEngine::compile_query(&jsonpath_query)?;
    let result = engine.run::<_, CountResult>(&input)?;
    assert_eq!(result.get(), 1u64, "result != expected");
    Ok(())
}
#[test]
fn empty_array_root_with_query_select_the_root_with_mmap_input_and_index_result_using_main_engine(
) -> Result<(), Box<dyn Error>> {
    println ! ("on document empty_array.toml running the query $ (select the root) with Input impl MmapInput and result mode IndexResult");
    let jsonpath_query = JsonPathQuery::parse("$")?;
    let tmp_file = mmap_tmp_file::create_with_contents("[]\n")?;
    let input = unsafe { MmapInput::map_file(&tmp_file)? };
    let engine = MainEngine::compile_query(&jsonpath_query)?;
    let result = engine.run::<_, IndexResult>(&input)?;
    assert_eq!(result.get(), vec![0usize,], "result != expected");
    Ok(())
}
#[test]
fn empty_array_root_with_query_select_the_root_with_mmap_input_and_index_result_using_recursive_engine(
) -> Result<(), Box<dyn Error>> {
    println ! ("on document empty_array.toml running the query $ (select the root) with Input impl MmapInput and result mode IndexResult");
    let jsonpath_query = JsonPathQuery::parse("$")?;
    let tmp_file = mmap_tmp_file::create_with_contents("[]\n")?;
    let input = unsafe { MmapInput::map_file(&tmp_file)? };
    let engine = RecursiveEngine::compile_query(&jsonpath_query)?;
    let result = engine.run::<_, IndexResult>(&input)?;
    assert_eq!(result.get(), vec![0usize,], "result != expected");
    Ok(())
}
#[test]
fn empty_array_root_with_query_select_the_first_item_which_does_not_exist_with_owned_bytes_and_count_result_using_main_engine(
) -> Result<(), Box<dyn Error>> {
    println ! ("on document empty_array.toml running the query $[0] (select the first item (which does not exist)) with Input impl OwnedBytes and result mode CountResult");
    let jsonpath_query = JsonPathQuery::parse("$[0]")?;
    let raw_json = "[]\n";
    let input = OwnedBytes::new(&raw_json.as_bytes())?;
    let engine = MainEngine::compile_query(&jsonpath_query)?;
    let result = engine.run::<_, CountResult>(&input)?;
    assert_eq!(result.get(), 0u64, "result != expected");
    Ok(())
}
#[test]
fn empty_array_root_with_query_select_the_first_item_which_does_not_exist_with_owned_bytes_and_count_result_using_recursive_engine(
) -> Result<(), Box<dyn Error>> {
    println ! ("on document empty_array.toml running the query $[0] (select the first item (which does not exist)) with Input impl OwnedBytes and result mode CountResult");
    let jsonpath_query = JsonPathQuery::parse("$[0]")?;
    let raw_json = "[]\n";
    let input = OwnedBytes::new(&raw_json.as_bytes())?;
    let engine = RecursiveEngine::compile_query(&jsonpath_query)?;
    let result = engine.run::<_, CountResult>(&input)?;
    assert_eq!(result.get(), 0u64, "result != expected");
    Ok(())
}
#[test]
fn empty_array_root_with_query_select_the_first_item_which_does_not_exist_with_owned_bytes_and_index_result_using_main_engine(
) -> Result<(), Box<dyn Error>> {
    println ! ("on document empty_array.toml running the query $[0] (select the first item (which does not exist)) with Input impl OwnedBytes and result mode IndexResult");
    let jsonpath_query = JsonPathQuery::parse("$[0]")?;
    let raw_json = "[]\n";
    let input = OwnedBytes::new(&raw_json.as_bytes())?;
    let engine = MainEngine::compile_query(&jsonpath_query)?;
    let result = engine.run::<_, IndexResult>(&input)?;
    assert_eq!(result.get(), vec![], "result != expected");
    Ok(())
}
#[test]
fn empty_array_root_with_query_select_the_first_item_which_does_not_exist_with_owned_bytes_and_index_result_using_recursive_engine(
) -> Result<(), Box<dyn Error>> {
    println ! ("on document empty_array.toml running the query $[0] (select the first item (which does not exist)) with Input impl OwnedBytes and result mode IndexResult");
    let jsonpath_query = JsonPathQuery::parse("$[0]")?;
    let raw_json = "[]\n";
    let input = OwnedBytes::new(&raw_json.as_bytes())?;
    let engine = RecursiveEngine::compile_query(&jsonpath_query)?;
    let result = engine.run::<_, IndexResult>(&input)?;
    assert_eq!(result.get(), vec![], "result != expected");
    Ok(())
}
#[test]
fn empty_array_root_with_query_select_the_first_item_which_does_not_exist_with_buffered_input_and_count_result_using_main_engine(
) -> Result<(), Box<dyn Error>> {
    println ! ("on document empty_array.toml running the query $[0] (select the first item (which does not exist)) with Input impl BufferedInput and result mode CountResult");
    let jsonpath_query = JsonPathQuery::parse("$[0]")?;
    let read_string = ReadString::new("[]\n".to_string());
    let input = BufferedInput::new(read_string);
    let engine = MainEngine::compile_query(&jsonpath_query)?;
    let result = engine.run::<_, CountResult>(&input)?;
    assert_eq!(result.get(), 0u64, "result != expected");
    Ok(())
}
#[test]
fn empty_array_root_with_query_select_the_first_item_which_does_not_exist_with_buffered_input_and_count_result_using_recursive_engine(
) -> Result<(), Box<dyn Error>> {
    println ! ("on document empty_array.toml running the query $[0] (select the first item (which does not exist)) with Input impl BufferedInput and result mode CountResult");
    let jsonpath_query = JsonPathQuery::parse("$[0]")?;
    let read_string = ReadString::new("[]\n".to_string());
    let input = BufferedInput::new(read_string);
    let engine = RecursiveEngine::compile_query(&jsonpath_query)?;
    let result = engine.run::<_, CountResult>(&input)?;
    assert_eq!(result.get(), 0u64, "result != expected");
    Ok(())
}
#[test]
fn empty_array_root_with_query_select_the_first_item_which_does_not_exist_with_buffered_input_and_index_result_using_main_engine(
) -> Result<(), Box<dyn Error>> {
    println ! ("on document empty_array.toml running the query $[0] (select the first item (which does not exist)) with Input impl BufferedInput and result mode IndexResult");
    let jsonpath_query = JsonPathQuery::parse("$[0]")?;
    let read_string = ReadString::new("[]\n".to_string());
    let input = BufferedInput::new(read_string);
    let engine = MainEngine::compile_query(&jsonpath_query)?;
    let result = engine.run::<_, IndexResult>(&input)?;
    assert_eq!(result.get(), vec![], "result != expected");
    Ok(())
}
#[test]
fn empty_array_root_with_query_select_the_first_item_which_does_not_exist_with_buffered_input_and_index_result_using_recursive_engine(
) -> Result<(), Box<dyn Error>> {
    println ! ("on document empty_array.toml running the query $[0] (select the first item (which does not exist)) with Input impl BufferedInput and result mode IndexResult");
    let jsonpath_query = JsonPathQuery::parse("$[0]")?;
    let read_string = ReadString::new("[]\n".to_string());
    let input = BufferedInput::new(read_string);
    let engine = RecursiveEngine::compile_query(&jsonpath_query)?;
    let result = engine.run::<_, IndexResult>(&input)?;
    assert_eq!(result.get(), vec![], "result != expected");
    Ok(())
}
#[test]
fn empty_array_root_with_query_select_the_first_item_which_does_not_exist_with_mmap_input_and_count_result_using_main_engine(
) -> Result<(), Box<dyn Error>> {
    println ! ("on document empty_array.toml running the query $[0] (select the first item (which does not exist)) with Input impl MmapInput and result mode CountResult");
    let jsonpath_query = JsonPathQuery::parse("$[0]")?;
    let tmp_file = mmap_tmp_file::create_with_contents("[]\n")?;
    let input = unsafe { MmapInput::map_file(&tmp_file)? };
    let engine = MainEngine::compile_query(&jsonpath_query)?;
    let result = engine.run::<_, CountResult>(&input)?;
    assert_eq!(result.get(), 0u64, "result != expected");
    Ok(())
}
#[test]
fn empty_array_root_with_query_select_the_first_item_which_does_not_exist_with_mmap_input_and_count_result_using_recursive_engine(
) -> Result<(), Box<dyn Error>> {
    println ! ("on document empty_array.toml running the query $[0] (select the first item (which does not exist)) with Input impl MmapInput and result mode CountResult");
    let jsonpath_query = JsonPathQuery::parse("$[0]")?;
    let tmp_file = mmap_tmp_file::create_with_contents("[]\n")?;
    let input = unsafe { MmapInput::map_file(&tmp_file)? };
    let engine = RecursiveEngine::compile_query(&jsonpath_query)?;
    let result = engine.run::<_, CountResult>(&input)?;
    assert_eq!(result.get(), 0u64, "result != expected");
    Ok(())
}
#[test]
fn empty_array_root_with_query_select_the_first_item_which_does_not_exist_with_mmap_input_and_index_result_using_main_engine(
) -> Result<(), Box<dyn Error>> {
    println ! ("on document empty_array.toml running the query $[0] (select the first item (which does not exist)) with Input impl MmapInput and result mode IndexResult");
    let jsonpath_query = JsonPathQuery::parse("$[0]")?;
    let tmp_file = mmap_tmp_file::create_with_contents("[]\n")?;
    let input = unsafe { MmapInput::map_file(&tmp_file)? };
    let engine = MainEngine::compile_query(&jsonpath_query)?;
    let result = engine.run::<_, IndexResult>(&input)?;
    assert_eq!(result.get(), vec![], "result != expected");
    Ok(())
}
#[test]
fn empty_array_root_with_query_select_the_first_item_which_does_not_exist_with_mmap_input_and_index_result_using_recursive_engine(
) -> Result<(), Box<dyn Error>> {
    println ! ("on document empty_array.toml running the query $[0] (select the first item (which does not exist)) with Input impl MmapInput and result mode IndexResult");
    let jsonpath_query = JsonPathQuery::parse("$[0]")?;
    let tmp_file = mmap_tmp_file::create_with_contents("[]\n")?;
    let input = unsafe { MmapInput::map_file(&tmp_file)? };
    let engine = RecursiveEngine::compile_query(&jsonpath_query)?;
    let result = engine.run::<_, IndexResult>(&input)?;
    assert_eq!(result.get(), vec![], "result != expected");
    Ok(())
}
#[test]
fn empty_array_root_with_query_select_any_item_there_are_none_with_owned_bytes_and_count_result_using_main_engine(
) -> Result<(), Box<dyn Error>> {
    println ! ("on document empty_array.toml running the query $[*] (select any item (there are none)) with Input impl OwnedBytes and result mode CountResult");
    let jsonpath_query = JsonPathQuery::parse("$[*]")?;
    let raw_json = "[]\n";
    let input = OwnedBytes::new(&raw_json.as_bytes())?;
    let engine = MainEngine::compile_query(&jsonpath_query)?;
    let result = engine.run::<_, CountResult>(&input)?;
    assert_eq!(result.get(), 0u64, "result != expected");
    Ok(())
}
#[test]
fn empty_array_root_with_query_select_any_item_there_are_none_with_owned_bytes_and_count_result_using_recursive_engine(
) -> Result<(), Box<dyn Error>> {
    println ! ("on document empty_array.toml running the query $[*] (select any item (there are none)) with Input impl OwnedBytes and result mode CountResult");
    let jsonpath_query = JsonPathQuery::parse("$[*]")?;
    let raw_json = "[]\n";
    let input = OwnedBytes::new(&raw_json.as_bytes())?;
    let engine = RecursiveEngine::compile_query(&jsonpath_query)?;
    let result = engine.run::<_, CountResult>(&input)?;
    assert_eq!(result.get(), 0u64, "result != expected");
    Ok(())
}
#[test]
fn empty_array_root_with_query_select_any_item_there_are_none_with_owned_bytes_and_index_result_using_main_engine(
) -> Result<(), Box<dyn Error>> {
    println ! ("on document empty_array.toml running the query $[*] (select any item (there are none)) with Input impl OwnedBytes and result mode IndexResult");
    let jsonpath_query = JsonPathQuery::parse("$[*]")?;
    let raw_json = "[]\n";
    let input = OwnedBytes::new(&raw_json.as_bytes())?;
    let engine = MainEngine::compile_query(&jsonpath_query)?;
    let result = engine.run::<_, IndexResult>(&input)?;
    assert_eq!(result.get(), vec![], "result != expected");
    Ok(())
}
#[test]
fn empty_array_root_with_query_select_any_item_there_are_none_with_owned_bytes_and_index_result_using_recursive_engine(
) -> Result<(), Box<dyn Error>> {
    println ! ("on document empty_array.toml running the query $[*] (select any item (there are none)) with Input impl OwnedBytes and result mode IndexResult");
    let jsonpath_query = JsonPathQuery::parse("$[*]")?;
    let raw_json = "[]\n";
    let input = OwnedBytes::new(&raw_json.as_bytes())?;
    let engine = RecursiveEngine::compile_query(&jsonpath_query)?;
    let result = engine.run::<_, IndexResult>(&input)?;
    assert_eq!(result.get(), vec![], "result != expected");
    Ok(())
}
#[test]
fn empty_array_root_with_query_select_any_item_there_are_none_with_buffered_input_and_count_result_using_main_engine(
) -> Result<(), Box<dyn Error>> {
    println ! ("on document empty_array.toml running the query $[*] (select any item (there are none)) with Input impl BufferedInput and result mode CountResult");
    let jsonpath_query = JsonPathQuery::parse("$[*]")?;
    let read_string = ReadString::new("[]\n".to_string());
    let input = BufferedInput::new(read_string);
    let engine = MainEngine::compile_query(&jsonpath_query)?;
    let result = engine.run::<_, CountResult>(&input)?;
    assert_eq!(result.get(), 0u64, "result != expected");
    Ok(())
}
#[test]
fn empty_array_root_with_query_select_any_item_there_are_none_with_buffered_input_and_count_result_using_recursive_engine(
) -> Result<(), Box<dyn Error>> {
    println ! ("on document empty_array.toml running the query $[*] (select any item (there are none)) with Input impl BufferedInput and result mode CountResult");
    let jsonpath_query = JsonPathQuery::parse("$[*]")?;
    let read_string = ReadString::new("[]\n".to_string());
    let input = BufferedInput::new(read_string);
    let engine = RecursiveEngine::compile_query(&jsonpath_query)?;
    let result = engine.run::<_, CountResult>(&input)?;
    assert_eq!(result.get(), 0u64, "result != expected");
    Ok(())
}
#[test]
fn empty_array_root_with_query_select_any_item_there_are_none_with_buffered_input_and_index_result_using_main_engine(
) -> Result<(), Box<dyn Error>> {
    println ! ("on document empty_array.toml running the query $[*] (select any item (there are none)) with Input impl BufferedInput and result mode IndexResult");
    let jsonpath_query = JsonPathQuery::parse("$[*]")?;
    let read_string = ReadString::new("[]\n".to_string());
    let input = BufferedInput::new(read_string);
    let engine = MainEngine::compile_query(&jsonpath_query)?;
    let result = engine.run::<_, IndexResult>(&input)?;
    assert_eq!(result.get(), vec![], "result != expected");
    Ok(())
}
#[test]
fn empty_array_root_with_query_select_any_item_there_are_none_with_buffered_input_and_index_result_using_recursive_engine(
) -> Result<(), Box<dyn Error>> {
    println ! ("on document empty_array.toml running the query $[*] (select any item (there are none)) with Input impl BufferedInput and result mode IndexResult");
    let jsonpath_query = JsonPathQuery::parse("$[*]")?;
    let read_string = ReadString::new("[]\n".to_string());
    let input = BufferedInput::new(read_string);
    let engine = RecursiveEngine::compile_query(&jsonpath_query)?;
    let result = engine.run::<_, IndexResult>(&input)?;
    assert_eq!(result.get(), vec![], "result != expected");
    Ok(())
}
#[test]
fn empty_array_root_with_query_select_any_item_there_are_none_with_mmap_input_and_count_result_using_main_engine(
) -> Result<(), Box<dyn Error>> {
    println ! ("on document empty_array.toml running the query $[*] (select any item (there are none)) with Input impl MmapInput and result mode CountResult");
    let jsonpath_query = JsonPathQuery::parse("$[*]")?;
    let tmp_file = mmap_tmp_file::create_with_contents("[]\n")?;
    let input = unsafe { MmapInput::map_file(&tmp_file)? };
    let engine = MainEngine::compile_query(&jsonpath_query)?;
    let result = engine.run::<_, CountResult>(&input)?;
    assert_eq!(result.get(), 0u64, "result != expected");
    Ok(())
}
#[test]
fn empty_array_root_with_query_select_any_item_there_are_none_with_mmap_input_and_count_result_using_recursive_engine(
) -> Result<(), Box<dyn Error>> {
    println ! ("on document empty_array.toml running the query $[*] (select any item (there are none)) with Input impl MmapInput and result mode CountResult");
    let jsonpath_query = JsonPathQuery::parse("$[*]")?;
    let tmp_file = mmap_tmp_file::create_with_contents("[]\n")?;
    let input = unsafe { MmapInput::map_file(&tmp_file)? };
    let engine = RecursiveEngine::compile_query(&jsonpath_query)?;
    let result = engine.run::<_, CountResult>(&input)?;
    assert_eq!(result.get(), 0u64, "result != expected");
    Ok(())
}
#[test]
fn empty_array_root_with_query_select_any_item_there_are_none_with_mmap_input_and_index_result_using_main_engine(
) -> Result<(), Box<dyn Error>> {
    println ! ("on document empty_array.toml running the query $[*] (select any item (there are none)) with Input impl MmapInput and result mode IndexResult");
    let jsonpath_query = JsonPathQuery::parse("$[*]")?;
    let tmp_file = mmap_tmp_file::create_with_contents("[]\n")?;
    let input = unsafe { MmapInput::map_file(&tmp_file)? };
    let engine = MainEngine::compile_query(&jsonpath_query)?;
    let result = engine.run::<_, IndexResult>(&input)?;
    assert_eq!(result.get(), vec![], "result != expected");
    Ok(())
}
#[test]
fn empty_array_root_with_query_select_any_item_there_are_none_with_mmap_input_and_index_result_using_recursive_engine(
) -> Result<(), Box<dyn Error>> {
    println ! ("on document empty_array.toml running the query $[*] (select any item (there are none)) with Input impl MmapInput and result mode IndexResult");
    let jsonpath_query = JsonPathQuery::parse("$[*]")?;
    let tmp_file = mmap_tmp_file::create_with_contents("[]\n")?;
    let input = unsafe { MmapInput::map_file(&tmp_file)? };
    let engine = RecursiveEngine::compile_query(&jsonpath_query)?;
    let result = engine.run::<_, IndexResult>(&input)?;
    assert_eq!(result.get(), vec![], "result != expected");
    Ok(())
}
#[test]
fn empty_array_root_with_query_select_any_descendant_there_are_none_with_owned_bytes_and_count_result_using_main_engine(
) -> Result<(), Box<dyn Error>> {
    println ! ("on document empty_array.toml running the query $..* (select any descendant (there are none)) with Input impl OwnedBytes and result mode CountResult");
    let jsonpath_query = JsonPathQuery::parse("$..*")?;
    let raw_json = "[]\n";
    let input = OwnedBytes::new(&raw_json.as_bytes())?;
    let engine = MainEngine::compile_query(&jsonpath_query)?;
    let result = engine.run::<_, CountResult>(&input)?;
    assert_eq!(result.get(), 0u64, "result != expected");
    Ok(())
}
#[test]
fn empty_array_root_with_query_select_any_descendant_there_are_none_with_owned_bytes_and_count_result_using_recursive_engine(
) -> Result<(), Box<dyn Error>> {
    println ! ("on document empty_array.toml running the query $..* (select any descendant (there are none)) with Input impl OwnedBytes and result mode CountResult");
    let jsonpath_query = JsonPathQuery::parse("$..*")?;
    let raw_json = "[]\n";
    let input = OwnedBytes::new(&raw_json.as_bytes())?;
    let engine = RecursiveEngine::compile_query(&jsonpath_query)?;
    let result = engine.run::<_, CountResult>(&input)?;
    assert_eq!(result.get(), 0u64, "result != expected");
    Ok(())
}
#[test]
fn empty_array_root_with_query_select_any_descendant_there_are_none_with_owned_bytes_and_index_result_using_main_engine(
) -> Result<(), Box<dyn Error>> {
    println ! ("on document empty_array.toml running the query $..* (select any descendant (there are none)) with Input impl OwnedBytes and result mode IndexResult");
    let jsonpath_query = JsonPathQuery::parse("$..*")?;
    let raw_json = "[]\n";
    let input = OwnedBytes::new(&raw_json.as_bytes())?;
    let engine = MainEngine::compile_query(&jsonpath_query)?;
    let result = engine.run::<_, IndexResult>(&input)?;
    assert_eq!(result.get(), vec![], "result != expected");
    Ok(())
}
#[test]
fn empty_array_root_with_query_select_any_descendant_there_are_none_with_owned_bytes_and_index_result_using_recursive_engine(
) -> Result<(), Box<dyn Error>> {
    println ! ("on document empty_array.toml running the query $..* (select any descendant (there are none)) with Input impl OwnedBytes and result mode IndexResult");
    let jsonpath_query = JsonPathQuery::parse("$..*")?;
    let raw_json = "[]\n";
    let input = OwnedBytes::new(&raw_json.as_bytes())?;
    let engine = RecursiveEngine::compile_query(&jsonpath_query)?;
    let result = engine.run::<_, IndexResult>(&input)?;
    assert_eq!(result.get(), vec![], "result != expected");
    Ok(())
}
#[test]
fn empty_array_root_with_query_select_any_descendant_there_are_none_with_buffered_input_and_count_result_using_main_engine(
) -> Result<(), Box<dyn Error>> {
    println ! ("on document empty_array.toml running the query $..* (select any descendant (there are none)) with Input impl BufferedInput and result mode CountResult");
    let jsonpath_query = JsonPathQuery::parse("$..*")?;
    let read_string = ReadString::new("[]\n".to_string());
    let input = BufferedInput::new(read_string);
    let engine = MainEngine::compile_query(&jsonpath_query)?;
    let result = engine.run::<_, CountResult>(&input)?;
    assert_eq!(result.get(), 0u64, "result != expected");
    Ok(())
}
#[test]
fn empty_array_root_with_query_select_any_descendant_there_are_none_with_buffered_input_and_count_result_using_recursive_engine(
) -> Result<(), Box<dyn Error>> {
    println ! ("on document empty_array.toml running the query $..* (select any descendant (there are none)) with Input impl BufferedInput and result mode CountResult");
    let jsonpath_query = JsonPathQuery::parse("$..*")?;
    let read_string = ReadString::new("[]\n".to_string());
    let input = BufferedInput::new(read_string);
    let engine = RecursiveEngine::compile_query(&jsonpath_query)?;
    let result = engine.run::<_, CountResult>(&input)?;
    assert_eq!(result.get(), 0u64, "result != expected");
    Ok(())
}
#[test]
fn empty_array_root_with_query_select_any_descendant_there_are_none_with_buffered_input_and_index_result_using_main_engine(
) -> Result<(), Box<dyn Error>> {
    println ! ("on document empty_array.toml running the query $..* (select any descendant (there are none)) with Input impl BufferedInput and result mode IndexResult");
    let jsonpath_query = JsonPathQuery::parse("$..*")?;
    let read_string = ReadString::new("[]\n".to_string());
    let input = BufferedInput::new(read_string);
    let engine = MainEngine::compile_query(&jsonpath_query)?;
    let result = engine.run::<_, IndexResult>(&input)?;
    assert_eq!(result.get(), vec![], "result != expected");
    Ok(())
}
#[test]
fn empty_array_root_with_query_select_any_descendant_there_are_none_with_buffered_input_and_index_result_using_recursive_engine(
) -> Result<(), Box<dyn Error>> {
    println ! ("on document empty_array.toml running the query $..* (select any descendant (there are none)) with Input impl BufferedInput and result mode IndexResult");
    let jsonpath_query = JsonPathQuery::parse("$..*")?;
    let read_string = ReadString::new("[]\n".to_string());
    let input = BufferedInput::new(read_string);
    let engine = RecursiveEngine::compile_query(&jsonpath_query)?;
    let result = engine.run::<_, IndexResult>(&input)?;
    assert_eq!(result.get(), vec![], "result != expected");
    Ok(())
}
#[test]
fn empty_array_root_with_query_select_any_descendant_there_are_none_with_mmap_input_and_count_result_using_main_engine(
) -> Result<(), Box<dyn Error>> {
    println ! ("on document empty_array.toml running the query $..* (select any descendant (there are none)) with Input impl MmapInput and result mode CountResult");
    let jsonpath_query = JsonPathQuery::parse("$..*")?;
    let tmp_file = mmap_tmp_file::create_with_contents("[]\n")?;
    let input = unsafe { MmapInput::map_file(&tmp_file)? };
    let engine = MainEngine::compile_query(&jsonpath_query)?;
    let result = engine.run::<_, CountResult>(&input)?;
    assert_eq!(result.get(), 0u64, "result != expected");
    Ok(())
}
#[test]
fn empty_array_root_with_query_select_any_descendant_there_are_none_with_mmap_input_and_count_result_using_recursive_engine(
) -> Result<(), Box<dyn Error>> {
    println ! ("on document empty_array.toml running the query $..* (select any descendant (there are none)) with Input impl MmapInput and result mode CountResult");
    let jsonpath_query = JsonPathQuery::parse("$..*")?;
    let tmp_file = mmap_tmp_file::create_with_contents("[]\n")?;
    let input = unsafe { MmapInput::map_file(&tmp_file)? };
    let engine = RecursiveEngine::compile_query(&jsonpath_query)?;
    let result = engine.run::<_, CountResult>(&input)?;
    assert_eq!(result.get(), 0u64, "result != expected");
    Ok(())
}
#[test]
fn empty_array_root_with_query_select_any_descendant_there_are_none_with_mmap_input_and_index_result_using_main_engine(
) -> Result<(), Box<dyn Error>> {
    println ! ("on document empty_array.toml running the query $..* (select any descendant (there are none)) with Input impl MmapInput and result mode IndexResult");
    let jsonpath_query = JsonPathQuery::parse("$..*")?;
    let tmp_file = mmap_tmp_file::create_with_contents("[]\n")?;
    let input = unsafe { MmapInput::map_file(&tmp_file)? };
    let engine = MainEngine::compile_query(&jsonpath_query)?;
    let result = engine.run::<_, IndexResult>(&input)?;
    assert_eq!(result.get(), vec![], "result != expected");
    Ok(())
}
#[test]
fn empty_array_root_with_query_select_any_descendant_there_are_none_with_mmap_input_and_index_result_using_recursive_engine(
) -> Result<(), Box<dyn Error>> {
    println ! ("on document empty_array.toml running the query $..* (select any descendant (there are none)) with Input impl MmapInput and result mode IndexResult");
    let jsonpath_query = JsonPathQuery::parse("$..*")?;
    let tmp_file = mmap_tmp_file::create_with_contents("[]\n")?;
    let input = unsafe { MmapInput::map_file(&tmp_file)? };
    let engine = RecursiveEngine::compile_query(&jsonpath_query)?;
    let result = engine.run::<_, IndexResult>(&input)?;
    assert_eq!(result.get(), vec![], "result != expected");
    Ok(())
}
#[test]
fn empty_document_with_query_select_the_root_empty_query_with_owned_bytes_and_count_result_using_main_engine(
) -> Result<(), Box<dyn Error>> {
    println ! ("on document empty.toml running the query  (select the root (empty query)) with Input impl OwnedBytes and result mode CountResult");
    let jsonpath_query = JsonPathQuery::parse("")?;
    let raw_json = "";
    let input = OwnedBytes::new(&raw_json.as_bytes())?;
    let engine = MainEngine::compile_query(&jsonpath_query)?;
    let result = engine.run::<_, CountResult>(&input)?;
    assert_eq!(result.get(), 0u64, "result != expected");
    Ok(())
}
#[test]
fn empty_document_with_query_select_the_root_empty_query_with_owned_bytes_and_count_result_using_recursive_engine(
) -> Result<(), Box<dyn Error>> {
    println ! ("on document empty.toml running the query  (select the root (empty query)) with Input impl OwnedBytes and result mode CountResult");
    let jsonpath_query = JsonPathQuery::parse("")?;
    let raw_json = "";
    let input = OwnedBytes::new(&raw_json.as_bytes())?;
    let engine = RecursiveEngine::compile_query(&jsonpath_query)?;
    let result = engine.run::<_, CountResult>(&input)?;
    assert_eq!(result.get(), 0u64, "result != expected");
    Ok(())
}
#[test]
fn empty_document_with_query_select_the_root_empty_query_with_owned_bytes_and_index_result_using_main_engine(
) -> Result<(), Box<dyn Error>> {
    println ! ("on document empty.toml running the query  (select the root (empty query)) with Input impl OwnedBytes and result mode IndexResult");
    let jsonpath_query = JsonPathQuery::parse("")?;
    let raw_json = "";
    let input = OwnedBytes::new(&raw_json.as_bytes())?;
    let engine = MainEngine::compile_query(&jsonpath_query)?;
    let result = engine.run::<_, IndexResult>(&input)?;
    assert_eq!(result.get(), vec![], "result != expected");
    Ok(())
}
#[test]
fn empty_document_with_query_select_the_root_empty_query_with_owned_bytes_and_index_result_using_recursive_engine(
) -> Result<(), Box<dyn Error>> {
    println ! ("on document empty.toml running the query  (select the root (empty query)) with Input impl OwnedBytes and result mode IndexResult");
    let jsonpath_query = JsonPathQuery::parse("")?;
    let raw_json = "";
    let input = OwnedBytes::new(&raw_json.as_bytes())?;
    let engine = RecursiveEngine::compile_query(&jsonpath_query)?;
    let result = engine.run::<_, IndexResult>(&input)?;
    assert_eq!(result.get(), vec![], "result != expected");
    Ok(())
}
#[test]
fn empty_document_with_query_select_the_root_empty_query_with_buffered_input_and_count_result_using_main_engine(
) -> Result<(), Box<dyn Error>> {
    println ! ("on document empty.toml running the query  (select the root (empty query)) with Input impl BufferedInput and result mode CountResult");
    let jsonpath_query = JsonPathQuery::parse("")?;
    let read_string = ReadString::new("".to_string());
    let input = BufferedInput::new(read_string);
    let engine = MainEngine::compile_query(&jsonpath_query)?;
    let result = engine.run::<_, CountResult>(&input)?;
    assert_eq!(result.get(), 0u64, "result != expected");
    Ok(())
}
#[test]
fn empty_document_with_query_select_the_root_empty_query_with_buffered_input_and_count_result_using_recursive_engine(
) -> Result<(), Box<dyn Error>> {
    println ! ("on document empty.toml running the query  (select the root (empty query)) with Input impl BufferedInput and result mode CountResult");
    let jsonpath_query = JsonPathQuery::parse("")?;
    let read_string = ReadString::new("".to_string());
    let input = BufferedInput::new(read_string);
    let engine = RecursiveEngine::compile_query(&jsonpath_query)?;
    let result = engine.run::<_, CountResult>(&input)?;
    assert_eq!(result.get(), 0u64, "result != expected");
    Ok(())
}
#[test]
fn empty_document_with_query_select_the_root_empty_query_with_buffered_input_and_index_result_using_main_engine(
) -> Result<(), Box<dyn Error>> {
    println ! ("on document empty.toml running the query  (select the root (empty query)) with Input impl BufferedInput and result mode IndexResult");
    let jsonpath_query = JsonPathQuery::parse("")?;
    let read_string = ReadString::new("".to_string());
    let input = BufferedInput::new(read_string);
    let engine = MainEngine::compile_query(&jsonpath_query)?;
    let result = engine.run::<_, IndexResult>(&input)?;
    assert_eq!(result.get(), vec![], "result != expected");
    Ok(())
}
#[test]
fn empty_document_with_query_select_the_root_empty_query_with_buffered_input_and_index_result_using_recursive_engine(
) -> Result<(), Box<dyn Error>> {
    println ! ("on document empty.toml running the query  (select the root (empty query)) with Input impl BufferedInput and result mode IndexResult");
    let jsonpath_query = JsonPathQuery::parse("")?;
    let read_string = ReadString::new("".to_string());
    let input = BufferedInput::new(read_string);
    let engine = RecursiveEngine::compile_query(&jsonpath_query)?;
    let result = engine.run::<_, IndexResult>(&input)?;
    assert_eq!(result.get(), vec![], "result != expected");
    Ok(())
}
#[test]
fn empty_document_with_query_select_the_root_empty_query_with_mmap_input_and_count_result_using_main_engine(
) -> Result<(), Box<dyn Error>> {
    println ! ("on document empty.toml running the query  (select the root (empty query)) with Input impl MmapInput and result mode CountResult");
    let jsonpath_query = JsonPathQuery::parse("")?;
    let tmp_file = mmap_tmp_file::create_with_contents("")?;
    let input = unsafe { MmapInput::map_file(&tmp_file)? };
    let engine = MainEngine::compile_query(&jsonpath_query)?;
    let result = engine.run::<_, CountResult>(&input)?;
    assert_eq!(result.get(), 0u64, "result != expected");
    Ok(())
}
#[test]
fn empty_document_with_query_select_the_root_empty_query_with_mmap_input_and_count_result_using_recursive_engine(
) -> Result<(), Box<dyn Error>> {
    println ! ("on document empty.toml running the query  (select the root (empty query)) with Input impl MmapInput and result mode CountResult");
    let jsonpath_query = JsonPathQuery::parse("")?;
    let tmp_file = mmap_tmp_file::create_with_contents("")?;
    let input = unsafe { MmapInput::map_file(&tmp_file)? };
    let engine = RecursiveEngine::compile_query(&jsonpath_query)?;
    let result = engine.run::<_, CountResult>(&input)?;
    assert_eq!(result.get(), 0u64, "result != expected");
    Ok(())
}
#[test]
fn empty_document_with_query_select_the_root_empty_query_with_mmap_input_and_index_result_using_main_engine(
) -> Result<(), Box<dyn Error>> {
    println ! ("on document empty.toml running the query  (select the root (empty query)) with Input impl MmapInput and result mode IndexResult");
    let jsonpath_query = JsonPathQuery::parse("")?;
    let tmp_file = mmap_tmp_file::create_with_contents("")?;
    let input = unsafe { MmapInput::map_file(&tmp_file)? };
    let engine = MainEngine::compile_query(&jsonpath_query)?;
    let result = engine.run::<_, IndexResult>(&input)?;
    assert_eq!(result.get(), vec![], "result != expected");
    Ok(())
}
#[test]
fn empty_document_with_query_select_the_root_empty_query_with_mmap_input_and_index_result_using_recursive_engine(
) -> Result<(), Box<dyn Error>> {
    println ! ("on document empty.toml running the query  (select the root (empty query)) with Input impl MmapInput and result mode IndexResult");
    let jsonpath_query = JsonPathQuery::parse("")?;
    let tmp_file = mmap_tmp_file::create_with_contents("")?;
    let input = unsafe { MmapInput::map_file(&tmp_file)? };
    let engine = RecursiveEngine::compile_query(&jsonpath_query)?;
    let result = engine.run::<_, IndexResult>(&input)?;
    assert_eq!(result.get(), vec![], "result != expected");
    Ok(())
}
#[test]
fn empty_document_with_query_select_the_root_with_owned_bytes_and_count_result_using_main_engine(
) -> Result<(), Box<dyn Error>> {
    println ! ("on document empty.toml running the query $ (select the root) with Input impl OwnedBytes and result mode CountResult");
    let jsonpath_query = JsonPathQuery::parse("$")?;
    let raw_json = "";
    let input = OwnedBytes::new(&raw_json.as_bytes())?;
    let engine = MainEngine::compile_query(&jsonpath_query)?;
    let result = engine.run::<_, CountResult>(&input)?;
    assert_eq!(result.get(), 0u64, "result != expected");
    Ok(())
}
#[test]
fn empty_document_with_query_select_the_root_with_owned_bytes_and_count_result_using_recursive_engine(
) -> Result<(), Box<dyn Error>> {
    println ! ("on document empty.toml running the query $ (select the root) with Input impl OwnedBytes and result mode CountResult");
    let jsonpath_query = JsonPathQuery::parse("$")?;
    let raw_json = "";
    let input = OwnedBytes::new(&raw_json.as_bytes())?;
    let engine = RecursiveEngine::compile_query(&jsonpath_query)?;
    let result = engine.run::<_, CountResult>(&input)?;
    assert_eq!(result.get(), 0u64, "result != expected");
    Ok(())
}
#[test]
fn empty_document_with_query_select_the_root_with_owned_bytes_and_index_result_using_main_engine(
) -> Result<(), Box<dyn Error>> {
    println ! ("on document empty.toml running the query $ (select the root) with Input impl OwnedBytes and result mode IndexResult");
    let jsonpath_query = JsonPathQuery::parse("$")?;
    let raw_json = "";
    let input = OwnedBytes::new(&raw_json.as_bytes())?;
    let engine = MainEngine::compile_query(&jsonpath_query)?;
    let result = engine.run::<_, IndexResult>(&input)?;
    assert_eq!(result.get(), vec![], "result != expected");
    Ok(())
}
#[test]
fn empty_document_with_query_select_the_root_with_owned_bytes_and_index_result_using_recursive_engine(
) -> Result<(), Box<dyn Error>> {
    println ! ("on document empty.toml running the query $ (select the root) with Input impl OwnedBytes and result mode IndexResult");
    let jsonpath_query = JsonPathQuery::parse("$")?;
    let raw_json = "";
    let input = OwnedBytes::new(&raw_json.as_bytes())?;
    let engine = RecursiveEngine::compile_query(&jsonpath_query)?;
    let result = engine.run::<_, IndexResult>(&input)?;
    assert_eq!(result.get(), vec![], "result != expected");
    Ok(())
}
#[test]
fn empty_document_with_query_select_the_root_with_buffered_input_and_count_result_using_main_engine(
) -> Result<(), Box<dyn Error>> {
    println ! ("on document empty.toml running the query $ (select the root) with Input impl BufferedInput and result mode CountResult");
    let jsonpath_query = JsonPathQuery::parse("$")?;
    let read_string = ReadString::new("".to_string());
    let input = BufferedInput::new(read_string);
    let engine = MainEngine::compile_query(&jsonpath_query)?;
    let result = engine.run::<_, CountResult>(&input)?;
    assert_eq!(result.get(), 0u64, "result != expected");
    Ok(())
}
#[test]
fn empty_document_with_query_select_the_root_with_buffered_input_and_count_result_using_recursive_engine(
) -> Result<(), Box<dyn Error>> {
    println ! ("on document empty.toml running the query $ (select the root) with Input impl BufferedInput and result mode CountResult");
    let jsonpath_query = JsonPathQuery::parse("$")?;
    let read_string = ReadString::new("".to_string());
    let input = BufferedInput::new(read_string);
    let engine = RecursiveEngine::compile_query(&jsonpath_query)?;
    let result = engine.run::<_, CountResult>(&input)?;
    assert_eq!(result.get(), 0u64, "result != expected");
    Ok(())
}
#[test]
fn empty_document_with_query_select_the_root_with_buffered_input_and_index_result_using_main_engine(
) -> Result<(), Box<dyn Error>> {
    println ! ("on document empty.toml running the query $ (select the root) with Input impl BufferedInput and result mode IndexResult");
    let jsonpath_query = JsonPathQuery::parse("$")?;
    let read_string = ReadString::new("".to_string());
    let input = BufferedInput::new(read_string);
    let engine = MainEngine::compile_query(&jsonpath_query)?;
    let result = engine.run::<_, IndexResult>(&input)?;
    assert_eq!(result.get(), vec![], "result != expected");
    Ok(())
}
#[test]
fn empty_document_with_query_select_the_root_with_buffered_input_and_index_result_using_recursive_engine(
) -> Result<(), Box<dyn Error>> {
    println ! ("on document empty.toml running the query $ (select the root) with Input impl BufferedInput and result mode IndexResult");
    let jsonpath_query = JsonPathQuery::parse("$")?;
    let read_string = ReadString::new("".to_string());
    let input = BufferedInput::new(read_string);
    let engine = RecursiveEngine::compile_query(&jsonpath_query)?;
    let result = engine.run::<_, IndexResult>(&input)?;
    assert_eq!(result.get(), vec![], "result != expected");
    Ok(())
}
#[test]
fn empty_document_with_query_select_the_root_with_mmap_input_and_count_result_using_main_engine(
) -> Result<(), Box<dyn Error>> {
    println ! ("on document empty.toml running the query $ (select the root) with Input impl MmapInput and result mode CountResult");
    let jsonpath_query = JsonPathQuery::parse("$")?;
    let tmp_file = mmap_tmp_file::create_with_contents("")?;
    let input = unsafe { MmapInput::map_file(&tmp_file)? };
    let engine = MainEngine::compile_query(&jsonpath_query)?;
    let result = engine.run::<_, CountResult>(&input)?;
    assert_eq!(result.get(), 0u64, "result != expected");
    Ok(())
}
#[test]
fn empty_document_with_query_select_the_root_with_mmap_input_and_count_result_using_recursive_engine(
) -> Result<(), Box<dyn Error>> {
    println ! ("on document empty.toml running the query $ (select the root) with Input impl MmapInput and result mode CountResult");
    let jsonpath_query = JsonPathQuery::parse("$")?;
    let tmp_file = mmap_tmp_file::create_with_contents("")?;
    let input = unsafe { MmapInput::map_file(&tmp_file)? };
    let engine = RecursiveEngine::compile_query(&jsonpath_query)?;
    let result = engine.run::<_, CountResult>(&input)?;
    assert_eq!(result.get(), 0u64, "result != expected");
    Ok(())
}
#[test]
fn empty_document_with_query_select_the_root_with_mmap_input_and_index_result_using_main_engine(
) -> Result<(), Box<dyn Error>> {
    println ! ("on document empty.toml running the query $ (select the root) with Input impl MmapInput and result mode IndexResult");
    let jsonpath_query = JsonPathQuery::parse("$")?;
    let tmp_file = mmap_tmp_file::create_with_contents("")?;
    let input = unsafe { MmapInput::map_file(&tmp_file)? };
    let engine = MainEngine::compile_query(&jsonpath_query)?;
    let result = engine.run::<_, IndexResult>(&input)?;
    assert_eq!(result.get(), vec![], "result != expected");
    Ok(())
}
#[test]
fn empty_document_with_query_select_the_root_with_mmap_input_and_index_result_using_recursive_engine(
) -> Result<(), Box<dyn Error>> {
    println ! ("on document empty.toml running the query $ (select the root) with Input impl MmapInput and result mode IndexResult");
    let jsonpath_query = JsonPathQuery::parse("$")?;
    let tmp_file = mmap_tmp_file::create_with_contents("")?;
    let input = unsafe { MmapInput::map_file(&tmp_file)? };
    let engine = RecursiveEngine::compile_query(&jsonpath_query)?;
    let result = engine.run::<_, IndexResult>(&input)?;
    assert_eq!(result.get(), vec![], "result != expected");
    Ok(())
}
#[test]
fn with_query_select_a_number_that_is_a_child_with_owned_bytes_and_count_result_using_main_engine(
) -> Result<(), Box<dyn Error>> {
    println ! ("on document atomic_descendant.toml running the query $..a (select a number that is a child) with Input impl OwnedBytes and result mode CountResult");
    let jsonpath_query = JsonPathQuery::parse("$..a")?;
    let raw_json = "{\n    \"a\": 42,\n    \"b\": [\n        {\n            \"b\": 43\n        }\n    ]\n}\n";
    let input = OwnedBytes::new(&raw_json.as_bytes())?;
    let engine = MainEngine::compile_query(&jsonpath_query)?;
    let result = engine.run::<_, CountResult>(&input)?;
    assert_eq!(result.get(), 1u64, "result != expected");
    Ok(())
}
#[test]
fn with_query_select_a_number_that_is_a_child_with_owned_bytes_and_count_result_using_recursive_engine(
) -> Result<(), Box<dyn Error>> {
    println ! ("on document atomic_descendant.toml running the query $..a (select a number that is a child) with Input impl OwnedBytes and result mode CountResult");
    let jsonpath_query = JsonPathQuery::parse("$..a")?;
    let raw_json = "{\n    \"a\": 42,\n    \"b\": [\n        {\n            \"b\": 43\n        }\n    ]\n}\n";
    let input = OwnedBytes::new(&raw_json.as_bytes())?;
    let engine = RecursiveEngine::compile_query(&jsonpath_query)?;
    let result = engine.run::<_, CountResult>(&input)?;
    assert_eq!(result.get(), 1u64, "result != expected");
    Ok(())
}
#[test]
fn with_query_select_a_number_that_is_a_child_with_owned_bytes_and_index_result_using_main_engine(
) -> Result<(), Box<dyn Error>> {
    println ! ("on document atomic_descendant.toml running the query $..a (select a number that is a child) with Input impl OwnedBytes and result mode IndexResult");
    let jsonpath_query = JsonPathQuery::parse("$..a")?;
    let raw_json = "{\n    \"a\": 42,\n    \"b\": [\n        {\n            \"b\": 43\n        }\n    ]\n}\n";
    let input = OwnedBytes::new(&raw_json.as_bytes())?;
    let engine = MainEngine::compile_query(&jsonpath_query)?;
    let result = engine.run::<_, IndexResult>(&input)?;
    assert_eq!(result.get(), vec![11usize,], "result != expected");
    Ok(())
}
#[test]
fn with_query_select_a_number_that_is_a_child_with_owned_bytes_and_index_result_using_recursive_engine(
) -> Result<(), Box<dyn Error>> {
    println ! ("on document atomic_descendant.toml running the query $..a (select a number that is a child) with Input impl OwnedBytes and result mode IndexResult");
    let jsonpath_query = JsonPathQuery::parse("$..a")?;
    let raw_json = "{\n    \"a\": 42,\n    \"b\": [\n        {\n            \"b\": 43\n        }\n    ]\n}\n";
    let input = OwnedBytes::new(&raw_json.as_bytes())?;
    let engine = RecursiveEngine::compile_query(&jsonpath_query)?;
    let result = engine.run::<_, IndexResult>(&input)?;
    assert_eq!(result.get(), vec![11usize,], "result != expected");
    Ok(())
}
#[test]
fn with_query_select_a_number_that_is_a_child_with_buffered_input_and_count_result_using_main_engine(
) -> Result<(), Box<dyn Error>> {
    println ! ("on document atomic_descendant.toml running the query $..a (select a number that is a child) with Input impl BufferedInput and result mode CountResult");
    let jsonpath_query = JsonPathQuery::parse("$..a")?;
    let read_string = ReadString::new(
        "{\n    \"a\": 42,\n    \"b\": [\n        {\n            \"b\": 43\n        }\n    ]\n}\n".to_string(),
    );
    let input = BufferedInput::new(read_string);
    let engine = MainEngine::compile_query(&jsonpath_query)?;
    let result = engine.run::<_, CountResult>(&input)?;
    assert_eq!(result.get(), 1u64, "result != expected");
    Ok(())
}
#[test]
fn with_query_select_a_number_that_is_a_child_with_buffered_input_and_count_result_using_recursive_engine(
) -> Result<(), Box<dyn Error>> {
    println ! ("on document atomic_descendant.toml running the query $..a (select a number that is a child) with Input impl BufferedInput and result mode CountResult");
    let jsonpath_query = JsonPathQuery::parse("$..a")?;
    let read_string = ReadString::new(
        "{\n    \"a\": 42,\n    \"b\": [\n        {\n            \"b\": 43\n        }\n    ]\n}\n".to_string(),
    );
    let input = BufferedInput::new(read_string);
    let engine = RecursiveEngine::compile_query(&jsonpath_query)?;
    let result = engine.run::<_, CountResult>(&input)?;
    assert_eq!(result.get(), 1u64, "result != expected");
    Ok(())
}
#[test]
fn with_query_select_a_number_that_is_a_child_with_buffered_input_and_index_result_using_main_engine(
) -> Result<(), Box<dyn Error>> {
    println ! ("on document atomic_descendant.toml running the query $..a (select a number that is a child) with Input impl BufferedInput and result mode IndexResult");
    let jsonpath_query = JsonPathQuery::parse("$..a")?;
    let read_string = ReadString::new(
        "{\n    \"a\": 42,\n    \"b\": [\n        {\n            \"b\": 43\n        }\n    ]\n}\n".to_string(),
    );
    let input = BufferedInput::new(read_string);
    let engine = MainEngine::compile_query(&jsonpath_query)?;
    let result = engine.run::<_, IndexResult>(&input)?;
    assert_eq!(result.get(), vec![11usize,], "result != expected");
    Ok(())
}
#[test]
fn with_query_select_a_number_that_is_a_child_with_buffered_input_and_index_result_using_recursive_engine(
) -> Result<(), Box<dyn Error>> {
    println ! ("on document atomic_descendant.toml running the query $..a (select a number that is a child) with Input impl BufferedInput and result mode IndexResult");
    let jsonpath_query = JsonPathQuery::parse("$..a")?;
    let read_string = ReadString::new(
        "{\n    \"a\": 42,\n    \"b\": [\n        {\n            \"b\": 43\n        }\n    ]\n}\n".to_string(),
    );
    let input = BufferedInput::new(read_string);
    let engine = RecursiveEngine::compile_query(&jsonpath_query)?;
    let result = engine.run::<_, IndexResult>(&input)?;
    assert_eq!(result.get(), vec![11usize,], "result != expected");
    Ok(())
}
#[test]
fn with_query_select_a_number_that_is_a_child_with_mmap_input_and_count_result_using_main_engine(
) -> Result<(), Box<dyn Error>> {
    println ! ("on document atomic_descendant.toml running the query $..a (select a number that is a child) with Input impl MmapInput and result mode CountResult");
    let jsonpath_query = JsonPathQuery::parse("$..a")?;
    let tmp_file = mmap_tmp_file::create_with_contents(
        "{\n    \"a\": 42,\n    \"b\": [\n        {\n            \"b\": 43\n        }\n    ]\n}\n",
    )?;
    let input = unsafe { MmapInput::map_file(&tmp_file)? };
    let engine = MainEngine::compile_query(&jsonpath_query)?;
    let result = engine.run::<_, CountResult>(&input)?;
    assert_eq!(result.get(), 1u64, "result != expected");
    Ok(())
}
#[test]
fn with_query_select_a_number_that_is_a_child_with_mmap_input_and_count_result_using_recursive_engine(
) -> Result<(), Box<dyn Error>> {
    println ! ("on document atomic_descendant.toml running the query $..a (select a number that is a child) with Input impl MmapInput and result mode CountResult");
    let jsonpath_query = JsonPathQuery::parse("$..a")?;
    let tmp_file = mmap_tmp_file::create_with_contents(
        "{\n    \"a\": 42,\n    \"b\": [\n        {\n            \"b\": 43\n        }\n    ]\n}\n",
    )?;
    let input = unsafe { MmapInput::map_file(&tmp_file)? };
    let engine = RecursiveEngine::compile_query(&jsonpath_query)?;
    let result = engine.run::<_, CountResult>(&input)?;
    assert_eq!(result.get(), 1u64, "result != expected");
    Ok(())
}
#[test]
fn with_query_select_a_number_that_is_a_child_with_mmap_input_and_index_result_using_main_engine(
) -> Result<(), Box<dyn Error>> {
    println ! ("on document atomic_descendant.toml running the query $..a (select a number that is a child) with Input impl MmapInput and result mode IndexResult");
    let jsonpath_query = JsonPathQuery::parse("$..a")?;
    let tmp_file = mmap_tmp_file::create_with_contents(
        "{\n    \"a\": 42,\n    \"b\": [\n        {\n            \"b\": 43\n        }\n    ]\n}\n",
    )?;
    let input = unsafe { MmapInput::map_file(&tmp_file)? };
    let engine = MainEngine::compile_query(&jsonpath_query)?;
    let result = engine.run::<_, IndexResult>(&input)?;
    assert_eq!(result.get(), vec![11usize,], "result != expected");
    Ok(())
}
#[test]
fn with_query_select_a_number_that_is_a_child_with_mmap_input_and_index_result_using_recursive_engine(
) -> Result<(), Box<dyn Error>> {
    println ! ("on document atomic_descendant.toml running the query $..a (select a number that is a child) with Input impl MmapInput and result mode IndexResult");
    let jsonpath_query = JsonPathQuery::parse("$..a")?;
    let tmp_file = mmap_tmp_file::create_with_contents(
        "{\n    \"a\": 42,\n    \"b\": [\n        {\n            \"b\": 43\n        }\n    ]\n}\n",
    )?;
    let input = unsafe { MmapInput::map_file(&tmp_file)? };
    let engine = RecursiveEngine::compile_query(&jsonpath_query)?;
    let result = engine.run::<_, IndexResult>(&input)?;
    assert_eq!(result.get(), vec![11usize,], "result != expected");
    Ok(())
}
#[test]
fn with_query_look_for_descendants_of_an_atomic_value_with_owned_bytes_and_count_result_using_main_engine(
) -> Result<(), Box<dyn Error>> {
    println ! ("on document atomic_descendant.toml running the query $..a..b (look for descendants of an atomic value) with Input impl OwnedBytes and result mode CountResult");
    let jsonpath_query = JsonPathQuery::parse("$..a..b")?;
    let raw_json = "{\n    \"a\": 42,\n    \"b\": [\n        {\n            \"b\": 43\n        }\n    ]\n}\n";
    let input = OwnedBytes::new(&raw_json.as_bytes())?;
    let engine = MainEngine::compile_query(&jsonpath_query)?;
    let result = engine.run::<_, CountResult>(&input)?;
    assert_eq!(result.get(), 0u64, "result != expected");
    Ok(())
}
#[test]
fn with_query_look_for_descendants_of_an_atomic_value_with_owned_bytes_and_count_result_using_recursive_engine(
) -> Result<(), Box<dyn Error>> {
    println ! ("on document atomic_descendant.toml running the query $..a..b (look for descendants of an atomic value) with Input impl OwnedBytes and result mode CountResult");
    let jsonpath_query = JsonPathQuery::parse("$..a..b")?;
    let raw_json = "{\n    \"a\": 42,\n    \"b\": [\n        {\n            \"b\": 43\n        }\n    ]\n}\n";
    let input = OwnedBytes::new(&raw_json.as_bytes())?;
    let engine = RecursiveEngine::compile_query(&jsonpath_query)?;
    let result = engine.run::<_, CountResult>(&input)?;
    assert_eq!(result.get(), 0u64, "result != expected");
    Ok(())
}
#[test]
fn with_query_look_for_descendants_of_an_atomic_value_with_owned_bytes_and_index_result_using_main_engine(
) -> Result<(), Box<dyn Error>> {
    println ! ("on document atomic_descendant.toml running the query $..a..b (look for descendants of an atomic value) with Input impl OwnedBytes and result mode IndexResult");
    let jsonpath_query = JsonPathQuery::parse("$..a..b")?;
    let raw_json = "{\n    \"a\": 42,\n    \"b\": [\n        {\n            \"b\": 43\n        }\n    ]\n}\n";
    let input = OwnedBytes::new(&raw_json.as_bytes())?;
    let engine = MainEngine::compile_query(&jsonpath_query)?;
    let result = engine.run::<_, IndexResult>(&input)?;
    assert_eq!(result.get(), vec![], "result != expected");
    Ok(())
}
#[test]
fn with_query_look_for_descendants_of_an_atomic_value_with_owned_bytes_and_index_result_using_recursive_engine(
) -> Result<(), Box<dyn Error>> {
    println ! ("on document atomic_descendant.toml running the query $..a..b (look for descendants of an atomic value) with Input impl OwnedBytes and result mode IndexResult");
    let jsonpath_query = JsonPathQuery::parse("$..a..b")?;
    let raw_json = "{\n    \"a\": 42,\n    \"b\": [\n        {\n            \"b\": 43\n        }\n    ]\n}\n";
    let input = OwnedBytes::new(&raw_json.as_bytes())?;
    let engine = RecursiveEngine::compile_query(&jsonpath_query)?;
    let result = engine.run::<_, IndexResult>(&input)?;
    assert_eq!(result.get(), vec![], "result != expected");
    Ok(())
}
#[test]
fn with_query_look_for_descendants_of_an_atomic_value_with_buffered_input_and_count_result_using_main_engine(
) -> Result<(), Box<dyn Error>> {
    println ! ("on document atomic_descendant.toml running the query $..a..b (look for descendants of an atomic value) with Input impl BufferedInput and result mode CountResult");
    let jsonpath_query = JsonPathQuery::parse("$..a..b")?;
    let read_string = ReadString::new(
        "{\n    \"a\": 42,\n    \"b\": [\n        {\n            \"b\": 43\n        }\n    ]\n}\n".to_string(),
    );
    let input = BufferedInput::new(read_string);
    let engine = MainEngine::compile_query(&jsonpath_query)?;
    let result = engine.run::<_, CountResult>(&input)?;
    assert_eq!(result.get(), 0u64, "result != expected");
    Ok(())
}
#[test]
fn with_query_look_for_descendants_of_an_atomic_value_with_buffered_input_and_count_result_using_recursive_engine(
) -> Result<(), Box<dyn Error>> {
    println ! ("on document atomic_descendant.toml running the query $..a..b (look for descendants of an atomic value) with Input impl BufferedInput and result mode CountResult");
    let jsonpath_query = JsonPathQuery::parse("$..a..b")?;
    let read_string = ReadString::new(
        "{\n    \"a\": 42,\n    \"b\": [\n        {\n            \"b\": 43\n        }\n    ]\n}\n".to_string(),
    );
    let input = BufferedInput::new(read_string);
    let engine = RecursiveEngine::compile_query(&jsonpath_query)?;
    let result = engine.run::<_, CountResult>(&input)?;
    assert_eq!(result.get(), 0u64, "result != expected");
    Ok(())
}
#[test]
fn with_query_look_for_descendants_of_an_atomic_value_with_buffered_input_and_index_result_using_main_engine(
) -> Result<(), Box<dyn Error>> {
    println ! ("on document atomic_descendant.toml running the query $..a..b (look for descendants of an atomic value) with Input impl BufferedInput and result mode IndexResult");
    let jsonpath_query = JsonPathQuery::parse("$..a..b")?;
    let read_string = ReadString::new(
        "{\n    \"a\": 42,\n    \"b\": [\n        {\n            \"b\": 43\n        }\n    ]\n}\n".to_string(),
    );
    let input = BufferedInput::new(read_string);
    let engine = MainEngine::compile_query(&jsonpath_query)?;
    let result = engine.run::<_, IndexResult>(&input)?;
    assert_eq!(result.get(), vec![], "result != expected");
    Ok(())
}
#[test]
fn with_query_look_for_descendants_of_an_atomic_value_with_buffered_input_and_index_result_using_recursive_engine(
) -> Result<(), Box<dyn Error>> {
    println ! ("on document atomic_descendant.toml running the query $..a..b (look for descendants of an atomic value) with Input impl BufferedInput and result mode IndexResult");
    let jsonpath_query = JsonPathQuery::parse("$..a..b")?;
    let read_string = ReadString::new(
        "{\n    \"a\": 42,\n    \"b\": [\n        {\n            \"b\": 43\n        }\n    ]\n}\n".to_string(),
    );
    let input = BufferedInput::new(read_string);
    let engine = RecursiveEngine::compile_query(&jsonpath_query)?;
    let result = engine.run::<_, IndexResult>(&input)?;
    assert_eq!(result.get(), vec![], "result != expected");
    Ok(())
}
#[test]
fn with_query_look_for_descendants_of_an_atomic_value_with_mmap_input_and_count_result_using_main_engine(
) -> Result<(), Box<dyn Error>> {
    println ! ("on document atomic_descendant.toml running the query $..a..b (look for descendants of an atomic value) with Input impl MmapInput and result mode CountResult");
    let jsonpath_query = JsonPathQuery::parse("$..a..b")?;
    let tmp_file = mmap_tmp_file::create_with_contents(
        "{\n    \"a\": 42,\n    \"b\": [\n        {\n            \"b\": 43\n        }\n    ]\n}\n",
    )?;
    let input = unsafe { MmapInput::map_file(&tmp_file)? };
    let engine = MainEngine::compile_query(&jsonpath_query)?;
    let result = engine.run::<_, CountResult>(&input)?;
    assert_eq!(result.get(), 0u64, "result != expected");
    Ok(())
}
#[test]
fn with_query_look_for_descendants_of_an_atomic_value_with_mmap_input_and_count_result_using_recursive_engine(
) -> Result<(), Box<dyn Error>> {
    println ! ("on document atomic_descendant.toml running the query $..a..b (look for descendants of an atomic value) with Input impl MmapInput and result mode CountResult");
    let jsonpath_query = JsonPathQuery::parse("$..a..b")?;
    let tmp_file = mmap_tmp_file::create_with_contents(
        "{\n    \"a\": 42,\n    \"b\": [\n        {\n            \"b\": 43\n        }\n    ]\n}\n",
    )?;
    let input = unsafe { MmapInput::map_file(&tmp_file)? };
    let engine = RecursiveEngine::compile_query(&jsonpath_query)?;
    let result = engine.run::<_, CountResult>(&input)?;
    assert_eq!(result.get(), 0u64, "result != expected");
    Ok(())
}
#[test]
fn with_query_look_for_descendants_of_an_atomic_value_with_mmap_input_and_index_result_using_main_engine(
) -> Result<(), Box<dyn Error>> {
    println ! ("on document atomic_descendant.toml running the query $..a..b (look for descendants of an atomic value) with Input impl MmapInput and result mode IndexResult");
    let jsonpath_query = JsonPathQuery::parse("$..a..b")?;
    let tmp_file = mmap_tmp_file::create_with_contents(
        "{\n    \"a\": 42,\n    \"b\": [\n        {\n            \"b\": 43\n        }\n    ]\n}\n",
    )?;
    let input = unsafe { MmapInput::map_file(&tmp_file)? };
    let engine = MainEngine::compile_query(&jsonpath_query)?;
    let result = engine.run::<_, IndexResult>(&input)?;
    assert_eq!(result.get(), vec![], "result != expected");
    Ok(())
}
#[test]
fn with_query_look_for_descendants_of_an_atomic_value_with_mmap_input_and_index_result_using_recursive_engine(
) -> Result<(), Box<dyn Error>> {
    println ! ("on document atomic_descendant.toml running the query $..a..b (look for descendants of an atomic value) with Input impl MmapInput and result mode IndexResult");
    let jsonpath_query = JsonPathQuery::parse("$..a..b")?;
    let tmp_file = mmap_tmp_file::create_with_contents(
        "{\n    \"a\": 42,\n    \"b\": [\n        {\n            \"b\": 43\n        }\n    ]\n}\n",
    )?;
    let input = unsafe { MmapInput::map_file(&tmp_file)? };
    let engine = RecursiveEngine::compile_query(&jsonpath_query)?;
    let result = engine.run::<_, IndexResult>(&input)?;
    assert_eq!(result.get(), vec![], "result != expected");
    Ok(())
}
#[test]
fn with_query_look_for_b_on_at_least_one_level_of_nesting_with_owned_bytes_and_count_result_using_main_engine(
) -> Result<(), Box<dyn Error>> {
    println ! ("on document atomic_descendant.toml running the query $..*..b (look for 'b' on at least one level of nesting) with Input impl OwnedBytes and result mode CountResult");
    let jsonpath_query = JsonPathQuery::parse("$..*..b")?;
    let raw_json = "{\n    \"a\": 42,\n    \"b\": [\n        {\n            \"b\": 43\n        }\n    ]\n}\n";
    let input = OwnedBytes::new(&raw_json.as_bytes())?;
    let engine = MainEngine::compile_query(&jsonpath_query)?;
    let result = engine.run::<_, CountResult>(&input)?;
    assert_eq!(result.get(), 1u64, "result != expected");
    Ok(())
}
#[test]
fn with_query_look_for_b_on_at_least_one_level_of_nesting_with_owned_bytes_and_count_result_using_recursive_engine(
) -> Result<(), Box<dyn Error>> {
    println ! ("on document atomic_descendant.toml running the query $..*..b (look for 'b' on at least one level of nesting) with Input impl OwnedBytes and result mode CountResult");
    let jsonpath_query = JsonPathQuery::parse("$..*..b")?;
    let raw_json = "{\n    \"a\": 42,\n    \"b\": [\n        {\n            \"b\": 43\n        }\n    ]\n}\n";
    let input = OwnedBytes::new(&raw_json.as_bytes())?;
    let engine = RecursiveEngine::compile_query(&jsonpath_query)?;
    let result = engine.run::<_, CountResult>(&input)?;
    assert_eq!(result.get(), 1u64, "result != expected");
    Ok(())
}
#[test]
fn with_query_look_for_b_on_at_least_one_level_of_nesting_with_owned_bytes_and_index_result_using_main_engine(
) -> Result<(), Box<dyn Error>> {
    println ! ("on document atomic_descendant.toml running the query $..*..b (look for 'b' on at least one level of nesting) with Input impl OwnedBytes and result mode IndexResult");
    let jsonpath_query = JsonPathQuery::parse("$..*..b")?;
    let raw_json = "{\n    \"a\": 42,\n    \"b\": [\n        {\n            \"b\": 43\n        }\n    ]\n}\n";
    let input = OwnedBytes::new(&raw_json.as_bytes())?;
    let engine = MainEngine::compile_query(&jsonpath_query)?;
    let result = engine.run::<_, IndexResult>(&input)?;
    assert_eq!(result.get(), vec![53usize,], "result != expected");
    Ok(())
}
#[test]
fn with_query_look_for_b_on_at_least_one_level_of_nesting_with_owned_bytes_and_index_result_using_recursive_engine(
) -> Result<(), Box<dyn Error>> {
    println ! ("on document atomic_descendant.toml running the query $..*..b (look for 'b' on at least one level of nesting) with Input impl OwnedBytes and result mode IndexResult");
    let jsonpath_query = JsonPathQuery::parse("$..*..b")?;
    let raw_json = "{\n    \"a\": 42,\n    \"b\": [\n        {\n            \"b\": 43\n        }\n    ]\n}\n";
    let input = OwnedBytes::new(&raw_json.as_bytes())?;
    let engine = RecursiveEngine::compile_query(&jsonpath_query)?;
    let result = engine.run::<_, IndexResult>(&input)?;
    assert_eq!(result.get(), vec![53usize,], "result != expected");
    Ok(())
}
#[test]
fn with_query_look_for_b_on_at_least_one_level_of_nesting_with_buffered_input_and_count_result_using_main_engine(
) -> Result<(), Box<dyn Error>> {
    println ! ("on document atomic_descendant.toml running the query $..*..b (look for 'b' on at least one level of nesting) with Input impl BufferedInput and result mode CountResult");
    let jsonpath_query = JsonPathQuery::parse("$..*..b")?;
    let read_string = ReadString::new(
        "{\n    \"a\": 42,\n    \"b\": [\n        {\n            \"b\": 43\n        }\n    ]\n}\n".to_string(),
    );
    let input = BufferedInput::new(read_string);
    let engine = MainEngine::compile_query(&jsonpath_query)?;
    let result = engine.run::<_, CountResult>(&input)?;
    assert_eq!(result.get(), 1u64, "result != expected");
    Ok(())
}
#[test]
fn with_query_look_for_b_on_at_least_one_level_of_nesting_with_buffered_input_and_count_result_using_recursive_engine(
) -> Result<(), Box<dyn Error>> {
    println ! ("on document atomic_descendant.toml running the query $..*..b (look for 'b' on at least one level of nesting) with Input impl BufferedInput and result mode CountResult");
    let jsonpath_query = JsonPathQuery::parse("$..*..b")?;
    let read_string = ReadString::new(
        "{\n    \"a\": 42,\n    \"b\": [\n        {\n            \"b\": 43\n        }\n    ]\n}\n".to_string(),
    );
    let input = BufferedInput::new(read_string);
    let engine = RecursiveEngine::compile_query(&jsonpath_query)?;
    let result = engine.run::<_, CountResult>(&input)?;
    assert_eq!(result.get(), 1u64, "result != expected");
    Ok(())
}
#[test]
fn with_query_look_for_b_on_at_least_one_level_of_nesting_with_buffered_input_and_index_result_using_main_engine(
) -> Result<(), Box<dyn Error>> {
    println ! ("on document atomic_descendant.toml running the query $..*..b (look for 'b' on at least one level of nesting) with Input impl BufferedInput and result mode IndexResult");
    let jsonpath_query = JsonPathQuery::parse("$..*..b")?;
    let read_string = ReadString::new(
        "{\n    \"a\": 42,\n    \"b\": [\n        {\n            \"b\": 43\n        }\n    ]\n}\n".to_string(),
    );
    let input = BufferedInput::new(read_string);
    let engine = MainEngine::compile_query(&jsonpath_query)?;
    let result = engine.run::<_, IndexResult>(&input)?;
    assert_eq!(result.get(), vec![53usize,], "result != expected");
    Ok(())
}
#[test]
fn with_query_look_for_b_on_at_least_one_level_of_nesting_with_buffered_input_and_index_result_using_recursive_engine(
) -> Result<(), Box<dyn Error>> {
    println ! ("on document atomic_descendant.toml running the query $..*..b (look for 'b' on at least one level of nesting) with Input impl BufferedInput and result mode IndexResult");
    let jsonpath_query = JsonPathQuery::parse("$..*..b")?;
    let read_string = ReadString::new(
        "{\n    \"a\": 42,\n    \"b\": [\n        {\n            \"b\": 43\n        }\n    ]\n}\n".to_string(),
    );
    let input = BufferedInput::new(read_string);
    let engine = RecursiveEngine::compile_query(&jsonpath_query)?;
    let result = engine.run::<_, IndexResult>(&input)?;
    assert_eq!(result.get(), vec![53usize,], "result != expected");
    Ok(())
}
#[test]
fn with_query_look_for_b_on_at_least_one_level_of_nesting_with_mmap_input_and_count_result_using_main_engine(
) -> Result<(), Box<dyn Error>> {
    println ! ("on document atomic_descendant.toml running the query $..*..b (look for 'b' on at least one level of nesting) with Input impl MmapInput and result mode CountResult");
    let jsonpath_query = JsonPathQuery::parse("$..*..b")?;
    let tmp_file = mmap_tmp_file::create_with_contents(
        "{\n    \"a\": 42,\n    \"b\": [\n        {\n            \"b\": 43\n        }\n    ]\n}\n",
    )?;
    let input = unsafe { MmapInput::map_file(&tmp_file)? };
    let engine = MainEngine::compile_query(&jsonpath_query)?;
    let result = engine.run::<_, CountResult>(&input)?;
    assert_eq!(result.get(), 1u64, "result != expected");
    Ok(())
}
#[test]
fn with_query_look_for_b_on_at_least_one_level_of_nesting_with_mmap_input_and_count_result_using_recursive_engine(
) -> Result<(), Box<dyn Error>> {
    println ! ("on document atomic_descendant.toml running the query $..*..b (look for 'b' on at least one level of nesting) with Input impl MmapInput and result mode CountResult");
    let jsonpath_query = JsonPathQuery::parse("$..*..b")?;
    let tmp_file = mmap_tmp_file::create_with_contents(
        "{\n    \"a\": 42,\n    \"b\": [\n        {\n            \"b\": 43\n        }\n    ]\n}\n",
    )?;
    let input = unsafe { MmapInput::map_file(&tmp_file)? };
    let engine = RecursiveEngine::compile_query(&jsonpath_query)?;
    let result = engine.run::<_, CountResult>(&input)?;
    assert_eq!(result.get(), 1u64, "result != expected");
    Ok(())
}
#[test]
fn with_query_look_for_b_on_at_least_one_level_of_nesting_with_mmap_input_and_index_result_using_main_engine(
) -> Result<(), Box<dyn Error>> {
    println ! ("on document atomic_descendant.toml running the query $..*..b (look for 'b' on at least one level of nesting) with Input impl MmapInput and result mode IndexResult");
    let jsonpath_query = JsonPathQuery::parse("$..*..b")?;
    let tmp_file = mmap_tmp_file::create_with_contents(
        "{\n    \"a\": 42,\n    \"b\": [\n        {\n            \"b\": 43\n        }\n    ]\n}\n",
    )?;
    let input = unsafe { MmapInput::map_file(&tmp_file)? };
    let engine = MainEngine::compile_query(&jsonpath_query)?;
    let result = engine.run::<_, IndexResult>(&input)?;
    assert_eq!(result.get(), vec![53usize,], "result != expected");
    Ok(())
}
#[test]
fn with_query_look_for_b_on_at_least_one_level_of_nesting_with_mmap_input_and_index_result_using_recursive_engine(
) -> Result<(), Box<dyn Error>> {
    println ! ("on document atomic_descendant.toml running the query $..*..b (look for 'b' on at least one level of nesting) with Input impl MmapInput and result mode IndexResult");
    let jsonpath_query = JsonPathQuery::parse("$..*..b")?;
    let tmp_file = mmap_tmp_file::create_with_contents(
        "{\n    \"a\": 42,\n    \"b\": [\n        {\n            \"b\": 43\n        }\n    ]\n}\n",
    )?;
    let input = unsafe { MmapInput::map_file(&tmp_file)? };
    let engine = RecursiveEngine::compile_query(&jsonpath_query)?;
    let result = engine.run::<_, IndexResult>(&input)?;
    assert_eq!(result.get(), vec![53usize,], "result != expected");
    Ok(())
}
#[test]
fn with_query_select_all_decsendants_with_owned_bytes_and_count_result_using_main_engine() -> Result<(), Box<dyn Error>>
{
    println ! ("on document atomic_descendant.toml running the query $..* (select all decsendants) with Input impl OwnedBytes and result mode CountResult");
    let jsonpath_query = JsonPathQuery::parse("$..*")?;
    let raw_json = "{\n    \"a\": 42,\n    \"b\": [\n        {\n            \"b\": 43\n        }\n    ]\n}\n";
    let input = OwnedBytes::new(&raw_json.as_bytes())?;
    let engine = MainEngine::compile_query(&jsonpath_query)?;
    let result = engine.run::<_, CountResult>(&input)?;
    assert_eq!(result.get(), 4u64, "result != expected");
    Ok(())
}
#[test]
fn with_query_select_all_decsendants_with_owned_bytes_and_count_result_using_recursive_engine(
) -> Result<(), Box<dyn Error>> {
    println ! ("on document atomic_descendant.toml running the query $..* (select all decsendants) with Input impl OwnedBytes and result mode CountResult");
    let jsonpath_query = JsonPathQuery::parse("$..*")?;
    let raw_json = "{\n    \"a\": 42,\n    \"b\": [\n        {\n            \"b\": 43\n        }\n    ]\n}\n";
    let input = OwnedBytes::new(&raw_json.as_bytes())?;
    let engine = RecursiveEngine::compile_query(&jsonpath_query)?;
    let result = engine.run::<_, CountResult>(&input)?;
    assert_eq!(result.get(), 4u64, "result != expected");
    Ok(())
}
#[test]
fn with_query_select_all_decsendants_with_owned_bytes_and_index_result_using_main_engine() -> Result<(), Box<dyn Error>>
{
    println ! ("on document atomic_descendant.toml running the query $..* (select all decsendants) with Input impl OwnedBytes and result mode IndexResult");
    let jsonpath_query = JsonPathQuery::parse("$..*")?;
    let raw_json = "{\n    \"a\": 42,\n    \"b\": [\n        {\n            \"b\": 43\n        }\n    ]\n}\n";
    let input = OwnedBytes::new(&raw_json.as_bytes())?;
    let engine = MainEngine::compile_query(&jsonpath_query)?;
    let result = engine.run::<_, IndexResult>(&input)?;
    assert_eq!(
        result.get(),
        vec![11usize, 24usize, 34usize, 53usize,],
        "result != expected"
    );
    Ok(())
}
#[test]
fn with_query_select_all_decsendants_with_owned_bytes_and_index_result_using_recursive_engine(
) -> Result<(), Box<dyn Error>> {
    println ! ("on document atomic_descendant.toml running the query $..* (select all decsendants) with Input impl OwnedBytes and result mode IndexResult");
    let jsonpath_query = JsonPathQuery::parse("$..*")?;
    let raw_json = "{\n    \"a\": 42,\n    \"b\": [\n        {\n            \"b\": 43\n        }\n    ]\n}\n";
    let input = OwnedBytes::new(&raw_json.as_bytes())?;
    let engine = RecursiveEngine::compile_query(&jsonpath_query)?;
    let result = engine.run::<_, IndexResult>(&input)?;
    assert_eq!(
        result.get(),
        vec![11usize, 24usize, 34usize, 53usize,],
        "result != expected"
    );
    Ok(())
}
#[test]
fn with_query_select_all_decsendants_with_buffered_input_and_count_result_using_main_engine(
) -> Result<(), Box<dyn Error>> {
    println ! ("on document atomic_descendant.toml running the query $..* (select all decsendants) with Input impl BufferedInput and result mode CountResult");
    let jsonpath_query = JsonPathQuery::parse("$..*")?;
    let read_string = ReadString::new(
        "{\n    \"a\": 42,\n    \"b\": [\n        {\n            \"b\": 43\n        }\n    ]\n}\n".to_string(),
    );
    let input = BufferedInput::new(read_string);
    let engine = MainEngine::compile_query(&jsonpath_query)?;
    let result = engine.run::<_, CountResult>(&input)?;
    assert_eq!(result.get(), 4u64, "result != expected");
    Ok(())
}
#[test]
fn with_query_select_all_decsendants_with_buffered_input_and_count_result_using_recursive_engine(
) -> Result<(), Box<dyn Error>> {
    println ! ("on document atomic_descendant.toml running the query $..* (select all decsendants) with Input impl BufferedInput and result mode CountResult");
    let jsonpath_query = JsonPathQuery::parse("$..*")?;
    let read_string = ReadString::new(
        "{\n    \"a\": 42,\n    \"b\": [\n        {\n            \"b\": 43\n        }\n    ]\n}\n".to_string(),
    );
    let input = BufferedInput::new(read_string);
    let engine = RecursiveEngine::compile_query(&jsonpath_query)?;
    let result = engine.run::<_, CountResult>(&input)?;
    assert_eq!(result.get(), 4u64, "result != expected");
    Ok(())
}
#[test]
fn with_query_select_all_decsendants_with_buffered_input_and_index_result_using_main_engine(
) -> Result<(), Box<dyn Error>> {
    println ! ("on document atomic_descendant.toml running the query $..* (select all decsendants) with Input impl BufferedInput and result mode IndexResult");
    let jsonpath_query = JsonPathQuery::parse("$..*")?;
    let read_string = ReadString::new(
        "{\n    \"a\": 42,\n    \"b\": [\n        {\n            \"b\": 43\n        }\n    ]\n}\n".to_string(),
    );
    let input = BufferedInput::new(read_string);
    let engine = MainEngine::compile_query(&jsonpath_query)?;
    let result = engine.run::<_, IndexResult>(&input)?;
    assert_eq!(
        result.get(),
        vec![11usize, 24usize, 34usize, 53usize,],
        "result != expected"
    );
    Ok(())
}
#[test]
fn with_query_select_all_decsendants_with_buffered_input_and_index_result_using_recursive_engine(
) -> Result<(), Box<dyn Error>> {
    println ! ("on document atomic_descendant.toml running the query $..* (select all decsendants) with Input impl BufferedInput and result mode IndexResult");
    let jsonpath_query = JsonPathQuery::parse("$..*")?;
    let read_string = ReadString::new(
        "{\n    \"a\": 42,\n    \"b\": [\n        {\n            \"b\": 43\n        }\n    ]\n}\n".to_string(),
    );
    let input = BufferedInput::new(read_string);
    let engine = RecursiveEngine::compile_query(&jsonpath_query)?;
    let result = engine.run::<_, IndexResult>(&input)?;
    assert_eq!(
        result.get(),
        vec![11usize, 24usize, 34usize, 53usize,],
        "result != expected"
    );
    Ok(())
}
#[test]
fn with_query_select_all_decsendants_with_mmap_input_and_count_result_using_main_engine() -> Result<(), Box<dyn Error>>
{
    println ! ("on document atomic_descendant.toml running the query $..* (select all decsendants) with Input impl MmapInput and result mode CountResult");
    let jsonpath_query = JsonPathQuery::parse("$..*")?;
    let tmp_file = mmap_tmp_file::create_with_contents(
        "{\n    \"a\": 42,\n    \"b\": [\n        {\n            \"b\": 43\n        }\n    ]\n}\n",
    )?;
    let input = unsafe { MmapInput::map_file(&tmp_file)? };
    let engine = MainEngine::compile_query(&jsonpath_query)?;
    let result = engine.run::<_, CountResult>(&input)?;
    assert_eq!(result.get(), 4u64, "result != expected");
    Ok(())
}
#[test]
fn with_query_select_all_decsendants_with_mmap_input_and_count_result_using_recursive_engine(
) -> Result<(), Box<dyn Error>> {
    println ! ("on document atomic_descendant.toml running the query $..* (select all decsendants) with Input impl MmapInput and result mode CountResult");
    let jsonpath_query = JsonPathQuery::parse("$..*")?;
    let tmp_file = mmap_tmp_file::create_with_contents(
        "{\n    \"a\": 42,\n    \"b\": [\n        {\n            \"b\": 43\n        }\n    ]\n}\n",
    )?;
    let input = unsafe { MmapInput::map_file(&tmp_file)? };
    let engine = RecursiveEngine::compile_query(&jsonpath_query)?;
    let result = engine.run::<_, CountResult>(&input)?;
    assert_eq!(result.get(), 4u64, "result != expected");
    Ok(())
}
#[test]
fn with_query_select_all_decsendants_with_mmap_input_and_index_result_using_main_engine() -> Result<(), Box<dyn Error>>
{
    println ! ("on document atomic_descendant.toml running the query $..* (select all decsendants) with Input impl MmapInput and result mode IndexResult");
    let jsonpath_query = JsonPathQuery::parse("$..*")?;
    let tmp_file = mmap_tmp_file::create_with_contents(
        "{\n    \"a\": 42,\n    \"b\": [\n        {\n            \"b\": 43\n        }\n    ]\n}\n",
    )?;
    let input = unsafe { MmapInput::map_file(&tmp_file)? };
    let engine = MainEngine::compile_query(&jsonpath_query)?;
    let result = engine.run::<_, IndexResult>(&input)?;
    assert_eq!(
        result.get(),
        vec![11usize, 24usize, 34usize, 53usize,],
        "result != expected"
    );
    Ok(())
}
#[test]
fn with_query_select_all_decsendants_with_mmap_input_and_index_result_using_recursive_engine(
) -> Result<(), Box<dyn Error>> {
    println ! ("on document atomic_descendant.toml running the query $..* (select all decsendants) with Input impl MmapInput and result mode IndexResult");
    let jsonpath_query = JsonPathQuery::parse("$..*")?;
    let tmp_file = mmap_tmp_file::create_with_contents(
        "{\n    \"a\": 42,\n    \"b\": [\n        {\n            \"b\": 43\n        }\n    ]\n}\n",
    )?;
    let input = unsafe { MmapInput::map_file(&tmp_file)? };
    let engine = RecursiveEngine::compile_query(&jsonpath_query)?;
    let result = engine.run::<_, IndexResult>(&input)?;
    assert_eq!(
        result.get(),
        vec![11usize, 24usize, 34usize, 53usize,],
        "result != expected"
    );
    Ok(())
}
#[test]
fn with_query_select_first_item_from_list_descendants_with_owned_bytes_and_count_result_using_main_engine(
) -> Result<(), Box<dyn Error>> {
    println ! ("on document atomic_descendant.toml running the query $..[0] (select first item from list descendants) with Input impl OwnedBytes and result mode CountResult");
    let jsonpath_query = JsonPathQuery::parse("$..[0]")?;
    let raw_json = "{\n    \"a\": 42,\n    \"b\": [\n        {\n            \"b\": 43\n        }\n    ]\n}\n";
    let input = OwnedBytes::new(&raw_json.as_bytes())?;
    let engine = MainEngine::compile_query(&jsonpath_query)?;
    let result = engine.run::<_, CountResult>(&input)?;
    assert_eq!(result.get(), 1u64, "result != expected");
    Ok(())
}
#[test]
fn with_query_select_first_item_from_list_descendants_with_owned_bytes_and_count_result_using_recursive_engine(
) -> Result<(), Box<dyn Error>> {
    println ! ("on document atomic_descendant.toml running the query $..[0] (select first item from list descendants) with Input impl OwnedBytes and result mode CountResult");
    let jsonpath_query = JsonPathQuery::parse("$..[0]")?;
    let raw_json = "{\n    \"a\": 42,\n    \"b\": [\n        {\n            \"b\": 43\n        }\n    ]\n}\n";
    let input = OwnedBytes::new(&raw_json.as_bytes())?;
    let engine = RecursiveEngine::compile_query(&jsonpath_query)?;
    let result = engine.run::<_, CountResult>(&input)?;
    assert_eq!(result.get(), 1u64, "result != expected");
    Ok(())
}
#[test]
fn with_query_select_first_item_from_list_descendants_with_owned_bytes_and_index_result_using_main_engine(
) -> Result<(), Box<dyn Error>> {
    println ! ("on document atomic_descendant.toml running the query $..[0] (select first item from list descendants) with Input impl OwnedBytes and result mode IndexResult");
    let jsonpath_query = JsonPathQuery::parse("$..[0]")?;
    let raw_json = "{\n    \"a\": 42,\n    \"b\": [\n        {\n            \"b\": 43\n        }\n    ]\n}\n";
    let input = OwnedBytes::new(&raw_json.as_bytes())?;
    let engine = MainEngine::compile_query(&jsonpath_query)?;
    let result = engine.run::<_, IndexResult>(&input)?;
    assert_eq!(result.get(), vec![34usize,], "result != expected");
    Ok(())
}
#[test]
fn with_query_select_first_item_from_list_descendants_with_owned_bytes_and_index_result_using_recursive_engine(
) -> Result<(), Box<dyn Error>> {
    println ! ("on document atomic_descendant.toml running the query $..[0] (select first item from list descendants) with Input impl OwnedBytes and result mode IndexResult");
    let jsonpath_query = JsonPathQuery::parse("$..[0]")?;
    let raw_json = "{\n    \"a\": 42,\n    \"b\": [\n        {\n            \"b\": 43\n        }\n    ]\n}\n";
    let input = OwnedBytes::new(&raw_json.as_bytes())?;
    let engine = RecursiveEngine::compile_query(&jsonpath_query)?;
    let result = engine.run::<_, IndexResult>(&input)?;
    assert_eq!(result.get(), vec![34usize,], "result != expected");
    Ok(())
}
#[test]
fn with_query_select_first_item_from_list_descendants_with_buffered_input_and_count_result_using_main_engine(
) -> Result<(), Box<dyn Error>> {
    println ! ("on document atomic_descendant.toml running the query $..[0] (select first item from list descendants) with Input impl BufferedInput and result mode CountResult");
    let jsonpath_query = JsonPathQuery::parse("$..[0]")?;
    let read_string = ReadString::new(
        "{\n    \"a\": 42,\n    \"b\": [\n        {\n            \"b\": 43\n        }\n    ]\n}\n".to_string(),
    );
    let input = BufferedInput::new(read_string);
    let engine = MainEngine::compile_query(&jsonpath_query)?;
    let result = engine.run::<_, CountResult>(&input)?;
    assert_eq!(result.get(), 1u64, "result != expected");
    Ok(())
}
#[test]
fn with_query_select_first_item_from_list_descendants_with_buffered_input_and_count_result_using_recursive_engine(
) -> Result<(), Box<dyn Error>> {
    println ! ("on document atomic_descendant.toml running the query $..[0] (select first item from list descendants) with Input impl BufferedInput and result mode CountResult");
    let jsonpath_query = JsonPathQuery::parse("$..[0]")?;
    let read_string = ReadString::new(
        "{\n    \"a\": 42,\n    \"b\": [\n        {\n            \"b\": 43\n        }\n    ]\n}\n".to_string(),
    );
    let input = BufferedInput::new(read_string);
    let engine = RecursiveEngine::compile_query(&jsonpath_query)?;
    let result = engine.run::<_, CountResult>(&input)?;
    assert_eq!(result.get(), 1u64, "result != expected");
    Ok(())
}
#[test]
fn with_query_select_first_item_from_list_descendants_with_buffered_input_and_index_result_using_main_engine(
) -> Result<(), Box<dyn Error>> {
    println ! ("on document atomic_descendant.toml running the query $..[0] (select first item from list descendants) with Input impl BufferedInput and result mode IndexResult");
    let jsonpath_query = JsonPathQuery::parse("$..[0]")?;
    let read_string = ReadString::new(
        "{\n    \"a\": 42,\n    \"b\": [\n        {\n            \"b\": 43\n        }\n    ]\n}\n".to_string(),
    );
    let input = BufferedInput::new(read_string);
    let engine = MainEngine::compile_query(&jsonpath_query)?;
    let result = engine.run::<_, IndexResult>(&input)?;
    assert_eq!(result.get(), vec![34usize,], "result != expected");
    Ok(())
}
#[test]
fn with_query_select_first_item_from_list_descendants_with_buffered_input_and_index_result_using_recursive_engine(
) -> Result<(), Box<dyn Error>> {
    println ! ("on document atomic_descendant.toml running the query $..[0] (select first item from list descendants) with Input impl BufferedInput and result mode IndexResult");
    let jsonpath_query = JsonPathQuery::parse("$..[0]")?;
    let read_string = ReadString::new(
        "{\n    \"a\": 42,\n    \"b\": [\n        {\n            \"b\": 43\n        }\n    ]\n}\n".to_string(),
    );
    let input = BufferedInput::new(read_string);
    let engine = RecursiveEngine::compile_query(&jsonpath_query)?;
    let result = engine.run::<_, IndexResult>(&input)?;
    assert_eq!(result.get(), vec![34usize,], "result != expected");
    Ok(())
}
#[test]
fn with_query_select_first_item_from_list_descendants_with_mmap_input_and_count_result_using_main_engine(
) -> Result<(), Box<dyn Error>> {
    println ! ("on document atomic_descendant.toml running the query $..[0] (select first item from list descendants) with Input impl MmapInput and result mode CountResult");
    let jsonpath_query = JsonPathQuery::parse("$..[0]")?;
    let tmp_file = mmap_tmp_file::create_with_contents(
        "{\n    \"a\": 42,\n    \"b\": [\n        {\n            \"b\": 43\n        }\n    ]\n}\n",
    )?;
    let input = unsafe { MmapInput::map_file(&tmp_file)? };
    let engine = MainEngine::compile_query(&jsonpath_query)?;
    let result = engine.run::<_, CountResult>(&input)?;
    assert_eq!(result.get(), 1u64, "result != expected");
    Ok(())
}
#[test]
fn with_query_select_first_item_from_list_descendants_with_mmap_input_and_count_result_using_recursive_engine(
) -> Result<(), Box<dyn Error>> {
    println ! ("on document atomic_descendant.toml running the query $..[0] (select first item from list descendants) with Input impl MmapInput and result mode CountResult");
    let jsonpath_query = JsonPathQuery::parse("$..[0]")?;
    let tmp_file = mmap_tmp_file::create_with_contents(
        "{\n    \"a\": 42,\n    \"b\": [\n        {\n            \"b\": 43\n        }\n    ]\n}\n",
    )?;
    let input = unsafe { MmapInput::map_file(&tmp_file)? };
    let engine = RecursiveEngine::compile_query(&jsonpath_query)?;
    let result = engine.run::<_, CountResult>(&input)?;
    assert_eq!(result.get(), 1u64, "result != expected");
    Ok(())
}
#[test]
fn with_query_select_first_item_from_list_descendants_with_mmap_input_and_index_result_using_main_engine(
) -> Result<(), Box<dyn Error>> {
    println ! ("on document atomic_descendant.toml running the query $..[0] (select first item from list descendants) with Input impl MmapInput and result mode IndexResult");
    let jsonpath_query = JsonPathQuery::parse("$..[0]")?;
    let tmp_file = mmap_tmp_file::create_with_contents(
        "{\n    \"a\": 42,\n    \"b\": [\n        {\n            \"b\": 43\n        }\n    ]\n}\n",
    )?;
    let input = unsafe { MmapInput::map_file(&tmp_file)? };
    let engine = MainEngine::compile_query(&jsonpath_query)?;
    let result = engine.run::<_, IndexResult>(&input)?;
    assert_eq!(result.get(), vec![34usize,], "result != expected");
    Ok(())
}
#[test]
fn with_query_select_first_item_from_list_descendants_with_mmap_input_and_index_result_using_recursive_engine(
) -> Result<(), Box<dyn Error>> {
    println ! ("on document atomic_descendant.toml running the query $..[0] (select first item from list descendants) with Input impl MmapInput and result mode IndexResult");
    let jsonpath_query = JsonPathQuery::parse("$..[0]")?;
    let tmp_file = mmap_tmp_file::create_with_contents(
        "{\n    \"a\": 42,\n    \"b\": [\n        {\n            \"b\": 43\n        }\n    ]\n}\n",
    )?;
    let input = unsafe { MmapInput::map_file(&tmp_file)? };
    let engine = RecursiveEngine::compile_query(&jsonpath_query)?;
    let result = engine.run::<_, IndexResult>(&input)?;
    assert_eq!(result.get(), vec![34usize,], "result != expected");
    Ok(())
}
#[test]
fn empty_object_root_with_query_select_the_root_empty_query_with_owned_bytes_and_count_result_using_main_engine(
) -> Result<(), Box<dyn Error>> {
    println ! ("on document empty_object.toml running the query  (select the root (empty query)) with Input impl OwnedBytes and result mode CountResult");
    let jsonpath_query = JsonPathQuery::parse("")?;
    let raw_json = "{}\n";
    let input = OwnedBytes::new(&raw_json.as_bytes())?;
    let engine = MainEngine::compile_query(&jsonpath_query)?;
    let result = engine.run::<_, CountResult>(&input)?;
    assert_eq!(result.get(), 1u64, "result != expected");
    Ok(())
}
#[test]
fn empty_object_root_with_query_select_the_root_empty_query_with_owned_bytes_and_count_result_using_recursive_engine(
) -> Result<(), Box<dyn Error>> {
    println ! ("on document empty_object.toml running the query  (select the root (empty query)) with Input impl OwnedBytes and result mode CountResult");
    let jsonpath_query = JsonPathQuery::parse("")?;
    let raw_json = "{}\n";
    let input = OwnedBytes::new(&raw_json.as_bytes())?;
    let engine = RecursiveEngine::compile_query(&jsonpath_query)?;
    let result = engine.run::<_, CountResult>(&input)?;
    assert_eq!(result.get(), 1u64, "result != expected");
    Ok(())
}
#[test]
fn empty_object_root_with_query_select_the_root_empty_query_with_owned_bytes_and_index_result_using_main_engine(
) -> Result<(), Box<dyn Error>> {
    println ! ("on document empty_object.toml running the query  (select the root (empty query)) with Input impl OwnedBytes and result mode IndexResult");
    let jsonpath_query = JsonPathQuery::parse("")?;
    let raw_json = "{}\n";
    let input = OwnedBytes::new(&raw_json.as_bytes())?;
    let engine = MainEngine::compile_query(&jsonpath_query)?;
    let result = engine.run::<_, IndexResult>(&input)?;
    assert_eq!(result.get(), vec![0usize,], "result != expected");
    Ok(())
}
#[test]
fn empty_object_root_with_query_select_the_root_empty_query_with_owned_bytes_and_index_result_using_recursive_engine(
) -> Result<(), Box<dyn Error>> {
    println ! ("on document empty_object.toml running the query  (select the root (empty query)) with Input impl OwnedBytes and result mode IndexResult");
    let jsonpath_query = JsonPathQuery::parse("")?;
    let raw_json = "{}\n";
    let input = OwnedBytes::new(&raw_json.as_bytes())?;
    let engine = RecursiveEngine::compile_query(&jsonpath_query)?;
    let result = engine.run::<_, IndexResult>(&input)?;
    assert_eq!(result.get(), vec![0usize,], "result != expected");
    Ok(())
}
#[test]
fn empty_object_root_with_query_select_the_root_empty_query_with_buffered_input_and_count_result_using_main_engine(
) -> Result<(), Box<dyn Error>> {
    println ! ("on document empty_object.toml running the query  (select the root (empty query)) with Input impl BufferedInput and result mode CountResult");
    let jsonpath_query = JsonPathQuery::parse("")?;
    let read_string = ReadString::new("{}\n".to_string());
    let input = BufferedInput::new(read_string);
    let engine = MainEngine::compile_query(&jsonpath_query)?;
    let result = engine.run::<_, CountResult>(&input)?;
    assert_eq!(result.get(), 1u64, "result != expected");
    Ok(())
}
#[test]
fn empty_object_root_with_query_select_the_root_empty_query_with_buffered_input_and_count_result_using_recursive_engine(
) -> Result<(), Box<dyn Error>> {
    println ! ("on document empty_object.toml running the query  (select the root (empty query)) with Input impl BufferedInput and result mode CountResult");
    let jsonpath_query = JsonPathQuery::parse("")?;
    let read_string = ReadString::new("{}\n".to_string());
    let input = BufferedInput::new(read_string);
    let engine = RecursiveEngine::compile_query(&jsonpath_query)?;
    let result = engine.run::<_, CountResult>(&input)?;
    assert_eq!(result.get(), 1u64, "result != expected");
    Ok(())
}
#[test]
fn empty_object_root_with_query_select_the_root_empty_query_with_buffered_input_and_index_result_using_main_engine(
) -> Result<(), Box<dyn Error>> {
    println ! ("on document empty_object.toml running the query  (select the root (empty query)) with Input impl BufferedInput and result mode IndexResult");
    let jsonpath_query = JsonPathQuery::parse("")?;
    let read_string = ReadString::new("{}\n".to_string());
    let input = BufferedInput::new(read_string);
    let engine = MainEngine::compile_query(&jsonpath_query)?;
    let result = engine.run::<_, IndexResult>(&input)?;
    assert_eq!(result.get(), vec![0usize,], "result != expected");
    Ok(())
}
#[test]
fn empty_object_root_with_query_select_the_root_empty_query_with_buffered_input_and_index_result_using_recursive_engine(
) -> Result<(), Box<dyn Error>> {
    println ! ("on document empty_object.toml running the query  (select the root (empty query)) with Input impl BufferedInput and result mode IndexResult");
    let jsonpath_query = JsonPathQuery::parse("")?;
    let read_string = ReadString::new("{}\n".to_string());
    let input = BufferedInput::new(read_string);
    let engine = RecursiveEngine::compile_query(&jsonpath_query)?;
    let result = engine.run::<_, IndexResult>(&input)?;
    assert_eq!(result.get(), vec![0usize,], "result != expected");
    Ok(())
}
#[test]
fn empty_object_root_with_query_select_the_root_empty_query_with_mmap_input_and_count_result_using_main_engine(
) -> Result<(), Box<dyn Error>> {
    println ! ("on document empty_object.toml running the query  (select the root (empty query)) with Input impl MmapInput and result mode CountResult");
    let jsonpath_query = JsonPathQuery::parse("")?;
    let tmp_file = mmap_tmp_file::create_with_contents("{}\n")?;
    let input = unsafe { MmapInput::map_file(&tmp_file)? };
    let engine = MainEngine::compile_query(&jsonpath_query)?;
    let result = engine.run::<_, CountResult>(&input)?;
    assert_eq!(result.get(), 1u64, "result != expected");
    Ok(())
}
#[test]
fn empty_object_root_with_query_select_the_root_empty_query_with_mmap_input_and_count_result_using_recursive_engine(
) -> Result<(), Box<dyn Error>> {
    println ! ("on document empty_object.toml running the query  (select the root (empty query)) with Input impl MmapInput and result mode CountResult");
    let jsonpath_query = JsonPathQuery::parse("")?;
    let tmp_file = mmap_tmp_file::create_with_contents("{}\n")?;
    let input = unsafe { MmapInput::map_file(&tmp_file)? };
    let engine = RecursiveEngine::compile_query(&jsonpath_query)?;
    let result = engine.run::<_, CountResult>(&input)?;
    assert_eq!(result.get(), 1u64, "result != expected");
    Ok(())
}
#[test]
fn empty_object_root_with_query_select_the_root_empty_query_with_mmap_input_and_index_result_using_main_engine(
) -> Result<(), Box<dyn Error>> {
    println ! ("on document empty_object.toml running the query  (select the root (empty query)) with Input impl MmapInput and result mode IndexResult");
    let jsonpath_query = JsonPathQuery::parse("")?;
    let tmp_file = mmap_tmp_file::create_with_contents("{}\n")?;
    let input = unsafe { MmapInput::map_file(&tmp_file)? };
    let engine = MainEngine::compile_query(&jsonpath_query)?;
    let result = engine.run::<_, IndexResult>(&input)?;
    assert_eq!(result.get(), vec![0usize,], "result != expected");
    Ok(())
}
#[test]
fn empty_object_root_with_query_select_the_root_empty_query_with_mmap_input_and_index_result_using_recursive_engine(
) -> Result<(), Box<dyn Error>> {
    println ! ("on document empty_object.toml running the query  (select the root (empty query)) with Input impl MmapInput and result mode IndexResult");
    let jsonpath_query = JsonPathQuery::parse("")?;
    let tmp_file = mmap_tmp_file::create_with_contents("{}\n")?;
    let input = unsafe { MmapInput::map_file(&tmp_file)? };
    let engine = RecursiveEngine::compile_query(&jsonpath_query)?;
    let result = engine.run::<_, IndexResult>(&input)?;
    assert_eq!(result.get(), vec![0usize,], "result != expected");
    Ok(())
}
#[test]
fn empty_object_root_with_query_select_the_root_with_owned_bytes_and_count_result_using_main_engine(
) -> Result<(), Box<dyn Error>> {
    println ! ("on document empty_object.toml running the query $ (select the root) with Input impl OwnedBytes and result mode CountResult");
    let jsonpath_query = JsonPathQuery::parse("$")?;
    let raw_json = "{}\n";
    let input = OwnedBytes::new(&raw_json.as_bytes())?;
    let engine = MainEngine::compile_query(&jsonpath_query)?;
    let result = engine.run::<_, CountResult>(&input)?;
    assert_eq!(result.get(), 1u64, "result != expected");
    Ok(())
}
#[test]
fn empty_object_root_with_query_select_the_root_with_owned_bytes_and_count_result_using_recursive_engine(
) -> Result<(), Box<dyn Error>> {
    println ! ("on document empty_object.toml running the query $ (select the root) with Input impl OwnedBytes and result mode CountResult");
    let jsonpath_query = JsonPathQuery::parse("$")?;
    let raw_json = "{}\n";
    let input = OwnedBytes::new(&raw_json.as_bytes())?;
    let engine = RecursiveEngine::compile_query(&jsonpath_query)?;
    let result = engine.run::<_, CountResult>(&input)?;
    assert_eq!(result.get(), 1u64, "result != expected");
    Ok(())
}
#[test]
fn empty_object_root_with_query_select_the_root_with_owned_bytes_and_index_result_using_main_engine(
) -> Result<(), Box<dyn Error>> {
    println ! ("on document empty_object.toml running the query $ (select the root) with Input impl OwnedBytes and result mode IndexResult");
    let jsonpath_query = JsonPathQuery::parse("$")?;
    let raw_json = "{}\n";
    let input = OwnedBytes::new(&raw_json.as_bytes())?;
    let engine = MainEngine::compile_query(&jsonpath_query)?;
    let result = engine.run::<_, IndexResult>(&input)?;
    assert_eq!(result.get(), vec![0usize,], "result != expected");
    Ok(())
}
#[test]
fn empty_object_root_with_query_select_the_root_with_owned_bytes_and_index_result_using_recursive_engine(
) -> Result<(), Box<dyn Error>> {
    println ! ("on document empty_object.toml running the query $ (select the root) with Input impl OwnedBytes and result mode IndexResult");
    let jsonpath_query = JsonPathQuery::parse("$")?;
    let raw_json = "{}\n";
    let input = OwnedBytes::new(&raw_json.as_bytes())?;
    let engine = RecursiveEngine::compile_query(&jsonpath_query)?;
    let result = engine.run::<_, IndexResult>(&input)?;
    assert_eq!(result.get(), vec![0usize,], "result != expected");
    Ok(())
}
#[test]
fn empty_object_root_with_query_select_the_root_with_buffered_input_and_count_result_using_main_engine(
) -> Result<(), Box<dyn Error>> {
    println ! ("on document empty_object.toml running the query $ (select the root) with Input impl BufferedInput and result mode CountResult");
    let jsonpath_query = JsonPathQuery::parse("$")?;
    let read_string = ReadString::new("{}\n".to_string());
    let input = BufferedInput::new(read_string);
    let engine = MainEngine::compile_query(&jsonpath_query)?;
    let result = engine.run::<_, CountResult>(&input)?;
    assert_eq!(result.get(), 1u64, "result != expected");
    Ok(())
}
#[test]
fn empty_object_root_with_query_select_the_root_with_buffered_input_and_count_result_using_recursive_engine(
) -> Result<(), Box<dyn Error>> {
    println ! ("on document empty_object.toml running the query $ (select the root) with Input impl BufferedInput and result mode CountResult");
    let jsonpath_query = JsonPathQuery::parse("$")?;
    let read_string = ReadString::new("{}\n".to_string());
    let input = BufferedInput::new(read_string);
    let engine = RecursiveEngine::compile_query(&jsonpath_query)?;
    let result = engine.run::<_, CountResult>(&input)?;
    assert_eq!(result.get(), 1u64, "result != expected");
    Ok(())
}
#[test]
fn empty_object_root_with_query_select_the_root_with_buffered_input_and_index_result_using_main_engine(
) -> Result<(), Box<dyn Error>> {
    println ! ("on document empty_object.toml running the query $ (select the root) with Input impl BufferedInput and result mode IndexResult");
    let jsonpath_query = JsonPathQuery::parse("$")?;
    let read_string = ReadString::new("{}\n".to_string());
    let input = BufferedInput::new(read_string);
    let engine = MainEngine::compile_query(&jsonpath_query)?;
    let result = engine.run::<_, IndexResult>(&input)?;
    assert_eq!(result.get(), vec![0usize,], "result != expected");
    Ok(())
}
#[test]
fn empty_object_root_with_query_select_the_root_with_buffered_input_and_index_result_using_recursive_engine(
) -> Result<(), Box<dyn Error>> {
    println ! ("on document empty_object.toml running the query $ (select the root) with Input impl BufferedInput and result mode IndexResult");
    let jsonpath_query = JsonPathQuery::parse("$")?;
    let read_string = ReadString::new("{}\n".to_string());
    let input = BufferedInput::new(read_string);
    let engine = RecursiveEngine::compile_query(&jsonpath_query)?;
    let result = engine.run::<_, IndexResult>(&input)?;
    assert_eq!(result.get(), vec![0usize,], "result != expected");
    Ok(())
}
#[test]
fn empty_object_root_with_query_select_the_root_with_mmap_input_and_count_result_using_main_engine(
) -> Result<(), Box<dyn Error>> {
    println ! ("on document empty_object.toml running the query $ (select the root) with Input impl MmapInput and result mode CountResult");
    let jsonpath_query = JsonPathQuery::parse("$")?;
    let tmp_file = mmap_tmp_file::create_with_contents("{}\n")?;
    let input = unsafe { MmapInput::map_file(&tmp_file)? };
    let engine = MainEngine::compile_query(&jsonpath_query)?;
    let result = engine.run::<_, CountResult>(&input)?;
    assert_eq!(result.get(), 1u64, "result != expected");
    Ok(())
}
#[test]
fn empty_object_root_with_query_select_the_root_with_mmap_input_and_count_result_using_recursive_engine(
) -> Result<(), Box<dyn Error>> {
    println ! ("on document empty_object.toml running the query $ (select the root) with Input impl MmapInput and result mode CountResult");
    let jsonpath_query = JsonPathQuery::parse("$")?;
    let tmp_file = mmap_tmp_file::create_with_contents("{}\n")?;
    let input = unsafe { MmapInput::map_file(&tmp_file)? };
    let engine = RecursiveEngine::compile_query(&jsonpath_query)?;
    let result = engine.run::<_, CountResult>(&input)?;
    assert_eq!(result.get(), 1u64, "result != expected");
    Ok(())
}
#[test]
fn empty_object_root_with_query_select_the_root_with_mmap_input_and_index_result_using_main_engine(
) -> Result<(), Box<dyn Error>> {
    println ! ("on document empty_object.toml running the query $ (select the root) with Input impl MmapInput and result mode IndexResult");
    let jsonpath_query = JsonPathQuery::parse("$")?;
    let tmp_file = mmap_tmp_file::create_with_contents("{}\n")?;
    let input = unsafe { MmapInput::map_file(&tmp_file)? };
    let engine = MainEngine::compile_query(&jsonpath_query)?;
    let result = engine.run::<_, IndexResult>(&input)?;
    assert_eq!(result.get(), vec![0usize,], "result != expected");
    Ok(())
}
#[test]
fn empty_object_root_with_query_select_the_root_with_mmap_input_and_index_result_using_recursive_engine(
) -> Result<(), Box<dyn Error>> {
    println ! ("on document empty_object.toml running the query $ (select the root) with Input impl MmapInput and result mode IndexResult");
    let jsonpath_query = JsonPathQuery::parse("$")?;
    let tmp_file = mmap_tmp_file::create_with_contents("{}\n")?;
    let input = unsafe { MmapInput::map_file(&tmp_file)? };
    let engine = RecursiveEngine::compile_query(&jsonpath_query)?;
    let result = engine.run::<_, IndexResult>(&input)?;
    assert_eq!(result.get(), vec![0usize,], "result != expected");
    Ok(())
}
#[test]
fn empty_object_root_with_query_select_the_child_named_a_which_does_not_exist_with_owned_bytes_and_count_result_using_main_engine(
) -> Result<(), Box<dyn Error>> {
    println ! ("on document empty_object.toml running the query $.a (select the child named 'a' (which does not exist)) with Input impl OwnedBytes and result mode CountResult");
    let jsonpath_query = JsonPathQuery::parse("$.a")?;
    let raw_json = "{}\n";
    let input = OwnedBytes::new(&raw_json.as_bytes())?;
    let engine = MainEngine::compile_query(&jsonpath_query)?;
    let result = engine.run::<_, CountResult>(&input)?;
    assert_eq!(result.get(), 0u64, "result != expected");
    Ok(())
}
#[test]
fn empty_object_root_with_query_select_the_child_named_a_which_does_not_exist_with_owned_bytes_and_count_result_using_recursive_engine(
) -> Result<(), Box<dyn Error>> {
    println ! ("on document empty_object.toml running the query $.a (select the child named 'a' (which does not exist)) with Input impl OwnedBytes and result mode CountResult");
    let jsonpath_query = JsonPathQuery::parse("$.a")?;
    let raw_json = "{}\n";
    let input = OwnedBytes::new(&raw_json.as_bytes())?;
    let engine = RecursiveEngine::compile_query(&jsonpath_query)?;
    let result = engine.run::<_, CountResult>(&input)?;
    assert_eq!(result.get(), 0u64, "result != expected");
    Ok(())
}
#[test]
fn empty_object_root_with_query_select_the_child_named_a_which_does_not_exist_with_owned_bytes_and_index_result_using_main_engine(
) -> Result<(), Box<dyn Error>> {
    println ! ("on document empty_object.toml running the query $.a (select the child named 'a' (which does not exist)) with Input impl OwnedBytes and result mode IndexResult");
    let jsonpath_query = JsonPathQuery::parse("$.a")?;
    let raw_json = "{}\n";
    let input = OwnedBytes::new(&raw_json.as_bytes())?;
    let engine = MainEngine::compile_query(&jsonpath_query)?;
    let result = engine.run::<_, IndexResult>(&input)?;
    assert_eq!(result.get(), vec![], "result != expected");
    Ok(())
}
#[test]
fn empty_object_root_with_query_select_the_child_named_a_which_does_not_exist_with_owned_bytes_and_index_result_using_recursive_engine(
) -> Result<(), Box<dyn Error>> {
    println ! ("on document empty_object.toml running the query $.a (select the child named 'a' (which does not exist)) with Input impl OwnedBytes and result mode IndexResult");
    let jsonpath_query = JsonPathQuery::parse("$.a")?;
    let raw_json = "{}\n";
    let input = OwnedBytes::new(&raw_json.as_bytes())?;
    let engine = RecursiveEngine::compile_query(&jsonpath_query)?;
    let result = engine.run::<_, IndexResult>(&input)?;
    assert_eq!(result.get(), vec![], "result != expected");
    Ok(())
}
#[test]
fn empty_object_root_with_query_select_the_child_named_a_which_does_not_exist_with_buffered_input_and_count_result_using_main_engine(
) -> Result<(), Box<dyn Error>> {
    println ! ("on document empty_object.toml running the query $.a (select the child named 'a' (which does not exist)) with Input impl BufferedInput and result mode CountResult");
    let jsonpath_query = JsonPathQuery::parse("$.a")?;
    let read_string = ReadString::new("{}\n".to_string());
    let input = BufferedInput::new(read_string);
    let engine = MainEngine::compile_query(&jsonpath_query)?;
    let result = engine.run::<_, CountResult>(&input)?;
    assert_eq!(result.get(), 0u64, "result != expected");
    Ok(())
}
#[test]
fn empty_object_root_with_query_select_the_child_named_a_which_does_not_exist_with_buffered_input_and_count_result_using_recursive_engine(
) -> Result<(), Box<dyn Error>> {
    println ! ("on document empty_object.toml running the query $.a (select the child named 'a' (which does not exist)) with Input impl BufferedInput and result mode CountResult");
    let jsonpath_query = JsonPathQuery::parse("$.a")?;
    let read_string = ReadString::new("{}\n".to_string());
    let input = BufferedInput::new(read_string);
    let engine = RecursiveEngine::compile_query(&jsonpath_query)?;
    let result = engine.run::<_, CountResult>(&input)?;
    assert_eq!(result.get(), 0u64, "result != expected");
    Ok(())
}
#[test]
fn empty_object_root_with_query_select_the_child_named_a_which_does_not_exist_with_buffered_input_and_index_result_using_main_engine(
) -> Result<(), Box<dyn Error>> {
    println ! ("on document empty_object.toml running the query $.a (select the child named 'a' (which does not exist)) with Input impl BufferedInput and result mode IndexResult");
    let jsonpath_query = JsonPathQuery::parse("$.a")?;
    let read_string = ReadString::new("{}\n".to_string());
    let input = BufferedInput::new(read_string);
    let engine = MainEngine::compile_query(&jsonpath_query)?;
    let result = engine.run::<_, IndexResult>(&input)?;
    assert_eq!(result.get(), vec![], "result != expected");
    Ok(())
}
#[test]
fn empty_object_root_with_query_select_the_child_named_a_which_does_not_exist_with_buffered_input_and_index_result_using_recursive_engine(
) -> Result<(), Box<dyn Error>> {
    println ! ("on document empty_object.toml running the query $.a (select the child named 'a' (which does not exist)) with Input impl BufferedInput and result mode IndexResult");
    let jsonpath_query = JsonPathQuery::parse("$.a")?;
    let read_string = ReadString::new("{}\n".to_string());
    let input = BufferedInput::new(read_string);
    let engine = RecursiveEngine::compile_query(&jsonpath_query)?;
    let result = engine.run::<_, IndexResult>(&input)?;
    assert_eq!(result.get(), vec![], "result != expected");
    Ok(())
}
#[test]
fn empty_object_root_with_query_select_the_child_named_a_which_does_not_exist_with_mmap_input_and_count_result_using_main_engine(
) -> Result<(), Box<dyn Error>> {
    println ! ("on document empty_object.toml running the query $.a (select the child named 'a' (which does not exist)) with Input impl MmapInput and result mode CountResult");
    let jsonpath_query = JsonPathQuery::parse("$.a")?;
    let tmp_file = mmap_tmp_file::create_with_contents("{}\n")?;
    let input = unsafe { MmapInput::map_file(&tmp_file)? };
    let engine = MainEngine::compile_query(&jsonpath_query)?;
    let result = engine.run::<_, CountResult>(&input)?;
    assert_eq!(result.get(), 0u64, "result != expected");
    Ok(())
}
#[test]
fn empty_object_root_with_query_select_the_child_named_a_which_does_not_exist_with_mmap_input_and_count_result_using_recursive_engine(
) -> Result<(), Box<dyn Error>> {
    println ! ("on document empty_object.toml running the query $.a (select the child named 'a' (which does not exist)) with Input impl MmapInput and result mode CountResult");
    let jsonpath_query = JsonPathQuery::parse("$.a")?;
    let tmp_file = mmap_tmp_file::create_with_contents("{}\n")?;
    let input = unsafe { MmapInput::map_file(&tmp_file)? };
    let engine = RecursiveEngine::compile_query(&jsonpath_query)?;
    let result = engine.run::<_, CountResult>(&input)?;
    assert_eq!(result.get(), 0u64, "result != expected");
    Ok(())
}
#[test]
fn empty_object_root_with_query_select_the_child_named_a_which_does_not_exist_with_mmap_input_and_index_result_using_main_engine(
) -> Result<(), Box<dyn Error>> {
    println ! ("on document empty_object.toml running the query $.a (select the child named 'a' (which does not exist)) with Input impl MmapInput and result mode IndexResult");
    let jsonpath_query = JsonPathQuery::parse("$.a")?;
    let tmp_file = mmap_tmp_file::create_with_contents("{}\n")?;
    let input = unsafe { MmapInput::map_file(&tmp_file)? };
    let engine = MainEngine::compile_query(&jsonpath_query)?;
    let result = engine.run::<_, IndexResult>(&input)?;
    assert_eq!(result.get(), vec![], "result != expected");
    Ok(())
}
#[test]
fn empty_object_root_with_query_select_the_child_named_a_which_does_not_exist_with_mmap_input_and_index_result_using_recursive_engine(
) -> Result<(), Box<dyn Error>> {
    println ! ("on document empty_object.toml running the query $.a (select the child named 'a' (which does not exist)) with Input impl MmapInput and result mode IndexResult");
    let jsonpath_query = JsonPathQuery::parse("$.a")?;
    let tmp_file = mmap_tmp_file::create_with_contents("{}\n")?;
    let input = unsafe { MmapInput::map_file(&tmp_file)? };
    let engine = RecursiveEngine::compile_query(&jsonpath_query)?;
    let result = engine.run::<_, IndexResult>(&input)?;
    assert_eq!(result.get(), vec![], "result != expected");
    Ok(())
}
#[test]
fn empty_object_root_with_query_select_any_child_there_are_none_with_owned_bytes_and_count_result_using_main_engine(
) -> Result<(), Box<dyn Error>> {
    println ! ("on document empty_object.toml running the query $.* (select any child (there are none)) with Input impl OwnedBytes and result mode CountResult");
    let jsonpath_query = JsonPathQuery::parse("$.*")?;
    let raw_json = "{}\n";
    let input = OwnedBytes::new(&raw_json.as_bytes())?;
    let engine = MainEngine::compile_query(&jsonpath_query)?;
    let result = engine.run::<_, CountResult>(&input)?;
    assert_eq!(result.get(), 0u64, "result != expected");
    Ok(())
}
#[test]
fn empty_object_root_with_query_select_any_child_there_are_none_with_owned_bytes_and_count_result_using_recursive_engine(
) -> Result<(), Box<dyn Error>> {
    println ! ("on document empty_object.toml running the query $.* (select any child (there are none)) with Input impl OwnedBytes and result mode CountResult");
    let jsonpath_query = JsonPathQuery::parse("$.*")?;
    let raw_json = "{}\n";
    let input = OwnedBytes::new(&raw_json.as_bytes())?;
    let engine = RecursiveEngine::compile_query(&jsonpath_query)?;
    let result = engine.run::<_, CountResult>(&input)?;
    assert_eq!(result.get(), 0u64, "result != expected");
    Ok(())
}
#[test]
fn empty_object_root_with_query_select_any_child_there_are_none_with_owned_bytes_and_index_result_using_main_engine(
) -> Result<(), Box<dyn Error>> {
    println ! ("on document empty_object.toml running the query $.* (select any child (there are none)) with Input impl OwnedBytes and result mode IndexResult");
    let jsonpath_query = JsonPathQuery::parse("$.*")?;
    let raw_json = "{}\n";
    let input = OwnedBytes::new(&raw_json.as_bytes())?;
    let engine = MainEngine::compile_query(&jsonpath_query)?;
    let result = engine.run::<_, IndexResult>(&input)?;
    assert_eq!(result.get(), vec![], "result != expected");
    Ok(())
}
#[test]
fn empty_object_root_with_query_select_any_child_there_are_none_with_owned_bytes_and_index_result_using_recursive_engine(
) -> Result<(), Box<dyn Error>> {
    println ! ("on document empty_object.toml running the query $.* (select any child (there are none)) with Input impl OwnedBytes and result mode IndexResult");
    let jsonpath_query = JsonPathQuery::parse("$.*")?;
    let raw_json = "{}\n";
    let input = OwnedBytes::new(&raw_json.as_bytes())?;
    let engine = RecursiveEngine::compile_query(&jsonpath_query)?;
    let result = engine.run::<_, IndexResult>(&input)?;
    assert_eq!(result.get(), vec![], "result != expected");
    Ok(())
}
#[test]
fn empty_object_root_with_query_select_any_child_there_are_none_with_buffered_input_and_count_result_using_main_engine(
) -> Result<(), Box<dyn Error>> {
    println ! ("on document empty_object.toml running the query $.* (select any child (there are none)) with Input impl BufferedInput and result mode CountResult");
    let jsonpath_query = JsonPathQuery::parse("$.*")?;
    let read_string = ReadString::new("{}\n".to_string());
    let input = BufferedInput::new(read_string);
    let engine = MainEngine::compile_query(&jsonpath_query)?;
    let result = engine.run::<_, CountResult>(&input)?;
    assert_eq!(result.get(), 0u64, "result != expected");
    Ok(())
}
#[test]
fn empty_object_root_with_query_select_any_child_there_are_none_with_buffered_input_and_count_result_using_recursive_engine(
) -> Result<(), Box<dyn Error>> {
    println ! ("on document empty_object.toml running the query $.* (select any child (there are none)) with Input impl BufferedInput and result mode CountResult");
    let jsonpath_query = JsonPathQuery::parse("$.*")?;
    let read_string = ReadString::new("{}\n".to_string());
    let input = BufferedInput::new(read_string);
    let engine = RecursiveEngine::compile_query(&jsonpath_query)?;
    let result = engine.run::<_, CountResult>(&input)?;
    assert_eq!(result.get(), 0u64, "result != expected");
    Ok(())
}
#[test]
fn empty_object_root_with_query_select_any_child_there_are_none_with_buffered_input_and_index_result_using_main_engine(
) -> Result<(), Box<dyn Error>> {
    println ! ("on document empty_object.toml running the query $.* (select any child (there are none)) with Input impl BufferedInput and result mode IndexResult");
    let jsonpath_query = JsonPathQuery::parse("$.*")?;
    let read_string = ReadString::new("{}\n".to_string());
    let input = BufferedInput::new(read_string);
    let engine = MainEngine::compile_query(&jsonpath_query)?;
    let result = engine.run::<_, IndexResult>(&input)?;
    assert_eq!(result.get(), vec![], "result != expected");
    Ok(())
}
#[test]
fn empty_object_root_with_query_select_any_child_there_are_none_with_buffered_input_and_index_result_using_recursive_engine(
) -> Result<(), Box<dyn Error>> {
    println ! ("on document empty_object.toml running the query $.* (select any child (there are none)) with Input impl BufferedInput and result mode IndexResult");
    let jsonpath_query = JsonPathQuery::parse("$.*")?;
    let read_string = ReadString::new("{}\n".to_string());
    let input = BufferedInput::new(read_string);
    let engine = RecursiveEngine::compile_query(&jsonpath_query)?;
    let result = engine.run::<_, IndexResult>(&input)?;
    assert_eq!(result.get(), vec![], "result != expected");
    Ok(())
}
#[test]
fn empty_object_root_with_query_select_any_child_there_are_none_with_mmap_input_and_count_result_using_main_engine(
) -> Result<(), Box<dyn Error>> {
    println ! ("on document empty_object.toml running the query $.* (select any child (there are none)) with Input impl MmapInput and result mode CountResult");
    let jsonpath_query = JsonPathQuery::parse("$.*")?;
    let tmp_file = mmap_tmp_file::create_with_contents("{}\n")?;
    let input = unsafe { MmapInput::map_file(&tmp_file)? };
    let engine = MainEngine::compile_query(&jsonpath_query)?;
    let result = engine.run::<_, CountResult>(&input)?;
    assert_eq!(result.get(), 0u64, "result != expected");
    Ok(())
}
#[test]
fn empty_object_root_with_query_select_any_child_there_are_none_with_mmap_input_and_count_result_using_recursive_engine(
) -> Result<(), Box<dyn Error>> {
    println ! ("on document empty_object.toml running the query $.* (select any child (there are none)) with Input impl MmapInput and result mode CountResult");
    let jsonpath_query = JsonPathQuery::parse("$.*")?;
    let tmp_file = mmap_tmp_file::create_with_contents("{}\n")?;
    let input = unsafe { MmapInput::map_file(&tmp_file)? };
    let engine = RecursiveEngine::compile_query(&jsonpath_query)?;
    let result = engine.run::<_, CountResult>(&input)?;
    assert_eq!(result.get(), 0u64, "result != expected");
    Ok(())
}
#[test]
fn empty_object_root_with_query_select_any_child_there_are_none_with_mmap_input_and_index_result_using_main_engine(
) -> Result<(), Box<dyn Error>> {
    println ! ("on document empty_object.toml running the query $.* (select any child (there are none)) with Input impl MmapInput and result mode IndexResult");
    let jsonpath_query = JsonPathQuery::parse("$.*")?;
    let tmp_file = mmap_tmp_file::create_with_contents("{}\n")?;
    let input = unsafe { MmapInput::map_file(&tmp_file)? };
    let engine = MainEngine::compile_query(&jsonpath_query)?;
    let result = engine.run::<_, IndexResult>(&input)?;
    assert_eq!(result.get(), vec![], "result != expected");
    Ok(())
}
#[test]
fn empty_object_root_with_query_select_any_child_there_are_none_with_mmap_input_and_index_result_using_recursive_engine(
) -> Result<(), Box<dyn Error>> {
    println ! ("on document empty_object.toml running the query $.* (select any child (there are none)) with Input impl MmapInput and result mode IndexResult");
    let jsonpath_query = JsonPathQuery::parse("$.*")?;
    let tmp_file = mmap_tmp_file::create_with_contents("{}\n")?;
    let input = unsafe { MmapInput::map_file(&tmp_file)? };
    let engine = RecursiveEngine::compile_query(&jsonpath_query)?;
    let result = engine.run::<_, IndexResult>(&input)?;
    assert_eq!(result.get(), vec![], "result != expected");
    Ok(())
}
#[test]
fn empty_object_root_with_query_select_any_descendant_there_are_none_with_owned_bytes_and_count_result_using_main_engine(
) -> Result<(), Box<dyn Error>> {
    println ! ("on document empty_object.toml running the query $..* (select any descendant (there are none)) with Input impl OwnedBytes and result mode CountResult");
    let jsonpath_query = JsonPathQuery::parse("$..*")?;
    let raw_json = "{}\n";
    let input = OwnedBytes::new(&raw_json.as_bytes())?;
    let engine = MainEngine::compile_query(&jsonpath_query)?;
    let result = engine.run::<_, CountResult>(&input)?;
    assert_eq!(result.get(), 0u64, "result != expected");
    Ok(())
}
#[test]
fn empty_object_root_with_query_select_any_descendant_there_are_none_with_owned_bytes_and_count_result_using_recursive_engine(
) -> Result<(), Box<dyn Error>> {
    println ! ("on document empty_object.toml running the query $..* (select any descendant (there are none)) with Input impl OwnedBytes and result mode CountResult");
    let jsonpath_query = JsonPathQuery::parse("$..*")?;
    let raw_json = "{}\n";
    let input = OwnedBytes::new(&raw_json.as_bytes())?;
    let engine = RecursiveEngine::compile_query(&jsonpath_query)?;
    let result = engine.run::<_, CountResult>(&input)?;
    assert_eq!(result.get(), 0u64, "result != expected");
    Ok(())
}
#[test]
fn empty_object_root_with_query_select_any_descendant_there_are_none_with_owned_bytes_and_index_result_using_main_engine(
) -> Result<(), Box<dyn Error>> {
    println ! ("on document empty_object.toml running the query $..* (select any descendant (there are none)) with Input impl OwnedBytes and result mode IndexResult");
    let jsonpath_query = JsonPathQuery::parse("$..*")?;
    let raw_json = "{}\n";
    let input = OwnedBytes::new(&raw_json.as_bytes())?;
    let engine = MainEngine::compile_query(&jsonpath_query)?;
    let result = engine.run::<_, IndexResult>(&input)?;
    assert_eq!(result.get(), vec![], "result != expected");
    Ok(())
}
#[test]
fn empty_object_root_with_query_select_any_descendant_there_are_none_with_owned_bytes_and_index_result_using_recursive_engine(
) -> Result<(), Box<dyn Error>> {
    println ! ("on document empty_object.toml running the query $..* (select any descendant (there are none)) with Input impl OwnedBytes and result mode IndexResult");
    let jsonpath_query = JsonPathQuery::parse("$..*")?;
    let raw_json = "{}\n";
    let input = OwnedBytes::new(&raw_json.as_bytes())?;
    let engine = RecursiveEngine::compile_query(&jsonpath_query)?;
    let result = engine.run::<_, IndexResult>(&input)?;
    assert_eq!(result.get(), vec![], "result != expected");
    Ok(())
}
#[test]
fn empty_object_root_with_query_select_any_descendant_there_are_none_with_buffered_input_and_count_result_using_main_engine(
) -> Result<(), Box<dyn Error>> {
    println ! ("on document empty_object.toml running the query $..* (select any descendant (there are none)) with Input impl BufferedInput and result mode CountResult");
    let jsonpath_query = JsonPathQuery::parse("$..*")?;
    let read_string = ReadString::new("{}\n".to_string());
    let input = BufferedInput::new(read_string);
    let engine = MainEngine::compile_query(&jsonpath_query)?;
    let result = engine.run::<_, CountResult>(&input)?;
    assert_eq!(result.get(), 0u64, "result != expected");
    Ok(())
}
#[test]
fn empty_object_root_with_query_select_any_descendant_there_are_none_with_buffered_input_and_count_result_using_recursive_engine(
) -> Result<(), Box<dyn Error>> {
    println ! ("on document empty_object.toml running the query $..* (select any descendant (there are none)) with Input impl BufferedInput and result mode CountResult");
    let jsonpath_query = JsonPathQuery::parse("$..*")?;
    let read_string = ReadString::new("{}\n".to_string());
    let input = BufferedInput::new(read_string);
    let engine = RecursiveEngine::compile_query(&jsonpath_query)?;
    let result = engine.run::<_, CountResult>(&input)?;
    assert_eq!(result.get(), 0u64, "result != expected");
    Ok(())
}
#[test]
fn empty_object_root_with_query_select_any_descendant_there_are_none_with_buffered_input_and_index_result_using_main_engine(
) -> Result<(), Box<dyn Error>> {
    println ! ("on document empty_object.toml running the query $..* (select any descendant (there are none)) with Input impl BufferedInput and result mode IndexResult");
    let jsonpath_query = JsonPathQuery::parse("$..*")?;
    let read_string = ReadString::new("{}\n".to_string());
    let input = BufferedInput::new(read_string);
    let engine = MainEngine::compile_query(&jsonpath_query)?;
    let result = engine.run::<_, IndexResult>(&input)?;
    assert_eq!(result.get(), vec![], "result != expected");
    Ok(())
}
#[test]
fn empty_object_root_with_query_select_any_descendant_there_are_none_with_buffered_input_and_index_result_using_recursive_engine(
) -> Result<(), Box<dyn Error>> {
    println ! ("on document empty_object.toml running the query $..* (select any descendant (there are none)) with Input impl BufferedInput and result mode IndexResult");
    let jsonpath_query = JsonPathQuery::parse("$..*")?;
    let read_string = ReadString::new("{}\n".to_string());
    let input = BufferedInput::new(read_string);
    let engine = RecursiveEngine::compile_query(&jsonpath_query)?;
    let result = engine.run::<_, IndexResult>(&input)?;
    assert_eq!(result.get(), vec![], "result != expected");
    Ok(())
}
#[test]
fn empty_object_root_with_query_select_any_descendant_there_are_none_with_mmap_input_and_count_result_using_main_engine(
) -> Result<(), Box<dyn Error>> {
    println ! ("on document empty_object.toml running the query $..* (select any descendant (there are none)) with Input impl MmapInput and result mode CountResult");
    let jsonpath_query = JsonPathQuery::parse("$..*")?;
    let tmp_file = mmap_tmp_file::create_with_contents("{}\n")?;
    let input = unsafe { MmapInput::map_file(&tmp_file)? };
    let engine = MainEngine::compile_query(&jsonpath_query)?;
    let result = engine.run::<_, CountResult>(&input)?;
    assert_eq!(result.get(), 0u64, "result != expected");
    Ok(())
}
#[test]
fn empty_object_root_with_query_select_any_descendant_there_are_none_with_mmap_input_and_count_result_using_recursive_engine(
) -> Result<(), Box<dyn Error>> {
    println ! ("on document empty_object.toml running the query $..* (select any descendant (there are none)) with Input impl MmapInput and result mode CountResult");
    let jsonpath_query = JsonPathQuery::parse("$..*")?;
    let tmp_file = mmap_tmp_file::create_with_contents("{}\n")?;
    let input = unsafe { MmapInput::map_file(&tmp_file)? };
    let engine = RecursiveEngine::compile_query(&jsonpath_query)?;
    let result = engine.run::<_, CountResult>(&input)?;
    assert_eq!(result.get(), 0u64, "result != expected");
    Ok(())
}
#[test]
fn empty_object_root_with_query_select_any_descendant_there_are_none_with_mmap_input_and_index_result_using_main_engine(
) -> Result<(), Box<dyn Error>> {
    println ! ("on document empty_object.toml running the query $..* (select any descendant (there are none)) with Input impl MmapInput and result mode IndexResult");
    let jsonpath_query = JsonPathQuery::parse("$..*")?;
    let tmp_file = mmap_tmp_file::create_with_contents("{}\n")?;
    let input = unsafe { MmapInput::map_file(&tmp_file)? };
    let engine = MainEngine::compile_query(&jsonpath_query)?;
    let result = engine.run::<_, IndexResult>(&input)?;
    assert_eq!(result.get(), vec![], "result != expected");
    Ok(())
}
#[test]
fn empty_object_root_with_query_select_any_descendant_there_are_none_with_mmap_input_and_index_result_using_recursive_engine(
) -> Result<(), Box<dyn Error>> {
    println ! ("on document empty_object.toml running the query $..* (select any descendant (there are none)) with Input impl MmapInput and result mode IndexResult");
    let jsonpath_query = JsonPathQuery::parse("$..*")?;
    let tmp_file = mmap_tmp_file::create_with_contents("{}\n")?;
    let input = unsafe { MmapInput::map_file(&tmp_file)? };
    let engine = RecursiveEngine::compile_query(&jsonpath_query)?;
    let result = engine.run::<_, IndexResult>(&input)?;
    assert_eq!(result.get(), vec![], "result != expected");
    Ok(())
}
