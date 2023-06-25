use pretty_assertions::assert_eq;
use rsonpath::engine::{main::MainEngine, recursive::RecursiveEngine, Compiler, Engine};
use rsonpath::input::*;
use rsonpath::query::JsonPathQuery;
use rsonpath::result::*;
use std::error::Error;
use std::fs;
#[test]
fn compressed_with_query_look_for_b_on_at_least_one_level_of_nesting_with_buffered_input_and_count_result_using_main_engine(
) -> Result<(), Box<dyn Error>> {
    println ! ("on document atomic_descendant.toml running the query $..*..b (look for 'b' on at least one level of nesting) with Input impl BufferedInput and result mode CountResult");
    let jsonpath_query = JsonPathQuery::parse("$..*..b")?;
    let json_file = fs::File::open("./tests/documents/json/compressed/atomic_descendant.json")?;
    let input = BufferedInput::new(json_file);
    let engine = MainEngine::compile_query(&jsonpath_query)?;
    let result = engine.run::<_, CountResult>(&input)?;
    assert_eq!(result.get(), 1u64, "result != expected");
    Ok(())
}
#[test]
fn compressed_with_query_look_for_b_on_at_least_one_level_of_nesting_with_buffered_input_and_count_result_using_recursive_engine(
) -> Result<(), Box<dyn Error>> {
    println ! ("on document atomic_descendant.toml running the query $..*..b (look for 'b' on at least one level of nesting) with Input impl BufferedInput and result mode CountResult");
    let jsonpath_query = JsonPathQuery::parse("$..*..b")?;
    let json_file = fs::File::open("./tests/documents/json/compressed/atomic_descendant.json")?;
    let input = BufferedInput::new(json_file);
    let engine = RecursiveEngine::compile_query(&jsonpath_query)?;
    let result = engine.run::<_, CountResult>(&input)?;
    assert_eq!(result.get(), 1u64, "result != expected");
    Ok(())
}
#[test]
fn compressed_with_query_look_for_b_on_at_least_one_level_of_nesting_with_buffered_input_and_index_result_using_main_engine(
) -> Result<(), Box<dyn Error>> {
    println ! ("on document atomic_descendant.toml running the query $..*..b (look for 'b' on at least one level of nesting) with Input impl BufferedInput and result mode IndexResult");
    let jsonpath_query = JsonPathQuery::parse("$..*..b")?;
    let json_file = fs::File::open("./tests/documents/json/compressed/atomic_descendant.json")?;
    let input = BufferedInput::new(json_file);
    let engine = MainEngine::compile_query(&jsonpath_query)?;
    let result = engine.run::<_, IndexResult>(&input)?;
    assert_eq!(result.get(), vec![18usize,], "result != expected");
    Ok(())
}
#[test]
fn compressed_with_query_look_for_b_on_at_least_one_level_of_nesting_with_buffered_input_and_index_result_using_recursive_engine(
) -> Result<(), Box<dyn Error>> {
    println ! ("on document atomic_descendant.toml running the query $..*..b (look for 'b' on at least one level of nesting) with Input impl BufferedInput and result mode IndexResult");
    let jsonpath_query = JsonPathQuery::parse("$..*..b")?;
    let json_file = fs::File::open("./tests/documents/json/compressed/atomic_descendant.json")?;
    let input = BufferedInput::new(json_file);
    let engine = RecursiveEngine::compile_query(&jsonpath_query)?;
    let result = engine.run::<_, IndexResult>(&input)?;
    assert_eq!(result.get(), vec![18usize,], "result != expected");
    Ok(())
}
#[test]
fn compressed_with_query_look_for_b_on_at_least_one_level_of_nesting_with_mmap_input_and_count_result_using_main_engine(
) -> Result<(), Box<dyn Error>> {
    println ! ("on document atomic_descendant.toml running the query $..*..b (look for 'b' on at least one level of nesting) with Input impl MmapInput and result mode CountResult");
    let jsonpath_query = JsonPathQuery::parse("$..*..b")?;
    let json_file = fs::File::open("./tests/documents/json/compressed/atomic_descendant.json")?;
    let input = unsafe { MmapInput::map_file(&json_file)? };
    let engine = MainEngine::compile_query(&jsonpath_query)?;
    let result = engine.run::<_, CountResult>(&input)?;
    assert_eq!(result.get(), 1u64, "result != expected");
    Ok(())
}
#[test]
fn compressed_with_query_look_for_b_on_at_least_one_level_of_nesting_with_mmap_input_and_count_result_using_recursive_engine(
) -> Result<(), Box<dyn Error>> {
    println ! ("on document atomic_descendant.toml running the query $..*..b (look for 'b' on at least one level of nesting) with Input impl MmapInput and result mode CountResult");
    let jsonpath_query = JsonPathQuery::parse("$..*..b")?;
    let json_file = fs::File::open("./tests/documents/json/compressed/atomic_descendant.json")?;
    let input = unsafe { MmapInput::map_file(&json_file)? };
    let engine = RecursiveEngine::compile_query(&jsonpath_query)?;
    let result = engine.run::<_, CountResult>(&input)?;
    assert_eq!(result.get(), 1u64, "result != expected");
    Ok(())
}
#[test]
fn compressed_with_query_look_for_b_on_at_least_one_level_of_nesting_with_mmap_input_and_index_result_using_main_engine(
) -> Result<(), Box<dyn Error>> {
    println ! ("on document atomic_descendant.toml running the query $..*..b (look for 'b' on at least one level of nesting) with Input impl MmapInput and result mode IndexResult");
    let jsonpath_query = JsonPathQuery::parse("$..*..b")?;
    let json_file = fs::File::open("./tests/documents/json/compressed/atomic_descendant.json")?;
    let input = unsafe { MmapInput::map_file(&json_file)? };
    let engine = MainEngine::compile_query(&jsonpath_query)?;
    let result = engine.run::<_, IndexResult>(&input)?;
    assert_eq!(result.get(), vec![18usize,], "result != expected");
    Ok(())
}
#[test]
fn compressed_with_query_look_for_b_on_at_least_one_level_of_nesting_with_mmap_input_and_index_result_using_recursive_engine(
) -> Result<(), Box<dyn Error>> {
    println ! ("on document atomic_descendant.toml running the query $..*..b (look for 'b' on at least one level of nesting) with Input impl MmapInput and result mode IndexResult");
    let jsonpath_query = JsonPathQuery::parse("$..*..b")?;
    let json_file = fs::File::open("./tests/documents/json/compressed/atomic_descendant.json")?;
    let input = unsafe { MmapInput::map_file(&json_file)? };
    let engine = RecursiveEngine::compile_query(&jsonpath_query)?;
    let result = engine.run::<_, IndexResult>(&input)?;
    assert_eq!(result.get(), vec![18usize,], "result != expected");
    Ok(())
}
#[test]
fn compressed_with_query_look_for_b_on_at_least_one_level_of_nesting_with_owned_bytes_and_count_result_using_main_engine(
) -> Result<(), Box<dyn Error>> {
    println ! ("on document atomic_descendant.toml running the query $..*..b (look for 'b' on at least one level of nesting) with Input impl OwnedBytes and result mode CountResult");
    let jsonpath_query = JsonPathQuery::parse("$..*..b")?;
    let raw_json = fs::read_to_string("./tests/documents/json/compressed/atomic_descendant.json")?;
    let input = OwnedBytes::new(&raw_json.as_bytes())?;
    let engine = MainEngine::compile_query(&jsonpath_query)?;
    let result = engine.run::<_, CountResult>(&input)?;
    assert_eq!(result.get(), 1u64, "result != expected");
    Ok(())
}
#[test]
fn compressed_with_query_look_for_b_on_at_least_one_level_of_nesting_with_owned_bytes_and_count_result_using_recursive_engine(
) -> Result<(), Box<dyn Error>> {
    println ! ("on document atomic_descendant.toml running the query $..*..b (look for 'b' on at least one level of nesting) with Input impl OwnedBytes and result mode CountResult");
    let jsonpath_query = JsonPathQuery::parse("$..*..b")?;
    let raw_json = fs::read_to_string("./tests/documents/json/compressed/atomic_descendant.json")?;
    let input = OwnedBytes::new(&raw_json.as_bytes())?;
    let engine = RecursiveEngine::compile_query(&jsonpath_query)?;
    let result = engine.run::<_, CountResult>(&input)?;
    assert_eq!(result.get(), 1u64, "result != expected");
    Ok(())
}
#[test]
fn compressed_with_query_look_for_b_on_at_least_one_level_of_nesting_with_owned_bytes_and_index_result_using_main_engine(
) -> Result<(), Box<dyn Error>> {
    println ! ("on document atomic_descendant.toml running the query $..*..b (look for 'b' on at least one level of nesting) with Input impl OwnedBytes and result mode IndexResult");
    let jsonpath_query = JsonPathQuery::parse("$..*..b")?;
    let raw_json = fs::read_to_string("./tests/documents/json/compressed/atomic_descendant.json")?;
    let input = OwnedBytes::new(&raw_json.as_bytes())?;
    let engine = MainEngine::compile_query(&jsonpath_query)?;
    let result = engine.run::<_, IndexResult>(&input)?;
    assert_eq!(result.get(), vec![18usize,], "result != expected");
    Ok(())
}
#[test]
fn compressed_with_query_look_for_b_on_at_least_one_level_of_nesting_with_owned_bytes_and_index_result_using_recursive_engine(
) -> Result<(), Box<dyn Error>> {
    println ! ("on document atomic_descendant.toml running the query $..*..b (look for 'b' on at least one level of nesting) with Input impl OwnedBytes and result mode IndexResult");
    let jsonpath_query = JsonPathQuery::parse("$..*..b")?;
    let raw_json = fs::read_to_string("./tests/documents/json/compressed/atomic_descendant.json")?;
    let input = OwnedBytes::new(&raw_json.as_bytes())?;
    let engine = RecursiveEngine::compile_query(&jsonpath_query)?;
    let result = engine.run::<_, IndexResult>(&input)?;
    assert_eq!(result.get(), vec![18usize,], "result != expected");
    Ok(())
}
#[test]
fn compressed_with_query_look_for_descendants_of_an_atomic_value_with_buffered_input_and_count_result_using_main_engine(
) -> Result<(), Box<dyn Error>> {
    println ! ("on document atomic_descendant.toml running the query $..a..b (look for descendants of an atomic value) with Input impl BufferedInput and result mode CountResult");
    let jsonpath_query = JsonPathQuery::parse("$..a..b")?;
    let json_file = fs::File::open("./tests/documents/json/compressed/atomic_descendant.json")?;
    let input = BufferedInput::new(json_file);
    let engine = MainEngine::compile_query(&jsonpath_query)?;
    let result = engine.run::<_, CountResult>(&input)?;
    assert_eq!(result.get(), 0u64, "result != expected");
    Ok(())
}
#[test]
fn compressed_with_query_look_for_descendants_of_an_atomic_value_with_buffered_input_and_count_result_using_recursive_engine(
) -> Result<(), Box<dyn Error>> {
    println ! ("on document atomic_descendant.toml running the query $..a..b (look for descendants of an atomic value) with Input impl BufferedInput and result mode CountResult");
    let jsonpath_query = JsonPathQuery::parse("$..a..b")?;
    let json_file = fs::File::open("./tests/documents/json/compressed/atomic_descendant.json")?;
    let input = BufferedInput::new(json_file);
    let engine = RecursiveEngine::compile_query(&jsonpath_query)?;
    let result = engine.run::<_, CountResult>(&input)?;
    assert_eq!(result.get(), 0u64, "result != expected");
    Ok(())
}
#[test]
fn compressed_with_query_look_for_descendants_of_an_atomic_value_with_buffered_input_and_index_result_using_main_engine(
) -> Result<(), Box<dyn Error>> {
    println ! ("on document atomic_descendant.toml running the query $..a..b (look for descendants of an atomic value) with Input impl BufferedInput and result mode IndexResult");
    let jsonpath_query = JsonPathQuery::parse("$..a..b")?;
    let json_file = fs::File::open("./tests/documents/json/compressed/atomic_descendant.json")?;
    let input = BufferedInput::new(json_file);
    let engine = MainEngine::compile_query(&jsonpath_query)?;
    let result = engine.run::<_, IndexResult>(&input)?;
    assert_eq!(result.get(), vec![], "result != expected");
    Ok(())
}
#[test]
fn compressed_with_query_look_for_descendants_of_an_atomic_value_with_buffered_input_and_index_result_using_recursive_engine(
) -> Result<(), Box<dyn Error>> {
    println ! ("on document atomic_descendant.toml running the query $..a..b (look for descendants of an atomic value) with Input impl BufferedInput and result mode IndexResult");
    let jsonpath_query = JsonPathQuery::parse("$..a..b")?;
    let json_file = fs::File::open("./tests/documents/json/compressed/atomic_descendant.json")?;
    let input = BufferedInput::new(json_file);
    let engine = RecursiveEngine::compile_query(&jsonpath_query)?;
    let result = engine.run::<_, IndexResult>(&input)?;
    assert_eq!(result.get(), vec![], "result != expected");
    Ok(())
}
#[test]
fn compressed_with_query_look_for_descendants_of_an_atomic_value_with_mmap_input_and_count_result_using_main_engine(
) -> Result<(), Box<dyn Error>> {
    println ! ("on document atomic_descendant.toml running the query $..a..b (look for descendants of an atomic value) with Input impl MmapInput and result mode CountResult");
    let jsonpath_query = JsonPathQuery::parse("$..a..b")?;
    let json_file = fs::File::open("./tests/documents/json/compressed/atomic_descendant.json")?;
    let input = unsafe { MmapInput::map_file(&json_file)? };
    let engine = MainEngine::compile_query(&jsonpath_query)?;
    let result = engine.run::<_, CountResult>(&input)?;
    assert_eq!(result.get(), 0u64, "result != expected");
    Ok(())
}
#[test]
fn compressed_with_query_look_for_descendants_of_an_atomic_value_with_mmap_input_and_count_result_using_recursive_engine(
) -> Result<(), Box<dyn Error>> {
    println ! ("on document atomic_descendant.toml running the query $..a..b (look for descendants of an atomic value) with Input impl MmapInput and result mode CountResult");
    let jsonpath_query = JsonPathQuery::parse("$..a..b")?;
    let json_file = fs::File::open("./tests/documents/json/compressed/atomic_descendant.json")?;
    let input = unsafe { MmapInput::map_file(&json_file)? };
    let engine = RecursiveEngine::compile_query(&jsonpath_query)?;
    let result = engine.run::<_, CountResult>(&input)?;
    assert_eq!(result.get(), 0u64, "result != expected");
    Ok(())
}
#[test]
fn compressed_with_query_look_for_descendants_of_an_atomic_value_with_mmap_input_and_index_result_using_main_engine(
) -> Result<(), Box<dyn Error>> {
    println ! ("on document atomic_descendant.toml running the query $..a..b (look for descendants of an atomic value) with Input impl MmapInput and result mode IndexResult");
    let jsonpath_query = JsonPathQuery::parse("$..a..b")?;
    let json_file = fs::File::open("./tests/documents/json/compressed/atomic_descendant.json")?;
    let input = unsafe { MmapInput::map_file(&json_file)? };
    let engine = MainEngine::compile_query(&jsonpath_query)?;
    let result = engine.run::<_, IndexResult>(&input)?;
    assert_eq!(result.get(), vec![], "result != expected");
    Ok(())
}
#[test]
fn compressed_with_query_look_for_descendants_of_an_atomic_value_with_mmap_input_and_index_result_using_recursive_engine(
) -> Result<(), Box<dyn Error>> {
    println ! ("on document atomic_descendant.toml running the query $..a..b (look for descendants of an atomic value) with Input impl MmapInput and result mode IndexResult");
    let jsonpath_query = JsonPathQuery::parse("$..a..b")?;
    let json_file = fs::File::open("./tests/documents/json/compressed/atomic_descendant.json")?;
    let input = unsafe { MmapInput::map_file(&json_file)? };
    let engine = RecursiveEngine::compile_query(&jsonpath_query)?;
    let result = engine.run::<_, IndexResult>(&input)?;
    assert_eq!(result.get(), vec![], "result != expected");
    Ok(())
}
#[test]
fn compressed_with_query_look_for_descendants_of_an_atomic_value_with_owned_bytes_and_count_result_using_main_engine(
) -> Result<(), Box<dyn Error>> {
    println ! ("on document atomic_descendant.toml running the query $..a..b (look for descendants of an atomic value) with Input impl OwnedBytes and result mode CountResult");
    let jsonpath_query = JsonPathQuery::parse("$..a..b")?;
    let raw_json = fs::read_to_string("./tests/documents/json/compressed/atomic_descendant.json")?;
    let input = OwnedBytes::new(&raw_json.as_bytes())?;
    let engine = MainEngine::compile_query(&jsonpath_query)?;
    let result = engine.run::<_, CountResult>(&input)?;
    assert_eq!(result.get(), 0u64, "result != expected");
    Ok(())
}
#[test]
fn compressed_with_query_look_for_descendants_of_an_atomic_value_with_owned_bytes_and_count_result_using_recursive_engine(
) -> Result<(), Box<dyn Error>> {
    println ! ("on document atomic_descendant.toml running the query $..a..b (look for descendants of an atomic value) with Input impl OwnedBytes and result mode CountResult");
    let jsonpath_query = JsonPathQuery::parse("$..a..b")?;
    let raw_json = fs::read_to_string("./tests/documents/json/compressed/atomic_descendant.json")?;
    let input = OwnedBytes::new(&raw_json.as_bytes())?;
    let engine = RecursiveEngine::compile_query(&jsonpath_query)?;
    let result = engine.run::<_, CountResult>(&input)?;
    assert_eq!(result.get(), 0u64, "result != expected");
    Ok(())
}
#[test]
fn compressed_with_query_look_for_descendants_of_an_atomic_value_with_owned_bytes_and_index_result_using_main_engine(
) -> Result<(), Box<dyn Error>> {
    println ! ("on document atomic_descendant.toml running the query $..a..b (look for descendants of an atomic value) with Input impl OwnedBytes and result mode IndexResult");
    let jsonpath_query = JsonPathQuery::parse("$..a..b")?;
    let raw_json = fs::read_to_string("./tests/documents/json/compressed/atomic_descendant.json")?;
    let input = OwnedBytes::new(&raw_json.as_bytes())?;
    let engine = MainEngine::compile_query(&jsonpath_query)?;
    let result = engine.run::<_, IndexResult>(&input)?;
    assert_eq!(result.get(), vec![], "result != expected");
    Ok(())
}
#[test]
fn compressed_with_query_look_for_descendants_of_an_atomic_value_with_owned_bytes_and_index_result_using_recursive_engine(
) -> Result<(), Box<dyn Error>> {
    println ! ("on document atomic_descendant.toml running the query $..a..b (look for descendants of an atomic value) with Input impl OwnedBytes and result mode IndexResult");
    let jsonpath_query = JsonPathQuery::parse("$..a..b")?;
    let raw_json = fs::read_to_string("./tests/documents/json/compressed/atomic_descendant.json")?;
    let input = OwnedBytes::new(&raw_json.as_bytes())?;
    let engine = RecursiveEngine::compile_query(&jsonpath_query)?;
    let result = engine.run::<_, IndexResult>(&input)?;
    assert_eq!(result.get(), vec![], "result != expected");
    Ok(())
}
#[test]
fn compressed_with_query_select_a_number_that_is_a_child_with_buffered_input_and_count_result_using_main_engine(
) -> Result<(), Box<dyn Error>> {
    println ! ("on document atomic_descendant.toml running the query $..a (select a number that is a child) with Input impl BufferedInput and result mode CountResult");
    let jsonpath_query = JsonPathQuery::parse("$..a")?;
    let json_file = fs::File::open("./tests/documents/json/compressed/atomic_descendant.json")?;
    let input = BufferedInput::new(json_file);
    let engine = MainEngine::compile_query(&jsonpath_query)?;
    let result = engine.run::<_, CountResult>(&input)?;
    assert_eq!(result.get(), 1u64, "result != expected");
    Ok(())
}
#[test]
fn compressed_with_query_select_a_number_that_is_a_child_with_buffered_input_and_count_result_using_recursive_engine(
) -> Result<(), Box<dyn Error>> {
    println ! ("on document atomic_descendant.toml running the query $..a (select a number that is a child) with Input impl BufferedInput and result mode CountResult");
    let jsonpath_query = JsonPathQuery::parse("$..a")?;
    let json_file = fs::File::open("./tests/documents/json/compressed/atomic_descendant.json")?;
    let input = BufferedInput::new(json_file);
    let engine = RecursiveEngine::compile_query(&jsonpath_query)?;
    let result = engine.run::<_, CountResult>(&input)?;
    assert_eq!(result.get(), 1u64, "result != expected");
    Ok(())
}
#[test]
fn compressed_with_query_select_a_number_that_is_a_child_with_buffered_input_and_index_result_using_main_engine(
) -> Result<(), Box<dyn Error>> {
    println ! ("on document atomic_descendant.toml running the query $..a (select a number that is a child) with Input impl BufferedInput and result mode IndexResult");
    let jsonpath_query = JsonPathQuery::parse("$..a")?;
    let json_file = fs::File::open("./tests/documents/json/compressed/atomic_descendant.json")?;
    let input = BufferedInput::new(json_file);
    let engine = MainEngine::compile_query(&jsonpath_query)?;
    let result = engine.run::<_, IndexResult>(&input)?;
    assert_eq!(result.get(), vec![5usize,], "result != expected");
    Ok(())
}
#[test]
fn compressed_with_query_select_a_number_that_is_a_child_with_buffered_input_and_index_result_using_recursive_engine(
) -> Result<(), Box<dyn Error>> {
    println ! ("on document atomic_descendant.toml running the query $..a (select a number that is a child) with Input impl BufferedInput and result mode IndexResult");
    let jsonpath_query = JsonPathQuery::parse("$..a")?;
    let json_file = fs::File::open("./tests/documents/json/compressed/atomic_descendant.json")?;
    let input = BufferedInput::new(json_file);
    let engine = RecursiveEngine::compile_query(&jsonpath_query)?;
    let result = engine.run::<_, IndexResult>(&input)?;
    assert_eq!(result.get(), vec![5usize,], "result != expected");
    Ok(())
}
#[test]
fn compressed_with_query_select_a_number_that_is_a_child_with_mmap_input_and_count_result_using_main_engine(
) -> Result<(), Box<dyn Error>> {
    println ! ("on document atomic_descendant.toml running the query $..a (select a number that is a child) with Input impl MmapInput and result mode CountResult");
    let jsonpath_query = JsonPathQuery::parse("$..a")?;
    let json_file = fs::File::open("./tests/documents/json/compressed/atomic_descendant.json")?;
    let input = unsafe { MmapInput::map_file(&json_file)? };
    let engine = MainEngine::compile_query(&jsonpath_query)?;
    let result = engine.run::<_, CountResult>(&input)?;
    assert_eq!(result.get(), 1u64, "result != expected");
    Ok(())
}
#[test]
fn compressed_with_query_select_a_number_that_is_a_child_with_mmap_input_and_count_result_using_recursive_engine(
) -> Result<(), Box<dyn Error>> {
    println ! ("on document atomic_descendant.toml running the query $..a (select a number that is a child) with Input impl MmapInput and result mode CountResult");
    let jsonpath_query = JsonPathQuery::parse("$..a")?;
    let json_file = fs::File::open("./tests/documents/json/compressed/atomic_descendant.json")?;
    let input = unsafe { MmapInput::map_file(&json_file)? };
    let engine = RecursiveEngine::compile_query(&jsonpath_query)?;
    let result = engine.run::<_, CountResult>(&input)?;
    assert_eq!(result.get(), 1u64, "result != expected");
    Ok(())
}
#[test]
fn compressed_with_query_select_a_number_that_is_a_child_with_mmap_input_and_index_result_using_main_engine(
) -> Result<(), Box<dyn Error>> {
    println ! ("on document atomic_descendant.toml running the query $..a (select a number that is a child) with Input impl MmapInput and result mode IndexResult");
    let jsonpath_query = JsonPathQuery::parse("$..a")?;
    let json_file = fs::File::open("./tests/documents/json/compressed/atomic_descendant.json")?;
    let input = unsafe { MmapInput::map_file(&json_file)? };
    let engine = MainEngine::compile_query(&jsonpath_query)?;
    let result = engine.run::<_, IndexResult>(&input)?;
    assert_eq!(result.get(), vec![5usize,], "result != expected");
    Ok(())
}
#[test]
fn compressed_with_query_select_a_number_that_is_a_child_with_mmap_input_and_index_result_using_recursive_engine(
) -> Result<(), Box<dyn Error>> {
    println ! ("on document atomic_descendant.toml running the query $..a (select a number that is a child) with Input impl MmapInput and result mode IndexResult");
    let jsonpath_query = JsonPathQuery::parse("$..a")?;
    let json_file = fs::File::open("./tests/documents/json/compressed/atomic_descendant.json")?;
    let input = unsafe { MmapInput::map_file(&json_file)? };
    let engine = RecursiveEngine::compile_query(&jsonpath_query)?;
    let result = engine.run::<_, IndexResult>(&input)?;
    assert_eq!(result.get(), vec![5usize,], "result != expected");
    Ok(())
}
#[test]
fn compressed_with_query_select_a_number_that_is_a_child_with_owned_bytes_and_count_result_using_main_engine(
) -> Result<(), Box<dyn Error>> {
    println ! ("on document atomic_descendant.toml running the query $..a (select a number that is a child) with Input impl OwnedBytes and result mode CountResult");
    let jsonpath_query = JsonPathQuery::parse("$..a")?;
    let raw_json = fs::read_to_string("./tests/documents/json/compressed/atomic_descendant.json")?;
    let input = OwnedBytes::new(&raw_json.as_bytes())?;
    let engine = MainEngine::compile_query(&jsonpath_query)?;
    let result = engine.run::<_, CountResult>(&input)?;
    assert_eq!(result.get(), 1u64, "result != expected");
    Ok(())
}
#[test]
fn compressed_with_query_select_a_number_that_is_a_child_with_owned_bytes_and_count_result_using_recursive_engine(
) -> Result<(), Box<dyn Error>> {
    println ! ("on document atomic_descendant.toml running the query $..a (select a number that is a child) with Input impl OwnedBytes and result mode CountResult");
    let jsonpath_query = JsonPathQuery::parse("$..a")?;
    let raw_json = fs::read_to_string("./tests/documents/json/compressed/atomic_descendant.json")?;
    let input = OwnedBytes::new(&raw_json.as_bytes())?;
    let engine = RecursiveEngine::compile_query(&jsonpath_query)?;
    let result = engine.run::<_, CountResult>(&input)?;
    assert_eq!(result.get(), 1u64, "result != expected");
    Ok(())
}
#[test]
fn compressed_with_query_select_a_number_that_is_a_child_with_owned_bytes_and_index_result_using_main_engine(
) -> Result<(), Box<dyn Error>> {
    println ! ("on document atomic_descendant.toml running the query $..a (select a number that is a child) with Input impl OwnedBytes and result mode IndexResult");
    let jsonpath_query = JsonPathQuery::parse("$..a")?;
    let raw_json = fs::read_to_string("./tests/documents/json/compressed/atomic_descendant.json")?;
    let input = OwnedBytes::new(&raw_json.as_bytes())?;
    let engine = MainEngine::compile_query(&jsonpath_query)?;
    let result = engine.run::<_, IndexResult>(&input)?;
    assert_eq!(result.get(), vec![5usize,], "result != expected");
    Ok(())
}
#[test]
fn compressed_with_query_select_a_number_that_is_a_child_with_owned_bytes_and_index_result_using_recursive_engine(
) -> Result<(), Box<dyn Error>> {
    println ! ("on document atomic_descendant.toml running the query $..a (select a number that is a child) with Input impl OwnedBytes and result mode IndexResult");
    let jsonpath_query = JsonPathQuery::parse("$..a")?;
    let raw_json = fs::read_to_string("./tests/documents/json/compressed/atomic_descendant.json")?;
    let input = OwnedBytes::new(&raw_json.as_bytes())?;
    let engine = RecursiveEngine::compile_query(&jsonpath_query)?;
    let result = engine.run::<_, IndexResult>(&input)?;
    assert_eq!(result.get(), vec![5usize,], "result != expected");
    Ok(())
}
#[test]
fn compressed_with_query_select_all_decsendants_with_buffered_input_and_count_result_using_main_engine(
) -> Result<(), Box<dyn Error>> {
    println ! ("on document atomic_descendant.toml running the query $..* (select all decsendants) with Input impl BufferedInput and result mode CountResult");
    let jsonpath_query = JsonPathQuery::parse("$..*")?;
    let json_file = fs::File::open("./tests/documents/json/compressed/atomic_descendant.json")?;
    let input = BufferedInput::new(json_file);
    let engine = MainEngine::compile_query(&jsonpath_query)?;
    let result = engine.run::<_, CountResult>(&input)?;
    assert_eq!(result.get(), 4u64, "result != expected");
    Ok(())
}
#[test]
fn compressed_with_query_select_all_decsendants_with_buffered_input_and_count_result_using_recursive_engine(
) -> Result<(), Box<dyn Error>> {
    println ! ("on document atomic_descendant.toml running the query $..* (select all decsendants) with Input impl BufferedInput and result mode CountResult");
    let jsonpath_query = JsonPathQuery::parse("$..*")?;
    let json_file = fs::File::open("./tests/documents/json/compressed/atomic_descendant.json")?;
    let input = BufferedInput::new(json_file);
    let engine = RecursiveEngine::compile_query(&jsonpath_query)?;
    let result = engine.run::<_, CountResult>(&input)?;
    assert_eq!(result.get(), 4u64, "result != expected");
    Ok(())
}
#[test]
fn compressed_with_query_select_all_decsendants_with_buffered_input_and_index_result_using_main_engine(
) -> Result<(), Box<dyn Error>> {
    println ! ("on document atomic_descendant.toml running the query $..* (select all decsendants) with Input impl BufferedInput and result mode IndexResult");
    let jsonpath_query = JsonPathQuery::parse("$..*")?;
    let json_file = fs::File::open("./tests/documents/json/compressed/atomic_descendant.json")?;
    let input = BufferedInput::new(json_file);
    let engine = MainEngine::compile_query(&jsonpath_query)?;
    let result = engine.run::<_, IndexResult>(&input)?;
    assert_eq!(
        result.get(),
        vec![5usize, 12usize, 13usize, 18usize,],
        "result != expected"
    );
    Ok(())
}
#[test]
fn compressed_with_query_select_all_decsendants_with_buffered_input_and_index_result_using_recursive_engine(
) -> Result<(), Box<dyn Error>> {
    println ! ("on document atomic_descendant.toml running the query $..* (select all decsendants) with Input impl BufferedInput and result mode IndexResult");
    let jsonpath_query = JsonPathQuery::parse("$..*")?;
    let json_file = fs::File::open("./tests/documents/json/compressed/atomic_descendant.json")?;
    let input = BufferedInput::new(json_file);
    let engine = RecursiveEngine::compile_query(&jsonpath_query)?;
    let result = engine.run::<_, IndexResult>(&input)?;
    assert_eq!(
        result.get(),
        vec![5usize, 12usize, 13usize, 18usize,],
        "result != expected"
    );
    Ok(())
}
#[test]
fn compressed_with_query_select_all_decsendants_with_mmap_input_and_count_result_using_main_engine(
) -> Result<(), Box<dyn Error>> {
    println ! ("on document atomic_descendant.toml running the query $..* (select all decsendants) with Input impl MmapInput and result mode CountResult");
    let jsonpath_query = JsonPathQuery::parse("$..*")?;
    let json_file = fs::File::open("./tests/documents/json/compressed/atomic_descendant.json")?;
    let input = unsafe { MmapInput::map_file(&json_file)? };
    let engine = MainEngine::compile_query(&jsonpath_query)?;
    let result = engine.run::<_, CountResult>(&input)?;
    assert_eq!(result.get(), 4u64, "result != expected");
    Ok(())
}
#[test]
fn compressed_with_query_select_all_decsendants_with_mmap_input_and_count_result_using_recursive_engine(
) -> Result<(), Box<dyn Error>> {
    println ! ("on document atomic_descendant.toml running the query $..* (select all decsendants) with Input impl MmapInput and result mode CountResult");
    let jsonpath_query = JsonPathQuery::parse("$..*")?;
    let json_file = fs::File::open("./tests/documents/json/compressed/atomic_descendant.json")?;
    let input = unsafe { MmapInput::map_file(&json_file)? };
    let engine = RecursiveEngine::compile_query(&jsonpath_query)?;
    let result = engine.run::<_, CountResult>(&input)?;
    assert_eq!(result.get(), 4u64, "result != expected");
    Ok(())
}
#[test]
fn compressed_with_query_select_all_decsendants_with_mmap_input_and_index_result_using_main_engine(
) -> Result<(), Box<dyn Error>> {
    println ! ("on document atomic_descendant.toml running the query $..* (select all decsendants) with Input impl MmapInput and result mode IndexResult");
    let jsonpath_query = JsonPathQuery::parse("$..*")?;
    let json_file = fs::File::open("./tests/documents/json/compressed/atomic_descendant.json")?;
    let input = unsafe { MmapInput::map_file(&json_file)? };
    let engine = MainEngine::compile_query(&jsonpath_query)?;
    let result = engine.run::<_, IndexResult>(&input)?;
    assert_eq!(
        result.get(),
        vec![5usize, 12usize, 13usize, 18usize,],
        "result != expected"
    );
    Ok(())
}
#[test]
fn compressed_with_query_select_all_decsendants_with_mmap_input_and_index_result_using_recursive_engine(
) -> Result<(), Box<dyn Error>> {
    println ! ("on document atomic_descendant.toml running the query $..* (select all decsendants) with Input impl MmapInput and result mode IndexResult");
    let jsonpath_query = JsonPathQuery::parse("$..*")?;
    let json_file = fs::File::open("./tests/documents/json/compressed/atomic_descendant.json")?;
    let input = unsafe { MmapInput::map_file(&json_file)? };
    let engine = RecursiveEngine::compile_query(&jsonpath_query)?;
    let result = engine.run::<_, IndexResult>(&input)?;
    assert_eq!(
        result.get(),
        vec![5usize, 12usize, 13usize, 18usize,],
        "result != expected"
    );
    Ok(())
}
#[test]
fn compressed_with_query_select_all_decsendants_with_owned_bytes_and_count_result_using_main_engine(
) -> Result<(), Box<dyn Error>> {
    println ! ("on document atomic_descendant.toml running the query $..* (select all decsendants) with Input impl OwnedBytes and result mode CountResult");
    let jsonpath_query = JsonPathQuery::parse("$..*")?;
    let raw_json = fs::read_to_string("./tests/documents/json/compressed/atomic_descendant.json")?;
    let input = OwnedBytes::new(&raw_json.as_bytes())?;
    let engine = MainEngine::compile_query(&jsonpath_query)?;
    let result = engine.run::<_, CountResult>(&input)?;
    assert_eq!(result.get(), 4u64, "result != expected");
    Ok(())
}
#[test]
fn compressed_with_query_select_all_decsendants_with_owned_bytes_and_count_result_using_recursive_engine(
) -> Result<(), Box<dyn Error>> {
    println ! ("on document atomic_descendant.toml running the query $..* (select all decsendants) with Input impl OwnedBytes and result mode CountResult");
    let jsonpath_query = JsonPathQuery::parse("$..*")?;
    let raw_json = fs::read_to_string("./tests/documents/json/compressed/atomic_descendant.json")?;
    let input = OwnedBytes::new(&raw_json.as_bytes())?;
    let engine = RecursiveEngine::compile_query(&jsonpath_query)?;
    let result = engine.run::<_, CountResult>(&input)?;
    assert_eq!(result.get(), 4u64, "result != expected");
    Ok(())
}
#[test]
fn compressed_with_query_select_all_decsendants_with_owned_bytes_and_index_result_using_main_engine(
) -> Result<(), Box<dyn Error>> {
    println ! ("on document atomic_descendant.toml running the query $..* (select all decsendants) with Input impl OwnedBytes and result mode IndexResult");
    let jsonpath_query = JsonPathQuery::parse("$..*")?;
    let raw_json = fs::read_to_string("./tests/documents/json/compressed/atomic_descendant.json")?;
    let input = OwnedBytes::new(&raw_json.as_bytes())?;
    let engine = MainEngine::compile_query(&jsonpath_query)?;
    let result = engine.run::<_, IndexResult>(&input)?;
    assert_eq!(
        result.get(),
        vec![5usize, 12usize, 13usize, 18usize,],
        "result != expected"
    );
    Ok(())
}
#[test]
fn compressed_with_query_select_all_decsendants_with_owned_bytes_and_index_result_using_recursive_engine(
) -> Result<(), Box<dyn Error>> {
    println ! ("on document atomic_descendant.toml running the query $..* (select all decsendants) with Input impl OwnedBytes and result mode IndexResult");
    let jsonpath_query = JsonPathQuery::parse("$..*")?;
    let raw_json = fs::read_to_string("./tests/documents/json/compressed/atomic_descendant.json")?;
    let input = OwnedBytes::new(&raw_json.as_bytes())?;
    let engine = RecursiveEngine::compile_query(&jsonpath_query)?;
    let result = engine.run::<_, IndexResult>(&input)?;
    assert_eq!(
        result.get(),
        vec![5usize, 12usize, 13usize, 18usize,],
        "result != expected"
    );
    Ok(())
}
#[test]
fn compressed_with_query_select_first_item_from_list_descendants_with_buffered_input_and_count_result_using_main_engine(
) -> Result<(), Box<dyn Error>> {
    println ! ("on document atomic_descendant.toml running the query $..[0] (select first item from list descendants) with Input impl BufferedInput and result mode CountResult");
    let jsonpath_query = JsonPathQuery::parse("$..[0]")?;
    let json_file = fs::File::open("./tests/documents/json/compressed/atomic_descendant.json")?;
    let input = BufferedInput::new(json_file);
    let engine = MainEngine::compile_query(&jsonpath_query)?;
    let result = engine.run::<_, CountResult>(&input)?;
    assert_eq!(result.get(), 1u64, "result != expected");
    Ok(())
}
#[test]
fn compressed_with_query_select_first_item_from_list_descendants_with_buffered_input_and_count_result_using_recursive_engine(
) -> Result<(), Box<dyn Error>> {
    println ! ("on document atomic_descendant.toml running the query $..[0] (select first item from list descendants) with Input impl BufferedInput and result mode CountResult");
    let jsonpath_query = JsonPathQuery::parse("$..[0]")?;
    let json_file = fs::File::open("./tests/documents/json/compressed/atomic_descendant.json")?;
    let input = BufferedInput::new(json_file);
    let engine = RecursiveEngine::compile_query(&jsonpath_query)?;
    let result = engine.run::<_, CountResult>(&input)?;
    assert_eq!(result.get(), 1u64, "result != expected");
    Ok(())
}
#[test]
fn compressed_with_query_select_first_item_from_list_descendants_with_buffered_input_and_index_result_using_main_engine(
) -> Result<(), Box<dyn Error>> {
    println ! ("on document atomic_descendant.toml running the query $..[0] (select first item from list descendants) with Input impl BufferedInput and result mode IndexResult");
    let jsonpath_query = JsonPathQuery::parse("$..[0]")?;
    let json_file = fs::File::open("./tests/documents/json/compressed/atomic_descendant.json")?;
    let input = BufferedInput::new(json_file);
    let engine = MainEngine::compile_query(&jsonpath_query)?;
    let result = engine.run::<_, IndexResult>(&input)?;
    assert_eq!(result.get(), vec![13usize,], "result != expected");
    Ok(())
}
#[test]
fn compressed_with_query_select_first_item_from_list_descendants_with_buffered_input_and_index_result_using_recursive_engine(
) -> Result<(), Box<dyn Error>> {
    println ! ("on document atomic_descendant.toml running the query $..[0] (select first item from list descendants) with Input impl BufferedInput and result mode IndexResult");
    let jsonpath_query = JsonPathQuery::parse("$..[0]")?;
    let json_file = fs::File::open("./tests/documents/json/compressed/atomic_descendant.json")?;
    let input = BufferedInput::new(json_file);
    let engine = RecursiveEngine::compile_query(&jsonpath_query)?;
    let result = engine.run::<_, IndexResult>(&input)?;
    assert_eq!(result.get(), vec![13usize,], "result != expected");
    Ok(())
}
#[test]
fn compressed_with_query_select_first_item_from_list_descendants_with_mmap_input_and_count_result_using_main_engine(
) -> Result<(), Box<dyn Error>> {
    println ! ("on document atomic_descendant.toml running the query $..[0] (select first item from list descendants) with Input impl MmapInput and result mode CountResult");
    let jsonpath_query = JsonPathQuery::parse("$..[0]")?;
    let json_file = fs::File::open("./tests/documents/json/compressed/atomic_descendant.json")?;
    let input = unsafe { MmapInput::map_file(&json_file)? };
    let engine = MainEngine::compile_query(&jsonpath_query)?;
    let result = engine.run::<_, CountResult>(&input)?;
    assert_eq!(result.get(), 1u64, "result != expected");
    Ok(())
}
#[test]
fn compressed_with_query_select_first_item_from_list_descendants_with_mmap_input_and_count_result_using_recursive_engine(
) -> Result<(), Box<dyn Error>> {
    println ! ("on document atomic_descendant.toml running the query $..[0] (select first item from list descendants) with Input impl MmapInput and result mode CountResult");
    let jsonpath_query = JsonPathQuery::parse("$..[0]")?;
    let json_file = fs::File::open("./tests/documents/json/compressed/atomic_descendant.json")?;
    let input = unsafe { MmapInput::map_file(&json_file)? };
    let engine = RecursiveEngine::compile_query(&jsonpath_query)?;
    let result = engine.run::<_, CountResult>(&input)?;
    assert_eq!(result.get(), 1u64, "result != expected");
    Ok(())
}
#[test]
fn compressed_with_query_select_first_item_from_list_descendants_with_mmap_input_and_index_result_using_main_engine(
) -> Result<(), Box<dyn Error>> {
    println ! ("on document atomic_descendant.toml running the query $..[0] (select first item from list descendants) with Input impl MmapInput and result mode IndexResult");
    let jsonpath_query = JsonPathQuery::parse("$..[0]")?;
    let json_file = fs::File::open("./tests/documents/json/compressed/atomic_descendant.json")?;
    let input = unsafe { MmapInput::map_file(&json_file)? };
    let engine = MainEngine::compile_query(&jsonpath_query)?;
    let result = engine.run::<_, IndexResult>(&input)?;
    assert_eq!(result.get(), vec![13usize,], "result != expected");
    Ok(())
}
#[test]
fn compressed_with_query_select_first_item_from_list_descendants_with_mmap_input_and_index_result_using_recursive_engine(
) -> Result<(), Box<dyn Error>> {
    println ! ("on document atomic_descendant.toml running the query $..[0] (select first item from list descendants) with Input impl MmapInput and result mode IndexResult");
    let jsonpath_query = JsonPathQuery::parse("$..[0]")?;
    let json_file = fs::File::open("./tests/documents/json/compressed/atomic_descendant.json")?;
    let input = unsafe { MmapInput::map_file(&json_file)? };
    let engine = RecursiveEngine::compile_query(&jsonpath_query)?;
    let result = engine.run::<_, IndexResult>(&input)?;
    assert_eq!(result.get(), vec![13usize,], "result != expected");
    Ok(())
}
#[test]
fn compressed_with_query_select_first_item_from_list_descendants_with_owned_bytes_and_count_result_using_main_engine(
) -> Result<(), Box<dyn Error>> {
    println ! ("on document atomic_descendant.toml running the query $..[0] (select first item from list descendants) with Input impl OwnedBytes and result mode CountResult");
    let jsonpath_query = JsonPathQuery::parse("$..[0]")?;
    let raw_json = fs::read_to_string("./tests/documents/json/compressed/atomic_descendant.json")?;
    let input = OwnedBytes::new(&raw_json.as_bytes())?;
    let engine = MainEngine::compile_query(&jsonpath_query)?;
    let result = engine.run::<_, CountResult>(&input)?;
    assert_eq!(result.get(), 1u64, "result != expected");
    Ok(())
}
#[test]
fn compressed_with_query_select_first_item_from_list_descendants_with_owned_bytes_and_count_result_using_recursive_engine(
) -> Result<(), Box<dyn Error>> {
    println ! ("on document atomic_descendant.toml running the query $..[0] (select first item from list descendants) with Input impl OwnedBytes and result mode CountResult");
    let jsonpath_query = JsonPathQuery::parse("$..[0]")?;
    let raw_json = fs::read_to_string("./tests/documents/json/compressed/atomic_descendant.json")?;
    let input = OwnedBytes::new(&raw_json.as_bytes())?;
    let engine = RecursiveEngine::compile_query(&jsonpath_query)?;
    let result = engine.run::<_, CountResult>(&input)?;
    assert_eq!(result.get(), 1u64, "result != expected");
    Ok(())
}
#[test]
fn compressed_with_query_select_first_item_from_list_descendants_with_owned_bytes_and_index_result_using_main_engine(
) -> Result<(), Box<dyn Error>> {
    println ! ("on document atomic_descendant.toml running the query $..[0] (select first item from list descendants) with Input impl OwnedBytes and result mode IndexResult");
    let jsonpath_query = JsonPathQuery::parse("$..[0]")?;
    let raw_json = fs::read_to_string("./tests/documents/json/compressed/atomic_descendant.json")?;
    let input = OwnedBytes::new(&raw_json.as_bytes())?;
    let engine = MainEngine::compile_query(&jsonpath_query)?;
    let result = engine.run::<_, IndexResult>(&input)?;
    assert_eq!(result.get(), vec![13usize,], "result != expected");
    Ok(())
}
#[test]
fn compressed_with_query_select_first_item_from_list_descendants_with_owned_bytes_and_index_result_using_recursive_engine(
) -> Result<(), Box<dyn Error>> {
    println ! ("on document atomic_descendant.toml running the query $..[0] (select first item from list descendants) with Input impl OwnedBytes and result mode IndexResult");
    let jsonpath_query = JsonPathQuery::parse("$..[0]")?;
    let raw_json = fs::read_to_string("./tests/documents/json/compressed/atomic_descendant.json")?;
    let input = OwnedBytes::new(&raw_json.as_bytes())?;
    let engine = RecursiveEngine::compile_query(&jsonpath_query)?;
    let result = engine.run::<_, IndexResult>(&input)?;
    assert_eq!(result.get(), vec![13usize,], "result != expected");
    Ok(())
}
#[test]
fn empty_array_root_compressed_with_query_select_any_descendant_there_are_none_with_buffered_input_and_count_result_using_main_engine(
) -> Result<(), Box<dyn Error>> {
    println ! ("on document empty_array.toml running the query $..* (select any descendant (there are none)) with Input impl BufferedInput and result mode CountResult");
    let jsonpath_query = JsonPathQuery::parse("$..*")?;
    let json_file = fs::File::open("./tests/documents/json/compressed/empty_array.json")?;
    let input = BufferedInput::new(json_file);
    let engine = MainEngine::compile_query(&jsonpath_query)?;
    let result = engine.run::<_, CountResult>(&input)?;
    assert_eq!(result.get(), 0u64, "result != expected");
    Ok(())
}
#[test]
fn empty_array_root_compressed_with_query_select_any_descendant_there_are_none_with_buffered_input_and_count_result_using_recursive_engine(
) -> Result<(), Box<dyn Error>> {
    println ! ("on document empty_array.toml running the query $..* (select any descendant (there are none)) with Input impl BufferedInput and result mode CountResult");
    let jsonpath_query = JsonPathQuery::parse("$..*")?;
    let json_file = fs::File::open("./tests/documents/json/compressed/empty_array.json")?;
    let input = BufferedInput::new(json_file);
    let engine = RecursiveEngine::compile_query(&jsonpath_query)?;
    let result = engine.run::<_, CountResult>(&input)?;
    assert_eq!(result.get(), 0u64, "result != expected");
    Ok(())
}
#[test]
fn empty_array_root_compressed_with_query_select_any_descendant_there_are_none_with_buffered_input_and_index_result_using_main_engine(
) -> Result<(), Box<dyn Error>> {
    println ! ("on document empty_array.toml running the query $..* (select any descendant (there are none)) with Input impl BufferedInput and result mode IndexResult");
    let jsonpath_query = JsonPathQuery::parse("$..*")?;
    let json_file = fs::File::open("./tests/documents/json/compressed/empty_array.json")?;
    let input = BufferedInput::new(json_file);
    let engine = MainEngine::compile_query(&jsonpath_query)?;
    let result = engine.run::<_, IndexResult>(&input)?;
    assert_eq!(result.get(), vec![], "result != expected");
    Ok(())
}
#[test]
fn empty_array_root_compressed_with_query_select_any_descendant_there_are_none_with_buffered_input_and_index_result_using_recursive_engine(
) -> Result<(), Box<dyn Error>> {
    println ! ("on document empty_array.toml running the query $..* (select any descendant (there are none)) with Input impl BufferedInput and result mode IndexResult");
    let jsonpath_query = JsonPathQuery::parse("$..*")?;
    let json_file = fs::File::open("./tests/documents/json/compressed/empty_array.json")?;
    let input = BufferedInput::new(json_file);
    let engine = RecursiveEngine::compile_query(&jsonpath_query)?;
    let result = engine.run::<_, IndexResult>(&input)?;
    assert_eq!(result.get(), vec![], "result != expected");
    Ok(())
}
#[test]
fn empty_array_root_compressed_with_query_select_any_descendant_there_are_none_with_mmap_input_and_count_result_using_main_engine(
) -> Result<(), Box<dyn Error>> {
    println ! ("on document empty_array.toml running the query $..* (select any descendant (there are none)) with Input impl MmapInput and result mode CountResult");
    let jsonpath_query = JsonPathQuery::parse("$..*")?;
    let json_file = fs::File::open("./tests/documents/json/compressed/empty_array.json")?;
    let input = unsafe { MmapInput::map_file(&json_file)? };
    let engine = MainEngine::compile_query(&jsonpath_query)?;
    let result = engine.run::<_, CountResult>(&input)?;
    assert_eq!(result.get(), 0u64, "result != expected");
    Ok(())
}
#[test]
fn empty_array_root_compressed_with_query_select_any_descendant_there_are_none_with_mmap_input_and_count_result_using_recursive_engine(
) -> Result<(), Box<dyn Error>> {
    println ! ("on document empty_array.toml running the query $..* (select any descendant (there are none)) with Input impl MmapInput and result mode CountResult");
    let jsonpath_query = JsonPathQuery::parse("$..*")?;
    let json_file = fs::File::open("./tests/documents/json/compressed/empty_array.json")?;
    let input = unsafe { MmapInput::map_file(&json_file)? };
    let engine = RecursiveEngine::compile_query(&jsonpath_query)?;
    let result = engine.run::<_, CountResult>(&input)?;
    assert_eq!(result.get(), 0u64, "result != expected");
    Ok(())
}
#[test]
fn empty_array_root_compressed_with_query_select_any_descendant_there_are_none_with_mmap_input_and_index_result_using_main_engine(
) -> Result<(), Box<dyn Error>> {
    println ! ("on document empty_array.toml running the query $..* (select any descendant (there are none)) with Input impl MmapInput and result mode IndexResult");
    let jsonpath_query = JsonPathQuery::parse("$..*")?;
    let json_file = fs::File::open("./tests/documents/json/compressed/empty_array.json")?;
    let input = unsafe { MmapInput::map_file(&json_file)? };
    let engine = MainEngine::compile_query(&jsonpath_query)?;
    let result = engine.run::<_, IndexResult>(&input)?;
    assert_eq!(result.get(), vec![], "result != expected");
    Ok(())
}
#[test]
fn empty_array_root_compressed_with_query_select_any_descendant_there_are_none_with_mmap_input_and_index_result_using_recursive_engine(
) -> Result<(), Box<dyn Error>> {
    println ! ("on document empty_array.toml running the query $..* (select any descendant (there are none)) with Input impl MmapInput and result mode IndexResult");
    let jsonpath_query = JsonPathQuery::parse("$..*")?;
    let json_file = fs::File::open("./tests/documents/json/compressed/empty_array.json")?;
    let input = unsafe { MmapInput::map_file(&json_file)? };
    let engine = RecursiveEngine::compile_query(&jsonpath_query)?;
    let result = engine.run::<_, IndexResult>(&input)?;
    assert_eq!(result.get(), vec![], "result != expected");
    Ok(())
}
#[test]
fn empty_array_root_compressed_with_query_select_any_descendant_there_are_none_with_owned_bytes_and_count_result_using_main_engine(
) -> Result<(), Box<dyn Error>> {
    println ! ("on document empty_array.toml running the query $..* (select any descendant (there are none)) with Input impl OwnedBytes and result mode CountResult");
    let jsonpath_query = JsonPathQuery::parse("$..*")?;
    let raw_json = fs::read_to_string("./tests/documents/json/compressed/empty_array.json")?;
    let input = OwnedBytes::new(&raw_json.as_bytes())?;
    let engine = MainEngine::compile_query(&jsonpath_query)?;
    let result = engine.run::<_, CountResult>(&input)?;
    assert_eq!(result.get(), 0u64, "result != expected");
    Ok(())
}
#[test]
fn empty_array_root_compressed_with_query_select_any_descendant_there_are_none_with_owned_bytes_and_count_result_using_recursive_engine(
) -> Result<(), Box<dyn Error>> {
    println ! ("on document empty_array.toml running the query $..* (select any descendant (there are none)) with Input impl OwnedBytes and result mode CountResult");
    let jsonpath_query = JsonPathQuery::parse("$..*")?;
    let raw_json = fs::read_to_string("./tests/documents/json/compressed/empty_array.json")?;
    let input = OwnedBytes::new(&raw_json.as_bytes())?;
    let engine = RecursiveEngine::compile_query(&jsonpath_query)?;
    let result = engine.run::<_, CountResult>(&input)?;
    assert_eq!(result.get(), 0u64, "result != expected");
    Ok(())
}
#[test]
fn empty_array_root_compressed_with_query_select_any_descendant_there_are_none_with_owned_bytes_and_index_result_using_main_engine(
) -> Result<(), Box<dyn Error>> {
    println ! ("on document empty_array.toml running the query $..* (select any descendant (there are none)) with Input impl OwnedBytes and result mode IndexResult");
    let jsonpath_query = JsonPathQuery::parse("$..*")?;
    let raw_json = fs::read_to_string("./tests/documents/json/compressed/empty_array.json")?;
    let input = OwnedBytes::new(&raw_json.as_bytes())?;
    let engine = MainEngine::compile_query(&jsonpath_query)?;
    let result = engine.run::<_, IndexResult>(&input)?;
    assert_eq!(result.get(), vec![], "result != expected");
    Ok(())
}
#[test]
fn empty_array_root_compressed_with_query_select_any_descendant_there_are_none_with_owned_bytes_and_index_result_using_recursive_engine(
) -> Result<(), Box<dyn Error>> {
    println ! ("on document empty_array.toml running the query $..* (select any descendant (there are none)) with Input impl OwnedBytes and result mode IndexResult");
    let jsonpath_query = JsonPathQuery::parse("$..*")?;
    let raw_json = fs::read_to_string("./tests/documents/json/compressed/empty_array.json")?;
    let input = OwnedBytes::new(&raw_json.as_bytes())?;
    let engine = RecursiveEngine::compile_query(&jsonpath_query)?;
    let result = engine.run::<_, IndexResult>(&input)?;
    assert_eq!(result.get(), vec![], "result != expected");
    Ok(())
}
#[test]
fn empty_array_root_compressed_with_query_select_any_item_there_are_none_with_buffered_input_and_count_result_using_main_engine(
) -> Result<(), Box<dyn Error>> {
    println ! ("on document empty_array.toml running the query $[*] (select any item (there are none)) with Input impl BufferedInput and result mode CountResult");
    let jsonpath_query = JsonPathQuery::parse("$[*]")?;
    let json_file = fs::File::open("./tests/documents/json/compressed/empty_array.json")?;
    let input = BufferedInput::new(json_file);
    let engine = MainEngine::compile_query(&jsonpath_query)?;
    let result = engine.run::<_, CountResult>(&input)?;
    assert_eq!(result.get(), 0u64, "result != expected");
    Ok(())
}
#[test]
fn empty_array_root_compressed_with_query_select_any_item_there_are_none_with_buffered_input_and_count_result_using_recursive_engine(
) -> Result<(), Box<dyn Error>> {
    println ! ("on document empty_array.toml running the query $[*] (select any item (there are none)) with Input impl BufferedInput and result mode CountResult");
    let jsonpath_query = JsonPathQuery::parse("$[*]")?;
    let json_file = fs::File::open("./tests/documents/json/compressed/empty_array.json")?;
    let input = BufferedInput::new(json_file);
    let engine = RecursiveEngine::compile_query(&jsonpath_query)?;
    let result = engine.run::<_, CountResult>(&input)?;
    assert_eq!(result.get(), 0u64, "result != expected");
    Ok(())
}
#[test]
fn empty_array_root_compressed_with_query_select_any_item_there_are_none_with_buffered_input_and_index_result_using_main_engine(
) -> Result<(), Box<dyn Error>> {
    println ! ("on document empty_array.toml running the query $[*] (select any item (there are none)) with Input impl BufferedInput and result mode IndexResult");
    let jsonpath_query = JsonPathQuery::parse("$[*]")?;
    let json_file = fs::File::open("./tests/documents/json/compressed/empty_array.json")?;
    let input = BufferedInput::new(json_file);
    let engine = MainEngine::compile_query(&jsonpath_query)?;
    let result = engine.run::<_, IndexResult>(&input)?;
    assert_eq!(result.get(), vec![], "result != expected");
    Ok(())
}
#[test]
fn empty_array_root_compressed_with_query_select_any_item_there_are_none_with_buffered_input_and_index_result_using_recursive_engine(
) -> Result<(), Box<dyn Error>> {
    println ! ("on document empty_array.toml running the query $[*] (select any item (there are none)) with Input impl BufferedInput and result mode IndexResult");
    let jsonpath_query = JsonPathQuery::parse("$[*]")?;
    let json_file = fs::File::open("./tests/documents/json/compressed/empty_array.json")?;
    let input = BufferedInput::new(json_file);
    let engine = RecursiveEngine::compile_query(&jsonpath_query)?;
    let result = engine.run::<_, IndexResult>(&input)?;
    assert_eq!(result.get(), vec![], "result != expected");
    Ok(())
}
#[test]
fn empty_array_root_compressed_with_query_select_any_item_there_are_none_with_mmap_input_and_count_result_using_main_engine(
) -> Result<(), Box<dyn Error>> {
    println ! ("on document empty_array.toml running the query $[*] (select any item (there are none)) with Input impl MmapInput and result mode CountResult");
    let jsonpath_query = JsonPathQuery::parse("$[*]")?;
    let json_file = fs::File::open("./tests/documents/json/compressed/empty_array.json")?;
    let input = unsafe { MmapInput::map_file(&json_file)? };
    let engine = MainEngine::compile_query(&jsonpath_query)?;
    let result = engine.run::<_, CountResult>(&input)?;
    assert_eq!(result.get(), 0u64, "result != expected");
    Ok(())
}
#[test]
fn empty_array_root_compressed_with_query_select_any_item_there_are_none_with_mmap_input_and_count_result_using_recursive_engine(
) -> Result<(), Box<dyn Error>> {
    println ! ("on document empty_array.toml running the query $[*] (select any item (there are none)) with Input impl MmapInput and result mode CountResult");
    let jsonpath_query = JsonPathQuery::parse("$[*]")?;
    let json_file = fs::File::open("./tests/documents/json/compressed/empty_array.json")?;
    let input = unsafe { MmapInput::map_file(&json_file)? };
    let engine = RecursiveEngine::compile_query(&jsonpath_query)?;
    let result = engine.run::<_, CountResult>(&input)?;
    assert_eq!(result.get(), 0u64, "result != expected");
    Ok(())
}
#[test]
fn empty_array_root_compressed_with_query_select_any_item_there_are_none_with_mmap_input_and_index_result_using_main_engine(
) -> Result<(), Box<dyn Error>> {
    println ! ("on document empty_array.toml running the query $[*] (select any item (there are none)) with Input impl MmapInput and result mode IndexResult");
    let jsonpath_query = JsonPathQuery::parse("$[*]")?;
    let json_file = fs::File::open("./tests/documents/json/compressed/empty_array.json")?;
    let input = unsafe { MmapInput::map_file(&json_file)? };
    let engine = MainEngine::compile_query(&jsonpath_query)?;
    let result = engine.run::<_, IndexResult>(&input)?;
    assert_eq!(result.get(), vec![], "result != expected");
    Ok(())
}
#[test]
fn empty_array_root_compressed_with_query_select_any_item_there_are_none_with_mmap_input_and_index_result_using_recursive_engine(
) -> Result<(), Box<dyn Error>> {
    println ! ("on document empty_array.toml running the query $[*] (select any item (there are none)) with Input impl MmapInput and result mode IndexResult");
    let jsonpath_query = JsonPathQuery::parse("$[*]")?;
    let json_file = fs::File::open("./tests/documents/json/compressed/empty_array.json")?;
    let input = unsafe { MmapInput::map_file(&json_file)? };
    let engine = RecursiveEngine::compile_query(&jsonpath_query)?;
    let result = engine.run::<_, IndexResult>(&input)?;
    assert_eq!(result.get(), vec![], "result != expected");
    Ok(())
}
#[test]
fn empty_array_root_compressed_with_query_select_any_item_there_are_none_with_owned_bytes_and_count_result_using_main_engine(
) -> Result<(), Box<dyn Error>> {
    println ! ("on document empty_array.toml running the query $[*] (select any item (there are none)) with Input impl OwnedBytes and result mode CountResult");
    let jsonpath_query = JsonPathQuery::parse("$[*]")?;
    let raw_json = fs::read_to_string("./tests/documents/json/compressed/empty_array.json")?;
    let input = OwnedBytes::new(&raw_json.as_bytes())?;
    let engine = MainEngine::compile_query(&jsonpath_query)?;
    let result = engine.run::<_, CountResult>(&input)?;
    assert_eq!(result.get(), 0u64, "result != expected");
    Ok(())
}
#[test]
fn empty_array_root_compressed_with_query_select_any_item_there_are_none_with_owned_bytes_and_count_result_using_recursive_engine(
) -> Result<(), Box<dyn Error>> {
    println ! ("on document empty_array.toml running the query $[*] (select any item (there are none)) with Input impl OwnedBytes and result mode CountResult");
    let jsonpath_query = JsonPathQuery::parse("$[*]")?;
    let raw_json = fs::read_to_string("./tests/documents/json/compressed/empty_array.json")?;
    let input = OwnedBytes::new(&raw_json.as_bytes())?;
    let engine = RecursiveEngine::compile_query(&jsonpath_query)?;
    let result = engine.run::<_, CountResult>(&input)?;
    assert_eq!(result.get(), 0u64, "result != expected");
    Ok(())
}
#[test]
fn empty_array_root_compressed_with_query_select_any_item_there_are_none_with_owned_bytes_and_index_result_using_main_engine(
) -> Result<(), Box<dyn Error>> {
    println ! ("on document empty_array.toml running the query $[*] (select any item (there are none)) with Input impl OwnedBytes and result mode IndexResult");
    let jsonpath_query = JsonPathQuery::parse("$[*]")?;
    let raw_json = fs::read_to_string("./tests/documents/json/compressed/empty_array.json")?;
    let input = OwnedBytes::new(&raw_json.as_bytes())?;
    let engine = MainEngine::compile_query(&jsonpath_query)?;
    let result = engine.run::<_, IndexResult>(&input)?;
    assert_eq!(result.get(), vec![], "result != expected");
    Ok(())
}
#[test]
fn empty_array_root_compressed_with_query_select_any_item_there_are_none_with_owned_bytes_and_index_result_using_recursive_engine(
) -> Result<(), Box<dyn Error>> {
    println ! ("on document empty_array.toml running the query $[*] (select any item (there are none)) with Input impl OwnedBytes and result mode IndexResult");
    let jsonpath_query = JsonPathQuery::parse("$[*]")?;
    let raw_json = fs::read_to_string("./tests/documents/json/compressed/empty_array.json")?;
    let input = OwnedBytes::new(&raw_json.as_bytes())?;
    let engine = RecursiveEngine::compile_query(&jsonpath_query)?;
    let result = engine.run::<_, IndexResult>(&input)?;
    assert_eq!(result.get(), vec![], "result != expected");
    Ok(())
}
#[test]
fn empty_array_root_compressed_with_query_select_the_first_item_which_does_not_exist_with_buffered_input_and_count_result_using_main_engine(
) -> Result<(), Box<dyn Error>> {
    println ! ("on document empty_array.toml running the query $[0] (select the first item (which does not exist)) with Input impl BufferedInput and result mode CountResult");
    let jsonpath_query = JsonPathQuery::parse("$[0]")?;
    let json_file = fs::File::open("./tests/documents/json/compressed/empty_array.json")?;
    let input = BufferedInput::new(json_file);
    let engine = MainEngine::compile_query(&jsonpath_query)?;
    let result = engine.run::<_, CountResult>(&input)?;
    assert_eq!(result.get(), 0u64, "result != expected");
    Ok(())
}
#[test]
fn empty_array_root_compressed_with_query_select_the_first_item_which_does_not_exist_with_buffered_input_and_count_result_using_recursive_engine(
) -> Result<(), Box<dyn Error>> {
    println ! ("on document empty_array.toml running the query $[0] (select the first item (which does not exist)) with Input impl BufferedInput and result mode CountResult");
    let jsonpath_query = JsonPathQuery::parse("$[0]")?;
    let json_file = fs::File::open("./tests/documents/json/compressed/empty_array.json")?;
    let input = BufferedInput::new(json_file);
    let engine = RecursiveEngine::compile_query(&jsonpath_query)?;
    let result = engine.run::<_, CountResult>(&input)?;
    assert_eq!(result.get(), 0u64, "result != expected");
    Ok(())
}
#[test]
fn empty_array_root_compressed_with_query_select_the_first_item_which_does_not_exist_with_buffered_input_and_index_result_using_main_engine(
) -> Result<(), Box<dyn Error>> {
    println ! ("on document empty_array.toml running the query $[0] (select the first item (which does not exist)) with Input impl BufferedInput and result mode IndexResult");
    let jsonpath_query = JsonPathQuery::parse("$[0]")?;
    let json_file = fs::File::open("./tests/documents/json/compressed/empty_array.json")?;
    let input = BufferedInput::new(json_file);
    let engine = MainEngine::compile_query(&jsonpath_query)?;
    let result = engine.run::<_, IndexResult>(&input)?;
    assert_eq!(result.get(), vec![], "result != expected");
    Ok(())
}
#[test]
fn empty_array_root_compressed_with_query_select_the_first_item_which_does_not_exist_with_buffered_input_and_index_result_using_recursive_engine(
) -> Result<(), Box<dyn Error>> {
    println ! ("on document empty_array.toml running the query $[0] (select the first item (which does not exist)) with Input impl BufferedInput and result mode IndexResult");
    let jsonpath_query = JsonPathQuery::parse("$[0]")?;
    let json_file = fs::File::open("./tests/documents/json/compressed/empty_array.json")?;
    let input = BufferedInput::new(json_file);
    let engine = RecursiveEngine::compile_query(&jsonpath_query)?;
    let result = engine.run::<_, IndexResult>(&input)?;
    assert_eq!(result.get(), vec![], "result != expected");
    Ok(())
}
#[test]
fn empty_array_root_compressed_with_query_select_the_first_item_which_does_not_exist_with_mmap_input_and_count_result_using_main_engine(
) -> Result<(), Box<dyn Error>> {
    println ! ("on document empty_array.toml running the query $[0] (select the first item (which does not exist)) with Input impl MmapInput and result mode CountResult");
    let jsonpath_query = JsonPathQuery::parse("$[0]")?;
    let json_file = fs::File::open("./tests/documents/json/compressed/empty_array.json")?;
    let input = unsafe { MmapInput::map_file(&json_file)? };
    let engine = MainEngine::compile_query(&jsonpath_query)?;
    let result = engine.run::<_, CountResult>(&input)?;
    assert_eq!(result.get(), 0u64, "result != expected");
    Ok(())
}
#[test]
fn empty_array_root_compressed_with_query_select_the_first_item_which_does_not_exist_with_mmap_input_and_count_result_using_recursive_engine(
) -> Result<(), Box<dyn Error>> {
    println ! ("on document empty_array.toml running the query $[0] (select the first item (which does not exist)) with Input impl MmapInput and result mode CountResult");
    let jsonpath_query = JsonPathQuery::parse("$[0]")?;
    let json_file = fs::File::open("./tests/documents/json/compressed/empty_array.json")?;
    let input = unsafe { MmapInput::map_file(&json_file)? };
    let engine = RecursiveEngine::compile_query(&jsonpath_query)?;
    let result = engine.run::<_, CountResult>(&input)?;
    assert_eq!(result.get(), 0u64, "result != expected");
    Ok(())
}
#[test]
fn empty_array_root_compressed_with_query_select_the_first_item_which_does_not_exist_with_mmap_input_and_index_result_using_main_engine(
) -> Result<(), Box<dyn Error>> {
    println ! ("on document empty_array.toml running the query $[0] (select the first item (which does not exist)) with Input impl MmapInput and result mode IndexResult");
    let jsonpath_query = JsonPathQuery::parse("$[0]")?;
    let json_file = fs::File::open("./tests/documents/json/compressed/empty_array.json")?;
    let input = unsafe { MmapInput::map_file(&json_file)? };
    let engine = MainEngine::compile_query(&jsonpath_query)?;
    let result = engine.run::<_, IndexResult>(&input)?;
    assert_eq!(result.get(), vec![], "result != expected");
    Ok(())
}
#[test]
fn empty_array_root_compressed_with_query_select_the_first_item_which_does_not_exist_with_mmap_input_and_index_result_using_recursive_engine(
) -> Result<(), Box<dyn Error>> {
    println ! ("on document empty_array.toml running the query $[0] (select the first item (which does not exist)) with Input impl MmapInput and result mode IndexResult");
    let jsonpath_query = JsonPathQuery::parse("$[0]")?;
    let json_file = fs::File::open("./tests/documents/json/compressed/empty_array.json")?;
    let input = unsafe { MmapInput::map_file(&json_file)? };
    let engine = RecursiveEngine::compile_query(&jsonpath_query)?;
    let result = engine.run::<_, IndexResult>(&input)?;
    assert_eq!(result.get(), vec![], "result != expected");
    Ok(())
}
#[test]
fn empty_array_root_compressed_with_query_select_the_first_item_which_does_not_exist_with_owned_bytes_and_count_result_using_main_engine(
) -> Result<(), Box<dyn Error>> {
    println ! ("on document empty_array.toml running the query $[0] (select the first item (which does not exist)) with Input impl OwnedBytes and result mode CountResult");
    let jsonpath_query = JsonPathQuery::parse("$[0]")?;
    let raw_json = fs::read_to_string("./tests/documents/json/compressed/empty_array.json")?;
    let input = OwnedBytes::new(&raw_json.as_bytes())?;
    let engine = MainEngine::compile_query(&jsonpath_query)?;
    let result = engine.run::<_, CountResult>(&input)?;
    assert_eq!(result.get(), 0u64, "result != expected");
    Ok(())
}
#[test]
fn empty_array_root_compressed_with_query_select_the_first_item_which_does_not_exist_with_owned_bytes_and_count_result_using_recursive_engine(
) -> Result<(), Box<dyn Error>> {
    println ! ("on document empty_array.toml running the query $[0] (select the first item (which does not exist)) with Input impl OwnedBytes and result mode CountResult");
    let jsonpath_query = JsonPathQuery::parse("$[0]")?;
    let raw_json = fs::read_to_string("./tests/documents/json/compressed/empty_array.json")?;
    let input = OwnedBytes::new(&raw_json.as_bytes())?;
    let engine = RecursiveEngine::compile_query(&jsonpath_query)?;
    let result = engine.run::<_, CountResult>(&input)?;
    assert_eq!(result.get(), 0u64, "result != expected");
    Ok(())
}
#[test]
fn empty_array_root_compressed_with_query_select_the_first_item_which_does_not_exist_with_owned_bytes_and_index_result_using_main_engine(
) -> Result<(), Box<dyn Error>> {
    println ! ("on document empty_array.toml running the query $[0] (select the first item (which does not exist)) with Input impl OwnedBytes and result mode IndexResult");
    let jsonpath_query = JsonPathQuery::parse("$[0]")?;
    let raw_json = fs::read_to_string("./tests/documents/json/compressed/empty_array.json")?;
    let input = OwnedBytes::new(&raw_json.as_bytes())?;
    let engine = MainEngine::compile_query(&jsonpath_query)?;
    let result = engine.run::<_, IndexResult>(&input)?;
    assert_eq!(result.get(), vec![], "result != expected");
    Ok(())
}
#[test]
fn empty_array_root_compressed_with_query_select_the_first_item_which_does_not_exist_with_owned_bytes_and_index_result_using_recursive_engine(
) -> Result<(), Box<dyn Error>> {
    println ! ("on document empty_array.toml running the query $[0] (select the first item (which does not exist)) with Input impl OwnedBytes and result mode IndexResult");
    let jsonpath_query = JsonPathQuery::parse("$[0]")?;
    let raw_json = fs::read_to_string("./tests/documents/json/compressed/empty_array.json")?;
    let input = OwnedBytes::new(&raw_json.as_bytes())?;
    let engine = RecursiveEngine::compile_query(&jsonpath_query)?;
    let result = engine.run::<_, IndexResult>(&input)?;
    assert_eq!(result.get(), vec![], "result != expected");
    Ok(())
}
#[test]
fn empty_array_root_compressed_with_query_select_the_root_empty_query_with_buffered_input_and_count_result_using_main_engine(
) -> Result<(), Box<dyn Error>> {
    println ! ("on document empty_array.toml running the query  (select the root (empty query)) with Input impl BufferedInput and result mode CountResult");
    let jsonpath_query = JsonPathQuery::parse("")?;
    let json_file = fs::File::open("./tests/documents/json/compressed/empty_array.json")?;
    let input = BufferedInput::new(json_file);
    let engine = MainEngine::compile_query(&jsonpath_query)?;
    let result = engine.run::<_, CountResult>(&input)?;
    assert_eq!(result.get(), 1u64, "result != expected");
    Ok(())
}
#[test]
fn empty_array_root_compressed_with_query_select_the_root_empty_query_with_buffered_input_and_count_result_using_recursive_engine(
) -> Result<(), Box<dyn Error>> {
    println ! ("on document empty_array.toml running the query  (select the root (empty query)) with Input impl BufferedInput and result mode CountResult");
    let jsonpath_query = JsonPathQuery::parse("")?;
    let json_file = fs::File::open("./tests/documents/json/compressed/empty_array.json")?;
    let input = BufferedInput::new(json_file);
    let engine = RecursiveEngine::compile_query(&jsonpath_query)?;
    let result = engine.run::<_, CountResult>(&input)?;
    assert_eq!(result.get(), 1u64, "result != expected");
    Ok(())
}
#[test]
fn empty_array_root_compressed_with_query_select_the_root_empty_query_with_buffered_input_and_index_result_using_main_engine(
) -> Result<(), Box<dyn Error>> {
    println ! ("on document empty_array.toml running the query  (select the root (empty query)) with Input impl BufferedInput and result mode IndexResult");
    let jsonpath_query = JsonPathQuery::parse("")?;
    let json_file = fs::File::open("./tests/documents/json/compressed/empty_array.json")?;
    let input = BufferedInput::new(json_file);
    let engine = MainEngine::compile_query(&jsonpath_query)?;
    let result = engine.run::<_, IndexResult>(&input)?;
    assert_eq!(result.get(), vec![0usize,], "result != expected");
    Ok(())
}
#[test]
fn empty_array_root_compressed_with_query_select_the_root_empty_query_with_buffered_input_and_index_result_using_recursive_engine(
) -> Result<(), Box<dyn Error>> {
    println ! ("on document empty_array.toml running the query  (select the root (empty query)) with Input impl BufferedInput and result mode IndexResult");
    let jsonpath_query = JsonPathQuery::parse("")?;
    let json_file = fs::File::open("./tests/documents/json/compressed/empty_array.json")?;
    let input = BufferedInput::new(json_file);
    let engine = RecursiveEngine::compile_query(&jsonpath_query)?;
    let result = engine.run::<_, IndexResult>(&input)?;
    assert_eq!(result.get(), vec![0usize,], "result != expected");
    Ok(())
}
#[test]
fn empty_array_root_compressed_with_query_select_the_root_empty_query_with_mmap_input_and_count_result_using_main_engine(
) -> Result<(), Box<dyn Error>> {
    println ! ("on document empty_array.toml running the query  (select the root (empty query)) with Input impl MmapInput and result mode CountResult");
    let jsonpath_query = JsonPathQuery::parse("")?;
    let json_file = fs::File::open("./tests/documents/json/compressed/empty_array.json")?;
    let input = unsafe { MmapInput::map_file(&json_file)? };
    let engine = MainEngine::compile_query(&jsonpath_query)?;
    let result = engine.run::<_, CountResult>(&input)?;
    assert_eq!(result.get(), 1u64, "result != expected");
    Ok(())
}
#[test]
fn empty_array_root_compressed_with_query_select_the_root_empty_query_with_mmap_input_and_count_result_using_recursive_engine(
) -> Result<(), Box<dyn Error>> {
    println ! ("on document empty_array.toml running the query  (select the root (empty query)) with Input impl MmapInput and result mode CountResult");
    let jsonpath_query = JsonPathQuery::parse("")?;
    let json_file = fs::File::open("./tests/documents/json/compressed/empty_array.json")?;
    let input = unsafe { MmapInput::map_file(&json_file)? };
    let engine = RecursiveEngine::compile_query(&jsonpath_query)?;
    let result = engine.run::<_, CountResult>(&input)?;
    assert_eq!(result.get(), 1u64, "result != expected");
    Ok(())
}
#[test]
fn empty_array_root_compressed_with_query_select_the_root_empty_query_with_mmap_input_and_index_result_using_main_engine(
) -> Result<(), Box<dyn Error>> {
    println ! ("on document empty_array.toml running the query  (select the root (empty query)) with Input impl MmapInput and result mode IndexResult");
    let jsonpath_query = JsonPathQuery::parse("")?;
    let json_file = fs::File::open("./tests/documents/json/compressed/empty_array.json")?;
    let input = unsafe { MmapInput::map_file(&json_file)? };
    let engine = MainEngine::compile_query(&jsonpath_query)?;
    let result = engine.run::<_, IndexResult>(&input)?;
    assert_eq!(result.get(), vec![0usize,], "result != expected");
    Ok(())
}
#[test]
fn empty_array_root_compressed_with_query_select_the_root_empty_query_with_mmap_input_and_index_result_using_recursive_engine(
) -> Result<(), Box<dyn Error>> {
    println ! ("on document empty_array.toml running the query  (select the root (empty query)) with Input impl MmapInput and result mode IndexResult");
    let jsonpath_query = JsonPathQuery::parse("")?;
    let json_file = fs::File::open("./tests/documents/json/compressed/empty_array.json")?;
    let input = unsafe { MmapInput::map_file(&json_file)? };
    let engine = RecursiveEngine::compile_query(&jsonpath_query)?;
    let result = engine.run::<_, IndexResult>(&input)?;
    assert_eq!(result.get(), vec![0usize,], "result != expected");
    Ok(())
}
#[test]
fn empty_array_root_compressed_with_query_select_the_root_empty_query_with_owned_bytes_and_count_result_using_main_engine(
) -> Result<(), Box<dyn Error>> {
    println ! ("on document empty_array.toml running the query  (select the root (empty query)) with Input impl OwnedBytes and result mode CountResult");
    let jsonpath_query = JsonPathQuery::parse("")?;
    let raw_json = fs::read_to_string("./tests/documents/json/compressed/empty_array.json")?;
    let input = OwnedBytes::new(&raw_json.as_bytes())?;
    let engine = MainEngine::compile_query(&jsonpath_query)?;
    let result = engine.run::<_, CountResult>(&input)?;
    assert_eq!(result.get(), 1u64, "result != expected");
    Ok(())
}
#[test]
fn empty_array_root_compressed_with_query_select_the_root_empty_query_with_owned_bytes_and_count_result_using_recursive_engine(
) -> Result<(), Box<dyn Error>> {
    println ! ("on document empty_array.toml running the query  (select the root (empty query)) with Input impl OwnedBytes and result mode CountResult");
    let jsonpath_query = JsonPathQuery::parse("")?;
    let raw_json = fs::read_to_string("./tests/documents/json/compressed/empty_array.json")?;
    let input = OwnedBytes::new(&raw_json.as_bytes())?;
    let engine = RecursiveEngine::compile_query(&jsonpath_query)?;
    let result = engine.run::<_, CountResult>(&input)?;
    assert_eq!(result.get(), 1u64, "result != expected");
    Ok(())
}
#[test]
fn empty_array_root_compressed_with_query_select_the_root_empty_query_with_owned_bytes_and_index_result_using_main_engine(
) -> Result<(), Box<dyn Error>> {
    println ! ("on document empty_array.toml running the query  (select the root (empty query)) with Input impl OwnedBytes and result mode IndexResult");
    let jsonpath_query = JsonPathQuery::parse("")?;
    let raw_json = fs::read_to_string("./tests/documents/json/compressed/empty_array.json")?;
    let input = OwnedBytes::new(&raw_json.as_bytes())?;
    let engine = MainEngine::compile_query(&jsonpath_query)?;
    let result = engine.run::<_, IndexResult>(&input)?;
    assert_eq!(result.get(), vec![0usize,], "result != expected");
    Ok(())
}
#[test]
fn empty_array_root_compressed_with_query_select_the_root_empty_query_with_owned_bytes_and_index_result_using_recursive_engine(
) -> Result<(), Box<dyn Error>> {
    println ! ("on document empty_array.toml running the query  (select the root (empty query)) with Input impl OwnedBytes and result mode IndexResult");
    let jsonpath_query = JsonPathQuery::parse("")?;
    let raw_json = fs::read_to_string("./tests/documents/json/compressed/empty_array.json")?;
    let input = OwnedBytes::new(&raw_json.as_bytes())?;
    let engine = RecursiveEngine::compile_query(&jsonpath_query)?;
    let result = engine.run::<_, IndexResult>(&input)?;
    assert_eq!(result.get(), vec![0usize,], "result != expected");
    Ok(())
}
#[test]
fn empty_array_root_compressed_with_query_select_the_root_with_buffered_input_and_count_result_using_main_engine(
) -> Result<(), Box<dyn Error>> {
    println ! ("on document empty_array.toml running the query $ (select the root) with Input impl BufferedInput and result mode CountResult");
    let jsonpath_query = JsonPathQuery::parse("$")?;
    let json_file = fs::File::open("./tests/documents/json/compressed/empty_array.json")?;
    let input = BufferedInput::new(json_file);
    let engine = MainEngine::compile_query(&jsonpath_query)?;
    let result = engine.run::<_, CountResult>(&input)?;
    assert_eq!(result.get(), 1u64, "result != expected");
    Ok(())
}
#[test]
fn empty_array_root_compressed_with_query_select_the_root_with_buffered_input_and_count_result_using_recursive_engine(
) -> Result<(), Box<dyn Error>> {
    println ! ("on document empty_array.toml running the query $ (select the root) with Input impl BufferedInput and result mode CountResult");
    let jsonpath_query = JsonPathQuery::parse("$")?;
    let json_file = fs::File::open("./tests/documents/json/compressed/empty_array.json")?;
    let input = BufferedInput::new(json_file);
    let engine = RecursiveEngine::compile_query(&jsonpath_query)?;
    let result = engine.run::<_, CountResult>(&input)?;
    assert_eq!(result.get(), 1u64, "result != expected");
    Ok(())
}
#[test]
fn empty_array_root_compressed_with_query_select_the_root_with_buffered_input_and_index_result_using_main_engine(
) -> Result<(), Box<dyn Error>> {
    println ! ("on document empty_array.toml running the query $ (select the root) with Input impl BufferedInput and result mode IndexResult");
    let jsonpath_query = JsonPathQuery::parse("$")?;
    let json_file = fs::File::open("./tests/documents/json/compressed/empty_array.json")?;
    let input = BufferedInput::new(json_file);
    let engine = MainEngine::compile_query(&jsonpath_query)?;
    let result = engine.run::<_, IndexResult>(&input)?;
    assert_eq!(result.get(), vec![0usize,], "result != expected");
    Ok(())
}
#[test]
fn empty_array_root_compressed_with_query_select_the_root_with_buffered_input_and_index_result_using_recursive_engine(
) -> Result<(), Box<dyn Error>> {
    println ! ("on document empty_array.toml running the query $ (select the root) with Input impl BufferedInput and result mode IndexResult");
    let jsonpath_query = JsonPathQuery::parse("$")?;
    let json_file = fs::File::open("./tests/documents/json/compressed/empty_array.json")?;
    let input = BufferedInput::new(json_file);
    let engine = RecursiveEngine::compile_query(&jsonpath_query)?;
    let result = engine.run::<_, IndexResult>(&input)?;
    assert_eq!(result.get(), vec![0usize,], "result != expected");
    Ok(())
}
#[test]
fn empty_array_root_compressed_with_query_select_the_root_with_mmap_input_and_count_result_using_main_engine(
) -> Result<(), Box<dyn Error>> {
    println ! ("on document empty_array.toml running the query $ (select the root) with Input impl MmapInput and result mode CountResult");
    let jsonpath_query = JsonPathQuery::parse("$")?;
    let json_file = fs::File::open("./tests/documents/json/compressed/empty_array.json")?;
    let input = unsafe { MmapInput::map_file(&json_file)? };
    let engine = MainEngine::compile_query(&jsonpath_query)?;
    let result = engine.run::<_, CountResult>(&input)?;
    assert_eq!(result.get(), 1u64, "result != expected");
    Ok(())
}
#[test]
fn empty_array_root_compressed_with_query_select_the_root_with_mmap_input_and_count_result_using_recursive_engine(
) -> Result<(), Box<dyn Error>> {
    println ! ("on document empty_array.toml running the query $ (select the root) with Input impl MmapInput and result mode CountResult");
    let jsonpath_query = JsonPathQuery::parse("$")?;
    let json_file = fs::File::open("./tests/documents/json/compressed/empty_array.json")?;
    let input = unsafe { MmapInput::map_file(&json_file)? };
    let engine = RecursiveEngine::compile_query(&jsonpath_query)?;
    let result = engine.run::<_, CountResult>(&input)?;
    assert_eq!(result.get(), 1u64, "result != expected");
    Ok(())
}
#[test]
fn empty_array_root_compressed_with_query_select_the_root_with_mmap_input_and_index_result_using_main_engine(
) -> Result<(), Box<dyn Error>> {
    println ! ("on document empty_array.toml running the query $ (select the root) with Input impl MmapInput and result mode IndexResult");
    let jsonpath_query = JsonPathQuery::parse("$")?;
    let json_file = fs::File::open("./tests/documents/json/compressed/empty_array.json")?;
    let input = unsafe { MmapInput::map_file(&json_file)? };
    let engine = MainEngine::compile_query(&jsonpath_query)?;
    let result = engine.run::<_, IndexResult>(&input)?;
    assert_eq!(result.get(), vec![0usize,], "result != expected");
    Ok(())
}
#[test]
fn empty_array_root_compressed_with_query_select_the_root_with_mmap_input_and_index_result_using_recursive_engine(
) -> Result<(), Box<dyn Error>> {
    println ! ("on document empty_array.toml running the query $ (select the root) with Input impl MmapInput and result mode IndexResult");
    let jsonpath_query = JsonPathQuery::parse("$")?;
    let json_file = fs::File::open("./tests/documents/json/compressed/empty_array.json")?;
    let input = unsafe { MmapInput::map_file(&json_file)? };
    let engine = RecursiveEngine::compile_query(&jsonpath_query)?;
    let result = engine.run::<_, IndexResult>(&input)?;
    assert_eq!(result.get(), vec![0usize,], "result != expected");
    Ok(())
}
#[test]
fn empty_array_root_compressed_with_query_select_the_root_with_owned_bytes_and_count_result_using_main_engine(
) -> Result<(), Box<dyn Error>> {
    println ! ("on document empty_array.toml running the query $ (select the root) with Input impl OwnedBytes and result mode CountResult");
    let jsonpath_query = JsonPathQuery::parse("$")?;
    let raw_json = fs::read_to_string("./tests/documents/json/compressed/empty_array.json")?;
    let input = OwnedBytes::new(&raw_json.as_bytes())?;
    let engine = MainEngine::compile_query(&jsonpath_query)?;
    let result = engine.run::<_, CountResult>(&input)?;
    assert_eq!(result.get(), 1u64, "result != expected");
    Ok(())
}
#[test]
fn empty_array_root_compressed_with_query_select_the_root_with_owned_bytes_and_count_result_using_recursive_engine(
) -> Result<(), Box<dyn Error>> {
    println ! ("on document empty_array.toml running the query $ (select the root) with Input impl OwnedBytes and result mode CountResult");
    let jsonpath_query = JsonPathQuery::parse("$")?;
    let raw_json = fs::read_to_string("./tests/documents/json/compressed/empty_array.json")?;
    let input = OwnedBytes::new(&raw_json.as_bytes())?;
    let engine = RecursiveEngine::compile_query(&jsonpath_query)?;
    let result = engine.run::<_, CountResult>(&input)?;
    assert_eq!(result.get(), 1u64, "result != expected");
    Ok(())
}
#[test]
fn empty_array_root_compressed_with_query_select_the_root_with_owned_bytes_and_index_result_using_main_engine(
) -> Result<(), Box<dyn Error>> {
    println ! ("on document empty_array.toml running the query $ (select the root) with Input impl OwnedBytes and result mode IndexResult");
    let jsonpath_query = JsonPathQuery::parse("$")?;
    let raw_json = fs::read_to_string("./tests/documents/json/compressed/empty_array.json")?;
    let input = OwnedBytes::new(&raw_json.as_bytes())?;
    let engine = MainEngine::compile_query(&jsonpath_query)?;
    let result = engine.run::<_, IndexResult>(&input)?;
    assert_eq!(result.get(), vec![0usize,], "result != expected");
    Ok(())
}
#[test]
fn empty_array_root_compressed_with_query_select_the_root_with_owned_bytes_and_index_result_using_recursive_engine(
) -> Result<(), Box<dyn Error>> {
    println ! ("on document empty_array.toml running the query $ (select the root) with Input impl OwnedBytes and result mode IndexResult");
    let jsonpath_query = JsonPathQuery::parse("$")?;
    let raw_json = fs::read_to_string("./tests/documents/json/compressed/empty_array.json")?;
    let input = OwnedBytes::new(&raw_json.as_bytes())?;
    let engine = RecursiveEngine::compile_query(&jsonpath_query)?;
    let result = engine.run::<_, IndexResult>(&input)?;
    assert_eq!(result.get(), vec![0usize,], "result != expected");
    Ok(())
}
#[test]
fn empty_array_root_with_query_select_any_descendant_there_are_none_with_buffered_input_and_count_result_using_main_engine(
) -> Result<(), Box<dyn Error>> {
    println ! ("on document empty_array.toml running the query $..* (select any descendant (there are none)) with Input impl BufferedInput and result mode CountResult");
    let jsonpath_query = JsonPathQuery::parse("$..*")?;
    let json_file = fs::File::open("./tests/documents/json/empty_array.json")?;
    let input = BufferedInput::new(json_file);
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
    let json_file = fs::File::open("./tests/documents/json/empty_array.json")?;
    let input = BufferedInput::new(json_file);
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
    let json_file = fs::File::open("./tests/documents/json/empty_array.json")?;
    let input = BufferedInput::new(json_file);
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
    let json_file = fs::File::open("./tests/documents/json/empty_array.json")?;
    let input = BufferedInput::new(json_file);
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
    let json_file = fs::File::open("./tests/documents/json/empty_array.json")?;
    let input = unsafe { MmapInput::map_file(&json_file)? };
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
    let json_file = fs::File::open("./tests/documents/json/empty_array.json")?;
    let input = unsafe { MmapInput::map_file(&json_file)? };
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
    let json_file = fs::File::open("./tests/documents/json/empty_array.json")?;
    let input = unsafe { MmapInput::map_file(&json_file)? };
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
    let json_file = fs::File::open("./tests/documents/json/empty_array.json")?;
    let input = unsafe { MmapInput::map_file(&json_file)? };
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
    let raw_json = fs::read_to_string("./tests/documents/json/empty_array.json")?;
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
    let raw_json = fs::read_to_string("./tests/documents/json/empty_array.json")?;
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
    let raw_json = fs::read_to_string("./tests/documents/json/empty_array.json")?;
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
    let raw_json = fs::read_to_string("./tests/documents/json/empty_array.json")?;
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
    let json_file = fs::File::open("./tests/documents/json/empty_array.json")?;
    let input = BufferedInput::new(json_file);
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
    let json_file = fs::File::open("./tests/documents/json/empty_array.json")?;
    let input = BufferedInput::new(json_file);
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
    let json_file = fs::File::open("./tests/documents/json/empty_array.json")?;
    let input = BufferedInput::new(json_file);
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
    let json_file = fs::File::open("./tests/documents/json/empty_array.json")?;
    let input = BufferedInput::new(json_file);
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
    let json_file = fs::File::open("./tests/documents/json/empty_array.json")?;
    let input = unsafe { MmapInput::map_file(&json_file)? };
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
    let json_file = fs::File::open("./tests/documents/json/empty_array.json")?;
    let input = unsafe { MmapInput::map_file(&json_file)? };
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
    let json_file = fs::File::open("./tests/documents/json/empty_array.json")?;
    let input = unsafe { MmapInput::map_file(&json_file)? };
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
    let json_file = fs::File::open("./tests/documents/json/empty_array.json")?;
    let input = unsafe { MmapInput::map_file(&json_file)? };
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
    let raw_json = fs::read_to_string("./tests/documents/json/empty_array.json")?;
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
    let raw_json = fs::read_to_string("./tests/documents/json/empty_array.json")?;
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
    let raw_json = fs::read_to_string("./tests/documents/json/empty_array.json")?;
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
    let raw_json = fs::read_to_string("./tests/documents/json/empty_array.json")?;
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
    let json_file = fs::File::open("./tests/documents/json/empty_array.json")?;
    let input = BufferedInput::new(json_file);
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
    let json_file = fs::File::open("./tests/documents/json/empty_array.json")?;
    let input = BufferedInput::new(json_file);
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
    let json_file = fs::File::open("./tests/documents/json/empty_array.json")?;
    let input = BufferedInput::new(json_file);
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
    let json_file = fs::File::open("./tests/documents/json/empty_array.json")?;
    let input = BufferedInput::new(json_file);
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
    let json_file = fs::File::open("./tests/documents/json/empty_array.json")?;
    let input = unsafe { MmapInput::map_file(&json_file)? };
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
    let json_file = fs::File::open("./tests/documents/json/empty_array.json")?;
    let input = unsafe { MmapInput::map_file(&json_file)? };
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
    let json_file = fs::File::open("./tests/documents/json/empty_array.json")?;
    let input = unsafe { MmapInput::map_file(&json_file)? };
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
    let json_file = fs::File::open("./tests/documents/json/empty_array.json")?;
    let input = unsafe { MmapInput::map_file(&json_file)? };
    let engine = RecursiveEngine::compile_query(&jsonpath_query)?;
    let result = engine.run::<_, IndexResult>(&input)?;
    assert_eq!(result.get(), vec![], "result != expected");
    Ok(())
}
#[test]
fn empty_array_root_with_query_select_the_first_item_which_does_not_exist_with_owned_bytes_and_count_result_using_main_engine(
) -> Result<(), Box<dyn Error>> {
    println ! ("on document empty_array.toml running the query $[0] (select the first item (which does not exist)) with Input impl OwnedBytes and result mode CountResult");
    let jsonpath_query = JsonPathQuery::parse("$[0]")?;
    let raw_json = fs::read_to_string("./tests/documents/json/empty_array.json")?;
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
    let raw_json = fs::read_to_string("./tests/documents/json/empty_array.json")?;
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
    let raw_json = fs::read_to_string("./tests/documents/json/empty_array.json")?;
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
    let raw_json = fs::read_to_string("./tests/documents/json/empty_array.json")?;
    let input = OwnedBytes::new(&raw_json.as_bytes())?;
    let engine = RecursiveEngine::compile_query(&jsonpath_query)?;
    let result = engine.run::<_, IndexResult>(&input)?;
    assert_eq!(result.get(), vec![], "result != expected");
    Ok(())
}
#[test]
fn empty_array_root_with_query_select_the_root_empty_query_with_buffered_input_and_count_result_using_main_engine(
) -> Result<(), Box<dyn Error>> {
    println ! ("on document empty_array.toml running the query  (select the root (empty query)) with Input impl BufferedInput and result mode CountResult");
    let jsonpath_query = JsonPathQuery::parse("")?;
    let json_file = fs::File::open("./tests/documents/json/empty_array.json")?;
    let input = BufferedInput::new(json_file);
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
    let json_file = fs::File::open("./tests/documents/json/empty_array.json")?;
    let input = BufferedInput::new(json_file);
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
    let json_file = fs::File::open("./tests/documents/json/empty_array.json")?;
    let input = BufferedInput::new(json_file);
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
    let json_file = fs::File::open("./tests/documents/json/empty_array.json")?;
    let input = BufferedInput::new(json_file);
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
    let json_file = fs::File::open("./tests/documents/json/empty_array.json")?;
    let input = unsafe { MmapInput::map_file(&json_file)? };
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
    let json_file = fs::File::open("./tests/documents/json/empty_array.json")?;
    let input = unsafe { MmapInput::map_file(&json_file)? };
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
    let json_file = fs::File::open("./tests/documents/json/empty_array.json")?;
    let input = unsafe { MmapInput::map_file(&json_file)? };
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
    let json_file = fs::File::open("./tests/documents/json/empty_array.json")?;
    let input = unsafe { MmapInput::map_file(&json_file)? };
    let engine = RecursiveEngine::compile_query(&jsonpath_query)?;
    let result = engine.run::<_, IndexResult>(&input)?;
    assert_eq!(result.get(), vec![0usize,], "result != expected");
    Ok(())
}
#[test]
fn empty_array_root_with_query_select_the_root_empty_query_with_owned_bytes_and_count_result_using_main_engine(
) -> Result<(), Box<dyn Error>> {
    println ! ("on document empty_array.toml running the query  (select the root (empty query)) with Input impl OwnedBytes and result mode CountResult");
    let jsonpath_query = JsonPathQuery::parse("")?;
    let raw_json = fs::read_to_string("./tests/documents/json/empty_array.json")?;
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
    let raw_json = fs::read_to_string("./tests/documents/json/empty_array.json")?;
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
    let raw_json = fs::read_to_string("./tests/documents/json/empty_array.json")?;
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
    let raw_json = fs::read_to_string("./tests/documents/json/empty_array.json")?;
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
    let json_file = fs::File::open("./tests/documents/json/empty_array.json")?;
    let input = BufferedInput::new(json_file);
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
    let json_file = fs::File::open("./tests/documents/json/empty_array.json")?;
    let input = BufferedInput::new(json_file);
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
    let json_file = fs::File::open("./tests/documents/json/empty_array.json")?;
    let input = BufferedInput::new(json_file);
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
    let json_file = fs::File::open("./tests/documents/json/empty_array.json")?;
    let input = BufferedInput::new(json_file);
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
    let json_file = fs::File::open("./tests/documents/json/empty_array.json")?;
    let input = unsafe { MmapInput::map_file(&json_file)? };
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
    let json_file = fs::File::open("./tests/documents/json/empty_array.json")?;
    let input = unsafe { MmapInput::map_file(&json_file)? };
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
    let json_file = fs::File::open("./tests/documents/json/empty_array.json")?;
    let input = unsafe { MmapInput::map_file(&json_file)? };
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
    let json_file = fs::File::open("./tests/documents/json/empty_array.json")?;
    let input = unsafe { MmapInput::map_file(&json_file)? };
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
    let raw_json = fs::read_to_string("./tests/documents/json/empty_array.json")?;
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
    let raw_json = fs::read_to_string("./tests/documents/json/empty_array.json")?;
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
    let raw_json = fs::read_to_string("./tests/documents/json/empty_array.json")?;
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
    let raw_json = fs::read_to_string("./tests/documents/json/empty_array.json")?;
    let input = OwnedBytes::new(&raw_json.as_bytes())?;
    let engine = RecursiveEngine::compile_query(&jsonpath_query)?;
    let result = engine.run::<_, IndexResult>(&input)?;
    assert_eq!(result.get(), vec![0usize,], "result != expected");
    Ok(())
}
#[test]
fn empty_document_compressed_with_query_select_the_root_empty_query_with_buffered_input_and_count_result_using_main_engine(
) -> Result<(), Box<dyn Error>> {
    println ! ("on document empty.toml running the query  (select the root (empty query)) with Input impl BufferedInput and result mode CountResult");
    let jsonpath_query = JsonPathQuery::parse("")?;
    let json_file = fs::File::open("./tests/documents/json/compressed/empty.json")?;
    let input = BufferedInput::new(json_file);
    let engine = MainEngine::compile_query(&jsonpath_query)?;
    let result = engine.run::<_, CountResult>(&input)?;
    assert_eq!(result.get(), 0u64, "result != expected");
    Ok(())
}
#[test]
fn empty_document_compressed_with_query_select_the_root_empty_query_with_buffered_input_and_count_result_using_recursive_engine(
) -> Result<(), Box<dyn Error>> {
    println ! ("on document empty.toml running the query  (select the root (empty query)) with Input impl BufferedInput and result mode CountResult");
    let jsonpath_query = JsonPathQuery::parse("")?;
    let json_file = fs::File::open("./tests/documents/json/compressed/empty.json")?;
    let input = BufferedInput::new(json_file);
    let engine = RecursiveEngine::compile_query(&jsonpath_query)?;
    let result = engine.run::<_, CountResult>(&input)?;
    assert_eq!(result.get(), 0u64, "result != expected");
    Ok(())
}
#[test]
fn empty_document_compressed_with_query_select_the_root_empty_query_with_buffered_input_and_index_result_using_main_engine(
) -> Result<(), Box<dyn Error>> {
    println ! ("on document empty.toml running the query  (select the root (empty query)) with Input impl BufferedInput and result mode IndexResult");
    let jsonpath_query = JsonPathQuery::parse("")?;
    let json_file = fs::File::open("./tests/documents/json/compressed/empty.json")?;
    let input = BufferedInput::new(json_file);
    let engine = MainEngine::compile_query(&jsonpath_query)?;
    let result = engine.run::<_, IndexResult>(&input)?;
    assert_eq!(result.get(), vec![], "result != expected");
    Ok(())
}
#[test]
fn empty_document_compressed_with_query_select_the_root_empty_query_with_buffered_input_and_index_result_using_recursive_engine(
) -> Result<(), Box<dyn Error>> {
    println ! ("on document empty.toml running the query  (select the root (empty query)) with Input impl BufferedInput and result mode IndexResult");
    let jsonpath_query = JsonPathQuery::parse("")?;
    let json_file = fs::File::open("./tests/documents/json/compressed/empty.json")?;
    let input = BufferedInput::new(json_file);
    let engine = RecursiveEngine::compile_query(&jsonpath_query)?;
    let result = engine.run::<_, IndexResult>(&input)?;
    assert_eq!(result.get(), vec![], "result != expected");
    Ok(())
}
#[test]
fn empty_document_compressed_with_query_select_the_root_empty_query_with_mmap_input_and_count_result_using_main_engine(
) -> Result<(), Box<dyn Error>> {
    println ! ("on document empty.toml running the query  (select the root (empty query)) with Input impl MmapInput and result mode CountResult");
    let jsonpath_query = JsonPathQuery::parse("")?;
    let json_file = fs::File::open("./tests/documents/json/compressed/empty.json")?;
    let input = unsafe { MmapInput::map_file(&json_file)? };
    let engine = MainEngine::compile_query(&jsonpath_query)?;
    let result = engine.run::<_, CountResult>(&input)?;
    assert_eq!(result.get(), 0u64, "result != expected");
    Ok(())
}
#[test]
fn empty_document_compressed_with_query_select_the_root_empty_query_with_mmap_input_and_count_result_using_recursive_engine(
) -> Result<(), Box<dyn Error>> {
    println ! ("on document empty.toml running the query  (select the root (empty query)) with Input impl MmapInput and result mode CountResult");
    let jsonpath_query = JsonPathQuery::parse("")?;
    let json_file = fs::File::open("./tests/documents/json/compressed/empty.json")?;
    let input = unsafe { MmapInput::map_file(&json_file)? };
    let engine = RecursiveEngine::compile_query(&jsonpath_query)?;
    let result = engine.run::<_, CountResult>(&input)?;
    assert_eq!(result.get(), 0u64, "result != expected");
    Ok(())
}
#[test]
fn empty_document_compressed_with_query_select_the_root_empty_query_with_mmap_input_and_index_result_using_main_engine(
) -> Result<(), Box<dyn Error>> {
    println ! ("on document empty.toml running the query  (select the root (empty query)) with Input impl MmapInput and result mode IndexResult");
    let jsonpath_query = JsonPathQuery::parse("")?;
    let json_file = fs::File::open("./tests/documents/json/compressed/empty.json")?;
    let input = unsafe { MmapInput::map_file(&json_file)? };
    let engine = MainEngine::compile_query(&jsonpath_query)?;
    let result = engine.run::<_, IndexResult>(&input)?;
    assert_eq!(result.get(), vec![], "result != expected");
    Ok(())
}
#[test]
fn empty_document_compressed_with_query_select_the_root_empty_query_with_mmap_input_and_index_result_using_recursive_engine(
) -> Result<(), Box<dyn Error>> {
    println ! ("on document empty.toml running the query  (select the root (empty query)) with Input impl MmapInput and result mode IndexResult");
    let jsonpath_query = JsonPathQuery::parse("")?;
    let json_file = fs::File::open("./tests/documents/json/compressed/empty.json")?;
    let input = unsafe { MmapInput::map_file(&json_file)? };
    let engine = RecursiveEngine::compile_query(&jsonpath_query)?;
    let result = engine.run::<_, IndexResult>(&input)?;
    assert_eq!(result.get(), vec![], "result != expected");
    Ok(())
}
#[test]
fn empty_document_compressed_with_query_select_the_root_empty_query_with_owned_bytes_and_count_result_using_main_engine(
) -> Result<(), Box<dyn Error>> {
    println ! ("on document empty.toml running the query  (select the root (empty query)) with Input impl OwnedBytes and result mode CountResult");
    let jsonpath_query = JsonPathQuery::parse("")?;
    let raw_json = fs::read_to_string("./tests/documents/json/compressed/empty.json")?;
    let input = OwnedBytes::new(&raw_json.as_bytes())?;
    let engine = MainEngine::compile_query(&jsonpath_query)?;
    let result = engine.run::<_, CountResult>(&input)?;
    assert_eq!(result.get(), 0u64, "result != expected");
    Ok(())
}
#[test]
fn empty_document_compressed_with_query_select_the_root_empty_query_with_owned_bytes_and_count_result_using_recursive_engine(
) -> Result<(), Box<dyn Error>> {
    println ! ("on document empty.toml running the query  (select the root (empty query)) with Input impl OwnedBytes and result mode CountResult");
    let jsonpath_query = JsonPathQuery::parse("")?;
    let raw_json = fs::read_to_string("./tests/documents/json/compressed/empty.json")?;
    let input = OwnedBytes::new(&raw_json.as_bytes())?;
    let engine = RecursiveEngine::compile_query(&jsonpath_query)?;
    let result = engine.run::<_, CountResult>(&input)?;
    assert_eq!(result.get(), 0u64, "result != expected");
    Ok(())
}
#[test]
fn empty_document_compressed_with_query_select_the_root_empty_query_with_owned_bytes_and_index_result_using_main_engine(
) -> Result<(), Box<dyn Error>> {
    println ! ("on document empty.toml running the query  (select the root (empty query)) with Input impl OwnedBytes and result mode IndexResult");
    let jsonpath_query = JsonPathQuery::parse("")?;
    let raw_json = fs::read_to_string("./tests/documents/json/compressed/empty.json")?;
    let input = OwnedBytes::new(&raw_json.as_bytes())?;
    let engine = MainEngine::compile_query(&jsonpath_query)?;
    let result = engine.run::<_, IndexResult>(&input)?;
    assert_eq!(result.get(), vec![], "result != expected");
    Ok(())
}
#[test]
fn empty_document_compressed_with_query_select_the_root_empty_query_with_owned_bytes_and_index_result_using_recursive_engine(
) -> Result<(), Box<dyn Error>> {
    println ! ("on document empty.toml running the query  (select the root (empty query)) with Input impl OwnedBytes and result mode IndexResult");
    let jsonpath_query = JsonPathQuery::parse("")?;
    let raw_json = fs::read_to_string("./tests/documents/json/compressed/empty.json")?;
    let input = OwnedBytes::new(&raw_json.as_bytes())?;
    let engine = RecursiveEngine::compile_query(&jsonpath_query)?;
    let result = engine.run::<_, IndexResult>(&input)?;
    assert_eq!(result.get(), vec![], "result != expected");
    Ok(())
}
#[test]
fn empty_document_compressed_with_query_select_the_root_with_buffered_input_and_count_result_using_main_engine(
) -> Result<(), Box<dyn Error>> {
    println ! ("on document empty.toml running the query $ (select the root) with Input impl BufferedInput and result mode CountResult");
    let jsonpath_query = JsonPathQuery::parse("$")?;
    let json_file = fs::File::open("./tests/documents/json/compressed/empty.json")?;
    let input = BufferedInput::new(json_file);
    let engine = MainEngine::compile_query(&jsonpath_query)?;
    let result = engine.run::<_, CountResult>(&input)?;
    assert_eq!(result.get(), 0u64, "result != expected");
    Ok(())
}
#[test]
fn empty_document_compressed_with_query_select_the_root_with_buffered_input_and_count_result_using_recursive_engine(
) -> Result<(), Box<dyn Error>> {
    println ! ("on document empty.toml running the query $ (select the root) with Input impl BufferedInput and result mode CountResult");
    let jsonpath_query = JsonPathQuery::parse("$")?;
    let json_file = fs::File::open("./tests/documents/json/compressed/empty.json")?;
    let input = BufferedInput::new(json_file);
    let engine = RecursiveEngine::compile_query(&jsonpath_query)?;
    let result = engine.run::<_, CountResult>(&input)?;
    assert_eq!(result.get(), 0u64, "result != expected");
    Ok(())
}
#[test]
fn empty_document_compressed_with_query_select_the_root_with_buffered_input_and_index_result_using_main_engine(
) -> Result<(), Box<dyn Error>> {
    println ! ("on document empty.toml running the query $ (select the root) with Input impl BufferedInput and result mode IndexResult");
    let jsonpath_query = JsonPathQuery::parse("$")?;
    let json_file = fs::File::open("./tests/documents/json/compressed/empty.json")?;
    let input = BufferedInput::new(json_file);
    let engine = MainEngine::compile_query(&jsonpath_query)?;
    let result = engine.run::<_, IndexResult>(&input)?;
    assert_eq!(result.get(), vec![], "result != expected");
    Ok(())
}
#[test]
fn empty_document_compressed_with_query_select_the_root_with_buffered_input_and_index_result_using_recursive_engine(
) -> Result<(), Box<dyn Error>> {
    println ! ("on document empty.toml running the query $ (select the root) with Input impl BufferedInput and result mode IndexResult");
    let jsonpath_query = JsonPathQuery::parse("$")?;
    let json_file = fs::File::open("./tests/documents/json/compressed/empty.json")?;
    let input = BufferedInput::new(json_file);
    let engine = RecursiveEngine::compile_query(&jsonpath_query)?;
    let result = engine.run::<_, IndexResult>(&input)?;
    assert_eq!(result.get(), vec![], "result != expected");
    Ok(())
}
#[test]
fn empty_document_compressed_with_query_select_the_root_with_mmap_input_and_count_result_using_main_engine(
) -> Result<(), Box<dyn Error>> {
    println ! ("on document empty.toml running the query $ (select the root) with Input impl MmapInput and result mode CountResult");
    let jsonpath_query = JsonPathQuery::parse("$")?;
    let json_file = fs::File::open("./tests/documents/json/compressed/empty.json")?;
    let input = unsafe { MmapInput::map_file(&json_file)? };
    let engine = MainEngine::compile_query(&jsonpath_query)?;
    let result = engine.run::<_, CountResult>(&input)?;
    assert_eq!(result.get(), 0u64, "result != expected");
    Ok(())
}
#[test]
fn empty_document_compressed_with_query_select_the_root_with_mmap_input_and_count_result_using_recursive_engine(
) -> Result<(), Box<dyn Error>> {
    println ! ("on document empty.toml running the query $ (select the root) with Input impl MmapInput and result mode CountResult");
    let jsonpath_query = JsonPathQuery::parse("$")?;
    let json_file = fs::File::open("./tests/documents/json/compressed/empty.json")?;
    let input = unsafe { MmapInput::map_file(&json_file)? };
    let engine = RecursiveEngine::compile_query(&jsonpath_query)?;
    let result = engine.run::<_, CountResult>(&input)?;
    assert_eq!(result.get(), 0u64, "result != expected");
    Ok(())
}
#[test]
fn empty_document_compressed_with_query_select_the_root_with_mmap_input_and_index_result_using_main_engine(
) -> Result<(), Box<dyn Error>> {
    println ! ("on document empty.toml running the query $ (select the root) with Input impl MmapInput and result mode IndexResult");
    let jsonpath_query = JsonPathQuery::parse("$")?;
    let json_file = fs::File::open("./tests/documents/json/compressed/empty.json")?;
    let input = unsafe { MmapInput::map_file(&json_file)? };
    let engine = MainEngine::compile_query(&jsonpath_query)?;
    let result = engine.run::<_, IndexResult>(&input)?;
    assert_eq!(result.get(), vec![], "result != expected");
    Ok(())
}
#[test]
fn empty_document_compressed_with_query_select_the_root_with_mmap_input_and_index_result_using_recursive_engine(
) -> Result<(), Box<dyn Error>> {
    println ! ("on document empty.toml running the query $ (select the root) with Input impl MmapInput and result mode IndexResult");
    let jsonpath_query = JsonPathQuery::parse("$")?;
    let json_file = fs::File::open("./tests/documents/json/compressed/empty.json")?;
    let input = unsafe { MmapInput::map_file(&json_file)? };
    let engine = RecursiveEngine::compile_query(&jsonpath_query)?;
    let result = engine.run::<_, IndexResult>(&input)?;
    assert_eq!(result.get(), vec![], "result != expected");
    Ok(())
}
#[test]
fn empty_document_compressed_with_query_select_the_root_with_owned_bytes_and_count_result_using_main_engine(
) -> Result<(), Box<dyn Error>> {
    println ! ("on document empty.toml running the query $ (select the root) with Input impl OwnedBytes and result mode CountResult");
    let jsonpath_query = JsonPathQuery::parse("$")?;
    let raw_json = fs::read_to_string("./tests/documents/json/compressed/empty.json")?;
    let input = OwnedBytes::new(&raw_json.as_bytes())?;
    let engine = MainEngine::compile_query(&jsonpath_query)?;
    let result = engine.run::<_, CountResult>(&input)?;
    assert_eq!(result.get(), 0u64, "result != expected");
    Ok(())
}
#[test]
fn empty_document_compressed_with_query_select_the_root_with_owned_bytes_and_count_result_using_recursive_engine(
) -> Result<(), Box<dyn Error>> {
    println ! ("on document empty.toml running the query $ (select the root) with Input impl OwnedBytes and result mode CountResult");
    let jsonpath_query = JsonPathQuery::parse("$")?;
    let raw_json = fs::read_to_string("./tests/documents/json/compressed/empty.json")?;
    let input = OwnedBytes::new(&raw_json.as_bytes())?;
    let engine = RecursiveEngine::compile_query(&jsonpath_query)?;
    let result = engine.run::<_, CountResult>(&input)?;
    assert_eq!(result.get(), 0u64, "result != expected");
    Ok(())
}
#[test]
fn empty_document_compressed_with_query_select_the_root_with_owned_bytes_and_index_result_using_main_engine(
) -> Result<(), Box<dyn Error>> {
    println ! ("on document empty.toml running the query $ (select the root) with Input impl OwnedBytes and result mode IndexResult");
    let jsonpath_query = JsonPathQuery::parse("$")?;
    let raw_json = fs::read_to_string("./tests/documents/json/compressed/empty.json")?;
    let input = OwnedBytes::new(&raw_json.as_bytes())?;
    let engine = MainEngine::compile_query(&jsonpath_query)?;
    let result = engine.run::<_, IndexResult>(&input)?;
    assert_eq!(result.get(), vec![], "result != expected");
    Ok(())
}
#[test]
fn empty_document_compressed_with_query_select_the_root_with_owned_bytes_and_index_result_using_recursive_engine(
) -> Result<(), Box<dyn Error>> {
    println ! ("on document empty.toml running the query $ (select the root) with Input impl OwnedBytes and result mode IndexResult");
    let jsonpath_query = JsonPathQuery::parse("$")?;
    let raw_json = fs::read_to_string("./tests/documents/json/compressed/empty.json")?;
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
    let json_file = fs::File::open("./tests/documents/json/empty.json")?;
    let input = BufferedInput::new(json_file);
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
    let json_file = fs::File::open("./tests/documents/json/empty.json")?;
    let input = BufferedInput::new(json_file);
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
    let json_file = fs::File::open("./tests/documents/json/empty.json")?;
    let input = BufferedInput::new(json_file);
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
    let json_file = fs::File::open("./tests/documents/json/empty.json")?;
    let input = BufferedInput::new(json_file);
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
    let json_file = fs::File::open("./tests/documents/json/empty.json")?;
    let input = unsafe { MmapInput::map_file(&json_file)? };
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
    let json_file = fs::File::open("./tests/documents/json/empty.json")?;
    let input = unsafe { MmapInput::map_file(&json_file)? };
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
    let json_file = fs::File::open("./tests/documents/json/empty.json")?;
    let input = unsafe { MmapInput::map_file(&json_file)? };
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
    let json_file = fs::File::open("./tests/documents/json/empty.json")?;
    let input = unsafe { MmapInput::map_file(&json_file)? };
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
    let raw_json = fs::read_to_string("./tests/documents/json/empty.json")?;
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
    let raw_json = fs::read_to_string("./tests/documents/json/empty.json")?;
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
    let raw_json = fs::read_to_string("./tests/documents/json/empty.json")?;
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
    let raw_json = fs::read_to_string("./tests/documents/json/empty.json")?;
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
    let json_file = fs::File::open("./tests/documents/json/empty.json")?;
    let input = BufferedInput::new(json_file);
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
    let json_file = fs::File::open("./tests/documents/json/empty.json")?;
    let input = BufferedInput::new(json_file);
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
    let json_file = fs::File::open("./tests/documents/json/empty.json")?;
    let input = BufferedInput::new(json_file);
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
    let json_file = fs::File::open("./tests/documents/json/empty.json")?;
    let input = BufferedInput::new(json_file);
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
    let json_file = fs::File::open("./tests/documents/json/empty.json")?;
    let input = unsafe { MmapInput::map_file(&json_file)? };
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
    let json_file = fs::File::open("./tests/documents/json/empty.json")?;
    let input = unsafe { MmapInput::map_file(&json_file)? };
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
    let json_file = fs::File::open("./tests/documents/json/empty.json")?;
    let input = unsafe { MmapInput::map_file(&json_file)? };
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
    let json_file = fs::File::open("./tests/documents/json/empty.json")?;
    let input = unsafe { MmapInput::map_file(&json_file)? };
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
    let raw_json = fs::read_to_string("./tests/documents/json/empty.json")?;
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
    let raw_json = fs::read_to_string("./tests/documents/json/empty.json")?;
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
    let raw_json = fs::read_to_string("./tests/documents/json/empty.json")?;
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
    let raw_json = fs::read_to_string("./tests/documents/json/empty.json")?;
    let input = OwnedBytes::new(&raw_json.as_bytes())?;
    let engine = RecursiveEngine::compile_query(&jsonpath_query)?;
    let result = engine.run::<_, IndexResult>(&input)?;
    assert_eq!(result.get(), vec![], "result != expected");
    Ok(())
}
#[test]
fn empty_object_root_compressed_with_query_select_any_child_there_are_none_with_buffered_input_and_count_result_using_main_engine(
) -> Result<(), Box<dyn Error>> {
    println ! ("on document empty_object.toml running the query $.* (select any child (there are none)) with Input impl BufferedInput and result mode CountResult");
    let jsonpath_query = JsonPathQuery::parse("$.*")?;
    let json_file = fs::File::open("./tests/documents/json/compressed/empty_object.json")?;
    let input = BufferedInput::new(json_file);
    let engine = MainEngine::compile_query(&jsonpath_query)?;
    let result = engine.run::<_, CountResult>(&input)?;
    assert_eq!(result.get(), 0u64, "result != expected");
    Ok(())
}
#[test]
fn empty_object_root_compressed_with_query_select_any_child_there_are_none_with_buffered_input_and_count_result_using_recursive_engine(
) -> Result<(), Box<dyn Error>> {
    println ! ("on document empty_object.toml running the query $.* (select any child (there are none)) with Input impl BufferedInput and result mode CountResult");
    let jsonpath_query = JsonPathQuery::parse("$.*")?;
    let json_file = fs::File::open("./tests/documents/json/compressed/empty_object.json")?;
    let input = BufferedInput::new(json_file);
    let engine = RecursiveEngine::compile_query(&jsonpath_query)?;
    let result = engine.run::<_, CountResult>(&input)?;
    assert_eq!(result.get(), 0u64, "result != expected");
    Ok(())
}
#[test]
fn empty_object_root_compressed_with_query_select_any_child_there_are_none_with_buffered_input_and_index_result_using_main_engine(
) -> Result<(), Box<dyn Error>> {
    println ! ("on document empty_object.toml running the query $.* (select any child (there are none)) with Input impl BufferedInput and result mode IndexResult");
    let jsonpath_query = JsonPathQuery::parse("$.*")?;
    let json_file = fs::File::open("./tests/documents/json/compressed/empty_object.json")?;
    let input = BufferedInput::new(json_file);
    let engine = MainEngine::compile_query(&jsonpath_query)?;
    let result = engine.run::<_, IndexResult>(&input)?;
    assert_eq!(result.get(), vec![], "result != expected");
    Ok(())
}
#[test]
fn empty_object_root_compressed_with_query_select_any_child_there_are_none_with_buffered_input_and_index_result_using_recursive_engine(
) -> Result<(), Box<dyn Error>> {
    println ! ("on document empty_object.toml running the query $.* (select any child (there are none)) with Input impl BufferedInput and result mode IndexResult");
    let jsonpath_query = JsonPathQuery::parse("$.*")?;
    let json_file = fs::File::open("./tests/documents/json/compressed/empty_object.json")?;
    let input = BufferedInput::new(json_file);
    let engine = RecursiveEngine::compile_query(&jsonpath_query)?;
    let result = engine.run::<_, IndexResult>(&input)?;
    assert_eq!(result.get(), vec![], "result != expected");
    Ok(())
}
#[test]
fn empty_object_root_compressed_with_query_select_any_child_there_are_none_with_mmap_input_and_count_result_using_main_engine(
) -> Result<(), Box<dyn Error>> {
    println ! ("on document empty_object.toml running the query $.* (select any child (there are none)) with Input impl MmapInput and result mode CountResult");
    let jsonpath_query = JsonPathQuery::parse("$.*")?;
    let json_file = fs::File::open("./tests/documents/json/compressed/empty_object.json")?;
    let input = unsafe { MmapInput::map_file(&json_file)? };
    let engine = MainEngine::compile_query(&jsonpath_query)?;
    let result = engine.run::<_, CountResult>(&input)?;
    assert_eq!(result.get(), 0u64, "result != expected");
    Ok(())
}
#[test]
fn empty_object_root_compressed_with_query_select_any_child_there_are_none_with_mmap_input_and_count_result_using_recursive_engine(
) -> Result<(), Box<dyn Error>> {
    println ! ("on document empty_object.toml running the query $.* (select any child (there are none)) with Input impl MmapInput and result mode CountResult");
    let jsonpath_query = JsonPathQuery::parse("$.*")?;
    let json_file = fs::File::open("./tests/documents/json/compressed/empty_object.json")?;
    let input = unsafe { MmapInput::map_file(&json_file)? };
    let engine = RecursiveEngine::compile_query(&jsonpath_query)?;
    let result = engine.run::<_, CountResult>(&input)?;
    assert_eq!(result.get(), 0u64, "result != expected");
    Ok(())
}
#[test]
fn empty_object_root_compressed_with_query_select_any_child_there_are_none_with_mmap_input_and_index_result_using_main_engine(
) -> Result<(), Box<dyn Error>> {
    println ! ("on document empty_object.toml running the query $.* (select any child (there are none)) with Input impl MmapInput and result mode IndexResult");
    let jsonpath_query = JsonPathQuery::parse("$.*")?;
    let json_file = fs::File::open("./tests/documents/json/compressed/empty_object.json")?;
    let input = unsafe { MmapInput::map_file(&json_file)? };
    let engine = MainEngine::compile_query(&jsonpath_query)?;
    let result = engine.run::<_, IndexResult>(&input)?;
    assert_eq!(result.get(), vec![], "result != expected");
    Ok(())
}
#[test]
fn empty_object_root_compressed_with_query_select_any_child_there_are_none_with_mmap_input_and_index_result_using_recursive_engine(
) -> Result<(), Box<dyn Error>> {
    println ! ("on document empty_object.toml running the query $.* (select any child (there are none)) with Input impl MmapInput and result mode IndexResult");
    let jsonpath_query = JsonPathQuery::parse("$.*")?;
    let json_file = fs::File::open("./tests/documents/json/compressed/empty_object.json")?;
    let input = unsafe { MmapInput::map_file(&json_file)? };
    let engine = RecursiveEngine::compile_query(&jsonpath_query)?;
    let result = engine.run::<_, IndexResult>(&input)?;
    assert_eq!(result.get(), vec![], "result != expected");
    Ok(())
}
#[test]
fn empty_object_root_compressed_with_query_select_any_child_there_are_none_with_owned_bytes_and_count_result_using_main_engine(
) -> Result<(), Box<dyn Error>> {
    println ! ("on document empty_object.toml running the query $.* (select any child (there are none)) with Input impl OwnedBytes and result mode CountResult");
    let jsonpath_query = JsonPathQuery::parse("$.*")?;
    let raw_json = fs::read_to_string("./tests/documents/json/compressed/empty_object.json")?;
    let input = OwnedBytes::new(&raw_json.as_bytes())?;
    let engine = MainEngine::compile_query(&jsonpath_query)?;
    let result = engine.run::<_, CountResult>(&input)?;
    assert_eq!(result.get(), 0u64, "result != expected");
    Ok(())
}
#[test]
fn empty_object_root_compressed_with_query_select_any_child_there_are_none_with_owned_bytes_and_count_result_using_recursive_engine(
) -> Result<(), Box<dyn Error>> {
    println ! ("on document empty_object.toml running the query $.* (select any child (there are none)) with Input impl OwnedBytes and result mode CountResult");
    let jsonpath_query = JsonPathQuery::parse("$.*")?;
    let raw_json = fs::read_to_string("./tests/documents/json/compressed/empty_object.json")?;
    let input = OwnedBytes::new(&raw_json.as_bytes())?;
    let engine = RecursiveEngine::compile_query(&jsonpath_query)?;
    let result = engine.run::<_, CountResult>(&input)?;
    assert_eq!(result.get(), 0u64, "result != expected");
    Ok(())
}
#[test]
fn empty_object_root_compressed_with_query_select_any_child_there_are_none_with_owned_bytes_and_index_result_using_main_engine(
) -> Result<(), Box<dyn Error>> {
    println ! ("on document empty_object.toml running the query $.* (select any child (there are none)) with Input impl OwnedBytes and result mode IndexResult");
    let jsonpath_query = JsonPathQuery::parse("$.*")?;
    let raw_json = fs::read_to_string("./tests/documents/json/compressed/empty_object.json")?;
    let input = OwnedBytes::new(&raw_json.as_bytes())?;
    let engine = MainEngine::compile_query(&jsonpath_query)?;
    let result = engine.run::<_, IndexResult>(&input)?;
    assert_eq!(result.get(), vec![], "result != expected");
    Ok(())
}
#[test]
fn empty_object_root_compressed_with_query_select_any_child_there_are_none_with_owned_bytes_and_index_result_using_recursive_engine(
) -> Result<(), Box<dyn Error>> {
    println ! ("on document empty_object.toml running the query $.* (select any child (there are none)) with Input impl OwnedBytes and result mode IndexResult");
    let jsonpath_query = JsonPathQuery::parse("$.*")?;
    let raw_json = fs::read_to_string("./tests/documents/json/compressed/empty_object.json")?;
    let input = OwnedBytes::new(&raw_json.as_bytes())?;
    let engine = RecursiveEngine::compile_query(&jsonpath_query)?;
    let result = engine.run::<_, IndexResult>(&input)?;
    assert_eq!(result.get(), vec![], "result != expected");
    Ok(())
}
#[test]
fn empty_object_root_compressed_with_query_select_any_descendant_there_are_none_with_buffered_input_and_count_result_using_main_engine(
) -> Result<(), Box<dyn Error>> {
    println ! ("on document empty_object.toml running the query $..* (select any descendant (there are none)) with Input impl BufferedInput and result mode CountResult");
    let jsonpath_query = JsonPathQuery::parse("$..*")?;
    let json_file = fs::File::open("./tests/documents/json/compressed/empty_object.json")?;
    let input = BufferedInput::new(json_file);
    let engine = MainEngine::compile_query(&jsonpath_query)?;
    let result = engine.run::<_, CountResult>(&input)?;
    assert_eq!(result.get(), 0u64, "result != expected");
    Ok(())
}
#[test]
fn empty_object_root_compressed_with_query_select_any_descendant_there_are_none_with_buffered_input_and_count_result_using_recursive_engine(
) -> Result<(), Box<dyn Error>> {
    println ! ("on document empty_object.toml running the query $..* (select any descendant (there are none)) with Input impl BufferedInput and result mode CountResult");
    let jsonpath_query = JsonPathQuery::parse("$..*")?;
    let json_file = fs::File::open("./tests/documents/json/compressed/empty_object.json")?;
    let input = BufferedInput::new(json_file);
    let engine = RecursiveEngine::compile_query(&jsonpath_query)?;
    let result = engine.run::<_, CountResult>(&input)?;
    assert_eq!(result.get(), 0u64, "result != expected");
    Ok(())
}
#[test]
fn empty_object_root_compressed_with_query_select_any_descendant_there_are_none_with_buffered_input_and_index_result_using_main_engine(
) -> Result<(), Box<dyn Error>> {
    println ! ("on document empty_object.toml running the query $..* (select any descendant (there are none)) with Input impl BufferedInput and result mode IndexResult");
    let jsonpath_query = JsonPathQuery::parse("$..*")?;
    let json_file = fs::File::open("./tests/documents/json/compressed/empty_object.json")?;
    let input = BufferedInput::new(json_file);
    let engine = MainEngine::compile_query(&jsonpath_query)?;
    let result = engine.run::<_, IndexResult>(&input)?;
    assert_eq!(result.get(), vec![], "result != expected");
    Ok(())
}
#[test]
fn empty_object_root_compressed_with_query_select_any_descendant_there_are_none_with_buffered_input_and_index_result_using_recursive_engine(
) -> Result<(), Box<dyn Error>> {
    println ! ("on document empty_object.toml running the query $..* (select any descendant (there are none)) with Input impl BufferedInput and result mode IndexResult");
    let jsonpath_query = JsonPathQuery::parse("$..*")?;
    let json_file = fs::File::open("./tests/documents/json/compressed/empty_object.json")?;
    let input = BufferedInput::new(json_file);
    let engine = RecursiveEngine::compile_query(&jsonpath_query)?;
    let result = engine.run::<_, IndexResult>(&input)?;
    assert_eq!(result.get(), vec![], "result != expected");
    Ok(())
}
#[test]
fn empty_object_root_compressed_with_query_select_any_descendant_there_are_none_with_mmap_input_and_count_result_using_main_engine(
) -> Result<(), Box<dyn Error>> {
    println ! ("on document empty_object.toml running the query $..* (select any descendant (there are none)) with Input impl MmapInput and result mode CountResult");
    let jsonpath_query = JsonPathQuery::parse("$..*")?;
    let json_file = fs::File::open("./tests/documents/json/compressed/empty_object.json")?;
    let input = unsafe { MmapInput::map_file(&json_file)? };
    let engine = MainEngine::compile_query(&jsonpath_query)?;
    let result = engine.run::<_, CountResult>(&input)?;
    assert_eq!(result.get(), 0u64, "result != expected");
    Ok(())
}
#[test]
fn empty_object_root_compressed_with_query_select_any_descendant_there_are_none_with_mmap_input_and_count_result_using_recursive_engine(
) -> Result<(), Box<dyn Error>> {
    println ! ("on document empty_object.toml running the query $..* (select any descendant (there are none)) with Input impl MmapInput and result mode CountResult");
    let jsonpath_query = JsonPathQuery::parse("$..*")?;
    let json_file = fs::File::open("./tests/documents/json/compressed/empty_object.json")?;
    let input = unsafe { MmapInput::map_file(&json_file)? };
    let engine = RecursiveEngine::compile_query(&jsonpath_query)?;
    let result = engine.run::<_, CountResult>(&input)?;
    assert_eq!(result.get(), 0u64, "result != expected");
    Ok(())
}
#[test]
fn empty_object_root_compressed_with_query_select_any_descendant_there_are_none_with_mmap_input_and_index_result_using_main_engine(
) -> Result<(), Box<dyn Error>> {
    println ! ("on document empty_object.toml running the query $..* (select any descendant (there are none)) with Input impl MmapInput and result mode IndexResult");
    let jsonpath_query = JsonPathQuery::parse("$..*")?;
    let json_file = fs::File::open("./tests/documents/json/compressed/empty_object.json")?;
    let input = unsafe { MmapInput::map_file(&json_file)? };
    let engine = MainEngine::compile_query(&jsonpath_query)?;
    let result = engine.run::<_, IndexResult>(&input)?;
    assert_eq!(result.get(), vec![], "result != expected");
    Ok(())
}
#[test]
fn empty_object_root_compressed_with_query_select_any_descendant_there_are_none_with_mmap_input_and_index_result_using_recursive_engine(
) -> Result<(), Box<dyn Error>> {
    println ! ("on document empty_object.toml running the query $..* (select any descendant (there are none)) with Input impl MmapInput and result mode IndexResult");
    let jsonpath_query = JsonPathQuery::parse("$..*")?;
    let json_file = fs::File::open("./tests/documents/json/compressed/empty_object.json")?;
    let input = unsafe { MmapInput::map_file(&json_file)? };
    let engine = RecursiveEngine::compile_query(&jsonpath_query)?;
    let result = engine.run::<_, IndexResult>(&input)?;
    assert_eq!(result.get(), vec![], "result != expected");
    Ok(())
}
#[test]
fn empty_object_root_compressed_with_query_select_any_descendant_there_are_none_with_owned_bytes_and_count_result_using_main_engine(
) -> Result<(), Box<dyn Error>> {
    println ! ("on document empty_object.toml running the query $..* (select any descendant (there are none)) with Input impl OwnedBytes and result mode CountResult");
    let jsonpath_query = JsonPathQuery::parse("$..*")?;
    let raw_json = fs::read_to_string("./tests/documents/json/compressed/empty_object.json")?;
    let input = OwnedBytes::new(&raw_json.as_bytes())?;
    let engine = MainEngine::compile_query(&jsonpath_query)?;
    let result = engine.run::<_, CountResult>(&input)?;
    assert_eq!(result.get(), 0u64, "result != expected");
    Ok(())
}
#[test]
fn empty_object_root_compressed_with_query_select_any_descendant_there_are_none_with_owned_bytes_and_count_result_using_recursive_engine(
) -> Result<(), Box<dyn Error>> {
    println ! ("on document empty_object.toml running the query $..* (select any descendant (there are none)) with Input impl OwnedBytes and result mode CountResult");
    let jsonpath_query = JsonPathQuery::parse("$..*")?;
    let raw_json = fs::read_to_string("./tests/documents/json/compressed/empty_object.json")?;
    let input = OwnedBytes::new(&raw_json.as_bytes())?;
    let engine = RecursiveEngine::compile_query(&jsonpath_query)?;
    let result = engine.run::<_, CountResult>(&input)?;
    assert_eq!(result.get(), 0u64, "result != expected");
    Ok(())
}
#[test]
fn empty_object_root_compressed_with_query_select_any_descendant_there_are_none_with_owned_bytes_and_index_result_using_main_engine(
) -> Result<(), Box<dyn Error>> {
    println ! ("on document empty_object.toml running the query $..* (select any descendant (there are none)) with Input impl OwnedBytes and result mode IndexResult");
    let jsonpath_query = JsonPathQuery::parse("$..*")?;
    let raw_json = fs::read_to_string("./tests/documents/json/compressed/empty_object.json")?;
    let input = OwnedBytes::new(&raw_json.as_bytes())?;
    let engine = MainEngine::compile_query(&jsonpath_query)?;
    let result = engine.run::<_, IndexResult>(&input)?;
    assert_eq!(result.get(), vec![], "result != expected");
    Ok(())
}
#[test]
fn empty_object_root_compressed_with_query_select_any_descendant_there_are_none_with_owned_bytes_and_index_result_using_recursive_engine(
) -> Result<(), Box<dyn Error>> {
    println ! ("on document empty_object.toml running the query $..* (select any descendant (there are none)) with Input impl OwnedBytes and result mode IndexResult");
    let jsonpath_query = JsonPathQuery::parse("$..*")?;
    let raw_json = fs::read_to_string("./tests/documents/json/compressed/empty_object.json")?;
    let input = OwnedBytes::new(&raw_json.as_bytes())?;
    let engine = RecursiveEngine::compile_query(&jsonpath_query)?;
    let result = engine.run::<_, IndexResult>(&input)?;
    assert_eq!(result.get(), vec![], "result != expected");
    Ok(())
}
#[test]
fn empty_object_root_compressed_with_query_select_the_child_named_a_which_does_not_exist_with_buffered_input_and_count_result_using_main_engine(
) -> Result<(), Box<dyn Error>> {
    println ! ("on document empty_object.toml running the query $.a (select the child named 'a' (which does not exist)) with Input impl BufferedInput and result mode CountResult");
    let jsonpath_query = JsonPathQuery::parse("$.a")?;
    let json_file = fs::File::open("./tests/documents/json/compressed/empty_object.json")?;
    let input = BufferedInput::new(json_file);
    let engine = MainEngine::compile_query(&jsonpath_query)?;
    let result = engine.run::<_, CountResult>(&input)?;
    assert_eq!(result.get(), 0u64, "result != expected");
    Ok(())
}
#[test]
fn empty_object_root_compressed_with_query_select_the_child_named_a_which_does_not_exist_with_buffered_input_and_count_result_using_recursive_engine(
) -> Result<(), Box<dyn Error>> {
    println ! ("on document empty_object.toml running the query $.a (select the child named 'a' (which does not exist)) with Input impl BufferedInput and result mode CountResult");
    let jsonpath_query = JsonPathQuery::parse("$.a")?;
    let json_file = fs::File::open("./tests/documents/json/compressed/empty_object.json")?;
    let input = BufferedInput::new(json_file);
    let engine = RecursiveEngine::compile_query(&jsonpath_query)?;
    let result = engine.run::<_, CountResult>(&input)?;
    assert_eq!(result.get(), 0u64, "result != expected");
    Ok(())
}
#[test]
fn empty_object_root_compressed_with_query_select_the_child_named_a_which_does_not_exist_with_buffered_input_and_index_result_using_main_engine(
) -> Result<(), Box<dyn Error>> {
    println ! ("on document empty_object.toml running the query $.a (select the child named 'a' (which does not exist)) with Input impl BufferedInput and result mode IndexResult");
    let jsonpath_query = JsonPathQuery::parse("$.a")?;
    let json_file = fs::File::open("./tests/documents/json/compressed/empty_object.json")?;
    let input = BufferedInput::new(json_file);
    let engine = MainEngine::compile_query(&jsonpath_query)?;
    let result = engine.run::<_, IndexResult>(&input)?;
    assert_eq!(result.get(), vec![], "result != expected");
    Ok(())
}
#[test]
fn empty_object_root_compressed_with_query_select_the_child_named_a_which_does_not_exist_with_buffered_input_and_index_result_using_recursive_engine(
) -> Result<(), Box<dyn Error>> {
    println ! ("on document empty_object.toml running the query $.a (select the child named 'a' (which does not exist)) with Input impl BufferedInput and result mode IndexResult");
    let jsonpath_query = JsonPathQuery::parse("$.a")?;
    let json_file = fs::File::open("./tests/documents/json/compressed/empty_object.json")?;
    let input = BufferedInput::new(json_file);
    let engine = RecursiveEngine::compile_query(&jsonpath_query)?;
    let result = engine.run::<_, IndexResult>(&input)?;
    assert_eq!(result.get(), vec![], "result != expected");
    Ok(())
}
#[test]
fn empty_object_root_compressed_with_query_select_the_child_named_a_which_does_not_exist_with_mmap_input_and_count_result_using_main_engine(
) -> Result<(), Box<dyn Error>> {
    println ! ("on document empty_object.toml running the query $.a (select the child named 'a' (which does not exist)) with Input impl MmapInput and result mode CountResult");
    let jsonpath_query = JsonPathQuery::parse("$.a")?;
    let json_file = fs::File::open("./tests/documents/json/compressed/empty_object.json")?;
    let input = unsafe { MmapInput::map_file(&json_file)? };
    let engine = MainEngine::compile_query(&jsonpath_query)?;
    let result = engine.run::<_, CountResult>(&input)?;
    assert_eq!(result.get(), 0u64, "result != expected");
    Ok(())
}
#[test]
fn empty_object_root_compressed_with_query_select_the_child_named_a_which_does_not_exist_with_mmap_input_and_count_result_using_recursive_engine(
) -> Result<(), Box<dyn Error>> {
    println ! ("on document empty_object.toml running the query $.a (select the child named 'a' (which does not exist)) with Input impl MmapInput and result mode CountResult");
    let jsonpath_query = JsonPathQuery::parse("$.a")?;
    let json_file = fs::File::open("./tests/documents/json/compressed/empty_object.json")?;
    let input = unsafe { MmapInput::map_file(&json_file)? };
    let engine = RecursiveEngine::compile_query(&jsonpath_query)?;
    let result = engine.run::<_, CountResult>(&input)?;
    assert_eq!(result.get(), 0u64, "result != expected");
    Ok(())
}
#[test]
fn empty_object_root_compressed_with_query_select_the_child_named_a_which_does_not_exist_with_mmap_input_and_index_result_using_main_engine(
) -> Result<(), Box<dyn Error>> {
    println ! ("on document empty_object.toml running the query $.a (select the child named 'a' (which does not exist)) with Input impl MmapInput and result mode IndexResult");
    let jsonpath_query = JsonPathQuery::parse("$.a")?;
    let json_file = fs::File::open("./tests/documents/json/compressed/empty_object.json")?;
    let input = unsafe { MmapInput::map_file(&json_file)? };
    let engine = MainEngine::compile_query(&jsonpath_query)?;
    let result = engine.run::<_, IndexResult>(&input)?;
    assert_eq!(result.get(), vec![], "result != expected");
    Ok(())
}
#[test]
fn empty_object_root_compressed_with_query_select_the_child_named_a_which_does_not_exist_with_mmap_input_and_index_result_using_recursive_engine(
) -> Result<(), Box<dyn Error>> {
    println ! ("on document empty_object.toml running the query $.a (select the child named 'a' (which does not exist)) with Input impl MmapInput and result mode IndexResult");
    let jsonpath_query = JsonPathQuery::parse("$.a")?;
    let json_file = fs::File::open("./tests/documents/json/compressed/empty_object.json")?;
    let input = unsafe { MmapInput::map_file(&json_file)? };
    let engine = RecursiveEngine::compile_query(&jsonpath_query)?;
    let result = engine.run::<_, IndexResult>(&input)?;
    assert_eq!(result.get(), vec![], "result != expected");
    Ok(())
}
#[test]
fn empty_object_root_compressed_with_query_select_the_child_named_a_which_does_not_exist_with_owned_bytes_and_count_result_using_main_engine(
) -> Result<(), Box<dyn Error>> {
    println ! ("on document empty_object.toml running the query $.a (select the child named 'a' (which does not exist)) with Input impl OwnedBytes and result mode CountResult");
    let jsonpath_query = JsonPathQuery::parse("$.a")?;
    let raw_json = fs::read_to_string("./tests/documents/json/compressed/empty_object.json")?;
    let input = OwnedBytes::new(&raw_json.as_bytes())?;
    let engine = MainEngine::compile_query(&jsonpath_query)?;
    let result = engine.run::<_, CountResult>(&input)?;
    assert_eq!(result.get(), 0u64, "result != expected");
    Ok(())
}
#[test]
fn empty_object_root_compressed_with_query_select_the_child_named_a_which_does_not_exist_with_owned_bytes_and_count_result_using_recursive_engine(
) -> Result<(), Box<dyn Error>> {
    println ! ("on document empty_object.toml running the query $.a (select the child named 'a' (which does not exist)) with Input impl OwnedBytes and result mode CountResult");
    let jsonpath_query = JsonPathQuery::parse("$.a")?;
    let raw_json = fs::read_to_string("./tests/documents/json/compressed/empty_object.json")?;
    let input = OwnedBytes::new(&raw_json.as_bytes())?;
    let engine = RecursiveEngine::compile_query(&jsonpath_query)?;
    let result = engine.run::<_, CountResult>(&input)?;
    assert_eq!(result.get(), 0u64, "result != expected");
    Ok(())
}
#[test]
fn empty_object_root_compressed_with_query_select_the_child_named_a_which_does_not_exist_with_owned_bytes_and_index_result_using_main_engine(
) -> Result<(), Box<dyn Error>> {
    println ! ("on document empty_object.toml running the query $.a (select the child named 'a' (which does not exist)) with Input impl OwnedBytes and result mode IndexResult");
    let jsonpath_query = JsonPathQuery::parse("$.a")?;
    let raw_json = fs::read_to_string("./tests/documents/json/compressed/empty_object.json")?;
    let input = OwnedBytes::new(&raw_json.as_bytes())?;
    let engine = MainEngine::compile_query(&jsonpath_query)?;
    let result = engine.run::<_, IndexResult>(&input)?;
    assert_eq!(result.get(), vec![], "result != expected");
    Ok(())
}
#[test]
fn empty_object_root_compressed_with_query_select_the_child_named_a_which_does_not_exist_with_owned_bytes_and_index_result_using_recursive_engine(
) -> Result<(), Box<dyn Error>> {
    println ! ("on document empty_object.toml running the query $.a (select the child named 'a' (which does not exist)) with Input impl OwnedBytes and result mode IndexResult");
    let jsonpath_query = JsonPathQuery::parse("$.a")?;
    let raw_json = fs::read_to_string("./tests/documents/json/compressed/empty_object.json")?;
    let input = OwnedBytes::new(&raw_json.as_bytes())?;
    let engine = RecursiveEngine::compile_query(&jsonpath_query)?;
    let result = engine.run::<_, IndexResult>(&input)?;
    assert_eq!(result.get(), vec![], "result != expected");
    Ok(())
}
#[test]
fn empty_object_root_compressed_with_query_select_the_root_empty_query_with_buffered_input_and_count_result_using_main_engine(
) -> Result<(), Box<dyn Error>> {
    println ! ("on document empty_object.toml running the query  (select the root (empty query)) with Input impl BufferedInput and result mode CountResult");
    let jsonpath_query = JsonPathQuery::parse("")?;
    let json_file = fs::File::open("./tests/documents/json/compressed/empty_object.json")?;
    let input = BufferedInput::new(json_file);
    let engine = MainEngine::compile_query(&jsonpath_query)?;
    let result = engine.run::<_, CountResult>(&input)?;
    assert_eq!(result.get(), 1u64, "result != expected");
    Ok(())
}
#[test]
fn empty_object_root_compressed_with_query_select_the_root_empty_query_with_buffered_input_and_count_result_using_recursive_engine(
) -> Result<(), Box<dyn Error>> {
    println ! ("on document empty_object.toml running the query  (select the root (empty query)) with Input impl BufferedInput and result mode CountResult");
    let jsonpath_query = JsonPathQuery::parse("")?;
    let json_file = fs::File::open("./tests/documents/json/compressed/empty_object.json")?;
    let input = BufferedInput::new(json_file);
    let engine = RecursiveEngine::compile_query(&jsonpath_query)?;
    let result = engine.run::<_, CountResult>(&input)?;
    assert_eq!(result.get(), 1u64, "result != expected");
    Ok(())
}
#[test]
fn empty_object_root_compressed_with_query_select_the_root_empty_query_with_buffered_input_and_index_result_using_main_engine(
) -> Result<(), Box<dyn Error>> {
    println ! ("on document empty_object.toml running the query  (select the root (empty query)) with Input impl BufferedInput and result mode IndexResult");
    let jsonpath_query = JsonPathQuery::parse("")?;
    let json_file = fs::File::open("./tests/documents/json/compressed/empty_object.json")?;
    let input = BufferedInput::new(json_file);
    let engine = MainEngine::compile_query(&jsonpath_query)?;
    let result = engine.run::<_, IndexResult>(&input)?;
    assert_eq!(result.get(), vec![0usize,], "result != expected");
    Ok(())
}
#[test]
fn empty_object_root_compressed_with_query_select_the_root_empty_query_with_buffered_input_and_index_result_using_recursive_engine(
) -> Result<(), Box<dyn Error>> {
    println ! ("on document empty_object.toml running the query  (select the root (empty query)) with Input impl BufferedInput and result mode IndexResult");
    let jsonpath_query = JsonPathQuery::parse("")?;
    let json_file = fs::File::open("./tests/documents/json/compressed/empty_object.json")?;
    let input = BufferedInput::new(json_file);
    let engine = RecursiveEngine::compile_query(&jsonpath_query)?;
    let result = engine.run::<_, IndexResult>(&input)?;
    assert_eq!(result.get(), vec![0usize,], "result != expected");
    Ok(())
}
#[test]
fn empty_object_root_compressed_with_query_select_the_root_empty_query_with_mmap_input_and_count_result_using_main_engine(
) -> Result<(), Box<dyn Error>> {
    println ! ("on document empty_object.toml running the query  (select the root (empty query)) with Input impl MmapInput and result mode CountResult");
    let jsonpath_query = JsonPathQuery::parse("")?;
    let json_file = fs::File::open("./tests/documents/json/compressed/empty_object.json")?;
    let input = unsafe { MmapInput::map_file(&json_file)? };
    let engine = MainEngine::compile_query(&jsonpath_query)?;
    let result = engine.run::<_, CountResult>(&input)?;
    assert_eq!(result.get(), 1u64, "result != expected");
    Ok(())
}
#[test]
fn empty_object_root_compressed_with_query_select_the_root_empty_query_with_mmap_input_and_count_result_using_recursive_engine(
) -> Result<(), Box<dyn Error>> {
    println ! ("on document empty_object.toml running the query  (select the root (empty query)) with Input impl MmapInput and result mode CountResult");
    let jsonpath_query = JsonPathQuery::parse("")?;
    let json_file = fs::File::open("./tests/documents/json/compressed/empty_object.json")?;
    let input = unsafe { MmapInput::map_file(&json_file)? };
    let engine = RecursiveEngine::compile_query(&jsonpath_query)?;
    let result = engine.run::<_, CountResult>(&input)?;
    assert_eq!(result.get(), 1u64, "result != expected");
    Ok(())
}
#[test]
fn empty_object_root_compressed_with_query_select_the_root_empty_query_with_mmap_input_and_index_result_using_main_engine(
) -> Result<(), Box<dyn Error>> {
    println ! ("on document empty_object.toml running the query  (select the root (empty query)) with Input impl MmapInput and result mode IndexResult");
    let jsonpath_query = JsonPathQuery::parse("")?;
    let json_file = fs::File::open("./tests/documents/json/compressed/empty_object.json")?;
    let input = unsafe { MmapInput::map_file(&json_file)? };
    let engine = MainEngine::compile_query(&jsonpath_query)?;
    let result = engine.run::<_, IndexResult>(&input)?;
    assert_eq!(result.get(), vec![0usize,], "result != expected");
    Ok(())
}
#[test]
fn empty_object_root_compressed_with_query_select_the_root_empty_query_with_mmap_input_and_index_result_using_recursive_engine(
) -> Result<(), Box<dyn Error>> {
    println ! ("on document empty_object.toml running the query  (select the root (empty query)) with Input impl MmapInput and result mode IndexResult");
    let jsonpath_query = JsonPathQuery::parse("")?;
    let json_file = fs::File::open("./tests/documents/json/compressed/empty_object.json")?;
    let input = unsafe { MmapInput::map_file(&json_file)? };
    let engine = RecursiveEngine::compile_query(&jsonpath_query)?;
    let result = engine.run::<_, IndexResult>(&input)?;
    assert_eq!(result.get(), vec![0usize,], "result != expected");
    Ok(())
}
#[test]
fn empty_object_root_compressed_with_query_select_the_root_empty_query_with_owned_bytes_and_count_result_using_main_engine(
) -> Result<(), Box<dyn Error>> {
    println ! ("on document empty_object.toml running the query  (select the root (empty query)) with Input impl OwnedBytes and result mode CountResult");
    let jsonpath_query = JsonPathQuery::parse("")?;
    let raw_json = fs::read_to_string("./tests/documents/json/compressed/empty_object.json")?;
    let input = OwnedBytes::new(&raw_json.as_bytes())?;
    let engine = MainEngine::compile_query(&jsonpath_query)?;
    let result = engine.run::<_, CountResult>(&input)?;
    assert_eq!(result.get(), 1u64, "result != expected");
    Ok(())
}
#[test]
fn empty_object_root_compressed_with_query_select_the_root_empty_query_with_owned_bytes_and_count_result_using_recursive_engine(
) -> Result<(), Box<dyn Error>> {
    println ! ("on document empty_object.toml running the query  (select the root (empty query)) with Input impl OwnedBytes and result mode CountResult");
    let jsonpath_query = JsonPathQuery::parse("")?;
    let raw_json = fs::read_to_string("./tests/documents/json/compressed/empty_object.json")?;
    let input = OwnedBytes::new(&raw_json.as_bytes())?;
    let engine = RecursiveEngine::compile_query(&jsonpath_query)?;
    let result = engine.run::<_, CountResult>(&input)?;
    assert_eq!(result.get(), 1u64, "result != expected");
    Ok(())
}
#[test]
fn empty_object_root_compressed_with_query_select_the_root_empty_query_with_owned_bytes_and_index_result_using_main_engine(
) -> Result<(), Box<dyn Error>> {
    println ! ("on document empty_object.toml running the query  (select the root (empty query)) with Input impl OwnedBytes and result mode IndexResult");
    let jsonpath_query = JsonPathQuery::parse("")?;
    let raw_json = fs::read_to_string("./tests/documents/json/compressed/empty_object.json")?;
    let input = OwnedBytes::new(&raw_json.as_bytes())?;
    let engine = MainEngine::compile_query(&jsonpath_query)?;
    let result = engine.run::<_, IndexResult>(&input)?;
    assert_eq!(result.get(), vec![0usize,], "result != expected");
    Ok(())
}
#[test]
fn empty_object_root_compressed_with_query_select_the_root_empty_query_with_owned_bytes_and_index_result_using_recursive_engine(
) -> Result<(), Box<dyn Error>> {
    println ! ("on document empty_object.toml running the query  (select the root (empty query)) with Input impl OwnedBytes and result mode IndexResult");
    let jsonpath_query = JsonPathQuery::parse("")?;
    let raw_json = fs::read_to_string("./tests/documents/json/compressed/empty_object.json")?;
    let input = OwnedBytes::new(&raw_json.as_bytes())?;
    let engine = RecursiveEngine::compile_query(&jsonpath_query)?;
    let result = engine.run::<_, IndexResult>(&input)?;
    assert_eq!(result.get(), vec![0usize,], "result != expected");
    Ok(())
}
#[test]
fn empty_object_root_compressed_with_query_select_the_root_with_buffered_input_and_count_result_using_main_engine(
) -> Result<(), Box<dyn Error>> {
    println ! ("on document empty_object.toml running the query $ (select the root) with Input impl BufferedInput and result mode CountResult");
    let jsonpath_query = JsonPathQuery::parse("$")?;
    let json_file = fs::File::open("./tests/documents/json/compressed/empty_object.json")?;
    let input = BufferedInput::new(json_file);
    let engine = MainEngine::compile_query(&jsonpath_query)?;
    let result = engine.run::<_, CountResult>(&input)?;
    assert_eq!(result.get(), 1u64, "result != expected");
    Ok(())
}
#[test]
fn empty_object_root_compressed_with_query_select_the_root_with_buffered_input_and_count_result_using_recursive_engine(
) -> Result<(), Box<dyn Error>> {
    println ! ("on document empty_object.toml running the query $ (select the root) with Input impl BufferedInput and result mode CountResult");
    let jsonpath_query = JsonPathQuery::parse("$")?;
    let json_file = fs::File::open("./tests/documents/json/compressed/empty_object.json")?;
    let input = BufferedInput::new(json_file);
    let engine = RecursiveEngine::compile_query(&jsonpath_query)?;
    let result = engine.run::<_, CountResult>(&input)?;
    assert_eq!(result.get(), 1u64, "result != expected");
    Ok(())
}
#[test]
fn empty_object_root_compressed_with_query_select_the_root_with_buffered_input_and_index_result_using_main_engine(
) -> Result<(), Box<dyn Error>> {
    println ! ("on document empty_object.toml running the query $ (select the root) with Input impl BufferedInput and result mode IndexResult");
    let jsonpath_query = JsonPathQuery::parse("$")?;
    let json_file = fs::File::open("./tests/documents/json/compressed/empty_object.json")?;
    let input = BufferedInput::new(json_file);
    let engine = MainEngine::compile_query(&jsonpath_query)?;
    let result = engine.run::<_, IndexResult>(&input)?;
    assert_eq!(result.get(), vec![0usize,], "result != expected");
    Ok(())
}
#[test]
fn empty_object_root_compressed_with_query_select_the_root_with_buffered_input_and_index_result_using_recursive_engine(
) -> Result<(), Box<dyn Error>> {
    println ! ("on document empty_object.toml running the query $ (select the root) with Input impl BufferedInput and result mode IndexResult");
    let jsonpath_query = JsonPathQuery::parse("$")?;
    let json_file = fs::File::open("./tests/documents/json/compressed/empty_object.json")?;
    let input = BufferedInput::new(json_file);
    let engine = RecursiveEngine::compile_query(&jsonpath_query)?;
    let result = engine.run::<_, IndexResult>(&input)?;
    assert_eq!(result.get(), vec![0usize,], "result != expected");
    Ok(())
}
#[test]
fn empty_object_root_compressed_with_query_select_the_root_with_mmap_input_and_count_result_using_main_engine(
) -> Result<(), Box<dyn Error>> {
    println ! ("on document empty_object.toml running the query $ (select the root) with Input impl MmapInput and result mode CountResult");
    let jsonpath_query = JsonPathQuery::parse("$")?;
    let json_file = fs::File::open("./tests/documents/json/compressed/empty_object.json")?;
    let input = unsafe { MmapInput::map_file(&json_file)? };
    let engine = MainEngine::compile_query(&jsonpath_query)?;
    let result = engine.run::<_, CountResult>(&input)?;
    assert_eq!(result.get(), 1u64, "result != expected");
    Ok(())
}
#[test]
fn empty_object_root_compressed_with_query_select_the_root_with_mmap_input_and_count_result_using_recursive_engine(
) -> Result<(), Box<dyn Error>> {
    println ! ("on document empty_object.toml running the query $ (select the root) with Input impl MmapInput and result mode CountResult");
    let jsonpath_query = JsonPathQuery::parse("$")?;
    let json_file = fs::File::open("./tests/documents/json/compressed/empty_object.json")?;
    let input = unsafe { MmapInput::map_file(&json_file)? };
    let engine = RecursiveEngine::compile_query(&jsonpath_query)?;
    let result = engine.run::<_, CountResult>(&input)?;
    assert_eq!(result.get(), 1u64, "result != expected");
    Ok(())
}
#[test]
fn empty_object_root_compressed_with_query_select_the_root_with_mmap_input_and_index_result_using_main_engine(
) -> Result<(), Box<dyn Error>> {
    println ! ("on document empty_object.toml running the query $ (select the root) with Input impl MmapInput and result mode IndexResult");
    let jsonpath_query = JsonPathQuery::parse("$")?;
    let json_file = fs::File::open("./tests/documents/json/compressed/empty_object.json")?;
    let input = unsafe { MmapInput::map_file(&json_file)? };
    let engine = MainEngine::compile_query(&jsonpath_query)?;
    let result = engine.run::<_, IndexResult>(&input)?;
    assert_eq!(result.get(), vec![0usize,], "result != expected");
    Ok(())
}
#[test]
fn empty_object_root_compressed_with_query_select_the_root_with_mmap_input_and_index_result_using_recursive_engine(
) -> Result<(), Box<dyn Error>> {
    println ! ("on document empty_object.toml running the query $ (select the root) with Input impl MmapInput and result mode IndexResult");
    let jsonpath_query = JsonPathQuery::parse("$")?;
    let json_file = fs::File::open("./tests/documents/json/compressed/empty_object.json")?;
    let input = unsafe { MmapInput::map_file(&json_file)? };
    let engine = RecursiveEngine::compile_query(&jsonpath_query)?;
    let result = engine.run::<_, IndexResult>(&input)?;
    assert_eq!(result.get(), vec![0usize,], "result != expected");
    Ok(())
}
#[test]
fn empty_object_root_compressed_with_query_select_the_root_with_owned_bytes_and_count_result_using_main_engine(
) -> Result<(), Box<dyn Error>> {
    println ! ("on document empty_object.toml running the query $ (select the root) with Input impl OwnedBytes and result mode CountResult");
    let jsonpath_query = JsonPathQuery::parse("$")?;
    let raw_json = fs::read_to_string("./tests/documents/json/compressed/empty_object.json")?;
    let input = OwnedBytes::new(&raw_json.as_bytes())?;
    let engine = MainEngine::compile_query(&jsonpath_query)?;
    let result = engine.run::<_, CountResult>(&input)?;
    assert_eq!(result.get(), 1u64, "result != expected");
    Ok(())
}
#[test]
fn empty_object_root_compressed_with_query_select_the_root_with_owned_bytes_and_count_result_using_recursive_engine(
) -> Result<(), Box<dyn Error>> {
    println ! ("on document empty_object.toml running the query $ (select the root) with Input impl OwnedBytes and result mode CountResult");
    let jsonpath_query = JsonPathQuery::parse("$")?;
    let raw_json = fs::read_to_string("./tests/documents/json/compressed/empty_object.json")?;
    let input = OwnedBytes::new(&raw_json.as_bytes())?;
    let engine = RecursiveEngine::compile_query(&jsonpath_query)?;
    let result = engine.run::<_, CountResult>(&input)?;
    assert_eq!(result.get(), 1u64, "result != expected");
    Ok(())
}
#[test]
fn empty_object_root_compressed_with_query_select_the_root_with_owned_bytes_and_index_result_using_main_engine(
) -> Result<(), Box<dyn Error>> {
    println ! ("on document empty_object.toml running the query $ (select the root) with Input impl OwnedBytes and result mode IndexResult");
    let jsonpath_query = JsonPathQuery::parse("$")?;
    let raw_json = fs::read_to_string("./tests/documents/json/compressed/empty_object.json")?;
    let input = OwnedBytes::new(&raw_json.as_bytes())?;
    let engine = MainEngine::compile_query(&jsonpath_query)?;
    let result = engine.run::<_, IndexResult>(&input)?;
    assert_eq!(result.get(), vec![0usize,], "result != expected");
    Ok(())
}
#[test]
fn empty_object_root_compressed_with_query_select_the_root_with_owned_bytes_and_index_result_using_recursive_engine(
) -> Result<(), Box<dyn Error>> {
    println ! ("on document empty_object.toml running the query $ (select the root) with Input impl OwnedBytes and result mode IndexResult");
    let jsonpath_query = JsonPathQuery::parse("$")?;
    let raw_json = fs::read_to_string("./tests/documents/json/compressed/empty_object.json")?;
    let input = OwnedBytes::new(&raw_json.as_bytes())?;
    let engine = RecursiveEngine::compile_query(&jsonpath_query)?;
    let result = engine.run::<_, IndexResult>(&input)?;
    assert_eq!(result.get(), vec![0usize,], "result != expected");
    Ok(())
}
#[test]
fn empty_object_root_with_query_select_any_child_there_are_none_with_buffered_input_and_count_result_using_main_engine(
) -> Result<(), Box<dyn Error>> {
    println ! ("on document empty_object.toml running the query $.* (select any child (there are none)) with Input impl BufferedInput and result mode CountResult");
    let jsonpath_query = JsonPathQuery::parse("$.*")?;
    let json_file = fs::File::open("./tests/documents/json/empty_object.json")?;
    let input = BufferedInput::new(json_file);
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
    let json_file = fs::File::open("./tests/documents/json/empty_object.json")?;
    let input = BufferedInput::new(json_file);
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
    let json_file = fs::File::open("./tests/documents/json/empty_object.json")?;
    let input = BufferedInput::new(json_file);
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
    let json_file = fs::File::open("./tests/documents/json/empty_object.json")?;
    let input = BufferedInput::new(json_file);
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
    let json_file = fs::File::open("./tests/documents/json/empty_object.json")?;
    let input = unsafe { MmapInput::map_file(&json_file)? };
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
    let json_file = fs::File::open("./tests/documents/json/empty_object.json")?;
    let input = unsafe { MmapInput::map_file(&json_file)? };
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
    let json_file = fs::File::open("./tests/documents/json/empty_object.json")?;
    let input = unsafe { MmapInput::map_file(&json_file)? };
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
    let json_file = fs::File::open("./tests/documents/json/empty_object.json")?;
    let input = unsafe { MmapInput::map_file(&json_file)? };
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
    let raw_json = fs::read_to_string("./tests/documents/json/empty_object.json")?;
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
    let raw_json = fs::read_to_string("./tests/documents/json/empty_object.json")?;
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
    let raw_json = fs::read_to_string("./tests/documents/json/empty_object.json")?;
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
    let raw_json = fs::read_to_string("./tests/documents/json/empty_object.json")?;
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
    let json_file = fs::File::open("./tests/documents/json/empty_object.json")?;
    let input = BufferedInput::new(json_file);
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
    let json_file = fs::File::open("./tests/documents/json/empty_object.json")?;
    let input = BufferedInput::new(json_file);
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
    let json_file = fs::File::open("./tests/documents/json/empty_object.json")?;
    let input = BufferedInput::new(json_file);
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
    let json_file = fs::File::open("./tests/documents/json/empty_object.json")?;
    let input = BufferedInput::new(json_file);
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
    let json_file = fs::File::open("./tests/documents/json/empty_object.json")?;
    let input = unsafe { MmapInput::map_file(&json_file)? };
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
    let json_file = fs::File::open("./tests/documents/json/empty_object.json")?;
    let input = unsafe { MmapInput::map_file(&json_file)? };
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
    let json_file = fs::File::open("./tests/documents/json/empty_object.json")?;
    let input = unsafe { MmapInput::map_file(&json_file)? };
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
    let json_file = fs::File::open("./tests/documents/json/empty_object.json")?;
    let input = unsafe { MmapInput::map_file(&json_file)? };
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
    let raw_json = fs::read_to_string("./tests/documents/json/empty_object.json")?;
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
    let raw_json = fs::read_to_string("./tests/documents/json/empty_object.json")?;
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
    let raw_json = fs::read_to_string("./tests/documents/json/empty_object.json")?;
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
    let raw_json = fs::read_to_string("./tests/documents/json/empty_object.json")?;
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
    let json_file = fs::File::open("./tests/documents/json/empty_object.json")?;
    let input = BufferedInput::new(json_file);
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
    let json_file = fs::File::open("./tests/documents/json/empty_object.json")?;
    let input = BufferedInput::new(json_file);
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
    let json_file = fs::File::open("./tests/documents/json/empty_object.json")?;
    let input = BufferedInput::new(json_file);
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
    let json_file = fs::File::open("./tests/documents/json/empty_object.json")?;
    let input = BufferedInput::new(json_file);
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
    let json_file = fs::File::open("./tests/documents/json/empty_object.json")?;
    let input = unsafe { MmapInput::map_file(&json_file)? };
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
    let json_file = fs::File::open("./tests/documents/json/empty_object.json")?;
    let input = unsafe { MmapInput::map_file(&json_file)? };
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
    let json_file = fs::File::open("./tests/documents/json/empty_object.json")?;
    let input = unsafe { MmapInput::map_file(&json_file)? };
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
    let json_file = fs::File::open("./tests/documents/json/empty_object.json")?;
    let input = unsafe { MmapInput::map_file(&json_file)? };
    let engine = RecursiveEngine::compile_query(&jsonpath_query)?;
    let result = engine.run::<_, IndexResult>(&input)?;
    assert_eq!(result.get(), vec![], "result != expected");
    Ok(())
}
#[test]
fn empty_object_root_with_query_select_the_child_named_a_which_does_not_exist_with_owned_bytes_and_count_result_using_main_engine(
) -> Result<(), Box<dyn Error>> {
    println ! ("on document empty_object.toml running the query $.a (select the child named 'a' (which does not exist)) with Input impl OwnedBytes and result mode CountResult");
    let jsonpath_query = JsonPathQuery::parse("$.a")?;
    let raw_json = fs::read_to_string("./tests/documents/json/empty_object.json")?;
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
    let raw_json = fs::read_to_string("./tests/documents/json/empty_object.json")?;
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
    let raw_json = fs::read_to_string("./tests/documents/json/empty_object.json")?;
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
    let raw_json = fs::read_to_string("./tests/documents/json/empty_object.json")?;
    let input = OwnedBytes::new(&raw_json.as_bytes())?;
    let engine = RecursiveEngine::compile_query(&jsonpath_query)?;
    let result = engine.run::<_, IndexResult>(&input)?;
    assert_eq!(result.get(), vec![], "result != expected");
    Ok(())
}
#[test]
fn empty_object_root_with_query_select_the_root_empty_query_with_buffered_input_and_count_result_using_main_engine(
) -> Result<(), Box<dyn Error>> {
    println ! ("on document empty_object.toml running the query  (select the root (empty query)) with Input impl BufferedInput and result mode CountResult");
    let jsonpath_query = JsonPathQuery::parse("")?;
    let json_file = fs::File::open("./tests/documents/json/empty_object.json")?;
    let input = BufferedInput::new(json_file);
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
    let json_file = fs::File::open("./tests/documents/json/empty_object.json")?;
    let input = BufferedInput::new(json_file);
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
    let json_file = fs::File::open("./tests/documents/json/empty_object.json")?;
    let input = BufferedInput::new(json_file);
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
    let json_file = fs::File::open("./tests/documents/json/empty_object.json")?;
    let input = BufferedInput::new(json_file);
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
    let json_file = fs::File::open("./tests/documents/json/empty_object.json")?;
    let input = unsafe { MmapInput::map_file(&json_file)? };
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
    let json_file = fs::File::open("./tests/documents/json/empty_object.json")?;
    let input = unsafe { MmapInput::map_file(&json_file)? };
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
    let json_file = fs::File::open("./tests/documents/json/empty_object.json")?;
    let input = unsafe { MmapInput::map_file(&json_file)? };
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
    let json_file = fs::File::open("./tests/documents/json/empty_object.json")?;
    let input = unsafe { MmapInput::map_file(&json_file)? };
    let engine = RecursiveEngine::compile_query(&jsonpath_query)?;
    let result = engine.run::<_, IndexResult>(&input)?;
    assert_eq!(result.get(), vec![0usize,], "result != expected");
    Ok(())
}
#[test]
fn empty_object_root_with_query_select_the_root_empty_query_with_owned_bytes_and_count_result_using_main_engine(
) -> Result<(), Box<dyn Error>> {
    println ! ("on document empty_object.toml running the query  (select the root (empty query)) with Input impl OwnedBytes and result mode CountResult");
    let jsonpath_query = JsonPathQuery::parse("")?;
    let raw_json = fs::read_to_string("./tests/documents/json/empty_object.json")?;
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
    let raw_json = fs::read_to_string("./tests/documents/json/empty_object.json")?;
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
    let raw_json = fs::read_to_string("./tests/documents/json/empty_object.json")?;
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
    let raw_json = fs::read_to_string("./tests/documents/json/empty_object.json")?;
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
    let json_file = fs::File::open("./tests/documents/json/empty_object.json")?;
    let input = BufferedInput::new(json_file);
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
    let json_file = fs::File::open("./tests/documents/json/empty_object.json")?;
    let input = BufferedInput::new(json_file);
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
    let json_file = fs::File::open("./tests/documents/json/empty_object.json")?;
    let input = BufferedInput::new(json_file);
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
    let json_file = fs::File::open("./tests/documents/json/empty_object.json")?;
    let input = BufferedInput::new(json_file);
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
    let json_file = fs::File::open("./tests/documents/json/empty_object.json")?;
    let input = unsafe { MmapInput::map_file(&json_file)? };
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
    let json_file = fs::File::open("./tests/documents/json/empty_object.json")?;
    let input = unsafe { MmapInput::map_file(&json_file)? };
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
    let json_file = fs::File::open("./tests/documents/json/empty_object.json")?;
    let input = unsafe { MmapInput::map_file(&json_file)? };
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
    let json_file = fs::File::open("./tests/documents/json/empty_object.json")?;
    let input = unsafe { MmapInput::map_file(&json_file)? };
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
    let raw_json = fs::read_to_string("./tests/documents/json/empty_object.json")?;
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
    let raw_json = fs::read_to_string("./tests/documents/json/empty_object.json")?;
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
    let raw_json = fs::read_to_string("./tests/documents/json/empty_object.json")?;
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
    let raw_json = fs::read_to_string("./tests/documents/json/empty_object.json")?;
    let input = OwnedBytes::new(&raw_json.as_bytes())?;
    let engine = RecursiveEngine::compile_query(&jsonpath_query)?;
    let result = engine.run::<_, IndexResult>(&input)?;
    assert_eq!(result.get(), vec![0usize,], "result != expected");
    Ok(())
}
#[test]
fn extract_from_twitter_json_containing_urls_with_multiple_escaped_slashes_compressed_with_query_descendant_entities_then_url_with_buffered_input_and_count_result_using_main_engine(
) -> Result<(), Box<dyn Error>> {
    println ! ("on document twitter_urls.toml running the query $..entities..url (descendant entities then url) with Input impl BufferedInput and result mode CountResult");
    let jsonpath_query = JsonPathQuery::parse("$..entities..url")?;
    let json_file = fs::File::open("./tests/documents/json/compressed/twitter_urls.json")?;
    let input = BufferedInput::new(json_file);
    let engine = MainEngine::compile_query(&jsonpath_query)?;
    let result = engine.run::<_, CountResult>(&input)?;
    assert_eq!(result.get(), 4u64, "result != expected");
    Ok(())
}
#[test]
fn extract_from_twitter_json_containing_urls_with_multiple_escaped_slashes_compressed_with_query_descendant_entities_then_url_with_buffered_input_and_count_result_using_recursive_engine(
) -> Result<(), Box<dyn Error>> {
    println ! ("on document twitter_urls.toml running the query $..entities..url (descendant entities then url) with Input impl BufferedInput and result mode CountResult");
    let jsonpath_query = JsonPathQuery::parse("$..entities..url")?;
    let json_file = fs::File::open("./tests/documents/json/compressed/twitter_urls.json")?;
    let input = BufferedInput::new(json_file);
    let engine = RecursiveEngine::compile_query(&jsonpath_query)?;
    let result = engine.run::<_, CountResult>(&input)?;
    assert_eq!(result.get(), 4u64, "result != expected");
    Ok(())
}
#[test]
fn extract_from_twitter_json_containing_urls_with_multiple_escaped_slashes_compressed_with_query_descendant_entities_then_url_with_buffered_input_and_index_result_using_main_engine(
) -> Result<(), Box<dyn Error>> {
    println ! ("on document twitter_urls.toml running the query $..entities..url (descendant entities then url) with Input impl BufferedInput and result mode IndexResult");
    let jsonpath_query = JsonPathQuery::parse("$..entities..url")?;
    let json_file = fs::File::open("./tests/documents/json/compressed/twitter_urls.json")?;
    let input = BufferedInput::new(json_file);
    let engine = MainEngine::compile_query(&jsonpath_query)?;
    let result = engine.run::<_, IndexResult>(&input)?;
    assert_eq!(
        result.get(),
        vec![151usize, 198usize, 341usize, 388usize,],
        "result != expected"
    );
    Ok(())
}
#[test]
fn extract_from_twitter_json_containing_urls_with_multiple_escaped_slashes_compressed_with_query_descendant_entities_then_url_with_buffered_input_and_index_result_using_recursive_engine(
) -> Result<(), Box<dyn Error>> {
    println ! ("on document twitter_urls.toml running the query $..entities..url (descendant entities then url) with Input impl BufferedInput and result mode IndexResult");
    let jsonpath_query = JsonPathQuery::parse("$..entities..url")?;
    let json_file = fs::File::open("./tests/documents/json/compressed/twitter_urls.json")?;
    let input = BufferedInput::new(json_file);
    let engine = RecursiveEngine::compile_query(&jsonpath_query)?;
    let result = engine.run::<_, IndexResult>(&input)?;
    assert_eq!(
        result.get(),
        vec![151usize, 198usize, 341usize, 388usize,],
        "result != expected"
    );
    Ok(())
}
#[test]
fn extract_from_twitter_json_containing_urls_with_multiple_escaped_slashes_compressed_with_query_descendant_entities_then_url_with_mmap_input_and_count_result_using_main_engine(
) -> Result<(), Box<dyn Error>> {
    println ! ("on document twitter_urls.toml running the query $..entities..url (descendant entities then url) with Input impl MmapInput and result mode CountResult");
    let jsonpath_query = JsonPathQuery::parse("$..entities..url")?;
    let json_file = fs::File::open("./tests/documents/json/compressed/twitter_urls.json")?;
    let input = unsafe { MmapInput::map_file(&json_file)? };
    let engine = MainEngine::compile_query(&jsonpath_query)?;
    let result = engine.run::<_, CountResult>(&input)?;
    assert_eq!(result.get(), 4u64, "result != expected");
    Ok(())
}
#[test]
fn extract_from_twitter_json_containing_urls_with_multiple_escaped_slashes_compressed_with_query_descendant_entities_then_url_with_mmap_input_and_count_result_using_recursive_engine(
) -> Result<(), Box<dyn Error>> {
    println ! ("on document twitter_urls.toml running the query $..entities..url (descendant entities then url) with Input impl MmapInput and result mode CountResult");
    let jsonpath_query = JsonPathQuery::parse("$..entities..url")?;
    let json_file = fs::File::open("./tests/documents/json/compressed/twitter_urls.json")?;
    let input = unsafe { MmapInput::map_file(&json_file)? };
    let engine = RecursiveEngine::compile_query(&jsonpath_query)?;
    let result = engine.run::<_, CountResult>(&input)?;
    assert_eq!(result.get(), 4u64, "result != expected");
    Ok(())
}
#[test]
fn extract_from_twitter_json_containing_urls_with_multiple_escaped_slashes_compressed_with_query_descendant_entities_then_url_with_mmap_input_and_index_result_using_main_engine(
) -> Result<(), Box<dyn Error>> {
    println ! ("on document twitter_urls.toml running the query $..entities..url (descendant entities then url) with Input impl MmapInput and result mode IndexResult");
    let jsonpath_query = JsonPathQuery::parse("$..entities..url")?;
    let json_file = fs::File::open("./tests/documents/json/compressed/twitter_urls.json")?;
    let input = unsafe { MmapInput::map_file(&json_file)? };
    let engine = MainEngine::compile_query(&jsonpath_query)?;
    let result = engine.run::<_, IndexResult>(&input)?;
    assert_eq!(
        result.get(),
        vec![151usize, 198usize, 341usize, 388usize,],
        "result != expected"
    );
    Ok(())
}
#[test]
fn extract_from_twitter_json_containing_urls_with_multiple_escaped_slashes_compressed_with_query_descendant_entities_then_url_with_mmap_input_and_index_result_using_recursive_engine(
) -> Result<(), Box<dyn Error>> {
    println ! ("on document twitter_urls.toml running the query $..entities..url (descendant entities then url) with Input impl MmapInput and result mode IndexResult");
    let jsonpath_query = JsonPathQuery::parse("$..entities..url")?;
    let json_file = fs::File::open("./tests/documents/json/compressed/twitter_urls.json")?;
    let input = unsafe { MmapInput::map_file(&json_file)? };
    let engine = RecursiveEngine::compile_query(&jsonpath_query)?;
    let result = engine.run::<_, IndexResult>(&input)?;
    assert_eq!(
        result.get(),
        vec![151usize, 198usize, 341usize, 388usize,],
        "result != expected"
    );
    Ok(())
}
#[test]
fn extract_from_twitter_json_containing_urls_with_multiple_escaped_slashes_compressed_with_query_descendant_entities_then_url_with_owned_bytes_and_count_result_using_main_engine(
) -> Result<(), Box<dyn Error>> {
    println ! ("on document twitter_urls.toml running the query $..entities..url (descendant entities then url) with Input impl OwnedBytes and result mode CountResult");
    let jsonpath_query = JsonPathQuery::parse("$..entities..url")?;
    let raw_json = fs::read_to_string("./tests/documents/json/compressed/twitter_urls.json")?;
    let input = OwnedBytes::new(&raw_json.as_bytes())?;
    let engine = MainEngine::compile_query(&jsonpath_query)?;
    let result = engine.run::<_, CountResult>(&input)?;
    assert_eq!(result.get(), 4u64, "result != expected");
    Ok(())
}
#[test]
fn extract_from_twitter_json_containing_urls_with_multiple_escaped_slashes_compressed_with_query_descendant_entities_then_url_with_owned_bytes_and_count_result_using_recursive_engine(
) -> Result<(), Box<dyn Error>> {
    println ! ("on document twitter_urls.toml running the query $..entities..url (descendant entities then url) with Input impl OwnedBytes and result mode CountResult");
    let jsonpath_query = JsonPathQuery::parse("$..entities..url")?;
    let raw_json = fs::read_to_string("./tests/documents/json/compressed/twitter_urls.json")?;
    let input = OwnedBytes::new(&raw_json.as_bytes())?;
    let engine = RecursiveEngine::compile_query(&jsonpath_query)?;
    let result = engine.run::<_, CountResult>(&input)?;
    assert_eq!(result.get(), 4u64, "result != expected");
    Ok(())
}
#[test]
fn extract_from_twitter_json_containing_urls_with_multiple_escaped_slashes_compressed_with_query_descendant_entities_then_url_with_owned_bytes_and_index_result_using_main_engine(
) -> Result<(), Box<dyn Error>> {
    println ! ("on document twitter_urls.toml running the query $..entities..url (descendant entities then url) with Input impl OwnedBytes and result mode IndexResult");
    let jsonpath_query = JsonPathQuery::parse("$..entities..url")?;
    let raw_json = fs::read_to_string("./tests/documents/json/compressed/twitter_urls.json")?;
    let input = OwnedBytes::new(&raw_json.as_bytes())?;
    let engine = MainEngine::compile_query(&jsonpath_query)?;
    let result = engine.run::<_, IndexResult>(&input)?;
    assert_eq!(
        result.get(),
        vec![151usize, 198usize, 341usize, 388usize,],
        "result != expected"
    );
    Ok(())
}
#[test]
fn extract_from_twitter_json_containing_urls_with_multiple_escaped_slashes_compressed_with_query_descendant_entities_then_url_with_owned_bytes_and_index_result_using_recursive_engine(
) -> Result<(), Box<dyn Error>> {
    println ! ("on document twitter_urls.toml running the query $..entities..url (descendant entities then url) with Input impl OwnedBytes and result mode IndexResult");
    let jsonpath_query = JsonPathQuery::parse("$..entities..url")?;
    let raw_json = fs::read_to_string("./tests/documents/json/compressed/twitter_urls.json")?;
    let input = OwnedBytes::new(&raw_json.as_bytes())?;
    let engine = RecursiveEngine::compile_query(&jsonpath_query)?;
    let result = engine.run::<_, IndexResult>(&input)?;
    assert_eq!(
        result.get(),
        vec![151usize, 198usize, 341usize, 388usize,],
        "result != expected"
    );
    Ok(())
}
#[test]
fn extract_from_twitter_json_containing_urls_with_multiple_escaped_slashes_compressed_with_query_descendant_for_url_limited_to_direct_urls_arrays_with_buffered_input_and_count_result_using_main_engine(
) -> Result<(), Box<dyn Error>> {
    println ! ("on document twitter_urls.toml running the query $..entities.urls..url (descendant for url limited to direct urls arrays) with Input impl BufferedInput and result mode CountResult");
    let jsonpath_query = JsonPathQuery::parse("$..entities.urls..url")?;
    let json_file = fs::File::open("./tests/documents/json/compressed/twitter_urls.json")?;
    let input = BufferedInput::new(json_file);
    let engine = MainEngine::compile_query(&jsonpath_query)?;
    let result = engine.run::<_, CountResult>(&input)?;
    assert_eq!(result.get(), 2u64, "result != expected");
    Ok(())
}
#[test]
fn extract_from_twitter_json_containing_urls_with_multiple_escaped_slashes_compressed_with_query_descendant_for_url_limited_to_direct_urls_arrays_with_buffered_input_and_count_result_using_recursive_engine(
) -> Result<(), Box<dyn Error>> {
    println ! ("on document twitter_urls.toml running the query $..entities.urls..url (descendant for url limited to direct urls arrays) with Input impl BufferedInput and result mode CountResult");
    let jsonpath_query = JsonPathQuery::parse("$..entities.urls..url")?;
    let json_file = fs::File::open("./tests/documents/json/compressed/twitter_urls.json")?;
    let input = BufferedInput::new(json_file);
    let engine = RecursiveEngine::compile_query(&jsonpath_query)?;
    let result = engine.run::<_, CountResult>(&input)?;
    assert_eq!(result.get(), 2u64, "result != expected");
    Ok(())
}
#[test]
fn extract_from_twitter_json_containing_urls_with_multiple_escaped_slashes_compressed_with_query_descendant_for_url_limited_to_direct_urls_arrays_with_buffered_input_and_index_result_using_main_engine(
) -> Result<(), Box<dyn Error>> {
    println ! ("on document twitter_urls.toml running the query $..entities.urls..url (descendant for url limited to direct urls arrays) with Input impl BufferedInput and result mode IndexResult");
    let jsonpath_query = JsonPathQuery::parse("$..entities.urls..url")?;
    let json_file = fs::File::open("./tests/documents/json/compressed/twitter_urls.json")?;
    let input = BufferedInput::new(json_file);
    let engine = MainEngine::compile_query(&jsonpath_query)?;
    let result = engine.run::<_, IndexResult>(&input)?;
    assert_eq!(result.get(), vec![151usize, 341usize,], "result != expected");
    Ok(())
}
#[test]
fn extract_from_twitter_json_containing_urls_with_multiple_escaped_slashes_compressed_with_query_descendant_for_url_limited_to_direct_urls_arrays_with_buffered_input_and_index_result_using_recursive_engine(
) -> Result<(), Box<dyn Error>> {
    println ! ("on document twitter_urls.toml running the query $..entities.urls..url (descendant for url limited to direct urls arrays) with Input impl BufferedInput and result mode IndexResult");
    let jsonpath_query = JsonPathQuery::parse("$..entities.urls..url")?;
    let json_file = fs::File::open("./tests/documents/json/compressed/twitter_urls.json")?;
    let input = BufferedInput::new(json_file);
    let engine = RecursiveEngine::compile_query(&jsonpath_query)?;
    let result = engine.run::<_, IndexResult>(&input)?;
    assert_eq!(result.get(), vec![151usize, 341usize,], "result != expected");
    Ok(())
}
#[test]
fn extract_from_twitter_json_containing_urls_with_multiple_escaped_slashes_compressed_with_query_descendant_for_url_limited_to_direct_urls_arrays_with_mmap_input_and_count_result_using_main_engine(
) -> Result<(), Box<dyn Error>> {
    println ! ("on document twitter_urls.toml running the query $..entities.urls..url (descendant for url limited to direct urls arrays) with Input impl MmapInput and result mode CountResult");
    let jsonpath_query = JsonPathQuery::parse("$..entities.urls..url")?;
    let json_file = fs::File::open("./tests/documents/json/compressed/twitter_urls.json")?;
    let input = unsafe { MmapInput::map_file(&json_file)? };
    let engine = MainEngine::compile_query(&jsonpath_query)?;
    let result = engine.run::<_, CountResult>(&input)?;
    assert_eq!(result.get(), 2u64, "result != expected");
    Ok(())
}
#[test]
fn extract_from_twitter_json_containing_urls_with_multiple_escaped_slashes_compressed_with_query_descendant_for_url_limited_to_direct_urls_arrays_with_mmap_input_and_count_result_using_recursive_engine(
) -> Result<(), Box<dyn Error>> {
    println ! ("on document twitter_urls.toml running the query $..entities.urls..url (descendant for url limited to direct urls arrays) with Input impl MmapInput and result mode CountResult");
    let jsonpath_query = JsonPathQuery::parse("$..entities.urls..url")?;
    let json_file = fs::File::open("./tests/documents/json/compressed/twitter_urls.json")?;
    let input = unsafe { MmapInput::map_file(&json_file)? };
    let engine = RecursiveEngine::compile_query(&jsonpath_query)?;
    let result = engine.run::<_, CountResult>(&input)?;
    assert_eq!(result.get(), 2u64, "result != expected");
    Ok(())
}
#[test]
fn extract_from_twitter_json_containing_urls_with_multiple_escaped_slashes_compressed_with_query_descendant_for_url_limited_to_direct_urls_arrays_with_mmap_input_and_index_result_using_main_engine(
) -> Result<(), Box<dyn Error>> {
    println ! ("on document twitter_urls.toml running the query $..entities.urls..url (descendant for url limited to direct urls arrays) with Input impl MmapInput and result mode IndexResult");
    let jsonpath_query = JsonPathQuery::parse("$..entities.urls..url")?;
    let json_file = fs::File::open("./tests/documents/json/compressed/twitter_urls.json")?;
    let input = unsafe { MmapInput::map_file(&json_file)? };
    let engine = MainEngine::compile_query(&jsonpath_query)?;
    let result = engine.run::<_, IndexResult>(&input)?;
    assert_eq!(result.get(), vec![151usize, 341usize,], "result != expected");
    Ok(())
}
#[test]
fn extract_from_twitter_json_containing_urls_with_multiple_escaped_slashes_compressed_with_query_descendant_for_url_limited_to_direct_urls_arrays_with_mmap_input_and_index_result_using_recursive_engine(
) -> Result<(), Box<dyn Error>> {
    println ! ("on document twitter_urls.toml running the query $..entities.urls..url (descendant for url limited to direct urls arrays) with Input impl MmapInput and result mode IndexResult");
    let jsonpath_query = JsonPathQuery::parse("$..entities.urls..url")?;
    let json_file = fs::File::open("./tests/documents/json/compressed/twitter_urls.json")?;
    let input = unsafe { MmapInput::map_file(&json_file)? };
    let engine = RecursiveEngine::compile_query(&jsonpath_query)?;
    let result = engine.run::<_, IndexResult>(&input)?;
    assert_eq!(result.get(), vec![151usize, 341usize,], "result != expected");
    Ok(())
}
#[test]
fn extract_from_twitter_json_containing_urls_with_multiple_escaped_slashes_compressed_with_query_descendant_for_url_limited_to_direct_urls_arrays_with_owned_bytes_and_count_result_using_main_engine(
) -> Result<(), Box<dyn Error>> {
    println ! ("on document twitter_urls.toml running the query $..entities.urls..url (descendant for url limited to direct urls arrays) with Input impl OwnedBytes and result mode CountResult");
    let jsonpath_query = JsonPathQuery::parse("$..entities.urls..url")?;
    let raw_json = fs::read_to_string("./tests/documents/json/compressed/twitter_urls.json")?;
    let input = OwnedBytes::new(&raw_json.as_bytes())?;
    let engine = MainEngine::compile_query(&jsonpath_query)?;
    let result = engine.run::<_, CountResult>(&input)?;
    assert_eq!(result.get(), 2u64, "result != expected");
    Ok(())
}
#[test]
fn extract_from_twitter_json_containing_urls_with_multiple_escaped_slashes_compressed_with_query_descendant_for_url_limited_to_direct_urls_arrays_with_owned_bytes_and_count_result_using_recursive_engine(
) -> Result<(), Box<dyn Error>> {
    println ! ("on document twitter_urls.toml running the query $..entities.urls..url (descendant for url limited to direct urls arrays) with Input impl OwnedBytes and result mode CountResult");
    let jsonpath_query = JsonPathQuery::parse("$..entities.urls..url")?;
    let raw_json = fs::read_to_string("./tests/documents/json/compressed/twitter_urls.json")?;
    let input = OwnedBytes::new(&raw_json.as_bytes())?;
    let engine = RecursiveEngine::compile_query(&jsonpath_query)?;
    let result = engine.run::<_, CountResult>(&input)?;
    assert_eq!(result.get(), 2u64, "result != expected");
    Ok(())
}
#[test]
fn extract_from_twitter_json_containing_urls_with_multiple_escaped_slashes_compressed_with_query_descendant_for_url_limited_to_direct_urls_arrays_with_owned_bytes_and_index_result_using_main_engine(
) -> Result<(), Box<dyn Error>> {
    println ! ("on document twitter_urls.toml running the query $..entities.urls..url (descendant for url limited to direct urls arrays) with Input impl OwnedBytes and result mode IndexResult");
    let jsonpath_query = JsonPathQuery::parse("$..entities.urls..url")?;
    let raw_json = fs::read_to_string("./tests/documents/json/compressed/twitter_urls.json")?;
    let input = OwnedBytes::new(&raw_json.as_bytes())?;
    let engine = MainEngine::compile_query(&jsonpath_query)?;
    let result = engine.run::<_, IndexResult>(&input)?;
    assert_eq!(result.get(), vec![151usize, 341usize,], "result != expected");
    Ok(())
}
#[test]
fn extract_from_twitter_json_containing_urls_with_multiple_escaped_slashes_compressed_with_query_descendant_for_url_limited_to_direct_urls_arrays_with_owned_bytes_and_index_result_using_recursive_engine(
) -> Result<(), Box<dyn Error>> {
    println ! ("on document twitter_urls.toml running the query $..entities.urls..url (descendant for url limited to direct urls arrays) with Input impl OwnedBytes and result mode IndexResult");
    let jsonpath_query = JsonPathQuery::parse("$..entities.urls..url")?;
    let raw_json = fs::read_to_string("./tests/documents/json/compressed/twitter_urls.json")?;
    let input = OwnedBytes::new(&raw_json.as_bytes())?;
    let engine = RecursiveEngine::compile_query(&jsonpath_query)?;
    let result = engine.run::<_, IndexResult>(&input)?;
    assert_eq!(result.get(), vec![151usize, 341usize,], "result != expected");
    Ok(())
}
#[test]
fn extract_from_twitter_json_containing_urls_with_multiple_escaped_slashes_compressed_with_query_descendant_for_url_limited_to_urls_arrays_with_buffered_input_and_count_result_using_main_engine(
) -> Result<(), Box<dyn Error>> {
    println ! ("on document twitter_urls.toml running the query $..entities..urls..url (descendant for url limited to urls arrays) with Input impl BufferedInput and result mode CountResult");
    let jsonpath_query = JsonPathQuery::parse("$..entities..urls..url")?;
    let json_file = fs::File::open("./tests/documents/json/compressed/twitter_urls.json")?;
    let input = BufferedInput::new(json_file);
    let engine = MainEngine::compile_query(&jsonpath_query)?;
    let result = engine.run::<_, CountResult>(&input)?;
    assert_eq!(result.get(), 2u64, "result != expected");
    Ok(())
}
#[test]
fn extract_from_twitter_json_containing_urls_with_multiple_escaped_slashes_compressed_with_query_descendant_for_url_limited_to_urls_arrays_with_buffered_input_and_count_result_using_recursive_engine(
) -> Result<(), Box<dyn Error>> {
    println ! ("on document twitter_urls.toml running the query $..entities..urls..url (descendant for url limited to urls arrays) with Input impl BufferedInput and result mode CountResult");
    let jsonpath_query = JsonPathQuery::parse("$..entities..urls..url")?;
    let json_file = fs::File::open("./tests/documents/json/compressed/twitter_urls.json")?;
    let input = BufferedInput::new(json_file);
    let engine = RecursiveEngine::compile_query(&jsonpath_query)?;
    let result = engine.run::<_, CountResult>(&input)?;
    assert_eq!(result.get(), 2u64, "result != expected");
    Ok(())
}
#[test]
fn extract_from_twitter_json_containing_urls_with_multiple_escaped_slashes_compressed_with_query_descendant_for_url_limited_to_urls_arrays_with_buffered_input_and_index_result_using_main_engine(
) -> Result<(), Box<dyn Error>> {
    println ! ("on document twitter_urls.toml running the query $..entities..urls..url (descendant for url limited to urls arrays) with Input impl BufferedInput and result mode IndexResult");
    let jsonpath_query = JsonPathQuery::parse("$..entities..urls..url")?;
    let json_file = fs::File::open("./tests/documents/json/compressed/twitter_urls.json")?;
    let input = BufferedInput::new(json_file);
    let engine = MainEngine::compile_query(&jsonpath_query)?;
    let result = engine.run::<_, IndexResult>(&input)?;
    assert_eq!(result.get(), vec![151usize, 341usize,], "result != expected");
    Ok(())
}
#[test]
fn extract_from_twitter_json_containing_urls_with_multiple_escaped_slashes_compressed_with_query_descendant_for_url_limited_to_urls_arrays_with_buffered_input_and_index_result_using_recursive_engine(
) -> Result<(), Box<dyn Error>> {
    println ! ("on document twitter_urls.toml running the query $..entities..urls..url (descendant for url limited to urls arrays) with Input impl BufferedInput and result mode IndexResult");
    let jsonpath_query = JsonPathQuery::parse("$..entities..urls..url")?;
    let json_file = fs::File::open("./tests/documents/json/compressed/twitter_urls.json")?;
    let input = BufferedInput::new(json_file);
    let engine = RecursiveEngine::compile_query(&jsonpath_query)?;
    let result = engine.run::<_, IndexResult>(&input)?;
    assert_eq!(result.get(), vec![151usize, 341usize,], "result != expected");
    Ok(())
}
#[test]
fn extract_from_twitter_json_containing_urls_with_multiple_escaped_slashes_compressed_with_query_descendant_for_url_limited_to_urls_arrays_with_mmap_input_and_count_result_using_main_engine(
) -> Result<(), Box<dyn Error>> {
    println ! ("on document twitter_urls.toml running the query $..entities..urls..url (descendant for url limited to urls arrays) with Input impl MmapInput and result mode CountResult");
    let jsonpath_query = JsonPathQuery::parse("$..entities..urls..url")?;
    let json_file = fs::File::open("./tests/documents/json/compressed/twitter_urls.json")?;
    let input = unsafe { MmapInput::map_file(&json_file)? };
    let engine = MainEngine::compile_query(&jsonpath_query)?;
    let result = engine.run::<_, CountResult>(&input)?;
    assert_eq!(result.get(), 2u64, "result != expected");
    Ok(())
}
#[test]
fn extract_from_twitter_json_containing_urls_with_multiple_escaped_slashes_compressed_with_query_descendant_for_url_limited_to_urls_arrays_with_mmap_input_and_count_result_using_recursive_engine(
) -> Result<(), Box<dyn Error>> {
    println ! ("on document twitter_urls.toml running the query $..entities..urls..url (descendant for url limited to urls arrays) with Input impl MmapInput and result mode CountResult");
    let jsonpath_query = JsonPathQuery::parse("$..entities..urls..url")?;
    let json_file = fs::File::open("./tests/documents/json/compressed/twitter_urls.json")?;
    let input = unsafe { MmapInput::map_file(&json_file)? };
    let engine = RecursiveEngine::compile_query(&jsonpath_query)?;
    let result = engine.run::<_, CountResult>(&input)?;
    assert_eq!(result.get(), 2u64, "result != expected");
    Ok(())
}
#[test]
fn extract_from_twitter_json_containing_urls_with_multiple_escaped_slashes_compressed_with_query_descendant_for_url_limited_to_urls_arrays_with_mmap_input_and_index_result_using_main_engine(
) -> Result<(), Box<dyn Error>> {
    println ! ("on document twitter_urls.toml running the query $..entities..urls..url (descendant for url limited to urls arrays) with Input impl MmapInput and result mode IndexResult");
    let jsonpath_query = JsonPathQuery::parse("$..entities..urls..url")?;
    let json_file = fs::File::open("./tests/documents/json/compressed/twitter_urls.json")?;
    let input = unsafe { MmapInput::map_file(&json_file)? };
    let engine = MainEngine::compile_query(&jsonpath_query)?;
    let result = engine.run::<_, IndexResult>(&input)?;
    assert_eq!(result.get(), vec![151usize, 341usize,], "result != expected");
    Ok(())
}
#[test]
fn extract_from_twitter_json_containing_urls_with_multiple_escaped_slashes_compressed_with_query_descendant_for_url_limited_to_urls_arrays_with_mmap_input_and_index_result_using_recursive_engine(
) -> Result<(), Box<dyn Error>> {
    println ! ("on document twitter_urls.toml running the query $..entities..urls..url (descendant for url limited to urls arrays) with Input impl MmapInput and result mode IndexResult");
    let jsonpath_query = JsonPathQuery::parse("$..entities..urls..url")?;
    let json_file = fs::File::open("./tests/documents/json/compressed/twitter_urls.json")?;
    let input = unsafe { MmapInput::map_file(&json_file)? };
    let engine = RecursiveEngine::compile_query(&jsonpath_query)?;
    let result = engine.run::<_, IndexResult>(&input)?;
    assert_eq!(result.get(), vec![151usize, 341usize,], "result != expected");
    Ok(())
}
#[test]
fn extract_from_twitter_json_containing_urls_with_multiple_escaped_slashes_compressed_with_query_descendant_for_url_limited_to_urls_arrays_with_owned_bytes_and_count_result_using_main_engine(
) -> Result<(), Box<dyn Error>> {
    println ! ("on document twitter_urls.toml running the query $..entities..urls..url (descendant for url limited to urls arrays) with Input impl OwnedBytes and result mode CountResult");
    let jsonpath_query = JsonPathQuery::parse("$..entities..urls..url")?;
    let raw_json = fs::read_to_string("./tests/documents/json/compressed/twitter_urls.json")?;
    let input = OwnedBytes::new(&raw_json.as_bytes())?;
    let engine = MainEngine::compile_query(&jsonpath_query)?;
    let result = engine.run::<_, CountResult>(&input)?;
    assert_eq!(result.get(), 2u64, "result != expected");
    Ok(())
}
#[test]
fn extract_from_twitter_json_containing_urls_with_multiple_escaped_slashes_compressed_with_query_descendant_for_url_limited_to_urls_arrays_with_owned_bytes_and_count_result_using_recursive_engine(
) -> Result<(), Box<dyn Error>> {
    println ! ("on document twitter_urls.toml running the query $..entities..urls..url (descendant for url limited to urls arrays) with Input impl OwnedBytes and result mode CountResult");
    let jsonpath_query = JsonPathQuery::parse("$..entities..urls..url")?;
    let raw_json = fs::read_to_string("./tests/documents/json/compressed/twitter_urls.json")?;
    let input = OwnedBytes::new(&raw_json.as_bytes())?;
    let engine = RecursiveEngine::compile_query(&jsonpath_query)?;
    let result = engine.run::<_, CountResult>(&input)?;
    assert_eq!(result.get(), 2u64, "result != expected");
    Ok(())
}
#[test]
fn extract_from_twitter_json_containing_urls_with_multiple_escaped_slashes_compressed_with_query_descendant_for_url_limited_to_urls_arrays_with_owned_bytes_and_index_result_using_main_engine(
) -> Result<(), Box<dyn Error>> {
    println ! ("on document twitter_urls.toml running the query $..entities..urls..url (descendant for url limited to urls arrays) with Input impl OwnedBytes and result mode IndexResult");
    let jsonpath_query = JsonPathQuery::parse("$..entities..urls..url")?;
    let raw_json = fs::read_to_string("./tests/documents/json/compressed/twitter_urls.json")?;
    let input = OwnedBytes::new(&raw_json.as_bytes())?;
    let engine = MainEngine::compile_query(&jsonpath_query)?;
    let result = engine.run::<_, IndexResult>(&input)?;
    assert_eq!(result.get(), vec![151usize, 341usize,], "result != expected");
    Ok(())
}
#[test]
fn extract_from_twitter_json_containing_urls_with_multiple_escaped_slashes_compressed_with_query_descendant_for_url_limited_to_urls_arrays_with_owned_bytes_and_index_result_using_recursive_engine(
) -> Result<(), Box<dyn Error>> {
    println ! ("on document twitter_urls.toml running the query $..entities..urls..url (descendant for url limited to urls arrays) with Input impl OwnedBytes and result mode IndexResult");
    let jsonpath_query = JsonPathQuery::parse("$..entities..urls..url")?;
    let raw_json = fs::read_to_string("./tests/documents/json/compressed/twitter_urls.json")?;
    let input = OwnedBytes::new(&raw_json.as_bytes())?;
    let engine = RecursiveEngine::compile_query(&jsonpath_query)?;
    let result = engine.run::<_, IndexResult>(&input)?;
    assert_eq!(result.get(), vec![151usize, 341usize,], "result != expected");
    Ok(())
}
#[test]
fn extract_from_twitter_json_containing_urls_with_multiple_escaped_slashes_compressed_with_query_descendant_search_for_url_with_buffered_input_and_count_result_using_main_engine(
) -> Result<(), Box<dyn Error>> {
    println ! ("on document twitter_urls.toml running the query $..url (descendant search for url) with Input impl BufferedInput and result mode CountResult");
    let jsonpath_query = JsonPathQuery::parse("$..url")?;
    let json_file = fs::File::open("./tests/documents/json/compressed/twitter_urls.json")?;
    let input = BufferedInput::new(json_file);
    let engine = MainEngine::compile_query(&jsonpath_query)?;
    let result = engine.run::<_, CountResult>(&input)?;
    assert_eq!(result.get(), 8u64, "result != expected");
    Ok(())
}
#[test]
fn extract_from_twitter_json_containing_urls_with_multiple_escaped_slashes_compressed_with_query_descendant_search_for_url_with_buffered_input_and_count_result_using_recursive_engine(
) -> Result<(), Box<dyn Error>> {
    println ! ("on document twitter_urls.toml running the query $..url (descendant search for url) with Input impl BufferedInput and result mode CountResult");
    let jsonpath_query = JsonPathQuery::parse("$..url")?;
    let json_file = fs::File::open("./tests/documents/json/compressed/twitter_urls.json")?;
    let input = BufferedInput::new(json_file);
    let engine = RecursiveEngine::compile_query(&jsonpath_query)?;
    let result = engine.run::<_, CountResult>(&input)?;
    assert_eq!(result.get(), 8u64, "result != expected");
    Ok(())
}
#[test]
fn extract_from_twitter_json_containing_urls_with_multiple_escaped_slashes_compressed_with_query_descendant_search_for_url_with_buffered_input_and_index_result_using_main_engine(
) -> Result<(), Box<dyn Error>> {
    println ! ("on document twitter_urls.toml running the query $..url (descendant search for url) with Input impl BufferedInput and result mode IndexResult");
    let jsonpath_query = JsonPathQuery::parse("$..url")?;
    let json_file = fs::File::open("./tests/documents/json/compressed/twitter_urls.json")?;
    let input = BufferedInput::new(json_file);
    let engine = MainEngine::compile_query(&jsonpath_query)?;
    let result = engine.run::<_, IndexResult>(&input)?;
    assert_eq!(
        result.get(),
        vec![16usize, 90usize, 151usize, 198usize, 267usize, 341usize, 388usize, 426usize,],
        "result != expected"
    );
    Ok(())
}
#[test]
fn extract_from_twitter_json_containing_urls_with_multiple_escaped_slashes_compressed_with_query_descendant_search_for_url_with_buffered_input_and_index_result_using_recursive_engine(
) -> Result<(), Box<dyn Error>> {
    println ! ("on document twitter_urls.toml running the query $..url (descendant search for url) with Input impl BufferedInput and result mode IndexResult");
    let jsonpath_query = JsonPathQuery::parse("$..url")?;
    let json_file = fs::File::open("./tests/documents/json/compressed/twitter_urls.json")?;
    let input = BufferedInput::new(json_file);
    let engine = RecursiveEngine::compile_query(&jsonpath_query)?;
    let result = engine.run::<_, IndexResult>(&input)?;
    assert_eq!(
        result.get(),
        vec![16usize, 90usize, 151usize, 198usize, 267usize, 341usize, 388usize, 426usize,],
        "result != expected"
    );
    Ok(())
}
#[test]
fn extract_from_twitter_json_containing_urls_with_multiple_escaped_slashes_compressed_with_query_descendant_search_for_url_with_mmap_input_and_count_result_using_main_engine(
) -> Result<(), Box<dyn Error>> {
    println ! ("on document twitter_urls.toml running the query $..url (descendant search for url) with Input impl MmapInput and result mode CountResult");
    let jsonpath_query = JsonPathQuery::parse("$..url")?;
    let json_file = fs::File::open("./tests/documents/json/compressed/twitter_urls.json")?;
    let input = unsafe { MmapInput::map_file(&json_file)? };
    let engine = MainEngine::compile_query(&jsonpath_query)?;
    let result = engine.run::<_, CountResult>(&input)?;
    assert_eq!(result.get(), 8u64, "result != expected");
    Ok(())
}
#[test]
fn extract_from_twitter_json_containing_urls_with_multiple_escaped_slashes_compressed_with_query_descendant_search_for_url_with_mmap_input_and_count_result_using_recursive_engine(
) -> Result<(), Box<dyn Error>> {
    println ! ("on document twitter_urls.toml running the query $..url (descendant search for url) with Input impl MmapInput and result mode CountResult");
    let jsonpath_query = JsonPathQuery::parse("$..url")?;
    let json_file = fs::File::open("./tests/documents/json/compressed/twitter_urls.json")?;
    let input = unsafe { MmapInput::map_file(&json_file)? };
    let engine = RecursiveEngine::compile_query(&jsonpath_query)?;
    let result = engine.run::<_, CountResult>(&input)?;
    assert_eq!(result.get(), 8u64, "result != expected");
    Ok(())
}
#[test]
fn extract_from_twitter_json_containing_urls_with_multiple_escaped_slashes_compressed_with_query_descendant_search_for_url_with_mmap_input_and_index_result_using_main_engine(
) -> Result<(), Box<dyn Error>> {
    println ! ("on document twitter_urls.toml running the query $..url (descendant search for url) with Input impl MmapInput and result mode IndexResult");
    let jsonpath_query = JsonPathQuery::parse("$..url")?;
    let json_file = fs::File::open("./tests/documents/json/compressed/twitter_urls.json")?;
    let input = unsafe { MmapInput::map_file(&json_file)? };
    let engine = MainEngine::compile_query(&jsonpath_query)?;
    let result = engine.run::<_, IndexResult>(&input)?;
    assert_eq!(
        result.get(),
        vec![16usize, 90usize, 151usize, 198usize, 267usize, 341usize, 388usize, 426usize,],
        "result != expected"
    );
    Ok(())
}
#[test]
fn extract_from_twitter_json_containing_urls_with_multiple_escaped_slashes_compressed_with_query_descendant_search_for_url_with_mmap_input_and_index_result_using_recursive_engine(
) -> Result<(), Box<dyn Error>> {
    println ! ("on document twitter_urls.toml running the query $..url (descendant search for url) with Input impl MmapInput and result mode IndexResult");
    let jsonpath_query = JsonPathQuery::parse("$..url")?;
    let json_file = fs::File::open("./tests/documents/json/compressed/twitter_urls.json")?;
    let input = unsafe { MmapInput::map_file(&json_file)? };
    let engine = RecursiveEngine::compile_query(&jsonpath_query)?;
    let result = engine.run::<_, IndexResult>(&input)?;
    assert_eq!(
        result.get(),
        vec![16usize, 90usize, 151usize, 198usize, 267usize, 341usize, 388usize, 426usize,],
        "result != expected"
    );
    Ok(())
}
#[test]
fn extract_from_twitter_json_containing_urls_with_multiple_escaped_slashes_compressed_with_query_descendant_search_for_url_with_owned_bytes_and_count_result_using_main_engine(
) -> Result<(), Box<dyn Error>> {
    println ! ("on document twitter_urls.toml running the query $..url (descendant search for url) with Input impl OwnedBytes and result mode CountResult");
    let jsonpath_query = JsonPathQuery::parse("$..url")?;
    let raw_json = fs::read_to_string("./tests/documents/json/compressed/twitter_urls.json")?;
    let input = OwnedBytes::new(&raw_json.as_bytes())?;
    let engine = MainEngine::compile_query(&jsonpath_query)?;
    let result = engine.run::<_, CountResult>(&input)?;
    assert_eq!(result.get(), 8u64, "result != expected");
    Ok(())
}
#[test]
fn extract_from_twitter_json_containing_urls_with_multiple_escaped_slashes_compressed_with_query_descendant_search_for_url_with_owned_bytes_and_count_result_using_recursive_engine(
) -> Result<(), Box<dyn Error>> {
    println ! ("on document twitter_urls.toml running the query $..url (descendant search for url) with Input impl OwnedBytes and result mode CountResult");
    let jsonpath_query = JsonPathQuery::parse("$..url")?;
    let raw_json = fs::read_to_string("./tests/documents/json/compressed/twitter_urls.json")?;
    let input = OwnedBytes::new(&raw_json.as_bytes())?;
    let engine = RecursiveEngine::compile_query(&jsonpath_query)?;
    let result = engine.run::<_, CountResult>(&input)?;
    assert_eq!(result.get(), 8u64, "result != expected");
    Ok(())
}
#[test]
fn extract_from_twitter_json_containing_urls_with_multiple_escaped_slashes_compressed_with_query_descendant_search_for_url_with_owned_bytes_and_index_result_using_main_engine(
) -> Result<(), Box<dyn Error>> {
    println ! ("on document twitter_urls.toml running the query $..url (descendant search for url) with Input impl OwnedBytes and result mode IndexResult");
    let jsonpath_query = JsonPathQuery::parse("$..url")?;
    let raw_json = fs::read_to_string("./tests/documents/json/compressed/twitter_urls.json")?;
    let input = OwnedBytes::new(&raw_json.as_bytes())?;
    let engine = MainEngine::compile_query(&jsonpath_query)?;
    let result = engine.run::<_, IndexResult>(&input)?;
    assert_eq!(
        result.get(),
        vec![16usize, 90usize, 151usize, 198usize, 267usize, 341usize, 388usize, 426usize,],
        "result != expected"
    );
    Ok(())
}
#[test]
fn extract_from_twitter_json_containing_urls_with_multiple_escaped_slashes_compressed_with_query_descendant_search_for_url_with_owned_bytes_and_index_result_using_recursive_engine(
) -> Result<(), Box<dyn Error>> {
    println ! ("on document twitter_urls.toml running the query $..url (descendant search for url) with Input impl OwnedBytes and result mode IndexResult");
    let jsonpath_query = JsonPathQuery::parse("$..url")?;
    let raw_json = fs::read_to_string("./tests/documents/json/compressed/twitter_urls.json")?;
    let input = OwnedBytes::new(&raw_json.as_bytes())?;
    let engine = RecursiveEngine::compile_query(&jsonpath_query)?;
    let result = engine.run::<_, IndexResult>(&input)?;
    assert_eq!(
        result.get(),
        vec![16usize, 90usize, 151usize, 198usize, 267usize, 341usize, 388usize, 426usize,],
        "result != expected"
    );
    Ok(())
}
#[test]
fn extract_from_twitter_json_containing_urls_with_multiple_escaped_slashes_compressed_with_query_direct_path_to_the_top_level_url_with_buffered_input_and_count_result_using_main_engine(
) -> Result<(), Box<dyn Error>> {
    println ! ("on document twitter_urls.toml running the query $[0].url (direct path to the top-level url) with Input impl BufferedInput and result mode CountResult");
    let jsonpath_query = JsonPathQuery::parse("$[0].url")?;
    let json_file = fs::File::open("./tests/documents/json/compressed/twitter_urls.json")?;
    let input = BufferedInput::new(json_file);
    let engine = MainEngine::compile_query(&jsonpath_query)?;
    let result = engine.run::<_, CountResult>(&input)?;
    assert_eq!(result.get(), 1u64, "result != expected");
    Ok(())
}
#[test]
fn extract_from_twitter_json_containing_urls_with_multiple_escaped_slashes_compressed_with_query_direct_path_to_the_top_level_url_with_buffered_input_and_count_result_using_recursive_engine(
) -> Result<(), Box<dyn Error>> {
    println ! ("on document twitter_urls.toml running the query $[0].url (direct path to the top-level url) with Input impl BufferedInput and result mode CountResult");
    let jsonpath_query = JsonPathQuery::parse("$[0].url")?;
    let json_file = fs::File::open("./tests/documents/json/compressed/twitter_urls.json")?;
    let input = BufferedInput::new(json_file);
    let engine = RecursiveEngine::compile_query(&jsonpath_query)?;
    let result = engine.run::<_, CountResult>(&input)?;
    assert_eq!(result.get(), 1u64, "result != expected");
    Ok(())
}
#[test]
fn extract_from_twitter_json_containing_urls_with_multiple_escaped_slashes_compressed_with_query_direct_path_to_the_top_level_url_with_buffered_input_and_index_result_using_main_engine(
) -> Result<(), Box<dyn Error>> {
    println ! ("on document twitter_urls.toml running the query $[0].url (direct path to the top-level url) with Input impl BufferedInput and result mode IndexResult");
    let jsonpath_query = JsonPathQuery::parse("$[0].url")?;
    let json_file = fs::File::open("./tests/documents/json/compressed/twitter_urls.json")?;
    let input = BufferedInput::new(json_file);
    let engine = MainEngine::compile_query(&jsonpath_query)?;
    let result = engine.run::<_, IndexResult>(&input)?;
    assert_eq!(result.get(), vec![426usize,], "result != expected");
    Ok(())
}
#[test]
fn extract_from_twitter_json_containing_urls_with_multiple_escaped_slashes_compressed_with_query_direct_path_to_the_top_level_url_with_buffered_input_and_index_result_using_recursive_engine(
) -> Result<(), Box<dyn Error>> {
    println ! ("on document twitter_urls.toml running the query $[0].url (direct path to the top-level url) with Input impl BufferedInput and result mode IndexResult");
    let jsonpath_query = JsonPathQuery::parse("$[0].url")?;
    let json_file = fs::File::open("./tests/documents/json/compressed/twitter_urls.json")?;
    let input = BufferedInput::new(json_file);
    let engine = RecursiveEngine::compile_query(&jsonpath_query)?;
    let result = engine.run::<_, IndexResult>(&input)?;
    assert_eq!(result.get(), vec![426usize,], "result != expected");
    Ok(())
}
#[test]
fn extract_from_twitter_json_containing_urls_with_multiple_escaped_slashes_compressed_with_query_direct_path_to_the_top_level_url_with_mmap_input_and_count_result_using_main_engine(
) -> Result<(), Box<dyn Error>> {
    println ! ("on document twitter_urls.toml running the query $[0].url (direct path to the top-level url) with Input impl MmapInput and result mode CountResult");
    let jsonpath_query = JsonPathQuery::parse("$[0].url")?;
    let json_file = fs::File::open("./tests/documents/json/compressed/twitter_urls.json")?;
    let input = unsafe { MmapInput::map_file(&json_file)? };
    let engine = MainEngine::compile_query(&jsonpath_query)?;
    let result = engine.run::<_, CountResult>(&input)?;
    assert_eq!(result.get(), 1u64, "result != expected");
    Ok(())
}
#[test]
fn extract_from_twitter_json_containing_urls_with_multiple_escaped_slashes_compressed_with_query_direct_path_to_the_top_level_url_with_mmap_input_and_count_result_using_recursive_engine(
) -> Result<(), Box<dyn Error>> {
    println ! ("on document twitter_urls.toml running the query $[0].url (direct path to the top-level url) with Input impl MmapInput and result mode CountResult");
    let jsonpath_query = JsonPathQuery::parse("$[0].url")?;
    let json_file = fs::File::open("./tests/documents/json/compressed/twitter_urls.json")?;
    let input = unsafe { MmapInput::map_file(&json_file)? };
    let engine = RecursiveEngine::compile_query(&jsonpath_query)?;
    let result = engine.run::<_, CountResult>(&input)?;
    assert_eq!(result.get(), 1u64, "result != expected");
    Ok(())
}
#[test]
fn extract_from_twitter_json_containing_urls_with_multiple_escaped_slashes_compressed_with_query_direct_path_to_the_top_level_url_with_mmap_input_and_index_result_using_main_engine(
) -> Result<(), Box<dyn Error>> {
    println ! ("on document twitter_urls.toml running the query $[0].url (direct path to the top-level url) with Input impl MmapInput and result mode IndexResult");
    let jsonpath_query = JsonPathQuery::parse("$[0].url")?;
    let json_file = fs::File::open("./tests/documents/json/compressed/twitter_urls.json")?;
    let input = unsafe { MmapInput::map_file(&json_file)? };
    let engine = MainEngine::compile_query(&jsonpath_query)?;
    let result = engine.run::<_, IndexResult>(&input)?;
    assert_eq!(result.get(), vec![426usize,], "result != expected");
    Ok(())
}
#[test]
fn extract_from_twitter_json_containing_urls_with_multiple_escaped_slashes_compressed_with_query_direct_path_to_the_top_level_url_with_mmap_input_and_index_result_using_recursive_engine(
) -> Result<(), Box<dyn Error>> {
    println ! ("on document twitter_urls.toml running the query $[0].url (direct path to the top-level url) with Input impl MmapInput and result mode IndexResult");
    let jsonpath_query = JsonPathQuery::parse("$[0].url")?;
    let json_file = fs::File::open("./tests/documents/json/compressed/twitter_urls.json")?;
    let input = unsafe { MmapInput::map_file(&json_file)? };
    let engine = RecursiveEngine::compile_query(&jsonpath_query)?;
    let result = engine.run::<_, IndexResult>(&input)?;
    assert_eq!(result.get(), vec![426usize,], "result != expected");
    Ok(())
}
#[test]
fn extract_from_twitter_json_containing_urls_with_multiple_escaped_slashes_compressed_with_query_direct_path_to_the_top_level_url_with_owned_bytes_and_count_result_using_main_engine(
) -> Result<(), Box<dyn Error>> {
    println ! ("on document twitter_urls.toml running the query $[0].url (direct path to the top-level url) with Input impl OwnedBytes and result mode CountResult");
    let jsonpath_query = JsonPathQuery::parse("$[0].url")?;
    let raw_json = fs::read_to_string("./tests/documents/json/compressed/twitter_urls.json")?;
    let input = OwnedBytes::new(&raw_json.as_bytes())?;
    let engine = MainEngine::compile_query(&jsonpath_query)?;
    let result = engine.run::<_, CountResult>(&input)?;
    assert_eq!(result.get(), 1u64, "result != expected");
    Ok(())
}
#[test]
fn extract_from_twitter_json_containing_urls_with_multiple_escaped_slashes_compressed_with_query_direct_path_to_the_top_level_url_with_owned_bytes_and_count_result_using_recursive_engine(
) -> Result<(), Box<dyn Error>> {
    println ! ("on document twitter_urls.toml running the query $[0].url (direct path to the top-level url) with Input impl OwnedBytes and result mode CountResult");
    let jsonpath_query = JsonPathQuery::parse("$[0].url")?;
    let raw_json = fs::read_to_string("./tests/documents/json/compressed/twitter_urls.json")?;
    let input = OwnedBytes::new(&raw_json.as_bytes())?;
    let engine = RecursiveEngine::compile_query(&jsonpath_query)?;
    let result = engine.run::<_, CountResult>(&input)?;
    assert_eq!(result.get(), 1u64, "result != expected");
    Ok(())
}
#[test]
fn extract_from_twitter_json_containing_urls_with_multiple_escaped_slashes_compressed_with_query_direct_path_to_the_top_level_url_with_owned_bytes_and_index_result_using_main_engine(
) -> Result<(), Box<dyn Error>> {
    println ! ("on document twitter_urls.toml running the query $[0].url (direct path to the top-level url) with Input impl OwnedBytes and result mode IndexResult");
    let jsonpath_query = JsonPathQuery::parse("$[0].url")?;
    let raw_json = fs::read_to_string("./tests/documents/json/compressed/twitter_urls.json")?;
    let input = OwnedBytes::new(&raw_json.as_bytes())?;
    let engine = MainEngine::compile_query(&jsonpath_query)?;
    let result = engine.run::<_, IndexResult>(&input)?;
    assert_eq!(result.get(), vec![426usize,], "result != expected");
    Ok(())
}
#[test]
fn extract_from_twitter_json_containing_urls_with_multiple_escaped_slashes_compressed_with_query_direct_path_to_the_top_level_url_with_owned_bytes_and_index_result_using_recursive_engine(
) -> Result<(), Box<dyn Error>> {
    println ! ("on document twitter_urls.toml running the query $[0].url (direct path to the top-level url) with Input impl OwnedBytes and result mode IndexResult");
    let jsonpath_query = JsonPathQuery::parse("$[0].url")?;
    let raw_json = fs::read_to_string("./tests/documents/json/compressed/twitter_urls.json")?;
    let input = OwnedBytes::new(&raw_json.as_bytes())?;
    let engine = RecursiveEngine::compile_query(&jsonpath_query)?;
    let result = engine.run::<_, IndexResult>(&input)?;
    assert_eq!(result.get(), vec![426usize,], "result != expected");
    Ok(())
}
#[test]
fn extract_from_twitter_json_containing_urls_with_multiple_escaped_slashes_with_query_descendant_entities_then_url_with_buffered_input_and_count_result_using_main_engine(
) -> Result<(), Box<dyn Error>> {
    println ! ("on document twitter_urls.toml running the query $..entities..url (descendant entities then url) with Input impl BufferedInput and result mode CountResult");
    let jsonpath_query = JsonPathQuery::parse("$..entities..url")?;
    let json_file = fs::File::open("./tests/documents/json/twitter_urls.json")?;
    let input = BufferedInput::new(json_file);
    let engine = MainEngine::compile_query(&jsonpath_query)?;
    let result = engine.run::<_, CountResult>(&input)?;
    assert_eq!(result.get(), 4u64, "result != expected");
    Ok(())
}
#[test]
fn extract_from_twitter_json_containing_urls_with_multiple_escaped_slashes_with_query_descendant_entities_then_url_with_buffered_input_and_count_result_using_recursive_engine(
) -> Result<(), Box<dyn Error>> {
    println ! ("on document twitter_urls.toml running the query $..entities..url (descendant entities then url) with Input impl BufferedInput and result mode CountResult");
    let jsonpath_query = JsonPathQuery::parse("$..entities..url")?;
    let json_file = fs::File::open("./tests/documents/json/twitter_urls.json")?;
    let input = BufferedInput::new(json_file);
    let engine = RecursiveEngine::compile_query(&jsonpath_query)?;
    let result = engine.run::<_, CountResult>(&input)?;
    assert_eq!(result.get(), 4u64, "result != expected");
    Ok(())
}
#[test]
fn extract_from_twitter_json_containing_urls_with_multiple_escaped_slashes_with_query_descendant_entities_then_url_with_buffered_input_and_index_result_using_main_engine(
) -> Result<(), Box<dyn Error>> {
    println ! ("on document twitter_urls.toml running the query $..entities..url (descendant entities then url) with Input impl BufferedInput and result mode IndexResult");
    let jsonpath_query = JsonPathQuery::parse("$..entities..url")?;
    let json_file = fs::File::open("./tests/documents/json/twitter_urls.json")?;
    let input = BufferedInput::new(json_file);
    let engine = MainEngine::compile_query(&jsonpath_query)?;
    let result = engine.run::<_, IndexResult>(&input)?;
    assert_eq!(
        result.get(),
        vec![323usize, 473usize, 883usize, 1013usize,],
        "result != expected"
    );
    Ok(())
}
#[test]
fn extract_from_twitter_json_containing_urls_with_multiple_escaped_slashes_with_query_descendant_entities_then_url_with_buffered_input_and_index_result_using_recursive_engine(
) -> Result<(), Box<dyn Error>> {
    println ! ("on document twitter_urls.toml running the query $..entities..url (descendant entities then url) with Input impl BufferedInput and result mode IndexResult");
    let jsonpath_query = JsonPathQuery::parse("$..entities..url")?;
    let json_file = fs::File::open("./tests/documents/json/twitter_urls.json")?;
    let input = BufferedInput::new(json_file);
    let engine = RecursiveEngine::compile_query(&jsonpath_query)?;
    let result = engine.run::<_, IndexResult>(&input)?;
    assert_eq!(
        result.get(),
        vec![323usize, 473usize, 883usize, 1013usize,],
        "result != expected"
    );
    Ok(())
}
#[test]
fn extract_from_twitter_json_containing_urls_with_multiple_escaped_slashes_with_query_descendant_entities_then_url_with_mmap_input_and_count_result_using_main_engine(
) -> Result<(), Box<dyn Error>> {
    println ! ("on document twitter_urls.toml running the query $..entities..url (descendant entities then url) with Input impl MmapInput and result mode CountResult");
    let jsonpath_query = JsonPathQuery::parse("$..entities..url")?;
    let json_file = fs::File::open("./tests/documents/json/twitter_urls.json")?;
    let input = unsafe { MmapInput::map_file(&json_file)? };
    let engine = MainEngine::compile_query(&jsonpath_query)?;
    let result = engine.run::<_, CountResult>(&input)?;
    assert_eq!(result.get(), 4u64, "result != expected");
    Ok(())
}
#[test]
fn extract_from_twitter_json_containing_urls_with_multiple_escaped_slashes_with_query_descendant_entities_then_url_with_mmap_input_and_count_result_using_recursive_engine(
) -> Result<(), Box<dyn Error>> {
    println ! ("on document twitter_urls.toml running the query $..entities..url (descendant entities then url) with Input impl MmapInput and result mode CountResult");
    let jsonpath_query = JsonPathQuery::parse("$..entities..url")?;
    let json_file = fs::File::open("./tests/documents/json/twitter_urls.json")?;
    let input = unsafe { MmapInput::map_file(&json_file)? };
    let engine = RecursiveEngine::compile_query(&jsonpath_query)?;
    let result = engine.run::<_, CountResult>(&input)?;
    assert_eq!(result.get(), 4u64, "result != expected");
    Ok(())
}
#[test]
fn extract_from_twitter_json_containing_urls_with_multiple_escaped_slashes_with_query_descendant_entities_then_url_with_mmap_input_and_index_result_using_main_engine(
) -> Result<(), Box<dyn Error>> {
    println ! ("on document twitter_urls.toml running the query $..entities..url (descendant entities then url) with Input impl MmapInput and result mode IndexResult");
    let jsonpath_query = JsonPathQuery::parse("$..entities..url")?;
    let json_file = fs::File::open("./tests/documents/json/twitter_urls.json")?;
    let input = unsafe { MmapInput::map_file(&json_file)? };
    let engine = MainEngine::compile_query(&jsonpath_query)?;
    let result = engine.run::<_, IndexResult>(&input)?;
    assert_eq!(
        result.get(),
        vec![323usize, 473usize, 883usize, 1013usize,],
        "result != expected"
    );
    Ok(())
}
#[test]
fn extract_from_twitter_json_containing_urls_with_multiple_escaped_slashes_with_query_descendant_entities_then_url_with_mmap_input_and_index_result_using_recursive_engine(
) -> Result<(), Box<dyn Error>> {
    println ! ("on document twitter_urls.toml running the query $..entities..url (descendant entities then url) with Input impl MmapInput and result mode IndexResult");
    let jsonpath_query = JsonPathQuery::parse("$..entities..url")?;
    let json_file = fs::File::open("./tests/documents/json/twitter_urls.json")?;
    let input = unsafe { MmapInput::map_file(&json_file)? };
    let engine = RecursiveEngine::compile_query(&jsonpath_query)?;
    let result = engine.run::<_, IndexResult>(&input)?;
    assert_eq!(
        result.get(),
        vec![323usize, 473usize, 883usize, 1013usize,],
        "result != expected"
    );
    Ok(())
}
#[test]
fn extract_from_twitter_json_containing_urls_with_multiple_escaped_slashes_with_query_descendant_entities_then_url_with_owned_bytes_and_count_result_using_main_engine(
) -> Result<(), Box<dyn Error>> {
    println ! ("on document twitter_urls.toml running the query $..entities..url (descendant entities then url) with Input impl OwnedBytes and result mode CountResult");
    let jsonpath_query = JsonPathQuery::parse("$..entities..url")?;
    let raw_json = fs::read_to_string("./tests/documents/json/twitter_urls.json")?;
    let input = OwnedBytes::new(&raw_json.as_bytes())?;
    let engine = MainEngine::compile_query(&jsonpath_query)?;
    let result = engine.run::<_, CountResult>(&input)?;
    assert_eq!(result.get(), 4u64, "result != expected");
    Ok(())
}
#[test]
fn extract_from_twitter_json_containing_urls_with_multiple_escaped_slashes_with_query_descendant_entities_then_url_with_owned_bytes_and_count_result_using_recursive_engine(
) -> Result<(), Box<dyn Error>> {
    println ! ("on document twitter_urls.toml running the query $..entities..url (descendant entities then url) with Input impl OwnedBytes and result mode CountResult");
    let jsonpath_query = JsonPathQuery::parse("$..entities..url")?;
    let raw_json = fs::read_to_string("./tests/documents/json/twitter_urls.json")?;
    let input = OwnedBytes::new(&raw_json.as_bytes())?;
    let engine = RecursiveEngine::compile_query(&jsonpath_query)?;
    let result = engine.run::<_, CountResult>(&input)?;
    assert_eq!(result.get(), 4u64, "result != expected");
    Ok(())
}
#[test]
fn extract_from_twitter_json_containing_urls_with_multiple_escaped_slashes_with_query_descendant_entities_then_url_with_owned_bytes_and_index_result_using_main_engine(
) -> Result<(), Box<dyn Error>> {
    println ! ("on document twitter_urls.toml running the query $..entities..url (descendant entities then url) with Input impl OwnedBytes and result mode IndexResult");
    let jsonpath_query = JsonPathQuery::parse("$..entities..url")?;
    let raw_json = fs::read_to_string("./tests/documents/json/twitter_urls.json")?;
    let input = OwnedBytes::new(&raw_json.as_bytes())?;
    let engine = MainEngine::compile_query(&jsonpath_query)?;
    let result = engine.run::<_, IndexResult>(&input)?;
    assert_eq!(
        result.get(),
        vec![323usize, 473usize, 883usize, 1013usize,],
        "result != expected"
    );
    Ok(())
}
#[test]
fn extract_from_twitter_json_containing_urls_with_multiple_escaped_slashes_with_query_descendant_entities_then_url_with_owned_bytes_and_index_result_using_recursive_engine(
) -> Result<(), Box<dyn Error>> {
    println ! ("on document twitter_urls.toml running the query $..entities..url (descendant entities then url) with Input impl OwnedBytes and result mode IndexResult");
    let jsonpath_query = JsonPathQuery::parse("$..entities..url")?;
    let raw_json = fs::read_to_string("./tests/documents/json/twitter_urls.json")?;
    let input = OwnedBytes::new(&raw_json.as_bytes())?;
    let engine = RecursiveEngine::compile_query(&jsonpath_query)?;
    let result = engine.run::<_, IndexResult>(&input)?;
    assert_eq!(
        result.get(),
        vec![323usize, 473usize, 883usize, 1013usize,],
        "result != expected"
    );
    Ok(())
}
#[test]
fn extract_from_twitter_json_containing_urls_with_multiple_escaped_slashes_with_query_descendant_for_url_limited_to_direct_urls_arrays_with_buffered_input_and_count_result_using_main_engine(
) -> Result<(), Box<dyn Error>> {
    println ! ("on document twitter_urls.toml running the query $..entities.urls..url (descendant for url limited to direct urls arrays) with Input impl BufferedInput and result mode CountResult");
    let jsonpath_query = JsonPathQuery::parse("$..entities.urls..url")?;
    let json_file = fs::File::open("./tests/documents/json/twitter_urls.json")?;
    let input = BufferedInput::new(json_file);
    let engine = MainEngine::compile_query(&jsonpath_query)?;
    let result = engine.run::<_, CountResult>(&input)?;
    assert_eq!(result.get(), 2u64, "result != expected");
    Ok(())
}
#[test]
fn extract_from_twitter_json_containing_urls_with_multiple_escaped_slashes_with_query_descendant_for_url_limited_to_direct_urls_arrays_with_buffered_input_and_count_result_using_recursive_engine(
) -> Result<(), Box<dyn Error>> {
    println ! ("on document twitter_urls.toml running the query $..entities.urls..url (descendant for url limited to direct urls arrays) with Input impl BufferedInput and result mode CountResult");
    let jsonpath_query = JsonPathQuery::parse("$..entities.urls..url")?;
    let json_file = fs::File::open("./tests/documents/json/twitter_urls.json")?;
    let input = BufferedInput::new(json_file);
    let engine = RecursiveEngine::compile_query(&jsonpath_query)?;
    let result = engine.run::<_, CountResult>(&input)?;
    assert_eq!(result.get(), 2u64, "result != expected");
    Ok(())
}
#[test]
fn extract_from_twitter_json_containing_urls_with_multiple_escaped_slashes_with_query_descendant_for_url_limited_to_direct_urls_arrays_with_buffered_input_and_index_result_using_main_engine(
) -> Result<(), Box<dyn Error>> {
    println ! ("on document twitter_urls.toml running the query $..entities.urls..url (descendant for url limited to direct urls arrays) with Input impl BufferedInput and result mode IndexResult");
    let jsonpath_query = JsonPathQuery::parse("$..entities.urls..url")?;
    let json_file = fs::File::open("./tests/documents/json/twitter_urls.json")?;
    let input = BufferedInput::new(json_file);
    let engine = MainEngine::compile_query(&jsonpath_query)?;
    let result = engine.run::<_, IndexResult>(&input)?;
    assert_eq!(result.get(), vec![323usize, 883usize,], "result != expected");
    Ok(())
}
#[test]
fn extract_from_twitter_json_containing_urls_with_multiple_escaped_slashes_with_query_descendant_for_url_limited_to_direct_urls_arrays_with_buffered_input_and_index_result_using_recursive_engine(
) -> Result<(), Box<dyn Error>> {
    println ! ("on document twitter_urls.toml running the query $..entities.urls..url (descendant for url limited to direct urls arrays) with Input impl BufferedInput and result mode IndexResult");
    let jsonpath_query = JsonPathQuery::parse("$..entities.urls..url")?;
    let json_file = fs::File::open("./tests/documents/json/twitter_urls.json")?;
    let input = BufferedInput::new(json_file);
    let engine = RecursiveEngine::compile_query(&jsonpath_query)?;
    let result = engine.run::<_, IndexResult>(&input)?;
    assert_eq!(result.get(), vec![323usize, 883usize,], "result != expected");
    Ok(())
}
#[test]
fn extract_from_twitter_json_containing_urls_with_multiple_escaped_slashes_with_query_descendant_for_url_limited_to_direct_urls_arrays_with_mmap_input_and_count_result_using_main_engine(
) -> Result<(), Box<dyn Error>> {
    println ! ("on document twitter_urls.toml running the query $..entities.urls..url (descendant for url limited to direct urls arrays) with Input impl MmapInput and result mode CountResult");
    let jsonpath_query = JsonPathQuery::parse("$..entities.urls..url")?;
    let json_file = fs::File::open("./tests/documents/json/twitter_urls.json")?;
    let input = unsafe { MmapInput::map_file(&json_file)? };
    let engine = MainEngine::compile_query(&jsonpath_query)?;
    let result = engine.run::<_, CountResult>(&input)?;
    assert_eq!(result.get(), 2u64, "result != expected");
    Ok(())
}
#[test]
fn extract_from_twitter_json_containing_urls_with_multiple_escaped_slashes_with_query_descendant_for_url_limited_to_direct_urls_arrays_with_mmap_input_and_count_result_using_recursive_engine(
) -> Result<(), Box<dyn Error>> {
    println ! ("on document twitter_urls.toml running the query $..entities.urls..url (descendant for url limited to direct urls arrays) with Input impl MmapInput and result mode CountResult");
    let jsonpath_query = JsonPathQuery::parse("$..entities.urls..url")?;
    let json_file = fs::File::open("./tests/documents/json/twitter_urls.json")?;
    let input = unsafe { MmapInput::map_file(&json_file)? };
    let engine = RecursiveEngine::compile_query(&jsonpath_query)?;
    let result = engine.run::<_, CountResult>(&input)?;
    assert_eq!(result.get(), 2u64, "result != expected");
    Ok(())
}
#[test]
fn extract_from_twitter_json_containing_urls_with_multiple_escaped_slashes_with_query_descendant_for_url_limited_to_direct_urls_arrays_with_mmap_input_and_index_result_using_main_engine(
) -> Result<(), Box<dyn Error>> {
    println ! ("on document twitter_urls.toml running the query $..entities.urls..url (descendant for url limited to direct urls arrays) with Input impl MmapInput and result mode IndexResult");
    let jsonpath_query = JsonPathQuery::parse("$..entities.urls..url")?;
    let json_file = fs::File::open("./tests/documents/json/twitter_urls.json")?;
    let input = unsafe { MmapInput::map_file(&json_file)? };
    let engine = MainEngine::compile_query(&jsonpath_query)?;
    let result = engine.run::<_, IndexResult>(&input)?;
    assert_eq!(result.get(), vec![323usize, 883usize,], "result != expected");
    Ok(())
}
#[test]
fn extract_from_twitter_json_containing_urls_with_multiple_escaped_slashes_with_query_descendant_for_url_limited_to_direct_urls_arrays_with_mmap_input_and_index_result_using_recursive_engine(
) -> Result<(), Box<dyn Error>> {
    println ! ("on document twitter_urls.toml running the query $..entities.urls..url (descendant for url limited to direct urls arrays) with Input impl MmapInput and result mode IndexResult");
    let jsonpath_query = JsonPathQuery::parse("$..entities.urls..url")?;
    let json_file = fs::File::open("./tests/documents/json/twitter_urls.json")?;
    let input = unsafe { MmapInput::map_file(&json_file)? };
    let engine = RecursiveEngine::compile_query(&jsonpath_query)?;
    let result = engine.run::<_, IndexResult>(&input)?;
    assert_eq!(result.get(), vec![323usize, 883usize,], "result != expected");
    Ok(())
}
#[test]
fn extract_from_twitter_json_containing_urls_with_multiple_escaped_slashes_with_query_descendant_for_url_limited_to_direct_urls_arrays_with_owned_bytes_and_count_result_using_main_engine(
) -> Result<(), Box<dyn Error>> {
    println ! ("on document twitter_urls.toml running the query $..entities.urls..url (descendant for url limited to direct urls arrays) with Input impl OwnedBytes and result mode CountResult");
    let jsonpath_query = JsonPathQuery::parse("$..entities.urls..url")?;
    let raw_json = fs::read_to_string("./tests/documents/json/twitter_urls.json")?;
    let input = OwnedBytes::new(&raw_json.as_bytes())?;
    let engine = MainEngine::compile_query(&jsonpath_query)?;
    let result = engine.run::<_, CountResult>(&input)?;
    assert_eq!(result.get(), 2u64, "result != expected");
    Ok(())
}
#[test]
fn extract_from_twitter_json_containing_urls_with_multiple_escaped_slashes_with_query_descendant_for_url_limited_to_direct_urls_arrays_with_owned_bytes_and_count_result_using_recursive_engine(
) -> Result<(), Box<dyn Error>> {
    println ! ("on document twitter_urls.toml running the query $..entities.urls..url (descendant for url limited to direct urls arrays) with Input impl OwnedBytes and result mode CountResult");
    let jsonpath_query = JsonPathQuery::parse("$..entities.urls..url")?;
    let raw_json = fs::read_to_string("./tests/documents/json/twitter_urls.json")?;
    let input = OwnedBytes::new(&raw_json.as_bytes())?;
    let engine = RecursiveEngine::compile_query(&jsonpath_query)?;
    let result = engine.run::<_, CountResult>(&input)?;
    assert_eq!(result.get(), 2u64, "result != expected");
    Ok(())
}
#[test]
fn extract_from_twitter_json_containing_urls_with_multiple_escaped_slashes_with_query_descendant_for_url_limited_to_direct_urls_arrays_with_owned_bytes_and_index_result_using_main_engine(
) -> Result<(), Box<dyn Error>> {
    println ! ("on document twitter_urls.toml running the query $..entities.urls..url (descendant for url limited to direct urls arrays) with Input impl OwnedBytes and result mode IndexResult");
    let jsonpath_query = JsonPathQuery::parse("$..entities.urls..url")?;
    let raw_json = fs::read_to_string("./tests/documents/json/twitter_urls.json")?;
    let input = OwnedBytes::new(&raw_json.as_bytes())?;
    let engine = MainEngine::compile_query(&jsonpath_query)?;
    let result = engine.run::<_, IndexResult>(&input)?;
    assert_eq!(result.get(), vec![323usize, 883usize,], "result != expected");
    Ok(())
}
#[test]
fn extract_from_twitter_json_containing_urls_with_multiple_escaped_slashes_with_query_descendant_for_url_limited_to_direct_urls_arrays_with_owned_bytes_and_index_result_using_recursive_engine(
) -> Result<(), Box<dyn Error>> {
    println ! ("on document twitter_urls.toml running the query $..entities.urls..url (descendant for url limited to direct urls arrays) with Input impl OwnedBytes and result mode IndexResult");
    let jsonpath_query = JsonPathQuery::parse("$..entities.urls..url")?;
    let raw_json = fs::read_to_string("./tests/documents/json/twitter_urls.json")?;
    let input = OwnedBytes::new(&raw_json.as_bytes())?;
    let engine = RecursiveEngine::compile_query(&jsonpath_query)?;
    let result = engine.run::<_, IndexResult>(&input)?;
    assert_eq!(result.get(), vec![323usize, 883usize,], "result != expected");
    Ok(())
}
#[test]
fn extract_from_twitter_json_containing_urls_with_multiple_escaped_slashes_with_query_descendant_for_url_limited_to_urls_arrays_with_buffered_input_and_count_result_using_main_engine(
) -> Result<(), Box<dyn Error>> {
    println ! ("on document twitter_urls.toml running the query $..entities..urls..url (descendant for url limited to urls arrays) with Input impl BufferedInput and result mode CountResult");
    let jsonpath_query = JsonPathQuery::parse("$..entities..urls..url")?;
    let json_file = fs::File::open("./tests/documents/json/twitter_urls.json")?;
    let input = BufferedInput::new(json_file);
    let engine = MainEngine::compile_query(&jsonpath_query)?;
    let result = engine.run::<_, CountResult>(&input)?;
    assert_eq!(result.get(), 2u64, "result != expected");
    Ok(())
}
#[test]
fn extract_from_twitter_json_containing_urls_with_multiple_escaped_slashes_with_query_descendant_for_url_limited_to_urls_arrays_with_buffered_input_and_count_result_using_recursive_engine(
) -> Result<(), Box<dyn Error>> {
    println ! ("on document twitter_urls.toml running the query $..entities..urls..url (descendant for url limited to urls arrays) with Input impl BufferedInput and result mode CountResult");
    let jsonpath_query = JsonPathQuery::parse("$..entities..urls..url")?;
    let json_file = fs::File::open("./tests/documents/json/twitter_urls.json")?;
    let input = BufferedInput::new(json_file);
    let engine = RecursiveEngine::compile_query(&jsonpath_query)?;
    let result = engine.run::<_, CountResult>(&input)?;
    assert_eq!(result.get(), 2u64, "result != expected");
    Ok(())
}
#[test]
fn extract_from_twitter_json_containing_urls_with_multiple_escaped_slashes_with_query_descendant_for_url_limited_to_urls_arrays_with_buffered_input_and_index_result_using_main_engine(
) -> Result<(), Box<dyn Error>> {
    println ! ("on document twitter_urls.toml running the query $..entities..urls..url (descendant for url limited to urls arrays) with Input impl BufferedInput and result mode IndexResult");
    let jsonpath_query = JsonPathQuery::parse("$..entities..urls..url")?;
    let json_file = fs::File::open("./tests/documents/json/twitter_urls.json")?;
    let input = BufferedInput::new(json_file);
    let engine = MainEngine::compile_query(&jsonpath_query)?;
    let result = engine.run::<_, IndexResult>(&input)?;
    assert_eq!(result.get(), vec![323usize, 883usize,], "result != expected");
    Ok(())
}
#[test]
fn extract_from_twitter_json_containing_urls_with_multiple_escaped_slashes_with_query_descendant_for_url_limited_to_urls_arrays_with_buffered_input_and_index_result_using_recursive_engine(
) -> Result<(), Box<dyn Error>> {
    println ! ("on document twitter_urls.toml running the query $..entities..urls..url (descendant for url limited to urls arrays) with Input impl BufferedInput and result mode IndexResult");
    let jsonpath_query = JsonPathQuery::parse("$..entities..urls..url")?;
    let json_file = fs::File::open("./tests/documents/json/twitter_urls.json")?;
    let input = BufferedInput::new(json_file);
    let engine = RecursiveEngine::compile_query(&jsonpath_query)?;
    let result = engine.run::<_, IndexResult>(&input)?;
    assert_eq!(result.get(), vec![323usize, 883usize,], "result != expected");
    Ok(())
}
#[test]
fn extract_from_twitter_json_containing_urls_with_multiple_escaped_slashes_with_query_descendant_for_url_limited_to_urls_arrays_with_mmap_input_and_count_result_using_main_engine(
) -> Result<(), Box<dyn Error>> {
    println ! ("on document twitter_urls.toml running the query $..entities..urls..url (descendant for url limited to urls arrays) with Input impl MmapInput and result mode CountResult");
    let jsonpath_query = JsonPathQuery::parse("$..entities..urls..url")?;
    let json_file = fs::File::open("./tests/documents/json/twitter_urls.json")?;
    let input = unsafe { MmapInput::map_file(&json_file)? };
    let engine = MainEngine::compile_query(&jsonpath_query)?;
    let result = engine.run::<_, CountResult>(&input)?;
    assert_eq!(result.get(), 2u64, "result != expected");
    Ok(())
}
#[test]
fn extract_from_twitter_json_containing_urls_with_multiple_escaped_slashes_with_query_descendant_for_url_limited_to_urls_arrays_with_mmap_input_and_count_result_using_recursive_engine(
) -> Result<(), Box<dyn Error>> {
    println ! ("on document twitter_urls.toml running the query $..entities..urls..url (descendant for url limited to urls arrays) with Input impl MmapInput and result mode CountResult");
    let jsonpath_query = JsonPathQuery::parse("$..entities..urls..url")?;
    let json_file = fs::File::open("./tests/documents/json/twitter_urls.json")?;
    let input = unsafe { MmapInput::map_file(&json_file)? };
    let engine = RecursiveEngine::compile_query(&jsonpath_query)?;
    let result = engine.run::<_, CountResult>(&input)?;
    assert_eq!(result.get(), 2u64, "result != expected");
    Ok(())
}
#[test]
fn extract_from_twitter_json_containing_urls_with_multiple_escaped_slashes_with_query_descendant_for_url_limited_to_urls_arrays_with_mmap_input_and_index_result_using_main_engine(
) -> Result<(), Box<dyn Error>> {
    println ! ("on document twitter_urls.toml running the query $..entities..urls..url (descendant for url limited to urls arrays) with Input impl MmapInput and result mode IndexResult");
    let jsonpath_query = JsonPathQuery::parse("$..entities..urls..url")?;
    let json_file = fs::File::open("./tests/documents/json/twitter_urls.json")?;
    let input = unsafe { MmapInput::map_file(&json_file)? };
    let engine = MainEngine::compile_query(&jsonpath_query)?;
    let result = engine.run::<_, IndexResult>(&input)?;
    assert_eq!(result.get(), vec![323usize, 883usize,], "result != expected");
    Ok(())
}
#[test]
fn extract_from_twitter_json_containing_urls_with_multiple_escaped_slashes_with_query_descendant_for_url_limited_to_urls_arrays_with_mmap_input_and_index_result_using_recursive_engine(
) -> Result<(), Box<dyn Error>> {
    println ! ("on document twitter_urls.toml running the query $..entities..urls..url (descendant for url limited to urls arrays) with Input impl MmapInput and result mode IndexResult");
    let jsonpath_query = JsonPathQuery::parse("$..entities..urls..url")?;
    let json_file = fs::File::open("./tests/documents/json/twitter_urls.json")?;
    let input = unsafe { MmapInput::map_file(&json_file)? };
    let engine = RecursiveEngine::compile_query(&jsonpath_query)?;
    let result = engine.run::<_, IndexResult>(&input)?;
    assert_eq!(result.get(), vec![323usize, 883usize,], "result != expected");
    Ok(())
}
#[test]
fn extract_from_twitter_json_containing_urls_with_multiple_escaped_slashes_with_query_descendant_for_url_limited_to_urls_arrays_with_owned_bytes_and_count_result_using_main_engine(
) -> Result<(), Box<dyn Error>> {
    println ! ("on document twitter_urls.toml running the query $..entities..urls..url (descendant for url limited to urls arrays) with Input impl OwnedBytes and result mode CountResult");
    let jsonpath_query = JsonPathQuery::parse("$..entities..urls..url")?;
    let raw_json = fs::read_to_string("./tests/documents/json/twitter_urls.json")?;
    let input = OwnedBytes::new(&raw_json.as_bytes())?;
    let engine = MainEngine::compile_query(&jsonpath_query)?;
    let result = engine.run::<_, CountResult>(&input)?;
    assert_eq!(result.get(), 2u64, "result != expected");
    Ok(())
}
#[test]
fn extract_from_twitter_json_containing_urls_with_multiple_escaped_slashes_with_query_descendant_for_url_limited_to_urls_arrays_with_owned_bytes_and_count_result_using_recursive_engine(
) -> Result<(), Box<dyn Error>> {
    println ! ("on document twitter_urls.toml running the query $..entities..urls..url (descendant for url limited to urls arrays) with Input impl OwnedBytes and result mode CountResult");
    let jsonpath_query = JsonPathQuery::parse("$..entities..urls..url")?;
    let raw_json = fs::read_to_string("./tests/documents/json/twitter_urls.json")?;
    let input = OwnedBytes::new(&raw_json.as_bytes())?;
    let engine = RecursiveEngine::compile_query(&jsonpath_query)?;
    let result = engine.run::<_, CountResult>(&input)?;
    assert_eq!(result.get(), 2u64, "result != expected");
    Ok(())
}
#[test]
fn extract_from_twitter_json_containing_urls_with_multiple_escaped_slashes_with_query_descendant_for_url_limited_to_urls_arrays_with_owned_bytes_and_index_result_using_main_engine(
) -> Result<(), Box<dyn Error>> {
    println ! ("on document twitter_urls.toml running the query $..entities..urls..url (descendant for url limited to urls arrays) with Input impl OwnedBytes and result mode IndexResult");
    let jsonpath_query = JsonPathQuery::parse("$..entities..urls..url")?;
    let raw_json = fs::read_to_string("./tests/documents/json/twitter_urls.json")?;
    let input = OwnedBytes::new(&raw_json.as_bytes())?;
    let engine = MainEngine::compile_query(&jsonpath_query)?;
    let result = engine.run::<_, IndexResult>(&input)?;
    assert_eq!(result.get(), vec![323usize, 883usize,], "result != expected");
    Ok(())
}
#[test]
fn extract_from_twitter_json_containing_urls_with_multiple_escaped_slashes_with_query_descendant_for_url_limited_to_urls_arrays_with_owned_bytes_and_index_result_using_recursive_engine(
) -> Result<(), Box<dyn Error>> {
    println ! ("on document twitter_urls.toml running the query $..entities..urls..url (descendant for url limited to urls arrays) with Input impl OwnedBytes and result mode IndexResult");
    let jsonpath_query = JsonPathQuery::parse("$..entities..urls..url")?;
    let raw_json = fs::read_to_string("./tests/documents/json/twitter_urls.json")?;
    let input = OwnedBytes::new(&raw_json.as_bytes())?;
    let engine = RecursiveEngine::compile_query(&jsonpath_query)?;
    let result = engine.run::<_, IndexResult>(&input)?;
    assert_eq!(result.get(), vec![323usize, 883usize,], "result != expected");
    Ok(())
}
#[test]
fn extract_from_twitter_json_containing_urls_with_multiple_escaped_slashes_with_query_descendant_search_for_url_with_buffered_input_and_count_result_using_main_engine(
) -> Result<(), Box<dyn Error>> {
    println ! ("on document twitter_urls.toml running the query $..url (descendant search for url) with Input impl BufferedInput and result mode CountResult");
    let jsonpath_query = JsonPathQuery::parse("$..url")?;
    let json_file = fs::File::open("./tests/documents/json/twitter_urls.json")?;
    let input = BufferedInput::new(json_file);
    let engine = MainEngine::compile_query(&jsonpath_query)?;
    let result = engine.run::<_, CountResult>(&input)?;
    assert_eq!(result.get(), 8u64, "result != expected");
    Ok(())
}
#[test]
fn extract_from_twitter_json_containing_urls_with_multiple_escaped_slashes_with_query_descendant_search_for_url_with_buffered_input_and_count_result_using_recursive_engine(
) -> Result<(), Box<dyn Error>> {
    println ! ("on document twitter_urls.toml running the query $..url (descendant search for url) with Input impl BufferedInput and result mode CountResult");
    let jsonpath_query = JsonPathQuery::parse("$..url")?;
    let json_file = fs::File::open("./tests/documents/json/twitter_urls.json")?;
    let input = BufferedInput::new(json_file);
    let engine = RecursiveEngine::compile_query(&jsonpath_query)?;
    let result = engine.run::<_, CountResult>(&input)?;
    assert_eq!(result.get(), 8u64, "result != expected");
    Ok(())
}
#[test]
fn extract_from_twitter_json_containing_urls_with_multiple_escaped_slashes_with_query_descendant_search_for_url_with_buffered_input_and_index_result_using_main_engine(
) -> Result<(), Box<dyn Error>> {
    println ! ("on document twitter_urls.toml running the query $..url (descendant search for url) with Input impl BufferedInput and result mode IndexResult");
    let jsonpath_query = JsonPathQuery::parse("$..url")?;
    let json_file = fs::File::open("./tests/documents/json/twitter_urls.json")?;
    let input = BufferedInput::new(json_file);
    let engine = MainEngine::compile_query(&jsonpath_query)?;
    let result = engine.run::<_, IndexResult>(&input)?;
    assert_eq!(
        result.get(),
        vec![45usize, 170usize, 323usize, 473usize, 672usize, 883usize, 1013usize, 1100usize,],
        "result != expected"
    );
    Ok(())
}
#[test]
fn extract_from_twitter_json_containing_urls_with_multiple_escaped_slashes_with_query_descendant_search_for_url_with_buffered_input_and_index_result_using_recursive_engine(
) -> Result<(), Box<dyn Error>> {
    println ! ("on document twitter_urls.toml running the query $..url (descendant search for url) with Input impl BufferedInput and result mode IndexResult");
    let jsonpath_query = JsonPathQuery::parse("$..url")?;
    let json_file = fs::File::open("./tests/documents/json/twitter_urls.json")?;
    let input = BufferedInput::new(json_file);
    let engine = RecursiveEngine::compile_query(&jsonpath_query)?;
    let result = engine.run::<_, IndexResult>(&input)?;
    assert_eq!(
        result.get(),
        vec![45usize, 170usize, 323usize, 473usize, 672usize, 883usize, 1013usize, 1100usize,],
        "result != expected"
    );
    Ok(())
}
#[test]
fn extract_from_twitter_json_containing_urls_with_multiple_escaped_slashes_with_query_descendant_search_for_url_with_mmap_input_and_count_result_using_main_engine(
) -> Result<(), Box<dyn Error>> {
    println ! ("on document twitter_urls.toml running the query $..url (descendant search for url) with Input impl MmapInput and result mode CountResult");
    let jsonpath_query = JsonPathQuery::parse("$..url")?;
    let json_file = fs::File::open("./tests/documents/json/twitter_urls.json")?;
    let input = unsafe { MmapInput::map_file(&json_file)? };
    let engine = MainEngine::compile_query(&jsonpath_query)?;
    let result = engine.run::<_, CountResult>(&input)?;
    assert_eq!(result.get(), 8u64, "result != expected");
    Ok(())
}
#[test]
fn extract_from_twitter_json_containing_urls_with_multiple_escaped_slashes_with_query_descendant_search_for_url_with_mmap_input_and_count_result_using_recursive_engine(
) -> Result<(), Box<dyn Error>> {
    println ! ("on document twitter_urls.toml running the query $..url (descendant search for url) with Input impl MmapInput and result mode CountResult");
    let jsonpath_query = JsonPathQuery::parse("$..url")?;
    let json_file = fs::File::open("./tests/documents/json/twitter_urls.json")?;
    let input = unsafe { MmapInput::map_file(&json_file)? };
    let engine = RecursiveEngine::compile_query(&jsonpath_query)?;
    let result = engine.run::<_, CountResult>(&input)?;
    assert_eq!(result.get(), 8u64, "result != expected");
    Ok(())
}
#[test]
fn extract_from_twitter_json_containing_urls_with_multiple_escaped_slashes_with_query_descendant_search_for_url_with_mmap_input_and_index_result_using_main_engine(
) -> Result<(), Box<dyn Error>> {
    println ! ("on document twitter_urls.toml running the query $..url (descendant search for url) with Input impl MmapInput and result mode IndexResult");
    let jsonpath_query = JsonPathQuery::parse("$..url")?;
    let json_file = fs::File::open("./tests/documents/json/twitter_urls.json")?;
    let input = unsafe { MmapInput::map_file(&json_file)? };
    let engine = MainEngine::compile_query(&jsonpath_query)?;
    let result = engine.run::<_, IndexResult>(&input)?;
    assert_eq!(
        result.get(),
        vec![45usize, 170usize, 323usize, 473usize, 672usize, 883usize, 1013usize, 1100usize,],
        "result != expected"
    );
    Ok(())
}
#[test]
fn extract_from_twitter_json_containing_urls_with_multiple_escaped_slashes_with_query_descendant_search_for_url_with_mmap_input_and_index_result_using_recursive_engine(
) -> Result<(), Box<dyn Error>> {
    println ! ("on document twitter_urls.toml running the query $..url (descendant search for url) with Input impl MmapInput and result mode IndexResult");
    let jsonpath_query = JsonPathQuery::parse("$..url")?;
    let json_file = fs::File::open("./tests/documents/json/twitter_urls.json")?;
    let input = unsafe { MmapInput::map_file(&json_file)? };
    let engine = RecursiveEngine::compile_query(&jsonpath_query)?;
    let result = engine.run::<_, IndexResult>(&input)?;
    assert_eq!(
        result.get(),
        vec![45usize, 170usize, 323usize, 473usize, 672usize, 883usize, 1013usize, 1100usize,],
        "result != expected"
    );
    Ok(())
}
#[test]
fn extract_from_twitter_json_containing_urls_with_multiple_escaped_slashes_with_query_descendant_search_for_url_with_owned_bytes_and_count_result_using_main_engine(
) -> Result<(), Box<dyn Error>> {
    println ! ("on document twitter_urls.toml running the query $..url (descendant search for url) with Input impl OwnedBytes and result mode CountResult");
    let jsonpath_query = JsonPathQuery::parse("$..url")?;
    let raw_json = fs::read_to_string("./tests/documents/json/twitter_urls.json")?;
    let input = OwnedBytes::new(&raw_json.as_bytes())?;
    let engine = MainEngine::compile_query(&jsonpath_query)?;
    let result = engine.run::<_, CountResult>(&input)?;
    assert_eq!(result.get(), 8u64, "result != expected");
    Ok(())
}
#[test]
fn extract_from_twitter_json_containing_urls_with_multiple_escaped_slashes_with_query_descendant_search_for_url_with_owned_bytes_and_count_result_using_recursive_engine(
) -> Result<(), Box<dyn Error>> {
    println ! ("on document twitter_urls.toml running the query $..url (descendant search for url) with Input impl OwnedBytes and result mode CountResult");
    let jsonpath_query = JsonPathQuery::parse("$..url")?;
    let raw_json = fs::read_to_string("./tests/documents/json/twitter_urls.json")?;
    let input = OwnedBytes::new(&raw_json.as_bytes())?;
    let engine = RecursiveEngine::compile_query(&jsonpath_query)?;
    let result = engine.run::<_, CountResult>(&input)?;
    assert_eq!(result.get(), 8u64, "result != expected");
    Ok(())
}
#[test]
fn extract_from_twitter_json_containing_urls_with_multiple_escaped_slashes_with_query_descendant_search_for_url_with_owned_bytes_and_index_result_using_main_engine(
) -> Result<(), Box<dyn Error>> {
    println ! ("on document twitter_urls.toml running the query $..url (descendant search for url) with Input impl OwnedBytes and result mode IndexResult");
    let jsonpath_query = JsonPathQuery::parse("$..url")?;
    let raw_json = fs::read_to_string("./tests/documents/json/twitter_urls.json")?;
    let input = OwnedBytes::new(&raw_json.as_bytes())?;
    let engine = MainEngine::compile_query(&jsonpath_query)?;
    let result = engine.run::<_, IndexResult>(&input)?;
    assert_eq!(
        result.get(),
        vec![45usize, 170usize, 323usize, 473usize, 672usize, 883usize, 1013usize, 1100usize,],
        "result != expected"
    );
    Ok(())
}
#[test]
fn extract_from_twitter_json_containing_urls_with_multiple_escaped_slashes_with_query_descendant_search_for_url_with_owned_bytes_and_index_result_using_recursive_engine(
) -> Result<(), Box<dyn Error>> {
    println ! ("on document twitter_urls.toml running the query $..url (descendant search for url) with Input impl OwnedBytes and result mode IndexResult");
    let jsonpath_query = JsonPathQuery::parse("$..url")?;
    let raw_json = fs::read_to_string("./tests/documents/json/twitter_urls.json")?;
    let input = OwnedBytes::new(&raw_json.as_bytes())?;
    let engine = RecursiveEngine::compile_query(&jsonpath_query)?;
    let result = engine.run::<_, IndexResult>(&input)?;
    assert_eq!(
        result.get(),
        vec![45usize, 170usize, 323usize, 473usize, 672usize, 883usize, 1013usize, 1100usize,],
        "result != expected"
    );
    Ok(())
}
#[test]
fn extract_from_twitter_json_containing_urls_with_multiple_escaped_slashes_with_query_direct_path_to_the_top_level_url_with_buffered_input_and_count_result_using_main_engine(
) -> Result<(), Box<dyn Error>> {
    println ! ("on document twitter_urls.toml running the query $[0].url (direct path to the top-level url) with Input impl BufferedInput and result mode CountResult");
    let jsonpath_query = JsonPathQuery::parse("$[0].url")?;
    let json_file = fs::File::open("./tests/documents/json/twitter_urls.json")?;
    let input = BufferedInput::new(json_file);
    let engine = MainEngine::compile_query(&jsonpath_query)?;
    let result = engine.run::<_, CountResult>(&input)?;
    assert_eq!(result.get(), 1u64, "result != expected");
    Ok(())
}
#[test]
fn extract_from_twitter_json_containing_urls_with_multiple_escaped_slashes_with_query_direct_path_to_the_top_level_url_with_buffered_input_and_count_result_using_recursive_engine(
) -> Result<(), Box<dyn Error>> {
    println ! ("on document twitter_urls.toml running the query $[0].url (direct path to the top-level url) with Input impl BufferedInput and result mode CountResult");
    let jsonpath_query = JsonPathQuery::parse("$[0].url")?;
    let json_file = fs::File::open("./tests/documents/json/twitter_urls.json")?;
    let input = BufferedInput::new(json_file);
    let engine = RecursiveEngine::compile_query(&jsonpath_query)?;
    let result = engine.run::<_, CountResult>(&input)?;
    assert_eq!(result.get(), 1u64, "result != expected");
    Ok(())
}
#[test]
fn extract_from_twitter_json_containing_urls_with_multiple_escaped_slashes_with_query_direct_path_to_the_top_level_url_with_buffered_input_and_index_result_using_main_engine(
) -> Result<(), Box<dyn Error>> {
    println ! ("on document twitter_urls.toml running the query $[0].url (direct path to the top-level url) with Input impl BufferedInput and result mode IndexResult");
    let jsonpath_query = JsonPathQuery::parse("$[0].url")?;
    let json_file = fs::File::open("./tests/documents/json/twitter_urls.json")?;
    let input = BufferedInput::new(json_file);
    let engine = MainEngine::compile_query(&jsonpath_query)?;
    let result = engine.run::<_, IndexResult>(&input)?;
    assert_eq!(result.get(), vec![1100usize,], "result != expected");
    Ok(())
}
#[test]
fn extract_from_twitter_json_containing_urls_with_multiple_escaped_slashes_with_query_direct_path_to_the_top_level_url_with_buffered_input_and_index_result_using_recursive_engine(
) -> Result<(), Box<dyn Error>> {
    println ! ("on document twitter_urls.toml running the query $[0].url (direct path to the top-level url) with Input impl BufferedInput and result mode IndexResult");
    let jsonpath_query = JsonPathQuery::parse("$[0].url")?;
    let json_file = fs::File::open("./tests/documents/json/twitter_urls.json")?;
    let input = BufferedInput::new(json_file);
    let engine = RecursiveEngine::compile_query(&jsonpath_query)?;
    let result = engine.run::<_, IndexResult>(&input)?;
    assert_eq!(result.get(), vec![1100usize,], "result != expected");
    Ok(())
}
#[test]
fn extract_from_twitter_json_containing_urls_with_multiple_escaped_slashes_with_query_direct_path_to_the_top_level_url_with_mmap_input_and_count_result_using_main_engine(
) -> Result<(), Box<dyn Error>> {
    println ! ("on document twitter_urls.toml running the query $[0].url (direct path to the top-level url) with Input impl MmapInput and result mode CountResult");
    let jsonpath_query = JsonPathQuery::parse("$[0].url")?;
    let json_file = fs::File::open("./tests/documents/json/twitter_urls.json")?;
    let input = unsafe { MmapInput::map_file(&json_file)? };
    let engine = MainEngine::compile_query(&jsonpath_query)?;
    let result = engine.run::<_, CountResult>(&input)?;
    assert_eq!(result.get(), 1u64, "result != expected");
    Ok(())
}
#[test]
fn extract_from_twitter_json_containing_urls_with_multiple_escaped_slashes_with_query_direct_path_to_the_top_level_url_with_mmap_input_and_count_result_using_recursive_engine(
) -> Result<(), Box<dyn Error>> {
    println ! ("on document twitter_urls.toml running the query $[0].url (direct path to the top-level url) with Input impl MmapInput and result mode CountResult");
    let jsonpath_query = JsonPathQuery::parse("$[0].url")?;
    let json_file = fs::File::open("./tests/documents/json/twitter_urls.json")?;
    let input = unsafe { MmapInput::map_file(&json_file)? };
    let engine = RecursiveEngine::compile_query(&jsonpath_query)?;
    let result = engine.run::<_, CountResult>(&input)?;
    assert_eq!(result.get(), 1u64, "result != expected");
    Ok(())
}
#[test]
fn extract_from_twitter_json_containing_urls_with_multiple_escaped_slashes_with_query_direct_path_to_the_top_level_url_with_mmap_input_and_index_result_using_main_engine(
) -> Result<(), Box<dyn Error>> {
    println ! ("on document twitter_urls.toml running the query $[0].url (direct path to the top-level url) with Input impl MmapInput and result mode IndexResult");
    let jsonpath_query = JsonPathQuery::parse("$[0].url")?;
    let json_file = fs::File::open("./tests/documents/json/twitter_urls.json")?;
    let input = unsafe { MmapInput::map_file(&json_file)? };
    let engine = MainEngine::compile_query(&jsonpath_query)?;
    let result = engine.run::<_, IndexResult>(&input)?;
    assert_eq!(result.get(), vec![1100usize,], "result != expected");
    Ok(())
}
#[test]
fn extract_from_twitter_json_containing_urls_with_multiple_escaped_slashes_with_query_direct_path_to_the_top_level_url_with_mmap_input_and_index_result_using_recursive_engine(
) -> Result<(), Box<dyn Error>> {
    println ! ("on document twitter_urls.toml running the query $[0].url (direct path to the top-level url) with Input impl MmapInput and result mode IndexResult");
    let jsonpath_query = JsonPathQuery::parse("$[0].url")?;
    let json_file = fs::File::open("./tests/documents/json/twitter_urls.json")?;
    let input = unsafe { MmapInput::map_file(&json_file)? };
    let engine = RecursiveEngine::compile_query(&jsonpath_query)?;
    let result = engine.run::<_, IndexResult>(&input)?;
    assert_eq!(result.get(), vec![1100usize,], "result != expected");
    Ok(())
}
#[test]
fn extract_from_twitter_json_containing_urls_with_multiple_escaped_slashes_with_query_direct_path_to_the_top_level_url_with_owned_bytes_and_count_result_using_main_engine(
) -> Result<(), Box<dyn Error>> {
    println ! ("on document twitter_urls.toml running the query $[0].url (direct path to the top-level url) with Input impl OwnedBytes and result mode CountResult");
    let jsonpath_query = JsonPathQuery::parse("$[0].url")?;
    let raw_json = fs::read_to_string("./tests/documents/json/twitter_urls.json")?;
    let input = OwnedBytes::new(&raw_json.as_bytes())?;
    let engine = MainEngine::compile_query(&jsonpath_query)?;
    let result = engine.run::<_, CountResult>(&input)?;
    assert_eq!(result.get(), 1u64, "result != expected");
    Ok(())
}
#[test]
fn extract_from_twitter_json_containing_urls_with_multiple_escaped_slashes_with_query_direct_path_to_the_top_level_url_with_owned_bytes_and_count_result_using_recursive_engine(
) -> Result<(), Box<dyn Error>> {
    println ! ("on document twitter_urls.toml running the query $[0].url (direct path to the top-level url) with Input impl OwnedBytes and result mode CountResult");
    let jsonpath_query = JsonPathQuery::parse("$[0].url")?;
    let raw_json = fs::read_to_string("./tests/documents/json/twitter_urls.json")?;
    let input = OwnedBytes::new(&raw_json.as_bytes())?;
    let engine = RecursiveEngine::compile_query(&jsonpath_query)?;
    let result = engine.run::<_, CountResult>(&input)?;
    assert_eq!(result.get(), 1u64, "result != expected");
    Ok(())
}
#[test]
fn extract_from_twitter_json_containing_urls_with_multiple_escaped_slashes_with_query_direct_path_to_the_top_level_url_with_owned_bytes_and_index_result_using_main_engine(
) -> Result<(), Box<dyn Error>> {
    println ! ("on document twitter_urls.toml running the query $[0].url (direct path to the top-level url) with Input impl OwnedBytes and result mode IndexResult");
    let jsonpath_query = JsonPathQuery::parse("$[0].url")?;
    let raw_json = fs::read_to_string("./tests/documents/json/twitter_urls.json")?;
    let input = OwnedBytes::new(&raw_json.as_bytes())?;
    let engine = MainEngine::compile_query(&jsonpath_query)?;
    let result = engine.run::<_, IndexResult>(&input)?;
    assert_eq!(result.get(), vec![1100usize,], "result != expected");
    Ok(())
}
#[test]
fn extract_from_twitter_json_containing_urls_with_multiple_escaped_slashes_with_query_direct_path_to_the_top_level_url_with_owned_bytes_and_index_result_using_recursive_engine(
) -> Result<(), Box<dyn Error>> {
    println ! ("on document twitter_urls.toml running the query $[0].url (direct path to the top-level url) with Input impl OwnedBytes and result mode IndexResult");
    let jsonpath_query = JsonPathQuery::parse("$[0].url")?;
    let raw_json = fs::read_to_string("./tests/documents/json/twitter_urls.json")?;
    let input = OwnedBytes::new(&raw_json.as_bytes())?;
    let engine = RecursiveEngine::compile_query(&jsonpath_query)?;
    let result = engine.run::<_, IndexResult>(&input)?;
    assert_eq!(result.get(), vec![1100usize,], "result != expected");
    Ok(())
}
#[test]
fn label_b_and_b_with_escaped_quote_to_trick_naive_string_comparison_compressed_with_query_descendant_search_for_b_with_a_leading_quote_with_buffered_input_and_count_result_using_main_engine(
) -> Result<(), Box<dyn Error>> {
    println ! ("on document memchr_trap.toml running the query $..['\"b'] (descendant search for 'b' with a leading quote) with Input impl BufferedInput and result mode CountResult");
    let jsonpath_query = JsonPathQuery::parse("$..['\"b']")?;
    let json_file = fs::File::open("./tests/documents/json/compressed/memchr_trap.json")?;
    let input = BufferedInput::new(json_file);
    let engine = MainEngine::compile_query(&jsonpath_query)?;
    let result = engine.run::<_, CountResult>(&input)?;
    assert_eq!(result.get(), 1u64, "result != expected");
    Ok(())
}
#[test]
fn label_b_and_b_with_escaped_quote_to_trick_naive_string_comparison_compressed_with_query_descendant_search_for_b_with_a_leading_quote_with_buffered_input_and_count_result_using_recursive_engine(
) -> Result<(), Box<dyn Error>> {
    println ! ("on document memchr_trap.toml running the query $..['\"b'] (descendant search for 'b' with a leading quote) with Input impl BufferedInput and result mode CountResult");
    let jsonpath_query = JsonPathQuery::parse("$..['\"b']")?;
    let json_file = fs::File::open("./tests/documents/json/compressed/memchr_trap.json")?;
    let input = BufferedInput::new(json_file);
    let engine = RecursiveEngine::compile_query(&jsonpath_query)?;
    let result = engine.run::<_, CountResult>(&input)?;
    assert_eq!(result.get(), 1u64, "result != expected");
    Ok(())
}
#[test]
fn label_b_and_b_with_escaped_quote_to_trick_naive_string_comparison_compressed_with_query_descendant_search_for_b_with_a_leading_quote_with_buffered_input_and_index_result_using_main_engine(
) -> Result<(), Box<dyn Error>> {
    println ! ("on document memchr_trap.toml running the query $..['\"b'] (descendant search for 'b' with a leading quote) with Input impl BufferedInput and result mode IndexResult");
    let jsonpath_query = JsonPathQuery::parse("$..['\"b']")?;
    let json_file = fs::File::open("./tests/documents/json/compressed/memchr_trap.json")?;
    let input = BufferedInput::new(json_file);
    let engine = MainEngine::compile_query(&jsonpath_query)?;
    let result = engine.run::<_, IndexResult>(&input)?;
    assert_eq!(result.get(), vec![12usize,], "result != expected");
    Ok(())
}
#[test]
fn label_b_and_b_with_escaped_quote_to_trick_naive_string_comparison_compressed_with_query_descendant_search_for_b_with_a_leading_quote_with_buffered_input_and_index_result_using_recursive_engine(
) -> Result<(), Box<dyn Error>> {
    println ! ("on document memchr_trap.toml running the query $..['\"b'] (descendant search for 'b' with a leading quote) with Input impl BufferedInput and result mode IndexResult");
    let jsonpath_query = JsonPathQuery::parse("$..['\"b']")?;
    let json_file = fs::File::open("./tests/documents/json/compressed/memchr_trap.json")?;
    let input = BufferedInput::new(json_file);
    let engine = RecursiveEngine::compile_query(&jsonpath_query)?;
    let result = engine.run::<_, IndexResult>(&input)?;
    assert_eq!(result.get(), vec![12usize,], "result != expected");
    Ok(())
}
#[test]
fn label_b_and_b_with_escaped_quote_to_trick_naive_string_comparison_compressed_with_query_descendant_search_for_b_with_a_leading_quote_with_mmap_input_and_count_result_using_main_engine(
) -> Result<(), Box<dyn Error>> {
    println ! ("on document memchr_trap.toml running the query $..['\"b'] (descendant search for 'b' with a leading quote) with Input impl MmapInput and result mode CountResult");
    let jsonpath_query = JsonPathQuery::parse("$..['\"b']")?;
    let json_file = fs::File::open("./tests/documents/json/compressed/memchr_trap.json")?;
    let input = unsafe { MmapInput::map_file(&json_file)? };
    let engine = MainEngine::compile_query(&jsonpath_query)?;
    let result = engine.run::<_, CountResult>(&input)?;
    assert_eq!(result.get(), 1u64, "result != expected");
    Ok(())
}
#[test]
fn label_b_and_b_with_escaped_quote_to_trick_naive_string_comparison_compressed_with_query_descendant_search_for_b_with_a_leading_quote_with_mmap_input_and_count_result_using_recursive_engine(
) -> Result<(), Box<dyn Error>> {
    println ! ("on document memchr_trap.toml running the query $..['\"b'] (descendant search for 'b' with a leading quote) with Input impl MmapInput and result mode CountResult");
    let jsonpath_query = JsonPathQuery::parse("$..['\"b']")?;
    let json_file = fs::File::open("./tests/documents/json/compressed/memchr_trap.json")?;
    let input = unsafe { MmapInput::map_file(&json_file)? };
    let engine = RecursiveEngine::compile_query(&jsonpath_query)?;
    let result = engine.run::<_, CountResult>(&input)?;
    assert_eq!(result.get(), 1u64, "result != expected");
    Ok(())
}
#[test]
fn label_b_and_b_with_escaped_quote_to_trick_naive_string_comparison_compressed_with_query_descendant_search_for_b_with_a_leading_quote_with_mmap_input_and_index_result_using_main_engine(
) -> Result<(), Box<dyn Error>> {
    println ! ("on document memchr_trap.toml running the query $..['\"b'] (descendant search for 'b' with a leading quote) with Input impl MmapInput and result mode IndexResult");
    let jsonpath_query = JsonPathQuery::parse("$..['\"b']")?;
    let json_file = fs::File::open("./tests/documents/json/compressed/memchr_trap.json")?;
    let input = unsafe { MmapInput::map_file(&json_file)? };
    let engine = MainEngine::compile_query(&jsonpath_query)?;
    let result = engine.run::<_, IndexResult>(&input)?;
    assert_eq!(result.get(), vec![12usize,], "result != expected");
    Ok(())
}
#[test]
fn label_b_and_b_with_escaped_quote_to_trick_naive_string_comparison_compressed_with_query_descendant_search_for_b_with_a_leading_quote_with_mmap_input_and_index_result_using_recursive_engine(
) -> Result<(), Box<dyn Error>> {
    println ! ("on document memchr_trap.toml running the query $..['\"b'] (descendant search for 'b' with a leading quote) with Input impl MmapInput and result mode IndexResult");
    let jsonpath_query = JsonPathQuery::parse("$..['\"b']")?;
    let json_file = fs::File::open("./tests/documents/json/compressed/memchr_trap.json")?;
    let input = unsafe { MmapInput::map_file(&json_file)? };
    let engine = RecursiveEngine::compile_query(&jsonpath_query)?;
    let result = engine.run::<_, IndexResult>(&input)?;
    assert_eq!(result.get(), vec![12usize,], "result != expected");
    Ok(())
}
#[test]
fn label_b_and_b_with_escaped_quote_to_trick_naive_string_comparison_compressed_with_query_descendant_search_for_b_with_a_leading_quote_with_owned_bytes_and_count_result_using_main_engine(
) -> Result<(), Box<dyn Error>> {
    println ! ("on document memchr_trap.toml running the query $..['\"b'] (descendant search for 'b' with a leading quote) with Input impl OwnedBytes and result mode CountResult");
    let jsonpath_query = JsonPathQuery::parse("$..['\"b']")?;
    let raw_json = fs::read_to_string("./tests/documents/json/compressed/memchr_trap.json")?;
    let input = OwnedBytes::new(&raw_json.as_bytes())?;
    let engine = MainEngine::compile_query(&jsonpath_query)?;
    let result = engine.run::<_, CountResult>(&input)?;
    assert_eq!(result.get(), 1u64, "result != expected");
    Ok(())
}
#[test]
fn label_b_and_b_with_escaped_quote_to_trick_naive_string_comparison_compressed_with_query_descendant_search_for_b_with_a_leading_quote_with_owned_bytes_and_count_result_using_recursive_engine(
) -> Result<(), Box<dyn Error>> {
    println ! ("on document memchr_trap.toml running the query $..['\"b'] (descendant search for 'b' with a leading quote) with Input impl OwnedBytes and result mode CountResult");
    let jsonpath_query = JsonPathQuery::parse("$..['\"b']")?;
    let raw_json = fs::read_to_string("./tests/documents/json/compressed/memchr_trap.json")?;
    let input = OwnedBytes::new(&raw_json.as_bytes())?;
    let engine = RecursiveEngine::compile_query(&jsonpath_query)?;
    let result = engine.run::<_, CountResult>(&input)?;
    assert_eq!(result.get(), 1u64, "result != expected");
    Ok(())
}
#[test]
fn label_b_and_b_with_escaped_quote_to_trick_naive_string_comparison_compressed_with_query_descendant_search_for_b_with_a_leading_quote_with_owned_bytes_and_index_result_using_main_engine(
) -> Result<(), Box<dyn Error>> {
    println ! ("on document memchr_trap.toml running the query $..['\"b'] (descendant search for 'b' with a leading quote) with Input impl OwnedBytes and result mode IndexResult");
    let jsonpath_query = JsonPathQuery::parse("$..['\"b']")?;
    let raw_json = fs::read_to_string("./tests/documents/json/compressed/memchr_trap.json")?;
    let input = OwnedBytes::new(&raw_json.as_bytes())?;
    let engine = MainEngine::compile_query(&jsonpath_query)?;
    let result = engine.run::<_, IndexResult>(&input)?;
    assert_eq!(result.get(), vec![12usize,], "result != expected");
    Ok(())
}
#[test]
fn label_b_and_b_with_escaped_quote_to_trick_naive_string_comparison_compressed_with_query_descendant_search_for_b_with_a_leading_quote_with_owned_bytes_and_index_result_using_recursive_engine(
) -> Result<(), Box<dyn Error>> {
    println ! ("on document memchr_trap.toml running the query $..['\"b'] (descendant search for 'b' with a leading quote) with Input impl OwnedBytes and result mode IndexResult");
    let jsonpath_query = JsonPathQuery::parse("$..['\"b']")?;
    let raw_json = fs::read_to_string("./tests/documents/json/compressed/memchr_trap.json")?;
    let input = OwnedBytes::new(&raw_json.as_bytes())?;
    let engine = RecursiveEngine::compile_query(&jsonpath_query)?;
    let result = engine.run::<_, IndexResult>(&input)?;
    assert_eq!(result.get(), vec![12usize,], "result != expected");
    Ok(())
}
#[test]
fn label_b_and_b_with_escaped_quote_to_trick_naive_string_comparison_compressed_with_query_descendant_search_for_b_with_buffered_input_and_count_result_using_main_engine(
) -> Result<(), Box<dyn Error>> {
    println ! ("on document memchr_trap.toml running the query $..b (descendant search for 'b') with Input impl BufferedInput and result mode CountResult");
    let jsonpath_query = JsonPathQuery::parse("$..b")?;
    let json_file = fs::File::open("./tests/documents/json/compressed/memchr_trap.json")?;
    let input = BufferedInput::new(json_file);
    let engine = MainEngine::compile_query(&jsonpath_query)?;
    let result = engine.run::<_, CountResult>(&input)?;
    assert_eq!(result.get(), 1u64, "result != expected");
    Ok(())
}
#[test]
fn label_b_and_b_with_escaped_quote_to_trick_naive_string_comparison_compressed_with_query_descendant_search_for_b_with_buffered_input_and_count_result_using_recursive_engine(
) -> Result<(), Box<dyn Error>> {
    println ! ("on document memchr_trap.toml running the query $..b (descendant search for 'b') with Input impl BufferedInput and result mode CountResult");
    let jsonpath_query = JsonPathQuery::parse("$..b")?;
    let json_file = fs::File::open("./tests/documents/json/compressed/memchr_trap.json")?;
    let input = BufferedInput::new(json_file);
    let engine = RecursiveEngine::compile_query(&jsonpath_query)?;
    let result = engine.run::<_, CountResult>(&input)?;
    assert_eq!(result.get(), 1u64, "result != expected");
    Ok(())
}
#[test]
fn label_b_and_b_with_escaped_quote_to_trick_naive_string_comparison_compressed_with_query_descendant_search_for_b_with_buffered_input_and_index_result_using_main_engine(
) -> Result<(), Box<dyn Error>> {
    println ! ("on document memchr_trap.toml running the query $..b (descendant search for 'b') with Input impl BufferedInput and result mode IndexResult");
    let jsonpath_query = JsonPathQuery::parse("$..b")?;
    let json_file = fs::File::open("./tests/documents/json/compressed/memchr_trap.json")?;
    let input = BufferedInput::new(json_file);
    let engine = MainEngine::compile_query(&jsonpath_query)?;
    let result = engine.run::<_, IndexResult>(&input)?;
    assert_eq!(result.get(), vec![19usize,], "result != expected");
    Ok(())
}
#[test]
fn label_b_and_b_with_escaped_quote_to_trick_naive_string_comparison_compressed_with_query_descendant_search_for_b_with_buffered_input_and_index_result_using_recursive_engine(
) -> Result<(), Box<dyn Error>> {
    println ! ("on document memchr_trap.toml running the query $..b (descendant search for 'b') with Input impl BufferedInput and result mode IndexResult");
    let jsonpath_query = JsonPathQuery::parse("$..b")?;
    let json_file = fs::File::open("./tests/documents/json/compressed/memchr_trap.json")?;
    let input = BufferedInput::new(json_file);
    let engine = RecursiveEngine::compile_query(&jsonpath_query)?;
    let result = engine.run::<_, IndexResult>(&input)?;
    assert_eq!(result.get(), vec![19usize,], "result != expected");
    Ok(())
}
#[test]
fn label_b_and_b_with_escaped_quote_to_trick_naive_string_comparison_compressed_with_query_descendant_search_for_b_with_mmap_input_and_count_result_using_main_engine(
) -> Result<(), Box<dyn Error>> {
    println ! ("on document memchr_trap.toml running the query $..b (descendant search for 'b') with Input impl MmapInput and result mode CountResult");
    let jsonpath_query = JsonPathQuery::parse("$..b")?;
    let json_file = fs::File::open("./tests/documents/json/compressed/memchr_trap.json")?;
    let input = unsafe { MmapInput::map_file(&json_file)? };
    let engine = MainEngine::compile_query(&jsonpath_query)?;
    let result = engine.run::<_, CountResult>(&input)?;
    assert_eq!(result.get(), 1u64, "result != expected");
    Ok(())
}
#[test]
fn label_b_and_b_with_escaped_quote_to_trick_naive_string_comparison_compressed_with_query_descendant_search_for_b_with_mmap_input_and_count_result_using_recursive_engine(
) -> Result<(), Box<dyn Error>> {
    println ! ("on document memchr_trap.toml running the query $..b (descendant search for 'b') with Input impl MmapInput and result mode CountResult");
    let jsonpath_query = JsonPathQuery::parse("$..b")?;
    let json_file = fs::File::open("./tests/documents/json/compressed/memchr_trap.json")?;
    let input = unsafe { MmapInput::map_file(&json_file)? };
    let engine = RecursiveEngine::compile_query(&jsonpath_query)?;
    let result = engine.run::<_, CountResult>(&input)?;
    assert_eq!(result.get(), 1u64, "result != expected");
    Ok(())
}
#[test]
fn label_b_and_b_with_escaped_quote_to_trick_naive_string_comparison_compressed_with_query_descendant_search_for_b_with_mmap_input_and_index_result_using_main_engine(
) -> Result<(), Box<dyn Error>> {
    println ! ("on document memchr_trap.toml running the query $..b (descendant search for 'b') with Input impl MmapInput and result mode IndexResult");
    let jsonpath_query = JsonPathQuery::parse("$..b")?;
    let json_file = fs::File::open("./tests/documents/json/compressed/memchr_trap.json")?;
    let input = unsafe { MmapInput::map_file(&json_file)? };
    let engine = MainEngine::compile_query(&jsonpath_query)?;
    let result = engine.run::<_, IndexResult>(&input)?;
    assert_eq!(result.get(), vec![19usize,], "result != expected");
    Ok(())
}
#[test]
fn label_b_and_b_with_escaped_quote_to_trick_naive_string_comparison_compressed_with_query_descendant_search_for_b_with_mmap_input_and_index_result_using_recursive_engine(
) -> Result<(), Box<dyn Error>> {
    println ! ("on document memchr_trap.toml running the query $..b (descendant search for 'b') with Input impl MmapInput and result mode IndexResult");
    let jsonpath_query = JsonPathQuery::parse("$..b")?;
    let json_file = fs::File::open("./tests/documents/json/compressed/memchr_trap.json")?;
    let input = unsafe { MmapInput::map_file(&json_file)? };
    let engine = RecursiveEngine::compile_query(&jsonpath_query)?;
    let result = engine.run::<_, IndexResult>(&input)?;
    assert_eq!(result.get(), vec![19usize,], "result != expected");
    Ok(())
}
#[test]
fn label_b_and_b_with_escaped_quote_to_trick_naive_string_comparison_compressed_with_query_descendant_search_for_b_with_owned_bytes_and_count_result_using_main_engine(
) -> Result<(), Box<dyn Error>> {
    println ! ("on document memchr_trap.toml running the query $..b (descendant search for 'b') with Input impl OwnedBytes and result mode CountResult");
    let jsonpath_query = JsonPathQuery::parse("$..b")?;
    let raw_json = fs::read_to_string("./tests/documents/json/compressed/memchr_trap.json")?;
    let input = OwnedBytes::new(&raw_json.as_bytes())?;
    let engine = MainEngine::compile_query(&jsonpath_query)?;
    let result = engine.run::<_, CountResult>(&input)?;
    assert_eq!(result.get(), 1u64, "result != expected");
    Ok(())
}
#[test]
fn label_b_and_b_with_escaped_quote_to_trick_naive_string_comparison_compressed_with_query_descendant_search_for_b_with_owned_bytes_and_count_result_using_recursive_engine(
) -> Result<(), Box<dyn Error>> {
    println ! ("on document memchr_trap.toml running the query $..b (descendant search for 'b') with Input impl OwnedBytes and result mode CountResult");
    let jsonpath_query = JsonPathQuery::parse("$..b")?;
    let raw_json = fs::read_to_string("./tests/documents/json/compressed/memchr_trap.json")?;
    let input = OwnedBytes::new(&raw_json.as_bytes())?;
    let engine = RecursiveEngine::compile_query(&jsonpath_query)?;
    let result = engine.run::<_, CountResult>(&input)?;
    assert_eq!(result.get(), 1u64, "result != expected");
    Ok(())
}
#[test]
fn label_b_and_b_with_escaped_quote_to_trick_naive_string_comparison_compressed_with_query_descendant_search_for_b_with_owned_bytes_and_index_result_using_main_engine(
) -> Result<(), Box<dyn Error>> {
    println ! ("on document memchr_trap.toml running the query $..b (descendant search for 'b') with Input impl OwnedBytes and result mode IndexResult");
    let jsonpath_query = JsonPathQuery::parse("$..b")?;
    let raw_json = fs::read_to_string("./tests/documents/json/compressed/memchr_trap.json")?;
    let input = OwnedBytes::new(&raw_json.as_bytes())?;
    let engine = MainEngine::compile_query(&jsonpath_query)?;
    let result = engine.run::<_, IndexResult>(&input)?;
    assert_eq!(result.get(), vec![19usize,], "result != expected");
    Ok(())
}
#[test]
fn label_b_and_b_with_escaped_quote_to_trick_naive_string_comparison_compressed_with_query_descendant_search_for_b_with_owned_bytes_and_index_result_using_recursive_engine(
) -> Result<(), Box<dyn Error>> {
    println ! ("on document memchr_trap.toml running the query $..b (descendant search for 'b') with Input impl OwnedBytes and result mode IndexResult");
    let jsonpath_query = JsonPathQuery::parse("$..b")?;
    let raw_json = fs::read_to_string("./tests/documents/json/compressed/memchr_trap.json")?;
    let input = OwnedBytes::new(&raw_json.as_bytes())?;
    let engine = RecursiveEngine::compile_query(&jsonpath_query)?;
    let result = engine.run::<_, IndexResult>(&input)?;
    assert_eq!(result.get(), vec![19usize,], "result != expected");
    Ok(())
}
#[test]
fn label_b_and_b_with_escaped_quote_to_trick_naive_string_comparison_with_query_descendant_search_for_b_with_a_leading_quote_with_buffered_input_and_count_result_using_main_engine(
) -> Result<(), Box<dyn Error>> {
    println ! ("on document memchr_trap.toml running the query $..['\"b'] (descendant search for 'b' with a leading quote) with Input impl BufferedInput and result mode CountResult");
    let jsonpath_query = JsonPathQuery::parse("$..['\"b']")?;
    let json_file = fs::File::open("./tests/documents/json/memchr_trap.json")?;
    let input = BufferedInput::new(json_file);
    let engine = MainEngine::compile_query(&jsonpath_query)?;
    let result = engine.run::<_, CountResult>(&input)?;
    assert_eq!(result.get(), 1u64, "result != expected");
    Ok(())
}
#[test]
fn label_b_and_b_with_escaped_quote_to_trick_naive_string_comparison_with_query_descendant_search_for_b_with_a_leading_quote_with_buffered_input_and_count_result_using_recursive_engine(
) -> Result<(), Box<dyn Error>> {
    println ! ("on document memchr_trap.toml running the query $..['\"b'] (descendant search for 'b' with a leading quote) with Input impl BufferedInput and result mode CountResult");
    let jsonpath_query = JsonPathQuery::parse("$..['\"b']")?;
    let json_file = fs::File::open("./tests/documents/json/memchr_trap.json")?;
    let input = BufferedInput::new(json_file);
    let engine = RecursiveEngine::compile_query(&jsonpath_query)?;
    let result = engine.run::<_, CountResult>(&input)?;
    assert_eq!(result.get(), 1u64, "result != expected");
    Ok(())
}
#[test]
fn label_b_and_b_with_escaped_quote_to_trick_naive_string_comparison_with_query_descendant_search_for_b_with_a_leading_quote_with_buffered_input_and_index_result_using_main_engine(
) -> Result<(), Box<dyn Error>> {
    println ! ("on document memchr_trap.toml running the query $..['\"b'] (descendant search for 'b' with a leading quote) with Input impl BufferedInput and result mode IndexResult");
    let jsonpath_query = JsonPathQuery::parse("$..['\"b']")?;
    let json_file = fs::File::open("./tests/documents/json/memchr_trap.json")?;
    let input = BufferedInput::new(json_file);
    let engine = MainEngine::compile_query(&jsonpath_query)?;
    let result = engine.run::<_, IndexResult>(&input)?;
    assert_eq!(result.get(), vec![28usize,], "result != expected");
    Ok(())
}
#[test]
fn label_b_and_b_with_escaped_quote_to_trick_naive_string_comparison_with_query_descendant_search_for_b_with_a_leading_quote_with_buffered_input_and_index_result_using_recursive_engine(
) -> Result<(), Box<dyn Error>> {
    println ! ("on document memchr_trap.toml running the query $..['\"b'] (descendant search for 'b' with a leading quote) with Input impl BufferedInput and result mode IndexResult");
    let jsonpath_query = JsonPathQuery::parse("$..['\"b']")?;
    let json_file = fs::File::open("./tests/documents/json/memchr_trap.json")?;
    let input = BufferedInput::new(json_file);
    let engine = RecursiveEngine::compile_query(&jsonpath_query)?;
    let result = engine.run::<_, IndexResult>(&input)?;
    assert_eq!(result.get(), vec![28usize,], "result != expected");
    Ok(())
}
#[test]
fn label_b_and_b_with_escaped_quote_to_trick_naive_string_comparison_with_query_descendant_search_for_b_with_a_leading_quote_with_mmap_input_and_count_result_using_main_engine(
) -> Result<(), Box<dyn Error>> {
    println ! ("on document memchr_trap.toml running the query $..['\"b'] (descendant search for 'b' with a leading quote) with Input impl MmapInput and result mode CountResult");
    let jsonpath_query = JsonPathQuery::parse("$..['\"b']")?;
    let json_file = fs::File::open("./tests/documents/json/memchr_trap.json")?;
    let input = unsafe { MmapInput::map_file(&json_file)? };
    let engine = MainEngine::compile_query(&jsonpath_query)?;
    let result = engine.run::<_, CountResult>(&input)?;
    assert_eq!(result.get(), 1u64, "result != expected");
    Ok(())
}
#[test]
fn label_b_and_b_with_escaped_quote_to_trick_naive_string_comparison_with_query_descendant_search_for_b_with_a_leading_quote_with_mmap_input_and_count_result_using_recursive_engine(
) -> Result<(), Box<dyn Error>> {
    println ! ("on document memchr_trap.toml running the query $..['\"b'] (descendant search for 'b' with a leading quote) with Input impl MmapInput and result mode CountResult");
    let jsonpath_query = JsonPathQuery::parse("$..['\"b']")?;
    let json_file = fs::File::open("./tests/documents/json/memchr_trap.json")?;
    let input = unsafe { MmapInput::map_file(&json_file)? };
    let engine = RecursiveEngine::compile_query(&jsonpath_query)?;
    let result = engine.run::<_, CountResult>(&input)?;
    assert_eq!(result.get(), 1u64, "result != expected");
    Ok(())
}
#[test]
fn label_b_and_b_with_escaped_quote_to_trick_naive_string_comparison_with_query_descendant_search_for_b_with_a_leading_quote_with_mmap_input_and_index_result_using_main_engine(
) -> Result<(), Box<dyn Error>> {
    println ! ("on document memchr_trap.toml running the query $..['\"b'] (descendant search for 'b' with a leading quote) with Input impl MmapInput and result mode IndexResult");
    let jsonpath_query = JsonPathQuery::parse("$..['\"b']")?;
    let json_file = fs::File::open("./tests/documents/json/memchr_trap.json")?;
    let input = unsafe { MmapInput::map_file(&json_file)? };
    let engine = MainEngine::compile_query(&jsonpath_query)?;
    let result = engine.run::<_, IndexResult>(&input)?;
    assert_eq!(result.get(), vec![28usize,], "result != expected");
    Ok(())
}
#[test]
fn label_b_and_b_with_escaped_quote_to_trick_naive_string_comparison_with_query_descendant_search_for_b_with_a_leading_quote_with_mmap_input_and_index_result_using_recursive_engine(
) -> Result<(), Box<dyn Error>> {
    println ! ("on document memchr_trap.toml running the query $..['\"b'] (descendant search for 'b' with a leading quote) with Input impl MmapInput and result mode IndexResult");
    let jsonpath_query = JsonPathQuery::parse("$..['\"b']")?;
    let json_file = fs::File::open("./tests/documents/json/memchr_trap.json")?;
    let input = unsafe { MmapInput::map_file(&json_file)? };
    let engine = RecursiveEngine::compile_query(&jsonpath_query)?;
    let result = engine.run::<_, IndexResult>(&input)?;
    assert_eq!(result.get(), vec![28usize,], "result != expected");
    Ok(())
}
#[test]
fn label_b_and_b_with_escaped_quote_to_trick_naive_string_comparison_with_query_descendant_search_for_b_with_a_leading_quote_with_owned_bytes_and_count_result_using_main_engine(
) -> Result<(), Box<dyn Error>> {
    println ! ("on document memchr_trap.toml running the query $..['\"b'] (descendant search for 'b' with a leading quote) with Input impl OwnedBytes and result mode CountResult");
    let jsonpath_query = JsonPathQuery::parse("$..['\"b']")?;
    let raw_json = fs::read_to_string("./tests/documents/json/memchr_trap.json")?;
    let input = OwnedBytes::new(&raw_json.as_bytes())?;
    let engine = MainEngine::compile_query(&jsonpath_query)?;
    let result = engine.run::<_, CountResult>(&input)?;
    assert_eq!(result.get(), 1u64, "result != expected");
    Ok(())
}
#[test]
fn label_b_and_b_with_escaped_quote_to_trick_naive_string_comparison_with_query_descendant_search_for_b_with_a_leading_quote_with_owned_bytes_and_count_result_using_recursive_engine(
) -> Result<(), Box<dyn Error>> {
    println ! ("on document memchr_trap.toml running the query $..['\"b'] (descendant search for 'b' with a leading quote) with Input impl OwnedBytes and result mode CountResult");
    let jsonpath_query = JsonPathQuery::parse("$..['\"b']")?;
    let raw_json = fs::read_to_string("./tests/documents/json/memchr_trap.json")?;
    let input = OwnedBytes::new(&raw_json.as_bytes())?;
    let engine = RecursiveEngine::compile_query(&jsonpath_query)?;
    let result = engine.run::<_, CountResult>(&input)?;
    assert_eq!(result.get(), 1u64, "result != expected");
    Ok(())
}
#[test]
fn label_b_and_b_with_escaped_quote_to_trick_naive_string_comparison_with_query_descendant_search_for_b_with_a_leading_quote_with_owned_bytes_and_index_result_using_main_engine(
) -> Result<(), Box<dyn Error>> {
    println ! ("on document memchr_trap.toml running the query $..['\"b'] (descendant search for 'b' with a leading quote) with Input impl OwnedBytes and result mode IndexResult");
    let jsonpath_query = JsonPathQuery::parse("$..['\"b']")?;
    let raw_json = fs::read_to_string("./tests/documents/json/memchr_trap.json")?;
    let input = OwnedBytes::new(&raw_json.as_bytes())?;
    let engine = MainEngine::compile_query(&jsonpath_query)?;
    let result = engine.run::<_, IndexResult>(&input)?;
    assert_eq!(result.get(), vec![28usize,], "result != expected");
    Ok(())
}
#[test]
fn label_b_and_b_with_escaped_quote_to_trick_naive_string_comparison_with_query_descendant_search_for_b_with_a_leading_quote_with_owned_bytes_and_index_result_using_recursive_engine(
) -> Result<(), Box<dyn Error>> {
    println ! ("on document memchr_trap.toml running the query $..['\"b'] (descendant search for 'b' with a leading quote) with Input impl OwnedBytes and result mode IndexResult");
    let jsonpath_query = JsonPathQuery::parse("$..['\"b']")?;
    let raw_json = fs::read_to_string("./tests/documents/json/memchr_trap.json")?;
    let input = OwnedBytes::new(&raw_json.as_bytes())?;
    let engine = RecursiveEngine::compile_query(&jsonpath_query)?;
    let result = engine.run::<_, IndexResult>(&input)?;
    assert_eq!(result.get(), vec![28usize,], "result != expected");
    Ok(())
}
#[test]
fn label_b_and_b_with_escaped_quote_to_trick_naive_string_comparison_with_query_descendant_search_for_b_with_buffered_input_and_count_result_using_main_engine(
) -> Result<(), Box<dyn Error>> {
    println ! ("on document memchr_trap.toml running the query $..b (descendant search for 'b') with Input impl BufferedInput and result mode CountResult");
    let jsonpath_query = JsonPathQuery::parse("$..b")?;
    let json_file = fs::File::open("./tests/documents/json/memchr_trap.json")?;
    let input = BufferedInput::new(json_file);
    let engine = MainEngine::compile_query(&jsonpath_query)?;
    let result = engine.run::<_, CountResult>(&input)?;
    assert_eq!(result.get(), 1u64, "result != expected");
    Ok(())
}
#[test]
fn label_b_and_b_with_escaped_quote_to_trick_naive_string_comparison_with_query_descendant_search_for_b_with_buffered_input_and_count_result_using_recursive_engine(
) -> Result<(), Box<dyn Error>> {
    println ! ("on document memchr_trap.toml running the query $..b (descendant search for 'b') with Input impl BufferedInput and result mode CountResult");
    let jsonpath_query = JsonPathQuery::parse("$..b")?;
    let json_file = fs::File::open("./tests/documents/json/memchr_trap.json")?;
    let input = BufferedInput::new(json_file);
    let engine = RecursiveEngine::compile_query(&jsonpath_query)?;
    let result = engine.run::<_, CountResult>(&input)?;
    assert_eq!(result.get(), 1u64, "result != expected");
    Ok(())
}
#[test]
fn label_b_and_b_with_escaped_quote_to_trick_naive_string_comparison_with_query_descendant_search_for_b_with_buffered_input_and_index_result_using_main_engine(
) -> Result<(), Box<dyn Error>> {
    println ! ("on document memchr_trap.toml running the query $..b (descendant search for 'b') with Input impl BufferedInput and result mode IndexResult");
    let jsonpath_query = JsonPathQuery::parse("$..b")?;
    let json_file = fs::File::open("./tests/documents/json/memchr_trap.json")?;
    let input = BufferedInput::new(json_file);
    let engine = MainEngine::compile_query(&jsonpath_query)?;
    let result = engine.run::<_, IndexResult>(&input)?;
    assert_eq!(result.get(), vec![45usize,], "result != expected");
    Ok(())
}
#[test]
fn label_b_and_b_with_escaped_quote_to_trick_naive_string_comparison_with_query_descendant_search_for_b_with_buffered_input_and_index_result_using_recursive_engine(
) -> Result<(), Box<dyn Error>> {
    println ! ("on document memchr_trap.toml running the query $..b (descendant search for 'b') with Input impl BufferedInput and result mode IndexResult");
    let jsonpath_query = JsonPathQuery::parse("$..b")?;
    let json_file = fs::File::open("./tests/documents/json/memchr_trap.json")?;
    let input = BufferedInput::new(json_file);
    let engine = RecursiveEngine::compile_query(&jsonpath_query)?;
    let result = engine.run::<_, IndexResult>(&input)?;
    assert_eq!(result.get(), vec![45usize,], "result != expected");
    Ok(())
}
#[test]
fn label_b_and_b_with_escaped_quote_to_trick_naive_string_comparison_with_query_descendant_search_for_b_with_mmap_input_and_count_result_using_main_engine(
) -> Result<(), Box<dyn Error>> {
    println ! ("on document memchr_trap.toml running the query $..b (descendant search for 'b') with Input impl MmapInput and result mode CountResult");
    let jsonpath_query = JsonPathQuery::parse("$..b")?;
    let json_file = fs::File::open("./tests/documents/json/memchr_trap.json")?;
    let input = unsafe { MmapInput::map_file(&json_file)? };
    let engine = MainEngine::compile_query(&jsonpath_query)?;
    let result = engine.run::<_, CountResult>(&input)?;
    assert_eq!(result.get(), 1u64, "result != expected");
    Ok(())
}
#[test]
fn label_b_and_b_with_escaped_quote_to_trick_naive_string_comparison_with_query_descendant_search_for_b_with_mmap_input_and_count_result_using_recursive_engine(
) -> Result<(), Box<dyn Error>> {
    println ! ("on document memchr_trap.toml running the query $..b (descendant search for 'b') with Input impl MmapInput and result mode CountResult");
    let jsonpath_query = JsonPathQuery::parse("$..b")?;
    let json_file = fs::File::open("./tests/documents/json/memchr_trap.json")?;
    let input = unsafe { MmapInput::map_file(&json_file)? };
    let engine = RecursiveEngine::compile_query(&jsonpath_query)?;
    let result = engine.run::<_, CountResult>(&input)?;
    assert_eq!(result.get(), 1u64, "result != expected");
    Ok(())
}
#[test]
fn label_b_and_b_with_escaped_quote_to_trick_naive_string_comparison_with_query_descendant_search_for_b_with_mmap_input_and_index_result_using_main_engine(
) -> Result<(), Box<dyn Error>> {
    println ! ("on document memchr_trap.toml running the query $..b (descendant search for 'b') with Input impl MmapInput and result mode IndexResult");
    let jsonpath_query = JsonPathQuery::parse("$..b")?;
    let json_file = fs::File::open("./tests/documents/json/memchr_trap.json")?;
    let input = unsafe { MmapInput::map_file(&json_file)? };
    let engine = MainEngine::compile_query(&jsonpath_query)?;
    let result = engine.run::<_, IndexResult>(&input)?;
    assert_eq!(result.get(), vec![45usize,], "result != expected");
    Ok(())
}
#[test]
fn label_b_and_b_with_escaped_quote_to_trick_naive_string_comparison_with_query_descendant_search_for_b_with_mmap_input_and_index_result_using_recursive_engine(
) -> Result<(), Box<dyn Error>> {
    println ! ("on document memchr_trap.toml running the query $..b (descendant search for 'b') with Input impl MmapInput and result mode IndexResult");
    let jsonpath_query = JsonPathQuery::parse("$..b")?;
    let json_file = fs::File::open("./tests/documents/json/memchr_trap.json")?;
    let input = unsafe { MmapInput::map_file(&json_file)? };
    let engine = RecursiveEngine::compile_query(&jsonpath_query)?;
    let result = engine.run::<_, IndexResult>(&input)?;
    assert_eq!(result.get(), vec![45usize,], "result != expected");
    Ok(())
}
#[test]
fn label_b_and_b_with_escaped_quote_to_trick_naive_string_comparison_with_query_descendant_search_for_b_with_owned_bytes_and_count_result_using_main_engine(
) -> Result<(), Box<dyn Error>> {
    println ! ("on document memchr_trap.toml running the query $..b (descendant search for 'b') with Input impl OwnedBytes and result mode CountResult");
    let jsonpath_query = JsonPathQuery::parse("$..b")?;
    let raw_json = fs::read_to_string("./tests/documents/json/memchr_trap.json")?;
    let input = OwnedBytes::new(&raw_json.as_bytes())?;
    let engine = MainEngine::compile_query(&jsonpath_query)?;
    let result = engine.run::<_, CountResult>(&input)?;
    assert_eq!(result.get(), 1u64, "result != expected");
    Ok(())
}
#[test]
fn label_b_and_b_with_escaped_quote_to_trick_naive_string_comparison_with_query_descendant_search_for_b_with_owned_bytes_and_count_result_using_recursive_engine(
) -> Result<(), Box<dyn Error>> {
    println ! ("on document memchr_trap.toml running the query $..b (descendant search for 'b') with Input impl OwnedBytes and result mode CountResult");
    let jsonpath_query = JsonPathQuery::parse("$..b")?;
    let raw_json = fs::read_to_string("./tests/documents/json/memchr_trap.json")?;
    let input = OwnedBytes::new(&raw_json.as_bytes())?;
    let engine = RecursiveEngine::compile_query(&jsonpath_query)?;
    let result = engine.run::<_, CountResult>(&input)?;
    assert_eq!(result.get(), 1u64, "result != expected");
    Ok(())
}
#[test]
fn label_b_and_b_with_escaped_quote_to_trick_naive_string_comparison_with_query_descendant_search_for_b_with_owned_bytes_and_index_result_using_main_engine(
) -> Result<(), Box<dyn Error>> {
    println ! ("on document memchr_trap.toml running the query $..b (descendant search for 'b') with Input impl OwnedBytes and result mode IndexResult");
    let jsonpath_query = JsonPathQuery::parse("$..b")?;
    let raw_json = fs::read_to_string("./tests/documents/json/memchr_trap.json")?;
    let input = OwnedBytes::new(&raw_json.as_bytes())?;
    let engine = MainEngine::compile_query(&jsonpath_query)?;
    let result = engine.run::<_, IndexResult>(&input)?;
    assert_eq!(result.get(), vec![45usize,], "result != expected");
    Ok(())
}
#[test]
fn label_b_and_b_with_escaped_quote_to_trick_naive_string_comparison_with_query_descendant_search_for_b_with_owned_bytes_and_index_result_using_recursive_engine(
) -> Result<(), Box<dyn Error>> {
    println ! ("on document memchr_trap.toml running the query $..b (descendant search for 'b') with Input impl OwnedBytes and result mode IndexResult");
    let jsonpath_query = JsonPathQuery::parse("$..b")?;
    let raw_json = fs::read_to_string("./tests/documents/json/memchr_trap.json")?;
    let input = OwnedBytes::new(&raw_json.as_bytes())?;
    let engine = RecursiveEngine::compile_query(&jsonpath_query)?;
    let result = engine.run::<_, IndexResult>(&input)?;
    assert_eq!(result.get(), vec![45usize,], "result != expected");
    Ok(())
}
#[test]
fn object_with_two_labels_x_and_x_with_a_preceding_escaped_double_quote_compressed_with_query_select_x_directly_with_buffered_input_and_count_result_using_main_engine(
) -> Result<(), Box<dyn Error>> {
    println ! ("on document quote_escape.toml running the query $['x'] (select 'x' directly) with Input impl BufferedInput and result mode CountResult");
    let jsonpath_query = JsonPathQuery::parse("$['x']")?;
    let json_file = fs::File::open("./tests/documents/json/compressed/quote_escape.json")?;
    let input = BufferedInput::new(json_file);
    let engine = MainEngine::compile_query(&jsonpath_query)?;
    let result = engine.run::<_, CountResult>(&input)?;
    assert_eq!(result.get(), 1u64, "result != expected");
    Ok(())
}
#[test]
fn object_with_two_labels_x_and_x_with_a_preceding_escaped_double_quote_compressed_with_query_select_x_directly_with_buffered_input_and_count_result_using_recursive_engine(
) -> Result<(), Box<dyn Error>> {
    println ! ("on document quote_escape.toml running the query $['x'] (select 'x' directly) with Input impl BufferedInput and result mode CountResult");
    let jsonpath_query = JsonPathQuery::parse("$['x']")?;
    let json_file = fs::File::open("./tests/documents/json/compressed/quote_escape.json")?;
    let input = BufferedInput::new(json_file);
    let engine = RecursiveEngine::compile_query(&jsonpath_query)?;
    let result = engine.run::<_, CountResult>(&input)?;
    assert_eq!(result.get(), 1u64, "result != expected");
    Ok(())
}
#[test]
fn object_with_two_labels_x_and_x_with_a_preceding_escaped_double_quote_compressed_with_query_select_x_directly_with_buffered_input_and_index_result_using_main_engine(
) -> Result<(), Box<dyn Error>> {
    println ! ("on document quote_escape.toml running the query $['x'] (select 'x' directly) with Input impl BufferedInput and result mode IndexResult");
    let jsonpath_query = JsonPathQuery::parse("$['x']")?;
    let json_file = fs::File::open("./tests/documents/json/compressed/quote_escape.json")?;
    let input = BufferedInput::new(json_file);
    let engine = MainEngine::compile_query(&jsonpath_query)?;
    let result = engine.run::<_, IndexResult>(&input)?;
    assert_eq!(result.get(), vec![14usize,], "result != expected");
    Ok(())
}
#[test]
fn object_with_two_labels_x_and_x_with_a_preceding_escaped_double_quote_compressed_with_query_select_x_directly_with_buffered_input_and_index_result_using_recursive_engine(
) -> Result<(), Box<dyn Error>> {
    println ! ("on document quote_escape.toml running the query $['x'] (select 'x' directly) with Input impl BufferedInput and result mode IndexResult");
    let jsonpath_query = JsonPathQuery::parse("$['x']")?;
    let json_file = fs::File::open("./tests/documents/json/compressed/quote_escape.json")?;
    let input = BufferedInput::new(json_file);
    let engine = RecursiveEngine::compile_query(&jsonpath_query)?;
    let result = engine.run::<_, IndexResult>(&input)?;
    assert_eq!(result.get(), vec![14usize,], "result != expected");
    Ok(())
}
#[test]
fn object_with_two_labels_x_and_x_with_a_preceding_escaped_double_quote_compressed_with_query_select_x_directly_with_mmap_input_and_count_result_using_main_engine(
) -> Result<(), Box<dyn Error>> {
    println ! ("on document quote_escape.toml running the query $['x'] (select 'x' directly) with Input impl MmapInput and result mode CountResult");
    let jsonpath_query = JsonPathQuery::parse("$['x']")?;
    let json_file = fs::File::open("./tests/documents/json/compressed/quote_escape.json")?;
    let input = unsafe { MmapInput::map_file(&json_file)? };
    let engine = MainEngine::compile_query(&jsonpath_query)?;
    let result = engine.run::<_, CountResult>(&input)?;
    assert_eq!(result.get(), 1u64, "result != expected");
    Ok(())
}
#[test]
fn object_with_two_labels_x_and_x_with_a_preceding_escaped_double_quote_compressed_with_query_select_x_directly_with_mmap_input_and_count_result_using_recursive_engine(
) -> Result<(), Box<dyn Error>> {
    println ! ("on document quote_escape.toml running the query $['x'] (select 'x' directly) with Input impl MmapInput and result mode CountResult");
    let jsonpath_query = JsonPathQuery::parse("$['x']")?;
    let json_file = fs::File::open("./tests/documents/json/compressed/quote_escape.json")?;
    let input = unsafe { MmapInput::map_file(&json_file)? };
    let engine = RecursiveEngine::compile_query(&jsonpath_query)?;
    let result = engine.run::<_, CountResult>(&input)?;
    assert_eq!(result.get(), 1u64, "result != expected");
    Ok(())
}
#[test]
fn object_with_two_labels_x_and_x_with_a_preceding_escaped_double_quote_compressed_with_query_select_x_directly_with_mmap_input_and_index_result_using_main_engine(
) -> Result<(), Box<dyn Error>> {
    println ! ("on document quote_escape.toml running the query $['x'] (select 'x' directly) with Input impl MmapInput and result mode IndexResult");
    let jsonpath_query = JsonPathQuery::parse("$['x']")?;
    let json_file = fs::File::open("./tests/documents/json/compressed/quote_escape.json")?;
    let input = unsafe { MmapInput::map_file(&json_file)? };
    let engine = MainEngine::compile_query(&jsonpath_query)?;
    let result = engine.run::<_, IndexResult>(&input)?;
    assert_eq!(result.get(), vec![14usize,], "result != expected");
    Ok(())
}
#[test]
fn object_with_two_labels_x_and_x_with_a_preceding_escaped_double_quote_compressed_with_query_select_x_directly_with_mmap_input_and_index_result_using_recursive_engine(
) -> Result<(), Box<dyn Error>> {
    println ! ("on document quote_escape.toml running the query $['x'] (select 'x' directly) with Input impl MmapInput and result mode IndexResult");
    let jsonpath_query = JsonPathQuery::parse("$['x']")?;
    let json_file = fs::File::open("./tests/documents/json/compressed/quote_escape.json")?;
    let input = unsafe { MmapInput::map_file(&json_file)? };
    let engine = RecursiveEngine::compile_query(&jsonpath_query)?;
    let result = engine.run::<_, IndexResult>(&input)?;
    assert_eq!(result.get(), vec![14usize,], "result != expected");
    Ok(())
}
#[test]
fn object_with_two_labels_x_and_x_with_a_preceding_escaped_double_quote_compressed_with_query_select_x_directly_with_owned_bytes_and_count_result_using_main_engine(
) -> Result<(), Box<dyn Error>> {
    println ! ("on document quote_escape.toml running the query $['x'] (select 'x' directly) with Input impl OwnedBytes and result mode CountResult");
    let jsonpath_query = JsonPathQuery::parse("$['x']")?;
    let raw_json = fs::read_to_string("./tests/documents/json/compressed/quote_escape.json")?;
    let input = OwnedBytes::new(&raw_json.as_bytes())?;
    let engine = MainEngine::compile_query(&jsonpath_query)?;
    let result = engine.run::<_, CountResult>(&input)?;
    assert_eq!(result.get(), 1u64, "result != expected");
    Ok(())
}
#[test]
fn object_with_two_labels_x_and_x_with_a_preceding_escaped_double_quote_compressed_with_query_select_x_directly_with_owned_bytes_and_count_result_using_recursive_engine(
) -> Result<(), Box<dyn Error>> {
    println ! ("on document quote_escape.toml running the query $['x'] (select 'x' directly) with Input impl OwnedBytes and result mode CountResult");
    let jsonpath_query = JsonPathQuery::parse("$['x']")?;
    let raw_json = fs::read_to_string("./tests/documents/json/compressed/quote_escape.json")?;
    let input = OwnedBytes::new(&raw_json.as_bytes())?;
    let engine = RecursiveEngine::compile_query(&jsonpath_query)?;
    let result = engine.run::<_, CountResult>(&input)?;
    assert_eq!(result.get(), 1u64, "result != expected");
    Ok(())
}
#[test]
fn object_with_two_labels_x_and_x_with_a_preceding_escaped_double_quote_compressed_with_query_select_x_directly_with_owned_bytes_and_index_result_using_main_engine(
) -> Result<(), Box<dyn Error>> {
    println ! ("on document quote_escape.toml running the query $['x'] (select 'x' directly) with Input impl OwnedBytes and result mode IndexResult");
    let jsonpath_query = JsonPathQuery::parse("$['x']")?;
    let raw_json = fs::read_to_string("./tests/documents/json/compressed/quote_escape.json")?;
    let input = OwnedBytes::new(&raw_json.as_bytes())?;
    let engine = MainEngine::compile_query(&jsonpath_query)?;
    let result = engine.run::<_, IndexResult>(&input)?;
    assert_eq!(result.get(), vec![14usize,], "result != expected");
    Ok(())
}
#[test]
fn object_with_two_labels_x_and_x_with_a_preceding_escaped_double_quote_compressed_with_query_select_x_directly_with_owned_bytes_and_index_result_using_recursive_engine(
) -> Result<(), Box<dyn Error>> {
    println ! ("on document quote_escape.toml running the query $['x'] (select 'x' directly) with Input impl OwnedBytes and result mode IndexResult");
    let jsonpath_query = JsonPathQuery::parse("$['x']")?;
    let raw_json = fs::read_to_string("./tests/documents/json/compressed/quote_escape.json")?;
    let input = OwnedBytes::new(&raw_json.as_bytes())?;
    let engine = RecursiveEngine::compile_query(&jsonpath_query)?;
    let result = engine.run::<_, IndexResult>(&input)?;
    assert_eq!(result.get(), vec![14usize,], "result != expected");
    Ok(())
}
#[test]
fn object_with_two_labels_x_and_x_with_a_preceding_escaped_double_quote_compressed_with_query_select_x_with_quote_directly_with_buffered_input_and_count_result_using_main_engine(
) -> Result<(), Box<dyn Error>> {
    println ! ("on document quote_escape.toml running the query $['\"x'] (select 'x' with quote directly) with Input impl BufferedInput and result mode CountResult");
    let jsonpath_query = JsonPathQuery::parse("$['\"x']")?;
    let json_file = fs::File::open("./tests/documents/json/compressed/quote_escape.json")?;
    let input = BufferedInput::new(json_file);
    let engine = MainEngine::compile_query(&jsonpath_query)?;
    let result = engine.run::<_, CountResult>(&input)?;
    assert_eq!(result.get(), 1u64, "result != expected");
    Ok(())
}
#[test]
fn object_with_two_labels_x_and_x_with_a_preceding_escaped_double_quote_compressed_with_query_select_x_with_quote_directly_with_buffered_input_and_count_result_using_recursive_engine(
) -> Result<(), Box<dyn Error>> {
    println ! ("on document quote_escape.toml running the query $['\"x'] (select 'x' with quote directly) with Input impl BufferedInput and result mode CountResult");
    let jsonpath_query = JsonPathQuery::parse("$['\"x']")?;
    let json_file = fs::File::open("./tests/documents/json/compressed/quote_escape.json")?;
    let input = BufferedInput::new(json_file);
    let engine = RecursiveEngine::compile_query(&jsonpath_query)?;
    let result = engine.run::<_, CountResult>(&input)?;
    assert_eq!(result.get(), 1u64, "result != expected");
    Ok(())
}
#[test]
fn object_with_two_labels_x_and_x_with_a_preceding_escaped_double_quote_compressed_with_query_select_x_with_quote_directly_with_buffered_input_and_index_result_using_main_engine(
) -> Result<(), Box<dyn Error>> {
    println ! ("on document quote_escape.toml running the query $['\"x'] (select 'x' with quote directly) with Input impl BufferedInput and result mode IndexResult");
    let jsonpath_query = JsonPathQuery::parse("$['\"x']")?;
    let json_file = fs::File::open("./tests/documents/json/compressed/quote_escape.json")?;
    let input = BufferedInput::new(json_file);
    let engine = MainEngine::compile_query(&jsonpath_query)?;
    let result = engine.run::<_, IndexResult>(&input)?;
    assert_eq!(result.get(), vec![7usize,], "result != expected");
    Ok(())
}
#[test]
fn object_with_two_labels_x_and_x_with_a_preceding_escaped_double_quote_compressed_with_query_select_x_with_quote_directly_with_buffered_input_and_index_result_using_recursive_engine(
) -> Result<(), Box<dyn Error>> {
    println ! ("on document quote_escape.toml running the query $['\"x'] (select 'x' with quote directly) with Input impl BufferedInput and result mode IndexResult");
    let jsonpath_query = JsonPathQuery::parse("$['\"x']")?;
    let json_file = fs::File::open("./tests/documents/json/compressed/quote_escape.json")?;
    let input = BufferedInput::new(json_file);
    let engine = RecursiveEngine::compile_query(&jsonpath_query)?;
    let result = engine.run::<_, IndexResult>(&input)?;
    assert_eq!(result.get(), vec![7usize,], "result != expected");
    Ok(())
}
#[test]
fn object_with_two_labels_x_and_x_with_a_preceding_escaped_double_quote_compressed_with_query_select_x_with_quote_directly_with_mmap_input_and_count_result_using_main_engine(
) -> Result<(), Box<dyn Error>> {
    println ! ("on document quote_escape.toml running the query $['\"x'] (select 'x' with quote directly) with Input impl MmapInput and result mode CountResult");
    let jsonpath_query = JsonPathQuery::parse("$['\"x']")?;
    let json_file = fs::File::open("./tests/documents/json/compressed/quote_escape.json")?;
    let input = unsafe { MmapInput::map_file(&json_file)? };
    let engine = MainEngine::compile_query(&jsonpath_query)?;
    let result = engine.run::<_, CountResult>(&input)?;
    assert_eq!(result.get(), 1u64, "result != expected");
    Ok(())
}
#[test]
fn object_with_two_labels_x_and_x_with_a_preceding_escaped_double_quote_compressed_with_query_select_x_with_quote_directly_with_mmap_input_and_count_result_using_recursive_engine(
) -> Result<(), Box<dyn Error>> {
    println ! ("on document quote_escape.toml running the query $['\"x'] (select 'x' with quote directly) with Input impl MmapInput and result mode CountResult");
    let jsonpath_query = JsonPathQuery::parse("$['\"x']")?;
    let json_file = fs::File::open("./tests/documents/json/compressed/quote_escape.json")?;
    let input = unsafe { MmapInput::map_file(&json_file)? };
    let engine = RecursiveEngine::compile_query(&jsonpath_query)?;
    let result = engine.run::<_, CountResult>(&input)?;
    assert_eq!(result.get(), 1u64, "result != expected");
    Ok(())
}
#[test]
fn object_with_two_labels_x_and_x_with_a_preceding_escaped_double_quote_compressed_with_query_select_x_with_quote_directly_with_mmap_input_and_index_result_using_main_engine(
) -> Result<(), Box<dyn Error>> {
    println ! ("on document quote_escape.toml running the query $['\"x'] (select 'x' with quote directly) with Input impl MmapInput and result mode IndexResult");
    let jsonpath_query = JsonPathQuery::parse("$['\"x']")?;
    let json_file = fs::File::open("./tests/documents/json/compressed/quote_escape.json")?;
    let input = unsafe { MmapInput::map_file(&json_file)? };
    let engine = MainEngine::compile_query(&jsonpath_query)?;
    let result = engine.run::<_, IndexResult>(&input)?;
    assert_eq!(result.get(), vec![7usize,], "result != expected");
    Ok(())
}
#[test]
fn object_with_two_labels_x_and_x_with_a_preceding_escaped_double_quote_compressed_with_query_select_x_with_quote_directly_with_mmap_input_and_index_result_using_recursive_engine(
) -> Result<(), Box<dyn Error>> {
    println ! ("on document quote_escape.toml running the query $['\"x'] (select 'x' with quote directly) with Input impl MmapInput and result mode IndexResult");
    let jsonpath_query = JsonPathQuery::parse("$['\"x']")?;
    let json_file = fs::File::open("./tests/documents/json/compressed/quote_escape.json")?;
    let input = unsafe { MmapInput::map_file(&json_file)? };
    let engine = RecursiveEngine::compile_query(&jsonpath_query)?;
    let result = engine.run::<_, IndexResult>(&input)?;
    assert_eq!(result.get(), vec![7usize,], "result != expected");
    Ok(())
}
#[test]
fn object_with_two_labels_x_and_x_with_a_preceding_escaped_double_quote_compressed_with_query_select_x_with_quote_directly_with_owned_bytes_and_count_result_using_main_engine(
) -> Result<(), Box<dyn Error>> {
    println ! ("on document quote_escape.toml running the query $['\"x'] (select 'x' with quote directly) with Input impl OwnedBytes and result mode CountResult");
    let jsonpath_query = JsonPathQuery::parse("$['\"x']")?;
    let raw_json = fs::read_to_string("./tests/documents/json/compressed/quote_escape.json")?;
    let input = OwnedBytes::new(&raw_json.as_bytes())?;
    let engine = MainEngine::compile_query(&jsonpath_query)?;
    let result = engine.run::<_, CountResult>(&input)?;
    assert_eq!(result.get(), 1u64, "result != expected");
    Ok(())
}
#[test]
fn object_with_two_labels_x_and_x_with_a_preceding_escaped_double_quote_compressed_with_query_select_x_with_quote_directly_with_owned_bytes_and_count_result_using_recursive_engine(
) -> Result<(), Box<dyn Error>> {
    println ! ("on document quote_escape.toml running the query $['\"x'] (select 'x' with quote directly) with Input impl OwnedBytes and result mode CountResult");
    let jsonpath_query = JsonPathQuery::parse("$['\"x']")?;
    let raw_json = fs::read_to_string("./tests/documents/json/compressed/quote_escape.json")?;
    let input = OwnedBytes::new(&raw_json.as_bytes())?;
    let engine = RecursiveEngine::compile_query(&jsonpath_query)?;
    let result = engine.run::<_, CountResult>(&input)?;
    assert_eq!(result.get(), 1u64, "result != expected");
    Ok(())
}
#[test]
fn object_with_two_labels_x_and_x_with_a_preceding_escaped_double_quote_compressed_with_query_select_x_with_quote_directly_with_owned_bytes_and_index_result_using_main_engine(
) -> Result<(), Box<dyn Error>> {
    println ! ("on document quote_escape.toml running the query $['\"x'] (select 'x' with quote directly) with Input impl OwnedBytes and result mode IndexResult");
    let jsonpath_query = JsonPathQuery::parse("$['\"x']")?;
    let raw_json = fs::read_to_string("./tests/documents/json/compressed/quote_escape.json")?;
    let input = OwnedBytes::new(&raw_json.as_bytes())?;
    let engine = MainEngine::compile_query(&jsonpath_query)?;
    let result = engine.run::<_, IndexResult>(&input)?;
    assert_eq!(result.get(), vec![7usize,], "result != expected");
    Ok(())
}
#[test]
fn object_with_two_labels_x_and_x_with_a_preceding_escaped_double_quote_compressed_with_query_select_x_with_quote_directly_with_owned_bytes_and_index_result_using_recursive_engine(
) -> Result<(), Box<dyn Error>> {
    println ! ("on document quote_escape.toml running the query $['\"x'] (select 'x' with quote directly) with Input impl OwnedBytes and result mode IndexResult");
    let jsonpath_query = JsonPathQuery::parse("$['\"x']")?;
    let raw_json = fs::read_to_string("./tests/documents/json/compressed/quote_escape.json")?;
    let input = OwnedBytes::new(&raw_json.as_bytes())?;
    let engine = RecursiveEngine::compile_query(&jsonpath_query)?;
    let result = engine.run::<_, IndexResult>(&input)?;
    assert_eq!(result.get(), vec![7usize,], "result != expected");
    Ok(())
}
#[test]
fn object_with_two_labels_x_and_x_with_a_preceding_escaped_double_quote_with_query_select_x_directly_with_buffered_input_and_count_result_using_main_engine(
) -> Result<(), Box<dyn Error>> {
    println ! ("on document quote_escape.toml running the query $['x'] (select 'x' directly) with Input impl BufferedInput and result mode CountResult");
    let jsonpath_query = JsonPathQuery::parse("$['x']")?;
    let json_file = fs::File::open("./tests/documents/json/quote_escape.json")?;
    let input = BufferedInput::new(json_file);
    let engine = MainEngine::compile_query(&jsonpath_query)?;
    let result = engine.run::<_, CountResult>(&input)?;
    assert_eq!(result.get(), 1u64, "result != expected");
    Ok(())
}
#[test]
fn object_with_two_labels_x_and_x_with_a_preceding_escaped_double_quote_with_query_select_x_directly_with_buffered_input_and_count_result_using_recursive_engine(
) -> Result<(), Box<dyn Error>> {
    println ! ("on document quote_escape.toml running the query $['x'] (select 'x' directly) with Input impl BufferedInput and result mode CountResult");
    let jsonpath_query = JsonPathQuery::parse("$['x']")?;
    let json_file = fs::File::open("./tests/documents/json/quote_escape.json")?;
    let input = BufferedInput::new(json_file);
    let engine = RecursiveEngine::compile_query(&jsonpath_query)?;
    let result = engine.run::<_, CountResult>(&input)?;
    assert_eq!(result.get(), 1u64, "result != expected");
    Ok(())
}
#[test]
fn object_with_two_labels_x_and_x_with_a_preceding_escaped_double_quote_with_query_select_x_directly_with_buffered_input_and_index_result_using_main_engine(
) -> Result<(), Box<dyn Error>> {
    println ! ("on document quote_escape.toml running the query $['x'] (select 'x' directly) with Input impl BufferedInput and result mode IndexResult");
    let jsonpath_query = JsonPathQuery::parse("$['x']")?;
    let json_file = fs::File::open("./tests/documents/json/quote_escape.json")?;
    let input = BufferedInput::new(json_file);
    let engine = MainEngine::compile_query(&jsonpath_query)?;
    let result = engine.run::<_, IndexResult>(&input)?;
    assert_eq!(result.get(), vec![26usize,], "result != expected");
    Ok(())
}
#[test]
fn object_with_two_labels_x_and_x_with_a_preceding_escaped_double_quote_with_query_select_x_directly_with_buffered_input_and_index_result_using_recursive_engine(
) -> Result<(), Box<dyn Error>> {
    println ! ("on document quote_escape.toml running the query $['x'] (select 'x' directly) with Input impl BufferedInput and result mode IndexResult");
    let jsonpath_query = JsonPathQuery::parse("$['x']")?;
    let json_file = fs::File::open("./tests/documents/json/quote_escape.json")?;
    let input = BufferedInput::new(json_file);
    let engine = RecursiveEngine::compile_query(&jsonpath_query)?;
    let result = engine.run::<_, IndexResult>(&input)?;
    assert_eq!(result.get(), vec![26usize,], "result != expected");
    Ok(())
}
#[test]
fn object_with_two_labels_x_and_x_with_a_preceding_escaped_double_quote_with_query_select_x_directly_with_mmap_input_and_count_result_using_main_engine(
) -> Result<(), Box<dyn Error>> {
    println ! ("on document quote_escape.toml running the query $['x'] (select 'x' directly) with Input impl MmapInput and result mode CountResult");
    let jsonpath_query = JsonPathQuery::parse("$['x']")?;
    let json_file = fs::File::open("./tests/documents/json/quote_escape.json")?;
    let input = unsafe { MmapInput::map_file(&json_file)? };
    let engine = MainEngine::compile_query(&jsonpath_query)?;
    let result = engine.run::<_, CountResult>(&input)?;
    assert_eq!(result.get(), 1u64, "result != expected");
    Ok(())
}
#[test]
fn object_with_two_labels_x_and_x_with_a_preceding_escaped_double_quote_with_query_select_x_directly_with_mmap_input_and_count_result_using_recursive_engine(
) -> Result<(), Box<dyn Error>> {
    println ! ("on document quote_escape.toml running the query $['x'] (select 'x' directly) with Input impl MmapInput and result mode CountResult");
    let jsonpath_query = JsonPathQuery::parse("$['x']")?;
    let json_file = fs::File::open("./tests/documents/json/quote_escape.json")?;
    let input = unsafe { MmapInput::map_file(&json_file)? };
    let engine = RecursiveEngine::compile_query(&jsonpath_query)?;
    let result = engine.run::<_, CountResult>(&input)?;
    assert_eq!(result.get(), 1u64, "result != expected");
    Ok(())
}
#[test]
fn object_with_two_labels_x_and_x_with_a_preceding_escaped_double_quote_with_query_select_x_directly_with_mmap_input_and_index_result_using_main_engine(
) -> Result<(), Box<dyn Error>> {
    println ! ("on document quote_escape.toml running the query $['x'] (select 'x' directly) with Input impl MmapInput and result mode IndexResult");
    let jsonpath_query = JsonPathQuery::parse("$['x']")?;
    let json_file = fs::File::open("./tests/documents/json/quote_escape.json")?;
    let input = unsafe { MmapInput::map_file(&json_file)? };
    let engine = MainEngine::compile_query(&jsonpath_query)?;
    let result = engine.run::<_, IndexResult>(&input)?;
    assert_eq!(result.get(), vec![26usize,], "result != expected");
    Ok(())
}
#[test]
fn object_with_two_labels_x_and_x_with_a_preceding_escaped_double_quote_with_query_select_x_directly_with_mmap_input_and_index_result_using_recursive_engine(
) -> Result<(), Box<dyn Error>> {
    println ! ("on document quote_escape.toml running the query $['x'] (select 'x' directly) with Input impl MmapInput and result mode IndexResult");
    let jsonpath_query = JsonPathQuery::parse("$['x']")?;
    let json_file = fs::File::open("./tests/documents/json/quote_escape.json")?;
    let input = unsafe { MmapInput::map_file(&json_file)? };
    let engine = RecursiveEngine::compile_query(&jsonpath_query)?;
    let result = engine.run::<_, IndexResult>(&input)?;
    assert_eq!(result.get(), vec![26usize,], "result != expected");
    Ok(())
}
#[test]
fn object_with_two_labels_x_and_x_with_a_preceding_escaped_double_quote_with_query_select_x_directly_with_owned_bytes_and_count_result_using_main_engine(
) -> Result<(), Box<dyn Error>> {
    println ! ("on document quote_escape.toml running the query $['x'] (select 'x' directly) with Input impl OwnedBytes and result mode CountResult");
    let jsonpath_query = JsonPathQuery::parse("$['x']")?;
    let raw_json = fs::read_to_string("./tests/documents/json/quote_escape.json")?;
    let input = OwnedBytes::new(&raw_json.as_bytes())?;
    let engine = MainEngine::compile_query(&jsonpath_query)?;
    let result = engine.run::<_, CountResult>(&input)?;
    assert_eq!(result.get(), 1u64, "result != expected");
    Ok(())
}
#[test]
fn object_with_two_labels_x_and_x_with_a_preceding_escaped_double_quote_with_query_select_x_directly_with_owned_bytes_and_count_result_using_recursive_engine(
) -> Result<(), Box<dyn Error>> {
    println ! ("on document quote_escape.toml running the query $['x'] (select 'x' directly) with Input impl OwnedBytes and result mode CountResult");
    let jsonpath_query = JsonPathQuery::parse("$['x']")?;
    let raw_json = fs::read_to_string("./tests/documents/json/quote_escape.json")?;
    let input = OwnedBytes::new(&raw_json.as_bytes())?;
    let engine = RecursiveEngine::compile_query(&jsonpath_query)?;
    let result = engine.run::<_, CountResult>(&input)?;
    assert_eq!(result.get(), 1u64, "result != expected");
    Ok(())
}
#[test]
fn object_with_two_labels_x_and_x_with_a_preceding_escaped_double_quote_with_query_select_x_directly_with_owned_bytes_and_index_result_using_main_engine(
) -> Result<(), Box<dyn Error>> {
    println ! ("on document quote_escape.toml running the query $['x'] (select 'x' directly) with Input impl OwnedBytes and result mode IndexResult");
    let jsonpath_query = JsonPathQuery::parse("$['x']")?;
    let raw_json = fs::read_to_string("./tests/documents/json/quote_escape.json")?;
    let input = OwnedBytes::new(&raw_json.as_bytes())?;
    let engine = MainEngine::compile_query(&jsonpath_query)?;
    let result = engine.run::<_, IndexResult>(&input)?;
    assert_eq!(result.get(), vec![26usize,], "result != expected");
    Ok(())
}
#[test]
fn object_with_two_labels_x_and_x_with_a_preceding_escaped_double_quote_with_query_select_x_directly_with_owned_bytes_and_index_result_using_recursive_engine(
) -> Result<(), Box<dyn Error>> {
    println ! ("on document quote_escape.toml running the query $['x'] (select 'x' directly) with Input impl OwnedBytes and result mode IndexResult");
    let jsonpath_query = JsonPathQuery::parse("$['x']")?;
    let raw_json = fs::read_to_string("./tests/documents/json/quote_escape.json")?;
    let input = OwnedBytes::new(&raw_json.as_bytes())?;
    let engine = RecursiveEngine::compile_query(&jsonpath_query)?;
    let result = engine.run::<_, IndexResult>(&input)?;
    assert_eq!(result.get(), vec![26usize,], "result != expected");
    Ok(())
}
#[test]
fn object_with_two_labels_x_and_x_with_a_preceding_escaped_double_quote_with_query_select_x_with_quote_directly_with_buffered_input_and_count_result_using_main_engine(
) -> Result<(), Box<dyn Error>> {
    println ! ("on document quote_escape.toml running the query $['\"x'] (select 'x' with quote directly) with Input impl BufferedInput and result mode CountResult");
    let jsonpath_query = JsonPathQuery::parse("$['\"x']")?;
    let json_file = fs::File::open("./tests/documents/json/quote_escape.json")?;
    let input = BufferedInput::new(json_file);
    let engine = MainEngine::compile_query(&jsonpath_query)?;
    let result = engine.run::<_, CountResult>(&input)?;
    assert_eq!(result.get(), 1u64, "result != expected");
    Ok(())
}
#[test]
fn object_with_two_labels_x_and_x_with_a_preceding_escaped_double_quote_with_query_select_x_with_quote_directly_with_buffered_input_and_count_result_using_recursive_engine(
) -> Result<(), Box<dyn Error>> {
    println ! ("on document quote_escape.toml running the query $['\"x'] (select 'x' with quote directly) with Input impl BufferedInput and result mode CountResult");
    let jsonpath_query = JsonPathQuery::parse("$['\"x']")?;
    let json_file = fs::File::open("./tests/documents/json/quote_escape.json")?;
    let input = BufferedInput::new(json_file);
    let engine = RecursiveEngine::compile_query(&jsonpath_query)?;
    let result = engine.run::<_, CountResult>(&input)?;
    assert_eq!(result.get(), 1u64, "result != expected");
    Ok(())
}
#[test]
fn object_with_two_labels_x_and_x_with_a_preceding_escaped_double_quote_with_query_select_x_with_quote_directly_with_buffered_input_and_index_result_using_main_engine(
) -> Result<(), Box<dyn Error>> {
    println ! ("on document quote_escape.toml running the query $['\"x'] (select 'x' with quote directly) with Input impl BufferedInput and result mode IndexResult");
    let jsonpath_query = JsonPathQuery::parse("$['\"x']")?;
    let json_file = fs::File::open("./tests/documents/json/quote_escape.json")?;
    let input = BufferedInput::new(json_file);
    let engine = MainEngine::compile_query(&jsonpath_query)?;
    let result = engine.run::<_, IndexResult>(&input)?;
    assert_eq!(result.get(), vec![13usize,], "result != expected");
    Ok(())
}
#[test]
fn object_with_two_labels_x_and_x_with_a_preceding_escaped_double_quote_with_query_select_x_with_quote_directly_with_buffered_input_and_index_result_using_recursive_engine(
) -> Result<(), Box<dyn Error>> {
    println ! ("on document quote_escape.toml running the query $['\"x'] (select 'x' with quote directly) with Input impl BufferedInput and result mode IndexResult");
    let jsonpath_query = JsonPathQuery::parse("$['\"x']")?;
    let json_file = fs::File::open("./tests/documents/json/quote_escape.json")?;
    let input = BufferedInput::new(json_file);
    let engine = RecursiveEngine::compile_query(&jsonpath_query)?;
    let result = engine.run::<_, IndexResult>(&input)?;
    assert_eq!(result.get(), vec![13usize,], "result != expected");
    Ok(())
}
#[test]
fn object_with_two_labels_x_and_x_with_a_preceding_escaped_double_quote_with_query_select_x_with_quote_directly_with_mmap_input_and_count_result_using_main_engine(
) -> Result<(), Box<dyn Error>> {
    println ! ("on document quote_escape.toml running the query $['\"x'] (select 'x' with quote directly) with Input impl MmapInput and result mode CountResult");
    let jsonpath_query = JsonPathQuery::parse("$['\"x']")?;
    let json_file = fs::File::open("./tests/documents/json/quote_escape.json")?;
    let input = unsafe { MmapInput::map_file(&json_file)? };
    let engine = MainEngine::compile_query(&jsonpath_query)?;
    let result = engine.run::<_, CountResult>(&input)?;
    assert_eq!(result.get(), 1u64, "result != expected");
    Ok(())
}
#[test]
fn object_with_two_labels_x_and_x_with_a_preceding_escaped_double_quote_with_query_select_x_with_quote_directly_with_mmap_input_and_count_result_using_recursive_engine(
) -> Result<(), Box<dyn Error>> {
    println ! ("on document quote_escape.toml running the query $['\"x'] (select 'x' with quote directly) with Input impl MmapInput and result mode CountResult");
    let jsonpath_query = JsonPathQuery::parse("$['\"x']")?;
    let json_file = fs::File::open("./tests/documents/json/quote_escape.json")?;
    let input = unsafe { MmapInput::map_file(&json_file)? };
    let engine = RecursiveEngine::compile_query(&jsonpath_query)?;
    let result = engine.run::<_, CountResult>(&input)?;
    assert_eq!(result.get(), 1u64, "result != expected");
    Ok(())
}
#[test]
fn object_with_two_labels_x_and_x_with_a_preceding_escaped_double_quote_with_query_select_x_with_quote_directly_with_mmap_input_and_index_result_using_main_engine(
) -> Result<(), Box<dyn Error>> {
    println ! ("on document quote_escape.toml running the query $['\"x'] (select 'x' with quote directly) with Input impl MmapInput and result mode IndexResult");
    let jsonpath_query = JsonPathQuery::parse("$['\"x']")?;
    let json_file = fs::File::open("./tests/documents/json/quote_escape.json")?;
    let input = unsafe { MmapInput::map_file(&json_file)? };
    let engine = MainEngine::compile_query(&jsonpath_query)?;
    let result = engine.run::<_, IndexResult>(&input)?;
    assert_eq!(result.get(), vec![13usize,], "result != expected");
    Ok(())
}
#[test]
fn object_with_two_labels_x_and_x_with_a_preceding_escaped_double_quote_with_query_select_x_with_quote_directly_with_mmap_input_and_index_result_using_recursive_engine(
) -> Result<(), Box<dyn Error>> {
    println ! ("on document quote_escape.toml running the query $['\"x'] (select 'x' with quote directly) with Input impl MmapInput and result mode IndexResult");
    let jsonpath_query = JsonPathQuery::parse("$['\"x']")?;
    let json_file = fs::File::open("./tests/documents/json/quote_escape.json")?;
    let input = unsafe { MmapInput::map_file(&json_file)? };
    let engine = RecursiveEngine::compile_query(&jsonpath_query)?;
    let result = engine.run::<_, IndexResult>(&input)?;
    assert_eq!(result.get(), vec![13usize,], "result != expected");
    Ok(())
}
#[test]
fn object_with_two_labels_x_and_x_with_a_preceding_escaped_double_quote_with_query_select_x_with_quote_directly_with_owned_bytes_and_count_result_using_main_engine(
) -> Result<(), Box<dyn Error>> {
    println ! ("on document quote_escape.toml running the query $['\"x'] (select 'x' with quote directly) with Input impl OwnedBytes and result mode CountResult");
    let jsonpath_query = JsonPathQuery::parse("$['\"x']")?;
    let raw_json = fs::read_to_string("./tests/documents/json/quote_escape.json")?;
    let input = OwnedBytes::new(&raw_json.as_bytes())?;
    let engine = MainEngine::compile_query(&jsonpath_query)?;
    let result = engine.run::<_, CountResult>(&input)?;
    assert_eq!(result.get(), 1u64, "result != expected");
    Ok(())
}
#[test]
fn object_with_two_labels_x_and_x_with_a_preceding_escaped_double_quote_with_query_select_x_with_quote_directly_with_owned_bytes_and_count_result_using_recursive_engine(
) -> Result<(), Box<dyn Error>> {
    println ! ("on document quote_escape.toml running the query $['\"x'] (select 'x' with quote directly) with Input impl OwnedBytes and result mode CountResult");
    let jsonpath_query = JsonPathQuery::parse("$['\"x']")?;
    let raw_json = fs::read_to_string("./tests/documents/json/quote_escape.json")?;
    let input = OwnedBytes::new(&raw_json.as_bytes())?;
    let engine = RecursiveEngine::compile_query(&jsonpath_query)?;
    let result = engine.run::<_, CountResult>(&input)?;
    assert_eq!(result.get(), 1u64, "result != expected");
    Ok(())
}
#[test]
fn object_with_two_labels_x_and_x_with_a_preceding_escaped_double_quote_with_query_select_x_with_quote_directly_with_owned_bytes_and_index_result_using_main_engine(
) -> Result<(), Box<dyn Error>> {
    println ! ("on document quote_escape.toml running the query $['\"x'] (select 'x' with quote directly) with Input impl OwnedBytes and result mode IndexResult");
    let jsonpath_query = JsonPathQuery::parse("$['\"x']")?;
    let raw_json = fs::read_to_string("./tests/documents/json/quote_escape.json")?;
    let input = OwnedBytes::new(&raw_json.as_bytes())?;
    let engine = MainEngine::compile_query(&jsonpath_query)?;
    let result = engine.run::<_, IndexResult>(&input)?;
    assert_eq!(result.get(), vec![13usize,], "result != expected");
    Ok(())
}
#[test]
fn object_with_two_labels_x_and_x_with_a_preceding_escaped_double_quote_with_query_select_x_with_quote_directly_with_owned_bytes_and_index_result_using_recursive_engine(
) -> Result<(), Box<dyn Error>> {
    println ! ("on document quote_escape.toml running the query $['\"x'] (select 'x' with quote directly) with Input impl OwnedBytes and result mode IndexResult");
    let jsonpath_query = JsonPathQuery::parse("$['\"x']")?;
    let raw_json = fs::read_to_string("./tests/documents/json/quote_escape.json")?;
    let input = OwnedBytes::new(&raw_json.as_bytes())?;
    let engine = RecursiveEngine::compile_query(&jsonpath_query)?;
    let result = engine.run::<_, IndexResult>(&input)?;
    assert_eq!(result.get(), vec![13usize,], "result != expected");
    Ok(())
}
#[test]
fn short_json_with_objects_and_lists_given_as_an_example_on_jsonpath_com_compressed_with_query_descendant_search_for_number_with_buffered_input_and_count_result_using_main_engine(
) -> Result<(), Box<dyn Error>> {
    println ! ("on document jsonpath_com_example.toml running the query $..number (descendant search for 'number') with Input impl BufferedInput and result mode CountResult");
    let jsonpath_query = JsonPathQuery::parse("$..number")?;
    let json_file = fs::File::open("./tests/documents/json/compressed/jsonpath_com_example.json")?;
    let input = BufferedInput::new(json_file);
    let engine = MainEngine::compile_query(&jsonpath_query)?;
    let result = engine.run::<_, CountResult>(&input)?;
    assert_eq!(result.get(), 2u64, "result != expected");
    Ok(())
}
#[test]
fn short_json_with_objects_and_lists_given_as_an_example_on_jsonpath_com_compressed_with_query_descendant_search_for_number_with_buffered_input_and_count_result_using_recursive_engine(
) -> Result<(), Box<dyn Error>> {
    println ! ("on document jsonpath_com_example.toml running the query $..number (descendant search for 'number') with Input impl BufferedInput and result mode CountResult");
    let jsonpath_query = JsonPathQuery::parse("$..number")?;
    let json_file = fs::File::open("./tests/documents/json/compressed/jsonpath_com_example.json")?;
    let input = BufferedInput::new(json_file);
    let engine = RecursiveEngine::compile_query(&jsonpath_query)?;
    let result = engine.run::<_, CountResult>(&input)?;
    assert_eq!(result.get(), 2u64, "result != expected");
    Ok(())
}
#[test]
fn short_json_with_objects_and_lists_given_as_an_example_on_jsonpath_com_compressed_with_query_descendant_search_for_number_with_buffered_input_and_index_result_using_main_engine(
) -> Result<(), Box<dyn Error>> {
    println ! ("on document jsonpath_com_example.toml running the query $..number (descendant search for 'number') with Input impl BufferedInput and result mode IndexResult");
    let jsonpath_query = JsonPathQuery::parse("$..number")?;
    let json_file = fs::File::open("./tests/documents/json/compressed/jsonpath_com_example.json")?;
    let input = BufferedInput::new(json_file);
    let engine = MainEngine::compile_query(&jsonpath_query)?;
    let result = engine.run::<_, IndexResult>(&input)?;
    assert_eq!(result.get(), vec![169usize, 211usize,], "result != expected");
    Ok(())
}
#[test]
fn short_json_with_objects_and_lists_given_as_an_example_on_jsonpath_com_compressed_with_query_descendant_search_for_number_with_buffered_input_and_index_result_using_recursive_engine(
) -> Result<(), Box<dyn Error>> {
    println ! ("on document jsonpath_com_example.toml running the query $..number (descendant search for 'number') with Input impl BufferedInput and result mode IndexResult");
    let jsonpath_query = JsonPathQuery::parse("$..number")?;
    let json_file = fs::File::open("./tests/documents/json/compressed/jsonpath_com_example.json")?;
    let input = BufferedInput::new(json_file);
    let engine = RecursiveEngine::compile_query(&jsonpath_query)?;
    let result = engine.run::<_, IndexResult>(&input)?;
    assert_eq!(result.get(), vec![169usize, 211usize,], "result != expected");
    Ok(())
}
#[test]
fn short_json_with_objects_and_lists_given_as_an_example_on_jsonpath_com_compressed_with_query_descendant_search_for_number_with_mmap_input_and_count_result_using_main_engine(
) -> Result<(), Box<dyn Error>> {
    println ! ("on document jsonpath_com_example.toml running the query $..number (descendant search for 'number') with Input impl MmapInput and result mode CountResult");
    let jsonpath_query = JsonPathQuery::parse("$..number")?;
    let json_file = fs::File::open("./tests/documents/json/compressed/jsonpath_com_example.json")?;
    let input = unsafe { MmapInput::map_file(&json_file)? };
    let engine = MainEngine::compile_query(&jsonpath_query)?;
    let result = engine.run::<_, CountResult>(&input)?;
    assert_eq!(result.get(), 2u64, "result != expected");
    Ok(())
}
#[test]
fn short_json_with_objects_and_lists_given_as_an_example_on_jsonpath_com_compressed_with_query_descendant_search_for_number_with_mmap_input_and_count_result_using_recursive_engine(
) -> Result<(), Box<dyn Error>> {
    println ! ("on document jsonpath_com_example.toml running the query $..number (descendant search for 'number') with Input impl MmapInput and result mode CountResult");
    let jsonpath_query = JsonPathQuery::parse("$..number")?;
    let json_file = fs::File::open("./tests/documents/json/compressed/jsonpath_com_example.json")?;
    let input = unsafe { MmapInput::map_file(&json_file)? };
    let engine = RecursiveEngine::compile_query(&jsonpath_query)?;
    let result = engine.run::<_, CountResult>(&input)?;
    assert_eq!(result.get(), 2u64, "result != expected");
    Ok(())
}
#[test]
fn short_json_with_objects_and_lists_given_as_an_example_on_jsonpath_com_compressed_with_query_descendant_search_for_number_with_mmap_input_and_index_result_using_main_engine(
) -> Result<(), Box<dyn Error>> {
    println ! ("on document jsonpath_com_example.toml running the query $..number (descendant search for 'number') with Input impl MmapInput and result mode IndexResult");
    let jsonpath_query = JsonPathQuery::parse("$..number")?;
    let json_file = fs::File::open("./tests/documents/json/compressed/jsonpath_com_example.json")?;
    let input = unsafe { MmapInput::map_file(&json_file)? };
    let engine = MainEngine::compile_query(&jsonpath_query)?;
    let result = engine.run::<_, IndexResult>(&input)?;
    assert_eq!(result.get(), vec![169usize, 211usize,], "result != expected");
    Ok(())
}
#[test]
fn short_json_with_objects_and_lists_given_as_an_example_on_jsonpath_com_compressed_with_query_descendant_search_for_number_with_mmap_input_and_index_result_using_recursive_engine(
) -> Result<(), Box<dyn Error>> {
    println ! ("on document jsonpath_com_example.toml running the query $..number (descendant search for 'number') with Input impl MmapInput and result mode IndexResult");
    let jsonpath_query = JsonPathQuery::parse("$..number")?;
    let json_file = fs::File::open("./tests/documents/json/compressed/jsonpath_com_example.json")?;
    let input = unsafe { MmapInput::map_file(&json_file)? };
    let engine = RecursiveEngine::compile_query(&jsonpath_query)?;
    let result = engine.run::<_, IndexResult>(&input)?;
    assert_eq!(result.get(), vec![169usize, 211usize,], "result != expected");
    Ok(())
}
#[test]
fn short_json_with_objects_and_lists_given_as_an_example_on_jsonpath_com_compressed_with_query_descendant_search_for_number_with_owned_bytes_and_count_result_using_main_engine(
) -> Result<(), Box<dyn Error>> {
    println ! ("on document jsonpath_com_example.toml running the query $..number (descendant search for 'number') with Input impl OwnedBytes and result mode CountResult");
    let jsonpath_query = JsonPathQuery::parse("$..number")?;
    let raw_json = fs::read_to_string("./tests/documents/json/compressed/jsonpath_com_example.json")?;
    let input = OwnedBytes::new(&raw_json.as_bytes())?;
    let engine = MainEngine::compile_query(&jsonpath_query)?;
    let result = engine.run::<_, CountResult>(&input)?;
    assert_eq!(result.get(), 2u64, "result != expected");
    Ok(())
}
#[test]
fn short_json_with_objects_and_lists_given_as_an_example_on_jsonpath_com_compressed_with_query_descendant_search_for_number_with_owned_bytes_and_count_result_using_recursive_engine(
) -> Result<(), Box<dyn Error>> {
    println ! ("on document jsonpath_com_example.toml running the query $..number (descendant search for 'number') with Input impl OwnedBytes and result mode CountResult");
    let jsonpath_query = JsonPathQuery::parse("$..number")?;
    let raw_json = fs::read_to_string("./tests/documents/json/compressed/jsonpath_com_example.json")?;
    let input = OwnedBytes::new(&raw_json.as_bytes())?;
    let engine = RecursiveEngine::compile_query(&jsonpath_query)?;
    let result = engine.run::<_, CountResult>(&input)?;
    assert_eq!(result.get(), 2u64, "result != expected");
    Ok(())
}
#[test]
fn short_json_with_objects_and_lists_given_as_an_example_on_jsonpath_com_compressed_with_query_descendant_search_for_number_with_owned_bytes_and_index_result_using_main_engine(
) -> Result<(), Box<dyn Error>> {
    println ! ("on document jsonpath_com_example.toml running the query $..number (descendant search for 'number') with Input impl OwnedBytes and result mode IndexResult");
    let jsonpath_query = JsonPathQuery::parse("$..number")?;
    let raw_json = fs::read_to_string("./tests/documents/json/compressed/jsonpath_com_example.json")?;
    let input = OwnedBytes::new(&raw_json.as_bytes())?;
    let engine = MainEngine::compile_query(&jsonpath_query)?;
    let result = engine.run::<_, IndexResult>(&input)?;
    assert_eq!(result.get(), vec![169usize, 211usize,], "result != expected");
    Ok(())
}
#[test]
fn short_json_with_objects_and_lists_given_as_an_example_on_jsonpath_com_compressed_with_query_descendant_search_for_number_with_owned_bytes_and_index_result_using_recursive_engine(
) -> Result<(), Box<dyn Error>> {
    println ! ("on document jsonpath_com_example.toml running the query $..number (descendant search for 'number') with Input impl OwnedBytes and result mode IndexResult");
    let jsonpath_query = JsonPathQuery::parse("$..number")?;
    let raw_json = fs::read_to_string("./tests/documents/json/compressed/jsonpath_com_example.json")?;
    let input = OwnedBytes::new(&raw_json.as_bytes())?;
    let engine = RecursiveEngine::compile_query(&jsonpath_query)?;
    let result = engine.run::<_, IndexResult>(&input)?;
    assert_eq!(result.get(), vec![169usize, 211usize,], "result != expected");
    Ok(())
}
#[test]
fn short_json_with_objects_and_lists_given_as_an_example_on_jsonpath_com_compressed_with_query_select_exact_path_with_name_and_index_selectors_with_buffered_input_and_count_result_using_main_engine(
) -> Result<(), Box<dyn Error>> {
    println ! ("on document jsonpath_com_example.toml running the query $.phoneNumbers[0].type (select exact path with name and index selectors) with Input impl BufferedInput and result mode CountResult");
    let jsonpath_query = JsonPathQuery::parse("$.phoneNumbers[0].type")?;
    let json_file = fs::File::open("./tests/documents/json/compressed/jsonpath_com_example.json")?;
    let input = BufferedInput::new(json_file);
    let engine = MainEngine::compile_query(&jsonpath_query)?;
    let result = engine.run::<_, CountResult>(&input)?;
    assert_eq!(result.get(), 1u64, "result != expected");
    Ok(())
}
#[test]
fn short_json_with_objects_and_lists_given_as_an_example_on_jsonpath_com_compressed_with_query_select_exact_path_with_name_and_index_selectors_with_buffered_input_and_count_result_using_recursive_engine(
) -> Result<(), Box<dyn Error>> {
    println ! ("on document jsonpath_com_example.toml running the query $.phoneNumbers[0].type (select exact path with name and index selectors) with Input impl BufferedInput and result mode CountResult");
    let jsonpath_query = JsonPathQuery::parse("$.phoneNumbers[0].type")?;
    let json_file = fs::File::open("./tests/documents/json/compressed/jsonpath_com_example.json")?;
    let input = BufferedInput::new(json_file);
    let engine = RecursiveEngine::compile_query(&jsonpath_query)?;
    let result = engine.run::<_, CountResult>(&input)?;
    assert_eq!(result.get(), 1u64, "result != expected");
    Ok(())
}
#[test]
fn short_json_with_objects_and_lists_given_as_an_example_on_jsonpath_com_compressed_with_query_select_exact_path_with_name_and_index_selectors_with_buffered_input_and_index_result_using_main_engine(
) -> Result<(), Box<dyn Error>> {
    println ! ("on document jsonpath_com_example.toml running the query $.phoneNumbers[0].type (select exact path with name and index selectors) with Input impl BufferedInput and result mode IndexResult");
    let jsonpath_query = JsonPathQuery::parse("$.phoneNumbers[0].type")?;
    let json_file = fs::File::open("./tests/documents/json/compressed/jsonpath_com_example.json")?;
    let input = BufferedInput::new(json_file);
    let engine = MainEngine::compile_query(&jsonpath_query)?;
    let result = engine.run::<_, IndexResult>(&input)?;
    assert_eq!(result.get(), vec![151usize,], "result != expected");
    Ok(())
}
#[test]
fn short_json_with_objects_and_lists_given_as_an_example_on_jsonpath_com_compressed_with_query_select_exact_path_with_name_and_index_selectors_with_buffered_input_and_index_result_using_recursive_engine(
) -> Result<(), Box<dyn Error>> {
    println ! ("on document jsonpath_com_example.toml running the query $.phoneNumbers[0].type (select exact path with name and index selectors) with Input impl BufferedInput and result mode IndexResult");
    let jsonpath_query = JsonPathQuery::parse("$.phoneNumbers[0].type")?;
    let json_file = fs::File::open("./tests/documents/json/compressed/jsonpath_com_example.json")?;
    let input = BufferedInput::new(json_file);
    let engine = RecursiveEngine::compile_query(&jsonpath_query)?;
    let result = engine.run::<_, IndexResult>(&input)?;
    assert_eq!(result.get(), vec![151usize,], "result != expected");
    Ok(())
}
#[test]
fn short_json_with_objects_and_lists_given_as_an_example_on_jsonpath_com_compressed_with_query_select_exact_path_with_name_and_index_selectors_with_mmap_input_and_count_result_using_main_engine(
) -> Result<(), Box<dyn Error>> {
    println ! ("on document jsonpath_com_example.toml running the query $.phoneNumbers[0].type (select exact path with name and index selectors) with Input impl MmapInput and result mode CountResult");
    let jsonpath_query = JsonPathQuery::parse("$.phoneNumbers[0].type")?;
    let json_file = fs::File::open("./tests/documents/json/compressed/jsonpath_com_example.json")?;
    let input = unsafe { MmapInput::map_file(&json_file)? };
    let engine = MainEngine::compile_query(&jsonpath_query)?;
    let result = engine.run::<_, CountResult>(&input)?;
    assert_eq!(result.get(), 1u64, "result != expected");
    Ok(())
}
#[test]
fn short_json_with_objects_and_lists_given_as_an_example_on_jsonpath_com_compressed_with_query_select_exact_path_with_name_and_index_selectors_with_mmap_input_and_count_result_using_recursive_engine(
) -> Result<(), Box<dyn Error>> {
    println ! ("on document jsonpath_com_example.toml running the query $.phoneNumbers[0].type (select exact path with name and index selectors) with Input impl MmapInput and result mode CountResult");
    let jsonpath_query = JsonPathQuery::parse("$.phoneNumbers[0].type")?;
    let json_file = fs::File::open("./tests/documents/json/compressed/jsonpath_com_example.json")?;
    let input = unsafe { MmapInput::map_file(&json_file)? };
    let engine = RecursiveEngine::compile_query(&jsonpath_query)?;
    let result = engine.run::<_, CountResult>(&input)?;
    assert_eq!(result.get(), 1u64, "result != expected");
    Ok(())
}
#[test]
fn short_json_with_objects_and_lists_given_as_an_example_on_jsonpath_com_compressed_with_query_select_exact_path_with_name_and_index_selectors_with_mmap_input_and_index_result_using_main_engine(
) -> Result<(), Box<dyn Error>> {
    println ! ("on document jsonpath_com_example.toml running the query $.phoneNumbers[0].type (select exact path with name and index selectors) with Input impl MmapInput and result mode IndexResult");
    let jsonpath_query = JsonPathQuery::parse("$.phoneNumbers[0].type")?;
    let json_file = fs::File::open("./tests/documents/json/compressed/jsonpath_com_example.json")?;
    let input = unsafe { MmapInput::map_file(&json_file)? };
    let engine = MainEngine::compile_query(&jsonpath_query)?;
    let result = engine.run::<_, IndexResult>(&input)?;
    assert_eq!(result.get(), vec![151usize,], "result != expected");
    Ok(())
}
#[test]
fn short_json_with_objects_and_lists_given_as_an_example_on_jsonpath_com_compressed_with_query_select_exact_path_with_name_and_index_selectors_with_mmap_input_and_index_result_using_recursive_engine(
) -> Result<(), Box<dyn Error>> {
    println ! ("on document jsonpath_com_example.toml running the query $.phoneNumbers[0].type (select exact path with name and index selectors) with Input impl MmapInput and result mode IndexResult");
    let jsonpath_query = JsonPathQuery::parse("$.phoneNumbers[0].type")?;
    let json_file = fs::File::open("./tests/documents/json/compressed/jsonpath_com_example.json")?;
    let input = unsafe { MmapInput::map_file(&json_file)? };
    let engine = RecursiveEngine::compile_query(&jsonpath_query)?;
    let result = engine.run::<_, IndexResult>(&input)?;
    assert_eq!(result.get(), vec![151usize,], "result != expected");
    Ok(())
}
#[test]
fn short_json_with_objects_and_lists_given_as_an_example_on_jsonpath_com_compressed_with_query_select_exact_path_with_name_and_index_selectors_with_owned_bytes_and_count_result_using_main_engine(
) -> Result<(), Box<dyn Error>> {
    println ! ("on document jsonpath_com_example.toml running the query $.phoneNumbers[0].type (select exact path with name and index selectors) with Input impl OwnedBytes and result mode CountResult");
    let jsonpath_query = JsonPathQuery::parse("$.phoneNumbers[0].type")?;
    let raw_json = fs::read_to_string("./tests/documents/json/compressed/jsonpath_com_example.json")?;
    let input = OwnedBytes::new(&raw_json.as_bytes())?;
    let engine = MainEngine::compile_query(&jsonpath_query)?;
    let result = engine.run::<_, CountResult>(&input)?;
    assert_eq!(result.get(), 1u64, "result != expected");
    Ok(())
}
#[test]
fn short_json_with_objects_and_lists_given_as_an_example_on_jsonpath_com_compressed_with_query_select_exact_path_with_name_and_index_selectors_with_owned_bytes_and_count_result_using_recursive_engine(
) -> Result<(), Box<dyn Error>> {
    println ! ("on document jsonpath_com_example.toml running the query $.phoneNumbers[0].type (select exact path with name and index selectors) with Input impl OwnedBytes and result mode CountResult");
    let jsonpath_query = JsonPathQuery::parse("$.phoneNumbers[0].type")?;
    let raw_json = fs::read_to_string("./tests/documents/json/compressed/jsonpath_com_example.json")?;
    let input = OwnedBytes::new(&raw_json.as_bytes())?;
    let engine = RecursiveEngine::compile_query(&jsonpath_query)?;
    let result = engine.run::<_, CountResult>(&input)?;
    assert_eq!(result.get(), 1u64, "result != expected");
    Ok(())
}
#[test]
fn short_json_with_objects_and_lists_given_as_an_example_on_jsonpath_com_compressed_with_query_select_exact_path_with_name_and_index_selectors_with_owned_bytes_and_index_result_using_main_engine(
) -> Result<(), Box<dyn Error>> {
    println ! ("on document jsonpath_com_example.toml running the query $.phoneNumbers[0].type (select exact path with name and index selectors) with Input impl OwnedBytes and result mode IndexResult");
    let jsonpath_query = JsonPathQuery::parse("$.phoneNumbers[0].type")?;
    let raw_json = fs::read_to_string("./tests/documents/json/compressed/jsonpath_com_example.json")?;
    let input = OwnedBytes::new(&raw_json.as_bytes())?;
    let engine = MainEngine::compile_query(&jsonpath_query)?;
    let result = engine.run::<_, IndexResult>(&input)?;
    assert_eq!(result.get(), vec![151usize,], "result != expected");
    Ok(())
}
#[test]
fn short_json_with_objects_and_lists_given_as_an_example_on_jsonpath_com_compressed_with_query_select_exact_path_with_name_and_index_selectors_with_owned_bytes_and_index_result_using_recursive_engine(
) -> Result<(), Box<dyn Error>> {
    println ! ("on document jsonpath_com_example.toml running the query $.phoneNumbers[0].type (select exact path with name and index selectors) with Input impl OwnedBytes and result mode IndexResult");
    let jsonpath_query = JsonPathQuery::parse("$.phoneNumbers[0].type")?;
    let raw_json = fs::read_to_string("./tests/documents/json/compressed/jsonpath_com_example.json")?;
    let input = OwnedBytes::new(&raw_json.as_bytes())?;
    let engine = RecursiveEngine::compile_query(&jsonpath_query)?;
    let result = engine.run::<_, IndexResult>(&input)?;
    assert_eq!(result.get(), vec![151usize,], "result != expected");
    Ok(())
}
#[test]
fn short_json_with_objects_and_lists_given_as_an_example_on_jsonpath_com_with_query_descendant_search_for_number_with_buffered_input_and_count_result_using_main_engine(
) -> Result<(), Box<dyn Error>> {
    println ! ("on document jsonpath_com_example.toml running the query $..number (descendant search for 'number') with Input impl BufferedInput and result mode CountResult");
    let jsonpath_query = JsonPathQuery::parse("$..number")?;
    let json_file = fs::File::open("./tests/documents/json/jsonpath_com_example.json")?;
    let input = BufferedInput::new(json_file);
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
    let json_file = fs::File::open("./tests/documents/json/jsonpath_com_example.json")?;
    let input = BufferedInput::new(json_file);
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
    let json_file = fs::File::open("./tests/documents/json/jsonpath_com_example.json")?;
    let input = BufferedInput::new(json_file);
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
    let json_file = fs::File::open("./tests/documents/json/jsonpath_com_example.json")?;
    let input = BufferedInput::new(json_file);
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
    let json_file = fs::File::open("./tests/documents/json/jsonpath_com_example.json")?;
    let input = unsafe { MmapInput::map_file(&json_file)? };
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
    let json_file = fs::File::open("./tests/documents/json/jsonpath_com_example.json")?;
    let input = unsafe { MmapInput::map_file(&json_file)? };
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
    let json_file = fs::File::open("./tests/documents/json/jsonpath_com_example.json")?;
    let input = unsafe { MmapInput::map_file(&json_file)? };
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
    let json_file = fs::File::open("./tests/documents/json/jsonpath_com_example.json")?;
    let input = unsafe { MmapInput::map_file(&json_file)? };
    let engine = RecursiveEngine::compile_query(&jsonpath_query)?;
    let result = engine.run::<_, IndexResult>(&input)?;
    assert_eq!(result.get(), vec![271usize, 359usize,], "result != expected");
    Ok(())
}
#[test]
fn short_json_with_objects_and_lists_given_as_an_example_on_jsonpath_com_with_query_descendant_search_for_number_with_owned_bytes_and_count_result_using_main_engine(
) -> Result<(), Box<dyn Error>> {
    println ! ("on document jsonpath_com_example.toml running the query $..number (descendant search for 'number') with Input impl OwnedBytes and result mode CountResult");
    let jsonpath_query = JsonPathQuery::parse("$..number")?;
    let raw_json = fs::read_to_string("./tests/documents/json/jsonpath_com_example.json")?;
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
    let raw_json = fs::read_to_string("./tests/documents/json/jsonpath_com_example.json")?;
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
    let raw_json = fs::read_to_string("./tests/documents/json/jsonpath_com_example.json")?;
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
    let raw_json = fs::read_to_string("./tests/documents/json/jsonpath_com_example.json")?;
    let input = OwnedBytes::new(&raw_json.as_bytes())?;
    let engine = RecursiveEngine::compile_query(&jsonpath_query)?;
    let result = engine.run::<_, IndexResult>(&input)?;
    assert_eq!(result.get(), vec![271usize, 359usize,], "result != expected");
    Ok(())
}
#[test]
fn short_json_with_objects_and_lists_given_as_an_example_on_jsonpath_com_with_query_select_exact_path_with_name_and_index_selectors_with_buffered_input_and_count_result_using_main_engine(
) -> Result<(), Box<dyn Error>> {
    println ! ("on document jsonpath_com_example.toml running the query $.phoneNumbers[0].type (select exact path with name and index selectors) with Input impl BufferedInput and result mode CountResult");
    let jsonpath_query = JsonPathQuery::parse("$.phoneNumbers[0].type")?;
    let json_file = fs::File::open("./tests/documents/json/jsonpath_com_example.json")?;
    let input = BufferedInput::new(json_file);
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
    let json_file = fs::File::open("./tests/documents/json/jsonpath_com_example.json")?;
    let input = BufferedInput::new(json_file);
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
    let json_file = fs::File::open("./tests/documents/json/jsonpath_com_example.json")?;
    let input = BufferedInput::new(json_file);
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
    let json_file = fs::File::open("./tests/documents/json/jsonpath_com_example.json")?;
    let input = BufferedInput::new(json_file);
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
    let json_file = fs::File::open("./tests/documents/json/jsonpath_com_example.json")?;
    let input = unsafe { MmapInput::map_file(&json_file)? };
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
    let json_file = fs::File::open("./tests/documents/json/jsonpath_com_example.json")?;
    let input = unsafe { MmapInput::map_file(&json_file)? };
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
    let json_file = fs::File::open("./tests/documents/json/jsonpath_com_example.json")?;
    let input = unsafe { MmapInput::map_file(&json_file)? };
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
    let json_file = fs::File::open("./tests/documents/json/jsonpath_com_example.json")?;
    let input = unsafe { MmapInput::map_file(&json_file)? };
    let engine = RecursiveEngine::compile_query(&jsonpath_query)?;
    let result = engine.run::<_, IndexResult>(&input)?;
    assert_eq!(result.get(), vec![239usize,], "result != expected");
    Ok(())
}
#[test]
fn short_json_with_objects_and_lists_given_as_an_example_on_jsonpath_com_with_query_select_exact_path_with_name_and_index_selectors_with_owned_bytes_and_count_result_using_main_engine(
) -> Result<(), Box<dyn Error>> {
    println ! ("on document jsonpath_com_example.toml running the query $.phoneNumbers[0].type (select exact path with name and index selectors) with Input impl OwnedBytes and result mode CountResult");
    let jsonpath_query = JsonPathQuery::parse("$.phoneNumbers[0].type")?;
    let raw_json = fs::read_to_string("./tests/documents/json/jsonpath_com_example.json")?;
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
    let raw_json = fs::read_to_string("./tests/documents/json/jsonpath_com_example.json")?;
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
    let raw_json = fs::read_to_string("./tests/documents/json/jsonpath_com_example.json")?;
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
    let raw_json = fs::read_to_string("./tests/documents/json/jsonpath_com_example.json")?;
    let input = OwnedBytes::new(&raw_json.as_bytes())?;
    let engine = RecursiveEngine::compile_query(&jsonpath_query)?;
    let result = engine.run::<_, IndexResult>(&input)?;
    assert_eq!(result.get(), vec![239usize,], "result != expected");
    Ok(())
}
#[test]
fn twitter_json_from_simdjson_github_example_compressed_with_query_descendant_user_entities_url_with_buffered_input_and_count_result_using_main_engine(
) -> Result<(), Box<dyn Error>> {
    println ! ("on document twitter.toml running the query $..user..entities..url (descendant user entities url) with Input impl BufferedInput and result mode CountResult");
    let jsonpath_query = JsonPathQuery::parse("$..user..entities..url")?;
    let json_file = fs::File::open("./tests/documents/json/compressed/twitter.json")?;
    let input = BufferedInput::new(json_file);
    let engine = MainEngine::compile_query(&jsonpath_query)?;
    let result = engine.run::<_, CountResult>(&input)?;
    assert_eq!(result.get(), 44u64, "result != expected");
    Ok(())
}
#[test]
fn twitter_json_from_simdjson_github_example_compressed_with_query_descendant_user_entities_url_with_buffered_input_and_count_result_using_recursive_engine(
) -> Result<(), Box<dyn Error>> {
    println ! ("on document twitter.toml running the query $..user..entities..url (descendant user entities url) with Input impl BufferedInput and result mode CountResult");
    let jsonpath_query = JsonPathQuery::parse("$..user..entities..url")?;
    let json_file = fs::File::open("./tests/documents/json/compressed/twitter.json")?;
    let input = BufferedInput::new(json_file);
    let engine = RecursiveEngine::compile_query(&jsonpath_query)?;
    let result = engine.run::<_, CountResult>(&input)?;
    assert_eq!(result.get(), 44u64, "result != expected");
    Ok(())
}
#[test]
fn twitter_json_from_simdjson_github_example_compressed_with_query_descendant_user_entities_url_with_buffered_input_and_index_result_using_main_engine(
) -> Result<(), Box<dyn Error>> {
    println ! ("on document twitter.toml running the query $..user..entities..url (descendant user entities url) with Input impl BufferedInput and result mode IndexResult");
    let jsonpath_query = JsonPathQuery::parse("$..user..entities..url")?;
    let json_file = fs::File::open("./tests/documents/json/compressed/twitter.json")?;
    let input = BufferedInput::new(json_file);
    let engine = MainEngine::compile_query(&jsonpath_query)?;
    let result = engine.run::<_, IndexResult>(&input)?;
    assert_eq!(
        result.get(),
        vec![
            3488usize,
            3504usize,
            5803usize,
            5955usize,
            9836usize,
            9852usize,
            12718usize,
            12734usize,
            12913usize,
            52574usize,
            52590usize,
            64603usize,
            64619usize,
            77997usize,
            78013usize,
            78165usize,
            119307usize,
            119323usize,
            121918usize,
            121934usize,
            122097usize,
            201073usize,
            201089usize,
            212698usize,
            212714usize,
            212878usize,
            215343usize,
            215359usize,
            241826usize,
            241842usize,
            274278usize,
            288269usize,
            288285usize,
            310030usize,
            310046usize,
            312972usize,
            312988usize,
            445431usize,
            445447usize,
            454460usize,
            454476usize,
            464576usize,
            464592usize,
            464769usize,
        ],
        "result != expected"
    );
    Ok(())
}
#[test]
fn twitter_json_from_simdjson_github_example_compressed_with_query_descendant_user_entities_url_with_buffered_input_and_index_result_using_recursive_engine(
) -> Result<(), Box<dyn Error>> {
    println ! ("on document twitter.toml running the query $..user..entities..url (descendant user entities url) with Input impl BufferedInput and result mode IndexResult");
    let jsonpath_query = JsonPathQuery::parse("$..user..entities..url")?;
    let json_file = fs::File::open("./tests/documents/json/compressed/twitter.json")?;
    let input = BufferedInput::new(json_file);
    let engine = RecursiveEngine::compile_query(&jsonpath_query)?;
    let result = engine.run::<_, IndexResult>(&input)?;
    assert_eq!(
        result.get(),
        vec![
            3488usize,
            3504usize,
            5803usize,
            5955usize,
            9836usize,
            9852usize,
            12718usize,
            12734usize,
            12913usize,
            52574usize,
            52590usize,
            64603usize,
            64619usize,
            77997usize,
            78013usize,
            78165usize,
            119307usize,
            119323usize,
            121918usize,
            121934usize,
            122097usize,
            201073usize,
            201089usize,
            212698usize,
            212714usize,
            212878usize,
            215343usize,
            215359usize,
            241826usize,
            241842usize,
            274278usize,
            288269usize,
            288285usize,
            310030usize,
            310046usize,
            312972usize,
            312988usize,
            445431usize,
            445447usize,
            454460usize,
            454476usize,
            464576usize,
            464592usize,
            464769usize,
        ],
        "result != expected"
    );
    Ok(())
}
#[test]
fn twitter_json_from_simdjson_github_example_compressed_with_query_descendant_user_entities_url_with_mmap_input_and_count_result_using_main_engine(
) -> Result<(), Box<dyn Error>> {
    println ! ("on document twitter.toml running the query $..user..entities..url (descendant user entities url) with Input impl MmapInput and result mode CountResult");
    let jsonpath_query = JsonPathQuery::parse("$..user..entities..url")?;
    let json_file = fs::File::open("./tests/documents/json/compressed/twitter.json")?;
    let input = unsafe { MmapInput::map_file(&json_file)? };
    let engine = MainEngine::compile_query(&jsonpath_query)?;
    let result = engine.run::<_, CountResult>(&input)?;
    assert_eq!(result.get(), 44u64, "result != expected");
    Ok(())
}
#[test]
fn twitter_json_from_simdjson_github_example_compressed_with_query_descendant_user_entities_url_with_mmap_input_and_count_result_using_recursive_engine(
) -> Result<(), Box<dyn Error>> {
    println ! ("on document twitter.toml running the query $..user..entities..url (descendant user entities url) with Input impl MmapInput and result mode CountResult");
    let jsonpath_query = JsonPathQuery::parse("$..user..entities..url")?;
    let json_file = fs::File::open("./tests/documents/json/compressed/twitter.json")?;
    let input = unsafe { MmapInput::map_file(&json_file)? };
    let engine = RecursiveEngine::compile_query(&jsonpath_query)?;
    let result = engine.run::<_, CountResult>(&input)?;
    assert_eq!(result.get(), 44u64, "result != expected");
    Ok(())
}
#[test]
fn twitter_json_from_simdjson_github_example_compressed_with_query_descendant_user_entities_url_with_mmap_input_and_index_result_using_main_engine(
) -> Result<(), Box<dyn Error>> {
    println ! ("on document twitter.toml running the query $..user..entities..url (descendant user entities url) with Input impl MmapInput and result mode IndexResult");
    let jsonpath_query = JsonPathQuery::parse("$..user..entities..url")?;
    let json_file = fs::File::open("./tests/documents/json/compressed/twitter.json")?;
    let input = unsafe { MmapInput::map_file(&json_file)? };
    let engine = MainEngine::compile_query(&jsonpath_query)?;
    let result = engine.run::<_, IndexResult>(&input)?;
    assert_eq!(
        result.get(),
        vec![
            3488usize,
            3504usize,
            5803usize,
            5955usize,
            9836usize,
            9852usize,
            12718usize,
            12734usize,
            12913usize,
            52574usize,
            52590usize,
            64603usize,
            64619usize,
            77997usize,
            78013usize,
            78165usize,
            119307usize,
            119323usize,
            121918usize,
            121934usize,
            122097usize,
            201073usize,
            201089usize,
            212698usize,
            212714usize,
            212878usize,
            215343usize,
            215359usize,
            241826usize,
            241842usize,
            274278usize,
            288269usize,
            288285usize,
            310030usize,
            310046usize,
            312972usize,
            312988usize,
            445431usize,
            445447usize,
            454460usize,
            454476usize,
            464576usize,
            464592usize,
            464769usize,
        ],
        "result != expected"
    );
    Ok(())
}
#[test]
fn twitter_json_from_simdjson_github_example_compressed_with_query_descendant_user_entities_url_with_mmap_input_and_index_result_using_recursive_engine(
) -> Result<(), Box<dyn Error>> {
    println ! ("on document twitter.toml running the query $..user..entities..url (descendant user entities url) with Input impl MmapInput and result mode IndexResult");
    let jsonpath_query = JsonPathQuery::parse("$..user..entities..url")?;
    let json_file = fs::File::open("./tests/documents/json/compressed/twitter.json")?;
    let input = unsafe { MmapInput::map_file(&json_file)? };
    let engine = RecursiveEngine::compile_query(&jsonpath_query)?;
    let result = engine.run::<_, IndexResult>(&input)?;
    assert_eq!(
        result.get(),
        vec![
            3488usize,
            3504usize,
            5803usize,
            5955usize,
            9836usize,
            9852usize,
            12718usize,
            12734usize,
            12913usize,
            52574usize,
            52590usize,
            64603usize,
            64619usize,
            77997usize,
            78013usize,
            78165usize,
            119307usize,
            119323usize,
            121918usize,
            121934usize,
            122097usize,
            201073usize,
            201089usize,
            212698usize,
            212714usize,
            212878usize,
            215343usize,
            215359usize,
            241826usize,
            241842usize,
            274278usize,
            288269usize,
            288285usize,
            310030usize,
            310046usize,
            312972usize,
            312988usize,
            445431usize,
            445447usize,
            454460usize,
            454476usize,
            464576usize,
            464592usize,
            464769usize,
        ],
        "result != expected"
    );
    Ok(())
}
#[test]
fn twitter_json_from_simdjson_github_example_compressed_with_query_descendant_user_entities_url_with_owned_bytes_and_count_result_using_main_engine(
) -> Result<(), Box<dyn Error>> {
    println ! ("on document twitter.toml running the query $..user..entities..url (descendant user entities url) with Input impl OwnedBytes and result mode CountResult");
    let jsonpath_query = JsonPathQuery::parse("$..user..entities..url")?;
    let raw_json = fs::read_to_string("./tests/documents/json/compressed/twitter.json")?;
    let input = OwnedBytes::new(&raw_json.as_bytes())?;
    let engine = MainEngine::compile_query(&jsonpath_query)?;
    let result = engine.run::<_, CountResult>(&input)?;
    assert_eq!(result.get(), 44u64, "result != expected");
    Ok(())
}
#[test]
fn twitter_json_from_simdjson_github_example_compressed_with_query_descendant_user_entities_url_with_owned_bytes_and_count_result_using_recursive_engine(
) -> Result<(), Box<dyn Error>> {
    println ! ("on document twitter.toml running the query $..user..entities..url (descendant user entities url) with Input impl OwnedBytes and result mode CountResult");
    let jsonpath_query = JsonPathQuery::parse("$..user..entities..url")?;
    let raw_json = fs::read_to_string("./tests/documents/json/compressed/twitter.json")?;
    let input = OwnedBytes::new(&raw_json.as_bytes())?;
    let engine = RecursiveEngine::compile_query(&jsonpath_query)?;
    let result = engine.run::<_, CountResult>(&input)?;
    assert_eq!(result.get(), 44u64, "result != expected");
    Ok(())
}
#[test]
fn twitter_json_from_simdjson_github_example_compressed_with_query_descendant_user_entities_url_with_owned_bytes_and_index_result_using_main_engine(
) -> Result<(), Box<dyn Error>> {
    println ! ("on document twitter.toml running the query $..user..entities..url (descendant user entities url) with Input impl OwnedBytes and result mode IndexResult");
    let jsonpath_query = JsonPathQuery::parse("$..user..entities..url")?;
    let raw_json = fs::read_to_string("./tests/documents/json/compressed/twitter.json")?;
    let input = OwnedBytes::new(&raw_json.as_bytes())?;
    let engine = MainEngine::compile_query(&jsonpath_query)?;
    let result = engine.run::<_, IndexResult>(&input)?;
    assert_eq!(
        result.get(),
        vec![
            3488usize,
            3504usize,
            5803usize,
            5955usize,
            9836usize,
            9852usize,
            12718usize,
            12734usize,
            12913usize,
            52574usize,
            52590usize,
            64603usize,
            64619usize,
            77997usize,
            78013usize,
            78165usize,
            119307usize,
            119323usize,
            121918usize,
            121934usize,
            122097usize,
            201073usize,
            201089usize,
            212698usize,
            212714usize,
            212878usize,
            215343usize,
            215359usize,
            241826usize,
            241842usize,
            274278usize,
            288269usize,
            288285usize,
            310030usize,
            310046usize,
            312972usize,
            312988usize,
            445431usize,
            445447usize,
            454460usize,
            454476usize,
            464576usize,
            464592usize,
            464769usize,
        ],
        "result != expected"
    );
    Ok(())
}
#[test]
fn twitter_json_from_simdjson_github_example_compressed_with_query_descendant_user_entities_url_with_owned_bytes_and_index_result_using_recursive_engine(
) -> Result<(), Box<dyn Error>> {
    println ! ("on document twitter.toml running the query $..user..entities..url (descendant user entities url) with Input impl OwnedBytes and result mode IndexResult");
    let jsonpath_query = JsonPathQuery::parse("$..user..entities..url")?;
    let raw_json = fs::read_to_string("./tests/documents/json/compressed/twitter.json")?;
    let input = OwnedBytes::new(&raw_json.as_bytes())?;
    let engine = RecursiveEngine::compile_query(&jsonpath_query)?;
    let result = engine.run::<_, IndexResult>(&input)?;
    assert_eq!(
        result.get(),
        vec![
            3488usize,
            3504usize,
            5803usize,
            5955usize,
            9836usize,
            9852usize,
            12718usize,
            12734usize,
            12913usize,
            52574usize,
            52590usize,
            64603usize,
            64619usize,
            77997usize,
            78013usize,
            78165usize,
            119307usize,
            119323usize,
            121918usize,
            121934usize,
            122097usize,
            201073usize,
            201089usize,
            212698usize,
            212714usize,
            212878usize,
            215343usize,
            215359usize,
            241826usize,
            241842usize,
            274278usize,
            288269usize,
            288285usize,
            310030usize,
            310046usize,
            312972usize,
            312988usize,
            445431usize,
            445447usize,
            454460usize,
            454476usize,
            464576usize,
            464592usize,
            464769usize,
        ],
        "result != expected"
    );
    Ok(())
}
#[test]
fn twitter_json_from_simdjson_github_example_with_query_descendant_user_entities_url_with_buffered_input_and_count_result_using_main_engine(
) -> Result<(), Box<dyn Error>> {
    println ! ("on document twitter.toml running the query $..user..entities..url (descendant user entities url) with Input impl BufferedInput and result mode CountResult");
    let jsonpath_query = JsonPathQuery::parse("$..user..entities..url")?;
    let json_file = fs::File::open("./tests/documents/json/twitter.json")?;
    let input = BufferedInput::new(json_file);
    let engine = MainEngine::compile_query(&jsonpath_query)?;
    let result = engine.run::<_, CountResult>(&input)?;
    assert_eq!(result.get(), 44u64, "result != expected");
    Ok(())
}
#[test]
fn twitter_json_from_simdjson_github_example_with_query_descendant_user_entities_url_with_buffered_input_and_count_result_using_recursive_engine(
) -> Result<(), Box<dyn Error>> {
    println ! ("on document twitter.toml running the query $..user..entities..url (descendant user entities url) with Input impl BufferedInput and result mode CountResult");
    let jsonpath_query = JsonPathQuery::parse("$..user..entities..url")?;
    let json_file = fs::File::open("./tests/documents/json/twitter.json")?;
    let input = BufferedInput::new(json_file);
    let engine = RecursiveEngine::compile_query(&jsonpath_query)?;
    let result = engine.run::<_, CountResult>(&input)?;
    assert_eq!(result.get(), 44u64, "result != expected");
    Ok(())
}
#[test]
fn twitter_json_from_simdjson_github_example_with_query_descendant_user_entities_url_with_buffered_input_and_index_result_using_main_engine(
) -> Result<(), Box<dyn Error>> {
    println ! ("on document twitter.toml running the query $..user..entities..url (descendant user entities url) with Input impl BufferedInput and result mode IndexResult");
    let jsonpath_query = JsonPathQuery::parse("$..user..entities..url")?;
    let json_file = fs::File::open("./tests/documents/json/twitter.json")?;
    let input = BufferedInput::new(json_file);
    let engine = MainEngine::compile_query(&jsonpath_query)?;
    let result = engine.run::<_, IndexResult>(&input)?;
    assert_eq!(
        result.get(),
        vec![
            5465usize,
            5570usize,
            9496usize,
            9985usize,
            18496usize,
            18601usize,
            23338usize,
            23443usize,
            24017usize,
            89785usize,
            89902usize,
            112198usize,
            112315usize,
            134220usize,
            134337usize,
            134936usize,
            201055usize,
            201160usize,
            205281usize,
            205398usize,
            206008usize,
            333130usize,
            333235usize,
            352432usize,
            352537usize,
            353096usize,
            357000usize,
            357117usize,
            399785usize,
            399902usize,
            451854usize,
            475584usize,
            475689usize,
            511442usize,
            511547usize,
            516538usize,
            516643usize,
            728252usize,
            728357usize,
            743602usize,
            743719usize,
            762797usize,
            762902usize,
            763474usize,
        ],
        "result != expected"
    );
    Ok(())
}
#[test]
fn twitter_json_from_simdjson_github_example_with_query_descendant_user_entities_url_with_buffered_input_and_index_result_using_recursive_engine(
) -> Result<(), Box<dyn Error>> {
    println ! ("on document twitter.toml running the query $..user..entities..url (descendant user entities url) with Input impl BufferedInput and result mode IndexResult");
    let jsonpath_query = JsonPathQuery::parse("$..user..entities..url")?;
    let json_file = fs::File::open("./tests/documents/json/twitter.json")?;
    let input = BufferedInput::new(json_file);
    let engine = RecursiveEngine::compile_query(&jsonpath_query)?;
    let result = engine.run::<_, IndexResult>(&input)?;
    assert_eq!(
        result.get(),
        vec![
            5465usize,
            5570usize,
            9496usize,
            9985usize,
            18496usize,
            18601usize,
            23338usize,
            23443usize,
            24017usize,
            89785usize,
            89902usize,
            112198usize,
            112315usize,
            134220usize,
            134337usize,
            134936usize,
            201055usize,
            201160usize,
            205281usize,
            205398usize,
            206008usize,
            333130usize,
            333235usize,
            352432usize,
            352537usize,
            353096usize,
            357000usize,
            357117usize,
            399785usize,
            399902usize,
            451854usize,
            475584usize,
            475689usize,
            511442usize,
            511547usize,
            516538usize,
            516643usize,
            728252usize,
            728357usize,
            743602usize,
            743719usize,
            762797usize,
            762902usize,
            763474usize,
        ],
        "result != expected"
    );
    Ok(())
}
#[test]
fn twitter_json_from_simdjson_github_example_with_query_descendant_user_entities_url_with_mmap_input_and_count_result_using_main_engine(
) -> Result<(), Box<dyn Error>> {
    println ! ("on document twitter.toml running the query $..user..entities..url (descendant user entities url) with Input impl MmapInput and result mode CountResult");
    let jsonpath_query = JsonPathQuery::parse("$..user..entities..url")?;
    let json_file = fs::File::open("./tests/documents/json/twitter.json")?;
    let input = unsafe { MmapInput::map_file(&json_file)? };
    let engine = MainEngine::compile_query(&jsonpath_query)?;
    let result = engine.run::<_, CountResult>(&input)?;
    assert_eq!(result.get(), 44u64, "result != expected");
    Ok(())
}
#[test]
fn twitter_json_from_simdjson_github_example_with_query_descendant_user_entities_url_with_mmap_input_and_count_result_using_recursive_engine(
) -> Result<(), Box<dyn Error>> {
    println ! ("on document twitter.toml running the query $..user..entities..url (descendant user entities url) with Input impl MmapInput and result mode CountResult");
    let jsonpath_query = JsonPathQuery::parse("$..user..entities..url")?;
    let json_file = fs::File::open("./tests/documents/json/twitter.json")?;
    let input = unsafe { MmapInput::map_file(&json_file)? };
    let engine = RecursiveEngine::compile_query(&jsonpath_query)?;
    let result = engine.run::<_, CountResult>(&input)?;
    assert_eq!(result.get(), 44u64, "result != expected");
    Ok(())
}
#[test]
fn twitter_json_from_simdjson_github_example_with_query_descendant_user_entities_url_with_mmap_input_and_index_result_using_main_engine(
) -> Result<(), Box<dyn Error>> {
    println ! ("on document twitter.toml running the query $..user..entities..url (descendant user entities url) with Input impl MmapInput and result mode IndexResult");
    let jsonpath_query = JsonPathQuery::parse("$..user..entities..url")?;
    let json_file = fs::File::open("./tests/documents/json/twitter.json")?;
    let input = unsafe { MmapInput::map_file(&json_file)? };
    let engine = MainEngine::compile_query(&jsonpath_query)?;
    let result = engine.run::<_, IndexResult>(&input)?;
    assert_eq!(
        result.get(),
        vec![
            5465usize,
            5570usize,
            9496usize,
            9985usize,
            18496usize,
            18601usize,
            23338usize,
            23443usize,
            24017usize,
            89785usize,
            89902usize,
            112198usize,
            112315usize,
            134220usize,
            134337usize,
            134936usize,
            201055usize,
            201160usize,
            205281usize,
            205398usize,
            206008usize,
            333130usize,
            333235usize,
            352432usize,
            352537usize,
            353096usize,
            357000usize,
            357117usize,
            399785usize,
            399902usize,
            451854usize,
            475584usize,
            475689usize,
            511442usize,
            511547usize,
            516538usize,
            516643usize,
            728252usize,
            728357usize,
            743602usize,
            743719usize,
            762797usize,
            762902usize,
            763474usize,
        ],
        "result != expected"
    );
    Ok(())
}
#[test]
fn twitter_json_from_simdjson_github_example_with_query_descendant_user_entities_url_with_mmap_input_and_index_result_using_recursive_engine(
) -> Result<(), Box<dyn Error>> {
    println ! ("on document twitter.toml running the query $..user..entities..url (descendant user entities url) with Input impl MmapInput and result mode IndexResult");
    let jsonpath_query = JsonPathQuery::parse("$..user..entities..url")?;
    let json_file = fs::File::open("./tests/documents/json/twitter.json")?;
    let input = unsafe { MmapInput::map_file(&json_file)? };
    let engine = RecursiveEngine::compile_query(&jsonpath_query)?;
    let result = engine.run::<_, IndexResult>(&input)?;
    assert_eq!(
        result.get(),
        vec![
            5465usize,
            5570usize,
            9496usize,
            9985usize,
            18496usize,
            18601usize,
            23338usize,
            23443usize,
            24017usize,
            89785usize,
            89902usize,
            112198usize,
            112315usize,
            134220usize,
            134337usize,
            134936usize,
            201055usize,
            201160usize,
            205281usize,
            205398usize,
            206008usize,
            333130usize,
            333235usize,
            352432usize,
            352537usize,
            353096usize,
            357000usize,
            357117usize,
            399785usize,
            399902usize,
            451854usize,
            475584usize,
            475689usize,
            511442usize,
            511547usize,
            516538usize,
            516643usize,
            728252usize,
            728357usize,
            743602usize,
            743719usize,
            762797usize,
            762902usize,
            763474usize,
        ],
        "result != expected"
    );
    Ok(())
}
#[test]
fn twitter_json_from_simdjson_github_example_with_query_descendant_user_entities_url_with_owned_bytes_and_count_result_using_main_engine(
) -> Result<(), Box<dyn Error>> {
    println ! ("on document twitter.toml running the query $..user..entities..url (descendant user entities url) with Input impl OwnedBytes and result mode CountResult");
    let jsonpath_query = JsonPathQuery::parse("$..user..entities..url")?;
    let raw_json = fs::read_to_string("./tests/documents/json/twitter.json")?;
    let input = OwnedBytes::new(&raw_json.as_bytes())?;
    let engine = MainEngine::compile_query(&jsonpath_query)?;
    let result = engine.run::<_, CountResult>(&input)?;
    assert_eq!(result.get(), 44u64, "result != expected");
    Ok(())
}
#[test]
fn twitter_json_from_simdjson_github_example_with_query_descendant_user_entities_url_with_owned_bytes_and_count_result_using_recursive_engine(
) -> Result<(), Box<dyn Error>> {
    println ! ("on document twitter.toml running the query $..user..entities..url (descendant user entities url) with Input impl OwnedBytes and result mode CountResult");
    let jsonpath_query = JsonPathQuery::parse("$..user..entities..url")?;
    let raw_json = fs::read_to_string("./tests/documents/json/twitter.json")?;
    let input = OwnedBytes::new(&raw_json.as_bytes())?;
    let engine = RecursiveEngine::compile_query(&jsonpath_query)?;
    let result = engine.run::<_, CountResult>(&input)?;
    assert_eq!(result.get(), 44u64, "result != expected");
    Ok(())
}
#[test]
fn twitter_json_from_simdjson_github_example_with_query_descendant_user_entities_url_with_owned_bytes_and_index_result_using_main_engine(
) -> Result<(), Box<dyn Error>> {
    println ! ("on document twitter.toml running the query $..user..entities..url (descendant user entities url) with Input impl OwnedBytes and result mode IndexResult");
    let jsonpath_query = JsonPathQuery::parse("$..user..entities..url")?;
    let raw_json = fs::read_to_string("./tests/documents/json/twitter.json")?;
    let input = OwnedBytes::new(&raw_json.as_bytes())?;
    let engine = MainEngine::compile_query(&jsonpath_query)?;
    let result = engine.run::<_, IndexResult>(&input)?;
    assert_eq!(
        result.get(),
        vec![
            5465usize,
            5570usize,
            9496usize,
            9985usize,
            18496usize,
            18601usize,
            23338usize,
            23443usize,
            24017usize,
            89785usize,
            89902usize,
            112198usize,
            112315usize,
            134220usize,
            134337usize,
            134936usize,
            201055usize,
            201160usize,
            205281usize,
            205398usize,
            206008usize,
            333130usize,
            333235usize,
            352432usize,
            352537usize,
            353096usize,
            357000usize,
            357117usize,
            399785usize,
            399902usize,
            451854usize,
            475584usize,
            475689usize,
            511442usize,
            511547usize,
            516538usize,
            516643usize,
            728252usize,
            728357usize,
            743602usize,
            743719usize,
            762797usize,
            762902usize,
            763474usize,
        ],
        "result != expected"
    );
    Ok(())
}
#[test]
fn twitter_json_from_simdjson_github_example_with_query_descendant_user_entities_url_with_owned_bytes_and_index_result_using_recursive_engine(
) -> Result<(), Box<dyn Error>> {
    println ! ("on document twitter.toml running the query $..user..entities..url (descendant user entities url) with Input impl OwnedBytes and result mode IndexResult");
    let jsonpath_query = JsonPathQuery::parse("$..user..entities..url")?;
    let raw_json = fs::read_to_string("./tests/documents/json/twitter.json")?;
    let input = OwnedBytes::new(&raw_json.as_bytes())?;
    let engine = RecursiveEngine::compile_query(&jsonpath_query)?;
    let result = engine.run::<_, IndexResult>(&input)?;
    assert_eq!(
        result.get(),
        vec![
            5465usize,
            5570usize,
            9496usize,
            9985usize,
            18496usize,
            18601usize,
            23338usize,
            23443usize,
            24017usize,
            89785usize,
            89902usize,
            112198usize,
            112315usize,
            134220usize,
            134337usize,
            134936usize,
            201055usize,
            201160usize,
            205281usize,
            205398usize,
            206008usize,
            333130usize,
            333235usize,
            352432usize,
            352537usize,
            353096usize,
            357000usize,
            357117usize,
            399785usize,
            399902usize,
            451854usize,
            475584usize,
            475689usize,
            511442usize,
            511547usize,
            516538usize,
            516643usize,
            728252usize,
            728357usize,
            743602usize,
            743719usize,
            762797usize,
            762902usize,
            763474usize,
        ],
        "result != expected"
    );
    Ok(())
}
#[test]
fn whitespace_separators_between_structurals_to_test_correctness_of_index_result_handling_compressed_with_query_select_a_by_child_with_buffered_input_and_count_result_using_main_engine(
) -> Result<(), Box<dyn Error>> {
    println ! ("on document index_result.toml running the query $.a (select a by child) with Input impl BufferedInput and result mode CountResult");
    let jsonpath_query = JsonPathQuery::parse("$.a")?;
    let json_file = fs::File::open("./tests/documents/json/compressed/index_result.json")?;
    let input = BufferedInput::new(json_file);
    let engine = MainEngine::compile_query(&jsonpath_query)?;
    let result = engine.run::<_, CountResult>(&input)?;
    assert_eq!(result.get(), 1u64, "result != expected");
    Ok(())
}
#[test]
fn whitespace_separators_between_structurals_to_test_correctness_of_index_result_handling_compressed_with_query_select_a_by_child_with_buffered_input_and_count_result_using_recursive_engine(
) -> Result<(), Box<dyn Error>> {
    println ! ("on document index_result.toml running the query $.a (select a by child) with Input impl BufferedInput and result mode CountResult");
    let jsonpath_query = JsonPathQuery::parse("$.a")?;
    let json_file = fs::File::open("./tests/documents/json/compressed/index_result.json")?;
    let input = BufferedInput::new(json_file);
    let engine = RecursiveEngine::compile_query(&jsonpath_query)?;
    let result = engine.run::<_, CountResult>(&input)?;
    assert_eq!(result.get(), 1u64, "result != expected");
    Ok(())
}
#[test]
fn whitespace_separators_between_structurals_to_test_correctness_of_index_result_handling_compressed_with_query_select_a_by_child_with_buffered_input_and_index_result_using_main_engine(
) -> Result<(), Box<dyn Error>> {
    println ! ("on document index_result.toml running the query $.a (select a by child) with Input impl BufferedInput and result mode IndexResult");
    let jsonpath_query = JsonPathQuery::parse("$.a")?;
    let json_file = fs::File::open("./tests/documents/json/compressed/index_result.json")?;
    let input = BufferedInput::new(json_file);
    let engine = MainEngine::compile_query(&jsonpath_query)?;
    let result = engine.run::<_, IndexResult>(&input)?;
    assert_eq!(result.get(), vec![5usize,], "result != expected");
    Ok(())
}
#[test]
fn whitespace_separators_between_structurals_to_test_correctness_of_index_result_handling_compressed_with_query_select_a_by_child_with_buffered_input_and_index_result_using_recursive_engine(
) -> Result<(), Box<dyn Error>> {
    println ! ("on document index_result.toml running the query $.a (select a by child) with Input impl BufferedInput and result mode IndexResult");
    let jsonpath_query = JsonPathQuery::parse("$.a")?;
    let json_file = fs::File::open("./tests/documents/json/compressed/index_result.json")?;
    let input = BufferedInput::new(json_file);
    let engine = RecursiveEngine::compile_query(&jsonpath_query)?;
    let result = engine.run::<_, IndexResult>(&input)?;
    assert_eq!(result.get(), vec![5usize,], "result != expected");
    Ok(())
}
#[test]
fn whitespace_separators_between_structurals_to_test_correctness_of_index_result_handling_compressed_with_query_select_a_by_child_with_mmap_input_and_count_result_using_main_engine(
) -> Result<(), Box<dyn Error>> {
    println ! ("on document index_result.toml running the query $.a (select a by child) with Input impl MmapInput and result mode CountResult");
    let jsonpath_query = JsonPathQuery::parse("$.a")?;
    let json_file = fs::File::open("./tests/documents/json/compressed/index_result.json")?;
    let input = unsafe { MmapInput::map_file(&json_file)? };
    let engine = MainEngine::compile_query(&jsonpath_query)?;
    let result = engine.run::<_, CountResult>(&input)?;
    assert_eq!(result.get(), 1u64, "result != expected");
    Ok(())
}
#[test]
fn whitespace_separators_between_structurals_to_test_correctness_of_index_result_handling_compressed_with_query_select_a_by_child_with_mmap_input_and_count_result_using_recursive_engine(
) -> Result<(), Box<dyn Error>> {
    println ! ("on document index_result.toml running the query $.a (select a by child) with Input impl MmapInput and result mode CountResult");
    let jsonpath_query = JsonPathQuery::parse("$.a")?;
    let json_file = fs::File::open("./tests/documents/json/compressed/index_result.json")?;
    let input = unsafe { MmapInput::map_file(&json_file)? };
    let engine = RecursiveEngine::compile_query(&jsonpath_query)?;
    let result = engine.run::<_, CountResult>(&input)?;
    assert_eq!(result.get(), 1u64, "result != expected");
    Ok(())
}
#[test]
fn whitespace_separators_between_structurals_to_test_correctness_of_index_result_handling_compressed_with_query_select_a_by_child_with_mmap_input_and_index_result_using_main_engine(
) -> Result<(), Box<dyn Error>> {
    println ! ("on document index_result.toml running the query $.a (select a by child) with Input impl MmapInput and result mode IndexResult");
    let jsonpath_query = JsonPathQuery::parse("$.a")?;
    let json_file = fs::File::open("./tests/documents/json/compressed/index_result.json")?;
    let input = unsafe { MmapInput::map_file(&json_file)? };
    let engine = MainEngine::compile_query(&jsonpath_query)?;
    let result = engine.run::<_, IndexResult>(&input)?;
    assert_eq!(result.get(), vec![5usize,], "result != expected");
    Ok(())
}
#[test]
fn whitespace_separators_between_structurals_to_test_correctness_of_index_result_handling_compressed_with_query_select_a_by_child_with_mmap_input_and_index_result_using_recursive_engine(
) -> Result<(), Box<dyn Error>> {
    println ! ("on document index_result.toml running the query $.a (select a by child) with Input impl MmapInput and result mode IndexResult");
    let jsonpath_query = JsonPathQuery::parse("$.a")?;
    let json_file = fs::File::open("./tests/documents/json/compressed/index_result.json")?;
    let input = unsafe { MmapInput::map_file(&json_file)? };
    let engine = RecursiveEngine::compile_query(&jsonpath_query)?;
    let result = engine.run::<_, IndexResult>(&input)?;
    assert_eq!(result.get(), vec![5usize,], "result != expected");
    Ok(())
}
#[test]
fn whitespace_separators_between_structurals_to_test_correctness_of_index_result_handling_compressed_with_query_select_a_by_child_with_owned_bytes_and_count_result_using_main_engine(
) -> Result<(), Box<dyn Error>> {
    println ! ("on document index_result.toml running the query $.a (select a by child) with Input impl OwnedBytes and result mode CountResult");
    let jsonpath_query = JsonPathQuery::parse("$.a")?;
    let raw_json = fs::read_to_string("./tests/documents/json/compressed/index_result.json")?;
    let input = OwnedBytes::new(&raw_json.as_bytes())?;
    let engine = MainEngine::compile_query(&jsonpath_query)?;
    let result = engine.run::<_, CountResult>(&input)?;
    assert_eq!(result.get(), 1u64, "result != expected");
    Ok(())
}
#[test]
fn whitespace_separators_between_structurals_to_test_correctness_of_index_result_handling_compressed_with_query_select_a_by_child_with_owned_bytes_and_count_result_using_recursive_engine(
) -> Result<(), Box<dyn Error>> {
    println ! ("on document index_result.toml running the query $.a (select a by child) with Input impl OwnedBytes and result mode CountResult");
    let jsonpath_query = JsonPathQuery::parse("$.a")?;
    let raw_json = fs::read_to_string("./tests/documents/json/compressed/index_result.json")?;
    let input = OwnedBytes::new(&raw_json.as_bytes())?;
    let engine = RecursiveEngine::compile_query(&jsonpath_query)?;
    let result = engine.run::<_, CountResult>(&input)?;
    assert_eq!(result.get(), 1u64, "result != expected");
    Ok(())
}
#[test]
fn whitespace_separators_between_structurals_to_test_correctness_of_index_result_handling_compressed_with_query_select_a_by_child_with_owned_bytes_and_index_result_using_main_engine(
) -> Result<(), Box<dyn Error>> {
    println ! ("on document index_result.toml running the query $.a (select a by child) with Input impl OwnedBytes and result mode IndexResult");
    let jsonpath_query = JsonPathQuery::parse("$.a")?;
    let raw_json = fs::read_to_string("./tests/documents/json/compressed/index_result.json")?;
    let input = OwnedBytes::new(&raw_json.as_bytes())?;
    let engine = MainEngine::compile_query(&jsonpath_query)?;
    let result = engine.run::<_, IndexResult>(&input)?;
    assert_eq!(result.get(), vec![5usize,], "result != expected");
    Ok(())
}
#[test]
fn whitespace_separators_between_structurals_to_test_correctness_of_index_result_handling_compressed_with_query_select_a_by_child_with_owned_bytes_and_index_result_using_recursive_engine(
) -> Result<(), Box<dyn Error>> {
    println ! ("on document index_result.toml running the query $.a (select a by child) with Input impl OwnedBytes and result mode IndexResult");
    let jsonpath_query = JsonPathQuery::parse("$.a")?;
    let raw_json = fs::read_to_string("./tests/documents/json/compressed/index_result.json")?;
    let input = OwnedBytes::new(&raw_json.as_bytes())?;
    let engine = RecursiveEngine::compile_query(&jsonpath_query)?;
    let result = engine.run::<_, IndexResult>(&input)?;
    assert_eq!(result.get(), vec![5usize,], "result != expected");
    Ok(())
}
#[test]
fn whitespace_separators_between_structurals_to_test_correctness_of_index_result_handling_compressed_with_query_select_a_by_descendant_with_buffered_input_and_count_result_using_main_engine(
) -> Result<(), Box<dyn Error>> {
    println ! ("on document index_result.toml running the query $..a (select a by descendant) with Input impl BufferedInput and result mode CountResult");
    let jsonpath_query = JsonPathQuery::parse("$..a")?;
    let json_file = fs::File::open("./tests/documents/json/compressed/index_result.json")?;
    let input = BufferedInput::new(json_file);
    let engine = MainEngine::compile_query(&jsonpath_query)?;
    let result = engine.run::<_, CountResult>(&input)?;
    assert_eq!(result.get(), 1u64, "result != expected");
    Ok(())
}
#[test]
fn whitespace_separators_between_structurals_to_test_correctness_of_index_result_handling_compressed_with_query_select_a_by_descendant_with_buffered_input_and_count_result_using_recursive_engine(
) -> Result<(), Box<dyn Error>> {
    println ! ("on document index_result.toml running the query $..a (select a by descendant) with Input impl BufferedInput and result mode CountResult");
    let jsonpath_query = JsonPathQuery::parse("$..a")?;
    let json_file = fs::File::open("./tests/documents/json/compressed/index_result.json")?;
    let input = BufferedInput::new(json_file);
    let engine = RecursiveEngine::compile_query(&jsonpath_query)?;
    let result = engine.run::<_, CountResult>(&input)?;
    assert_eq!(result.get(), 1u64, "result != expected");
    Ok(())
}
#[test]
fn whitespace_separators_between_structurals_to_test_correctness_of_index_result_handling_compressed_with_query_select_a_by_descendant_with_buffered_input_and_index_result_using_main_engine(
) -> Result<(), Box<dyn Error>> {
    println ! ("on document index_result.toml running the query $..a (select a by descendant) with Input impl BufferedInput and result mode IndexResult");
    let jsonpath_query = JsonPathQuery::parse("$..a")?;
    let json_file = fs::File::open("./tests/documents/json/compressed/index_result.json")?;
    let input = BufferedInput::new(json_file);
    let engine = MainEngine::compile_query(&jsonpath_query)?;
    let result = engine.run::<_, IndexResult>(&input)?;
    assert_eq!(result.get(), vec![5usize,], "result != expected");
    Ok(())
}
#[test]
fn whitespace_separators_between_structurals_to_test_correctness_of_index_result_handling_compressed_with_query_select_a_by_descendant_with_buffered_input_and_index_result_using_recursive_engine(
) -> Result<(), Box<dyn Error>> {
    println ! ("on document index_result.toml running the query $..a (select a by descendant) with Input impl BufferedInput and result mode IndexResult");
    let jsonpath_query = JsonPathQuery::parse("$..a")?;
    let json_file = fs::File::open("./tests/documents/json/compressed/index_result.json")?;
    let input = BufferedInput::new(json_file);
    let engine = RecursiveEngine::compile_query(&jsonpath_query)?;
    let result = engine.run::<_, IndexResult>(&input)?;
    assert_eq!(result.get(), vec![5usize,], "result != expected");
    Ok(())
}
#[test]
fn whitespace_separators_between_structurals_to_test_correctness_of_index_result_handling_compressed_with_query_select_a_by_descendant_with_mmap_input_and_count_result_using_main_engine(
) -> Result<(), Box<dyn Error>> {
    println ! ("on document index_result.toml running the query $..a (select a by descendant) with Input impl MmapInput and result mode CountResult");
    let jsonpath_query = JsonPathQuery::parse("$..a")?;
    let json_file = fs::File::open("./tests/documents/json/compressed/index_result.json")?;
    let input = unsafe { MmapInput::map_file(&json_file)? };
    let engine = MainEngine::compile_query(&jsonpath_query)?;
    let result = engine.run::<_, CountResult>(&input)?;
    assert_eq!(result.get(), 1u64, "result != expected");
    Ok(())
}
#[test]
fn whitespace_separators_between_structurals_to_test_correctness_of_index_result_handling_compressed_with_query_select_a_by_descendant_with_mmap_input_and_count_result_using_recursive_engine(
) -> Result<(), Box<dyn Error>> {
    println ! ("on document index_result.toml running the query $..a (select a by descendant) with Input impl MmapInput and result mode CountResult");
    let jsonpath_query = JsonPathQuery::parse("$..a")?;
    let json_file = fs::File::open("./tests/documents/json/compressed/index_result.json")?;
    let input = unsafe { MmapInput::map_file(&json_file)? };
    let engine = RecursiveEngine::compile_query(&jsonpath_query)?;
    let result = engine.run::<_, CountResult>(&input)?;
    assert_eq!(result.get(), 1u64, "result != expected");
    Ok(())
}
#[test]
fn whitespace_separators_between_structurals_to_test_correctness_of_index_result_handling_compressed_with_query_select_a_by_descendant_with_mmap_input_and_index_result_using_main_engine(
) -> Result<(), Box<dyn Error>> {
    println ! ("on document index_result.toml running the query $..a (select a by descendant) with Input impl MmapInput and result mode IndexResult");
    let jsonpath_query = JsonPathQuery::parse("$..a")?;
    let json_file = fs::File::open("./tests/documents/json/compressed/index_result.json")?;
    let input = unsafe { MmapInput::map_file(&json_file)? };
    let engine = MainEngine::compile_query(&jsonpath_query)?;
    let result = engine.run::<_, IndexResult>(&input)?;
    assert_eq!(result.get(), vec![5usize,], "result != expected");
    Ok(())
}
#[test]
fn whitespace_separators_between_structurals_to_test_correctness_of_index_result_handling_compressed_with_query_select_a_by_descendant_with_mmap_input_and_index_result_using_recursive_engine(
) -> Result<(), Box<dyn Error>> {
    println ! ("on document index_result.toml running the query $..a (select a by descendant) with Input impl MmapInput and result mode IndexResult");
    let jsonpath_query = JsonPathQuery::parse("$..a")?;
    let json_file = fs::File::open("./tests/documents/json/compressed/index_result.json")?;
    let input = unsafe { MmapInput::map_file(&json_file)? };
    let engine = RecursiveEngine::compile_query(&jsonpath_query)?;
    let result = engine.run::<_, IndexResult>(&input)?;
    assert_eq!(result.get(), vec![5usize,], "result != expected");
    Ok(())
}
#[test]
fn whitespace_separators_between_structurals_to_test_correctness_of_index_result_handling_compressed_with_query_select_a_by_descendant_with_owned_bytes_and_count_result_using_main_engine(
) -> Result<(), Box<dyn Error>> {
    println ! ("on document index_result.toml running the query $..a (select a by descendant) with Input impl OwnedBytes and result mode CountResult");
    let jsonpath_query = JsonPathQuery::parse("$..a")?;
    let raw_json = fs::read_to_string("./tests/documents/json/compressed/index_result.json")?;
    let input = OwnedBytes::new(&raw_json.as_bytes())?;
    let engine = MainEngine::compile_query(&jsonpath_query)?;
    let result = engine.run::<_, CountResult>(&input)?;
    assert_eq!(result.get(), 1u64, "result != expected");
    Ok(())
}
#[test]
fn whitespace_separators_between_structurals_to_test_correctness_of_index_result_handling_compressed_with_query_select_a_by_descendant_with_owned_bytes_and_count_result_using_recursive_engine(
) -> Result<(), Box<dyn Error>> {
    println ! ("on document index_result.toml running the query $..a (select a by descendant) with Input impl OwnedBytes and result mode CountResult");
    let jsonpath_query = JsonPathQuery::parse("$..a")?;
    let raw_json = fs::read_to_string("./tests/documents/json/compressed/index_result.json")?;
    let input = OwnedBytes::new(&raw_json.as_bytes())?;
    let engine = RecursiveEngine::compile_query(&jsonpath_query)?;
    let result = engine.run::<_, CountResult>(&input)?;
    assert_eq!(result.get(), 1u64, "result != expected");
    Ok(())
}
#[test]
fn whitespace_separators_between_structurals_to_test_correctness_of_index_result_handling_compressed_with_query_select_a_by_descendant_with_owned_bytes_and_index_result_using_main_engine(
) -> Result<(), Box<dyn Error>> {
    println ! ("on document index_result.toml running the query $..a (select a by descendant) with Input impl OwnedBytes and result mode IndexResult");
    let jsonpath_query = JsonPathQuery::parse("$..a")?;
    let raw_json = fs::read_to_string("./tests/documents/json/compressed/index_result.json")?;
    let input = OwnedBytes::new(&raw_json.as_bytes())?;
    let engine = MainEngine::compile_query(&jsonpath_query)?;
    let result = engine.run::<_, IndexResult>(&input)?;
    assert_eq!(result.get(), vec![5usize,], "result != expected");
    Ok(())
}
#[test]
fn whitespace_separators_between_structurals_to_test_correctness_of_index_result_handling_compressed_with_query_select_a_by_descendant_with_owned_bytes_and_index_result_using_recursive_engine(
) -> Result<(), Box<dyn Error>> {
    println ! ("on document index_result.toml running the query $..a (select a by descendant) with Input impl OwnedBytes and result mode IndexResult");
    let jsonpath_query = JsonPathQuery::parse("$..a")?;
    let raw_json = fs::read_to_string("./tests/documents/json/compressed/index_result.json")?;
    let input = OwnedBytes::new(&raw_json.as_bytes())?;
    let engine = RecursiveEngine::compile_query(&jsonpath_query)?;
    let result = engine.run::<_, IndexResult>(&input)?;
    assert_eq!(result.get(), vec![5usize,], "result != expected");
    Ok(())
}
#[test]
fn whitespace_separators_between_structurals_to_test_correctness_of_index_result_handling_compressed_with_query_select_each_item_on_the_list_with_wildcard_with_buffered_input_and_count_result_using_main_engine(
) -> Result<(), Box<dyn Error>> {
    println ! ("on document index_result.toml running the query $.a[*] (select each item on the list with wildcard) with Input impl BufferedInput and result mode CountResult");
    let jsonpath_query = JsonPathQuery::parse("$.a[*]")?;
    let json_file = fs::File::open("./tests/documents/json/compressed/index_result.json")?;
    let input = BufferedInput::new(json_file);
    let engine = MainEngine::compile_query(&jsonpath_query)?;
    let result = engine.run::<_, CountResult>(&input)?;
    assert_eq!(result.get(), 3u64, "result != expected");
    Ok(())
}
#[test]
fn whitespace_separators_between_structurals_to_test_correctness_of_index_result_handling_compressed_with_query_select_each_item_on_the_list_with_wildcard_with_buffered_input_and_count_result_using_recursive_engine(
) -> Result<(), Box<dyn Error>> {
    println ! ("on document index_result.toml running the query $.a[*] (select each item on the list with wildcard) with Input impl BufferedInput and result mode CountResult");
    let jsonpath_query = JsonPathQuery::parse("$.a[*]")?;
    let json_file = fs::File::open("./tests/documents/json/compressed/index_result.json")?;
    let input = BufferedInput::new(json_file);
    let engine = RecursiveEngine::compile_query(&jsonpath_query)?;
    let result = engine.run::<_, CountResult>(&input)?;
    assert_eq!(result.get(), 3u64, "result != expected");
    Ok(())
}
#[test]
fn whitespace_separators_between_structurals_to_test_correctness_of_index_result_handling_compressed_with_query_select_each_item_on_the_list_with_wildcard_with_buffered_input_and_index_result_using_main_engine(
) -> Result<(), Box<dyn Error>> {
    println ! ("on document index_result.toml running the query $.a[*] (select each item on the list with wildcard) with Input impl BufferedInput and result mode IndexResult");
    let jsonpath_query = JsonPathQuery::parse("$.a[*]")?;
    let json_file = fs::File::open("./tests/documents/json/compressed/index_result.json")?;
    let input = BufferedInput::new(json_file);
    let engine = MainEngine::compile_query(&jsonpath_query)?;
    let result = engine.run::<_, IndexResult>(&input)?;
    assert_eq!(result.get(), vec![6usize, 8usize, 10usize,], "result != expected");
    Ok(())
}
#[test]
fn whitespace_separators_between_structurals_to_test_correctness_of_index_result_handling_compressed_with_query_select_each_item_on_the_list_with_wildcard_with_buffered_input_and_index_result_using_recursive_engine(
) -> Result<(), Box<dyn Error>> {
    println ! ("on document index_result.toml running the query $.a[*] (select each item on the list with wildcard) with Input impl BufferedInput and result mode IndexResult");
    let jsonpath_query = JsonPathQuery::parse("$.a[*]")?;
    let json_file = fs::File::open("./tests/documents/json/compressed/index_result.json")?;
    let input = BufferedInput::new(json_file);
    let engine = RecursiveEngine::compile_query(&jsonpath_query)?;
    let result = engine.run::<_, IndexResult>(&input)?;
    assert_eq!(result.get(), vec![6usize, 8usize, 10usize,], "result != expected");
    Ok(())
}
#[test]
fn whitespace_separators_between_structurals_to_test_correctness_of_index_result_handling_compressed_with_query_select_each_item_on_the_list_with_wildcard_with_mmap_input_and_count_result_using_main_engine(
) -> Result<(), Box<dyn Error>> {
    println ! ("on document index_result.toml running the query $.a[*] (select each item on the list with wildcard) with Input impl MmapInput and result mode CountResult");
    let jsonpath_query = JsonPathQuery::parse("$.a[*]")?;
    let json_file = fs::File::open("./tests/documents/json/compressed/index_result.json")?;
    let input = unsafe { MmapInput::map_file(&json_file)? };
    let engine = MainEngine::compile_query(&jsonpath_query)?;
    let result = engine.run::<_, CountResult>(&input)?;
    assert_eq!(result.get(), 3u64, "result != expected");
    Ok(())
}
#[test]
fn whitespace_separators_between_structurals_to_test_correctness_of_index_result_handling_compressed_with_query_select_each_item_on_the_list_with_wildcard_with_mmap_input_and_count_result_using_recursive_engine(
) -> Result<(), Box<dyn Error>> {
    println ! ("on document index_result.toml running the query $.a[*] (select each item on the list with wildcard) with Input impl MmapInput and result mode CountResult");
    let jsonpath_query = JsonPathQuery::parse("$.a[*]")?;
    let json_file = fs::File::open("./tests/documents/json/compressed/index_result.json")?;
    let input = unsafe { MmapInput::map_file(&json_file)? };
    let engine = RecursiveEngine::compile_query(&jsonpath_query)?;
    let result = engine.run::<_, CountResult>(&input)?;
    assert_eq!(result.get(), 3u64, "result != expected");
    Ok(())
}
#[test]
fn whitespace_separators_between_structurals_to_test_correctness_of_index_result_handling_compressed_with_query_select_each_item_on_the_list_with_wildcard_with_mmap_input_and_index_result_using_main_engine(
) -> Result<(), Box<dyn Error>> {
    println ! ("on document index_result.toml running the query $.a[*] (select each item on the list with wildcard) with Input impl MmapInput and result mode IndexResult");
    let jsonpath_query = JsonPathQuery::parse("$.a[*]")?;
    let json_file = fs::File::open("./tests/documents/json/compressed/index_result.json")?;
    let input = unsafe { MmapInput::map_file(&json_file)? };
    let engine = MainEngine::compile_query(&jsonpath_query)?;
    let result = engine.run::<_, IndexResult>(&input)?;
    assert_eq!(result.get(), vec![6usize, 8usize, 10usize,], "result != expected");
    Ok(())
}
#[test]
fn whitespace_separators_between_structurals_to_test_correctness_of_index_result_handling_compressed_with_query_select_each_item_on_the_list_with_wildcard_with_mmap_input_and_index_result_using_recursive_engine(
) -> Result<(), Box<dyn Error>> {
    println ! ("on document index_result.toml running the query $.a[*] (select each item on the list with wildcard) with Input impl MmapInput and result mode IndexResult");
    let jsonpath_query = JsonPathQuery::parse("$.a[*]")?;
    let json_file = fs::File::open("./tests/documents/json/compressed/index_result.json")?;
    let input = unsafe { MmapInput::map_file(&json_file)? };
    let engine = RecursiveEngine::compile_query(&jsonpath_query)?;
    let result = engine.run::<_, IndexResult>(&input)?;
    assert_eq!(result.get(), vec![6usize, 8usize, 10usize,], "result != expected");
    Ok(())
}
#[test]
fn whitespace_separators_between_structurals_to_test_correctness_of_index_result_handling_compressed_with_query_select_each_item_on_the_list_with_wildcard_with_owned_bytes_and_count_result_using_main_engine(
) -> Result<(), Box<dyn Error>> {
    println ! ("on document index_result.toml running the query $.a[*] (select each item on the list with wildcard) with Input impl OwnedBytes and result mode CountResult");
    let jsonpath_query = JsonPathQuery::parse("$.a[*]")?;
    let raw_json = fs::read_to_string("./tests/documents/json/compressed/index_result.json")?;
    let input = OwnedBytes::new(&raw_json.as_bytes())?;
    let engine = MainEngine::compile_query(&jsonpath_query)?;
    let result = engine.run::<_, CountResult>(&input)?;
    assert_eq!(result.get(), 3u64, "result != expected");
    Ok(())
}
#[test]
fn whitespace_separators_between_structurals_to_test_correctness_of_index_result_handling_compressed_with_query_select_each_item_on_the_list_with_wildcard_with_owned_bytes_and_count_result_using_recursive_engine(
) -> Result<(), Box<dyn Error>> {
    println ! ("on document index_result.toml running the query $.a[*] (select each item on the list with wildcard) with Input impl OwnedBytes and result mode CountResult");
    let jsonpath_query = JsonPathQuery::parse("$.a[*]")?;
    let raw_json = fs::read_to_string("./tests/documents/json/compressed/index_result.json")?;
    let input = OwnedBytes::new(&raw_json.as_bytes())?;
    let engine = RecursiveEngine::compile_query(&jsonpath_query)?;
    let result = engine.run::<_, CountResult>(&input)?;
    assert_eq!(result.get(), 3u64, "result != expected");
    Ok(())
}
#[test]
fn whitespace_separators_between_structurals_to_test_correctness_of_index_result_handling_compressed_with_query_select_each_item_on_the_list_with_wildcard_with_owned_bytes_and_index_result_using_main_engine(
) -> Result<(), Box<dyn Error>> {
    println ! ("on document index_result.toml running the query $.a[*] (select each item on the list with wildcard) with Input impl OwnedBytes and result mode IndexResult");
    let jsonpath_query = JsonPathQuery::parse("$.a[*]")?;
    let raw_json = fs::read_to_string("./tests/documents/json/compressed/index_result.json")?;
    let input = OwnedBytes::new(&raw_json.as_bytes())?;
    let engine = MainEngine::compile_query(&jsonpath_query)?;
    let result = engine.run::<_, IndexResult>(&input)?;
    assert_eq!(result.get(), vec![6usize, 8usize, 10usize,], "result != expected");
    Ok(())
}
#[test]
fn whitespace_separators_between_structurals_to_test_correctness_of_index_result_handling_compressed_with_query_select_each_item_on_the_list_with_wildcard_with_owned_bytes_and_index_result_using_recursive_engine(
) -> Result<(), Box<dyn Error>> {
    println ! ("on document index_result.toml running the query $.a[*] (select each item on the list with wildcard) with Input impl OwnedBytes and result mode IndexResult");
    let jsonpath_query = JsonPathQuery::parse("$.a[*]")?;
    let raw_json = fs::read_to_string("./tests/documents/json/compressed/index_result.json")?;
    let input = OwnedBytes::new(&raw_json.as_bytes())?;
    let engine = RecursiveEngine::compile_query(&jsonpath_query)?;
    let result = engine.run::<_, IndexResult>(&input)?;
    assert_eq!(result.get(), vec![6usize, 8usize, 10usize,], "result != expected");
    Ok(())
}
#[test]
fn whitespace_separators_between_structurals_to_test_correctness_of_index_result_handling_compressed_with_query_select_root_with_buffered_input_and_count_result_using_main_engine(
) -> Result<(), Box<dyn Error>> {
    println ! ("on document index_result.toml running the query $ (select root) with Input impl BufferedInput and result mode CountResult");
    let jsonpath_query = JsonPathQuery::parse("$")?;
    let json_file = fs::File::open("./tests/documents/json/compressed/index_result.json")?;
    let input = BufferedInput::new(json_file);
    let engine = MainEngine::compile_query(&jsonpath_query)?;
    let result = engine.run::<_, CountResult>(&input)?;
    assert_eq!(result.get(), 1u64, "result != expected");
    Ok(())
}
#[test]
fn whitespace_separators_between_structurals_to_test_correctness_of_index_result_handling_compressed_with_query_select_root_with_buffered_input_and_count_result_using_recursive_engine(
) -> Result<(), Box<dyn Error>> {
    println ! ("on document index_result.toml running the query $ (select root) with Input impl BufferedInput and result mode CountResult");
    let jsonpath_query = JsonPathQuery::parse("$")?;
    let json_file = fs::File::open("./tests/documents/json/compressed/index_result.json")?;
    let input = BufferedInput::new(json_file);
    let engine = RecursiveEngine::compile_query(&jsonpath_query)?;
    let result = engine.run::<_, CountResult>(&input)?;
    assert_eq!(result.get(), 1u64, "result != expected");
    Ok(())
}
#[test]
fn whitespace_separators_between_structurals_to_test_correctness_of_index_result_handling_compressed_with_query_select_root_with_buffered_input_and_index_result_using_main_engine(
) -> Result<(), Box<dyn Error>> {
    println ! ("on document index_result.toml running the query $ (select root) with Input impl BufferedInput and result mode IndexResult");
    let jsonpath_query = JsonPathQuery::parse("$")?;
    let json_file = fs::File::open("./tests/documents/json/compressed/index_result.json")?;
    let input = BufferedInput::new(json_file);
    let engine = MainEngine::compile_query(&jsonpath_query)?;
    let result = engine.run::<_, IndexResult>(&input)?;
    assert_eq!(result.get(), vec![0usize,], "result != expected");
    Ok(())
}
#[test]
fn whitespace_separators_between_structurals_to_test_correctness_of_index_result_handling_compressed_with_query_select_root_with_buffered_input_and_index_result_using_recursive_engine(
) -> Result<(), Box<dyn Error>> {
    println ! ("on document index_result.toml running the query $ (select root) with Input impl BufferedInput and result mode IndexResult");
    let jsonpath_query = JsonPathQuery::parse("$")?;
    let json_file = fs::File::open("./tests/documents/json/compressed/index_result.json")?;
    let input = BufferedInput::new(json_file);
    let engine = RecursiveEngine::compile_query(&jsonpath_query)?;
    let result = engine.run::<_, IndexResult>(&input)?;
    assert_eq!(result.get(), vec![0usize,], "result != expected");
    Ok(())
}
#[test]
fn whitespace_separators_between_structurals_to_test_correctness_of_index_result_handling_compressed_with_query_select_root_with_mmap_input_and_count_result_using_main_engine(
) -> Result<(), Box<dyn Error>> {
    println ! ("on document index_result.toml running the query $ (select root) with Input impl MmapInput and result mode CountResult");
    let jsonpath_query = JsonPathQuery::parse("$")?;
    let json_file = fs::File::open("./tests/documents/json/compressed/index_result.json")?;
    let input = unsafe { MmapInput::map_file(&json_file)? };
    let engine = MainEngine::compile_query(&jsonpath_query)?;
    let result = engine.run::<_, CountResult>(&input)?;
    assert_eq!(result.get(), 1u64, "result != expected");
    Ok(())
}
#[test]
fn whitespace_separators_between_structurals_to_test_correctness_of_index_result_handling_compressed_with_query_select_root_with_mmap_input_and_count_result_using_recursive_engine(
) -> Result<(), Box<dyn Error>> {
    println ! ("on document index_result.toml running the query $ (select root) with Input impl MmapInput and result mode CountResult");
    let jsonpath_query = JsonPathQuery::parse("$")?;
    let json_file = fs::File::open("./tests/documents/json/compressed/index_result.json")?;
    let input = unsafe { MmapInput::map_file(&json_file)? };
    let engine = RecursiveEngine::compile_query(&jsonpath_query)?;
    let result = engine.run::<_, CountResult>(&input)?;
    assert_eq!(result.get(), 1u64, "result != expected");
    Ok(())
}
#[test]
fn whitespace_separators_between_structurals_to_test_correctness_of_index_result_handling_compressed_with_query_select_root_with_mmap_input_and_index_result_using_main_engine(
) -> Result<(), Box<dyn Error>> {
    println ! ("on document index_result.toml running the query $ (select root) with Input impl MmapInput and result mode IndexResult");
    let jsonpath_query = JsonPathQuery::parse("$")?;
    let json_file = fs::File::open("./tests/documents/json/compressed/index_result.json")?;
    let input = unsafe { MmapInput::map_file(&json_file)? };
    let engine = MainEngine::compile_query(&jsonpath_query)?;
    let result = engine.run::<_, IndexResult>(&input)?;
    assert_eq!(result.get(), vec![0usize,], "result != expected");
    Ok(())
}
#[test]
fn whitespace_separators_between_structurals_to_test_correctness_of_index_result_handling_compressed_with_query_select_root_with_mmap_input_and_index_result_using_recursive_engine(
) -> Result<(), Box<dyn Error>> {
    println ! ("on document index_result.toml running the query $ (select root) with Input impl MmapInput and result mode IndexResult");
    let jsonpath_query = JsonPathQuery::parse("$")?;
    let json_file = fs::File::open("./tests/documents/json/compressed/index_result.json")?;
    let input = unsafe { MmapInput::map_file(&json_file)? };
    let engine = RecursiveEngine::compile_query(&jsonpath_query)?;
    let result = engine.run::<_, IndexResult>(&input)?;
    assert_eq!(result.get(), vec![0usize,], "result != expected");
    Ok(())
}
#[test]
fn whitespace_separators_between_structurals_to_test_correctness_of_index_result_handling_compressed_with_query_select_root_with_owned_bytes_and_count_result_using_main_engine(
) -> Result<(), Box<dyn Error>> {
    println ! ("on document index_result.toml running the query $ (select root) with Input impl OwnedBytes and result mode CountResult");
    let jsonpath_query = JsonPathQuery::parse("$")?;
    let raw_json = fs::read_to_string("./tests/documents/json/compressed/index_result.json")?;
    let input = OwnedBytes::new(&raw_json.as_bytes())?;
    let engine = MainEngine::compile_query(&jsonpath_query)?;
    let result = engine.run::<_, CountResult>(&input)?;
    assert_eq!(result.get(), 1u64, "result != expected");
    Ok(())
}
#[test]
fn whitespace_separators_between_structurals_to_test_correctness_of_index_result_handling_compressed_with_query_select_root_with_owned_bytes_and_count_result_using_recursive_engine(
) -> Result<(), Box<dyn Error>> {
    println ! ("on document index_result.toml running the query $ (select root) with Input impl OwnedBytes and result mode CountResult");
    let jsonpath_query = JsonPathQuery::parse("$")?;
    let raw_json = fs::read_to_string("./tests/documents/json/compressed/index_result.json")?;
    let input = OwnedBytes::new(&raw_json.as_bytes())?;
    let engine = RecursiveEngine::compile_query(&jsonpath_query)?;
    let result = engine.run::<_, CountResult>(&input)?;
    assert_eq!(result.get(), 1u64, "result != expected");
    Ok(())
}
#[test]
fn whitespace_separators_between_structurals_to_test_correctness_of_index_result_handling_compressed_with_query_select_root_with_owned_bytes_and_index_result_using_main_engine(
) -> Result<(), Box<dyn Error>> {
    println ! ("on document index_result.toml running the query $ (select root) with Input impl OwnedBytes and result mode IndexResult");
    let jsonpath_query = JsonPathQuery::parse("$")?;
    let raw_json = fs::read_to_string("./tests/documents/json/compressed/index_result.json")?;
    let input = OwnedBytes::new(&raw_json.as_bytes())?;
    let engine = MainEngine::compile_query(&jsonpath_query)?;
    let result = engine.run::<_, IndexResult>(&input)?;
    assert_eq!(result.get(), vec![0usize,], "result != expected");
    Ok(())
}
#[test]
fn whitespace_separators_between_structurals_to_test_correctness_of_index_result_handling_compressed_with_query_select_root_with_owned_bytes_and_index_result_using_recursive_engine(
) -> Result<(), Box<dyn Error>> {
    println ! ("on document index_result.toml running the query $ (select root) with Input impl OwnedBytes and result mode IndexResult");
    let jsonpath_query = JsonPathQuery::parse("$")?;
    let raw_json = fs::read_to_string("./tests/documents/json/compressed/index_result.json")?;
    let input = OwnedBytes::new(&raw_json.as_bytes())?;
    let engine = RecursiveEngine::compile_query(&jsonpath_query)?;
    let result = engine.run::<_, IndexResult>(&input)?;
    assert_eq!(result.get(), vec![0usize,], "result != expected");
    Ok(())
}
#[test]
fn whitespace_separators_between_structurals_to_test_correctness_of_index_result_handling_with_query_select_a_by_child_with_buffered_input_and_count_result_using_main_engine(
) -> Result<(), Box<dyn Error>> {
    println ! ("on document index_result.toml running the query $.a (select a by child) with Input impl BufferedInput and result mode CountResult");
    let jsonpath_query = JsonPathQuery::parse("$.a")?;
    let json_file = fs::File::open("./tests/documents/json/index_result.json")?;
    let input = BufferedInput::new(json_file);
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
    let json_file = fs::File::open("./tests/documents/json/index_result.json")?;
    let input = BufferedInput::new(json_file);
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
    let json_file = fs::File::open("./tests/documents/json/index_result.json")?;
    let input = BufferedInput::new(json_file);
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
    let json_file = fs::File::open("./tests/documents/json/index_result.json")?;
    let input = BufferedInput::new(json_file);
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
    let json_file = fs::File::open("./tests/documents/json/index_result.json")?;
    let input = unsafe { MmapInput::map_file(&json_file)? };
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
    let json_file = fs::File::open("./tests/documents/json/index_result.json")?;
    let input = unsafe { MmapInput::map_file(&json_file)? };
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
    let json_file = fs::File::open("./tests/documents/json/index_result.json")?;
    let input = unsafe { MmapInput::map_file(&json_file)? };
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
    let json_file = fs::File::open("./tests/documents/json/index_result.json")?;
    let input = unsafe { MmapInput::map_file(&json_file)? };
    let engine = RecursiveEngine::compile_query(&jsonpath_query)?;
    let result = engine.run::<_, IndexResult>(&input)?;
    assert_eq!(result.get(), vec![9usize,], "result != expected");
    Ok(())
}
#[test]
fn whitespace_separators_between_structurals_to_test_correctness_of_index_result_handling_with_query_select_a_by_child_with_owned_bytes_and_count_result_using_main_engine(
) -> Result<(), Box<dyn Error>> {
    println ! ("on document index_result.toml running the query $.a (select a by child) with Input impl OwnedBytes and result mode CountResult");
    let jsonpath_query = JsonPathQuery::parse("$.a")?;
    let raw_json = fs::read_to_string("./tests/documents/json/index_result.json")?;
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
    let raw_json = fs::read_to_string("./tests/documents/json/index_result.json")?;
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
    let raw_json = fs::read_to_string("./tests/documents/json/index_result.json")?;
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
    let raw_json = fs::read_to_string("./tests/documents/json/index_result.json")?;
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
    let json_file = fs::File::open("./tests/documents/json/index_result.json")?;
    let input = BufferedInput::new(json_file);
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
    let json_file = fs::File::open("./tests/documents/json/index_result.json")?;
    let input = BufferedInput::new(json_file);
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
    let json_file = fs::File::open("./tests/documents/json/index_result.json")?;
    let input = BufferedInput::new(json_file);
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
    let json_file = fs::File::open("./tests/documents/json/index_result.json")?;
    let input = BufferedInput::new(json_file);
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
    let json_file = fs::File::open("./tests/documents/json/index_result.json")?;
    let input = unsafe { MmapInput::map_file(&json_file)? };
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
    let json_file = fs::File::open("./tests/documents/json/index_result.json")?;
    let input = unsafe { MmapInput::map_file(&json_file)? };
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
    let json_file = fs::File::open("./tests/documents/json/index_result.json")?;
    let input = unsafe { MmapInput::map_file(&json_file)? };
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
    let json_file = fs::File::open("./tests/documents/json/index_result.json")?;
    let input = unsafe { MmapInput::map_file(&json_file)? };
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
    let raw_json = fs::read_to_string("./tests/documents/json/index_result.json")?;
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
    let raw_json = fs::read_to_string("./tests/documents/json/index_result.json")?;
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
    let raw_json = fs::read_to_string("./tests/documents/json/index_result.json")?;
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
    let raw_json = fs::read_to_string("./tests/documents/json/index_result.json")?;
    let input = OwnedBytes::new(&raw_json.as_bytes())?;
    let engine = RecursiveEngine::compile_query(&jsonpath_query)?;
    let result = engine.run::<_, IndexResult>(&input)?;
    assert_eq!(result.get(), vec![9usize,], "result != expected");
    Ok(())
}
#[test]
fn whitespace_separators_between_structurals_to_test_correctness_of_index_result_handling_with_query_select_each_item_on_the_list_with_wildcard_with_buffered_input_and_count_result_using_main_engine(
) -> Result<(), Box<dyn Error>> {
    println ! ("on document index_result.toml running the query $.a[*] (select each item on the list with wildcard) with Input impl BufferedInput and result mode CountResult");
    let jsonpath_query = JsonPathQuery::parse("$.a[*]")?;
    let json_file = fs::File::open("./tests/documents/json/index_result.json")?;
    let input = BufferedInput::new(json_file);
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
    let json_file = fs::File::open("./tests/documents/json/index_result.json")?;
    let input = BufferedInput::new(json_file);
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
    let json_file = fs::File::open("./tests/documents/json/index_result.json")?;
    let input = BufferedInput::new(json_file);
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
    let json_file = fs::File::open("./tests/documents/json/index_result.json")?;
    let input = BufferedInput::new(json_file);
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
    let json_file = fs::File::open("./tests/documents/json/index_result.json")?;
    let input = unsafe { MmapInput::map_file(&json_file)? };
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
    let json_file = fs::File::open("./tests/documents/json/index_result.json")?;
    let input = unsafe { MmapInput::map_file(&json_file)? };
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
    let json_file = fs::File::open("./tests/documents/json/index_result.json")?;
    let input = unsafe { MmapInput::map_file(&json_file)? };
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
    let json_file = fs::File::open("./tests/documents/json/index_result.json")?;
    let input = unsafe { MmapInput::map_file(&json_file)? };
    let engine = RecursiveEngine::compile_query(&jsonpath_query)?;
    let result = engine.run::<_, IndexResult>(&input)?;
    assert_eq!(result.get(), vec![10usize, 16usize, 22usize,], "result != expected");
    Ok(())
}
#[test]
fn whitespace_separators_between_structurals_to_test_correctness_of_index_result_handling_with_query_select_each_item_on_the_list_with_wildcard_with_owned_bytes_and_count_result_using_main_engine(
) -> Result<(), Box<dyn Error>> {
    println ! ("on document index_result.toml running the query $.a[*] (select each item on the list with wildcard) with Input impl OwnedBytes and result mode CountResult");
    let jsonpath_query = JsonPathQuery::parse("$.a[*]")?;
    let raw_json = fs::read_to_string("./tests/documents/json/index_result.json")?;
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
    let raw_json = fs::read_to_string("./tests/documents/json/index_result.json")?;
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
    let raw_json = fs::read_to_string("./tests/documents/json/index_result.json")?;
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
    let raw_json = fs::read_to_string("./tests/documents/json/index_result.json")?;
    let input = OwnedBytes::new(&raw_json.as_bytes())?;
    let engine = RecursiveEngine::compile_query(&jsonpath_query)?;
    let result = engine.run::<_, IndexResult>(&input)?;
    assert_eq!(result.get(), vec![10usize, 16usize, 22usize,], "result != expected");
    Ok(())
}
#[test]
fn whitespace_separators_between_structurals_to_test_correctness_of_index_result_handling_with_query_select_root_with_buffered_input_and_count_result_using_main_engine(
) -> Result<(), Box<dyn Error>> {
    println ! ("on document index_result.toml running the query $ (select root) with Input impl BufferedInput and result mode CountResult");
    let jsonpath_query = JsonPathQuery::parse("$")?;
    let json_file = fs::File::open("./tests/documents/json/index_result.json")?;
    let input = BufferedInput::new(json_file);
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
    let json_file = fs::File::open("./tests/documents/json/index_result.json")?;
    let input = BufferedInput::new(json_file);
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
    let json_file = fs::File::open("./tests/documents/json/index_result.json")?;
    let input = BufferedInput::new(json_file);
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
    let json_file = fs::File::open("./tests/documents/json/index_result.json")?;
    let input = BufferedInput::new(json_file);
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
    let json_file = fs::File::open("./tests/documents/json/index_result.json")?;
    let input = unsafe { MmapInput::map_file(&json_file)? };
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
    let json_file = fs::File::open("./tests/documents/json/index_result.json")?;
    let input = unsafe { MmapInput::map_file(&json_file)? };
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
    let json_file = fs::File::open("./tests/documents/json/index_result.json")?;
    let input = unsafe { MmapInput::map_file(&json_file)? };
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
    let json_file = fs::File::open("./tests/documents/json/index_result.json")?;
    let input = unsafe { MmapInput::map_file(&json_file)? };
    let engine = RecursiveEngine::compile_query(&jsonpath_query)?;
    let result = engine.run::<_, IndexResult>(&input)?;
    assert_eq!(result.get(), vec![0usize,], "result != expected");
    Ok(())
}
#[test]
fn whitespace_separators_between_structurals_to_test_correctness_of_index_result_handling_with_query_select_root_with_owned_bytes_and_count_result_using_main_engine(
) -> Result<(), Box<dyn Error>> {
    println ! ("on document index_result.toml running the query $ (select root) with Input impl OwnedBytes and result mode CountResult");
    let jsonpath_query = JsonPathQuery::parse("$")?;
    let raw_json = fs::read_to_string("./tests/documents/json/index_result.json")?;
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
    let raw_json = fs::read_to_string("./tests/documents/json/index_result.json")?;
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
    let raw_json = fs::read_to_string("./tests/documents/json/index_result.json")?;
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
    let raw_json = fs::read_to_string("./tests/documents/json/index_result.json")?;
    let input = OwnedBytes::new(&raw_json.as_bytes())?;
    let engine = RecursiveEngine::compile_query(&jsonpath_query)?;
    let result = engine.run::<_, IndexResult>(&input)?;
    assert_eq!(result.get(), vec![0usize,], "result != expected");
    Ok(())
}
#[test]
fn with_query_look_for_b_on_at_least_one_level_of_nesting_with_buffered_input_and_count_result_using_main_engine(
) -> Result<(), Box<dyn Error>> {
    println ! ("on document atomic_descendant.toml running the query $..*..b (look for 'b' on at least one level of nesting) with Input impl BufferedInput and result mode CountResult");
    let jsonpath_query = JsonPathQuery::parse("$..*..b")?;
    let json_file = fs::File::open("./tests/documents/json/atomic_descendant.json")?;
    let input = BufferedInput::new(json_file);
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
    let json_file = fs::File::open("./tests/documents/json/atomic_descendant.json")?;
    let input = BufferedInput::new(json_file);
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
    let json_file = fs::File::open("./tests/documents/json/atomic_descendant.json")?;
    let input = BufferedInput::new(json_file);
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
    let json_file = fs::File::open("./tests/documents/json/atomic_descendant.json")?;
    let input = BufferedInput::new(json_file);
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
    let json_file = fs::File::open("./tests/documents/json/atomic_descendant.json")?;
    let input = unsafe { MmapInput::map_file(&json_file)? };
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
    let json_file = fs::File::open("./tests/documents/json/atomic_descendant.json")?;
    let input = unsafe { MmapInput::map_file(&json_file)? };
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
    let json_file = fs::File::open("./tests/documents/json/atomic_descendant.json")?;
    let input = unsafe { MmapInput::map_file(&json_file)? };
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
    let json_file = fs::File::open("./tests/documents/json/atomic_descendant.json")?;
    let input = unsafe { MmapInput::map_file(&json_file)? };
    let engine = RecursiveEngine::compile_query(&jsonpath_query)?;
    let result = engine.run::<_, IndexResult>(&input)?;
    assert_eq!(result.get(), vec![53usize,], "result != expected");
    Ok(())
}
#[test]
fn with_query_look_for_b_on_at_least_one_level_of_nesting_with_owned_bytes_and_count_result_using_main_engine(
) -> Result<(), Box<dyn Error>> {
    println ! ("on document atomic_descendant.toml running the query $..*..b (look for 'b' on at least one level of nesting) with Input impl OwnedBytes and result mode CountResult");
    let jsonpath_query = JsonPathQuery::parse("$..*..b")?;
    let raw_json = fs::read_to_string("./tests/documents/json/atomic_descendant.json")?;
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
    let raw_json = fs::read_to_string("./tests/documents/json/atomic_descendant.json")?;
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
    let raw_json = fs::read_to_string("./tests/documents/json/atomic_descendant.json")?;
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
    let raw_json = fs::read_to_string("./tests/documents/json/atomic_descendant.json")?;
    let input = OwnedBytes::new(&raw_json.as_bytes())?;
    let engine = RecursiveEngine::compile_query(&jsonpath_query)?;
    let result = engine.run::<_, IndexResult>(&input)?;
    assert_eq!(result.get(), vec![53usize,], "result != expected");
    Ok(())
}
#[test]
fn with_query_look_for_descendants_of_an_atomic_value_with_buffered_input_and_count_result_using_main_engine(
) -> Result<(), Box<dyn Error>> {
    println ! ("on document atomic_descendant.toml running the query $..a..b (look for descendants of an atomic value) with Input impl BufferedInput and result mode CountResult");
    let jsonpath_query = JsonPathQuery::parse("$..a..b")?;
    let json_file = fs::File::open("./tests/documents/json/atomic_descendant.json")?;
    let input = BufferedInput::new(json_file);
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
    let json_file = fs::File::open("./tests/documents/json/atomic_descendant.json")?;
    let input = BufferedInput::new(json_file);
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
    let json_file = fs::File::open("./tests/documents/json/atomic_descendant.json")?;
    let input = BufferedInput::new(json_file);
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
    let json_file = fs::File::open("./tests/documents/json/atomic_descendant.json")?;
    let input = BufferedInput::new(json_file);
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
    let json_file = fs::File::open("./tests/documents/json/atomic_descendant.json")?;
    let input = unsafe { MmapInput::map_file(&json_file)? };
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
    let json_file = fs::File::open("./tests/documents/json/atomic_descendant.json")?;
    let input = unsafe { MmapInput::map_file(&json_file)? };
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
    let json_file = fs::File::open("./tests/documents/json/atomic_descendant.json")?;
    let input = unsafe { MmapInput::map_file(&json_file)? };
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
    let json_file = fs::File::open("./tests/documents/json/atomic_descendant.json")?;
    let input = unsafe { MmapInput::map_file(&json_file)? };
    let engine = RecursiveEngine::compile_query(&jsonpath_query)?;
    let result = engine.run::<_, IndexResult>(&input)?;
    assert_eq!(result.get(), vec![], "result != expected");
    Ok(())
}
#[test]
fn with_query_look_for_descendants_of_an_atomic_value_with_owned_bytes_and_count_result_using_main_engine(
) -> Result<(), Box<dyn Error>> {
    println ! ("on document atomic_descendant.toml running the query $..a..b (look for descendants of an atomic value) with Input impl OwnedBytes and result mode CountResult");
    let jsonpath_query = JsonPathQuery::parse("$..a..b")?;
    let raw_json = fs::read_to_string("./tests/documents/json/atomic_descendant.json")?;
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
    let raw_json = fs::read_to_string("./tests/documents/json/atomic_descendant.json")?;
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
    let raw_json = fs::read_to_string("./tests/documents/json/atomic_descendant.json")?;
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
    let raw_json = fs::read_to_string("./tests/documents/json/atomic_descendant.json")?;
    let input = OwnedBytes::new(&raw_json.as_bytes())?;
    let engine = RecursiveEngine::compile_query(&jsonpath_query)?;
    let result = engine.run::<_, IndexResult>(&input)?;
    assert_eq!(result.get(), vec![], "result != expected");
    Ok(())
}
#[test]
fn with_query_select_a_number_that_is_a_child_with_buffered_input_and_count_result_using_main_engine(
) -> Result<(), Box<dyn Error>> {
    println ! ("on document atomic_descendant.toml running the query $..a (select a number that is a child) with Input impl BufferedInput and result mode CountResult");
    let jsonpath_query = JsonPathQuery::parse("$..a")?;
    let json_file = fs::File::open("./tests/documents/json/atomic_descendant.json")?;
    let input = BufferedInput::new(json_file);
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
    let json_file = fs::File::open("./tests/documents/json/atomic_descendant.json")?;
    let input = BufferedInput::new(json_file);
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
    let json_file = fs::File::open("./tests/documents/json/atomic_descendant.json")?;
    let input = BufferedInput::new(json_file);
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
    let json_file = fs::File::open("./tests/documents/json/atomic_descendant.json")?;
    let input = BufferedInput::new(json_file);
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
    let json_file = fs::File::open("./tests/documents/json/atomic_descendant.json")?;
    let input = unsafe { MmapInput::map_file(&json_file)? };
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
    let json_file = fs::File::open("./tests/documents/json/atomic_descendant.json")?;
    let input = unsafe { MmapInput::map_file(&json_file)? };
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
    let json_file = fs::File::open("./tests/documents/json/atomic_descendant.json")?;
    let input = unsafe { MmapInput::map_file(&json_file)? };
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
    let json_file = fs::File::open("./tests/documents/json/atomic_descendant.json")?;
    let input = unsafe { MmapInput::map_file(&json_file)? };
    let engine = RecursiveEngine::compile_query(&jsonpath_query)?;
    let result = engine.run::<_, IndexResult>(&input)?;
    assert_eq!(result.get(), vec![11usize,], "result != expected");
    Ok(())
}
#[test]
fn with_query_select_a_number_that_is_a_child_with_owned_bytes_and_count_result_using_main_engine(
) -> Result<(), Box<dyn Error>> {
    println ! ("on document atomic_descendant.toml running the query $..a (select a number that is a child) with Input impl OwnedBytes and result mode CountResult");
    let jsonpath_query = JsonPathQuery::parse("$..a")?;
    let raw_json = fs::read_to_string("./tests/documents/json/atomic_descendant.json")?;
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
    let raw_json = fs::read_to_string("./tests/documents/json/atomic_descendant.json")?;
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
    let raw_json = fs::read_to_string("./tests/documents/json/atomic_descendant.json")?;
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
    let raw_json = fs::read_to_string("./tests/documents/json/atomic_descendant.json")?;
    let input = OwnedBytes::new(&raw_json.as_bytes())?;
    let engine = RecursiveEngine::compile_query(&jsonpath_query)?;
    let result = engine.run::<_, IndexResult>(&input)?;
    assert_eq!(result.get(), vec![11usize,], "result != expected");
    Ok(())
}
#[test]
fn with_query_select_all_decsendants_with_buffered_input_and_count_result_using_main_engine(
) -> Result<(), Box<dyn Error>> {
    println ! ("on document atomic_descendant.toml running the query $..* (select all decsendants) with Input impl BufferedInput and result mode CountResult");
    let jsonpath_query = JsonPathQuery::parse("$..*")?;
    let json_file = fs::File::open("./tests/documents/json/atomic_descendant.json")?;
    let input = BufferedInput::new(json_file);
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
    let json_file = fs::File::open("./tests/documents/json/atomic_descendant.json")?;
    let input = BufferedInput::new(json_file);
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
    let json_file = fs::File::open("./tests/documents/json/atomic_descendant.json")?;
    let input = BufferedInput::new(json_file);
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
    let json_file = fs::File::open("./tests/documents/json/atomic_descendant.json")?;
    let input = BufferedInput::new(json_file);
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
    let json_file = fs::File::open("./tests/documents/json/atomic_descendant.json")?;
    let input = unsafe { MmapInput::map_file(&json_file)? };
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
    let json_file = fs::File::open("./tests/documents/json/atomic_descendant.json")?;
    let input = unsafe { MmapInput::map_file(&json_file)? };
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
    let json_file = fs::File::open("./tests/documents/json/atomic_descendant.json")?;
    let input = unsafe { MmapInput::map_file(&json_file)? };
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
    let json_file = fs::File::open("./tests/documents/json/atomic_descendant.json")?;
    let input = unsafe { MmapInput::map_file(&json_file)? };
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
fn with_query_select_all_decsendants_with_owned_bytes_and_count_result_using_main_engine() -> Result<(), Box<dyn Error>>
{
    println ! ("on document atomic_descendant.toml running the query $..* (select all decsendants) with Input impl OwnedBytes and result mode CountResult");
    let jsonpath_query = JsonPathQuery::parse("$..*")?;
    let raw_json = fs::read_to_string("./tests/documents/json/atomic_descendant.json")?;
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
    let raw_json = fs::read_to_string("./tests/documents/json/atomic_descendant.json")?;
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
    let raw_json = fs::read_to_string("./tests/documents/json/atomic_descendant.json")?;
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
    let raw_json = fs::read_to_string("./tests/documents/json/atomic_descendant.json")?;
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
fn with_query_select_first_item_from_list_descendants_with_buffered_input_and_count_result_using_main_engine(
) -> Result<(), Box<dyn Error>> {
    println ! ("on document atomic_descendant.toml running the query $..[0] (select first item from list descendants) with Input impl BufferedInput and result mode CountResult");
    let jsonpath_query = JsonPathQuery::parse("$..[0]")?;
    let json_file = fs::File::open("./tests/documents/json/atomic_descendant.json")?;
    let input = BufferedInput::new(json_file);
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
    let json_file = fs::File::open("./tests/documents/json/atomic_descendant.json")?;
    let input = BufferedInput::new(json_file);
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
    let json_file = fs::File::open("./tests/documents/json/atomic_descendant.json")?;
    let input = BufferedInput::new(json_file);
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
    let json_file = fs::File::open("./tests/documents/json/atomic_descendant.json")?;
    let input = BufferedInput::new(json_file);
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
    let json_file = fs::File::open("./tests/documents/json/atomic_descendant.json")?;
    let input = unsafe { MmapInput::map_file(&json_file)? };
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
    let json_file = fs::File::open("./tests/documents/json/atomic_descendant.json")?;
    let input = unsafe { MmapInput::map_file(&json_file)? };
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
    let json_file = fs::File::open("./tests/documents/json/atomic_descendant.json")?;
    let input = unsafe { MmapInput::map_file(&json_file)? };
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
    let json_file = fs::File::open("./tests/documents/json/atomic_descendant.json")?;
    let input = unsafe { MmapInput::map_file(&json_file)? };
    let engine = RecursiveEngine::compile_query(&jsonpath_query)?;
    let result = engine.run::<_, IndexResult>(&input)?;
    assert_eq!(result.get(), vec![34usize,], "result != expected");
    Ok(())
}
#[test]
fn with_query_select_first_item_from_list_descendants_with_owned_bytes_and_count_result_using_main_engine(
) -> Result<(), Box<dyn Error>> {
    println ! ("on document atomic_descendant.toml running the query $..[0] (select first item from list descendants) with Input impl OwnedBytes and result mode CountResult");
    let jsonpath_query = JsonPathQuery::parse("$..[0]")?;
    let raw_json = fs::read_to_string("./tests/documents/json/atomic_descendant.json")?;
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
    let raw_json = fs::read_to_string("./tests/documents/json/atomic_descendant.json")?;
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
    let raw_json = fs::read_to_string("./tests/documents/json/atomic_descendant.json")?;
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
    let raw_json = fs::read_to_string("./tests/documents/json/atomic_descendant.json")?;
    let input = OwnedBytes::new(&raw_json.as_bytes())?;
    let engine = RecursiveEngine::compile_query(&jsonpath_query)?;
    let result = engine.run::<_, IndexResult>(&input)?;
    assert_eq!(result.get(), vec![34usize,], "result != expected");
    Ok(())
}
