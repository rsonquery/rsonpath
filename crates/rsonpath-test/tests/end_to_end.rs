use pretty_assertions::assert_eq;
use rsonpath::engine::{main::MainEngine, recursive::RecursiveEngine, Compiler, Engine};
use rsonpath::input::*;
use rsonpath::query::JsonPathQuery;
use rsonpath::result::*;
use std::error::Error;
use std::fs;
#[test]
fn a_lot_of_atomic_leaves_with_query_select_all_leaves_directly_nested_at_an_a_with_buffered_input_and_count_result_using_main_engine(
) -> Result<(), Box<dyn Error>> {
    println ! ("on document dense_leaves.toml running the query $..a.* (select all leaves directly nested at an 'a') with Input impl BufferedInput and result mode CountResult");
    let jsonpath_query = JsonPathQuery::parse("$..a.*")?;
    let json_file = fs::File::open("/home/mat/rsonpath/crates/rsonpath-test/tests/documents/toml/dense_leaves.json")?;
    let input = BufferedInput::new(json_file);
    let engine = MainEngine::compile_query(&jsonpath_query)?;
    let result = engine.run::<_, CountResult>(&input)?;
    assert_eq!(result.get(), 7u64, "result != expected");
    Ok(())
}
#[test]
fn a_lot_of_atomic_leaves_with_query_select_all_leaves_directly_nested_at_an_a_with_buffered_input_and_count_result_using_recursive_engine(
) -> Result<(), Box<dyn Error>> {
    println ! ("on document dense_leaves.toml running the query $..a.* (select all leaves directly nested at an 'a') with Input impl BufferedInput and result mode CountResult");
    let jsonpath_query = JsonPathQuery::parse("$..a.*")?;
    let json_file = fs::File::open("/home/mat/rsonpath/crates/rsonpath-test/tests/documents/toml/dense_leaves.json")?;
    let input = BufferedInput::new(json_file);
    let engine = RecursiveEngine::compile_query(&jsonpath_query)?;
    let result = engine.run::<_, CountResult>(&input)?;
    assert_eq!(result.get(), 7u64, "result != expected");
    Ok(())
}
#[test]
fn a_lot_of_atomic_leaves_with_query_select_all_leaves_directly_nested_at_an_a_with_buffered_input_and_index_result_using_main_engine(
) -> Result<(), Box<dyn Error>> {
    println ! ("on document dense_leaves.toml running the query $..a.* (select all leaves directly nested at an 'a') with Input impl BufferedInput and result mode IndexResult");
    let jsonpath_query = JsonPathQuery::parse("$..a.*")?;
    let json_file = fs::File::open("/home/mat/rsonpath/crates/rsonpath-test/tests/documents/toml/dense_leaves.json")?;
    let input = BufferedInput::new(json_file);
    let engine = MainEngine::compile_query(&jsonpath_query)?;
    let result = engine.run::<_, IndexResult>(&input)?;
    assert_eq!(
        result.get(),
        vec![68usize, 93usize, 118usize, 143usize, 213usize, 240usize, 269usize,],
        "result != expected"
    );
    Ok(())
}
#[test]
fn a_lot_of_atomic_leaves_with_query_select_all_leaves_directly_nested_at_an_a_with_buffered_input_and_index_result_using_recursive_engine(
) -> Result<(), Box<dyn Error>> {
    println ! ("on document dense_leaves.toml running the query $..a.* (select all leaves directly nested at an 'a') with Input impl BufferedInput and result mode IndexResult");
    let jsonpath_query = JsonPathQuery::parse("$..a.*")?;
    let json_file = fs::File::open("/home/mat/rsonpath/crates/rsonpath-test/tests/documents/toml/dense_leaves.json")?;
    let input = BufferedInput::new(json_file);
    let engine = RecursiveEngine::compile_query(&jsonpath_query)?;
    let result = engine.run::<_, IndexResult>(&input)?;
    assert_eq!(
        result.get(),
        vec![68usize, 93usize, 118usize, 143usize, 213usize, 240usize, 269usize,],
        "result != expected"
    );
    Ok(())
}
#[test]
fn a_lot_of_atomic_leaves_with_query_select_all_leaves_directly_nested_at_an_a_with_mmap_input_and_count_result_using_main_engine(
) -> Result<(), Box<dyn Error>> {
    println ! ("on document dense_leaves.toml running the query $..a.* (select all leaves directly nested at an 'a') with Input impl MmapInput and result mode CountResult");
    let jsonpath_query = JsonPathQuery::parse("$..a.*")?;
    let json_file = fs::File::open("/home/mat/rsonpath/crates/rsonpath-test/tests/documents/toml/dense_leaves.json")?;
    let input = unsafe { MmapInput::map_file(&json_file)? };
    let engine = MainEngine::compile_query(&jsonpath_query)?;
    let result = engine.run::<_, CountResult>(&input)?;
    assert_eq!(result.get(), 7u64, "result != expected");
    Ok(())
}
#[test]
fn a_lot_of_atomic_leaves_with_query_select_all_leaves_directly_nested_at_an_a_with_mmap_input_and_count_result_using_recursive_engine(
) -> Result<(), Box<dyn Error>> {
    println ! ("on document dense_leaves.toml running the query $..a.* (select all leaves directly nested at an 'a') with Input impl MmapInput and result mode CountResult");
    let jsonpath_query = JsonPathQuery::parse("$..a.*")?;
    let json_file = fs::File::open("/home/mat/rsonpath/crates/rsonpath-test/tests/documents/toml/dense_leaves.json")?;
    let input = unsafe { MmapInput::map_file(&json_file)? };
    let engine = RecursiveEngine::compile_query(&jsonpath_query)?;
    let result = engine.run::<_, CountResult>(&input)?;
    assert_eq!(result.get(), 7u64, "result != expected");
    Ok(())
}
#[test]
fn a_lot_of_atomic_leaves_with_query_select_all_leaves_directly_nested_at_an_a_with_mmap_input_and_index_result_using_main_engine(
) -> Result<(), Box<dyn Error>> {
    println ! ("on document dense_leaves.toml running the query $..a.* (select all leaves directly nested at an 'a') with Input impl MmapInput and result mode IndexResult");
    let jsonpath_query = JsonPathQuery::parse("$..a.*")?;
    let json_file = fs::File::open("/home/mat/rsonpath/crates/rsonpath-test/tests/documents/toml/dense_leaves.json")?;
    let input = unsafe { MmapInput::map_file(&json_file)? };
    let engine = MainEngine::compile_query(&jsonpath_query)?;
    let result = engine.run::<_, IndexResult>(&input)?;
    assert_eq!(
        result.get(),
        vec![68usize, 93usize, 118usize, 143usize, 213usize, 240usize, 269usize,],
        "result != expected"
    );
    Ok(())
}
#[test]
fn a_lot_of_atomic_leaves_with_query_select_all_leaves_directly_nested_at_an_a_with_mmap_input_and_index_result_using_recursive_engine(
) -> Result<(), Box<dyn Error>> {
    println ! ("on document dense_leaves.toml running the query $..a.* (select all leaves directly nested at an 'a') with Input impl MmapInput and result mode IndexResult");
    let jsonpath_query = JsonPathQuery::parse("$..a.*")?;
    let json_file = fs::File::open("/home/mat/rsonpath/crates/rsonpath-test/tests/documents/toml/dense_leaves.json")?;
    let input = unsafe { MmapInput::map_file(&json_file)? };
    let engine = RecursiveEngine::compile_query(&jsonpath_query)?;
    let result = engine.run::<_, IndexResult>(&input)?;
    assert_eq!(
        result.get(),
        vec![68usize, 93usize, 118usize, 143usize, 213usize, 240usize, 269usize,],
        "result != expected"
    );
    Ok(())
}
#[test]
fn a_lot_of_atomic_leaves_with_query_select_all_leaves_directly_nested_at_an_a_with_owned_bytes_and_count_result_using_main_engine(
) -> Result<(), Box<dyn Error>> {
    println ! ("on document dense_leaves.toml running the query $..a.* (select all leaves directly nested at an 'a') with Input impl OwnedBytes and result mode CountResult");
    let jsonpath_query = JsonPathQuery::parse("$..a.*")?;
    let raw_json =
        fs::read_to_string("/home/mat/rsonpath/crates/rsonpath-test/tests/documents/toml/dense_leaves.json")?;
    let input = OwnedBytes::new(&raw_json.as_bytes())?;
    let engine = MainEngine::compile_query(&jsonpath_query)?;
    let result = engine.run::<_, CountResult>(&input)?;
    assert_eq!(result.get(), 7u64, "result != expected");
    Ok(())
}
#[test]
fn a_lot_of_atomic_leaves_with_query_select_all_leaves_directly_nested_at_an_a_with_owned_bytes_and_count_result_using_recursive_engine(
) -> Result<(), Box<dyn Error>> {
    println ! ("on document dense_leaves.toml running the query $..a.* (select all leaves directly nested at an 'a') with Input impl OwnedBytes and result mode CountResult");
    let jsonpath_query = JsonPathQuery::parse("$..a.*")?;
    let raw_json =
        fs::read_to_string("/home/mat/rsonpath/crates/rsonpath-test/tests/documents/toml/dense_leaves.json")?;
    let input = OwnedBytes::new(&raw_json.as_bytes())?;
    let engine = RecursiveEngine::compile_query(&jsonpath_query)?;
    let result = engine.run::<_, CountResult>(&input)?;
    assert_eq!(result.get(), 7u64, "result != expected");
    Ok(())
}
#[test]
fn a_lot_of_atomic_leaves_with_query_select_all_leaves_directly_nested_at_an_a_with_owned_bytes_and_index_result_using_main_engine(
) -> Result<(), Box<dyn Error>> {
    println ! ("on document dense_leaves.toml running the query $..a.* (select all leaves directly nested at an 'a') with Input impl OwnedBytes and result mode IndexResult");
    let jsonpath_query = JsonPathQuery::parse("$..a.*")?;
    let raw_json =
        fs::read_to_string("/home/mat/rsonpath/crates/rsonpath-test/tests/documents/toml/dense_leaves.json")?;
    let input = OwnedBytes::new(&raw_json.as_bytes())?;
    let engine = MainEngine::compile_query(&jsonpath_query)?;
    let result = engine.run::<_, IndexResult>(&input)?;
    assert_eq!(
        result.get(),
        vec![68usize, 93usize, 118usize, 143usize, 213usize, 240usize, 269usize,],
        "result != expected"
    );
    Ok(())
}
#[test]
fn a_lot_of_atomic_leaves_with_query_select_all_leaves_directly_nested_at_an_a_with_owned_bytes_and_index_result_using_recursive_engine(
) -> Result<(), Box<dyn Error>> {
    println ! ("on document dense_leaves.toml running the query $..a.* (select all leaves directly nested at an 'a') with Input impl OwnedBytes and result mode IndexResult");
    let jsonpath_query = JsonPathQuery::parse("$..a.*")?;
    let raw_json =
        fs::read_to_string("/home/mat/rsonpath/crates/rsonpath-test/tests/documents/toml/dense_leaves.json")?;
    let input = OwnedBytes::new(&raw_json.as_bytes())?;
    let engine = RecursiveEngine::compile_query(&jsonpath_query)?;
    let result = engine.run::<_, IndexResult>(&input)?;
    assert_eq!(
        result.get(),
        vec![68usize, 93usize, 118usize, 143usize, 213usize, 240usize, 269usize,],
        "result != expected"
    );
    Ok(())
}
#[test]
fn artifical_json_with_escapes_and_structural_characters_in_member_names_and_values_with_query_match_label_with_four_backslashes_at_the_end_it_does_not_exist_by_memchr_with_buffered_input_and_count_result_using_main_engine(
) -> Result<(), Box<dyn Error>> {
    println ! ("on document escapes.toml running the query $..['label\\\\\\\\'] (match label with four backslashes at the end (it does not exist) by memchr) with Input impl BufferedInput and result mode CountResult");
    let jsonpath_query = JsonPathQuery::parse("$..['label\\\\\\\\']")?;
    let json_file = fs::File::open("/home/mat/rsonpath/crates/rsonpath-test/tests/documents/toml/escapes.json")?;
    let input = BufferedInput::new(json_file);
    let engine = MainEngine::compile_query(&jsonpath_query)?;
    let result = engine.run::<_, CountResult>(&input)?;
    assert_eq!(result.get(), 0u64, "result != expected");
    Ok(())
}
#[test]
fn artifical_json_with_escapes_and_structural_characters_in_member_names_and_values_with_query_match_label_with_four_backslashes_at_the_end_it_does_not_exist_by_memchr_with_buffered_input_and_count_result_using_recursive_engine(
) -> Result<(), Box<dyn Error>> {
    println ! ("on document escapes.toml running the query $..['label\\\\\\\\'] (match label with four backslashes at the end (it does not exist) by memchr) with Input impl BufferedInput and result mode CountResult");
    let jsonpath_query = JsonPathQuery::parse("$..['label\\\\\\\\']")?;
    let json_file = fs::File::open("/home/mat/rsonpath/crates/rsonpath-test/tests/documents/toml/escapes.json")?;
    let input = BufferedInput::new(json_file);
    let engine = RecursiveEngine::compile_query(&jsonpath_query)?;
    let result = engine.run::<_, CountResult>(&input)?;
    assert_eq!(result.get(), 0u64, "result != expected");
    Ok(())
}
#[test]
fn artifical_json_with_escapes_and_structural_characters_in_member_names_and_values_with_query_match_label_with_four_backslashes_at_the_end_it_does_not_exist_by_memchr_with_buffered_input_and_index_result_using_main_engine(
) -> Result<(), Box<dyn Error>> {
    println ! ("on document escapes.toml running the query $..['label\\\\\\\\'] (match label with four backslashes at the end (it does not exist) by memchr) with Input impl BufferedInput and result mode IndexResult");
    let jsonpath_query = JsonPathQuery::parse("$..['label\\\\\\\\']")?;
    let json_file = fs::File::open("/home/mat/rsonpath/crates/rsonpath-test/tests/documents/toml/escapes.json")?;
    let input = BufferedInput::new(json_file);
    let engine = MainEngine::compile_query(&jsonpath_query)?;
    let result = engine.run::<_, IndexResult>(&input)?;
    assert_eq!(result.get(), vec![], "result != expected");
    Ok(())
}
#[test]
fn artifical_json_with_escapes_and_structural_characters_in_member_names_and_values_with_query_match_label_with_four_backslashes_at_the_end_it_does_not_exist_by_memchr_with_buffered_input_and_index_result_using_recursive_engine(
) -> Result<(), Box<dyn Error>> {
    println ! ("on document escapes.toml running the query $..['label\\\\\\\\'] (match label with four backslashes at the end (it does not exist) by memchr) with Input impl BufferedInput and result mode IndexResult");
    let jsonpath_query = JsonPathQuery::parse("$..['label\\\\\\\\']")?;
    let json_file = fs::File::open("/home/mat/rsonpath/crates/rsonpath-test/tests/documents/toml/escapes.json")?;
    let input = BufferedInput::new(json_file);
    let engine = RecursiveEngine::compile_query(&jsonpath_query)?;
    let result = engine.run::<_, IndexResult>(&input)?;
    assert_eq!(result.get(), vec![], "result != expected");
    Ok(())
}
#[test]
fn artifical_json_with_escapes_and_structural_characters_in_member_names_and_values_with_query_match_label_with_four_backslashes_at_the_end_it_does_not_exist_by_memchr_with_mmap_input_and_count_result_using_main_engine(
) -> Result<(), Box<dyn Error>> {
    println ! ("on document escapes.toml running the query $..['label\\\\\\\\'] (match label with four backslashes at the end (it does not exist) by memchr) with Input impl MmapInput and result mode CountResult");
    let jsonpath_query = JsonPathQuery::parse("$..['label\\\\\\\\']")?;
    let json_file = fs::File::open("/home/mat/rsonpath/crates/rsonpath-test/tests/documents/toml/escapes.json")?;
    let input = unsafe { MmapInput::map_file(&json_file)? };
    let engine = MainEngine::compile_query(&jsonpath_query)?;
    let result = engine.run::<_, CountResult>(&input)?;
    assert_eq!(result.get(), 0u64, "result != expected");
    Ok(())
}
#[test]
fn artifical_json_with_escapes_and_structural_characters_in_member_names_and_values_with_query_match_label_with_four_backslashes_at_the_end_it_does_not_exist_by_memchr_with_mmap_input_and_count_result_using_recursive_engine(
) -> Result<(), Box<dyn Error>> {
    println ! ("on document escapes.toml running the query $..['label\\\\\\\\'] (match label with four backslashes at the end (it does not exist) by memchr) with Input impl MmapInput and result mode CountResult");
    let jsonpath_query = JsonPathQuery::parse("$..['label\\\\\\\\']")?;
    let json_file = fs::File::open("/home/mat/rsonpath/crates/rsonpath-test/tests/documents/toml/escapes.json")?;
    let input = unsafe { MmapInput::map_file(&json_file)? };
    let engine = RecursiveEngine::compile_query(&jsonpath_query)?;
    let result = engine.run::<_, CountResult>(&input)?;
    assert_eq!(result.get(), 0u64, "result != expected");
    Ok(())
}
#[test]
fn artifical_json_with_escapes_and_structural_characters_in_member_names_and_values_with_query_match_label_with_four_backslashes_at_the_end_it_does_not_exist_by_memchr_with_mmap_input_and_index_result_using_main_engine(
) -> Result<(), Box<dyn Error>> {
    println ! ("on document escapes.toml running the query $..['label\\\\\\\\'] (match label with four backslashes at the end (it does not exist) by memchr) with Input impl MmapInput and result mode IndexResult");
    let jsonpath_query = JsonPathQuery::parse("$..['label\\\\\\\\']")?;
    let json_file = fs::File::open("/home/mat/rsonpath/crates/rsonpath-test/tests/documents/toml/escapes.json")?;
    let input = unsafe { MmapInput::map_file(&json_file)? };
    let engine = MainEngine::compile_query(&jsonpath_query)?;
    let result = engine.run::<_, IndexResult>(&input)?;
    assert_eq!(result.get(), vec![], "result != expected");
    Ok(())
}
#[test]
fn artifical_json_with_escapes_and_structural_characters_in_member_names_and_values_with_query_match_label_with_four_backslashes_at_the_end_it_does_not_exist_by_memchr_with_mmap_input_and_index_result_using_recursive_engine(
) -> Result<(), Box<dyn Error>> {
    println ! ("on document escapes.toml running the query $..['label\\\\\\\\'] (match label with four backslashes at the end (it does not exist) by memchr) with Input impl MmapInput and result mode IndexResult");
    let jsonpath_query = JsonPathQuery::parse("$..['label\\\\\\\\']")?;
    let json_file = fs::File::open("/home/mat/rsonpath/crates/rsonpath-test/tests/documents/toml/escapes.json")?;
    let input = unsafe { MmapInput::map_file(&json_file)? };
    let engine = RecursiveEngine::compile_query(&jsonpath_query)?;
    let result = engine.run::<_, IndexResult>(&input)?;
    assert_eq!(result.get(), vec![], "result != expected");
    Ok(())
}
#[test]
fn artifical_json_with_escapes_and_structural_characters_in_member_names_and_values_with_query_match_label_with_four_backslashes_at_the_end_it_does_not_exist_by_memchr_with_owned_bytes_and_count_result_using_main_engine(
) -> Result<(), Box<dyn Error>> {
    println ! ("on document escapes.toml running the query $..['label\\\\\\\\'] (match label with four backslashes at the end (it does not exist) by memchr) with Input impl OwnedBytes and result mode CountResult");
    let jsonpath_query = JsonPathQuery::parse("$..['label\\\\\\\\']")?;
    let raw_json = fs::read_to_string("/home/mat/rsonpath/crates/rsonpath-test/tests/documents/toml/escapes.json")?;
    let input = OwnedBytes::new(&raw_json.as_bytes())?;
    let engine = MainEngine::compile_query(&jsonpath_query)?;
    let result = engine.run::<_, CountResult>(&input)?;
    assert_eq!(result.get(), 0u64, "result != expected");
    Ok(())
}
#[test]
fn artifical_json_with_escapes_and_structural_characters_in_member_names_and_values_with_query_match_label_with_four_backslashes_at_the_end_it_does_not_exist_by_memchr_with_owned_bytes_and_count_result_using_recursive_engine(
) -> Result<(), Box<dyn Error>> {
    println ! ("on document escapes.toml running the query $..['label\\\\\\\\'] (match label with four backslashes at the end (it does not exist) by memchr) with Input impl OwnedBytes and result mode CountResult");
    let jsonpath_query = JsonPathQuery::parse("$..['label\\\\\\\\']")?;
    let raw_json = fs::read_to_string("/home/mat/rsonpath/crates/rsonpath-test/tests/documents/toml/escapes.json")?;
    let input = OwnedBytes::new(&raw_json.as_bytes())?;
    let engine = RecursiveEngine::compile_query(&jsonpath_query)?;
    let result = engine.run::<_, CountResult>(&input)?;
    assert_eq!(result.get(), 0u64, "result != expected");
    Ok(())
}
#[test]
fn artifical_json_with_escapes_and_structural_characters_in_member_names_and_values_with_query_match_label_with_four_backslashes_at_the_end_it_does_not_exist_by_memchr_with_owned_bytes_and_index_result_using_main_engine(
) -> Result<(), Box<dyn Error>> {
    println ! ("on document escapes.toml running the query $..['label\\\\\\\\'] (match label with four backslashes at the end (it does not exist) by memchr) with Input impl OwnedBytes and result mode IndexResult");
    let jsonpath_query = JsonPathQuery::parse("$..['label\\\\\\\\']")?;
    let raw_json = fs::read_to_string("/home/mat/rsonpath/crates/rsonpath-test/tests/documents/toml/escapes.json")?;
    let input = OwnedBytes::new(&raw_json.as_bytes())?;
    let engine = MainEngine::compile_query(&jsonpath_query)?;
    let result = engine.run::<_, IndexResult>(&input)?;
    assert_eq!(result.get(), vec![], "result != expected");
    Ok(())
}
#[test]
fn artifical_json_with_escapes_and_structural_characters_in_member_names_and_values_with_query_match_label_with_four_backslashes_at_the_end_it_does_not_exist_by_memchr_with_owned_bytes_and_index_result_using_recursive_engine(
) -> Result<(), Box<dyn Error>> {
    println ! ("on document escapes.toml running the query $..['label\\\\\\\\'] (match label with four backslashes at the end (it does not exist) by memchr) with Input impl OwnedBytes and result mode IndexResult");
    let jsonpath_query = JsonPathQuery::parse("$..['label\\\\\\\\']")?;
    let raw_json = fs::read_to_string("/home/mat/rsonpath/crates/rsonpath-test/tests/documents/toml/escapes.json")?;
    let input = OwnedBytes::new(&raw_json.as_bytes())?;
    let engine = RecursiveEngine::compile_query(&jsonpath_query)?;
    let result = engine.run::<_, IndexResult>(&input)?;
    assert_eq!(result.get(), vec![], "result != expected");
    Ok(())
}
#[test]
fn artifical_json_with_escapes_and_structural_characters_in_member_names_and_values_with_query_match_label_with_four_backslashes_at_the_end_it_does_not_exist_with_buffered_input_and_count_result_using_main_engine(
) -> Result<(), Box<dyn Error>> {
    println ! ("on document escapes.toml running the query $..a..b..['label\\\\\\\\'] (match label with four backslashes at the end (it does not exist)) with Input impl BufferedInput and result mode CountResult");
    let jsonpath_query = JsonPathQuery::parse("$..a..b..['label\\\\\\\\']")?;
    let json_file = fs::File::open("/home/mat/rsonpath/crates/rsonpath-test/tests/documents/toml/escapes.json")?;
    let input = BufferedInput::new(json_file);
    let engine = MainEngine::compile_query(&jsonpath_query)?;
    let result = engine.run::<_, CountResult>(&input)?;
    assert_eq!(result.get(), 0u64, "result != expected");
    Ok(())
}
#[test]
fn artifical_json_with_escapes_and_structural_characters_in_member_names_and_values_with_query_match_label_with_four_backslashes_at_the_end_it_does_not_exist_with_buffered_input_and_count_result_using_recursive_engine(
) -> Result<(), Box<dyn Error>> {
    println ! ("on document escapes.toml running the query $..a..b..['label\\\\\\\\'] (match label with four backslashes at the end (it does not exist)) with Input impl BufferedInput and result mode CountResult");
    let jsonpath_query = JsonPathQuery::parse("$..a..b..['label\\\\\\\\']")?;
    let json_file = fs::File::open("/home/mat/rsonpath/crates/rsonpath-test/tests/documents/toml/escapes.json")?;
    let input = BufferedInput::new(json_file);
    let engine = RecursiveEngine::compile_query(&jsonpath_query)?;
    let result = engine.run::<_, CountResult>(&input)?;
    assert_eq!(result.get(), 0u64, "result != expected");
    Ok(())
}
#[test]
fn artifical_json_with_escapes_and_structural_characters_in_member_names_and_values_with_query_match_label_with_four_backslashes_at_the_end_it_does_not_exist_with_buffered_input_and_index_result_using_main_engine(
) -> Result<(), Box<dyn Error>> {
    println ! ("on document escapes.toml running the query $..a..b..['label\\\\\\\\'] (match label with four backslashes at the end (it does not exist)) with Input impl BufferedInput and result mode IndexResult");
    let jsonpath_query = JsonPathQuery::parse("$..a..b..['label\\\\\\\\']")?;
    let json_file = fs::File::open("/home/mat/rsonpath/crates/rsonpath-test/tests/documents/toml/escapes.json")?;
    let input = BufferedInput::new(json_file);
    let engine = MainEngine::compile_query(&jsonpath_query)?;
    let result = engine.run::<_, IndexResult>(&input)?;
    assert_eq!(result.get(), vec![], "result != expected");
    Ok(())
}
#[test]
fn artifical_json_with_escapes_and_structural_characters_in_member_names_and_values_with_query_match_label_with_four_backslashes_at_the_end_it_does_not_exist_with_buffered_input_and_index_result_using_recursive_engine(
) -> Result<(), Box<dyn Error>> {
    println ! ("on document escapes.toml running the query $..a..b..['label\\\\\\\\'] (match label with four backslashes at the end (it does not exist)) with Input impl BufferedInput and result mode IndexResult");
    let jsonpath_query = JsonPathQuery::parse("$..a..b..['label\\\\\\\\']")?;
    let json_file = fs::File::open("/home/mat/rsonpath/crates/rsonpath-test/tests/documents/toml/escapes.json")?;
    let input = BufferedInput::new(json_file);
    let engine = RecursiveEngine::compile_query(&jsonpath_query)?;
    let result = engine.run::<_, IndexResult>(&input)?;
    assert_eq!(result.get(), vec![], "result != expected");
    Ok(())
}
#[test]
fn artifical_json_with_escapes_and_structural_characters_in_member_names_and_values_with_query_match_label_with_four_backslashes_at_the_end_it_does_not_exist_with_mmap_input_and_count_result_using_main_engine(
) -> Result<(), Box<dyn Error>> {
    println ! ("on document escapes.toml running the query $..a..b..['label\\\\\\\\'] (match label with four backslashes at the end (it does not exist)) with Input impl MmapInput and result mode CountResult");
    let jsonpath_query = JsonPathQuery::parse("$..a..b..['label\\\\\\\\']")?;
    let json_file = fs::File::open("/home/mat/rsonpath/crates/rsonpath-test/tests/documents/toml/escapes.json")?;
    let input = unsafe { MmapInput::map_file(&json_file)? };
    let engine = MainEngine::compile_query(&jsonpath_query)?;
    let result = engine.run::<_, CountResult>(&input)?;
    assert_eq!(result.get(), 0u64, "result != expected");
    Ok(())
}
#[test]
fn artifical_json_with_escapes_and_structural_characters_in_member_names_and_values_with_query_match_label_with_four_backslashes_at_the_end_it_does_not_exist_with_mmap_input_and_count_result_using_recursive_engine(
) -> Result<(), Box<dyn Error>> {
    println ! ("on document escapes.toml running the query $..a..b..['label\\\\\\\\'] (match label with four backslashes at the end (it does not exist)) with Input impl MmapInput and result mode CountResult");
    let jsonpath_query = JsonPathQuery::parse("$..a..b..['label\\\\\\\\']")?;
    let json_file = fs::File::open("/home/mat/rsonpath/crates/rsonpath-test/tests/documents/toml/escapes.json")?;
    let input = unsafe { MmapInput::map_file(&json_file)? };
    let engine = RecursiveEngine::compile_query(&jsonpath_query)?;
    let result = engine.run::<_, CountResult>(&input)?;
    assert_eq!(result.get(), 0u64, "result != expected");
    Ok(())
}
#[test]
fn artifical_json_with_escapes_and_structural_characters_in_member_names_and_values_with_query_match_label_with_four_backslashes_at_the_end_it_does_not_exist_with_mmap_input_and_index_result_using_main_engine(
) -> Result<(), Box<dyn Error>> {
    println ! ("on document escapes.toml running the query $..a..b..['label\\\\\\\\'] (match label with four backslashes at the end (it does not exist)) with Input impl MmapInput and result mode IndexResult");
    let jsonpath_query = JsonPathQuery::parse("$..a..b..['label\\\\\\\\']")?;
    let json_file = fs::File::open("/home/mat/rsonpath/crates/rsonpath-test/tests/documents/toml/escapes.json")?;
    let input = unsafe { MmapInput::map_file(&json_file)? };
    let engine = MainEngine::compile_query(&jsonpath_query)?;
    let result = engine.run::<_, IndexResult>(&input)?;
    assert_eq!(result.get(), vec![], "result != expected");
    Ok(())
}
#[test]
fn artifical_json_with_escapes_and_structural_characters_in_member_names_and_values_with_query_match_label_with_four_backslashes_at_the_end_it_does_not_exist_with_mmap_input_and_index_result_using_recursive_engine(
) -> Result<(), Box<dyn Error>> {
    println ! ("on document escapes.toml running the query $..a..b..['label\\\\\\\\'] (match label with four backslashes at the end (it does not exist)) with Input impl MmapInput and result mode IndexResult");
    let jsonpath_query = JsonPathQuery::parse("$..a..b..['label\\\\\\\\']")?;
    let json_file = fs::File::open("/home/mat/rsonpath/crates/rsonpath-test/tests/documents/toml/escapes.json")?;
    let input = unsafe { MmapInput::map_file(&json_file)? };
    let engine = RecursiveEngine::compile_query(&jsonpath_query)?;
    let result = engine.run::<_, IndexResult>(&input)?;
    assert_eq!(result.get(), vec![], "result != expected");
    Ok(())
}
#[test]
fn artifical_json_with_escapes_and_structural_characters_in_member_names_and_values_with_query_match_label_with_four_backslashes_at_the_end_it_does_not_exist_with_owned_bytes_and_count_result_using_main_engine(
) -> Result<(), Box<dyn Error>> {
    println ! ("on document escapes.toml running the query $..a..b..['label\\\\\\\\'] (match label with four backslashes at the end (it does not exist)) with Input impl OwnedBytes and result mode CountResult");
    let jsonpath_query = JsonPathQuery::parse("$..a..b..['label\\\\\\\\']")?;
    let raw_json = fs::read_to_string("/home/mat/rsonpath/crates/rsonpath-test/tests/documents/toml/escapes.json")?;
    let input = OwnedBytes::new(&raw_json.as_bytes())?;
    let engine = MainEngine::compile_query(&jsonpath_query)?;
    let result = engine.run::<_, CountResult>(&input)?;
    assert_eq!(result.get(), 0u64, "result != expected");
    Ok(())
}
#[test]
fn artifical_json_with_escapes_and_structural_characters_in_member_names_and_values_with_query_match_label_with_four_backslashes_at_the_end_it_does_not_exist_with_owned_bytes_and_count_result_using_recursive_engine(
) -> Result<(), Box<dyn Error>> {
    println ! ("on document escapes.toml running the query $..a..b..['label\\\\\\\\'] (match label with four backslashes at the end (it does not exist)) with Input impl OwnedBytes and result mode CountResult");
    let jsonpath_query = JsonPathQuery::parse("$..a..b..['label\\\\\\\\']")?;
    let raw_json = fs::read_to_string("/home/mat/rsonpath/crates/rsonpath-test/tests/documents/toml/escapes.json")?;
    let input = OwnedBytes::new(&raw_json.as_bytes())?;
    let engine = RecursiveEngine::compile_query(&jsonpath_query)?;
    let result = engine.run::<_, CountResult>(&input)?;
    assert_eq!(result.get(), 0u64, "result != expected");
    Ok(())
}
#[test]
fn artifical_json_with_escapes_and_structural_characters_in_member_names_and_values_with_query_match_label_with_four_backslashes_at_the_end_it_does_not_exist_with_owned_bytes_and_index_result_using_main_engine(
) -> Result<(), Box<dyn Error>> {
    println ! ("on document escapes.toml running the query $..a..b..['label\\\\\\\\'] (match label with four backslashes at the end (it does not exist)) with Input impl OwnedBytes and result mode IndexResult");
    let jsonpath_query = JsonPathQuery::parse("$..a..b..['label\\\\\\\\']")?;
    let raw_json = fs::read_to_string("/home/mat/rsonpath/crates/rsonpath-test/tests/documents/toml/escapes.json")?;
    let input = OwnedBytes::new(&raw_json.as_bytes())?;
    let engine = MainEngine::compile_query(&jsonpath_query)?;
    let result = engine.run::<_, IndexResult>(&input)?;
    assert_eq!(result.get(), vec![], "result != expected");
    Ok(())
}
#[test]
fn artifical_json_with_escapes_and_structural_characters_in_member_names_and_values_with_query_match_label_with_four_backslashes_at_the_end_it_does_not_exist_with_owned_bytes_and_index_result_using_recursive_engine(
) -> Result<(), Box<dyn Error>> {
    println ! ("on document escapes.toml running the query $..a..b..['label\\\\\\\\'] (match label with four backslashes at the end (it does not exist)) with Input impl OwnedBytes and result mode IndexResult");
    let jsonpath_query = JsonPathQuery::parse("$..a..b..['label\\\\\\\\']")?;
    let raw_json = fs::read_to_string("/home/mat/rsonpath/crates/rsonpath-test/tests/documents/toml/escapes.json")?;
    let input = OwnedBytes::new(&raw_json.as_bytes())?;
    let engine = RecursiveEngine::compile_query(&jsonpath_query)?;
    let result = engine.run::<_, IndexResult>(&input)?;
    assert_eq!(result.get(), vec![], "result != expected");
    Ok(())
}
#[test]
fn artifical_json_with_escapes_and_structural_characters_in_member_names_and_values_with_query_match_label_with_two_backslashes_at_the_end_by_memchr_with_buffered_input_and_count_result_using_main_engine(
) -> Result<(), Box<dyn Error>> {
    println ! ("on document escapes.toml running the query $..['label\\\\'] (match label with two backslashes at the end by memchr) with Input impl BufferedInput and result mode CountResult");
    let jsonpath_query = JsonPathQuery::parse("$..['label\\\\']")?;
    let json_file = fs::File::open("/home/mat/rsonpath/crates/rsonpath-test/tests/documents/toml/escapes.json")?;
    let input = BufferedInput::new(json_file);
    let engine = MainEngine::compile_query(&jsonpath_query)?;
    let result = engine.run::<_, CountResult>(&input)?;
    assert_eq!(result.get(), 1u64, "result != expected");
    Ok(())
}
#[test]
fn artifical_json_with_escapes_and_structural_characters_in_member_names_and_values_with_query_match_label_with_two_backslashes_at_the_end_by_memchr_with_buffered_input_and_count_result_using_recursive_engine(
) -> Result<(), Box<dyn Error>> {
    println ! ("on document escapes.toml running the query $..['label\\\\'] (match label with two backslashes at the end by memchr) with Input impl BufferedInput and result mode CountResult");
    let jsonpath_query = JsonPathQuery::parse("$..['label\\\\']")?;
    let json_file = fs::File::open("/home/mat/rsonpath/crates/rsonpath-test/tests/documents/toml/escapes.json")?;
    let input = BufferedInput::new(json_file);
    let engine = RecursiveEngine::compile_query(&jsonpath_query)?;
    let result = engine.run::<_, CountResult>(&input)?;
    assert_eq!(result.get(), 1u64, "result != expected");
    Ok(())
}
#[test]
fn artifical_json_with_escapes_and_structural_characters_in_member_names_and_values_with_query_match_label_with_two_backslashes_at_the_end_by_memchr_with_buffered_input_and_index_result_using_main_engine(
) -> Result<(), Box<dyn Error>> {
    println ! ("on document escapes.toml running the query $..['label\\\\'] (match label with two backslashes at the end by memchr) with Input impl BufferedInput and result mode IndexResult");
    let jsonpath_query = JsonPathQuery::parse("$..['label\\\\']")?;
    let json_file = fs::File::open("/home/mat/rsonpath/crates/rsonpath-test/tests/documents/toml/escapes.json")?;
    let input = BufferedInput::new(json_file);
    let engine = MainEngine::compile_query(&jsonpath_query)?;
    let result = engine.run::<_, IndexResult>(&input)?;
    assert_eq!(result.get(), vec![611usize,], "result != expected");
    Ok(())
}
#[test]
fn artifical_json_with_escapes_and_structural_characters_in_member_names_and_values_with_query_match_label_with_two_backslashes_at_the_end_by_memchr_with_buffered_input_and_index_result_using_recursive_engine(
) -> Result<(), Box<dyn Error>> {
    println ! ("on document escapes.toml running the query $..['label\\\\'] (match label with two backslashes at the end by memchr) with Input impl BufferedInput and result mode IndexResult");
    let jsonpath_query = JsonPathQuery::parse("$..['label\\\\']")?;
    let json_file = fs::File::open("/home/mat/rsonpath/crates/rsonpath-test/tests/documents/toml/escapes.json")?;
    let input = BufferedInput::new(json_file);
    let engine = RecursiveEngine::compile_query(&jsonpath_query)?;
    let result = engine.run::<_, IndexResult>(&input)?;
    assert_eq!(result.get(), vec![611usize,], "result != expected");
    Ok(())
}
#[test]
fn artifical_json_with_escapes_and_structural_characters_in_member_names_and_values_with_query_match_label_with_two_backslashes_at_the_end_by_memchr_with_mmap_input_and_count_result_using_main_engine(
) -> Result<(), Box<dyn Error>> {
    println ! ("on document escapes.toml running the query $..['label\\\\'] (match label with two backslashes at the end by memchr) with Input impl MmapInput and result mode CountResult");
    let jsonpath_query = JsonPathQuery::parse("$..['label\\\\']")?;
    let json_file = fs::File::open("/home/mat/rsonpath/crates/rsonpath-test/tests/documents/toml/escapes.json")?;
    let input = unsafe { MmapInput::map_file(&json_file)? };
    let engine = MainEngine::compile_query(&jsonpath_query)?;
    let result = engine.run::<_, CountResult>(&input)?;
    assert_eq!(result.get(), 1u64, "result != expected");
    Ok(())
}
#[test]
fn artifical_json_with_escapes_and_structural_characters_in_member_names_and_values_with_query_match_label_with_two_backslashes_at_the_end_by_memchr_with_mmap_input_and_count_result_using_recursive_engine(
) -> Result<(), Box<dyn Error>> {
    println ! ("on document escapes.toml running the query $..['label\\\\'] (match label with two backslashes at the end by memchr) with Input impl MmapInput and result mode CountResult");
    let jsonpath_query = JsonPathQuery::parse("$..['label\\\\']")?;
    let json_file = fs::File::open("/home/mat/rsonpath/crates/rsonpath-test/tests/documents/toml/escapes.json")?;
    let input = unsafe { MmapInput::map_file(&json_file)? };
    let engine = RecursiveEngine::compile_query(&jsonpath_query)?;
    let result = engine.run::<_, CountResult>(&input)?;
    assert_eq!(result.get(), 1u64, "result != expected");
    Ok(())
}
#[test]
fn artifical_json_with_escapes_and_structural_characters_in_member_names_and_values_with_query_match_label_with_two_backslashes_at_the_end_by_memchr_with_mmap_input_and_index_result_using_main_engine(
) -> Result<(), Box<dyn Error>> {
    println ! ("on document escapes.toml running the query $..['label\\\\'] (match label with two backslashes at the end by memchr) with Input impl MmapInput and result mode IndexResult");
    let jsonpath_query = JsonPathQuery::parse("$..['label\\\\']")?;
    let json_file = fs::File::open("/home/mat/rsonpath/crates/rsonpath-test/tests/documents/toml/escapes.json")?;
    let input = unsafe { MmapInput::map_file(&json_file)? };
    let engine = MainEngine::compile_query(&jsonpath_query)?;
    let result = engine.run::<_, IndexResult>(&input)?;
    assert_eq!(result.get(), vec![611usize,], "result != expected");
    Ok(())
}
#[test]
fn artifical_json_with_escapes_and_structural_characters_in_member_names_and_values_with_query_match_label_with_two_backslashes_at_the_end_by_memchr_with_mmap_input_and_index_result_using_recursive_engine(
) -> Result<(), Box<dyn Error>> {
    println ! ("on document escapes.toml running the query $..['label\\\\'] (match label with two backslashes at the end by memchr) with Input impl MmapInput and result mode IndexResult");
    let jsonpath_query = JsonPathQuery::parse("$..['label\\\\']")?;
    let json_file = fs::File::open("/home/mat/rsonpath/crates/rsonpath-test/tests/documents/toml/escapes.json")?;
    let input = unsafe { MmapInput::map_file(&json_file)? };
    let engine = RecursiveEngine::compile_query(&jsonpath_query)?;
    let result = engine.run::<_, IndexResult>(&input)?;
    assert_eq!(result.get(), vec![611usize,], "result != expected");
    Ok(())
}
#[test]
fn artifical_json_with_escapes_and_structural_characters_in_member_names_and_values_with_query_match_label_with_two_backslashes_at_the_end_by_memchr_with_owned_bytes_and_count_result_using_main_engine(
) -> Result<(), Box<dyn Error>> {
    println ! ("on document escapes.toml running the query $..['label\\\\'] (match label with two backslashes at the end by memchr) with Input impl OwnedBytes and result mode CountResult");
    let jsonpath_query = JsonPathQuery::parse("$..['label\\\\']")?;
    let raw_json = fs::read_to_string("/home/mat/rsonpath/crates/rsonpath-test/tests/documents/toml/escapes.json")?;
    let input = OwnedBytes::new(&raw_json.as_bytes())?;
    let engine = MainEngine::compile_query(&jsonpath_query)?;
    let result = engine.run::<_, CountResult>(&input)?;
    assert_eq!(result.get(), 1u64, "result != expected");
    Ok(())
}
#[test]
fn artifical_json_with_escapes_and_structural_characters_in_member_names_and_values_with_query_match_label_with_two_backslashes_at_the_end_by_memchr_with_owned_bytes_and_count_result_using_recursive_engine(
) -> Result<(), Box<dyn Error>> {
    println ! ("on document escapes.toml running the query $..['label\\\\'] (match label with two backslashes at the end by memchr) with Input impl OwnedBytes and result mode CountResult");
    let jsonpath_query = JsonPathQuery::parse("$..['label\\\\']")?;
    let raw_json = fs::read_to_string("/home/mat/rsonpath/crates/rsonpath-test/tests/documents/toml/escapes.json")?;
    let input = OwnedBytes::new(&raw_json.as_bytes())?;
    let engine = RecursiveEngine::compile_query(&jsonpath_query)?;
    let result = engine.run::<_, CountResult>(&input)?;
    assert_eq!(result.get(), 1u64, "result != expected");
    Ok(())
}
#[test]
fn artifical_json_with_escapes_and_structural_characters_in_member_names_and_values_with_query_match_label_with_two_backslashes_at_the_end_by_memchr_with_owned_bytes_and_index_result_using_main_engine(
) -> Result<(), Box<dyn Error>> {
    println ! ("on document escapes.toml running the query $..['label\\\\'] (match label with two backslashes at the end by memchr) with Input impl OwnedBytes and result mode IndexResult");
    let jsonpath_query = JsonPathQuery::parse("$..['label\\\\']")?;
    let raw_json = fs::read_to_string("/home/mat/rsonpath/crates/rsonpath-test/tests/documents/toml/escapes.json")?;
    let input = OwnedBytes::new(&raw_json.as_bytes())?;
    let engine = MainEngine::compile_query(&jsonpath_query)?;
    let result = engine.run::<_, IndexResult>(&input)?;
    assert_eq!(result.get(), vec![611usize,], "result != expected");
    Ok(())
}
#[test]
fn artifical_json_with_escapes_and_structural_characters_in_member_names_and_values_with_query_match_label_with_two_backslashes_at_the_end_by_memchr_with_owned_bytes_and_index_result_using_recursive_engine(
) -> Result<(), Box<dyn Error>> {
    println ! ("on document escapes.toml running the query $..['label\\\\'] (match label with two backslashes at the end by memchr) with Input impl OwnedBytes and result mode IndexResult");
    let jsonpath_query = JsonPathQuery::parse("$..['label\\\\']")?;
    let raw_json = fs::read_to_string("/home/mat/rsonpath/crates/rsonpath-test/tests/documents/toml/escapes.json")?;
    let input = OwnedBytes::new(&raw_json.as_bytes())?;
    let engine = RecursiveEngine::compile_query(&jsonpath_query)?;
    let result = engine.run::<_, IndexResult>(&input)?;
    assert_eq!(result.get(), vec![611usize,], "result != expected");
    Ok(())
}
#[test]
fn artifical_json_with_escapes_and_structural_characters_in_member_names_and_values_with_query_match_label_with_two_backslashes_at_the_end_with_buffered_input_and_count_result_using_main_engine(
) -> Result<(), Box<dyn Error>> {
    println ! ("on document escapes.toml running the query $..a..b..['label\\\\'] (match label with two backslashes at the end) with Input impl BufferedInput and result mode CountResult");
    let jsonpath_query = JsonPathQuery::parse("$..a..b..['label\\\\']")?;
    let json_file = fs::File::open("/home/mat/rsonpath/crates/rsonpath-test/tests/documents/toml/escapes.json")?;
    let input = BufferedInput::new(json_file);
    let engine = MainEngine::compile_query(&jsonpath_query)?;
    let result = engine.run::<_, CountResult>(&input)?;
    assert_eq!(result.get(), 1u64, "result != expected");
    Ok(())
}
#[test]
fn artifical_json_with_escapes_and_structural_characters_in_member_names_and_values_with_query_match_label_with_two_backslashes_at_the_end_with_buffered_input_and_count_result_using_recursive_engine(
) -> Result<(), Box<dyn Error>> {
    println ! ("on document escapes.toml running the query $..a..b..['label\\\\'] (match label with two backslashes at the end) with Input impl BufferedInput and result mode CountResult");
    let jsonpath_query = JsonPathQuery::parse("$..a..b..['label\\\\']")?;
    let json_file = fs::File::open("/home/mat/rsonpath/crates/rsonpath-test/tests/documents/toml/escapes.json")?;
    let input = BufferedInput::new(json_file);
    let engine = RecursiveEngine::compile_query(&jsonpath_query)?;
    let result = engine.run::<_, CountResult>(&input)?;
    assert_eq!(result.get(), 1u64, "result != expected");
    Ok(())
}
#[test]
fn artifical_json_with_escapes_and_structural_characters_in_member_names_and_values_with_query_match_label_with_two_backslashes_at_the_end_with_buffered_input_and_index_result_using_main_engine(
) -> Result<(), Box<dyn Error>> {
    println ! ("on document escapes.toml running the query $..a..b..['label\\\\'] (match label with two backslashes at the end) with Input impl BufferedInput and result mode IndexResult");
    let jsonpath_query = JsonPathQuery::parse("$..a..b..['label\\\\']")?;
    let json_file = fs::File::open("/home/mat/rsonpath/crates/rsonpath-test/tests/documents/toml/escapes.json")?;
    let input = BufferedInput::new(json_file);
    let engine = MainEngine::compile_query(&jsonpath_query)?;
    let result = engine.run::<_, IndexResult>(&input)?;
    assert_eq!(result.get(), vec![611usize,], "result != expected");
    Ok(())
}
#[test]
fn artifical_json_with_escapes_and_structural_characters_in_member_names_and_values_with_query_match_label_with_two_backslashes_at_the_end_with_buffered_input_and_index_result_using_recursive_engine(
) -> Result<(), Box<dyn Error>> {
    println ! ("on document escapes.toml running the query $..a..b..['label\\\\'] (match label with two backslashes at the end) with Input impl BufferedInput and result mode IndexResult");
    let jsonpath_query = JsonPathQuery::parse("$..a..b..['label\\\\']")?;
    let json_file = fs::File::open("/home/mat/rsonpath/crates/rsonpath-test/tests/documents/toml/escapes.json")?;
    let input = BufferedInput::new(json_file);
    let engine = RecursiveEngine::compile_query(&jsonpath_query)?;
    let result = engine.run::<_, IndexResult>(&input)?;
    assert_eq!(result.get(), vec![611usize,], "result != expected");
    Ok(())
}
#[test]
fn artifical_json_with_escapes_and_structural_characters_in_member_names_and_values_with_query_match_label_with_two_backslashes_at_the_end_with_mmap_input_and_count_result_using_main_engine(
) -> Result<(), Box<dyn Error>> {
    println ! ("on document escapes.toml running the query $..a..b..['label\\\\'] (match label with two backslashes at the end) with Input impl MmapInput and result mode CountResult");
    let jsonpath_query = JsonPathQuery::parse("$..a..b..['label\\\\']")?;
    let json_file = fs::File::open("/home/mat/rsonpath/crates/rsonpath-test/tests/documents/toml/escapes.json")?;
    let input = unsafe { MmapInput::map_file(&json_file)? };
    let engine = MainEngine::compile_query(&jsonpath_query)?;
    let result = engine.run::<_, CountResult>(&input)?;
    assert_eq!(result.get(), 1u64, "result != expected");
    Ok(())
}
#[test]
fn artifical_json_with_escapes_and_structural_characters_in_member_names_and_values_with_query_match_label_with_two_backslashes_at_the_end_with_mmap_input_and_count_result_using_recursive_engine(
) -> Result<(), Box<dyn Error>> {
    println ! ("on document escapes.toml running the query $..a..b..['label\\\\'] (match label with two backslashes at the end) with Input impl MmapInput and result mode CountResult");
    let jsonpath_query = JsonPathQuery::parse("$..a..b..['label\\\\']")?;
    let json_file = fs::File::open("/home/mat/rsonpath/crates/rsonpath-test/tests/documents/toml/escapes.json")?;
    let input = unsafe { MmapInput::map_file(&json_file)? };
    let engine = RecursiveEngine::compile_query(&jsonpath_query)?;
    let result = engine.run::<_, CountResult>(&input)?;
    assert_eq!(result.get(), 1u64, "result != expected");
    Ok(())
}
#[test]
fn artifical_json_with_escapes_and_structural_characters_in_member_names_and_values_with_query_match_label_with_two_backslashes_at_the_end_with_mmap_input_and_index_result_using_main_engine(
) -> Result<(), Box<dyn Error>> {
    println ! ("on document escapes.toml running the query $..a..b..['label\\\\'] (match label with two backslashes at the end) with Input impl MmapInput and result mode IndexResult");
    let jsonpath_query = JsonPathQuery::parse("$..a..b..['label\\\\']")?;
    let json_file = fs::File::open("/home/mat/rsonpath/crates/rsonpath-test/tests/documents/toml/escapes.json")?;
    let input = unsafe { MmapInput::map_file(&json_file)? };
    let engine = MainEngine::compile_query(&jsonpath_query)?;
    let result = engine.run::<_, IndexResult>(&input)?;
    assert_eq!(result.get(), vec![611usize,], "result != expected");
    Ok(())
}
#[test]
fn artifical_json_with_escapes_and_structural_characters_in_member_names_and_values_with_query_match_label_with_two_backslashes_at_the_end_with_mmap_input_and_index_result_using_recursive_engine(
) -> Result<(), Box<dyn Error>> {
    println ! ("on document escapes.toml running the query $..a..b..['label\\\\'] (match label with two backslashes at the end) with Input impl MmapInput and result mode IndexResult");
    let jsonpath_query = JsonPathQuery::parse("$..a..b..['label\\\\']")?;
    let json_file = fs::File::open("/home/mat/rsonpath/crates/rsonpath-test/tests/documents/toml/escapes.json")?;
    let input = unsafe { MmapInput::map_file(&json_file)? };
    let engine = RecursiveEngine::compile_query(&jsonpath_query)?;
    let result = engine.run::<_, IndexResult>(&input)?;
    assert_eq!(result.get(), vec![611usize,], "result != expected");
    Ok(())
}
#[test]
fn artifical_json_with_escapes_and_structural_characters_in_member_names_and_values_with_query_match_label_with_two_backslashes_at_the_end_with_owned_bytes_and_count_result_using_main_engine(
) -> Result<(), Box<dyn Error>> {
    println ! ("on document escapes.toml running the query $..a..b..['label\\\\'] (match label with two backslashes at the end) with Input impl OwnedBytes and result mode CountResult");
    let jsonpath_query = JsonPathQuery::parse("$..a..b..['label\\\\']")?;
    let raw_json = fs::read_to_string("/home/mat/rsonpath/crates/rsonpath-test/tests/documents/toml/escapes.json")?;
    let input = OwnedBytes::new(&raw_json.as_bytes())?;
    let engine = MainEngine::compile_query(&jsonpath_query)?;
    let result = engine.run::<_, CountResult>(&input)?;
    assert_eq!(result.get(), 1u64, "result != expected");
    Ok(())
}
#[test]
fn artifical_json_with_escapes_and_structural_characters_in_member_names_and_values_with_query_match_label_with_two_backslashes_at_the_end_with_owned_bytes_and_count_result_using_recursive_engine(
) -> Result<(), Box<dyn Error>> {
    println ! ("on document escapes.toml running the query $..a..b..['label\\\\'] (match label with two backslashes at the end) with Input impl OwnedBytes and result mode CountResult");
    let jsonpath_query = JsonPathQuery::parse("$..a..b..['label\\\\']")?;
    let raw_json = fs::read_to_string("/home/mat/rsonpath/crates/rsonpath-test/tests/documents/toml/escapes.json")?;
    let input = OwnedBytes::new(&raw_json.as_bytes())?;
    let engine = RecursiveEngine::compile_query(&jsonpath_query)?;
    let result = engine.run::<_, CountResult>(&input)?;
    assert_eq!(result.get(), 1u64, "result != expected");
    Ok(())
}
#[test]
fn artifical_json_with_escapes_and_structural_characters_in_member_names_and_values_with_query_match_label_with_two_backslashes_at_the_end_with_owned_bytes_and_index_result_using_main_engine(
) -> Result<(), Box<dyn Error>> {
    println ! ("on document escapes.toml running the query $..a..b..['label\\\\'] (match label with two backslashes at the end) with Input impl OwnedBytes and result mode IndexResult");
    let jsonpath_query = JsonPathQuery::parse("$..a..b..['label\\\\']")?;
    let raw_json = fs::read_to_string("/home/mat/rsonpath/crates/rsonpath-test/tests/documents/toml/escapes.json")?;
    let input = OwnedBytes::new(&raw_json.as_bytes())?;
    let engine = MainEngine::compile_query(&jsonpath_query)?;
    let result = engine.run::<_, IndexResult>(&input)?;
    assert_eq!(result.get(), vec![611usize,], "result != expected");
    Ok(())
}
#[test]
fn artifical_json_with_escapes_and_structural_characters_in_member_names_and_values_with_query_match_label_with_two_backslashes_at_the_end_with_owned_bytes_and_index_result_using_recursive_engine(
) -> Result<(), Box<dyn Error>> {
    println ! ("on document escapes.toml running the query $..a..b..['label\\\\'] (match label with two backslashes at the end) with Input impl OwnedBytes and result mode IndexResult");
    let jsonpath_query = JsonPathQuery::parse("$..a..b..['label\\\\']")?;
    let raw_json = fs::read_to_string("/home/mat/rsonpath/crates/rsonpath-test/tests/documents/toml/escapes.json")?;
    let input = OwnedBytes::new(&raw_json.as_bytes())?;
    let engine = RecursiveEngine::compile_query(&jsonpath_query)?;
    let result = engine.run::<_, IndexResult>(&input)?;
    assert_eq!(result.get(), vec![611usize,], "result != expected");
    Ok(())
}
#[test]
fn artificial_json_to_test_unique_member_name_child_paths_with_query_descendant_search_for_a_then_for_path_bc_then_for_d_with_buffered_input_and_count_result_using_main_engine(
) -> Result<(), Box<dyn Error>> {
    println ! ("on document child_simple.toml running the query $..a..b.c..d (descendant search for 'a', then for path 'bc', then for 'd') with Input impl BufferedInput and result mode CountResult");
    let jsonpath_query = JsonPathQuery::parse("$..a..b.c..d")?;
    let json_file = fs::File::open("/home/mat/rsonpath/crates/rsonpath-test/tests/documents/toml/child_simple.json")?;
    let input = BufferedInput::new(json_file);
    let engine = MainEngine::compile_query(&jsonpath_query)?;
    let result = engine.run::<_, CountResult>(&input)?;
    assert_eq!(result.get(), 3u64, "result != expected");
    Ok(())
}
#[test]
fn artificial_json_to_test_unique_member_name_child_paths_with_query_descendant_search_for_a_then_for_path_bc_then_for_d_with_buffered_input_and_count_result_using_recursive_engine(
) -> Result<(), Box<dyn Error>> {
    println ! ("on document child_simple.toml running the query $..a..b.c..d (descendant search for 'a', then for path 'bc', then for 'd') with Input impl BufferedInput and result mode CountResult");
    let jsonpath_query = JsonPathQuery::parse("$..a..b.c..d")?;
    let json_file = fs::File::open("/home/mat/rsonpath/crates/rsonpath-test/tests/documents/toml/child_simple.json")?;
    let input = BufferedInput::new(json_file);
    let engine = RecursiveEngine::compile_query(&jsonpath_query)?;
    let result = engine.run::<_, CountResult>(&input)?;
    assert_eq!(result.get(), 3u64, "result != expected");
    Ok(())
}
#[test]
fn artificial_json_to_test_unique_member_name_child_paths_with_query_descendant_search_for_a_then_for_path_bc_then_for_d_with_buffered_input_and_index_result_using_main_engine(
) -> Result<(), Box<dyn Error>> {
    println ! ("on document child_simple.toml running the query $..a..b.c..d (descendant search for 'a', then for path 'bc', then for 'd') with Input impl BufferedInput and result mode IndexResult");
    let jsonpath_query = JsonPathQuery::parse("$..a..b.c..d")?;
    let json_file = fs::File::open("/home/mat/rsonpath/crates/rsonpath-test/tests/documents/toml/child_simple.json")?;
    let input = BufferedInput::new(json_file);
    let engine = MainEngine::compile_query(&jsonpath_query)?;
    let result = engine.run::<_, IndexResult>(&input)?;
    assert_eq!(
        result.get(),
        vec![986usize, 1299usize, 1547usize,],
        "result != expected"
    );
    Ok(())
}
#[test]
fn artificial_json_to_test_unique_member_name_child_paths_with_query_descendant_search_for_a_then_for_path_bc_then_for_d_with_buffered_input_and_index_result_using_recursive_engine(
) -> Result<(), Box<dyn Error>> {
    println ! ("on document child_simple.toml running the query $..a..b.c..d (descendant search for 'a', then for path 'bc', then for 'd') with Input impl BufferedInput and result mode IndexResult");
    let jsonpath_query = JsonPathQuery::parse("$..a..b.c..d")?;
    let json_file = fs::File::open("/home/mat/rsonpath/crates/rsonpath-test/tests/documents/toml/child_simple.json")?;
    let input = BufferedInput::new(json_file);
    let engine = RecursiveEngine::compile_query(&jsonpath_query)?;
    let result = engine.run::<_, IndexResult>(&input)?;
    assert_eq!(
        result.get(),
        vec![986usize, 1299usize, 1547usize,],
        "result != expected"
    );
    Ok(())
}
#[test]
fn artificial_json_to_test_unique_member_name_child_paths_with_query_descendant_search_for_a_then_for_path_bc_then_for_d_with_mmap_input_and_count_result_using_main_engine(
) -> Result<(), Box<dyn Error>> {
    println ! ("on document child_simple.toml running the query $..a..b.c..d (descendant search for 'a', then for path 'bc', then for 'd') with Input impl MmapInput and result mode CountResult");
    let jsonpath_query = JsonPathQuery::parse("$..a..b.c..d")?;
    let json_file = fs::File::open("/home/mat/rsonpath/crates/rsonpath-test/tests/documents/toml/child_simple.json")?;
    let input = unsafe { MmapInput::map_file(&json_file)? };
    let engine = MainEngine::compile_query(&jsonpath_query)?;
    let result = engine.run::<_, CountResult>(&input)?;
    assert_eq!(result.get(), 3u64, "result != expected");
    Ok(())
}
#[test]
fn artificial_json_to_test_unique_member_name_child_paths_with_query_descendant_search_for_a_then_for_path_bc_then_for_d_with_mmap_input_and_count_result_using_recursive_engine(
) -> Result<(), Box<dyn Error>> {
    println ! ("on document child_simple.toml running the query $..a..b.c..d (descendant search for 'a', then for path 'bc', then for 'd') with Input impl MmapInput and result mode CountResult");
    let jsonpath_query = JsonPathQuery::parse("$..a..b.c..d")?;
    let json_file = fs::File::open("/home/mat/rsonpath/crates/rsonpath-test/tests/documents/toml/child_simple.json")?;
    let input = unsafe { MmapInput::map_file(&json_file)? };
    let engine = RecursiveEngine::compile_query(&jsonpath_query)?;
    let result = engine.run::<_, CountResult>(&input)?;
    assert_eq!(result.get(), 3u64, "result != expected");
    Ok(())
}
#[test]
fn artificial_json_to_test_unique_member_name_child_paths_with_query_descendant_search_for_a_then_for_path_bc_then_for_d_with_mmap_input_and_index_result_using_main_engine(
) -> Result<(), Box<dyn Error>> {
    println ! ("on document child_simple.toml running the query $..a..b.c..d (descendant search for 'a', then for path 'bc', then for 'd') with Input impl MmapInput and result mode IndexResult");
    let jsonpath_query = JsonPathQuery::parse("$..a..b.c..d")?;
    let json_file = fs::File::open("/home/mat/rsonpath/crates/rsonpath-test/tests/documents/toml/child_simple.json")?;
    let input = unsafe { MmapInput::map_file(&json_file)? };
    let engine = MainEngine::compile_query(&jsonpath_query)?;
    let result = engine.run::<_, IndexResult>(&input)?;
    assert_eq!(
        result.get(),
        vec![986usize, 1299usize, 1547usize,],
        "result != expected"
    );
    Ok(())
}
#[test]
fn artificial_json_to_test_unique_member_name_child_paths_with_query_descendant_search_for_a_then_for_path_bc_then_for_d_with_mmap_input_and_index_result_using_recursive_engine(
) -> Result<(), Box<dyn Error>> {
    println ! ("on document child_simple.toml running the query $..a..b.c..d (descendant search for 'a', then for path 'bc', then for 'd') with Input impl MmapInput and result mode IndexResult");
    let jsonpath_query = JsonPathQuery::parse("$..a..b.c..d")?;
    let json_file = fs::File::open("/home/mat/rsonpath/crates/rsonpath-test/tests/documents/toml/child_simple.json")?;
    let input = unsafe { MmapInput::map_file(&json_file)? };
    let engine = RecursiveEngine::compile_query(&jsonpath_query)?;
    let result = engine.run::<_, IndexResult>(&input)?;
    assert_eq!(
        result.get(),
        vec![986usize, 1299usize, 1547usize,],
        "result != expected"
    );
    Ok(())
}
#[test]
fn artificial_json_to_test_unique_member_name_child_paths_with_query_descendant_search_for_a_then_for_path_bc_then_for_d_with_owned_bytes_and_count_result_using_main_engine(
) -> Result<(), Box<dyn Error>> {
    println ! ("on document child_simple.toml running the query $..a..b.c..d (descendant search for 'a', then for path 'bc', then for 'd') with Input impl OwnedBytes and result mode CountResult");
    let jsonpath_query = JsonPathQuery::parse("$..a..b.c..d")?;
    let raw_json =
        fs::read_to_string("/home/mat/rsonpath/crates/rsonpath-test/tests/documents/toml/child_simple.json")?;
    let input = OwnedBytes::new(&raw_json.as_bytes())?;
    let engine = MainEngine::compile_query(&jsonpath_query)?;
    let result = engine.run::<_, CountResult>(&input)?;
    assert_eq!(result.get(), 3u64, "result != expected");
    Ok(())
}
#[test]
fn artificial_json_to_test_unique_member_name_child_paths_with_query_descendant_search_for_a_then_for_path_bc_then_for_d_with_owned_bytes_and_count_result_using_recursive_engine(
) -> Result<(), Box<dyn Error>> {
    println ! ("on document child_simple.toml running the query $..a..b.c..d (descendant search for 'a', then for path 'bc', then for 'd') with Input impl OwnedBytes and result mode CountResult");
    let jsonpath_query = JsonPathQuery::parse("$..a..b.c..d")?;
    let raw_json =
        fs::read_to_string("/home/mat/rsonpath/crates/rsonpath-test/tests/documents/toml/child_simple.json")?;
    let input = OwnedBytes::new(&raw_json.as_bytes())?;
    let engine = RecursiveEngine::compile_query(&jsonpath_query)?;
    let result = engine.run::<_, CountResult>(&input)?;
    assert_eq!(result.get(), 3u64, "result != expected");
    Ok(())
}
#[test]
fn artificial_json_to_test_unique_member_name_child_paths_with_query_descendant_search_for_a_then_for_path_bc_then_for_d_with_owned_bytes_and_index_result_using_main_engine(
) -> Result<(), Box<dyn Error>> {
    println ! ("on document child_simple.toml running the query $..a..b.c..d (descendant search for 'a', then for path 'bc', then for 'd') with Input impl OwnedBytes and result mode IndexResult");
    let jsonpath_query = JsonPathQuery::parse("$..a..b.c..d")?;
    let raw_json =
        fs::read_to_string("/home/mat/rsonpath/crates/rsonpath-test/tests/documents/toml/child_simple.json")?;
    let input = OwnedBytes::new(&raw_json.as_bytes())?;
    let engine = MainEngine::compile_query(&jsonpath_query)?;
    let result = engine.run::<_, IndexResult>(&input)?;
    assert_eq!(
        result.get(),
        vec![986usize, 1299usize, 1547usize,],
        "result != expected"
    );
    Ok(())
}
#[test]
fn artificial_json_to_test_unique_member_name_child_paths_with_query_descendant_search_for_a_then_for_path_bc_then_for_d_with_owned_bytes_and_index_result_using_recursive_engine(
) -> Result<(), Box<dyn Error>> {
    println ! ("on document child_simple.toml running the query $..a..b.c..d (descendant search for 'a', then for path 'bc', then for 'd') with Input impl OwnedBytes and result mode IndexResult");
    let jsonpath_query = JsonPathQuery::parse("$..a..b.c..d")?;
    let raw_json =
        fs::read_to_string("/home/mat/rsonpath/crates/rsonpath-test/tests/documents/toml/child_simple.json")?;
    let input = OwnedBytes::new(&raw_json.as_bytes())?;
    let engine = RecursiveEngine::compile_query(&jsonpath_query)?;
    let result = engine.run::<_, IndexResult>(&input)?;
    assert_eq!(
        result.get(),
        vec![986usize, 1299usize, 1547usize,],
        "result != expected"
    );
    Ok(())
}
#[test]
fn artificial_json_with_multiple_nested_same_member_names_to_stress_test_the_child_segment_with_query_select_the_path_ababc_with_buffered_input_and_count_result_using_main_engine(
) -> Result<(), Box<dyn Error>> {
    println ! ("on document child_hell.toml running the query $..x..a.b.a.b.c (select the path ababc) with Input impl BufferedInput and result mode CountResult");
    let jsonpath_query = JsonPathQuery::parse("$..x..a.b.a.b.c")?;
    let json_file = fs::File::open("/home/mat/rsonpath/crates/rsonpath-test/tests/documents/toml/child_hell.json")?;
    let input = BufferedInput::new(json_file);
    let engine = MainEngine::compile_query(&jsonpath_query)?;
    let result = engine.run::<_, CountResult>(&input)?;
    assert_eq!(result.get(), 6u64, "result != expected");
    Ok(())
}
#[test]
fn artificial_json_with_multiple_nested_same_member_names_to_stress_test_the_child_segment_with_query_select_the_path_ababc_with_buffered_input_and_count_result_using_recursive_engine(
) -> Result<(), Box<dyn Error>> {
    println ! ("on document child_hell.toml running the query $..x..a.b.a.b.c (select the path ababc) with Input impl BufferedInput and result mode CountResult");
    let jsonpath_query = JsonPathQuery::parse("$..x..a.b.a.b.c")?;
    let json_file = fs::File::open("/home/mat/rsonpath/crates/rsonpath-test/tests/documents/toml/child_hell.json")?;
    let input = BufferedInput::new(json_file);
    let engine = RecursiveEngine::compile_query(&jsonpath_query)?;
    let result = engine.run::<_, CountResult>(&input)?;
    assert_eq!(result.get(), 6u64, "result != expected");
    Ok(())
}
#[test]
fn artificial_json_with_multiple_nested_same_member_names_to_stress_test_the_child_segment_with_query_select_the_path_ababc_with_buffered_input_and_index_result_using_main_engine(
) -> Result<(), Box<dyn Error>> {
    println ! ("on document child_hell.toml running the query $..x..a.b.a.b.c (select the path ababc) with Input impl BufferedInput and result mode IndexResult");
    let jsonpath_query = JsonPathQuery::parse("$..x..a.b.a.b.c")?;
    let json_file = fs::File::open("/home/mat/rsonpath/crates/rsonpath-test/tests/documents/toml/child_hell.json")?;
    let input = BufferedInput::new(json_file);
    let engine = MainEngine::compile_query(&jsonpath_query)?;
    let result = engine.run::<_, IndexResult>(&input)?;
    assert_eq!(
        result.get(),
        vec![200usize, 758usize, 1229usize, 1905usize, 2042usize, 2209usize,],
        "result != expected"
    );
    Ok(())
}
#[test]
fn artificial_json_with_multiple_nested_same_member_names_to_stress_test_the_child_segment_with_query_select_the_path_ababc_with_buffered_input_and_index_result_using_recursive_engine(
) -> Result<(), Box<dyn Error>> {
    println ! ("on document child_hell.toml running the query $..x..a.b.a.b.c (select the path ababc) with Input impl BufferedInput and result mode IndexResult");
    let jsonpath_query = JsonPathQuery::parse("$..x..a.b.a.b.c")?;
    let json_file = fs::File::open("/home/mat/rsonpath/crates/rsonpath-test/tests/documents/toml/child_hell.json")?;
    let input = BufferedInput::new(json_file);
    let engine = RecursiveEngine::compile_query(&jsonpath_query)?;
    let result = engine.run::<_, IndexResult>(&input)?;
    assert_eq!(
        result.get(),
        vec![200usize, 758usize, 1229usize, 1905usize, 2042usize, 2209usize,],
        "result != expected"
    );
    Ok(())
}
#[test]
fn artificial_json_with_multiple_nested_same_member_names_to_stress_test_the_child_segment_with_query_select_the_path_ababc_with_mmap_input_and_count_result_using_main_engine(
) -> Result<(), Box<dyn Error>> {
    println ! ("on document child_hell.toml running the query $..x..a.b.a.b.c (select the path ababc) with Input impl MmapInput and result mode CountResult");
    let jsonpath_query = JsonPathQuery::parse("$..x..a.b.a.b.c")?;
    let json_file = fs::File::open("/home/mat/rsonpath/crates/rsonpath-test/tests/documents/toml/child_hell.json")?;
    let input = unsafe { MmapInput::map_file(&json_file)? };
    let engine = MainEngine::compile_query(&jsonpath_query)?;
    let result = engine.run::<_, CountResult>(&input)?;
    assert_eq!(result.get(), 6u64, "result != expected");
    Ok(())
}
#[test]
fn artificial_json_with_multiple_nested_same_member_names_to_stress_test_the_child_segment_with_query_select_the_path_ababc_with_mmap_input_and_count_result_using_recursive_engine(
) -> Result<(), Box<dyn Error>> {
    println ! ("on document child_hell.toml running the query $..x..a.b.a.b.c (select the path ababc) with Input impl MmapInput and result mode CountResult");
    let jsonpath_query = JsonPathQuery::parse("$..x..a.b.a.b.c")?;
    let json_file = fs::File::open("/home/mat/rsonpath/crates/rsonpath-test/tests/documents/toml/child_hell.json")?;
    let input = unsafe { MmapInput::map_file(&json_file)? };
    let engine = RecursiveEngine::compile_query(&jsonpath_query)?;
    let result = engine.run::<_, CountResult>(&input)?;
    assert_eq!(result.get(), 6u64, "result != expected");
    Ok(())
}
#[test]
fn artificial_json_with_multiple_nested_same_member_names_to_stress_test_the_child_segment_with_query_select_the_path_ababc_with_mmap_input_and_index_result_using_main_engine(
) -> Result<(), Box<dyn Error>> {
    println ! ("on document child_hell.toml running the query $..x..a.b.a.b.c (select the path ababc) with Input impl MmapInput and result mode IndexResult");
    let jsonpath_query = JsonPathQuery::parse("$..x..a.b.a.b.c")?;
    let json_file = fs::File::open("/home/mat/rsonpath/crates/rsonpath-test/tests/documents/toml/child_hell.json")?;
    let input = unsafe { MmapInput::map_file(&json_file)? };
    let engine = MainEngine::compile_query(&jsonpath_query)?;
    let result = engine.run::<_, IndexResult>(&input)?;
    assert_eq!(
        result.get(),
        vec![200usize, 758usize, 1229usize, 1905usize, 2042usize, 2209usize,],
        "result != expected"
    );
    Ok(())
}
#[test]
fn artificial_json_with_multiple_nested_same_member_names_to_stress_test_the_child_segment_with_query_select_the_path_ababc_with_mmap_input_and_index_result_using_recursive_engine(
) -> Result<(), Box<dyn Error>> {
    println ! ("on document child_hell.toml running the query $..x..a.b.a.b.c (select the path ababc) with Input impl MmapInput and result mode IndexResult");
    let jsonpath_query = JsonPathQuery::parse("$..x..a.b.a.b.c")?;
    let json_file = fs::File::open("/home/mat/rsonpath/crates/rsonpath-test/tests/documents/toml/child_hell.json")?;
    let input = unsafe { MmapInput::map_file(&json_file)? };
    let engine = RecursiveEngine::compile_query(&jsonpath_query)?;
    let result = engine.run::<_, IndexResult>(&input)?;
    assert_eq!(
        result.get(),
        vec![200usize, 758usize, 1229usize, 1905usize, 2042usize, 2209usize,],
        "result != expected"
    );
    Ok(())
}
#[test]
fn artificial_json_with_multiple_nested_same_member_names_to_stress_test_the_child_segment_with_query_select_the_path_ababc_with_owned_bytes_and_count_result_using_main_engine(
) -> Result<(), Box<dyn Error>> {
    println ! ("on document child_hell.toml running the query $..x..a.b.a.b.c (select the path ababc) with Input impl OwnedBytes and result mode CountResult");
    let jsonpath_query = JsonPathQuery::parse("$..x..a.b.a.b.c")?;
    let raw_json = fs::read_to_string("/home/mat/rsonpath/crates/rsonpath-test/tests/documents/toml/child_hell.json")?;
    let input = OwnedBytes::new(&raw_json.as_bytes())?;
    let engine = MainEngine::compile_query(&jsonpath_query)?;
    let result = engine.run::<_, CountResult>(&input)?;
    assert_eq!(result.get(), 6u64, "result != expected");
    Ok(())
}
#[test]
fn artificial_json_with_multiple_nested_same_member_names_to_stress_test_the_child_segment_with_query_select_the_path_ababc_with_owned_bytes_and_count_result_using_recursive_engine(
) -> Result<(), Box<dyn Error>> {
    println ! ("on document child_hell.toml running the query $..x..a.b.a.b.c (select the path ababc) with Input impl OwnedBytes and result mode CountResult");
    let jsonpath_query = JsonPathQuery::parse("$..x..a.b.a.b.c")?;
    let raw_json = fs::read_to_string("/home/mat/rsonpath/crates/rsonpath-test/tests/documents/toml/child_hell.json")?;
    let input = OwnedBytes::new(&raw_json.as_bytes())?;
    let engine = RecursiveEngine::compile_query(&jsonpath_query)?;
    let result = engine.run::<_, CountResult>(&input)?;
    assert_eq!(result.get(), 6u64, "result != expected");
    Ok(())
}
#[test]
fn artificial_json_with_multiple_nested_same_member_names_to_stress_test_the_child_segment_with_query_select_the_path_ababc_with_owned_bytes_and_index_result_using_main_engine(
) -> Result<(), Box<dyn Error>> {
    println ! ("on document child_hell.toml running the query $..x..a.b.a.b.c (select the path ababc) with Input impl OwnedBytes and result mode IndexResult");
    let jsonpath_query = JsonPathQuery::parse("$..x..a.b.a.b.c")?;
    let raw_json = fs::read_to_string("/home/mat/rsonpath/crates/rsonpath-test/tests/documents/toml/child_hell.json")?;
    let input = OwnedBytes::new(&raw_json.as_bytes())?;
    let engine = MainEngine::compile_query(&jsonpath_query)?;
    let result = engine.run::<_, IndexResult>(&input)?;
    assert_eq!(
        result.get(),
        vec![200usize, 758usize, 1229usize, 1905usize, 2042usize, 2209usize,],
        "result != expected"
    );
    Ok(())
}
#[test]
fn artificial_json_with_multiple_nested_same_member_names_to_stress_test_the_child_segment_with_query_select_the_path_ababc_with_owned_bytes_and_index_result_using_recursive_engine(
) -> Result<(), Box<dyn Error>> {
    println ! ("on document child_hell.toml running the query $..x..a.b.a.b.c (select the path ababc) with Input impl OwnedBytes and result mode IndexResult");
    let jsonpath_query = JsonPathQuery::parse("$..x..a.b.a.b.c")?;
    let raw_json = fs::read_to_string("/home/mat/rsonpath/crates/rsonpath-test/tests/documents/toml/child_hell.json")?;
    let input = OwnedBytes::new(&raw_json.as_bytes())?;
    let engine = RecursiveEngine::compile_query(&jsonpath_query)?;
    let result = engine.run::<_, IndexResult>(&input)?;
    assert_eq!(
        result.get(),
        vec![200usize, 758usize, 1229usize, 1905usize, 2042usize, 2209usize,],
        "result != expected"
    );
    Ok(())
}
#[test]
fn big_c_object_preceding_a_b_value_with_query_test_memchr_by_descendant_searching_for_b_with_buffered_input_and_count_result_using_main_engine(
) -> Result<(), Box<dyn Error>> {
    println ! ("on document skipping.toml running the query $..b (test memchr by descendant searching for 'b') with Input impl BufferedInput and result mode CountResult");
    let jsonpath_query = JsonPathQuery::parse("$..b")?;
    let json_file = fs::File::open("/home/mat/rsonpath/crates/rsonpath-test/tests/documents/toml/skipping.json")?;
    let input = BufferedInput::new(json_file);
    let engine = MainEngine::compile_query(&jsonpath_query)?;
    let result = engine.run::<_, CountResult>(&input)?;
    assert_eq!(result.get(), 1u64, "result != expected");
    Ok(())
}
#[test]
fn big_c_object_preceding_a_b_value_with_query_test_memchr_by_descendant_searching_for_b_with_buffered_input_and_count_result_using_recursive_engine(
) -> Result<(), Box<dyn Error>> {
    println ! ("on document skipping.toml running the query $..b (test memchr by descendant searching for 'b') with Input impl BufferedInput and result mode CountResult");
    let jsonpath_query = JsonPathQuery::parse("$..b")?;
    let json_file = fs::File::open("/home/mat/rsonpath/crates/rsonpath-test/tests/documents/toml/skipping.json")?;
    let input = BufferedInput::new(json_file);
    let engine = RecursiveEngine::compile_query(&jsonpath_query)?;
    let result = engine.run::<_, CountResult>(&input)?;
    assert_eq!(result.get(), 1u64, "result != expected");
    Ok(())
}
#[test]
fn big_c_object_preceding_a_b_value_with_query_test_memchr_by_descendant_searching_for_b_with_buffered_input_and_index_result_using_main_engine(
) -> Result<(), Box<dyn Error>> {
    println ! ("on document skipping.toml running the query $..b (test memchr by descendant searching for 'b') with Input impl BufferedInput and result mode IndexResult");
    let jsonpath_query = JsonPathQuery::parse("$..b")?;
    let json_file = fs::File::open("/home/mat/rsonpath/crates/rsonpath-test/tests/documents/toml/skipping.json")?;
    let input = BufferedInput::new(json_file);
    let engine = MainEngine::compile_query(&jsonpath_query)?;
    let result = engine.run::<_, IndexResult>(&input)?;
    assert_eq!(result.get(), vec![810usize,], "result != expected");
    Ok(())
}
#[test]
fn big_c_object_preceding_a_b_value_with_query_test_memchr_by_descendant_searching_for_b_with_buffered_input_and_index_result_using_recursive_engine(
) -> Result<(), Box<dyn Error>> {
    println ! ("on document skipping.toml running the query $..b (test memchr by descendant searching for 'b') with Input impl BufferedInput and result mode IndexResult");
    let jsonpath_query = JsonPathQuery::parse("$..b")?;
    let json_file = fs::File::open("/home/mat/rsonpath/crates/rsonpath-test/tests/documents/toml/skipping.json")?;
    let input = BufferedInput::new(json_file);
    let engine = RecursiveEngine::compile_query(&jsonpath_query)?;
    let result = engine.run::<_, IndexResult>(&input)?;
    assert_eq!(result.get(), vec![810usize,], "result != expected");
    Ok(())
}
#[test]
fn big_c_object_preceding_a_b_value_with_query_test_memchr_by_descendant_searching_for_b_with_mmap_input_and_count_result_using_main_engine(
) -> Result<(), Box<dyn Error>> {
    println ! ("on document skipping.toml running the query $..b (test memchr by descendant searching for 'b') with Input impl MmapInput and result mode CountResult");
    let jsonpath_query = JsonPathQuery::parse("$..b")?;
    let json_file = fs::File::open("/home/mat/rsonpath/crates/rsonpath-test/tests/documents/toml/skipping.json")?;
    let input = unsafe { MmapInput::map_file(&json_file)? };
    let engine = MainEngine::compile_query(&jsonpath_query)?;
    let result = engine.run::<_, CountResult>(&input)?;
    assert_eq!(result.get(), 1u64, "result != expected");
    Ok(())
}
#[test]
fn big_c_object_preceding_a_b_value_with_query_test_memchr_by_descendant_searching_for_b_with_mmap_input_and_count_result_using_recursive_engine(
) -> Result<(), Box<dyn Error>> {
    println ! ("on document skipping.toml running the query $..b (test memchr by descendant searching for 'b') with Input impl MmapInput and result mode CountResult");
    let jsonpath_query = JsonPathQuery::parse("$..b")?;
    let json_file = fs::File::open("/home/mat/rsonpath/crates/rsonpath-test/tests/documents/toml/skipping.json")?;
    let input = unsafe { MmapInput::map_file(&json_file)? };
    let engine = RecursiveEngine::compile_query(&jsonpath_query)?;
    let result = engine.run::<_, CountResult>(&input)?;
    assert_eq!(result.get(), 1u64, "result != expected");
    Ok(())
}
#[test]
fn big_c_object_preceding_a_b_value_with_query_test_memchr_by_descendant_searching_for_b_with_mmap_input_and_index_result_using_main_engine(
) -> Result<(), Box<dyn Error>> {
    println ! ("on document skipping.toml running the query $..b (test memchr by descendant searching for 'b') with Input impl MmapInput and result mode IndexResult");
    let jsonpath_query = JsonPathQuery::parse("$..b")?;
    let json_file = fs::File::open("/home/mat/rsonpath/crates/rsonpath-test/tests/documents/toml/skipping.json")?;
    let input = unsafe { MmapInput::map_file(&json_file)? };
    let engine = MainEngine::compile_query(&jsonpath_query)?;
    let result = engine.run::<_, IndexResult>(&input)?;
    assert_eq!(result.get(), vec![810usize,], "result != expected");
    Ok(())
}
#[test]
fn big_c_object_preceding_a_b_value_with_query_test_memchr_by_descendant_searching_for_b_with_mmap_input_and_index_result_using_recursive_engine(
) -> Result<(), Box<dyn Error>> {
    println ! ("on document skipping.toml running the query $..b (test memchr by descendant searching for 'b') with Input impl MmapInput and result mode IndexResult");
    let jsonpath_query = JsonPathQuery::parse("$..b")?;
    let json_file = fs::File::open("/home/mat/rsonpath/crates/rsonpath-test/tests/documents/toml/skipping.json")?;
    let input = unsafe { MmapInput::map_file(&json_file)? };
    let engine = RecursiveEngine::compile_query(&jsonpath_query)?;
    let result = engine.run::<_, IndexResult>(&input)?;
    assert_eq!(result.get(), vec![810usize,], "result != expected");
    Ok(())
}
#[test]
fn big_c_object_preceding_a_b_value_with_query_test_memchr_by_descendant_searching_for_b_with_owned_bytes_and_count_result_using_main_engine(
) -> Result<(), Box<dyn Error>> {
    println ! ("on document skipping.toml running the query $..b (test memchr by descendant searching for 'b') with Input impl OwnedBytes and result mode CountResult");
    let jsonpath_query = JsonPathQuery::parse("$..b")?;
    let raw_json = fs::read_to_string("/home/mat/rsonpath/crates/rsonpath-test/tests/documents/toml/skipping.json")?;
    let input = OwnedBytes::new(&raw_json.as_bytes())?;
    let engine = MainEngine::compile_query(&jsonpath_query)?;
    let result = engine.run::<_, CountResult>(&input)?;
    assert_eq!(result.get(), 1u64, "result != expected");
    Ok(())
}
#[test]
fn big_c_object_preceding_a_b_value_with_query_test_memchr_by_descendant_searching_for_b_with_owned_bytes_and_count_result_using_recursive_engine(
) -> Result<(), Box<dyn Error>> {
    println ! ("on document skipping.toml running the query $..b (test memchr by descendant searching for 'b') with Input impl OwnedBytes and result mode CountResult");
    let jsonpath_query = JsonPathQuery::parse("$..b")?;
    let raw_json = fs::read_to_string("/home/mat/rsonpath/crates/rsonpath-test/tests/documents/toml/skipping.json")?;
    let input = OwnedBytes::new(&raw_json.as_bytes())?;
    let engine = RecursiveEngine::compile_query(&jsonpath_query)?;
    let result = engine.run::<_, CountResult>(&input)?;
    assert_eq!(result.get(), 1u64, "result != expected");
    Ok(())
}
#[test]
fn big_c_object_preceding_a_b_value_with_query_test_memchr_by_descendant_searching_for_b_with_owned_bytes_and_index_result_using_main_engine(
) -> Result<(), Box<dyn Error>> {
    println ! ("on document skipping.toml running the query $..b (test memchr by descendant searching for 'b') with Input impl OwnedBytes and result mode IndexResult");
    let jsonpath_query = JsonPathQuery::parse("$..b")?;
    let raw_json = fs::read_to_string("/home/mat/rsonpath/crates/rsonpath-test/tests/documents/toml/skipping.json")?;
    let input = OwnedBytes::new(&raw_json.as_bytes())?;
    let engine = MainEngine::compile_query(&jsonpath_query)?;
    let result = engine.run::<_, IndexResult>(&input)?;
    assert_eq!(result.get(), vec![810usize,], "result != expected");
    Ok(())
}
#[test]
fn big_c_object_preceding_a_b_value_with_query_test_memchr_by_descendant_searching_for_b_with_owned_bytes_and_index_result_using_recursive_engine(
) -> Result<(), Box<dyn Error>> {
    println ! ("on document skipping.toml running the query $..b (test memchr by descendant searching for 'b') with Input impl OwnedBytes and result mode IndexResult");
    let jsonpath_query = JsonPathQuery::parse("$..b")?;
    let raw_json = fs::read_to_string("/home/mat/rsonpath/crates/rsonpath-test/tests/documents/toml/skipping.json")?;
    let input = OwnedBytes::new(&raw_json.as_bytes())?;
    let engine = RecursiveEngine::compile_query(&jsonpath_query)?;
    let result = engine.run::<_, IndexResult>(&input)?;
    assert_eq!(result.get(), vec![810usize,], "result != expected");
    Ok(())
}
#[test]
fn big_c_object_preceding_a_b_value_with_query_test_skipping_by_child_selecting_b_after_descendant_selecting_a_with_buffered_input_and_count_result_using_main_engine(
) -> Result<(), Box<dyn Error>> {
    println ! ("on document skipping.toml running the query $..a.b (test skipping by child-selecting 'b' after descendant selecting 'a') with Input impl BufferedInput and result mode CountResult");
    let jsonpath_query = JsonPathQuery::parse("$..a.b")?;
    let json_file = fs::File::open("/home/mat/rsonpath/crates/rsonpath-test/tests/documents/toml/skipping.json")?;
    let input = BufferedInput::new(json_file);
    let engine = MainEngine::compile_query(&jsonpath_query)?;
    let result = engine.run::<_, CountResult>(&input)?;
    assert_eq!(result.get(), 1u64, "result != expected");
    Ok(())
}
#[test]
fn big_c_object_preceding_a_b_value_with_query_test_skipping_by_child_selecting_b_after_descendant_selecting_a_with_buffered_input_and_count_result_using_recursive_engine(
) -> Result<(), Box<dyn Error>> {
    println ! ("on document skipping.toml running the query $..a.b (test skipping by child-selecting 'b' after descendant selecting 'a') with Input impl BufferedInput and result mode CountResult");
    let jsonpath_query = JsonPathQuery::parse("$..a.b")?;
    let json_file = fs::File::open("/home/mat/rsonpath/crates/rsonpath-test/tests/documents/toml/skipping.json")?;
    let input = BufferedInput::new(json_file);
    let engine = RecursiveEngine::compile_query(&jsonpath_query)?;
    let result = engine.run::<_, CountResult>(&input)?;
    assert_eq!(result.get(), 1u64, "result != expected");
    Ok(())
}
#[test]
fn big_c_object_preceding_a_b_value_with_query_test_skipping_by_child_selecting_b_after_descendant_selecting_a_with_buffered_input_and_index_result_using_main_engine(
) -> Result<(), Box<dyn Error>> {
    println ! ("on document skipping.toml running the query $..a.b (test skipping by child-selecting 'b' after descendant selecting 'a') with Input impl BufferedInput and result mode IndexResult");
    let jsonpath_query = JsonPathQuery::parse("$..a.b")?;
    let json_file = fs::File::open("/home/mat/rsonpath/crates/rsonpath-test/tests/documents/toml/skipping.json")?;
    let input = BufferedInput::new(json_file);
    let engine = MainEngine::compile_query(&jsonpath_query)?;
    let result = engine.run::<_, IndexResult>(&input)?;
    assert_eq!(result.get(), vec![810usize,], "result != expected");
    Ok(())
}
#[test]
fn big_c_object_preceding_a_b_value_with_query_test_skipping_by_child_selecting_b_after_descendant_selecting_a_with_buffered_input_and_index_result_using_recursive_engine(
) -> Result<(), Box<dyn Error>> {
    println ! ("on document skipping.toml running the query $..a.b (test skipping by child-selecting 'b' after descendant selecting 'a') with Input impl BufferedInput and result mode IndexResult");
    let jsonpath_query = JsonPathQuery::parse("$..a.b")?;
    let json_file = fs::File::open("/home/mat/rsonpath/crates/rsonpath-test/tests/documents/toml/skipping.json")?;
    let input = BufferedInput::new(json_file);
    let engine = RecursiveEngine::compile_query(&jsonpath_query)?;
    let result = engine.run::<_, IndexResult>(&input)?;
    assert_eq!(result.get(), vec![810usize,], "result != expected");
    Ok(())
}
#[test]
fn big_c_object_preceding_a_b_value_with_query_test_skipping_by_child_selecting_b_after_descendant_selecting_a_with_mmap_input_and_count_result_using_main_engine(
) -> Result<(), Box<dyn Error>> {
    println ! ("on document skipping.toml running the query $..a.b (test skipping by child-selecting 'b' after descendant selecting 'a') with Input impl MmapInput and result mode CountResult");
    let jsonpath_query = JsonPathQuery::parse("$..a.b")?;
    let json_file = fs::File::open("/home/mat/rsonpath/crates/rsonpath-test/tests/documents/toml/skipping.json")?;
    let input = unsafe { MmapInput::map_file(&json_file)? };
    let engine = MainEngine::compile_query(&jsonpath_query)?;
    let result = engine.run::<_, CountResult>(&input)?;
    assert_eq!(result.get(), 1u64, "result != expected");
    Ok(())
}
#[test]
fn big_c_object_preceding_a_b_value_with_query_test_skipping_by_child_selecting_b_after_descendant_selecting_a_with_mmap_input_and_count_result_using_recursive_engine(
) -> Result<(), Box<dyn Error>> {
    println ! ("on document skipping.toml running the query $..a.b (test skipping by child-selecting 'b' after descendant selecting 'a') with Input impl MmapInput and result mode CountResult");
    let jsonpath_query = JsonPathQuery::parse("$..a.b")?;
    let json_file = fs::File::open("/home/mat/rsonpath/crates/rsonpath-test/tests/documents/toml/skipping.json")?;
    let input = unsafe { MmapInput::map_file(&json_file)? };
    let engine = RecursiveEngine::compile_query(&jsonpath_query)?;
    let result = engine.run::<_, CountResult>(&input)?;
    assert_eq!(result.get(), 1u64, "result != expected");
    Ok(())
}
#[test]
fn big_c_object_preceding_a_b_value_with_query_test_skipping_by_child_selecting_b_after_descendant_selecting_a_with_mmap_input_and_index_result_using_main_engine(
) -> Result<(), Box<dyn Error>> {
    println ! ("on document skipping.toml running the query $..a.b (test skipping by child-selecting 'b' after descendant selecting 'a') with Input impl MmapInput and result mode IndexResult");
    let jsonpath_query = JsonPathQuery::parse("$..a.b")?;
    let json_file = fs::File::open("/home/mat/rsonpath/crates/rsonpath-test/tests/documents/toml/skipping.json")?;
    let input = unsafe { MmapInput::map_file(&json_file)? };
    let engine = MainEngine::compile_query(&jsonpath_query)?;
    let result = engine.run::<_, IndexResult>(&input)?;
    assert_eq!(result.get(), vec![810usize,], "result != expected");
    Ok(())
}
#[test]
fn big_c_object_preceding_a_b_value_with_query_test_skipping_by_child_selecting_b_after_descendant_selecting_a_with_mmap_input_and_index_result_using_recursive_engine(
) -> Result<(), Box<dyn Error>> {
    println ! ("on document skipping.toml running the query $..a.b (test skipping by child-selecting 'b' after descendant selecting 'a') with Input impl MmapInput and result mode IndexResult");
    let jsonpath_query = JsonPathQuery::parse("$..a.b")?;
    let json_file = fs::File::open("/home/mat/rsonpath/crates/rsonpath-test/tests/documents/toml/skipping.json")?;
    let input = unsafe { MmapInput::map_file(&json_file)? };
    let engine = RecursiveEngine::compile_query(&jsonpath_query)?;
    let result = engine.run::<_, IndexResult>(&input)?;
    assert_eq!(result.get(), vec![810usize,], "result != expected");
    Ok(())
}
#[test]
fn big_c_object_preceding_a_b_value_with_query_test_skipping_by_child_selecting_b_after_descendant_selecting_a_with_owned_bytes_and_count_result_using_main_engine(
) -> Result<(), Box<dyn Error>> {
    println ! ("on document skipping.toml running the query $..a.b (test skipping by child-selecting 'b' after descendant selecting 'a') with Input impl OwnedBytes and result mode CountResult");
    let jsonpath_query = JsonPathQuery::parse("$..a.b")?;
    let raw_json = fs::read_to_string("/home/mat/rsonpath/crates/rsonpath-test/tests/documents/toml/skipping.json")?;
    let input = OwnedBytes::new(&raw_json.as_bytes())?;
    let engine = MainEngine::compile_query(&jsonpath_query)?;
    let result = engine.run::<_, CountResult>(&input)?;
    assert_eq!(result.get(), 1u64, "result != expected");
    Ok(())
}
#[test]
fn big_c_object_preceding_a_b_value_with_query_test_skipping_by_child_selecting_b_after_descendant_selecting_a_with_owned_bytes_and_count_result_using_recursive_engine(
) -> Result<(), Box<dyn Error>> {
    println ! ("on document skipping.toml running the query $..a.b (test skipping by child-selecting 'b' after descendant selecting 'a') with Input impl OwnedBytes and result mode CountResult");
    let jsonpath_query = JsonPathQuery::parse("$..a.b")?;
    let raw_json = fs::read_to_string("/home/mat/rsonpath/crates/rsonpath-test/tests/documents/toml/skipping.json")?;
    let input = OwnedBytes::new(&raw_json.as_bytes())?;
    let engine = RecursiveEngine::compile_query(&jsonpath_query)?;
    let result = engine.run::<_, CountResult>(&input)?;
    assert_eq!(result.get(), 1u64, "result != expected");
    Ok(())
}
#[test]
fn big_c_object_preceding_a_b_value_with_query_test_skipping_by_child_selecting_b_after_descendant_selecting_a_with_owned_bytes_and_index_result_using_main_engine(
) -> Result<(), Box<dyn Error>> {
    println ! ("on document skipping.toml running the query $..a.b (test skipping by child-selecting 'b' after descendant selecting 'a') with Input impl OwnedBytes and result mode IndexResult");
    let jsonpath_query = JsonPathQuery::parse("$..a.b")?;
    let raw_json = fs::read_to_string("/home/mat/rsonpath/crates/rsonpath-test/tests/documents/toml/skipping.json")?;
    let input = OwnedBytes::new(&raw_json.as_bytes())?;
    let engine = MainEngine::compile_query(&jsonpath_query)?;
    let result = engine.run::<_, IndexResult>(&input)?;
    assert_eq!(result.get(), vec![810usize,], "result != expected");
    Ok(())
}
#[test]
fn big_c_object_preceding_a_b_value_with_query_test_skipping_by_child_selecting_b_after_descendant_selecting_a_with_owned_bytes_and_index_result_using_recursive_engine(
) -> Result<(), Box<dyn Error>> {
    println ! ("on document skipping.toml running the query $..a.b (test skipping by child-selecting 'b' after descendant selecting 'a') with Input impl OwnedBytes and result mode IndexResult");
    let jsonpath_query = JsonPathQuery::parse("$..a.b")?;
    let raw_json = fs::read_to_string("/home/mat/rsonpath/crates/rsonpath-test/tests/documents/toml/skipping.json")?;
    let input = OwnedBytes::new(&raw_json.as_bytes())?;
    let engine = RecursiveEngine::compile_query(&jsonpath_query)?;
    let result = engine.run::<_, IndexResult>(&input)?;
    assert_eq!(result.get(), vec![810usize,], "result != expected");
    Ok(())
}
#[test]
fn big_c_object_preceding_a_b_value_with_query_test_skipping_by_child_selecting_b_with_buffered_input_and_count_result_using_main_engine(
) -> Result<(), Box<dyn Error>> {
    println ! ("on document skipping.toml running the query $.a.b (test skipping by child-selecting 'b') with Input impl BufferedInput and result mode CountResult");
    let jsonpath_query = JsonPathQuery::parse("$.a.b")?;
    let json_file = fs::File::open("/home/mat/rsonpath/crates/rsonpath-test/tests/documents/toml/skipping.json")?;
    let input = BufferedInput::new(json_file);
    let engine = MainEngine::compile_query(&jsonpath_query)?;
    let result = engine.run::<_, CountResult>(&input)?;
    assert_eq!(result.get(), 1u64, "result != expected");
    Ok(())
}
#[test]
fn big_c_object_preceding_a_b_value_with_query_test_skipping_by_child_selecting_b_with_buffered_input_and_count_result_using_recursive_engine(
) -> Result<(), Box<dyn Error>> {
    println ! ("on document skipping.toml running the query $.a.b (test skipping by child-selecting 'b') with Input impl BufferedInput and result mode CountResult");
    let jsonpath_query = JsonPathQuery::parse("$.a.b")?;
    let json_file = fs::File::open("/home/mat/rsonpath/crates/rsonpath-test/tests/documents/toml/skipping.json")?;
    let input = BufferedInput::new(json_file);
    let engine = RecursiveEngine::compile_query(&jsonpath_query)?;
    let result = engine.run::<_, CountResult>(&input)?;
    assert_eq!(result.get(), 1u64, "result != expected");
    Ok(())
}
#[test]
fn big_c_object_preceding_a_b_value_with_query_test_skipping_by_child_selecting_b_with_buffered_input_and_index_result_using_main_engine(
) -> Result<(), Box<dyn Error>> {
    println ! ("on document skipping.toml running the query $.a.b (test skipping by child-selecting 'b') with Input impl BufferedInput and result mode IndexResult");
    let jsonpath_query = JsonPathQuery::parse("$.a.b")?;
    let json_file = fs::File::open("/home/mat/rsonpath/crates/rsonpath-test/tests/documents/toml/skipping.json")?;
    let input = BufferedInput::new(json_file);
    let engine = MainEngine::compile_query(&jsonpath_query)?;
    let result = engine.run::<_, IndexResult>(&input)?;
    assert_eq!(result.get(), vec![810usize,], "result != expected");
    Ok(())
}
#[test]
fn big_c_object_preceding_a_b_value_with_query_test_skipping_by_child_selecting_b_with_buffered_input_and_index_result_using_recursive_engine(
) -> Result<(), Box<dyn Error>> {
    println ! ("on document skipping.toml running the query $.a.b (test skipping by child-selecting 'b') with Input impl BufferedInput and result mode IndexResult");
    let jsonpath_query = JsonPathQuery::parse("$.a.b")?;
    let json_file = fs::File::open("/home/mat/rsonpath/crates/rsonpath-test/tests/documents/toml/skipping.json")?;
    let input = BufferedInput::new(json_file);
    let engine = RecursiveEngine::compile_query(&jsonpath_query)?;
    let result = engine.run::<_, IndexResult>(&input)?;
    assert_eq!(result.get(), vec![810usize,], "result != expected");
    Ok(())
}
#[test]
fn big_c_object_preceding_a_b_value_with_query_test_skipping_by_child_selecting_b_with_mmap_input_and_count_result_using_main_engine(
) -> Result<(), Box<dyn Error>> {
    println ! ("on document skipping.toml running the query $.a.b (test skipping by child-selecting 'b') with Input impl MmapInput and result mode CountResult");
    let jsonpath_query = JsonPathQuery::parse("$.a.b")?;
    let json_file = fs::File::open("/home/mat/rsonpath/crates/rsonpath-test/tests/documents/toml/skipping.json")?;
    let input = unsafe { MmapInput::map_file(&json_file)? };
    let engine = MainEngine::compile_query(&jsonpath_query)?;
    let result = engine.run::<_, CountResult>(&input)?;
    assert_eq!(result.get(), 1u64, "result != expected");
    Ok(())
}
#[test]
fn big_c_object_preceding_a_b_value_with_query_test_skipping_by_child_selecting_b_with_mmap_input_and_count_result_using_recursive_engine(
) -> Result<(), Box<dyn Error>> {
    println ! ("on document skipping.toml running the query $.a.b (test skipping by child-selecting 'b') with Input impl MmapInput and result mode CountResult");
    let jsonpath_query = JsonPathQuery::parse("$.a.b")?;
    let json_file = fs::File::open("/home/mat/rsonpath/crates/rsonpath-test/tests/documents/toml/skipping.json")?;
    let input = unsafe { MmapInput::map_file(&json_file)? };
    let engine = RecursiveEngine::compile_query(&jsonpath_query)?;
    let result = engine.run::<_, CountResult>(&input)?;
    assert_eq!(result.get(), 1u64, "result != expected");
    Ok(())
}
#[test]
fn big_c_object_preceding_a_b_value_with_query_test_skipping_by_child_selecting_b_with_mmap_input_and_index_result_using_main_engine(
) -> Result<(), Box<dyn Error>> {
    println ! ("on document skipping.toml running the query $.a.b (test skipping by child-selecting 'b') with Input impl MmapInput and result mode IndexResult");
    let jsonpath_query = JsonPathQuery::parse("$.a.b")?;
    let json_file = fs::File::open("/home/mat/rsonpath/crates/rsonpath-test/tests/documents/toml/skipping.json")?;
    let input = unsafe { MmapInput::map_file(&json_file)? };
    let engine = MainEngine::compile_query(&jsonpath_query)?;
    let result = engine.run::<_, IndexResult>(&input)?;
    assert_eq!(result.get(), vec![810usize,], "result != expected");
    Ok(())
}
#[test]
fn big_c_object_preceding_a_b_value_with_query_test_skipping_by_child_selecting_b_with_mmap_input_and_index_result_using_recursive_engine(
) -> Result<(), Box<dyn Error>> {
    println ! ("on document skipping.toml running the query $.a.b (test skipping by child-selecting 'b') with Input impl MmapInput and result mode IndexResult");
    let jsonpath_query = JsonPathQuery::parse("$.a.b")?;
    let json_file = fs::File::open("/home/mat/rsonpath/crates/rsonpath-test/tests/documents/toml/skipping.json")?;
    let input = unsafe { MmapInput::map_file(&json_file)? };
    let engine = RecursiveEngine::compile_query(&jsonpath_query)?;
    let result = engine.run::<_, IndexResult>(&input)?;
    assert_eq!(result.get(), vec![810usize,], "result != expected");
    Ok(())
}
#[test]
fn big_c_object_preceding_a_b_value_with_query_test_skipping_by_child_selecting_b_with_owned_bytes_and_count_result_using_main_engine(
) -> Result<(), Box<dyn Error>> {
    println ! ("on document skipping.toml running the query $.a.b (test skipping by child-selecting 'b') with Input impl OwnedBytes and result mode CountResult");
    let jsonpath_query = JsonPathQuery::parse("$.a.b")?;
    let raw_json = fs::read_to_string("/home/mat/rsonpath/crates/rsonpath-test/tests/documents/toml/skipping.json")?;
    let input = OwnedBytes::new(&raw_json.as_bytes())?;
    let engine = MainEngine::compile_query(&jsonpath_query)?;
    let result = engine.run::<_, CountResult>(&input)?;
    assert_eq!(result.get(), 1u64, "result != expected");
    Ok(())
}
#[test]
fn big_c_object_preceding_a_b_value_with_query_test_skipping_by_child_selecting_b_with_owned_bytes_and_count_result_using_recursive_engine(
) -> Result<(), Box<dyn Error>> {
    println ! ("on document skipping.toml running the query $.a.b (test skipping by child-selecting 'b') with Input impl OwnedBytes and result mode CountResult");
    let jsonpath_query = JsonPathQuery::parse("$.a.b")?;
    let raw_json = fs::read_to_string("/home/mat/rsonpath/crates/rsonpath-test/tests/documents/toml/skipping.json")?;
    let input = OwnedBytes::new(&raw_json.as_bytes())?;
    let engine = RecursiveEngine::compile_query(&jsonpath_query)?;
    let result = engine.run::<_, CountResult>(&input)?;
    assert_eq!(result.get(), 1u64, "result != expected");
    Ok(())
}
#[test]
fn big_c_object_preceding_a_b_value_with_query_test_skipping_by_child_selecting_b_with_owned_bytes_and_index_result_using_main_engine(
) -> Result<(), Box<dyn Error>> {
    println ! ("on document skipping.toml running the query $.a.b (test skipping by child-selecting 'b') with Input impl OwnedBytes and result mode IndexResult");
    let jsonpath_query = JsonPathQuery::parse("$.a.b")?;
    let raw_json = fs::read_to_string("/home/mat/rsonpath/crates/rsonpath-test/tests/documents/toml/skipping.json")?;
    let input = OwnedBytes::new(&raw_json.as_bytes())?;
    let engine = MainEngine::compile_query(&jsonpath_query)?;
    let result = engine.run::<_, IndexResult>(&input)?;
    assert_eq!(result.get(), vec![810usize,], "result != expected");
    Ok(())
}
#[test]
fn big_c_object_preceding_a_b_value_with_query_test_skipping_by_child_selecting_b_with_owned_bytes_and_index_result_using_recursive_engine(
) -> Result<(), Box<dyn Error>> {
    println ! ("on document skipping.toml running the query $.a.b (test skipping by child-selecting 'b') with Input impl OwnedBytes and result mode IndexResult");
    let jsonpath_query = JsonPathQuery::parse("$.a.b")?;
    let raw_json = fs::read_to_string("/home/mat/rsonpath/crates/rsonpath-test/tests/documents/toml/skipping.json")?;
    let input = OwnedBytes::new(&raw_json.as_bytes())?;
    let engine = RecursiveEngine::compile_query(&jsonpath_query)?;
    let result = engine.run::<_, IndexResult>(&input)?;
    assert_eq!(result.get(), vec![810usize,], "result != expected");
    Ok(())
}
#[test]
fn deeply_nested_list_structures_with_query_select_each_element_nested_at_a_with_buffered_input_and_count_result_using_main_engine(
) -> Result<(), Box<dyn Error>> {
    println ! ("on document deep_list_nesting.toml running the query $..a.* (select each element nested at 'a') with Input impl BufferedInput and result mode CountResult");
    let jsonpath_query = JsonPathQuery::parse("$..a.*")?;
    let json_file =
        fs::File::open("/home/mat/rsonpath/crates/rsonpath-test/tests/documents/toml/deep_list_nesting.json")?;
    let input = BufferedInput::new(json_file);
    let engine = MainEngine::compile_query(&jsonpath_query)?;
    let result = engine.run::<_, CountResult>(&input)?;
    assert_eq!(result.get(), 6u64, "result != expected");
    Ok(())
}
#[test]
fn deeply_nested_list_structures_with_query_select_each_element_nested_at_a_with_buffered_input_and_count_result_using_recursive_engine(
) -> Result<(), Box<dyn Error>> {
    println ! ("on document deep_list_nesting.toml running the query $..a.* (select each element nested at 'a') with Input impl BufferedInput and result mode CountResult");
    let jsonpath_query = JsonPathQuery::parse("$..a.*")?;
    let json_file =
        fs::File::open("/home/mat/rsonpath/crates/rsonpath-test/tests/documents/toml/deep_list_nesting.json")?;
    let input = BufferedInput::new(json_file);
    let engine = RecursiveEngine::compile_query(&jsonpath_query)?;
    let result = engine.run::<_, CountResult>(&input)?;
    assert_eq!(result.get(), 6u64, "result != expected");
    Ok(())
}
#[test]
fn deeply_nested_list_structures_with_query_select_each_element_nested_at_a_with_buffered_input_and_index_result_using_main_engine(
) -> Result<(), Box<dyn Error>> {
    println ! ("on document deep_list_nesting.toml running the query $..a.* (select each element nested at 'a') with Input impl BufferedInput and result mode IndexResult");
    let jsonpath_query = JsonPathQuery::parse("$..a.*")?;
    let json_file =
        fs::File::open("/home/mat/rsonpath/crates/rsonpath-test/tests/documents/toml/deep_list_nesting.json")?;
    let input = BufferedInput::new(json_file);
    let engine = MainEngine::compile_query(&jsonpath_query)?;
    let result = engine.run::<_, IndexResult>(&input)?;
    assert_eq!(
        result.get(),
        vec![63usize, 82usize, 101usize, 121usize, 141usize, 305usize,],
        "result != expected"
    );
    Ok(())
}
#[test]
fn deeply_nested_list_structures_with_query_select_each_element_nested_at_a_with_buffered_input_and_index_result_using_recursive_engine(
) -> Result<(), Box<dyn Error>> {
    println ! ("on document deep_list_nesting.toml running the query $..a.* (select each element nested at 'a') with Input impl BufferedInput and result mode IndexResult");
    let jsonpath_query = JsonPathQuery::parse("$..a.*")?;
    let json_file =
        fs::File::open("/home/mat/rsonpath/crates/rsonpath-test/tests/documents/toml/deep_list_nesting.json")?;
    let input = BufferedInput::new(json_file);
    let engine = RecursiveEngine::compile_query(&jsonpath_query)?;
    let result = engine.run::<_, IndexResult>(&input)?;
    assert_eq!(
        result.get(),
        vec![63usize, 82usize, 101usize, 121usize, 141usize, 305usize,],
        "result != expected"
    );
    Ok(())
}
#[test]
fn deeply_nested_list_structures_with_query_select_each_element_nested_at_a_with_mmap_input_and_count_result_using_main_engine(
) -> Result<(), Box<dyn Error>> {
    println ! ("on document deep_list_nesting.toml running the query $..a.* (select each element nested at 'a') with Input impl MmapInput and result mode CountResult");
    let jsonpath_query = JsonPathQuery::parse("$..a.*")?;
    let json_file =
        fs::File::open("/home/mat/rsonpath/crates/rsonpath-test/tests/documents/toml/deep_list_nesting.json")?;
    let input = unsafe { MmapInput::map_file(&json_file)? };
    let engine = MainEngine::compile_query(&jsonpath_query)?;
    let result = engine.run::<_, CountResult>(&input)?;
    assert_eq!(result.get(), 6u64, "result != expected");
    Ok(())
}
#[test]
fn deeply_nested_list_structures_with_query_select_each_element_nested_at_a_with_mmap_input_and_count_result_using_recursive_engine(
) -> Result<(), Box<dyn Error>> {
    println ! ("on document deep_list_nesting.toml running the query $..a.* (select each element nested at 'a') with Input impl MmapInput and result mode CountResult");
    let jsonpath_query = JsonPathQuery::parse("$..a.*")?;
    let json_file =
        fs::File::open("/home/mat/rsonpath/crates/rsonpath-test/tests/documents/toml/deep_list_nesting.json")?;
    let input = unsafe { MmapInput::map_file(&json_file)? };
    let engine = RecursiveEngine::compile_query(&jsonpath_query)?;
    let result = engine.run::<_, CountResult>(&input)?;
    assert_eq!(result.get(), 6u64, "result != expected");
    Ok(())
}
#[test]
fn deeply_nested_list_structures_with_query_select_each_element_nested_at_a_with_mmap_input_and_index_result_using_main_engine(
) -> Result<(), Box<dyn Error>> {
    println ! ("on document deep_list_nesting.toml running the query $..a.* (select each element nested at 'a') with Input impl MmapInput and result mode IndexResult");
    let jsonpath_query = JsonPathQuery::parse("$..a.*")?;
    let json_file =
        fs::File::open("/home/mat/rsonpath/crates/rsonpath-test/tests/documents/toml/deep_list_nesting.json")?;
    let input = unsafe { MmapInput::map_file(&json_file)? };
    let engine = MainEngine::compile_query(&jsonpath_query)?;
    let result = engine.run::<_, IndexResult>(&input)?;
    assert_eq!(
        result.get(),
        vec![63usize, 82usize, 101usize, 121usize, 141usize, 305usize,],
        "result != expected"
    );
    Ok(())
}
#[test]
fn deeply_nested_list_structures_with_query_select_each_element_nested_at_a_with_mmap_input_and_index_result_using_recursive_engine(
) -> Result<(), Box<dyn Error>> {
    println ! ("on document deep_list_nesting.toml running the query $..a.* (select each element nested at 'a') with Input impl MmapInput and result mode IndexResult");
    let jsonpath_query = JsonPathQuery::parse("$..a.*")?;
    let json_file =
        fs::File::open("/home/mat/rsonpath/crates/rsonpath-test/tests/documents/toml/deep_list_nesting.json")?;
    let input = unsafe { MmapInput::map_file(&json_file)? };
    let engine = RecursiveEngine::compile_query(&jsonpath_query)?;
    let result = engine.run::<_, IndexResult>(&input)?;
    assert_eq!(
        result.get(),
        vec![63usize, 82usize, 101usize, 121usize, 141usize, 305usize,],
        "result != expected"
    );
    Ok(())
}
#[test]
fn deeply_nested_list_structures_with_query_select_each_element_nested_at_a_with_owned_bytes_and_count_result_using_main_engine(
) -> Result<(), Box<dyn Error>> {
    println ! ("on document deep_list_nesting.toml running the query $..a.* (select each element nested at 'a') with Input impl OwnedBytes and result mode CountResult");
    let jsonpath_query = JsonPathQuery::parse("$..a.*")?;
    let raw_json =
        fs::read_to_string("/home/mat/rsonpath/crates/rsonpath-test/tests/documents/toml/deep_list_nesting.json")?;
    let input = OwnedBytes::new(&raw_json.as_bytes())?;
    let engine = MainEngine::compile_query(&jsonpath_query)?;
    let result = engine.run::<_, CountResult>(&input)?;
    assert_eq!(result.get(), 6u64, "result != expected");
    Ok(())
}
#[test]
fn deeply_nested_list_structures_with_query_select_each_element_nested_at_a_with_owned_bytes_and_count_result_using_recursive_engine(
) -> Result<(), Box<dyn Error>> {
    println ! ("on document deep_list_nesting.toml running the query $..a.* (select each element nested at 'a') with Input impl OwnedBytes and result mode CountResult");
    let jsonpath_query = JsonPathQuery::parse("$..a.*")?;
    let raw_json =
        fs::read_to_string("/home/mat/rsonpath/crates/rsonpath-test/tests/documents/toml/deep_list_nesting.json")?;
    let input = OwnedBytes::new(&raw_json.as_bytes())?;
    let engine = RecursiveEngine::compile_query(&jsonpath_query)?;
    let result = engine.run::<_, CountResult>(&input)?;
    assert_eq!(result.get(), 6u64, "result != expected");
    Ok(())
}
#[test]
fn deeply_nested_list_structures_with_query_select_each_element_nested_at_a_with_owned_bytes_and_index_result_using_main_engine(
) -> Result<(), Box<dyn Error>> {
    println ! ("on document deep_list_nesting.toml running the query $..a.* (select each element nested at 'a') with Input impl OwnedBytes and result mode IndexResult");
    let jsonpath_query = JsonPathQuery::parse("$..a.*")?;
    let raw_json =
        fs::read_to_string("/home/mat/rsonpath/crates/rsonpath-test/tests/documents/toml/deep_list_nesting.json")?;
    let input = OwnedBytes::new(&raw_json.as_bytes())?;
    let engine = MainEngine::compile_query(&jsonpath_query)?;
    let result = engine.run::<_, IndexResult>(&input)?;
    assert_eq!(
        result.get(),
        vec![63usize, 82usize, 101usize, 121usize, 141usize, 305usize,],
        "result != expected"
    );
    Ok(())
}
#[test]
fn deeply_nested_list_structures_with_query_select_each_element_nested_at_a_with_owned_bytes_and_index_result_using_recursive_engine(
) -> Result<(), Box<dyn Error>> {
    println ! ("on document deep_list_nesting.toml running the query $..a.* (select each element nested at 'a') with Input impl OwnedBytes and result mode IndexResult");
    let jsonpath_query = JsonPathQuery::parse("$..a.*")?;
    let raw_json =
        fs::read_to_string("/home/mat/rsonpath/crates/rsonpath-test/tests/documents/toml/deep_list_nesting.json")?;
    let input = OwnedBytes::new(&raw_json.as_bytes())?;
    let engine = RecursiveEngine::compile_query(&jsonpath_query)?;
    let result = engine.run::<_, IndexResult>(&input)?;
    assert_eq!(
        result.get(),
        vec![63usize, 82usize, 101usize, 121usize, 141usize, 305usize,],
        "result != expected"
    );
    Ok(())
}
#[test]
fn deeply_nested_single_atomic_int_after_a_list_of_ints_with_query_direct_path_to_the_third_element_on_the_list_with_buffered_input_and_count_result_using_main_engine(
) -> Result<(), Box<dyn Error>> {
    println ! ("on document int_after_list_of_ints.toml running the query $.a[0].c.d[2] (direct path to the third element on the list) with Input impl BufferedInput and result mode CountResult");
    let jsonpath_query = JsonPathQuery::parse("$.a[0].c.d[2]")?;
    let json_file =
        fs::File::open("/home/mat/rsonpath/crates/rsonpath-test/tests/documents/toml/int_after_list_of_ints.json")?;
    let input = BufferedInput::new(json_file);
    let engine = MainEngine::compile_query(&jsonpath_query)?;
    let result = engine.run::<_, CountResult>(&input)?;
    assert_eq!(result.get(), 1u64, "result != expected");
    Ok(())
}
#[test]
fn deeply_nested_single_atomic_int_after_a_list_of_ints_with_query_direct_path_to_the_third_element_on_the_list_with_buffered_input_and_count_result_using_recursive_engine(
) -> Result<(), Box<dyn Error>> {
    println ! ("on document int_after_list_of_ints.toml running the query $.a[0].c.d[2] (direct path to the third element on the list) with Input impl BufferedInput and result mode CountResult");
    let jsonpath_query = JsonPathQuery::parse("$.a[0].c.d[2]")?;
    let json_file =
        fs::File::open("/home/mat/rsonpath/crates/rsonpath-test/tests/documents/toml/int_after_list_of_ints.json")?;
    let input = BufferedInput::new(json_file);
    let engine = RecursiveEngine::compile_query(&jsonpath_query)?;
    let result = engine.run::<_, CountResult>(&input)?;
    assert_eq!(result.get(), 1u64, "result != expected");
    Ok(())
}
#[test]
fn deeply_nested_single_atomic_int_after_a_list_of_ints_with_query_direct_path_to_the_third_element_on_the_list_with_buffered_input_and_index_result_using_main_engine(
) -> Result<(), Box<dyn Error>> {
    println ! ("on document int_after_list_of_ints.toml running the query $.a[0].c.d[2] (direct path to the third element on the list) with Input impl BufferedInput and result mode IndexResult");
    let jsonpath_query = JsonPathQuery::parse("$.a[0].c.d[2]")?;
    let json_file =
        fs::File::open("/home/mat/rsonpath/crates/rsonpath-test/tests/documents/toml/int_after_list_of_ints.json")?;
    let input = BufferedInput::new(json_file);
    let engine = MainEngine::compile_query(&jsonpath_query)?;
    let result = engine.run::<_, IndexResult>(&input)?;
    assert_eq!(result.get(), vec![133usize,], "result != expected");
    Ok(())
}
#[test]
fn deeply_nested_single_atomic_int_after_a_list_of_ints_with_query_direct_path_to_the_third_element_on_the_list_with_buffered_input_and_index_result_using_recursive_engine(
) -> Result<(), Box<dyn Error>> {
    println ! ("on document int_after_list_of_ints.toml running the query $.a[0].c.d[2] (direct path to the third element on the list) with Input impl BufferedInput and result mode IndexResult");
    let jsonpath_query = JsonPathQuery::parse("$.a[0].c.d[2]")?;
    let json_file =
        fs::File::open("/home/mat/rsonpath/crates/rsonpath-test/tests/documents/toml/int_after_list_of_ints.json")?;
    let input = BufferedInput::new(json_file);
    let engine = RecursiveEngine::compile_query(&jsonpath_query)?;
    let result = engine.run::<_, IndexResult>(&input)?;
    assert_eq!(result.get(), vec![133usize,], "result != expected");
    Ok(())
}
#[test]
fn deeply_nested_single_atomic_int_after_a_list_of_ints_with_query_direct_path_to_the_third_element_on_the_list_with_mmap_input_and_count_result_using_main_engine(
) -> Result<(), Box<dyn Error>> {
    println ! ("on document int_after_list_of_ints.toml running the query $.a[0].c.d[2] (direct path to the third element on the list) with Input impl MmapInput and result mode CountResult");
    let jsonpath_query = JsonPathQuery::parse("$.a[0].c.d[2]")?;
    let json_file =
        fs::File::open("/home/mat/rsonpath/crates/rsonpath-test/tests/documents/toml/int_after_list_of_ints.json")?;
    let input = unsafe { MmapInput::map_file(&json_file)? };
    let engine = MainEngine::compile_query(&jsonpath_query)?;
    let result = engine.run::<_, CountResult>(&input)?;
    assert_eq!(result.get(), 1u64, "result != expected");
    Ok(())
}
#[test]
fn deeply_nested_single_atomic_int_after_a_list_of_ints_with_query_direct_path_to_the_third_element_on_the_list_with_mmap_input_and_count_result_using_recursive_engine(
) -> Result<(), Box<dyn Error>> {
    println ! ("on document int_after_list_of_ints.toml running the query $.a[0].c.d[2] (direct path to the third element on the list) with Input impl MmapInput and result mode CountResult");
    let jsonpath_query = JsonPathQuery::parse("$.a[0].c.d[2]")?;
    let json_file =
        fs::File::open("/home/mat/rsonpath/crates/rsonpath-test/tests/documents/toml/int_after_list_of_ints.json")?;
    let input = unsafe { MmapInput::map_file(&json_file)? };
    let engine = RecursiveEngine::compile_query(&jsonpath_query)?;
    let result = engine.run::<_, CountResult>(&input)?;
    assert_eq!(result.get(), 1u64, "result != expected");
    Ok(())
}
#[test]
fn deeply_nested_single_atomic_int_after_a_list_of_ints_with_query_direct_path_to_the_third_element_on_the_list_with_mmap_input_and_index_result_using_main_engine(
) -> Result<(), Box<dyn Error>> {
    println ! ("on document int_after_list_of_ints.toml running the query $.a[0].c.d[2] (direct path to the third element on the list) with Input impl MmapInput and result mode IndexResult");
    let jsonpath_query = JsonPathQuery::parse("$.a[0].c.d[2]")?;
    let json_file =
        fs::File::open("/home/mat/rsonpath/crates/rsonpath-test/tests/documents/toml/int_after_list_of_ints.json")?;
    let input = unsafe { MmapInput::map_file(&json_file)? };
    let engine = MainEngine::compile_query(&jsonpath_query)?;
    let result = engine.run::<_, IndexResult>(&input)?;
    assert_eq!(result.get(), vec![133usize,], "result != expected");
    Ok(())
}
#[test]
fn deeply_nested_single_atomic_int_after_a_list_of_ints_with_query_direct_path_to_the_third_element_on_the_list_with_mmap_input_and_index_result_using_recursive_engine(
) -> Result<(), Box<dyn Error>> {
    println ! ("on document int_after_list_of_ints.toml running the query $.a[0].c.d[2] (direct path to the third element on the list) with Input impl MmapInput and result mode IndexResult");
    let jsonpath_query = JsonPathQuery::parse("$.a[0].c.d[2]")?;
    let json_file =
        fs::File::open("/home/mat/rsonpath/crates/rsonpath-test/tests/documents/toml/int_after_list_of_ints.json")?;
    let input = unsafe { MmapInput::map_file(&json_file)? };
    let engine = RecursiveEngine::compile_query(&jsonpath_query)?;
    let result = engine.run::<_, IndexResult>(&input)?;
    assert_eq!(result.get(), vec![133usize,], "result != expected");
    Ok(())
}
#[test]
fn deeply_nested_single_atomic_int_after_a_list_of_ints_with_query_direct_path_to_the_third_element_on_the_list_with_owned_bytes_and_count_result_using_main_engine(
) -> Result<(), Box<dyn Error>> {
    println ! ("on document int_after_list_of_ints.toml running the query $.a[0].c.d[2] (direct path to the third element on the list) with Input impl OwnedBytes and result mode CountResult");
    let jsonpath_query = JsonPathQuery::parse("$.a[0].c.d[2]")?;
    let raw_json =
        fs::read_to_string("/home/mat/rsonpath/crates/rsonpath-test/tests/documents/toml/int_after_list_of_ints.json")?;
    let input = OwnedBytes::new(&raw_json.as_bytes())?;
    let engine = MainEngine::compile_query(&jsonpath_query)?;
    let result = engine.run::<_, CountResult>(&input)?;
    assert_eq!(result.get(), 1u64, "result != expected");
    Ok(())
}
#[test]
fn deeply_nested_single_atomic_int_after_a_list_of_ints_with_query_direct_path_to_the_third_element_on_the_list_with_owned_bytes_and_count_result_using_recursive_engine(
) -> Result<(), Box<dyn Error>> {
    println ! ("on document int_after_list_of_ints.toml running the query $.a[0].c.d[2] (direct path to the third element on the list) with Input impl OwnedBytes and result mode CountResult");
    let jsonpath_query = JsonPathQuery::parse("$.a[0].c.d[2]")?;
    let raw_json =
        fs::read_to_string("/home/mat/rsonpath/crates/rsonpath-test/tests/documents/toml/int_after_list_of_ints.json")?;
    let input = OwnedBytes::new(&raw_json.as_bytes())?;
    let engine = RecursiveEngine::compile_query(&jsonpath_query)?;
    let result = engine.run::<_, CountResult>(&input)?;
    assert_eq!(result.get(), 1u64, "result != expected");
    Ok(())
}
#[test]
fn deeply_nested_single_atomic_int_after_a_list_of_ints_with_query_direct_path_to_the_third_element_on_the_list_with_owned_bytes_and_index_result_using_main_engine(
) -> Result<(), Box<dyn Error>> {
    println ! ("on document int_after_list_of_ints.toml running the query $.a[0].c.d[2] (direct path to the third element on the list) with Input impl OwnedBytes and result mode IndexResult");
    let jsonpath_query = JsonPathQuery::parse("$.a[0].c.d[2]")?;
    let raw_json =
        fs::read_to_string("/home/mat/rsonpath/crates/rsonpath-test/tests/documents/toml/int_after_list_of_ints.json")?;
    let input = OwnedBytes::new(&raw_json.as_bytes())?;
    let engine = MainEngine::compile_query(&jsonpath_query)?;
    let result = engine.run::<_, IndexResult>(&input)?;
    assert_eq!(result.get(), vec![133usize,], "result != expected");
    Ok(())
}
#[test]
fn deeply_nested_single_atomic_int_after_a_list_of_ints_with_query_direct_path_to_the_third_element_on_the_list_with_owned_bytes_and_index_result_using_recursive_engine(
) -> Result<(), Box<dyn Error>> {
    println ! ("on document int_after_list_of_ints.toml running the query $.a[0].c.d[2] (direct path to the third element on the list) with Input impl OwnedBytes and result mode IndexResult");
    let jsonpath_query = JsonPathQuery::parse("$.a[0].c.d[2]")?;
    let raw_json =
        fs::read_to_string("/home/mat/rsonpath/crates/rsonpath-test/tests/documents/toml/int_after_list_of_ints.json")?;
    let input = OwnedBytes::new(&raw_json.as_bytes())?;
    let engine = RecursiveEngine::compile_query(&jsonpath_query)?;
    let result = engine.run::<_, IndexResult>(&input)?;
    assert_eq!(result.get(), vec![133usize,], "result != expected");
    Ok(())
}
#[test]
fn deeply_nested_single_atomic_int_after_a_list_of_ints_with_query_driect_path_to_the_fourth_element_on_the_list_which_does_not_exist_with_buffered_input_and_count_result_using_main_engine(
) -> Result<(), Box<dyn Error>> {
    println ! ("on document int_after_list_of_ints.toml running the query $.a[0].c.d[3] (driect path to the fourth element on the list (which does not exist)) with Input impl BufferedInput and result mode CountResult");
    let jsonpath_query = JsonPathQuery::parse("$.a[0].c.d[3]")?;
    let json_file =
        fs::File::open("/home/mat/rsonpath/crates/rsonpath-test/tests/documents/toml/int_after_list_of_ints.json")?;
    let input = BufferedInput::new(json_file);
    let engine = MainEngine::compile_query(&jsonpath_query)?;
    let result = engine.run::<_, CountResult>(&input)?;
    assert_eq!(result.get(), 0u64, "result != expected");
    Ok(())
}
#[test]
fn deeply_nested_single_atomic_int_after_a_list_of_ints_with_query_driect_path_to_the_fourth_element_on_the_list_which_does_not_exist_with_buffered_input_and_count_result_using_recursive_engine(
) -> Result<(), Box<dyn Error>> {
    println ! ("on document int_after_list_of_ints.toml running the query $.a[0].c.d[3] (driect path to the fourth element on the list (which does not exist)) with Input impl BufferedInput and result mode CountResult");
    let jsonpath_query = JsonPathQuery::parse("$.a[0].c.d[3]")?;
    let json_file =
        fs::File::open("/home/mat/rsonpath/crates/rsonpath-test/tests/documents/toml/int_after_list_of_ints.json")?;
    let input = BufferedInput::new(json_file);
    let engine = RecursiveEngine::compile_query(&jsonpath_query)?;
    let result = engine.run::<_, CountResult>(&input)?;
    assert_eq!(result.get(), 0u64, "result != expected");
    Ok(())
}
#[test]
fn deeply_nested_single_atomic_int_after_a_list_of_ints_with_query_driect_path_to_the_fourth_element_on_the_list_which_does_not_exist_with_buffered_input_and_index_result_using_main_engine(
) -> Result<(), Box<dyn Error>> {
    println ! ("on document int_after_list_of_ints.toml running the query $.a[0].c.d[3] (driect path to the fourth element on the list (which does not exist)) with Input impl BufferedInput and result mode IndexResult");
    let jsonpath_query = JsonPathQuery::parse("$.a[0].c.d[3]")?;
    let json_file =
        fs::File::open("/home/mat/rsonpath/crates/rsonpath-test/tests/documents/toml/int_after_list_of_ints.json")?;
    let input = BufferedInput::new(json_file);
    let engine = MainEngine::compile_query(&jsonpath_query)?;
    let result = engine.run::<_, IndexResult>(&input)?;
    assert_eq!(result.get(), vec![], "result != expected");
    Ok(())
}
#[test]
fn deeply_nested_single_atomic_int_after_a_list_of_ints_with_query_driect_path_to_the_fourth_element_on_the_list_which_does_not_exist_with_buffered_input_and_index_result_using_recursive_engine(
) -> Result<(), Box<dyn Error>> {
    println ! ("on document int_after_list_of_ints.toml running the query $.a[0].c.d[3] (driect path to the fourth element on the list (which does not exist)) with Input impl BufferedInput and result mode IndexResult");
    let jsonpath_query = JsonPathQuery::parse("$.a[0].c.d[3]")?;
    let json_file =
        fs::File::open("/home/mat/rsonpath/crates/rsonpath-test/tests/documents/toml/int_after_list_of_ints.json")?;
    let input = BufferedInput::new(json_file);
    let engine = RecursiveEngine::compile_query(&jsonpath_query)?;
    let result = engine.run::<_, IndexResult>(&input)?;
    assert_eq!(result.get(), vec![], "result != expected");
    Ok(())
}
#[test]
fn deeply_nested_single_atomic_int_after_a_list_of_ints_with_query_driect_path_to_the_fourth_element_on_the_list_which_does_not_exist_with_mmap_input_and_count_result_using_main_engine(
) -> Result<(), Box<dyn Error>> {
    println ! ("on document int_after_list_of_ints.toml running the query $.a[0].c.d[3] (driect path to the fourth element on the list (which does not exist)) with Input impl MmapInput and result mode CountResult");
    let jsonpath_query = JsonPathQuery::parse("$.a[0].c.d[3]")?;
    let json_file =
        fs::File::open("/home/mat/rsonpath/crates/rsonpath-test/tests/documents/toml/int_after_list_of_ints.json")?;
    let input = unsafe { MmapInput::map_file(&json_file)? };
    let engine = MainEngine::compile_query(&jsonpath_query)?;
    let result = engine.run::<_, CountResult>(&input)?;
    assert_eq!(result.get(), 0u64, "result != expected");
    Ok(())
}
#[test]
fn deeply_nested_single_atomic_int_after_a_list_of_ints_with_query_driect_path_to_the_fourth_element_on_the_list_which_does_not_exist_with_mmap_input_and_count_result_using_recursive_engine(
) -> Result<(), Box<dyn Error>> {
    println ! ("on document int_after_list_of_ints.toml running the query $.a[0].c.d[3] (driect path to the fourth element on the list (which does not exist)) with Input impl MmapInput and result mode CountResult");
    let jsonpath_query = JsonPathQuery::parse("$.a[0].c.d[3]")?;
    let json_file =
        fs::File::open("/home/mat/rsonpath/crates/rsonpath-test/tests/documents/toml/int_after_list_of_ints.json")?;
    let input = unsafe { MmapInput::map_file(&json_file)? };
    let engine = RecursiveEngine::compile_query(&jsonpath_query)?;
    let result = engine.run::<_, CountResult>(&input)?;
    assert_eq!(result.get(), 0u64, "result != expected");
    Ok(())
}
#[test]
fn deeply_nested_single_atomic_int_after_a_list_of_ints_with_query_driect_path_to_the_fourth_element_on_the_list_which_does_not_exist_with_mmap_input_and_index_result_using_main_engine(
) -> Result<(), Box<dyn Error>> {
    println ! ("on document int_after_list_of_ints.toml running the query $.a[0].c.d[3] (driect path to the fourth element on the list (which does not exist)) with Input impl MmapInput and result mode IndexResult");
    let jsonpath_query = JsonPathQuery::parse("$.a[0].c.d[3]")?;
    let json_file =
        fs::File::open("/home/mat/rsonpath/crates/rsonpath-test/tests/documents/toml/int_after_list_of_ints.json")?;
    let input = unsafe { MmapInput::map_file(&json_file)? };
    let engine = MainEngine::compile_query(&jsonpath_query)?;
    let result = engine.run::<_, IndexResult>(&input)?;
    assert_eq!(result.get(), vec![], "result != expected");
    Ok(())
}
#[test]
fn deeply_nested_single_atomic_int_after_a_list_of_ints_with_query_driect_path_to_the_fourth_element_on_the_list_which_does_not_exist_with_mmap_input_and_index_result_using_recursive_engine(
) -> Result<(), Box<dyn Error>> {
    println ! ("on document int_after_list_of_ints.toml running the query $.a[0].c.d[3] (driect path to the fourth element on the list (which does not exist)) with Input impl MmapInput and result mode IndexResult");
    let jsonpath_query = JsonPathQuery::parse("$.a[0].c.d[3]")?;
    let json_file =
        fs::File::open("/home/mat/rsonpath/crates/rsonpath-test/tests/documents/toml/int_after_list_of_ints.json")?;
    let input = unsafe { MmapInput::map_file(&json_file)? };
    let engine = RecursiveEngine::compile_query(&jsonpath_query)?;
    let result = engine.run::<_, IndexResult>(&input)?;
    assert_eq!(result.get(), vec![], "result != expected");
    Ok(())
}
#[test]
fn deeply_nested_single_atomic_int_after_a_list_of_ints_with_query_driect_path_to_the_fourth_element_on_the_list_which_does_not_exist_with_owned_bytes_and_count_result_using_main_engine(
) -> Result<(), Box<dyn Error>> {
    println ! ("on document int_after_list_of_ints.toml running the query $.a[0].c.d[3] (driect path to the fourth element on the list (which does not exist)) with Input impl OwnedBytes and result mode CountResult");
    let jsonpath_query = JsonPathQuery::parse("$.a[0].c.d[3]")?;
    let raw_json =
        fs::read_to_string("/home/mat/rsonpath/crates/rsonpath-test/tests/documents/toml/int_after_list_of_ints.json")?;
    let input = OwnedBytes::new(&raw_json.as_bytes())?;
    let engine = MainEngine::compile_query(&jsonpath_query)?;
    let result = engine.run::<_, CountResult>(&input)?;
    assert_eq!(result.get(), 0u64, "result != expected");
    Ok(())
}
#[test]
fn deeply_nested_single_atomic_int_after_a_list_of_ints_with_query_driect_path_to_the_fourth_element_on_the_list_which_does_not_exist_with_owned_bytes_and_count_result_using_recursive_engine(
) -> Result<(), Box<dyn Error>> {
    println ! ("on document int_after_list_of_ints.toml running the query $.a[0].c.d[3] (driect path to the fourth element on the list (which does not exist)) with Input impl OwnedBytes and result mode CountResult");
    let jsonpath_query = JsonPathQuery::parse("$.a[0].c.d[3]")?;
    let raw_json =
        fs::read_to_string("/home/mat/rsonpath/crates/rsonpath-test/tests/documents/toml/int_after_list_of_ints.json")?;
    let input = OwnedBytes::new(&raw_json.as_bytes())?;
    let engine = RecursiveEngine::compile_query(&jsonpath_query)?;
    let result = engine.run::<_, CountResult>(&input)?;
    assert_eq!(result.get(), 0u64, "result != expected");
    Ok(())
}
#[test]
fn deeply_nested_single_atomic_int_after_a_list_of_ints_with_query_driect_path_to_the_fourth_element_on_the_list_which_does_not_exist_with_owned_bytes_and_index_result_using_main_engine(
) -> Result<(), Box<dyn Error>> {
    println ! ("on document int_after_list_of_ints.toml running the query $.a[0].c.d[3] (driect path to the fourth element on the list (which does not exist)) with Input impl OwnedBytes and result mode IndexResult");
    let jsonpath_query = JsonPathQuery::parse("$.a[0].c.d[3]")?;
    let raw_json =
        fs::read_to_string("/home/mat/rsonpath/crates/rsonpath-test/tests/documents/toml/int_after_list_of_ints.json")?;
    let input = OwnedBytes::new(&raw_json.as_bytes())?;
    let engine = MainEngine::compile_query(&jsonpath_query)?;
    let result = engine.run::<_, IndexResult>(&input)?;
    assert_eq!(result.get(), vec![], "result != expected");
    Ok(())
}
#[test]
fn deeply_nested_single_atomic_int_after_a_list_of_ints_with_query_driect_path_to_the_fourth_element_on_the_list_which_does_not_exist_with_owned_bytes_and_index_result_using_recursive_engine(
) -> Result<(), Box<dyn Error>> {
    println ! ("on document int_after_list_of_ints.toml running the query $.a[0].c.d[3] (driect path to the fourth element on the list (which does not exist)) with Input impl OwnedBytes and result mode IndexResult");
    let jsonpath_query = JsonPathQuery::parse("$.a[0].c.d[3]")?;
    let raw_json =
        fs::read_to_string("/home/mat/rsonpath/crates/rsonpath-test/tests/documents/toml/int_after_list_of_ints.json")?;
    let input = OwnedBytes::new(&raw_json.as_bytes())?;
    let engine = RecursiveEngine::compile_query(&jsonpath_query)?;
    let result = engine.run::<_, IndexResult>(&input)?;
    assert_eq!(result.get(), vec![], "result != expected");
    Ok(())
}
#[test]
fn deeply_nested_single_atomic_int_after_a_list_of_ints_with_query_look_for_the_single_int_with_descendant_search_with_buffered_input_and_count_result_using_main_engine(
) -> Result<(), Box<dyn Error>> {
    println ! ("on document int_after_list_of_ints.toml running the query $.a..b (look for the single int with descendant search) with Input impl BufferedInput and result mode CountResult");
    let jsonpath_query = JsonPathQuery::parse("$.a..b")?;
    let json_file =
        fs::File::open("/home/mat/rsonpath/crates/rsonpath-test/tests/documents/toml/int_after_list_of_ints.json")?;
    let input = BufferedInput::new(json_file);
    let engine = MainEngine::compile_query(&jsonpath_query)?;
    let result = engine.run::<_, CountResult>(&input)?;
    assert_eq!(result.get(), 1u64, "result != expected");
    Ok(())
}
#[test]
fn deeply_nested_single_atomic_int_after_a_list_of_ints_with_query_look_for_the_single_int_with_descendant_search_with_buffered_input_and_count_result_using_recursive_engine(
) -> Result<(), Box<dyn Error>> {
    println ! ("on document int_after_list_of_ints.toml running the query $.a..b (look for the single int with descendant search) with Input impl BufferedInput and result mode CountResult");
    let jsonpath_query = JsonPathQuery::parse("$.a..b")?;
    let json_file =
        fs::File::open("/home/mat/rsonpath/crates/rsonpath-test/tests/documents/toml/int_after_list_of_ints.json")?;
    let input = BufferedInput::new(json_file);
    let engine = RecursiveEngine::compile_query(&jsonpath_query)?;
    let result = engine.run::<_, CountResult>(&input)?;
    assert_eq!(result.get(), 1u64, "result != expected");
    Ok(())
}
#[test]
fn deeply_nested_single_atomic_int_after_a_list_of_ints_with_query_look_for_the_single_int_with_descendant_search_with_buffered_input_and_index_result_using_main_engine(
) -> Result<(), Box<dyn Error>> {
    println ! ("on document int_after_list_of_ints.toml running the query $.a..b (look for the single int with descendant search) with Input impl BufferedInput and result mode IndexResult");
    let jsonpath_query = JsonPathQuery::parse("$.a..b")?;
    let json_file =
        fs::File::open("/home/mat/rsonpath/crates/rsonpath-test/tests/documents/toml/int_after_list_of_ints.json")?;
    let input = BufferedInput::new(json_file);
    let engine = MainEngine::compile_query(&jsonpath_query)?;
    let result = engine.run::<_, IndexResult>(&input)?;
    assert_eq!(result.get(), vec![176usize,], "result != expected");
    Ok(())
}
#[test]
fn deeply_nested_single_atomic_int_after_a_list_of_ints_with_query_look_for_the_single_int_with_descendant_search_with_buffered_input_and_index_result_using_recursive_engine(
) -> Result<(), Box<dyn Error>> {
    println ! ("on document int_after_list_of_ints.toml running the query $.a..b (look for the single int with descendant search) with Input impl BufferedInput and result mode IndexResult");
    let jsonpath_query = JsonPathQuery::parse("$.a..b")?;
    let json_file =
        fs::File::open("/home/mat/rsonpath/crates/rsonpath-test/tests/documents/toml/int_after_list_of_ints.json")?;
    let input = BufferedInput::new(json_file);
    let engine = RecursiveEngine::compile_query(&jsonpath_query)?;
    let result = engine.run::<_, IndexResult>(&input)?;
    assert_eq!(result.get(), vec![176usize,], "result != expected");
    Ok(())
}
#[test]
fn deeply_nested_single_atomic_int_after_a_list_of_ints_with_query_look_for_the_single_int_with_descendant_search_with_mmap_input_and_count_result_using_main_engine(
) -> Result<(), Box<dyn Error>> {
    println ! ("on document int_after_list_of_ints.toml running the query $.a..b (look for the single int with descendant search) with Input impl MmapInput and result mode CountResult");
    let jsonpath_query = JsonPathQuery::parse("$.a..b")?;
    let json_file =
        fs::File::open("/home/mat/rsonpath/crates/rsonpath-test/tests/documents/toml/int_after_list_of_ints.json")?;
    let input = unsafe { MmapInput::map_file(&json_file)? };
    let engine = MainEngine::compile_query(&jsonpath_query)?;
    let result = engine.run::<_, CountResult>(&input)?;
    assert_eq!(result.get(), 1u64, "result != expected");
    Ok(())
}
#[test]
fn deeply_nested_single_atomic_int_after_a_list_of_ints_with_query_look_for_the_single_int_with_descendant_search_with_mmap_input_and_count_result_using_recursive_engine(
) -> Result<(), Box<dyn Error>> {
    println ! ("on document int_after_list_of_ints.toml running the query $.a..b (look for the single int with descendant search) with Input impl MmapInput and result mode CountResult");
    let jsonpath_query = JsonPathQuery::parse("$.a..b")?;
    let json_file =
        fs::File::open("/home/mat/rsonpath/crates/rsonpath-test/tests/documents/toml/int_after_list_of_ints.json")?;
    let input = unsafe { MmapInput::map_file(&json_file)? };
    let engine = RecursiveEngine::compile_query(&jsonpath_query)?;
    let result = engine.run::<_, CountResult>(&input)?;
    assert_eq!(result.get(), 1u64, "result != expected");
    Ok(())
}
#[test]
fn deeply_nested_single_atomic_int_after_a_list_of_ints_with_query_look_for_the_single_int_with_descendant_search_with_mmap_input_and_index_result_using_main_engine(
) -> Result<(), Box<dyn Error>> {
    println ! ("on document int_after_list_of_ints.toml running the query $.a..b (look for the single int with descendant search) with Input impl MmapInput and result mode IndexResult");
    let jsonpath_query = JsonPathQuery::parse("$.a..b")?;
    let json_file =
        fs::File::open("/home/mat/rsonpath/crates/rsonpath-test/tests/documents/toml/int_after_list_of_ints.json")?;
    let input = unsafe { MmapInput::map_file(&json_file)? };
    let engine = MainEngine::compile_query(&jsonpath_query)?;
    let result = engine.run::<_, IndexResult>(&input)?;
    assert_eq!(result.get(), vec![176usize,], "result != expected");
    Ok(())
}
#[test]
fn deeply_nested_single_atomic_int_after_a_list_of_ints_with_query_look_for_the_single_int_with_descendant_search_with_mmap_input_and_index_result_using_recursive_engine(
) -> Result<(), Box<dyn Error>> {
    println ! ("on document int_after_list_of_ints.toml running the query $.a..b (look for the single int with descendant search) with Input impl MmapInput and result mode IndexResult");
    let jsonpath_query = JsonPathQuery::parse("$.a..b")?;
    let json_file =
        fs::File::open("/home/mat/rsonpath/crates/rsonpath-test/tests/documents/toml/int_after_list_of_ints.json")?;
    let input = unsafe { MmapInput::map_file(&json_file)? };
    let engine = RecursiveEngine::compile_query(&jsonpath_query)?;
    let result = engine.run::<_, IndexResult>(&input)?;
    assert_eq!(result.get(), vec![176usize,], "result != expected");
    Ok(())
}
#[test]
fn deeply_nested_single_atomic_int_after_a_list_of_ints_with_query_look_for_the_single_int_with_descendant_search_with_owned_bytes_and_count_result_using_main_engine(
) -> Result<(), Box<dyn Error>> {
    println ! ("on document int_after_list_of_ints.toml running the query $.a..b (look for the single int with descendant search) with Input impl OwnedBytes and result mode CountResult");
    let jsonpath_query = JsonPathQuery::parse("$.a..b")?;
    let raw_json =
        fs::read_to_string("/home/mat/rsonpath/crates/rsonpath-test/tests/documents/toml/int_after_list_of_ints.json")?;
    let input = OwnedBytes::new(&raw_json.as_bytes())?;
    let engine = MainEngine::compile_query(&jsonpath_query)?;
    let result = engine.run::<_, CountResult>(&input)?;
    assert_eq!(result.get(), 1u64, "result != expected");
    Ok(())
}
#[test]
fn deeply_nested_single_atomic_int_after_a_list_of_ints_with_query_look_for_the_single_int_with_descendant_search_with_owned_bytes_and_count_result_using_recursive_engine(
) -> Result<(), Box<dyn Error>> {
    println ! ("on document int_after_list_of_ints.toml running the query $.a..b (look for the single int with descendant search) with Input impl OwnedBytes and result mode CountResult");
    let jsonpath_query = JsonPathQuery::parse("$.a..b")?;
    let raw_json =
        fs::read_to_string("/home/mat/rsonpath/crates/rsonpath-test/tests/documents/toml/int_after_list_of_ints.json")?;
    let input = OwnedBytes::new(&raw_json.as_bytes())?;
    let engine = RecursiveEngine::compile_query(&jsonpath_query)?;
    let result = engine.run::<_, CountResult>(&input)?;
    assert_eq!(result.get(), 1u64, "result != expected");
    Ok(())
}
#[test]
fn deeply_nested_single_atomic_int_after_a_list_of_ints_with_query_look_for_the_single_int_with_descendant_search_with_owned_bytes_and_index_result_using_main_engine(
) -> Result<(), Box<dyn Error>> {
    println ! ("on document int_after_list_of_ints.toml running the query $.a..b (look for the single int with descendant search) with Input impl OwnedBytes and result mode IndexResult");
    let jsonpath_query = JsonPathQuery::parse("$.a..b")?;
    let raw_json =
        fs::read_to_string("/home/mat/rsonpath/crates/rsonpath-test/tests/documents/toml/int_after_list_of_ints.json")?;
    let input = OwnedBytes::new(&raw_json.as_bytes())?;
    let engine = MainEngine::compile_query(&jsonpath_query)?;
    let result = engine.run::<_, IndexResult>(&input)?;
    assert_eq!(result.get(), vec![176usize,], "result != expected");
    Ok(())
}
#[test]
fn deeply_nested_single_atomic_int_after_a_list_of_ints_with_query_look_for_the_single_int_with_descendant_search_with_owned_bytes_and_index_result_using_recursive_engine(
) -> Result<(), Box<dyn Error>> {
    println ! ("on document int_after_list_of_ints.toml running the query $.a..b (look for the single int with descendant search) with Input impl OwnedBytes and result mode IndexResult");
    let jsonpath_query = JsonPathQuery::parse("$.a..b")?;
    let raw_json =
        fs::read_to_string("/home/mat/rsonpath/crates/rsonpath-test/tests/documents/toml/int_after_list_of_ints.json")?;
    let input = OwnedBytes::new(&raw_json.as_bytes())?;
    let engine = RecursiveEngine::compile_query(&jsonpath_query)?;
    let result = engine.run::<_, IndexResult>(&input)?;
    assert_eq!(result.get(), vec![176usize,], "result != expected");
    Ok(())
}
#[test]
fn deeply_nested_single_atomic_int_after_a_list_of_ints_with_query_select_the_third_element_from_each_list_with_buffered_input_and_count_result_using_main_engine(
) -> Result<(), Box<dyn Error>> {
    println ! ("on document int_after_list_of_ints.toml running the query $..*[2] (select the third element from each list) with Input impl BufferedInput and result mode CountResult");
    let jsonpath_query = JsonPathQuery::parse("$..*[2]")?;
    let json_file =
        fs::File::open("/home/mat/rsonpath/crates/rsonpath-test/tests/documents/toml/int_after_list_of_ints.json")?;
    let input = BufferedInput::new(json_file);
    let engine = MainEngine::compile_query(&jsonpath_query)?;
    let result = engine.run::<_, CountResult>(&input)?;
    assert_eq!(result.get(), 1u64, "result != expected");
    Ok(())
}
#[test]
fn deeply_nested_single_atomic_int_after_a_list_of_ints_with_query_select_the_third_element_from_each_list_with_buffered_input_and_count_result_using_recursive_engine(
) -> Result<(), Box<dyn Error>> {
    println ! ("on document int_after_list_of_ints.toml running the query $..*[2] (select the third element from each list) with Input impl BufferedInput and result mode CountResult");
    let jsonpath_query = JsonPathQuery::parse("$..*[2]")?;
    let json_file =
        fs::File::open("/home/mat/rsonpath/crates/rsonpath-test/tests/documents/toml/int_after_list_of_ints.json")?;
    let input = BufferedInput::new(json_file);
    let engine = RecursiveEngine::compile_query(&jsonpath_query)?;
    let result = engine.run::<_, CountResult>(&input)?;
    assert_eq!(result.get(), 1u64, "result != expected");
    Ok(())
}
#[test]
fn deeply_nested_single_atomic_int_after_a_list_of_ints_with_query_select_the_third_element_from_each_list_with_buffered_input_and_index_result_using_main_engine(
) -> Result<(), Box<dyn Error>> {
    println ! ("on document int_after_list_of_ints.toml running the query $..*[2] (select the third element from each list) with Input impl BufferedInput and result mode IndexResult");
    let jsonpath_query = JsonPathQuery::parse("$..*[2]")?;
    let json_file =
        fs::File::open("/home/mat/rsonpath/crates/rsonpath-test/tests/documents/toml/int_after_list_of_ints.json")?;
    let input = BufferedInput::new(json_file);
    let engine = MainEngine::compile_query(&jsonpath_query)?;
    let result = engine.run::<_, IndexResult>(&input)?;
    assert_eq!(result.get(), vec![133usize,], "result != expected");
    Ok(())
}
#[test]
fn deeply_nested_single_atomic_int_after_a_list_of_ints_with_query_select_the_third_element_from_each_list_with_buffered_input_and_index_result_using_recursive_engine(
) -> Result<(), Box<dyn Error>> {
    println ! ("on document int_after_list_of_ints.toml running the query $..*[2] (select the third element from each list) with Input impl BufferedInput and result mode IndexResult");
    let jsonpath_query = JsonPathQuery::parse("$..*[2]")?;
    let json_file =
        fs::File::open("/home/mat/rsonpath/crates/rsonpath-test/tests/documents/toml/int_after_list_of_ints.json")?;
    let input = BufferedInput::new(json_file);
    let engine = RecursiveEngine::compile_query(&jsonpath_query)?;
    let result = engine.run::<_, IndexResult>(&input)?;
    assert_eq!(result.get(), vec![133usize,], "result != expected");
    Ok(())
}
#[test]
fn deeply_nested_single_atomic_int_after_a_list_of_ints_with_query_select_the_third_element_from_each_list_with_mmap_input_and_count_result_using_main_engine(
) -> Result<(), Box<dyn Error>> {
    println ! ("on document int_after_list_of_ints.toml running the query $..*[2] (select the third element from each list) with Input impl MmapInput and result mode CountResult");
    let jsonpath_query = JsonPathQuery::parse("$..*[2]")?;
    let json_file =
        fs::File::open("/home/mat/rsonpath/crates/rsonpath-test/tests/documents/toml/int_after_list_of_ints.json")?;
    let input = unsafe { MmapInput::map_file(&json_file)? };
    let engine = MainEngine::compile_query(&jsonpath_query)?;
    let result = engine.run::<_, CountResult>(&input)?;
    assert_eq!(result.get(), 1u64, "result != expected");
    Ok(())
}
#[test]
fn deeply_nested_single_atomic_int_after_a_list_of_ints_with_query_select_the_third_element_from_each_list_with_mmap_input_and_count_result_using_recursive_engine(
) -> Result<(), Box<dyn Error>> {
    println ! ("on document int_after_list_of_ints.toml running the query $..*[2] (select the third element from each list) with Input impl MmapInput and result mode CountResult");
    let jsonpath_query = JsonPathQuery::parse("$..*[2]")?;
    let json_file =
        fs::File::open("/home/mat/rsonpath/crates/rsonpath-test/tests/documents/toml/int_after_list_of_ints.json")?;
    let input = unsafe { MmapInput::map_file(&json_file)? };
    let engine = RecursiveEngine::compile_query(&jsonpath_query)?;
    let result = engine.run::<_, CountResult>(&input)?;
    assert_eq!(result.get(), 1u64, "result != expected");
    Ok(())
}
#[test]
fn deeply_nested_single_atomic_int_after_a_list_of_ints_with_query_select_the_third_element_from_each_list_with_mmap_input_and_index_result_using_main_engine(
) -> Result<(), Box<dyn Error>> {
    println ! ("on document int_after_list_of_ints.toml running the query $..*[2] (select the third element from each list) with Input impl MmapInput and result mode IndexResult");
    let jsonpath_query = JsonPathQuery::parse("$..*[2]")?;
    let json_file =
        fs::File::open("/home/mat/rsonpath/crates/rsonpath-test/tests/documents/toml/int_after_list_of_ints.json")?;
    let input = unsafe { MmapInput::map_file(&json_file)? };
    let engine = MainEngine::compile_query(&jsonpath_query)?;
    let result = engine.run::<_, IndexResult>(&input)?;
    assert_eq!(result.get(), vec![133usize,], "result != expected");
    Ok(())
}
#[test]
fn deeply_nested_single_atomic_int_after_a_list_of_ints_with_query_select_the_third_element_from_each_list_with_mmap_input_and_index_result_using_recursive_engine(
) -> Result<(), Box<dyn Error>> {
    println ! ("on document int_after_list_of_ints.toml running the query $..*[2] (select the third element from each list) with Input impl MmapInput and result mode IndexResult");
    let jsonpath_query = JsonPathQuery::parse("$..*[2]")?;
    let json_file =
        fs::File::open("/home/mat/rsonpath/crates/rsonpath-test/tests/documents/toml/int_after_list_of_ints.json")?;
    let input = unsafe { MmapInput::map_file(&json_file)? };
    let engine = RecursiveEngine::compile_query(&jsonpath_query)?;
    let result = engine.run::<_, IndexResult>(&input)?;
    assert_eq!(result.get(), vec![133usize,], "result != expected");
    Ok(())
}
#[test]
fn deeply_nested_single_atomic_int_after_a_list_of_ints_with_query_select_the_third_element_from_each_list_with_owned_bytes_and_count_result_using_main_engine(
) -> Result<(), Box<dyn Error>> {
    println ! ("on document int_after_list_of_ints.toml running the query $..*[2] (select the third element from each list) with Input impl OwnedBytes and result mode CountResult");
    let jsonpath_query = JsonPathQuery::parse("$..*[2]")?;
    let raw_json =
        fs::read_to_string("/home/mat/rsonpath/crates/rsonpath-test/tests/documents/toml/int_after_list_of_ints.json")?;
    let input = OwnedBytes::new(&raw_json.as_bytes())?;
    let engine = MainEngine::compile_query(&jsonpath_query)?;
    let result = engine.run::<_, CountResult>(&input)?;
    assert_eq!(result.get(), 1u64, "result != expected");
    Ok(())
}
#[test]
fn deeply_nested_single_atomic_int_after_a_list_of_ints_with_query_select_the_third_element_from_each_list_with_owned_bytes_and_count_result_using_recursive_engine(
) -> Result<(), Box<dyn Error>> {
    println ! ("on document int_after_list_of_ints.toml running the query $..*[2] (select the third element from each list) with Input impl OwnedBytes and result mode CountResult");
    let jsonpath_query = JsonPathQuery::parse("$..*[2]")?;
    let raw_json =
        fs::read_to_string("/home/mat/rsonpath/crates/rsonpath-test/tests/documents/toml/int_after_list_of_ints.json")?;
    let input = OwnedBytes::new(&raw_json.as_bytes())?;
    let engine = RecursiveEngine::compile_query(&jsonpath_query)?;
    let result = engine.run::<_, CountResult>(&input)?;
    assert_eq!(result.get(), 1u64, "result != expected");
    Ok(())
}
#[test]
fn deeply_nested_single_atomic_int_after_a_list_of_ints_with_query_select_the_third_element_from_each_list_with_owned_bytes_and_index_result_using_main_engine(
) -> Result<(), Box<dyn Error>> {
    println ! ("on document int_after_list_of_ints.toml running the query $..*[2] (select the third element from each list) with Input impl OwnedBytes and result mode IndexResult");
    let jsonpath_query = JsonPathQuery::parse("$..*[2]")?;
    let raw_json =
        fs::read_to_string("/home/mat/rsonpath/crates/rsonpath-test/tests/documents/toml/int_after_list_of_ints.json")?;
    let input = OwnedBytes::new(&raw_json.as_bytes())?;
    let engine = MainEngine::compile_query(&jsonpath_query)?;
    let result = engine.run::<_, IndexResult>(&input)?;
    assert_eq!(result.get(), vec![133usize,], "result != expected");
    Ok(())
}
#[test]
fn deeply_nested_single_atomic_int_after_a_list_of_ints_with_query_select_the_third_element_from_each_list_with_owned_bytes_and_index_result_using_recursive_engine(
) -> Result<(), Box<dyn Error>> {
    println ! ("on document int_after_list_of_ints.toml running the query $..*[2] (select the third element from each list) with Input impl OwnedBytes and result mode IndexResult");
    let jsonpath_query = JsonPathQuery::parse("$..*[2]")?;
    let raw_json =
        fs::read_to_string("/home/mat/rsonpath/crates/rsonpath-test/tests/documents/toml/int_after_list_of_ints.json")?;
    let input = OwnedBytes::new(&raw_json.as_bytes())?;
    let engine = RecursiveEngine::compile_query(&jsonpath_query)?;
    let result = engine.run::<_, IndexResult>(&input)?;
    assert_eq!(result.get(), vec![133usize,], "result != expected");
    Ok(())
}
#[test]
fn deeply_nested_single_atomic_int_after_a_list_of_ints_with_query_take_the_subdocumented_from_a_using_an_index_selector_with_buffered_input_and_count_result_using_main_engine(
) -> Result<(), Box<dyn Error>> {
    println ! ("on document int_after_list_of_ints.toml running the query $.a[0] (take the subdocumented from 'a' using an index selector) with Input impl BufferedInput and result mode CountResult");
    let jsonpath_query = JsonPathQuery::parse("$.a[0]")?;
    let json_file =
        fs::File::open("/home/mat/rsonpath/crates/rsonpath-test/tests/documents/toml/int_after_list_of_ints.json")?;
    let input = BufferedInput::new(json_file);
    let engine = MainEngine::compile_query(&jsonpath_query)?;
    let result = engine.run::<_, CountResult>(&input)?;
    assert_eq!(result.get(), 1u64, "result != expected");
    Ok(())
}
#[test]
fn deeply_nested_single_atomic_int_after_a_list_of_ints_with_query_take_the_subdocumented_from_a_using_an_index_selector_with_buffered_input_and_count_result_using_recursive_engine(
) -> Result<(), Box<dyn Error>> {
    println ! ("on document int_after_list_of_ints.toml running the query $.a[0] (take the subdocumented from 'a' using an index selector) with Input impl BufferedInput and result mode CountResult");
    let jsonpath_query = JsonPathQuery::parse("$.a[0]")?;
    let json_file =
        fs::File::open("/home/mat/rsonpath/crates/rsonpath-test/tests/documents/toml/int_after_list_of_ints.json")?;
    let input = BufferedInput::new(json_file);
    let engine = RecursiveEngine::compile_query(&jsonpath_query)?;
    let result = engine.run::<_, CountResult>(&input)?;
    assert_eq!(result.get(), 1u64, "result != expected");
    Ok(())
}
#[test]
fn deeply_nested_single_atomic_int_after_a_list_of_ints_with_query_take_the_subdocumented_from_a_using_an_index_selector_with_buffered_input_and_index_result_using_main_engine(
) -> Result<(), Box<dyn Error>> {
    println ! ("on document int_after_list_of_ints.toml running the query $.a[0] (take the subdocumented from 'a' using an index selector) with Input impl BufferedInput and result mode IndexResult");
    let jsonpath_query = JsonPathQuery::parse("$.a[0]")?;
    let json_file =
        fs::File::open("/home/mat/rsonpath/crates/rsonpath-test/tests/documents/toml/int_after_list_of_ints.json")?;
    let input = BufferedInput::new(json_file);
    let engine = MainEngine::compile_query(&jsonpath_query)?;
    let result = engine.run::<_, IndexResult>(&input)?;
    assert_eq!(result.get(), vec![21usize,], "result != expected");
    Ok(())
}
#[test]
fn deeply_nested_single_atomic_int_after_a_list_of_ints_with_query_take_the_subdocumented_from_a_using_an_index_selector_with_buffered_input_and_index_result_using_recursive_engine(
) -> Result<(), Box<dyn Error>> {
    println ! ("on document int_after_list_of_ints.toml running the query $.a[0] (take the subdocumented from 'a' using an index selector) with Input impl BufferedInput and result mode IndexResult");
    let jsonpath_query = JsonPathQuery::parse("$.a[0]")?;
    let json_file =
        fs::File::open("/home/mat/rsonpath/crates/rsonpath-test/tests/documents/toml/int_after_list_of_ints.json")?;
    let input = BufferedInput::new(json_file);
    let engine = RecursiveEngine::compile_query(&jsonpath_query)?;
    let result = engine.run::<_, IndexResult>(&input)?;
    assert_eq!(result.get(), vec![21usize,], "result != expected");
    Ok(())
}
#[test]
fn deeply_nested_single_atomic_int_after_a_list_of_ints_with_query_take_the_subdocumented_from_a_using_an_index_selector_with_mmap_input_and_count_result_using_main_engine(
) -> Result<(), Box<dyn Error>> {
    println ! ("on document int_after_list_of_ints.toml running the query $.a[0] (take the subdocumented from 'a' using an index selector) with Input impl MmapInput and result mode CountResult");
    let jsonpath_query = JsonPathQuery::parse("$.a[0]")?;
    let json_file =
        fs::File::open("/home/mat/rsonpath/crates/rsonpath-test/tests/documents/toml/int_after_list_of_ints.json")?;
    let input = unsafe { MmapInput::map_file(&json_file)? };
    let engine = MainEngine::compile_query(&jsonpath_query)?;
    let result = engine.run::<_, CountResult>(&input)?;
    assert_eq!(result.get(), 1u64, "result != expected");
    Ok(())
}
#[test]
fn deeply_nested_single_atomic_int_after_a_list_of_ints_with_query_take_the_subdocumented_from_a_using_an_index_selector_with_mmap_input_and_count_result_using_recursive_engine(
) -> Result<(), Box<dyn Error>> {
    println ! ("on document int_after_list_of_ints.toml running the query $.a[0] (take the subdocumented from 'a' using an index selector) with Input impl MmapInput and result mode CountResult");
    let jsonpath_query = JsonPathQuery::parse("$.a[0]")?;
    let json_file =
        fs::File::open("/home/mat/rsonpath/crates/rsonpath-test/tests/documents/toml/int_after_list_of_ints.json")?;
    let input = unsafe { MmapInput::map_file(&json_file)? };
    let engine = RecursiveEngine::compile_query(&jsonpath_query)?;
    let result = engine.run::<_, CountResult>(&input)?;
    assert_eq!(result.get(), 1u64, "result != expected");
    Ok(())
}
#[test]
fn deeply_nested_single_atomic_int_after_a_list_of_ints_with_query_take_the_subdocumented_from_a_using_an_index_selector_with_mmap_input_and_index_result_using_main_engine(
) -> Result<(), Box<dyn Error>> {
    println ! ("on document int_after_list_of_ints.toml running the query $.a[0] (take the subdocumented from 'a' using an index selector) with Input impl MmapInput and result mode IndexResult");
    let jsonpath_query = JsonPathQuery::parse("$.a[0]")?;
    let json_file =
        fs::File::open("/home/mat/rsonpath/crates/rsonpath-test/tests/documents/toml/int_after_list_of_ints.json")?;
    let input = unsafe { MmapInput::map_file(&json_file)? };
    let engine = MainEngine::compile_query(&jsonpath_query)?;
    let result = engine.run::<_, IndexResult>(&input)?;
    assert_eq!(result.get(), vec![21usize,], "result != expected");
    Ok(())
}
#[test]
fn deeply_nested_single_atomic_int_after_a_list_of_ints_with_query_take_the_subdocumented_from_a_using_an_index_selector_with_mmap_input_and_index_result_using_recursive_engine(
) -> Result<(), Box<dyn Error>> {
    println ! ("on document int_after_list_of_ints.toml running the query $.a[0] (take the subdocumented from 'a' using an index selector) with Input impl MmapInput and result mode IndexResult");
    let jsonpath_query = JsonPathQuery::parse("$.a[0]")?;
    let json_file =
        fs::File::open("/home/mat/rsonpath/crates/rsonpath-test/tests/documents/toml/int_after_list_of_ints.json")?;
    let input = unsafe { MmapInput::map_file(&json_file)? };
    let engine = RecursiveEngine::compile_query(&jsonpath_query)?;
    let result = engine.run::<_, IndexResult>(&input)?;
    assert_eq!(result.get(), vec![21usize,], "result != expected");
    Ok(())
}
#[test]
fn deeply_nested_single_atomic_int_after_a_list_of_ints_with_query_take_the_subdocumented_from_a_using_an_index_selector_with_owned_bytes_and_count_result_using_main_engine(
) -> Result<(), Box<dyn Error>> {
    println ! ("on document int_after_list_of_ints.toml running the query $.a[0] (take the subdocumented from 'a' using an index selector) with Input impl OwnedBytes and result mode CountResult");
    let jsonpath_query = JsonPathQuery::parse("$.a[0]")?;
    let raw_json =
        fs::read_to_string("/home/mat/rsonpath/crates/rsonpath-test/tests/documents/toml/int_after_list_of_ints.json")?;
    let input = OwnedBytes::new(&raw_json.as_bytes())?;
    let engine = MainEngine::compile_query(&jsonpath_query)?;
    let result = engine.run::<_, CountResult>(&input)?;
    assert_eq!(result.get(), 1u64, "result != expected");
    Ok(())
}
#[test]
fn deeply_nested_single_atomic_int_after_a_list_of_ints_with_query_take_the_subdocumented_from_a_using_an_index_selector_with_owned_bytes_and_count_result_using_recursive_engine(
) -> Result<(), Box<dyn Error>> {
    println ! ("on document int_after_list_of_ints.toml running the query $.a[0] (take the subdocumented from 'a' using an index selector) with Input impl OwnedBytes and result mode CountResult");
    let jsonpath_query = JsonPathQuery::parse("$.a[0]")?;
    let raw_json =
        fs::read_to_string("/home/mat/rsonpath/crates/rsonpath-test/tests/documents/toml/int_after_list_of_ints.json")?;
    let input = OwnedBytes::new(&raw_json.as_bytes())?;
    let engine = RecursiveEngine::compile_query(&jsonpath_query)?;
    let result = engine.run::<_, CountResult>(&input)?;
    assert_eq!(result.get(), 1u64, "result != expected");
    Ok(())
}
#[test]
fn deeply_nested_single_atomic_int_after_a_list_of_ints_with_query_take_the_subdocumented_from_a_using_an_index_selector_with_owned_bytes_and_index_result_using_main_engine(
) -> Result<(), Box<dyn Error>> {
    println ! ("on document int_after_list_of_ints.toml running the query $.a[0] (take the subdocumented from 'a' using an index selector) with Input impl OwnedBytes and result mode IndexResult");
    let jsonpath_query = JsonPathQuery::parse("$.a[0]")?;
    let raw_json =
        fs::read_to_string("/home/mat/rsonpath/crates/rsonpath-test/tests/documents/toml/int_after_list_of_ints.json")?;
    let input = OwnedBytes::new(&raw_json.as_bytes())?;
    let engine = MainEngine::compile_query(&jsonpath_query)?;
    let result = engine.run::<_, IndexResult>(&input)?;
    assert_eq!(result.get(), vec![21usize,], "result != expected");
    Ok(())
}
#[test]
fn deeply_nested_single_atomic_int_after_a_list_of_ints_with_query_take_the_subdocumented_from_a_using_an_index_selector_with_owned_bytes_and_index_result_using_recursive_engine(
) -> Result<(), Box<dyn Error>> {
    println ! ("on document int_after_list_of_ints.toml running the query $.a[0] (take the subdocumented from 'a' using an index selector) with Input impl OwnedBytes and result mode IndexResult");
    let jsonpath_query = JsonPathQuery::parse("$.a[0]")?;
    let raw_json =
        fs::read_to_string("/home/mat/rsonpath/crates/rsonpath-test/tests/documents/toml/int_after_list_of_ints.json")?;
    let input = OwnedBytes::new(&raw_json.as_bytes())?;
    let engine = RecursiveEngine::compile_query(&jsonpath_query)?;
    let result = engine.run::<_, IndexResult>(&input)?;
    assert_eq!(result.get(), vec![21usize,], "result != expected");
    Ok(())
}
#[test]
fn document_with_empty_lists_objects_and_singleton_lists_with_query_select_all_nodes_at_depth_1_and_then_their_first_array_element_with_buffered_input_and_count_result_using_main_engine(
) -> Result<(), Box<dyn Error>> {
    println ! ("on document singletons_and_empties.toml running the query $.*[0] (select all nodes at depth 1 and then their first array element) with Input impl BufferedInput and result mode CountResult");
    let jsonpath_query = JsonPathQuery::parse("$.*[0]")?;
    let json_file =
        fs::File::open("/home/mat/rsonpath/crates/rsonpath-test/tests/documents/toml/singletons_and_empties.json")?;
    let input = BufferedInput::new(json_file);
    let engine = MainEngine::compile_query(&jsonpath_query)?;
    let result = engine.run::<_, CountResult>(&input)?;
    assert_eq!(result.get(), 2u64, "result != expected");
    Ok(())
}
#[test]
fn document_with_empty_lists_objects_and_singleton_lists_with_query_select_all_nodes_at_depth_1_and_then_their_first_array_element_with_buffered_input_and_count_result_using_recursive_engine(
) -> Result<(), Box<dyn Error>> {
    println ! ("on document singletons_and_empties.toml running the query $.*[0] (select all nodes at depth 1 and then their first array element) with Input impl BufferedInput and result mode CountResult");
    let jsonpath_query = JsonPathQuery::parse("$.*[0]")?;
    let json_file =
        fs::File::open("/home/mat/rsonpath/crates/rsonpath-test/tests/documents/toml/singletons_and_empties.json")?;
    let input = BufferedInput::new(json_file);
    let engine = RecursiveEngine::compile_query(&jsonpath_query)?;
    let result = engine.run::<_, CountResult>(&input)?;
    assert_eq!(result.get(), 2u64, "result != expected");
    Ok(())
}
#[test]
fn document_with_empty_lists_objects_and_singleton_lists_with_query_select_all_nodes_at_depth_1_and_then_their_first_array_element_with_buffered_input_and_index_result_using_main_engine(
) -> Result<(), Box<dyn Error>> {
    println ! ("on document singletons_and_empties.toml running the query $.*[0] (select all nodes at depth 1 and then their first array element) with Input impl BufferedInput and result mode IndexResult");
    let jsonpath_query = JsonPathQuery::parse("$.*[0]")?;
    let json_file =
        fs::File::open("/home/mat/rsonpath/crates/rsonpath-test/tests/documents/toml/singletons_and_empties.json")?;
    let input = BufferedInput::new(json_file);
    let engine = MainEngine::compile_query(&jsonpath_query)?;
    let result = engine.run::<_, IndexResult>(&input)?;
    assert_eq!(result.get(), vec![21usize, 50usize,], "result != expected");
    Ok(())
}
#[test]
fn document_with_empty_lists_objects_and_singleton_lists_with_query_select_all_nodes_at_depth_1_and_then_their_first_array_element_with_buffered_input_and_index_result_using_recursive_engine(
) -> Result<(), Box<dyn Error>> {
    println ! ("on document singletons_and_empties.toml running the query $.*[0] (select all nodes at depth 1 and then their first array element) with Input impl BufferedInput and result mode IndexResult");
    let jsonpath_query = JsonPathQuery::parse("$.*[0]")?;
    let json_file =
        fs::File::open("/home/mat/rsonpath/crates/rsonpath-test/tests/documents/toml/singletons_and_empties.json")?;
    let input = BufferedInput::new(json_file);
    let engine = RecursiveEngine::compile_query(&jsonpath_query)?;
    let result = engine.run::<_, IndexResult>(&input)?;
    assert_eq!(result.get(), vec![21usize, 50usize,], "result != expected");
    Ok(())
}
#[test]
fn document_with_empty_lists_objects_and_singleton_lists_with_query_select_all_nodes_at_depth_1_and_then_their_first_array_element_with_mmap_input_and_count_result_using_main_engine(
) -> Result<(), Box<dyn Error>> {
    println ! ("on document singletons_and_empties.toml running the query $.*[0] (select all nodes at depth 1 and then their first array element) with Input impl MmapInput and result mode CountResult");
    let jsonpath_query = JsonPathQuery::parse("$.*[0]")?;
    let json_file =
        fs::File::open("/home/mat/rsonpath/crates/rsonpath-test/tests/documents/toml/singletons_and_empties.json")?;
    let input = unsafe { MmapInput::map_file(&json_file)? };
    let engine = MainEngine::compile_query(&jsonpath_query)?;
    let result = engine.run::<_, CountResult>(&input)?;
    assert_eq!(result.get(), 2u64, "result != expected");
    Ok(())
}
#[test]
fn document_with_empty_lists_objects_and_singleton_lists_with_query_select_all_nodes_at_depth_1_and_then_their_first_array_element_with_mmap_input_and_count_result_using_recursive_engine(
) -> Result<(), Box<dyn Error>> {
    println ! ("on document singletons_and_empties.toml running the query $.*[0] (select all nodes at depth 1 and then their first array element) with Input impl MmapInput and result mode CountResult");
    let jsonpath_query = JsonPathQuery::parse("$.*[0]")?;
    let json_file =
        fs::File::open("/home/mat/rsonpath/crates/rsonpath-test/tests/documents/toml/singletons_and_empties.json")?;
    let input = unsafe { MmapInput::map_file(&json_file)? };
    let engine = RecursiveEngine::compile_query(&jsonpath_query)?;
    let result = engine.run::<_, CountResult>(&input)?;
    assert_eq!(result.get(), 2u64, "result != expected");
    Ok(())
}
#[test]
fn document_with_empty_lists_objects_and_singleton_lists_with_query_select_all_nodes_at_depth_1_and_then_their_first_array_element_with_mmap_input_and_index_result_using_main_engine(
) -> Result<(), Box<dyn Error>> {
    println ! ("on document singletons_and_empties.toml running the query $.*[0] (select all nodes at depth 1 and then their first array element) with Input impl MmapInput and result mode IndexResult");
    let jsonpath_query = JsonPathQuery::parse("$.*[0]")?;
    let json_file =
        fs::File::open("/home/mat/rsonpath/crates/rsonpath-test/tests/documents/toml/singletons_and_empties.json")?;
    let input = unsafe { MmapInput::map_file(&json_file)? };
    let engine = MainEngine::compile_query(&jsonpath_query)?;
    let result = engine.run::<_, IndexResult>(&input)?;
    assert_eq!(result.get(), vec![21usize, 50usize,], "result != expected");
    Ok(())
}
#[test]
fn document_with_empty_lists_objects_and_singleton_lists_with_query_select_all_nodes_at_depth_1_and_then_their_first_array_element_with_mmap_input_and_index_result_using_recursive_engine(
) -> Result<(), Box<dyn Error>> {
    println ! ("on document singletons_and_empties.toml running the query $.*[0] (select all nodes at depth 1 and then their first array element) with Input impl MmapInput and result mode IndexResult");
    let jsonpath_query = JsonPathQuery::parse("$.*[0]")?;
    let json_file =
        fs::File::open("/home/mat/rsonpath/crates/rsonpath-test/tests/documents/toml/singletons_and_empties.json")?;
    let input = unsafe { MmapInput::map_file(&json_file)? };
    let engine = RecursiveEngine::compile_query(&jsonpath_query)?;
    let result = engine.run::<_, IndexResult>(&input)?;
    assert_eq!(result.get(), vec![21usize, 50usize,], "result != expected");
    Ok(())
}
#[test]
fn document_with_empty_lists_objects_and_singleton_lists_with_query_select_all_nodes_at_depth_1_and_then_their_first_array_element_with_owned_bytes_and_count_result_using_main_engine(
) -> Result<(), Box<dyn Error>> {
    println ! ("on document singletons_and_empties.toml running the query $.*[0] (select all nodes at depth 1 and then their first array element) with Input impl OwnedBytes and result mode CountResult");
    let jsonpath_query = JsonPathQuery::parse("$.*[0]")?;
    let raw_json =
        fs::read_to_string("/home/mat/rsonpath/crates/rsonpath-test/tests/documents/toml/singletons_and_empties.json")?;
    let input = OwnedBytes::new(&raw_json.as_bytes())?;
    let engine = MainEngine::compile_query(&jsonpath_query)?;
    let result = engine.run::<_, CountResult>(&input)?;
    assert_eq!(result.get(), 2u64, "result != expected");
    Ok(())
}
#[test]
fn document_with_empty_lists_objects_and_singleton_lists_with_query_select_all_nodes_at_depth_1_and_then_their_first_array_element_with_owned_bytes_and_count_result_using_recursive_engine(
) -> Result<(), Box<dyn Error>> {
    println ! ("on document singletons_and_empties.toml running the query $.*[0] (select all nodes at depth 1 and then their first array element) with Input impl OwnedBytes and result mode CountResult");
    let jsonpath_query = JsonPathQuery::parse("$.*[0]")?;
    let raw_json =
        fs::read_to_string("/home/mat/rsonpath/crates/rsonpath-test/tests/documents/toml/singletons_and_empties.json")?;
    let input = OwnedBytes::new(&raw_json.as_bytes())?;
    let engine = RecursiveEngine::compile_query(&jsonpath_query)?;
    let result = engine.run::<_, CountResult>(&input)?;
    assert_eq!(result.get(), 2u64, "result != expected");
    Ok(())
}
#[test]
fn document_with_empty_lists_objects_and_singleton_lists_with_query_select_all_nodes_at_depth_1_and_then_their_first_array_element_with_owned_bytes_and_index_result_using_main_engine(
) -> Result<(), Box<dyn Error>> {
    println ! ("on document singletons_and_empties.toml running the query $.*[0] (select all nodes at depth 1 and then their first array element) with Input impl OwnedBytes and result mode IndexResult");
    let jsonpath_query = JsonPathQuery::parse("$.*[0]")?;
    let raw_json =
        fs::read_to_string("/home/mat/rsonpath/crates/rsonpath-test/tests/documents/toml/singletons_and_empties.json")?;
    let input = OwnedBytes::new(&raw_json.as_bytes())?;
    let engine = MainEngine::compile_query(&jsonpath_query)?;
    let result = engine.run::<_, IndexResult>(&input)?;
    assert_eq!(result.get(), vec![21usize, 50usize,], "result != expected");
    Ok(())
}
#[test]
fn document_with_empty_lists_objects_and_singleton_lists_with_query_select_all_nodes_at_depth_1_and_then_their_first_array_element_with_owned_bytes_and_index_result_using_recursive_engine(
) -> Result<(), Box<dyn Error>> {
    println ! ("on document singletons_and_empties.toml running the query $.*[0] (select all nodes at depth 1 and then their first array element) with Input impl OwnedBytes and result mode IndexResult");
    let jsonpath_query = JsonPathQuery::parse("$.*[0]")?;
    let raw_json =
        fs::read_to_string("/home/mat/rsonpath/crates/rsonpath-test/tests/documents/toml/singletons_and_empties.json")?;
    let input = OwnedBytes::new(&raw_json.as_bytes())?;
    let engine = RecursiveEngine::compile_query(&jsonpath_query)?;
    let result = engine.run::<_, IndexResult>(&input)?;
    assert_eq!(result.get(), vec![21usize, 50usize,], "result != expected");
    Ok(())
}
#[test]
fn document_with_empty_lists_objects_and_singleton_lists_with_query_select_all_nodes_at_depth_2_with_buffered_input_and_count_result_using_main_engine(
) -> Result<(), Box<dyn Error>> {
    println ! ("on document singletons_and_empties.toml running the query $.*.* (select all nodes at depth 2) with Input impl BufferedInput and result mode CountResult");
    let jsonpath_query = JsonPathQuery::parse("$.*.*")?;
    let json_file =
        fs::File::open("/home/mat/rsonpath/crates/rsonpath-test/tests/documents/toml/singletons_and_empties.json")?;
    let input = BufferedInput::new(json_file);
    let engine = MainEngine::compile_query(&jsonpath_query)?;
    let result = engine.run::<_, CountResult>(&input)?;
    assert_eq!(result.get(), 2u64, "result != expected");
    Ok(())
}
#[test]
fn document_with_empty_lists_objects_and_singleton_lists_with_query_select_all_nodes_at_depth_2_with_buffered_input_and_count_result_using_recursive_engine(
) -> Result<(), Box<dyn Error>> {
    println ! ("on document singletons_and_empties.toml running the query $.*.* (select all nodes at depth 2) with Input impl BufferedInput and result mode CountResult");
    let jsonpath_query = JsonPathQuery::parse("$.*.*")?;
    let json_file =
        fs::File::open("/home/mat/rsonpath/crates/rsonpath-test/tests/documents/toml/singletons_and_empties.json")?;
    let input = BufferedInput::new(json_file);
    let engine = RecursiveEngine::compile_query(&jsonpath_query)?;
    let result = engine.run::<_, CountResult>(&input)?;
    assert_eq!(result.get(), 2u64, "result != expected");
    Ok(())
}
#[test]
fn document_with_empty_lists_objects_and_singleton_lists_with_query_select_all_nodes_at_depth_2_with_buffered_input_and_index_result_using_main_engine(
) -> Result<(), Box<dyn Error>> {
    println ! ("on document singletons_and_empties.toml running the query $.*.* (select all nodes at depth 2) with Input impl BufferedInput and result mode IndexResult");
    let jsonpath_query = JsonPathQuery::parse("$.*.*")?;
    let json_file =
        fs::File::open("/home/mat/rsonpath/crates/rsonpath-test/tests/documents/toml/singletons_and_empties.json")?;
    let input = BufferedInput::new(json_file);
    let engine = MainEngine::compile_query(&jsonpath_query)?;
    let result = engine.run::<_, IndexResult>(&input)?;
    assert_eq!(result.get(), vec![21usize, 50usize,], "result != expected");
    Ok(())
}
#[test]
fn document_with_empty_lists_objects_and_singleton_lists_with_query_select_all_nodes_at_depth_2_with_buffered_input_and_index_result_using_recursive_engine(
) -> Result<(), Box<dyn Error>> {
    println ! ("on document singletons_and_empties.toml running the query $.*.* (select all nodes at depth 2) with Input impl BufferedInput and result mode IndexResult");
    let jsonpath_query = JsonPathQuery::parse("$.*.*")?;
    let json_file =
        fs::File::open("/home/mat/rsonpath/crates/rsonpath-test/tests/documents/toml/singletons_and_empties.json")?;
    let input = BufferedInput::new(json_file);
    let engine = RecursiveEngine::compile_query(&jsonpath_query)?;
    let result = engine.run::<_, IndexResult>(&input)?;
    assert_eq!(result.get(), vec![21usize, 50usize,], "result != expected");
    Ok(())
}
#[test]
fn document_with_empty_lists_objects_and_singleton_lists_with_query_select_all_nodes_at_depth_2_with_mmap_input_and_count_result_using_main_engine(
) -> Result<(), Box<dyn Error>> {
    println ! ("on document singletons_and_empties.toml running the query $.*.* (select all nodes at depth 2) with Input impl MmapInput and result mode CountResult");
    let jsonpath_query = JsonPathQuery::parse("$.*.*")?;
    let json_file =
        fs::File::open("/home/mat/rsonpath/crates/rsonpath-test/tests/documents/toml/singletons_and_empties.json")?;
    let input = unsafe { MmapInput::map_file(&json_file)? };
    let engine = MainEngine::compile_query(&jsonpath_query)?;
    let result = engine.run::<_, CountResult>(&input)?;
    assert_eq!(result.get(), 2u64, "result != expected");
    Ok(())
}
#[test]
fn document_with_empty_lists_objects_and_singleton_lists_with_query_select_all_nodes_at_depth_2_with_mmap_input_and_count_result_using_recursive_engine(
) -> Result<(), Box<dyn Error>> {
    println ! ("on document singletons_and_empties.toml running the query $.*.* (select all nodes at depth 2) with Input impl MmapInput and result mode CountResult");
    let jsonpath_query = JsonPathQuery::parse("$.*.*")?;
    let json_file =
        fs::File::open("/home/mat/rsonpath/crates/rsonpath-test/tests/documents/toml/singletons_and_empties.json")?;
    let input = unsafe { MmapInput::map_file(&json_file)? };
    let engine = RecursiveEngine::compile_query(&jsonpath_query)?;
    let result = engine.run::<_, CountResult>(&input)?;
    assert_eq!(result.get(), 2u64, "result != expected");
    Ok(())
}
#[test]
fn document_with_empty_lists_objects_and_singleton_lists_with_query_select_all_nodes_at_depth_2_with_mmap_input_and_index_result_using_main_engine(
) -> Result<(), Box<dyn Error>> {
    println ! ("on document singletons_and_empties.toml running the query $.*.* (select all nodes at depth 2) with Input impl MmapInput and result mode IndexResult");
    let jsonpath_query = JsonPathQuery::parse("$.*.*")?;
    let json_file =
        fs::File::open("/home/mat/rsonpath/crates/rsonpath-test/tests/documents/toml/singletons_and_empties.json")?;
    let input = unsafe { MmapInput::map_file(&json_file)? };
    let engine = MainEngine::compile_query(&jsonpath_query)?;
    let result = engine.run::<_, IndexResult>(&input)?;
    assert_eq!(result.get(), vec![21usize, 50usize,], "result != expected");
    Ok(())
}
#[test]
fn document_with_empty_lists_objects_and_singleton_lists_with_query_select_all_nodes_at_depth_2_with_mmap_input_and_index_result_using_recursive_engine(
) -> Result<(), Box<dyn Error>> {
    println ! ("on document singletons_and_empties.toml running the query $.*.* (select all nodes at depth 2) with Input impl MmapInput and result mode IndexResult");
    let jsonpath_query = JsonPathQuery::parse("$.*.*")?;
    let json_file =
        fs::File::open("/home/mat/rsonpath/crates/rsonpath-test/tests/documents/toml/singletons_and_empties.json")?;
    let input = unsafe { MmapInput::map_file(&json_file)? };
    let engine = RecursiveEngine::compile_query(&jsonpath_query)?;
    let result = engine.run::<_, IndexResult>(&input)?;
    assert_eq!(result.get(), vec![21usize, 50usize,], "result != expected");
    Ok(())
}
#[test]
fn document_with_empty_lists_objects_and_singleton_lists_with_query_select_all_nodes_at_depth_2_with_owned_bytes_and_count_result_using_main_engine(
) -> Result<(), Box<dyn Error>> {
    println ! ("on document singletons_and_empties.toml running the query $.*.* (select all nodes at depth 2) with Input impl OwnedBytes and result mode CountResult");
    let jsonpath_query = JsonPathQuery::parse("$.*.*")?;
    let raw_json =
        fs::read_to_string("/home/mat/rsonpath/crates/rsonpath-test/tests/documents/toml/singletons_and_empties.json")?;
    let input = OwnedBytes::new(&raw_json.as_bytes())?;
    let engine = MainEngine::compile_query(&jsonpath_query)?;
    let result = engine.run::<_, CountResult>(&input)?;
    assert_eq!(result.get(), 2u64, "result != expected");
    Ok(())
}
#[test]
fn document_with_empty_lists_objects_and_singleton_lists_with_query_select_all_nodes_at_depth_2_with_owned_bytes_and_count_result_using_recursive_engine(
) -> Result<(), Box<dyn Error>> {
    println ! ("on document singletons_and_empties.toml running the query $.*.* (select all nodes at depth 2) with Input impl OwnedBytes and result mode CountResult");
    let jsonpath_query = JsonPathQuery::parse("$.*.*")?;
    let raw_json =
        fs::read_to_string("/home/mat/rsonpath/crates/rsonpath-test/tests/documents/toml/singletons_and_empties.json")?;
    let input = OwnedBytes::new(&raw_json.as_bytes())?;
    let engine = RecursiveEngine::compile_query(&jsonpath_query)?;
    let result = engine.run::<_, CountResult>(&input)?;
    assert_eq!(result.get(), 2u64, "result != expected");
    Ok(())
}
#[test]
fn document_with_empty_lists_objects_and_singleton_lists_with_query_select_all_nodes_at_depth_2_with_owned_bytes_and_index_result_using_main_engine(
) -> Result<(), Box<dyn Error>> {
    println ! ("on document singletons_and_empties.toml running the query $.*.* (select all nodes at depth 2) with Input impl OwnedBytes and result mode IndexResult");
    let jsonpath_query = JsonPathQuery::parse("$.*.*")?;
    let raw_json =
        fs::read_to_string("/home/mat/rsonpath/crates/rsonpath-test/tests/documents/toml/singletons_and_empties.json")?;
    let input = OwnedBytes::new(&raw_json.as_bytes())?;
    let engine = MainEngine::compile_query(&jsonpath_query)?;
    let result = engine.run::<_, IndexResult>(&input)?;
    assert_eq!(result.get(), vec![21usize, 50usize,], "result != expected");
    Ok(())
}
#[test]
fn document_with_empty_lists_objects_and_singleton_lists_with_query_select_all_nodes_at_depth_2_with_owned_bytes_and_index_result_using_recursive_engine(
) -> Result<(), Box<dyn Error>> {
    println ! ("on document singletons_and_empties.toml running the query $.*.* (select all nodes at depth 2) with Input impl OwnedBytes and result mode IndexResult");
    let jsonpath_query = JsonPathQuery::parse("$.*.*")?;
    let raw_json =
        fs::read_to_string("/home/mat/rsonpath/crates/rsonpath-test/tests/documents/toml/singletons_and_empties.json")?;
    let input = OwnedBytes::new(&raw_json.as_bytes())?;
    let engine = RecursiveEngine::compile_query(&jsonpath_query)?;
    let result = engine.run::<_, IndexResult>(&input)?;
    assert_eq!(result.get(), vec![21usize, 50usize,], "result != expected");
    Ok(())
}
#[test]
fn document_with_empty_lists_objects_and_singleton_lists_with_query_select_all_nodes_at_depth_at_least_2_with_buffered_input_and_count_result_using_main_engine(
) -> Result<(), Box<dyn Error>> {
    println ! ("on document singletons_and_empties.toml running the query $..*.* (select all nodes at depth at least 2) with Input impl BufferedInput and result mode CountResult");
    let jsonpath_query = JsonPathQuery::parse("$..*.*")?;
    let json_file =
        fs::File::open("/home/mat/rsonpath/crates/rsonpath-test/tests/documents/toml/singletons_and_empties.json")?;
    let input = BufferedInput::new(json_file);
    let engine = MainEngine::compile_query(&jsonpath_query)?;
    let result = engine.run::<_, CountResult>(&input)?;
    assert_eq!(result.get(), 2u64, "result != expected");
    Ok(())
}
#[test]
fn document_with_empty_lists_objects_and_singleton_lists_with_query_select_all_nodes_at_depth_at_least_2_with_buffered_input_and_count_result_using_recursive_engine(
) -> Result<(), Box<dyn Error>> {
    println ! ("on document singletons_and_empties.toml running the query $..*.* (select all nodes at depth at least 2) with Input impl BufferedInput and result mode CountResult");
    let jsonpath_query = JsonPathQuery::parse("$..*.*")?;
    let json_file =
        fs::File::open("/home/mat/rsonpath/crates/rsonpath-test/tests/documents/toml/singletons_and_empties.json")?;
    let input = BufferedInput::new(json_file);
    let engine = RecursiveEngine::compile_query(&jsonpath_query)?;
    let result = engine.run::<_, CountResult>(&input)?;
    assert_eq!(result.get(), 2u64, "result != expected");
    Ok(())
}
#[test]
fn document_with_empty_lists_objects_and_singleton_lists_with_query_select_all_nodes_at_depth_at_least_2_with_buffered_input_and_index_result_using_main_engine(
) -> Result<(), Box<dyn Error>> {
    println ! ("on document singletons_and_empties.toml running the query $..*.* (select all nodes at depth at least 2) with Input impl BufferedInput and result mode IndexResult");
    let jsonpath_query = JsonPathQuery::parse("$..*.*")?;
    let json_file =
        fs::File::open("/home/mat/rsonpath/crates/rsonpath-test/tests/documents/toml/singletons_and_empties.json")?;
    let input = BufferedInput::new(json_file);
    let engine = MainEngine::compile_query(&jsonpath_query)?;
    let result = engine.run::<_, IndexResult>(&input)?;
    assert_eq!(result.get(), vec![21usize, 50usize,], "result != expected");
    Ok(())
}
#[test]
fn document_with_empty_lists_objects_and_singleton_lists_with_query_select_all_nodes_at_depth_at_least_2_with_buffered_input_and_index_result_using_recursive_engine(
) -> Result<(), Box<dyn Error>> {
    println ! ("on document singletons_and_empties.toml running the query $..*.* (select all nodes at depth at least 2) with Input impl BufferedInput and result mode IndexResult");
    let jsonpath_query = JsonPathQuery::parse("$..*.*")?;
    let json_file =
        fs::File::open("/home/mat/rsonpath/crates/rsonpath-test/tests/documents/toml/singletons_and_empties.json")?;
    let input = BufferedInput::new(json_file);
    let engine = RecursiveEngine::compile_query(&jsonpath_query)?;
    let result = engine.run::<_, IndexResult>(&input)?;
    assert_eq!(result.get(), vec![21usize, 50usize,], "result != expected");
    Ok(())
}
#[test]
fn document_with_empty_lists_objects_and_singleton_lists_with_query_select_all_nodes_at_depth_at_least_2_with_mmap_input_and_count_result_using_main_engine(
) -> Result<(), Box<dyn Error>> {
    println ! ("on document singletons_and_empties.toml running the query $..*.* (select all nodes at depth at least 2) with Input impl MmapInput and result mode CountResult");
    let jsonpath_query = JsonPathQuery::parse("$..*.*")?;
    let json_file =
        fs::File::open("/home/mat/rsonpath/crates/rsonpath-test/tests/documents/toml/singletons_and_empties.json")?;
    let input = unsafe { MmapInput::map_file(&json_file)? };
    let engine = MainEngine::compile_query(&jsonpath_query)?;
    let result = engine.run::<_, CountResult>(&input)?;
    assert_eq!(result.get(), 2u64, "result != expected");
    Ok(())
}
#[test]
fn document_with_empty_lists_objects_and_singleton_lists_with_query_select_all_nodes_at_depth_at_least_2_with_mmap_input_and_count_result_using_recursive_engine(
) -> Result<(), Box<dyn Error>> {
    println ! ("on document singletons_and_empties.toml running the query $..*.* (select all nodes at depth at least 2) with Input impl MmapInput and result mode CountResult");
    let jsonpath_query = JsonPathQuery::parse("$..*.*")?;
    let json_file =
        fs::File::open("/home/mat/rsonpath/crates/rsonpath-test/tests/documents/toml/singletons_and_empties.json")?;
    let input = unsafe { MmapInput::map_file(&json_file)? };
    let engine = RecursiveEngine::compile_query(&jsonpath_query)?;
    let result = engine.run::<_, CountResult>(&input)?;
    assert_eq!(result.get(), 2u64, "result != expected");
    Ok(())
}
#[test]
fn document_with_empty_lists_objects_and_singleton_lists_with_query_select_all_nodes_at_depth_at_least_2_with_mmap_input_and_index_result_using_main_engine(
) -> Result<(), Box<dyn Error>> {
    println ! ("on document singletons_and_empties.toml running the query $..*.* (select all nodes at depth at least 2) with Input impl MmapInput and result mode IndexResult");
    let jsonpath_query = JsonPathQuery::parse("$..*.*")?;
    let json_file =
        fs::File::open("/home/mat/rsonpath/crates/rsonpath-test/tests/documents/toml/singletons_and_empties.json")?;
    let input = unsafe { MmapInput::map_file(&json_file)? };
    let engine = MainEngine::compile_query(&jsonpath_query)?;
    let result = engine.run::<_, IndexResult>(&input)?;
    assert_eq!(result.get(), vec![21usize, 50usize,], "result != expected");
    Ok(())
}
#[test]
fn document_with_empty_lists_objects_and_singleton_lists_with_query_select_all_nodes_at_depth_at_least_2_with_mmap_input_and_index_result_using_recursive_engine(
) -> Result<(), Box<dyn Error>> {
    println ! ("on document singletons_and_empties.toml running the query $..*.* (select all nodes at depth at least 2) with Input impl MmapInput and result mode IndexResult");
    let jsonpath_query = JsonPathQuery::parse("$..*.*")?;
    let json_file =
        fs::File::open("/home/mat/rsonpath/crates/rsonpath-test/tests/documents/toml/singletons_and_empties.json")?;
    let input = unsafe { MmapInput::map_file(&json_file)? };
    let engine = RecursiveEngine::compile_query(&jsonpath_query)?;
    let result = engine.run::<_, IndexResult>(&input)?;
    assert_eq!(result.get(), vec![21usize, 50usize,], "result != expected");
    Ok(())
}
#[test]
fn document_with_empty_lists_objects_and_singleton_lists_with_query_select_all_nodes_at_depth_at_least_2_with_owned_bytes_and_count_result_using_main_engine(
) -> Result<(), Box<dyn Error>> {
    println ! ("on document singletons_and_empties.toml running the query $..*.* (select all nodes at depth at least 2) with Input impl OwnedBytes and result mode CountResult");
    let jsonpath_query = JsonPathQuery::parse("$..*.*")?;
    let raw_json =
        fs::read_to_string("/home/mat/rsonpath/crates/rsonpath-test/tests/documents/toml/singletons_and_empties.json")?;
    let input = OwnedBytes::new(&raw_json.as_bytes())?;
    let engine = MainEngine::compile_query(&jsonpath_query)?;
    let result = engine.run::<_, CountResult>(&input)?;
    assert_eq!(result.get(), 2u64, "result != expected");
    Ok(())
}
#[test]
fn document_with_empty_lists_objects_and_singleton_lists_with_query_select_all_nodes_at_depth_at_least_2_with_owned_bytes_and_count_result_using_recursive_engine(
) -> Result<(), Box<dyn Error>> {
    println ! ("on document singletons_and_empties.toml running the query $..*.* (select all nodes at depth at least 2) with Input impl OwnedBytes and result mode CountResult");
    let jsonpath_query = JsonPathQuery::parse("$..*.*")?;
    let raw_json =
        fs::read_to_string("/home/mat/rsonpath/crates/rsonpath-test/tests/documents/toml/singletons_and_empties.json")?;
    let input = OwnedBytes::new(&raw_json.as_bytes())?;
    let engine = RecursiveEngine::compile_query(&jsonpath_query)?;
    let result = engine.run::<_, CountResult>(&input)?;
    assert_eq!(result.get(), 2u64, "result != expected");
    Ok(())
}
#[test]
fn document_with_empty_lists_objects_and_singleton_lists_with_query_select_all_nodes_at_depth_at_least_2_with_owned_bytes_and_index_result_using_main_engine(
) -> Result<(), Box<dyn Error>> {
    println ! ("on document singletons_and_empties.toml running the query $..*.* (select all nodes at depth at least 2) with Input impl OwnedBytes and result mode IndexResult");
    let jsonpath_query = JsonPathQuery::parse("$..*.*")?;
    let raw_json =
        fs::read_to_string("/home/mat/rsonpath/crates/rsonpath-test/tests/documents/toml/singletons_and_empties.json")?;
    let input = OwnedBytes::new(&raw_json.as_bytes())?;
    let engine = MainEngine::compile_query(&jsonpath_query)?;
    let result = engine.run::<_, IndexResult>(&input)?;
    assert_eq!(result.get(), vec![21usize, 50usize,], "result != expected");
    Ok(())
}
#[test]
fn document_with_empty_lists_objects_and_singleton_lists_with_query_select_all_nodes_at_depth_at_least_2_with_owned_bytes_and_index_result_using_recursive_engine(
) -> Result<(), Box<dyn Error>> {
    println ! ("on document singletons_and_empties.toml running the query $..*.* (select all nodes at depth at least 2) with Input impl OwnedBytes and result mode IndexResult");
    let jsonpath_query = JsonPathQuery::parse("$..*.*")?;
    let raw_json =
        fs::read_to_string("/home/mat/rsonpath/crates/rsonpath-test/tests/documents/toml/singletons_and_empties.json")?;
    let input = OwnedBytes::new(&raw_json.as_bytes())?;
    let engine = RecursiveEngine::compile_query(&jsonpath_query)?;
    let result = engine.run::<_, IndexResult>(&input)?;
    assert_eq!(result.get(), vec![21usize, 50usize,], "result != expected");
    Ok(())
}
#[test]
fn empty_array_root_with_query_select_any_descendant_there_are_none_with_buffered_input_and_count_result_using_main_engine(
) -> Result<(), Box<dyn Error>> {
    println ! ("on document empty_array.toml running the query $..* (select any descendant (there are none)) with Input impl BufferedInput and result mode CountResult");
    let jsonpath_query = JsonPathQuery::parse("$..*")?;
    let json_file = fs::File::open("/home/mat/rsonpath/crates/rsonpath-test/tests/documents/toml/empty_array.json")?;
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
    let json_file = fs::File::open("/home/mat/rsonpath/crates/rsonpath-test/tests/documents/toml/empty_array.json")?;
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
    let json_file = fs::File::open("/home/mat/rsonpath/crates/rsonpath-test/tests/documents/toml/empty_array.json")?;
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
    let json_file = fs::File::open("/home/mat/rsonpath/crates/rsonpath-test/tests/documents/toml/empty_array.json")?;
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
    let json_file = fs::File::open("/home/mat/rsonpath/crates/rsonpath-test/tests/documents/toml/empty_array.json")?;
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
    let json_file = fs::File::open("/home/mat/rsonpath/crates/rsonpath-test/tests/documents/toml/empty_array.json")?;
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
    let json_file = fs::File::open("/home/mat/rsonpath/crates/rsonpath-test/tests/documents/toml/empty_array.json")?;
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
    let json_file = fs::File::open("/home/mat/rsonpath/crates/rsonpath-test/tests/documents/toml/empty_array.json")?;
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
    let raw_json = fs::read_to_string("/home/mat/rsonpath/crates/rsonpath-test/tests/documents/toml/empty_array.json")?;
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
    let raw_json = fs::read_to_string("/home/mat/rsonpath/crates/rsonpath-test/tests/documents/toml/empty_array.json")?;
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
    let raw_json = fs::read_to_string("/home/mat/rsonpath/crates/rsonpath-test/tests/documents/toml/empty_array.json")?;
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
    let raw_json = fs::read_to_string("/home/mat/rsonpath/crates/rsonpath-test/tests/documents/toml/empty_array.json")?;
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
    let json_file = fs::File::open("/home/mat/rsonpath/crates/rsonpath-test/tests/documents/toml/empty_array.json")?;
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
    let json_file = fs::File::open("/home/mat/rsonpath/crates/rsonpath-test/tests/documents/toml/empty_array.json")?;
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
    let json_file = fs::File::open("/home/mat/rsonpath/crates/rsonpath-test/tests/documents/toml/empty_array.json")?;
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
    let json_file = fs::File::open("/home/mat/rsonpath/crates/rsonpath-test/tests/documents/toml/empty_array.json")?;
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
    let json_file = fs::File::open("/home/mat/rsonpath/crates/rsonpath-test/tests/documents/toml/empty_array.json")?;
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
    let json_file = fs::File::open("/home/mat/rsonpath/crates/rsonpath-test/tests/documents/toml/empty_array.json")?;
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
    let json_file = fs::File::open("/home/mat/rsonpath/crates/rsonpath-test/tests/documents/toml/empty_array.json")?;
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
    let json_file = fs::File::open("/home/mat/rsonpath/crates/rsonpath-test/tests/documents/toml/empty_array.json")?;
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
    let raw_json = fs::read_to_string("/home/mat/rsonpath/crates/rsonpath-test/tests/documents/toml/empty_array.json")?;
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
    let raw_json = fs::read_to_string("/home/mat/rsonpath/crates/rsonpath-test/tests/documents/toml/empty_array.json")?;
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
    let raw_json = fs::read_to_string("/home/mat/rsonpath/crates/rsonpath-test/tests/documents/toml/empty_array.json")?;
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
    let raw_json = fs::read_to_string("/home/mat/rsonpath/crates/rsonpath-test/tests/documents/toml/empty_array.json")?;
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
    let json_file = fs::File::open("/home/mat/rsonpath/crates/rsonpath-test/tests/documents/toml/empty_array.json")?;
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
    let json_file = fs::File::open("/home/mat/rsonpath/crates/rsonpath-test/tests/documents/toml/empty_array.json")?;
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
    let json_file = fs::File::open("/home/mat/rsonpath/crates/rsonpath-test/tests/documents/toml/empty_array.json")?;
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
    let json_file = fs::File::open("/home/mat/rsonpath/crates/rsonpath-test/tests/documents/toml/empty_array.json")?;
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
    let json_file = fs::File::open("/home/mat/rsonpath/crates/rsonpath-test/tests/documents/toml/empty_array.json")?;
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
    let json_file = fs::File::open("/home/mat/rsonpath/crates/rsonpath-test/tests/documents/toml/empty_array.json")?;
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
    let json_file = fs::File::open("/home/mat/rsonpath/crates/rsonpath-test/tests/documents/toml/empty_array.json")?;
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
    let json_file = fs::File::open("/home/mat/rsonpath/crates/rsonpath-test/tests/documents/toml/empty_array.json")?;
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
    let raw_json = fs::read_to_string("/home/mat/rsonpath/crates/rsonpath-test/tests/documents/toml/empty_array.json")?;
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
    let raw_json = fs::read_to_string("/home/mat/rsonpath/crates/rsonpath-test/tests/documents/toml/empty_array.json")?;
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
    let raw_json = fs::read_to_string("/home/mat/rsonpath/crates/rsonpath-test/tests/documents/toml/empty_array.json")?;
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
    let raw_json = fs::read_to_string("/home/mat/rsonpath/crates/rsonpath-test/tests/documents/toml/empty_array.json")?;
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
    let json_file = fs::File::open("/home/mat/rsonpath/crates/rsonpath-test/tests/documents/toml/empty_array.json")?;
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
    let json_file = fs::File::open("/home/mat/rsonpath/crates/rsonpath-test/tests/documents/toml/empty_array.json")?;
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
    let json_file = fs::File::open("/home/mat/rsonpath/crates/rsonpath-test/tests/documents/toml/empty_array.json")?;
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
    let json_file = fs::File::open("/home/mat/rsonpath/crates/rsonpath-test/tests/documents/toml/empty_array.json")?;
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
    let json_file = fs::File::open("/home/mat/rsonpath/crates/rsonpath-test/tests/documents/toml/empty_array.json")?;
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
    let json_file = fs::File::open("/home/mat/rsonpath/crates/rsonpath-test/tests/documents/toml/empty_array.json")?;
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
    let json_file = fs::File::open("/home/mat/rsonpath/crates/rsonpath-test/tests/documents/toml/empty_array.json")?;
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
    let json_file = fs::File::open("/home/mat/rsonpath/crates/rsonpath-test/tests/documents/toml/empty_array.json")?;
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
    let raw_json = fs::read_to_string("/home/mat/rsonpath/crates/rsonpath-test/tests/documents/toml/empty_array.json")?;
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
    let raw_json = fs::read_to_string("/home/mat/rsonpath/crates/rsonpath-test/tests/documents/toml/empty_array.json")?;
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
    let raw_json = fs::read_to_string("/home/mat/rsonpath/crates/rsonpath-test/tests/documents/toml/empty_array.json")?;
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
    let raw_json = fs::read_to_string("/home/mat/rsonpath/crates/rsonpath-test/tests/documents/toml/empty_array.json")?;
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
    let json_file = fs::File::open("/home/mat/rsonpath/crates/rsonpath-test/tests/documents/toml/empty_array.json")?;
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
    let json_file = fs::File::open("/home/mat/rsonpath/crates/rsonpath-test/tests/documents/toml/empty_array.json")?;
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
    let json_file = fs::File::open("/home/mat/rsonpath/crates/rsonpath-test/tests/documents/toml/empty_array.json")?;
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
    let json_file = fs::File::open("/home/mat/rsonpath/crates/rsonpath-test/tests/documents/toml/empty_array.json")?;
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
    let json_file = fs::File::open("/home/mat/rsonpath/crates/rsonpath-test/tests/documents/toml/empty_array.json")?;
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
    let json_file = fs::File::open("/home/mat/rsonpath/crates/rsonpath-test/tests/documents/toml/empty_array.json")?;
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
    let json_file = fs::File::open("/home/mat/rsonpath/crates/rsonpath-test/tests/documents/toml/empty_array.json")?;
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
    let json_file = fs::File::open("/home/mat/rsonpath/crates/rsonpath-test/tests/documents/toml/empty_array.json")?;
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
    let raw_json = fs::read_to_string("/home/mat/rsonpath/crates/rsonpath-test/tests/documents/toml/empty_array.json")?;
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
    let raw_json = fs::read_to_string("/home/mat/rsonpath/crates/rsonpath-test/tests/documents/toml/empty_array.json")?;
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
    let raw_json = fs::read_to_string("/home/mat/rsonpath/crates/rsonpath-test/tests/documents/toml/empty_array.json")?;
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
    let raw_json = fs::read_to_string("/home/mat/rsonpath/crates/rsonpath-test/tests/documents/toml/empty_array.json")?;
    let input = OwnedBytes::new(&raw_json.as_bytes())?;
    let engine = RecursiveEngine::compile_query(&jsonpath_query)?;
    let result = engine.run::<_, IndexResult>(&input)?;
    assert_eq!(result.get(), vec![0usize,], "result != expected");
    Ok(())
}
#[test]
fn empty_document_with_query_select_the_root_empty_query_with_buffered_input_and_count_result_using_main_engine(
) -> Result<(), Box<dyn Error>> {
    println ! ("on document empty.toml running the query  (select the root (empty query)) with Input impl BufferedInput and result mode CountResult");
    let jsonpath_query = JsonPathQuery::parse("")?;
    let json_file = fs::File::open("/home/mat/rsonpath/crates/rsonpath-test/tests/documents/toml/empty.json")?;
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
    let json_file = fs::File::open("/home/mat/rsonpath/crates/rsonpath-test/tests/documents/toml/empty.json")?;
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
    let json_file = fs::File::open("/home/mat/rsonpath/crates/rsonpath-test/tests/documents/toml/empty.json")?;
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
    let json_file = fs::File::open("/home/mat/rsonpath/crates/rsonpath-test/tests/documents/toml/empty.json")?;
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
    let json_file = fs::File::open("/home/mat/rsonpath/crates/rsonpath-test/tests/documents/toml/empty.json")?;
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
    let json_file = fs::File::open("/home/mat/rsonpath/crates/rsonpath-test/tests/documents/toml/empty.json")?;
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
    let json_file = fs::File::open("/home/mat/rsonpath/crates/rsonpath-test/tests/documents/toml/empty.json")?;
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
    let json_file = fs::File::open("/home/mat/rsonpath/crates/rsonpath-test/tests/documents/toml/empty.json")?;
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
    let raw_json = fs::read_to_string("/home/mat/rsonpath/crates/rsonpath-test/tests/documents/toml/empty.json")?;
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
    let raw_json = fs::read_to_string("/home/mat/rsonpath/crates/rsonpath-test/tests/documents/toml/empty.json")?;
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
    let raw_json = fs::read_to_string("/home/mat/rsonpath/crates/rsonpath-test/tests/documents/toml/empty.json")?;
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
    let raw_json = fs::read_to_string("/home/mat/rsonpath/crates/rsonpath-test/tests/documents/toml/empty.json")?;
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
    let json_file = fs::File::open("/home/mat/rsonpath/crates/rsonpath-test/tests/documents/toml/empty.json")?;
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
    let json_file = fs::File::open("/home/mat/rsonpath/crates/rsonpath-test/tests/documents/toml/empty.json")?;
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
    let json_file = fs::File::open("/home/mat/rsonpath/crates/rsonpath-test/tests/documents/toml/empty.json")?;
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
    let json_file = fs::File::open("/home/mat/rsonpath/crates/rsonpath-test/tests/documents/toml/empty.json")?;
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
    let json_file = fs::File::open("/home/mat/rsonpath/crates/rsonpath-test/tests/documents/toml/empty.json")?;
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
    let json_file = fs::File::open("/home/mat/rsonpath/crates/rsonpath-test/tests/documents/toml/empty.json")?;
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
    let json_file = fs::File::open("/home/mat/rsonpath/crates/rsonpath-test/tests/documents/toml/empty.json")?;
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
    let json_file = fs::File::open("/home/mat/rsonpath/crates/rsonpath-test/tests/documents/toml/empty.json")?;
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
    let raw_json = fs::read_to_string("/home/mat/rsonpath/crates/rsonpath-test/tests/documents/toml/empty.json")?;
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
    let raw_json = fs::read_to_string("/home/mat/rsonpath/crates/rsonpath-test/tests/documents/toml/empty.json")?;
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
    let raw_json = fs::read_to_string("/home/mat/rsonpath/crates/rsonpath-test/tests/documents/toml/empty.json")?;
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
    let raw_json = fs::read_to_string("/home/mat/rsonpath/crates/rsonpath-test/tests/documents/toml/empty.json")?;
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
    let json_file = fs::File::open("/home/mat/rsonpath/crates/rsonpath-test/tests/documents/toml/empty_object.json")?;
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
    let json_file = fs::File::open("/home/mat/rsonpath/crates/rsonpath-test/tests/documents/toml/empty_object.json")?;
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
    let json_file = fs::File::open("/home/mat/rsonpath/crates/rsonpath-test/tests/documents/toml/empty_object.json")?;
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
    let json_file = fs::File::open("/home/mat/rsonpath/crates/rsonpath-test/tests/documents/toml/empty_object.json")?;
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
    let json_file = fs::File::open("/home/mat/rsonpath/crates/rsonpath-test/tests/documents/toml/empty_object.json")?;
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
    let json_file = fs::File::open("/home/mat/rsonpath/crates/rsonpath-test/tests/documents/toml/empty_object.json")?;
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
    let json_file = fs::File::open("/home/mat/rsonpath/crates/rsonpath-test/tests/documents/toml/empty_object.json")?;
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
    let json_file = fs::File::open("/home/mat/rsonpath/crates/rsonpath-test/tests/documents/toml/empty_object.json")?;
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
    let raw_json =
        fs::read_to_string("/home/mat/rsonpath/crates/rsonpath-test/tests/documents/toml/empty_object.json")?;
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
    let raw_json =
        fs::read_to_string("/home/mat/rsonpath/crates/rsonpath-test/tests/documents/toml/empty_object.json")?;
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
    let raw_json =
        fs::read_to_string("/home/mat/rsonpath/crates/rsonpath-test/tests/documents/toml/empty_object.json")?;
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
    let raw_json =
        fs::read_to_string("/home/mat/rsonpath/crates/rsonpath-test/tests/documents/toml/empty_object.json")?;
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
    let json_file = fs::File::open("/home/mat/rsonpath/crates/rsonpath-test/tests/documents/toml/empty_object.json")?;
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
    let json_file = fs::File::open("/home/mat/rsonpath/crates/rsonpath-test/tests/documents/toml/empty_object.json")?;
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
    let json_file = fs::File::open("/home/mat/rsonpath/crates/rsonpath-test/tests/documents/toml/empty_object.json")?;
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
    let json_file = fs::File::open("/home/mat/rsonpath/crates/rsonpath-test/tests/documents/toml/empty_object.json")?;
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
    let json_file = fs::File::open("/home/mat/rsonpath/crates/rsonpath-test/tests/documents/toml/empty_object.json")?;
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
    let json_file = fs::File::open("/home/mat/rsonpath/crates/rsonpath-test/tests/documents/toml/empty_object.json")?;
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
    let json_file = fs::File::open("/home/mat/rsonpath/crates/rsonpath-test/tests/documents/toml/empty_object.json")?;
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
    let json_file = fs::File::open("/home/mat/rsonpath/crates/rsonpath-test/tests/documents/toml/empty_object.json")?;
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
    let raw_json =
        fs::read_to_string("/home/mat/rsonpath/crates/rsonpath-test/tests/documents/toml/empty_object.json")?;
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
    let raw_json =
        fs::read_to_string("/home/mat/rsonpath/crates/rsonpath-test/tests/documents/toml/empty_object.json")?;
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
    let raw_json =
        fs::read_to_string("/home/mat/rsonpath/crates/rsonpath-test/tests/documents/toml/empty_object.json")?;
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
    let raw_json =
        fs::read_to_string("/home/mat/rsonpath/crates/rsonpath-test/tests/documents/toml/empty_object.json")?;
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
    let json_file = fs::File::open("/home/mat/rsonpath/crates/rsonpath-test/tests/documents/toml/empty_object.json")?;
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
    let json_file = fs::File::open("/home/mat/rsonpath/crates/rsonpath-test/tests/documents/toml/empty_object.json")?;
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
    let json_file = fs::File::open("/home/mat/rsonpath/crates/rsonpath-test/tests/documents/toml/empty_object.json")?;
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
    let json_file = fs::File::open("/home/mat/rsonpath/crates/rsonpath-test/tests/documents/toml/empty_object.json")?;
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
    let json_file = fs::File::open("/home/mat/rsonpath/crates/rsonpath-test/tests/documents/toml/empty_object.json")?;
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
    let json_file = fs::File::open("/home/mat/rsonpath/crates/rsonpath-test/tests/documents/toml/empty_object.json")?;
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
    let json_file = fs::File::open("/home/mat/rsonpath/crates/rsonpath-test/tests/documents/toml/empty_object.json")?;
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
    let json_file = fs::File::open("/home/mat/rsonpath/crates/rsonpath-test/tests/documents/toml/empty_object.json")?;
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
    let raw_json =
        fs::read_to_string("/home/mat/rsonpath/crates/rsonpath-test/tests/documents/toml/empty_object.json")?;
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
    let raw_json =
        fs::read_to_string("/home/mat/rsonpath/crates/rsonpath-test/tests/documents/toml/empty_object.json")?;
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
    let raw_json =
        fs::read_to_string("/home/mat/rsonpath/crates/rsonpath-test/tests/documents/toml/empty_object.json")?;
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
    let raw_json =
        fs::read_to_string("/home/mat/rsonpath/crates/rsonpath-test/tests/documents/toml/empty_object.json")?;
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
    let json_file = fs::File::open("/home/mat/rsonpath/crates/rsonpath-test/tests/documents/toml/empty_object.json")?;
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
    let json_file = fs::File::open("/home/mat/rsonpath/crates/rsonpath-test/tests/documents/toml/empty_object.json")?;
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
    let json_file = fs::File::open("/home/mat/rsonpath/crates/rsonpath-test/tests/documents/toml/empty_object.json")?;
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
    let json_file = fs::File::open("/home/mat/rsonpath/crates/rsonpath-test/tests/documents/toml/empty_object.json")?;
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
    let json_file = fs::File::open("/home/mat/rsonpath/crates/rsonpath-test/tests/documents/toml/empty_object.json")?;
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
    let json_file = fs::File::open("/home/mat/rsonpath/crates/rsonpath-test/tests/documents/toml/empty_object.json")?;
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
    let json_file = fs::File::open("/home/mat/rsonpath/crates/rsonpath-test/tests/documents/toml/empty_object.json")?;
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
    let json_file = fs::File::open("/home/mat/rsonpath/crates/rsonpath-test/tests/documents/toml/empty_object.json")?;
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
    let raw_json =
        fs::read_to_string("/home/mat/rsonpath/crates/rsonpath-test/tests/documents/toml/empty_object.json")?;
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
    let raw_json =
        fs::read_to_string("/home/mat/rsonpath/crates/rsonpath-test/tests/documents/toml/empty_object.json")?;
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
    let raw_json =
        fs::read_to_string("/home/mat/rsonpath/crates/rsonpath-test/tests/documents/toml/empty_object.json")?;
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
    let raw_json =
        fs::read_to_string("/home/mat/rsonpath/crates/rsonpath-test/tests/documents/toml/empty_object.json")?;
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
    let json_file = fs::File::open("/home/mat/rsonpath/crates/rsonpath-test/tests/documents/toml/empty_object.json")?;
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
    let json_file = fs::File::open("/home/mat/rsonpath/crates/rsonpath-test/tests/documents/toml/empty_object.json")?;
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
    let json_file = fs::File::open("/home/mat/rsonpath/crates/rsonpath-test/tests/documents/toml/empty_object.json")?;
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
    let json_file = fs::File::open("/home/mat/rsonpath/crates/rsonpath-test/tests/documents/toml/empty_object.json")?;
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
    let json_file = fs::File::open("/home/mat/rsonpath/crates/rsonpath-test/tests/documents/toml/empty_object.json")?;
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
    let json_file = fs::File::open("/home/mat/rsonpath/crates/rsonpath-test/tests/documents/toml/empty_object.json")?;
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
    let json_file = fs::File::open("/home/mat/rsonpath/crates/rsonpath-test/tests/documents/toml/empty_object.json")?;
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
    let json_file = fs::File::open("/home/mat/rsonpath/crates/rsonpath-test/tests/documents/toml/empty_object.json")?;
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
    let raw_json =
        fs::read_to_string("/home/mat/rsonpath/crates/rsonpath-test/tests/documents/toml/empty_object.json")?;
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
    let raw_json =
        fs::read_to_string("/home/mat/rsonpath/crates/rsonpath-test/tests/documents/toml/empty_object.json")?;
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
    let raw_json =
        fs::read_to_string("/home/mat/rsonpath/crates/rsonpath-test/tests/documents/toml/empty_object.json")?;
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
    let raw_json =
        fs::read_to_string("/home/mat/rsonpath/crates/rsonpath-test/tests/documents/toml/empty_object.json")?;
    let input = OwnedBytes::new(&raw_json.as_bytes())?;
    let engine = RecursiveEngine::compile_query(&jsonpath_query)?;
    let result = engine.run::<_, IndexResult>(&input)?;
    assert_eq!(result.get(), vec![0usize,], "result != expected");
    Ok(())
}
#[test]
fn even_more_deeply_nested_list_structures_with_query_select_each_element_nested_at_a_and_then_each_element_nested_at_b_within_that_with_buffered_input_and_count_result_using_main_engine(
) -> Result<(), Box<dyn Error>> {
    println ! ("on document deep_list_nesting_2.toml running the query $..a..*..b..* (select each element nested at 'a', and then each element nested at 'b' within that) with Input impl BufferedInput and result mode CountResult");
    let jsonpath_query = JsonPathQuery::parse("$..a..*..b..*")?;
    let json_file =
        fs::File::open("/home/mat/rsonpath/crates/rsonpath-test/tests/documents/toml/deep_list_nesting_2.json")?;
    let input = BufferedInput::new(json_file);
    let engine = MainEngine::compile_query(&jsonpath_query)?;
    let result = engine.run::<_, CountResult>(&input)?;
    assert_eq!(result.get(), 9u64, "result != expected");
    Ok(())
}
#[test]
fn even_more_deeply_nested_list_structures_with_query_select_each_element_nested_at_a_and_then_each_element_nested_at_b_within_that_with_buffered_input_and_count_result_using_recursive_engine(
) -> Result<(), Box<dyn Error>> {
    println ! ("on document deep_list_nesting_2.toml running the query $..a..*..b..* (select each element nested at 'a', and then each element nested at 'b' within that) with Input impl BufferedInput and result mode CountResult");
    let jsonpath_query = JsonPathQuery::parse("$..a..*..b..*")?;
    let json_file =
        fs::File::open("/home/mat/rsonpath/crates/rsonpath-test/tests/documents/toml/deep_list_nesting_2.json")?;
    let input = BufferedInput::new(json_file);
    let engine = RecursiveEngine::compile_query(&jsonpath_query)?;
    let result = engine.run::<_, CountResult>(&input)?;
    assert_eq!(result.get(), 9u64, "result != expected");
    Ok(())
}
#[test]
fn even_more_deeply_nested_list_structures_with_query_select_each_element_nested_at_a_and_then_each_element_nested_at_b_within_that_with_buffered_input_and_index_result_using_main_engine(
) -> Result<(), Box<dyn Error>> {
    println ! ("on document deep_list_nesting_2.toml running the query $..a..*..b..* (select each element nested at 'a', and then each element nested at 'b' within that) with Input impl BufferedInput and result mode IndexResult");
    let jsonpath_query = JsonPathQuery::parse("$..a..*..b..*")?;
    let json_file =
        fs::File::open("/home/mat/rsonpath/crates/rsonpath-test/tests/documents/toml/deep_list_nesting_2.json")?;
    let input = BufferedInput::new(json_file);
    let engine = MainEngine::compile_query(&jsonpath_query)?;
    let result = engine.run::<_, IndexResult>(&input)?;
    assert_eq!(
        result.get(),
        vec![228usize, 401usize, 440usize, 479usize, 519usize, 559usize, 601usize, 679usize, 881usize,],
        "result != expected"
    );
    Ok(())
}
#[test]
fn even_more_deeply_nested_list_structures_with_query_select_each_element_nested_at_a_and_then_each_element_nested_at_b_within_that_with_buffered_input_and_index_result_using_recursive_engine(
) -> Result<(), Box<dyn Error>> {
    println ! ("on document deep_list_nesting_2.toml running the query $..a..*..b..* (select each element nested at 'a', and then each element nested at 'b' within that) with Input impl BufferedInput and result mode IndexResult");
    let jsonpath_query = JsonPathQuery::parse("$..a..*..b..*")?;
    let json_file =
        fs::File::open("/home/mat/rsonpath/crates/rsonpath-test/tests/documents/toml/deep_list_nesting_2.json")?;
    let input = BufferedInput::new(json_file);
    let engine = RecursiveEngine::compile_query(&jsonpath_query)?;
    let result = engine.run::<_, IndexResult>(&input)?;
    assert_eq!(
        result.get(),
        vec![228usize, 401usize, 440usize, 479usize, 519usize, 559usize, 601usize, 679usize, 881usize,],
        "result != expected"
    );
    Ok(())
}
#[test]
fn even_more_deeply_nested_list_structures_with_query_select_each_element_nested_at_a_and_then_each_element_nested_at_b_within_that_with_mmap_input_and_count_result_using_main_engine(
) -> Result<(), Box<dyn Error>> {
    println ! ("on document deep_list_nesting_2.toml running the query $..a..*..b..* (select each element nested at 'a', and then each element nested at 'b' within that) with Input impl MmapInput and result mode CountResult");
    let jsonpath_query = JsonPathQuery::parse("$..a..*..b..*")?;
    let json_file =
        fs::File::open("/home/mat/rsonpath/crates/rsonpath-test/tests/documents/toml/deep_list_nesting_2.json")?;
    let input = unsafe { MmapInput::map_file(&json_file)? };
    let engine = MainEngine::compile_query(&jsonpath_query)?;
    let result = engine.run::<_, CountResult>(&input)?;
    assert_eq!(result.get(), 9u64, "result != expected");
    Ok(())
}
#[test]
fn even_more_deeply_nested_list_structures_with_query_select_each_element_nested_at_a_and_then_each_element_nested_at_b_within_that_with_mmap_input_and_count_result_using_recursive_engine(
) -> Result<(), Box<dyn Error>> {
    println ! ("on document deep_list_nesting_2.toml running the query $..a..*..b..* (select each element nested at 'a', and then each element nested at 'b' within that) with Input impl MmapInput and result mode CountResult");
    let jsonpath_query = JsonPathQuery::parse("$..a..*..b..*")?;
    let json_file =
        fs::File::open("/home/mat/rsonpath/crates/rsonpath-test/tests/documents/toml/deep_list_nesting_2.json")?;
    let input = unsafe { MmapInput::map_file(&json_file)? };
    let engine = RecursiveEngine::compile_query(&jsonpath_query)?;
    let result = engine.run::<_, CountResult>(&input)?;
    assert_eq!(result.get(), 9u64, "result != expected");
    Ok(())
}
#[test]
fn even_more_deeply_nested_list_structures_with_query_select_each_element_nested_at_a_and_then_each_element_nested_at_b_within_that_with_mmap_input_and_index_result_using_main_engine(
) -> Result<(), Box<dyn Error>> {
    println ! ("on document deep_list_nesting_2.toml running the query $..a..*..b..* (select each element nested at 'a', and then each element nested at 'b' within that) with Input impl MmapInput and result mode IndexResult");
    let jsonpath_query = JsonPathQuery::parse("$..a..*..b..*")?;
    let json_file =
        fs::File::open("/home/mat/rsonpath/crates/rsonpath-test/tests/documents/toml/deep_list_nesting_2.json")?;
    let input = unsafe { MmapInput::map_file(&json_file)? };
    let engine = MainEngine::compile_query(&jsonpath_query)?;
    let result = engine.run::<_, IndexResult>(&input)?;
    assert_eq!(
        result.get(),
        vec![228usize, 401usize, 440usize, 479usize, 519usize, 559usize, 601usize, 679usize, 881usize,],
        "result != expected"
    );
    Ok(())
}
#[test]
fn even_more_deeply_nested_list_structures_with_query_select_each_element_nested_at_a_and_then_each_element_nested_at_b_within_that_with_mmap_input_and_index_result_using_recursive_engine(
) -> Result<(), Box<dyn Error>> {
    println ! ("on document deep_list_nesting_2.toml running the query $..a..*..b..* (select each element nested at 'a', and then each element nested at 'b' within that) with Input impl MmapInput and result mode IndexResult");
    let jsonpath_query = JsonPathQuery::parse("$..a..*..b..*")?;
    let json_file =
        fs::File::open("/home/mat/rsonpath/crates/rsonpath-test/tests/documents/toml/deep_list_nesting_2.json")?;
    let input = unsafe { MmapInput::map_file(&json_file)? };
    let engine = RecursiveEngine::compile_query(&jsonpath_query)?;
    let result = engine.run::<_, IndexResult>(&input)?;
    assert_eq!(
        result.get(),
        vec![228usize, 401usize, 440usize, 479usize, 519usize, 559usize, 601usize, 679usize, 881usize,],
        "result != expected"
    );
    Ok(())
}
#[test]
fn even_more_deeply_nested_list_structures_with_query_select_each_element_nested_at_a_and_then_each_element_nested_at_b_within_that_with_owned_bytes_and_count_result_using_main_engine(
) -> Result<(), Box<dyn Error>> {
    println ! ("on document deep_list_nesting_2.toml running the query $..a..*..b..* (select each element nested at 'a', and then each element nested at 'b' within that) with Input impl OwnedBytes and result mode CountResult");
    let jsonpath_query = JsonPathQuery::parse("$..a..*..b..*")?;
    let raw_json =
        fs::read_to_string("/home/mat/rsonpath/crates/rsonpath-test/tests/documents/toml/deep_list_nesting_2.json")?;
    let input = OwnedBytes::new(&raw_json.as_bytes())?;
    let engine = MainEngine::compile_query(&jsonpath_query)?;
    let result = engine.run::<_, CountResult>(&input)?;
    assert_eq!(result.get(), 9u64, "result != expected");
    Ok(())
}
#[test]
fn even_more_deeply_nested_list_structures_with_query_select_each_element_nested_at_a_and_then_each_element_nested_at_b_within_that_with_owned_bytes_and_count_result_using_recursive_engine(
) -> Result<(), Box<dyn Error>> {
    println ! ("on document deep_list_nesting_2.toml running the query $..a..*..b..* (select each element nested at 'a', and then each element nested at 'b' within that) with Input impl OwnedBytes and result mode CountResult");
    let jsonpath_query = JsonPathQuery::parse("$..a..*..b..*")?;
    let raw_json =
        fs::read_to_string("/home/mat/rsonpath/crates/rsonpath-test/tests/documents/toml/deep_list_nesting_2.json")?;
    let input = OwnedBytes::new(&raw_json.as_bytes())?;
    let engine = RecursiveEngine::compile_query(&jsonpath_query)?;
    let result = engine.run::<_, CountResult>(&input)?;
    assert_eq!(result.get(), 9u64, "result != expected");
    Ok(())
}
#[test]
fn even_more_deeply_nested_list_structures_with_query_select_each_element_nested_at_a_and_then_each_element_nested_at_b_within_that_with_owned_bytes_and_index_result_using_main_engine(
) -> Result<(), Box<dyn Error>> {
    println ! ("on document deep_list_nesting_2.toml running the query $..a..*..b..* (select each element nested at 'a', and then each element nested at 'b' within that) with Input impl OwnedBytes and result mode IndexResult");
    let jsonpath_query = JsonPathQuery::parse("$..a..*..b..*")?;
    let raw_json =
        fs::read_to_string("/home/mat/rsonpath/crates/rsonpath-test/tests/documents/toml/deep_list_nesting_2.json")?;
    let input = OwnedBytes::new(&raw_json.as_bytes())?;
    let engine = MainEngine::compile_query(&jsonpath_query)?;
    let result = engine.run::<_, IndexResult>(&input)?;
    assert_eq!(
        result.get(),
        vec![228usize, 401usize, 440usize, 479usize, 519usize, 559usize, 601usize, 679usize, 881usize,],
        "result != expected"
    );
    Ok(())
}
#[test]
fn even_more_deeply_nested_list_structures_with_query_select_each_element_nested_at_a_and_then_each_element_nested_at_b_within_that_with_owned_bytes_and_index_result_using_recursive_engine(
) -> Result<(), Box<dyn Error>> {
    println ! ("on document deep_list_nesting_2.toml running the query $..a..*..b..* (select each element nested at 'a', and then each element nested at 'b' within that) with Input impl OwnedBytes and result mode IndexResult");
    let jsonpath_query = JsonPathQuery::parse("$..a..*..b..*")?;
    let raw_json =
        fs::read_to_string("/home/mat/rsonpath/crates/rsonpath-test/tests/documents/toml/deep_list_nesting_2.json")?;
    let input = OwnedBytes::new(&raw_json.as_bytes())?;
    let engine = RecursiveEngine::compile_query(&jsonpath_query)?;
    let result = engine.run::<_, IndexResult>(&input)?;
    assert_eq!(
        result.get(),
        vec![228usize, 401usize, 440usize, 479usize, 519usize, 559usize, 601usize, 679usize, 881usize,],
        "result != expected"
    );
    Ok(())
}
#[test]
fn even_more_deeply_nested_list_structures_with_query_select_each_element_nested_directly_at_a_and_then_each_element_nested_directly_at_b_within_that_with_buffered_input_and_count_result_using_main_engine(
) -> Result<(), Box<dyn Error>> {
    println ! ("on document deep_list_nesting_2.toml running the query $..a.*..b.* (select each element nested directly at 'a', and then each element nested directly at 'b' within that) with Input impl BufferedInput and result mode CountResult");
    let jsonpath_query = JsonPathQuery::parse("$..a.*..b.*")?;
    let json_file =
        fs::File::open("/home/mat/rsonpath/crates/rsonpath-test/tests/documents/toml/deep_list_nesting_2.json")?;
    let input = BufferedInput::new(json_file);
    let engine = MainEngine::compile_query(&jsonpath_query)?;
    let result = engine.run::<_, CountResult>(&input)?;
    assert_eq!(result.get(), 8u64, "result != expected");
    Ok(())
}
#[test]
fn even_more_deeply_nested_list_structures_with_query_select_each_element_nested_directly_at_a_and_then_each_element_nested_directly_at_b_within_that_with_buffered_input_and_count_result_using_recursive_engine(
) -> Result<(), Box<dyn Error>> {
    println ! ("on document deep_list_nesting_2.toml running the query $..a.*..b.* (select each element nested directly at 'a', and then each element nested directly at 'b' within that) with Input impl BufferedInput and result mode CountResult");
    let jsonpath_query = JsonPathQuery::parse("$..a.*..b.*")?;
    let json_file =
        fs::File::open("/home/mat/rsonpath/crates/rsonpath-test/tests/documents/toml/deep_list_nesting_2.json")?;
    let input = BufferedInput::new(json_file);
    let engine = RecursiveEngine::compile_query(&jsonpath_query)?;
    let result = engine.run::<_, CountResult>(&input)?;
    assert_eq!(result.get(), 8u64, "result != expected");
    Ok(())
}
#[test]
fn even_more_deeply_nested_list_structures_with_query_select_each_element_nested_directly_at_a_and_then_each_element_nested_directly_at_b_within_that_with_buffered_input_and_index_result_using_main_engine(
) -> Result<(), Box<dyn Error>> {
    println ! ("on document deep_list_nesting_2.toml running the query $..a.*..b.* (select each element nested directly at 'a', and then each element nested directly at 'b' within that) with Input impl BufferedInput and result mode IndexResult");
    let jsonpath_query = JsonPathQuery::parse("$..a.*..b.*")?;
    let json_file =
        fs::File::open("/home/mat/rsonpath/crates/rsonpath-test/tests/documents/toml/deep_list_nesting_2.json")?;
    let input = BufferedInput::new(json_file);
    let engine = MainEngine::compile_query(&jsonpath_query)?;
    let result = engine.run::<_, IndexResult>(&input)?;
    assert_eq!(
        result.get(),
        vec![228usize, 401usize, 440usize, 479usize, 519usize, 559usize, 679usize, 881usize,],
        "result != expected"
    );
    Ok(())
}
#[test]
fn even_more_deeply_nested_list_structures_with_query_select_each_element_nested_directly_at_a_and_then_each_element_nested_directly_at_b_within_that_with_buffered_input_and_index_result_using_recursive_engine(
) -> Result<(), Box<dyn Error>> {
    println ! ("on document deep_list_nesting_2.toml running the query $..a.*..b.* (select each element nested directly at 'a', and then each element nested directly at 'b' within that) with Input impl BufferedInput and result mode IndexResult");
    let jsonpath_query = JsonPathQuery::parse("$..a.*..b.*")?;
    let json_file =
        fs::File::open("/home/mat/rsonpath/crates/rsonpath-test/tests/documents/toml/deep_list_nesting_2.json")?;
    let input = BufferedInput::new(json_file);
    let engine = RecursiveEngine::compile_query(&jsonpath_query)?;
    let result = engine.run::<_, IndexResult>(&input)?;
    assert_eq!(
        result.get(),
        vec![228usize, 401usize, 440usize, 479usize, 519usize, 559usize, 679usize, 881usize,],
        "result != expected"
    );
    Ok(())
}
#[test]
fn even_more_deeply_nested_list_structures_with_query_select_each_element_nested_directly_at_a_and_then_each_element_nested_directly_at_b_within_that_with_mmap_input_and_count_result_using_main_engine(
) -> Result<(), Box<dyn Error>> {
    println ! ("on document deep_list_nesting_2.toml running the query $..a.*..b.* (select each element nested directly at 'a', and then each element nested directly at 'b' within that) with Input impl MmapInput and result mode CountResult");
    let jsonpath_query = JsonPathQuery::parse("$..a.*..b.*")?;
    let json_file =
        fs::File::open("/home/mat/rsonpath/crates/rsonpath-test/tests/documents/toml/deep_list_nesting_2.json")?;
    let input = unsafe { MmapInput::map_file(&json_file)? };
    let engine = MainEngine::compile_query(&jsonpath_query)?;
    let result = engine.run::<_, CountResult>(&input)?;
    assert_eq!(result.get(), 8u64, "result != expected");
    Ok(())
}
#[test]
fn even_more_deeply_nested_list_structures_with_query_select_each_element_nested_directly_at_a_and_then_each_element_nested_directly_at_b_within_that_with_mmap_input_and_count_result_using_recursive_engine(
) -> Result<(), Box<dyn Error>> {
    println ! ("on document deep_list_nesting_2.toml running the query $..a.*..b.* (select each element nested directly at 'a', and then each element nested directly at 'b' within that) with Input impl MmapInput and result mode CountResult");
    let jsonpath_query = JsonPathQuery::parse("$..a.*..b.*")?;
    let json_file =
        fs::File::open("/home/mat/rsonpath/crates/rsonpath-test/tests/documents/toml/deep_list_nesting_2.json")?;
    let input = unsafe { MmapInput::map_file(&json_file)? };
    let engine = RecursiveEngine::compile_query(&jsonpath_query)?;
    let result = engine.run::<_, CountResult>(&input)?;
    assert_eq!(result.get(), 8u64, "result != expected");
    Ok(())
}
#[test]
fn even_more_deeply_nested_list_structures_with_query_select_each_element_nested_directly_at_a_and_then_each_element_nested_directly_at_b_within_that_with_mmap_input_and_index_result_using_main_engine(
) -> Result<(), Box<dyn Error>> {
    println ! ("on document deep_list_nesting_2.toml running the query $..a.*..b.* (select each element nested directly at 'a', and then each element nested directly at 'b' within that) with Input impl MmapInput and result mode IndexResult");
    let jsonpath_query = JsonPathQuery::parse("$..a.*..b.*")?;
    let json_file =
        fs::File::open("/home/mat/rsonpath/crates/rsonpath-test/tests/documents/toml/deep_list_nesting_2.json")?;
    let input = unsafe { MmapInput::map_file(&json_file)? };
    let engine = MainEngine::compile_query(&jsonpath_query)?;
    let result = engine.run::<_, IndexResult>(&input)?;
    assert_eq!(
        result.get(),
        vec![228usize, 401usize, 440usize, 479usize, 519usize, 559usize, 679usize, 881usize,],
        "result != expected"
    );
    Ok(())
}
#[test]
fn even_more_deeply_nested_list_structures_with_query_select_each_element_nested_directly_at_a_and_then_each_element_nested_directly_at_b_within_that_with_mmap_input_and_index_result_using_recursive_engine(
) -> Result<(), Box<dyn Error>> {
    println ! ("on document deep_list_nesting_2.toml running the query $..a.*..b.* (select each element nested directly at 'a', and then each element nested directly at 'b' within that) with Input impl MmapInput and result mode IndexResult");
    let jsonpath_query = JsonPathQuery::parse("$..a.*..b.*")?;
    let json_file =
        fs::File::open("/home/mat/rsonpath/crates/rsonpath-test/tests/documents/toml/deep_list_nesting_2.json")?;
    let input = unsafe { MmapInput::map_file(&json_file)? };
    let engine = RecursiveEngine::compile_query(&jsonpath_query)?;
    let result = engine.run::<_, IndexResult>(&input)?;
    assert_eq!(
        result.get(),
        vec![228usize, 401usize, 440usize, 479usize, 519usize, 559usize, 679usize, 881usize,],
        "result != expected"
    );
    Ok(())
}
#[test]
fn even_more_deeply_nested_list_structures_with_query_select_each_element_nested_directly_at_a_and_then_each_element_nested_directly_at_b_within_that_with_owned_bytes_and_count_result_using_main_engine(
) -> Result<(), Box<dyn Error>> {
    println ! ("on document deep_list_nesting_2.toml running the query $..a.*..b.* (select each element nested directly at 'a', and then each element nested directly at 'b' within that) with Input impl OwnedBytes and result mode CountResult");
    let jsonpath_query = JsonPathQuery::parse("$..a.*..b.*")?;
    let raw_json =
        fs::read_to_string("/home/mat/rsonpath/crates/rsonpath-test/tests/documents/toml/deep_list_nesting_2.json")?;
    let input = OwnedBytes::new(&raw_json.as_bytes())?;
    let engine = MainEngine::compile_query(&jsonpath_query)?;
    let result = engine.run::<_, CountResult>(&input)?;
    assert_eq!(result.get(), 8u64, "result != expected");
    Ok(())
}
#[test]
fn even_more_deeply_nested_list_structures_with_query_select_each_element_nested_directly_at_a_and_then_each_element_nested_directly_at_b_within_that_with_owned_bytes_and_count_result_using_recursive_engine(
) -> Result<(), Box<dyn Error>> {
    println ! ("on document deep_list_nesting_2.toml running the query $..a.*..b.* (select each element nested directly at 'a', and then each element nested directly at 'b' within that) with Input impl OwnedBytes and result mode CountResult");
    let jsonpath_query = JsonPathQuery::parse("$..a.*..b.*")?;
    let raw_json =
        fs::read_to_string("/home/mat/rsonpath/crates/rsonpath-test/tests/documents/toml/deep_list_nesting_2.json")?;
    let input = OwnedBytes::new(&raw_json.as_bytes())?;
    let engine = RecursiveEngine::compile_query(&jsonpath_query)?;
    let result = engine.run::<_, CountResult>(&input)?;
    assert_eq!(result.get(), 8u64, "result != expected");
    Ok(())
}
#[test]
fn even_more_deeply_nested_list_structures_with_query_select_each_element_nested_directly_at_a_and_then_each_element_nested_directly_at_b_within_that_with_owned_bytes_and_index_result_using_main_engine(
) -> Result<(), Box<dyn Error>> {
    println ! ("on document deep_list_nesting_2.toml running the query $..a.*..b.* (select each element nested directly at 'a', and then each element nested directly at 'b' within that) with Input impl OwnedBytes and result mode IndexResult");
    let jsonpath_query = JsonPathQuery::parse("$..a.*..b.*")?;
    let raw_json =
        fs::read_to_string("/home/mat/rsonpath/crates/rsonpath-test/tests/documents/toml/deep_list_nesting_2.json")?;
    let input = OwnedBytes::new(&raw_json.as_bytes())?;
    let engine = MainEngine::compile_query(&jsonpath_query)?;
    let result = engine.run::<_, IndexResult>(&input)?;
    assert_eq!(
        result.get(),
        vec![228usize, 401usize, 440usize, 479usize, 519usize, 559usize, 679usize, 881usize,],
        "result != expected"
    );
    Ok(())
}
#[test]
fn even_more_deeply_nested_list_structures_with_query_select_each_element_nested_directly_at_a_and_then_each_element_nested_directly_at_b_within_that_with_owned_bytes_and_index_result_using_recursive_engine(
) -> Result<(), Box<dyn Error>> {
    println ! ("on document deep_list_nesting_2.toml running the query $..a.*..b.* (select each element nested directly at 'a', and then each element nested directly at 'b' within that) with Input impl OwnedBytes and result mode IndexResult");
    let jsonpath_query = JsonPathQuery::parse("$..a.*..b.*")?;
    let raw_json =
        fs::read_to_string("/home/mat/rsonpath/crates/rsonpath-test/tests/documents/toml/deep_list_nesting_2.json")?;
    let input = OwnedBytes::new(&raw_json.as_bytes())?;
    let engine = RecursiveEngine::compile_query(&jsonpath_query)?;
    let result = engine.run::<_, IndexResult>(&input)?;
    assert_eq!(
        result.get(),
        vec![228usize, 401usize, 440usize, 479usize, 519usize, 559usize, 679usize, 881usize,],
        "result != expected"
    );
    Ok(())
}
#[test]
fn example_on_jsonpath_com_with_more_nested_structure_with_query_descendant_person_phone_number_number_with_buffered_input_and_count_result_using_main_engine(
) -> Result<(), Box<dyn Error>> {
    println ! ("on document jsonpath_com_modified.toml running the query $..person..phoneNumber..number (descendant person phoneNumber number) with Input impl BufferedInput and result mode CountResult");
    let jsonpath_query = JsonPathQuery::parse("$..person..phoneNumber..number")?;
    let json_file =
        fs::File::open("/home/mat/rsonpath/crates/rsonpath-test/tests/documents/toml/jsonpath_com_modified.json")?;
    let input = BufferedInput::new(json_file);
    let engine = MainEngine::compile_query(&jsonpath_query)?;
    let result = engine.run::<_, CountResult>(&input)?;
    assert_eq!(result.get(), 4u64, "result != expected");
    Ok(())
}
#[test]
fn example_on_jsonpath_com_with_more_nested_structure_with_query_descendant_person_phone_number_number_with_buffered_input_and_count_result_using_recursive_engine(
) -> Result<(), Box<dyn Error>> {
    println ! ("on document jsonpath_com_modified.toml running the query $..person..phoneNumber..number (descendant person phoneNumber number) with Input impl BufferedInput and result mode CountResult");
    let jsonpath_query = JsonPathQuery::parse("$..person..phoneNumber..number")?;
    let json_file =
        fs::File::open("/home/mat/rsonpath/crates/rsonpath-test/tests/documents/toml/jsonpath_com_modified.json")?;
    let input = BufferedInput::new(json_file);
    let engine = RecursiveEngine::compile_query(&jsonpath_query)?;
    let result = engine.run::<_, CountResult>(&input)?;
    assert_eq!(result.get(), 4u64, "result != expected");
    Ok(())
}
#[test]
fn example_on_jsonpath_com_with_more_nested_structure_with_query_descendant_person_phone_number_number_with_buffered_input_and_index_result_using_main_engine(
) -> Result<(), Box<dyn Error>> {
    println ! ("on document jsonpath_com_modified.toml running the query $..person..phoneNumber..number (descendant person phoneNumber number) with Input impl BufferedInput and result mode IndexResult");
    let jsonpath_query = JsonPathQuery::parse("$..person..phoneNumber..number")?;
    let json_file =
        fs::File::open("/home/mat/rsonpath/crates/rsonpath-test/tests/documents/toml/jsonpath_com_modified.json")?;
    let input = BufferedInput::new(json_file);
    let engine = MainEngine::compile_query(&jsonpath_query)?;
    let result = engine.run::<_, IndexResult>(&input)?;
    assert_eq!(
        result.get(),
        vec![334usize, 438usize, 936usize, 1072usize,],
        "result != expected"
    );
    Ok(())
}
#[test]
fn example_on_jsonpath_com_with_more_nested_structure_with_query_descendant_person_phone_number_number_with_buffered_input_and_index_result_using_recursive_engine(
) -> Result<(), Box<dyn Error>> {
    println ! ("on document jsonpath_com_modified.toml running the query $..person..phoneNumber..number (descendant person phoneNumber number) with Input impl BufferedInput and result mode IndexResult");
    let jsonpath_query = JsonPathQuery::parse("$..person..phoneNumber..number")?;
    let json_file =
        fs::File::open("/home/mat/rsonpath/crates/rsonpath-test/tests/documents/toml/jsonpath_com_modified.json")?;
    let input = BufferedInput::new(json_file);
    let engine = RecursiveEngine::compile_query(&jsonpath_query)?;
    let result = engine.run::<_, IndexResult>(&input)?;
    assert_eq!(
        result.get(),
        vec![334usize, 438usize, 936usize, 1072usize,],
        "result != expected"
    );
    Ok(())
}
#[test]
fn example_on_jsonpath_com_with_more_nested_structure_with_query_descendant_person_phone_number_number_with_mmap_input_and_count_result_using_main_engine(
) -> Result<(), Box<dyn Error>> {
    println ! ("on document jsonpath_com_modified.toml running the query $..person..phoneNumber..number (descendant person phoneNumber number) with Input impl MmapInput and result mode CountResult");
    let jsonpath_query = JsonPathQuery::parse("$..person..phoneNumber..number")?;
    let json_file =
        fs::File::open("/home/mat/rsonpath/crates/rsonpath-test/tests/documents/toml/jsonpath_com_modified.json")?;
    let input = unsafe { MmapInput::map_file(&json_file)? };
    let engine = MainEngine::compile_query(&jsonpath_query)?;
    let result = engine.run::<_, CountResult>(&input)?;
    assert_eq!(result.get(), 4u64, "result != expected");
    Ok(())
}
#[test]
fn example_on_jsonpath_com_with_more_nested_structure_with_query_descendant_person_phone_number_number_with_mmap_input_and_count_result_using_recursive_engine(
) -> Result<(), Box<dyn Error>> {
    println ! ("on document jsonpath_com_modified.toml running the query $..person..phoneNumber..number (descendant person phoneNumber number) with Input impl MmapInput and result mode CountResult");
    let jsonpath_query = JsonPathQuery::parse("$..person..phoneNumber..number")?;
    let json_file =
        fs::File::open("/home/mat/rsonpath/crates/rsonpath-test/tests/documents/toml/jsonpath_com_modified.json")?;
    let input = unsafe { MmapInput::map_file(&json_file)? };
    let engine = RecursiveEngine::compile_query(&jsonpath_query)?;
    let result = engine.run::<_, CountResult>(&input)?;
    assert_eq!(result.get(), 4u64, "result != expected");
    Ok(())
}
#[test]
fn example_on_jsonpath_com_with_more_nested_structure_with_query_descendant_person_phone_number_number_with_mmap_input_and_index_result_using_main_engine(
) -> Result<(), Box<dyn Error>> {
    println ! ("on document jsonpath_com_modified.toml running the query $..person..phoneNumber..number (descendant person phoneNumber number) with Input impl MmapInput and result mode IndexResult");
    let jsonpath_query = JsonPathQuery::parse("$..person..phoneNumber..number")?;
    let json_file =
        fs::File::open("/home/mat/rsonpath/crates/rsonpath-test/tests/documents/toml/jsonpath_com_modified.json")?;
    let input = unsafe { MmapInput::map_file(&json_file)? };
    let engine = MainEngine::compile_query(&jsonpath_query)?;
    let result = engine.run::<_, IndexResult>(&input)?;
    assert_eq!(
        result.get(),
        vec![334usize, 438usize, 936usize, 1072usize,],
        "result != expected"
    );
    Ok(())
}
#[test]
fn example_on_jsonpath_com_with_more_nested_structure_with_query_descendant_person_phone_number_number_with_mmap_input_and_index_result_using_recursive_engine(
) -> Result<(), Box<dyn Error>> {
    println ! ("on document jsonpath_com_modified.toml running the query $..person..phoneNumber..number (descendant person phoneNumber number) with Input impl MmapInput and result mode IndexResult");
    let jsonpath_query = JsonPathQuery::parse("$..person..phoneNumber..number")?;
    let json_file =
        fs::File::open("/home/mat/rsonpath/crates/rsonpath-test/tests/documents/toml/jsonpath_com_modified.json")?;
    let input = unsafe { MmapInput::map_file(&json_file)? };
    let engine = RecursiveEngine::compile_query(&jsonpath_query)?;
    let result = engine.run::<_, IndexResult>(&input)?;
    assert_eq!(
        result.get(),
        vec![334usize, 438usize, 936usize, 1072usize,],
        "result != expected"
    );
    Ok(())
}
#[test]
fn example_on_jsonpath_com_with_more_nested_structure_with_query_descendant_person_phone_number_number_with_owned_bytes_and_count_result_using_main_engine(
) -> Result<(), Box<dyn Error>> {
    println ! ("on document jsonpath_com_modified.toml running the query $..person..phoneNumber..number (descendant person phoneNumber number) with Input impl OwnedBytes and result mode CountResult");
    let jsonpath_query = JsonPathQuery::parse("$..person..phoneNumber..number")?;
    let raw_json =
        fs::read_to_string("/home/mat/rsonpath/crates/rsonpath-test/tests/documents/toml/jsonpath_com_modified.json")?;
    let input = OwnedBytes::new(&raw_json.as_bytes())?;
    let engine = MainEngine::compile_query(&jsonpath_query)?;
    let result = engine.run::<_, CountResult>(&input)?;
    assert_eq!(result.get(), 4u64, "result != expected");
    Ok(())
}
#[test]
fn example_on_jsonpath_com_with_more_nested_structure_with_query_descendant_person_phone_number_number_with_owned_bytes_and_count_result_using_recursive_engine(
) -> Result<(), Box<dyn Error>> {
    println ! ("on document jsonpath_com_modified.toml running the query $..person..phoneNumber..number (descendant person phoneNumber number) with Input impl OwnedBytes and result mode CountResult");
    let jsonpath_query = JsonPathQuery::parse("$..person..phoneNumber..number")?;
    let raw_json =
        fs::read_to_string("/home/mat/rsonpath/crates/rsonpath-test/tests/documents/toml/jsonpath_com_modified.json")?;
    let input = OwnedBytes::new(&raw_json.as_bytes())?;
    let engine = RecursiveEngine::compile_query(&jsonpath_query)?;
    let result = engine.run::<_, CountResult>(&input)?;
    assert_eq!(result.get(), 4u64, "result != expected");
    Ok(())
}
#[test]
fn example_on_jsonpath_com_with_more_nested_structure_with_query_descendant_person_phone_number_number_with_owned_bytes_and_index_result_using_main_engine(
) -> Result<(), Box<dyn Error>> {
    println ! ("on document jsonpath_com_modified.toml running the query $..person..phoneNumber..number (descendant person phoneNumber number) with Input impl OwnedBytes and result mode IndexResult");
    let jsonpath_query = JsonPathQuery::parse("$..person..phoneNumber..number")?;
    let raw_json =
        fs::read_to_string("/home/mat/rsonpath/crates/rsonpath-test/tests/documents/toml/jsonpath_com_modified.json")?;
    let input = OwnedBytes::new(&raw_json.as_bytes())?;
    let engine = MainEngine::compile_query(&jsonpath_query)?;
    let result = engine.run::<_, IndexResult>(&input)?;
    assert_eq!(
        result.get(),
        vec![334usize, 438usize, 936usize, 1072usize,],
        "result != expected"
    );
    Ok(())
}
#[test]
fn example_on_jsonpath_com_with_more_nested_structure_with_query_descendant_person_phone_number_number_with_owned_bytes_and_index_result_using_recursive_engine(
) -> Result<(), Box<dyn Error>> {
    println ! ("on document jsonpath_com_modified.toml running the query $..person..phoneNumber..number (descendant person phoneNumber number) with Input impl OwnedBytes and result mode IndexResult");
    let jsonpath_query = JsonPathQuery::parse("$..person..phoneNumber..number")?;
    let raw_json =
        fs::read_to_string("/home/mat/rsonpath/crates/rsonpath-test/tests/documents/toml/jsonpath_com_modified.json")?;
    let input = OwnedBytes::new(&raw_json.as_bytes())?;
    let engine = RecursiveEngine::compile_query(&jsonpath_query)?;
    let result = engine.run::<_, IndexResult>(&input)?;
    assert_eq!(
        result.get(),
        vec![334usize, 438usize, 936usize, 1072usize,],
        "result != expected"
    );
    Ok(())
}
#[test]
fn example_on_jsonpath_com_with_more_nested_structure_with_query_descendant_person_wildcard_type_with_buffered_input_and_count_result_using_main_engine(
) -> Result<(), Box<dyn Error>> {
    println ! ("on document jsonpath_com_modified.toml running the query $..person..*..type (descendant person, wildcard, type) with Input impl BufferedInput and result mode CountResult");
    let jsonpath_query = JsonPathQuery::parse("$..person..*..type")?;
    let json_file =
        fs::File::open("/home/mat/rsonpath/crates/rsonpath-test/tests/documents/toml/jsonpath_com_modified.json")?;
    let input = BufferedInput::new(json_file);
    let engine = MainEngine::compile_query(&jsonpath_query)?;
    let result = engine.run::<_, CountResult>(&input)?;
    assert_eq!(result.get(), 4u64, "result != expected");
    Ok(())
}
#[test]
fn example_on_jsonpath_com_with_more_nested_structure_with_query_descendant_person_wildcard_type_with_buffered_input_and_count_result_using_recursive_engine(
) -> Result<(), Box<dyn Error>> {
    println ! ("on document jsonpath_com_modified.toml running the query $..person..*..type (descendant person, wildcard, type) with Input impl BufferedInput and result mode CountResult");
    let jsonpath_query = JsonPathQuery::parse("$..person..*..type")?;
    let json_file =
        fs::File::open("/home/mat/rsonpath/crates/rsonpath-test/tests/documents/toml/jsonpath_com_modified.json")?;
    let input = BufferedInput::new(json_file);
    let engine = RecursiveEngine::compile_query(&jsonpath_query)?;
    let result = engine.run::<_, CountResult>(&input)?;
    assert_eq!(result.get(), 4u64, "result != expected");
    Ok(())
}
#[test]
fn example_on_jsonpath_com_with_more_nested_structure_with_query_descendant_person_wildcard_type_with_buffered_input_and_index_result_using_main_engine(
) -> Result<(), Box<dyn Error>> {
    println ! ("on document jsonpath_com_modified.toml running the query $..person..*..type (descendant person, wildcard, type) with Input impl BufferedInput and result mode IndexResult");
    let jsonpath_query = JsonPathQuery::parse("$..person..*..type")?;
    let json_file =
        fs::File::open("/home/mat/rsonpath/crates/rsonpath-test/tests/documents/toml/jsonpath_com_modified.json")?;
    let input = BufferedInput::new(json_file);
    let engine = MainEngine::compile_query(&jsonpath_query)?;
    let result = engine.run::<_, IndexResult>(&input)?;
    assert_eq!(
        result.get(),
        vec![298usize, 404usize, 892usize, 1030usize,],
        "result != expected"
    );
    Ok(())
}
#[test]
fn example_on_jsonpath_com_with_more_nested_structure_with_query_descendant_person_wildcard_type_with_buffered_input_and_index_result_using_recursive_engine(
) -> Result<(), Box<dyn Error>> {
    println ! ("on document jsonpath_com_modified.toml running the query $..person..*..type (descendant person, wildcard, type) with Input impl BufferedInput and result mode IndexResult");
    let jsonpath_query = JsonPathQuery::parse("$..person..*..type")?;
    let json_file =
        fs::File::open("/home/mat/rsonpath/crates/rsonpath-test/tests/documents/toml/jsonpath_com_modified.json")?;
    let input = BufferedInput::new(json_file);
    let engine = RecursiveEngine::compile_query(&jsonpath_query)?;
    let result = engine.run::<_, IndexResult>(&input)?;
    assert_eq!(
        result.get(),
        vec![298usize, 404usize, 892usize, 1030usize,],
        "result != expected"
    );
    Ok(())
}
#[test]
fn example_on_jsonpath_com_with_more_nested_structure_with_query_descendant_person_wildcard_type_with_mmap_input_and_count_result_using_main_engine(
) -> Result<(), Box<dyn Error>> {
    println ! ("on document jsonpath_com_modified.toml running the query $..person..*..type (descendant person, wildcard, type) with Input impl MmapInput and result mode CountResult");
    let jsonpath_query = JsonPathQuery::parse("$..person..*..type")?;
    let json_file =
        fs::File::open("/home/mat/rsonpath/crates/rsonpath-test/tests/documents/toml/jsonpath_com_modified.json")?;
    let input = unsafe { MmapInput::map_file(&json_file)? };
    let engine = MainEngine::compile_query(&jsonpath_query)?;
    let result = engine.run::<_, CountResult>(&input)?;
    assert_eq!(result.get(), 4u64, "result != expected");
    Ok(())
}
#[test]
fn example_on_jsonpath_com_with_more_nested_structure_with_query_descendant_person_wildcard_type_with_mmap_input_and_count_result_using_recursive_engine(
) -> Result<(), Box<dyn Error>> {
    println ! ("on document jsonpath_com_modified.toml running the query $..person..*..type (descendant person, wildcard, type) with Input impl MmapInput and result mode CountResult");
    let jsonpath_query = JsonPathQuery::parse("$..person..*..type")?;
    let json_file =
        fs::File::open("/home/mat/rsonpath/crates/rsonpath-test/tests/documents/toml/jsonpath_com_modified.json")?;
    let input = unsafe { MmapInput::map_file(&json_file)? };
    let engine = RecursiveEngine::compile_query(&jsonpath_query)?;
    let result = engine.run::<_, CountResult>(&input)?;
    assert_eq!(result.get(), 4u64, "result != expected");
    Ok(())
}
#[test]
fn example_on_jsonpath_com_with_more_nested_structure_with_query_descendant_person_wildcard_type_with_mmap_input_and_index_result_using_main_engine(
) -> Result<(), Box<dyn Error>> {
    println ! ("on document jsonpath_com_modified.toml running the query $..person..*..type (descendant person, wildcard, type) with Input impl MmapInput and result mode IndexResult");
    let jsonpath_query = JsonPathQuery::parse("$..person..*..type")?;
    let json_file =
        fs::File::open("/home/mat/rsonpath/crates/rsonpath-test/tests/documents/toml/jsonpath_com_modified.json")?;
    let input = unsafe { MmapInput::map_file(&json_file)? };
    let engine = MainEngine::compile_query(&jsonpath_query)?;
    let result = engine.run::<_, IndexResult>(&input)?;
    assert_eq!(
        result.get(),
        vec![298usize, 404usize, 892usize, 1030usize,],
        "result != expected"
    );
    Ok(())
}
#[test]
fn example_on_jsonpath_com_with_more_nested_structure_with_query_descendant_person_wildcard_type_with_mmap_input_and_index_result_using_recursive_engine(
) -> Result<(), Box<dyn Error>> {
    println ! ("on document jsonpath_com_modified.toml running the query $..person..*..type (descendant person, wildcard, type) with Input impl MmapInput and result mode IndexResult");
    let jsonpath_query = JsonPathQuery::parse("$..person..*..type")?;
    let json_file =
        fs::File::open("/home/mat/rsonpath/crates/rsonpath-test/tests/documents/toml/jsonpath_com_modified.json")?;
    let input = unsafe { MmapInput::map_file(&json_file)? };
    let engine = RecursiveEngine::compile_query(&jsonpath_query)?;
    let result = engine.run::<_, IndexResult>(&input)?;
    assert_eq!(
        result.get(),
        vec![298usize, 404usize, 892usize, 1030usize,],
        "result != expected"
    );
    Ok(())
}
#[test]
fn example_on_jsonpath_com_with_more_nested_structure_with_query_descendant_person_wildcard_type_with_owned_bytes_and_count_result_using_main_engine(
) -> Result<(), Box<dyn Error>> {
    println ! ("on document jsonpath_com_modified.toml running the query $..person..*..type (descendant person, wildcard, type) with Input impl OwnedBytes and result mode CountResult");
    let jsonpath_query = JsonPathQuery::parse("$..person..*..type")?;
    let raw_json =
        fs::read_to_string("/home/mat/rsonpath/crates/rsonpath-test/tests/documents/toml/jsonpath_com_modified.json")?;
    let input = OwnedBytes::new(&raw_json.as_bytes())?;
    let engine = MainEngine::compile_query(&jsonpath_query)?;
    let result = engine.run::<_, CountResult>(&input)?;
    assert_eq!(result.get(), 4u64, "result != expected");
    Ok(())
}
#[test]
fn example_on_jsonpath_com_with_more_nested_structure_with_query_descendant_person_wildcard_type_with_owned_bytes_and_count_result_using_recursive_engine(
) -> Result<(), Box<dyn Error>> {
    println ! ("on document jsonpath_com_modified.toml running the query $..person..*..type (descendant person, wildcard, type) with Input impl OwnedBytes and result mode CountResult");
    let jsonpath_query = JsonPathQuery::parse("$..person..*..type")?;
    let raw_json =
        fs::read_to_string("/home/mat/rsonpath/crates/rsonpath-test/tests/documents/toml/jsonpath_com_modified.json")?;
    let input = OwnedBytes::new(&raw_json.as_bytes())?;
    let engine = RecursiveEngine::compile_query(&jsonpath_query)?;
    let result = engine.run::<_, CountResult>(&input)?;
    assert_eq!(result.get(), 4u64, "result != expected");
    Ok(())
}
#[test]
fn example_on_jsonpath_com_with_more_nested_structure_with_query_descendant_person_wildcard_type_with_owned_bytes_and_index_result_using_main_engine(
) -> Result<(), Box<dyn Error>> {
    println ! ("on document jsonpath_com_modified.toml running the query $..person..*..type (descendant person, wildcard, type) with Input impl OwnedBytes and result mode IndexResult");
    let jsonpath_query = JsonPathQuery::parse("$..person..*..type")?;
    let raw_json =
        fs::read_to_string("/home/mat/rsonpath/crates/rsonpath-test/tests/documents/toml/jsonpath_com_modified.json")?;
    let input = OwnedBytes::new(&raw_json.as_bytes())?;
    let engine = MainEngine::compile_query(&jsonpath_query)?;
    let result = engine.run::<_, IndexResult>(&input)?;
    assert_eq!(
        result.get(),
        vec![298usize, 404usize, 892usize, 1030usize,],
        "result != expected"
    );
    Ok(())
}
#[test]
fn example_on_jsonpath_com_with_more_nested_structure_with_query_descendant_person_wildcard_type_with_owned_bytes_and_index_result_using_recursive_engine(
) -> Result<(), Box<dyn Error>> {
    println ! ("on document jsonpath_com_modified.toml running the query $..person..*..type (descendant person, wildcard, type) with Input impl OwnedBytes and result mode IndexResult");
    let jsonpath_query = JsonPathQuery::parse("$..person..*..type")?;
    let raw_json =
        fs::read_to_string("/home/mat/rsonpath/crates/rsonpath-test/tests/documents/toml/jsonpath_com_modified.json")?;
    let input = OwnedBytes::new(&raw_json.as_bytes())?;
    let engine = RecursiveEngine::compile_query(&jsonpath_query)?;
    let result = engine.run::<_, IndexResult>(&input)?;
    assert_eq!(
        result.get(),
        vec![298usize, 404usize, 892usize, 1030usize,],
        "result != expected"
    );
    Ok(())
}
#[test]
fn example_on_jsonpath_com_with_more_nested_structure_with_query_descendant_search_for_number_with_buffered_input_and_count_result_using_main_engine(
) -> Result<(), Box<dyn Error>> {
    println ! ("on document jsonpath_com_modified.toml running the query $..number (descendant search for 'number') with Input impl BufferedInput and result mode CountResult");
    let jsonpath_query = JsonPathQuery::parse("$..number")?;
    let json_file =
        fs::File::open("/home/mat/rsonpath/crates/rsonpath-test/tests/documents/toml/jsonpath_com_modified.json")?;
    let input = BufferedInput::new(json_file);
    let engine = MainEngine::compile_query(&jsonpath_query)?;
    let result = engine.run::<_, CountResult>(&input)?;
    assert_eq!(result.get(), 4u64, "result != expected");
    Ok(())
}
#[test]
fn example_on_jsonpath_com_with_more_nested_structure_with_query_descendant_search_for_number_with_buffered_input_and_count_result_using_recursive_engine(
) -> Result<(), Box<dyn Error>> {
    println ! ("on document jsonpath_com_modified.toml running the query $..number (descendant search for 'number') with Input impl BufferedInput and result mode CountResult");
    let jsonpath_query = JsonPathQuery::parse("$..number")?;
    let json_file =
        fs::File::open("/home/mat/rsonpath/crates/rsonpath-test/tests/documents/toml/jsonpath_com_modified.json")?;
    let input = BufferedInput::new(json_file);
    let engine = RecursiveEngine::compile_query(&jsonpath_query)?;
    let result = engine.run::<_, CountResult>(&input)?;
    assert_eq!(result.get(), 4u64, "result != expected");
    Ok(())
}
#[test]
fn example_on_jsonpath_com_with_more_nested_structure_with_query_descendant_search_for_number_with_buffered_input_and_index_result_using_main_engine(
) -> Result<(), Box<dyn Error>> {
    println ! ("on document jsonpath_com_modified.toml running the query $..number (descendant search for 'number') with Input impl BufferedInput and result mode IndexResult");
    let jsonpath_query = JsonPathQuery::parse("$..number")?;
    let json_file =
        fs::File::open("/home/mat/rsonpath/crates/rsonpath-test/tests/documents/toml/jsonpath_com_modified.json")?;
    let input = BufferedInput::new(json_file);
    let engine = MainEngine::compile_query(&jsonpath_query)?;
    let result = engine.run::<_, IndexResult>(&input)?;
    assert_eq!(
        result.get(),
        vec![334usize, 438usize, 936usize, 1072usize,],
        "result != expected"
    );
    Ok(())
}
#[test]
fn example_on_jsonpath_com_with_more_nested_structure_with_query_descendant_search_for_number_with_buffered_input_and_index_result_using_recursive_engine(
) -> Result<(), Box<dyn Error>> {
    println ! ("on document jsonpath_com_modified.toml running the query $..number (descendant search for 'number') with Input impl BufferedInput and result mode IndexResult");
    let jsonpath_query = JsonPathQuery::parse("$..number")?;
    let json_file =
        fs::File::open("/home/mat/rsonpath/crates/rsonpath-test/tests/documents/toml/jsonpath_com_modified.json")?;
    let input = BufferedInput::new(json_file);
    let engine = RecursiveEngine::compile_query(&jsonpath_query)?;
    let result = engine.run::<_, IndexResult>(&input)?;
    assert_eq!(
        result.get(),
        vec![334usize, 438usize, 936usize, 1072usize,],
        "result != expected"
    );
    Ok(())
}
#[test]
fn example_on_jsonpath_com_with_more_nested_structure_with_query_descendant_search_for_number_with_mmap_input_and_count_result_using_main_engine(
) -> Result<(), Box<dyn Error>> {
    println ! ("on document jsonpath_com_modified.toml running the query $..number (descendant search for 'number') with Input impl MmapInput and result mode CountResult");
    let jsonpath_query = JsonPathQuery::parse("$..number")?;
    let json_file =
        fs::File::open("/home/mat/rsonpath/crates/rsonpath-test/tests/documents/toml/jsonpath_com_modified.json")?;
    let input = unsafe { MmapInput::map_file(&json_file)? };
    let engine = MainEngine::compile_query(&jsonpath_query)?;
    let result = engine.run::<_, CountResult>(&input)?;
    assert_eq!(result.get(), 4u64, "result != expected");
    Ok(())
}
#[test]
fn example_on_jsonpath_com_with_more_nested_structure_with_query_descendant_search_for_number_with_mmap_input_and_count_result_using_recursive_engine(
) -> Result<(), Box<dyn Error>> {
    println ! ("on document jsonpath_com_modified.toml running the query $..number (descendant search for 'number') with Input impl MmapInput and result mode CountResult");
    let jsonpath_query = JsonPathQuery::parse("$..number")?;
    let json_file =
        fs::File::open("/home/mat/rsonpath/crates/rsonpath-test/tests/documents/toml/jsonpath_com_modified.json")?;
    let input = unsafe { MmapInput::map_file(&json_file)? };
    let engine = RecursiveEngine::compile_query(&jsonpath_query)?;
    let result = engine.run::<_, CountResult>(&input)?;
    assert_eq!(result.get(), 4u64, "result != expected");
    Ok(())
}
#[test]
fn example_on_jsonpath_com_with_more_nested_structure_with_query_descendant_search_for_number_with_mmap_input_and_index_result_using_main_engine(
) -> Result<(), Box<dyn Error>> {
    println ! ("on document jsonpath_com_modified.toml running the query $..number (descendant search for 'number') with Input impl MmapInput and result mode IndexResult");
    let jsonpath_query = JsonPathQuery::parse("$..number")?;
    let json_file =
        fs::File::open("/home/mat/rsonpath/crates/rsonpath-test/tests/documents/toml/jsonpath_com_modified.json")?;
    let input = unsafe { MmapInput::map_file(&json_file)? };
    let engine = MainEngine::compile_query(&jsonpath_query)?;
    let result = engine.run::<_, IndexResult>(&input)?;
    assert_eq!(
        result.get(),
        vec![334usize, 438usize, 936usize, 1072usize,],
        "result != expected"
    );
    Ok(())
}
#[test]
fn example_on_jsonpath_com_with_more_nested_structure_with_query_descendant_search_for_number_with_mmap_input_and_index_result_using_recursive_engine(
) -> Result<(), Box<dyn Error>> {
    println ! ("on document jsonpath_com_modified.toml running the query $..number (descendant search for 'number') with Input impl MmapInput and result mode IndexResult");
    let jsonpath_query = JsonPathQuery::parse("$..number")?;
    let json_file =
        fs::File::open("/home/mat/rsonpath/crates/rsonpath-test/tests/documents/toml/jsonpath_com_modified.json")?;
    let input = unsafe { MmapInput::map_file(&json_file)? };
    let engine = RecursiveEngine::compile_query(&jsonpath_query)?;
    let result = engine.run::<_, IndexResult>(&input)?;
    assert_eq!(
        result.get(),
        vec![334usize, 438usize, 936usize, 1072usize,],
        "result != expected"
    );
    Ok(())
}
#[test]
fn example_on_jsonpath_com_with_more_nested_structure_with_query_descendant_search_for_number_with_owned_bytes_and_count_result_using_main_engine(
) -> Result<(), Box<dyn Error>> {
    println ! ("on document jsonpath_com_modified.toml running the query $..number (descendant search for 'number') with Input impl OwnedBytes and result mode CountResult");
    let jsonpath_query = JsonPathQuery::parse("$..number")?;
    let raw_json =
        fs::read_to_string("/home/mat/rsonpath/crates/rsonpath-test/tests/documents/toml/jsonpath_com_modified.json")?;
    let input = OwnedBytes::new(&raw_json.as_bytes())?;
    let engine = MainEngine::compile_query(&jsonpath_query)?;
    let result = engine.run::<_, CountResult>(&input)?;
    assert_eq!(result.get(), 4u64, "result != expected");
    Ok(())
}
#[test]
fn example_on_jsonpath_com_with_more_nested_structure_with_query_descendant_search_for_number_with_owned_bytes_and_count_result_using_recursive_engine(
) -> Result<(), Box<dyn Error>> {
    println ! ("on document jsonpath_com_modified.toml running the query $..number (descendant search for 'number') with Input impl OwnedBytes and result mode CountResult");
    let jsonpath_query = JsonPathQuery::parse("$..number")?;
    let raw_json =
        fs::read_to_string("/home/mat/rsonpath/crates/rsonpath-test/tests/documents/toml/jsonpath_com_modified.json")?;
    let input = OwnedBytes::new(&raw_json.as_bytes())?;
    let engine = RecursiveEngine::compile_query(&jsonpath_query)?;
    let result = engine.run::<_, CountResult>(&input)?;
    assert_eq!(result.get(), 4u64, "result != expected");
    Ok(())
}
#[test]
fn example_on_jsonpath_com_with_more_nested_structure_with_query_descendant_search_for_number_with_owned_bytes_and_index_result_using_main_engine(
) -> Result<(), Box<dyn Error>> {
    println ! ("on document jsonpath_com_modified.toml running the query $..number (descendant search for 'number') with Input impl OwnedBytes and result mode IndexResult");
    let jsonpath_query = JsonPathQuery::parse("$..number")?;
    let raw_json =
        fs::read_to_string("/home/mat/rsonpath/crates/rsonpath-test/tests/documents/toml/jsonpath_com_modified.json")?;
    let input = OwnedBytes::new(&raw_json.as_bytes())?;
    let engine = MainEngine::compile_query(&jsonpath_query)?;
    let result = engine.run::<_, IndexResult>(&input)?;
    assert_eq!(
        result.get(),
        vec![334usize, 438usize, 936usize, 1072usize,],
        "result != expected"
    );
    Ok(())
}
#[test]
fn example_on_jsonpath_com_with_more_nested_structure_with_query_descendant_search_for_number_with_owned_bytes_and_index_result_using_recursive_engine(
) -> Result<(), Box<dyn Error>> {
    println ! ("on document jsonpath_com_modified.toml running the query $..number (descendant search for 'number') with Input impl OwnedBytes and result mode IndexResult");
    let jsonpath_query = JsonPathQuery::parse("$..number")?;
    let raw_json =
        fs::read_to_string("/home/mat/rsonpath/crates/rsonpath-test/tests/documents/toml/jsonpath_com_modified.json")?;
    let input = OwnedBytes::new(&raw_json.as_bytes())?;
    let engine = RecursiveEngine::compile_query(&jsonpath_query)?;
    let result = engine.run::<_, IndexResult>(&input)?;
    assert_eq!(
        result.get(),
        vec![334usize, 438usize, 936usize, 1072usize,],
        "result != expected"
    );
    Ok(())
}
#[test]
fn example_on_jsonpath_com_with_more_nested_structure_with_query_find_spouse_access_person_then_all_phone_numbers_with_buffered_input_and_count_result_using_main_engine(
) -> Result<(), Box<dyn Error>> {
    println ! ("on document jsonpath_com_modified.toml running the query $..spouse.person..phoneNumber[*] (find spouse, access person, then all phoneNumbers) with Input impl BufferedInput and result mode CountResult");
    let jsonpath_query = JsonPathQuery::parse("$..spouse.person..phoneNumber[*]")?;
    let json_file =
        fs::File::open("/home/mat/rsonpath/crates/rsonpath-test/tests/documents/toml/jsonpath_com_modified.json")?;
    let input = BufferedInput::new(json_file);
    let engine = MainEngine::compile_query(&jsonpath_query)?;
    let result = engine.run::<_, CountResult>(&input)?;
    assert_eq!(result.get(), 2u64, "result != expected");
    Ok(())
}
#[test]
fn example_on_jsonpath_com_with_more_nested_structure_with_query_find_spouse_access_person_then_all_phone_numbers_with_buffered_input_and_count_result_using_recursive_engine(
) -> Result<(), Box<dyn Error>> {
    println ! ("on document jsonpath_com_modified.toml running the query $..spouse.person..phoneNumber[*] (find spouse, access person, then all phoneNumbers) with Input impl BufferedInput and result mode CountResult");
    let jsonpath_query = JsonPathQuery::parse("$..spouse.person..phoneNumber[*]")?;
    let json_file =
        fs::File::open("/home/mat/rsonpath/crates/rsonpath-test/tests/documents/toml/jsonpath_com_modified.json")?;
    let input = BufferedInput::new(json_file);
    let engine = RecursiveEngine::compile_query(&jsonpath_query)?;
    let result = engine.run::<_, CountResult>(&input)?;
    assert_eq!(result.get(), 2u64, "result != expected");
    Ok(())
}
#[test]
fn example_on_jsonpath_com_with_more_nested_structure_with_query_find_spouse_access_person_then_all_phone_numbers_with_buffered_input_and_index_result_using_main_engine(
) -> Result<(), Box<dyn Error>> {
    println ! ("on document jsonpath_com_modified.toml running the query $..spouse.person..phoneNumber[*] (find spouse, access person, then all phoneNumbers) with Input impl BufferedInput and result mode IndexResult");
    let jsonpath_query = JsonPathQuery::parse("$..spouse.person..phoneNumber[*]")?;
    let json_file =
        fs::File::open("/home/mat/rsonpath/crates/rsonpath-test/tests/documents/toml/jsonpath_com_modified.json")?;
    let input = BufferedInput::new(json_file);
    let engine = MainEngine::compile_query(&jsonpath_query)?;
    let result = engine.run::<_, IndexResult>(&input)?;
    assert_eq!(result.get(), vec![858usize, 996usize,], "result != expected");
    Ok(())
}
#[test]
fn example_on_jsonpath_com_with_more_nested_structure_with_query_find_spouse_access_person_then_all_phone_numbers_with_buffered_input_and_index_result_using_recursive_engine(
) -> Result<(), Box<dyn Error>> {
    println ! ("on document jsonpath_com_modified.toml running the query $..spouse.person..phoneNumber[*] (find spouse, access person, then all phoneNumbers) with Input impl BufferedInput and result mode IndexResult");
    let jsonpath_query = JsonPathQuery::parse("$..spouse.person..phoneNumber[*]")?;
    let json_file =
        fs::File::open("/home/mat/rsonpath/crates/rsonpath-test/tests/documents/toml/jsonpath_com_modified.json")?;
    let input = BufferedInput::new(json_file);
    let engine = RecursiveEngine::compile_query(&jsonpath_query)?;
    let result = engine.run::<_, IndexResult>(&input)?;
    assert_eq!(result.get(), vec![858usize, 996usize,], "result != expected");
    Ok(())
}
#[test]
fn example_on_jsonpath_com_with_more_nested_structure_with_query_find_spouse_access_person_then_all_phone_numbers_with_mmap_input_and_count_result_using_main_engine(
) -> Result<(), Box<dyn Error>> {
    println ! ("on document jsonpath_com_modified.toml running the query $..spouse.person..phoneNumber[*] (find spouse, access person, then all phoneNumbers) with Input impl MmapInput and result mode CountResult");
    let jsonpath_query = JsonPathQuery::parse("$..spouse.person..phoneNumber[*]")?;
    let json_file =
        fs::File::open("/home/mat/rsonpath/crates/rsonpath-test/tests/documents/toml/jsonpath_com_modified.json")?;
    let input = unsafe { MmapInput::map_file(&json_file)? };
    let engine = MainEngine::compile_query(&jsonpath_query)?;
    let result = engine.run::<_, CountResult>(&input)?;
    assert_eq!(result.get(), 2u64, "result != expected");
    Ok(())
}
#[test]
fn example_on_jsonpath_com_with_more_nested_structure_with_query_find_spouse_access_person_then_all_phone_numbers_with_mmap_input_and_count_result_using_recursive_engine(
) -> Result<(), Box<dyn Error>> {
    println ! ("on document jsonpath_com_modified.toml running the query $..spouse.person..phoneNumber[*] (find spouse, access person, then all phoneNumbers) with Input impl MmapInput and result mode CountResult");
    let jsonpath_query = JsonPathQuery::parse("$..spouse.person..phoneNumber[*]")?;
    let json_file =
        fs::File::open("/home/mat/rsonpath/crates/rsonpath-test/tests/documents/toml/jsonpath_com_modified.json")?;
    let input = unsafe { MmapInput::map_file(&json_file)? };
    let engine = RecursiveEngine::compile_query(&jsonpath_query)?;
    let result = engine.run::<_, CountResult>(&input)?;
    assert_eq!(result.get(), 2u64, "result != expected");
    Ok(())
}
#[test]
fn example_on_jsonpath_com_with_more_nested_structure_with_query_find_spouse_access_person_then_all_phone_numbers_with_mmap_input_and_index_result_using_main_engine(
) -> Result<(), Box<dyn Error>> {
    println ! ("on document jsonpath_com_modified.toml running the query $..spouse.person..phoneNumber[*] (find spouse, access person, then all phoneNumbers) with Input impl MmapInput and result mode IndexResult");
    let jsonpath_query = JsonPathQuery::parse("$..spouse.person..phoneNumber[*]")?;
    let json_file =
        fs::File::open("/home/mat/rsonpath/crates/rsonpath-test/tests/documents/toml/jsonpath_com_modified.json")?;
    let input = unsafe { MmapInput::map_file(&json_file)? };
    let engine = MainEngine::compile_query(&jsonpath_query)?;
    let result = engine.run::<_, IndexResult>(&input)?;
    assert_eq!(result.get(), vec![858usize, 996usize,], "result != expected");
    Ok(())
}
#[test]
fn example_on_jsonpath_com_with_more_nested_structure_with_query_find_spouse_access_person_then_all_phone_numbers_with_mmap_input_and_index_result_using_recursive_engine(
) -> Result<(), Box<dyn Error>> {
    println ! ("on document jsonpath_com_modified.toml running the query $..spouse.person..phoneNumber[*] (find spouse, access person, then all phoneNumbers) with Input impl MmapInput and result mode IndexResult");
    let jsonpath_query = JsonPathQuery::parse("$..spouse.person..phoneNumber[*]")?;
    let json_file =
        fs::File::open("/home/mat/rsonpath/crates/rsonpath-test/tests/documents/toml/jsonpath_com_modified.json")?;
    let input = unsafe { MmapInput::map_file(&json_file)? };
    let engine = RecursiveEngine::compile_query(&jsonpath_query)?;
    let result = engine.run::<_, IndexResult>(&input)?;
    assert_eq!(result.get(), vec![858usize, 996usize,], "result != expected");
    Ok(())
}
#[test]
fn example_on_jsonpath_com_with_more_nested_structure_with_query_find_spouse_access_person_then_all_phone_numbers_with_owned_bytes_and_count_result_using_main_engine(
) -> Result<(), Box<dyn Error>> {
    println ! ("on document jsonpath_com_modified.toml running the query $..spouse.person..phoneNumber[*] (find spouse, access person, then all phoneNumbers) with Input impl OwnedBytes and result mode CountResult");
    let jsonpath_query = JsonPathQuery::parse("$..spouse.person..phoneNumber[*]")?;
    let raw_json =
        fs::read_to_string("/home/mat/rsonpath/crates/rsonpath-test/tests/documents/toml/jsonpath_com_modified.json")?;
    let input = OwnedBytes::new(&raw_json.as_bytes())?;
    let engine = MainEngine::compile_query(&jsonpath_query)?;
    let result = engine.run::<_, CountResult>(&input)?;
    assert_eq!(result.get(), 2u64, "result != expected");
    Ok(())
}
#[test]
fn example_on_jsonpath_com_with_more_nested_structure_with_query_find_spouse_access_person_then_all_phone_numbers_with_owned_bytes_and_count_result_using_recursive_engine(
) -> Result<(), Box<dyn Error>> {
    println ! ("on document jsonpath_com_modified.toml running the query $..spouse.person..phoneNumber[*] (find spouse, access person, then all phoneNumbers) with Input impl OwnedBytes and result mode CountResult");
    let jsonpath_query = JsonPathQuery::parse("$..spouse.person..phoneNumber[*]")?;
    let raw_json =
        fs::read_to_string("/home/mat/rsonpath/crates/rsonpath-test/tests/documents/toml/jsonpath_com_modified.json")?;
    let input = OwnedBytes::new(&raw_json.as_bytes())?;
    let engine = RecursiveEngine::compile_query(&jsonpath_query)?;
    let result = engine.run::<_, CountResult>(&input)?;
    assert_eq!(result.get(), 2u64, "result != expected");
    Ok(())
}
#[test]
fn example_on_jsonpath_com_with_more_nested_structure_with_query_find_spouse_access_person_then_all_phone_numbers_with_owned_bytes_and_index_result_using_main_engine(
) -> Result<(), Box<dyn Error>> {
    println ! ("on document jsonpath_com_modified.toml running the query $..spouse.person..phoneNumber[*] (find spouse, access person, then all phoneNumbers) with Input impl OwnedBytes and result mode IndexResult");
    let jsonpath_query = JsonPathQuery::parse("$..spouse.person..phoneNumber[*]")?;
    let raw_json =
        fs::read_to_string("/home/mat/rsonpath/crates/rsonpath-test/tests/documents/toml/jsonpath_com_modified.json")?;
    let input = OwnedBytes::new(&raw_json.as_bytes())?;
    let engine = MainEngine::compile_query(&jsonpath_query)?;
    let result = engine.run::<_, IndexResult>(&input)?;
    assert_eq!(result.get(), vec![858usize, 996usize,], "result != expected");
    Ok(())
}
#[test]
fn example_on_jsonpath_com_with_more_nested_structure_with_query_find_spouse_access_person_then_all_phone_numbers_with_owned_bytes_and_index_result_using_recursive_engine(
) -> Result<(), Box<dyn Error>> {
    println ! ("on document jsonpath_com_modified.toml running the query $..spouse.person..phoneNumber[*] (find spouse, access person, then all phoneNumbers) with Input impl OwnedBytes and result mode IndexResult");
    let jsonpath_query = JsonPathQuery::parse("$..spouse.person..phoneNumber[*]")?;
    let raw_json =
        fs::read_to_string("/home/mat/rsonpath/crates/rsonpath-test/tests/documents/toml/jsonpath_com_modified.json")?;
    let input = OwnedBytes::new(&raw_json.as_bytes())?;
    let engine = RecursiveEngine::compile_query(&jsonpath_query)?;
    let result = engine.run::<_, IndexResult>(&input)?;
    assert_eq!(result.get(), vec![858usize, 996usize,], "result != expected");
    Ok(())
}
#[test]
fn extract_from_twitter_json_containing_urls_with_multiple_escaped_slashes_with_query_descendant_entities_then_url_with_buffered_input_and_count_result_using_main_engine(
) -> Result<(), Box<dyn Error>> {
    println ! ("on document twitter_urls.toml running the query $..entities..url (descendant entities then url) with Input impl BufferedInput and result mode CountResult");
    let jsonpath_query = JsonPathQuery::parse("$..entities..url")?;
    let json_file = fs::File::open("/home/mat/rsonpath/crates/rsonpath-test/tests/documents/toml/twitter_urls.json")?;
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
    let json_file = fs::File::open("/home/mat/rsonpath/crates/rsonpath-test/tests/documents/toml/twitter_urls.json")?;
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
    let json_file = fs::File::open("/home/mat/rsonpath/crates/rsonpath-test/tests/documents/toml/twitter_urls.json")?;
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
    let json_file = fs::File::open("/home/mat/rsonpath/crates/rsonpath-test/tests/documents/toml/twitter_urls.json")?;
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
    let json_file = fs::File::open("/home/mat/rsonpath/crates/rsonpath-test/tests/documents/toml/twitter_urls.json")?;
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
    let json_file = fs::File::open("/home/mat/rsonpath/crates/rsonpath-test/tests/documents/toml/twitter_urls.json")?;
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
    let json_file = fs::File::open("/home/mat/rsonpath/crates/rsonpath-test/tests/documents/toml/twitter_urls.json")?;
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
    let json_file = fs::File::open("/home/mat/rsonpath/crates/rsonpath-test/tests/documents/toml/twitter_urls.json")?;
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
    let raw_json =
        fs::read_to_string("/home/mat/rsonpath/crates/rsonpath-test/tests/documents/toml/twitter_urls.json")?;
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
    let raw_json =
        fs::read_to_string("/home/mat/rsonpath/crates/rsonpath-test/tests/documents/toml/twitter_urls.json")?;
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
    let raw_json =
        fs::read_to_string("/home/mat/rsonpath/crates/rsonpath-test/tests/documents/toml/twitter_urls.json")?;
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
    let raw_json =
        fs::read_to_string("/home/mat/rsonpath/crates/rsonpath-test/tests/documents/toml/twitter_urls.json")?;
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
    let json_file = fs::File::open("/home/mat/rsonpath/crates/rsonpath-test/tests/documents/toml/twitter_urls.json")?;
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
    let json_file = fs::File::open("/home/mat/rsonpath/crates/rsonpath-test/tests/documents/toml/twitter_urls.json")?;
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
    let json_file = fs::File::open("/home/mat/rsonpath/crates/rsonpath-test/tests/documents/toml/twitter_urls.json")?;
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
    let json_file = fs::File::open("/home/mat/rsonpath/crates/rsonpath-test/tests/documents/toml/twitter_urls.json")?;
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
    let json_file = fs::File::open("/home/mat/rsonpath/crates/rsonpath-test/tests/documents/toml/twitter_urls.json")?;
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
    let json_file = fs::File::open("/home/mat/rsonpath/crates/rsonpath-test/tests/documents/toml/twitter_urls.json")?;
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
    let json_file = fs::File::open("/home/mat/rsonpath/crates/rsonpath-test/tests/documents/toml/twitter_urls.json")?;
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
    let json_file = fs::File::open("/home/mat/rsonpath/crates/rsonpath-test/tests/documents/toml/twitter_urls.json")?;
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
    let raw_json =
        fs::read_to_string("/home/mat/rsonpath/crates/rsonpath-test/tests/documents/toml/twitter_urls.json")?;
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
    let raw_json =
        fs::read_to_string("/home/mat/rsonpath/crates/rsonpath-test/tests/documents/toml/twitter_urls.json")?;
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
    let raw_json =
        fs::read_to_string("/home/mat/rsonpath/crates/rsonpath-test/tests/documents/toml/twitter_urls.json")?;
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
    let raw_json =
        fs::read_to_string("/home/mat/rsonpath/crates/rsonpath-test/tests/documents/toml/twitter_urls.json")?;
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
    let json_file = fs::File::open("/home/mat/rsonpath/crates/rsonpath-test/tests/documents/toml/twitter_urls.json")?;
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
    let json_file = fs::File::open("/home/mat/rsonpath/crates/rsonpath-test/tests/documents/toml/twitter_urls.json")?;
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
    let json_file = fs::File::open("/home/mat/rsonpath/crates/rsonpath-test/tests/documents/toml/twitter_urls.json")?;
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
    let json_file = fs::File::open("/home/mat/rsonpath/crates/rsonpath-test/tests/documents/toml/twitter_urls.json")?;
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
    let json_file = fs::File::open("/home/mat/rsonpath/crates/rsonpath-test/tests/documents/toml/twitter_urls.json")?;
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
    let json_file = fs::File::open("/home/mat/rsonpath/crates/rsonpath-test/tests/documents/toml/twitter_urls.json")?;
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
    let json_file = fs::File::open("/home/mat/rsonpath/crates/rsonpath-test/tests/documents/toml/twitter_urls.json")?;
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
    let json_file = fs::File::open("/home/mat/rsonpath/crates/rsonpath-test/tests/documents/toml/twitter_urls.json")?;
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
    let raw_json =
        fs::read_to_string("/home/mat/rsonpath/crates/rsonpath-test/tests/documents/toml/twitter_urls.json")?;
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
    let raw_json =
        fs::read_to_string("/home/mat/rsonpath/crates/rsonpath-test/tests/documents/toml/twitter_urls.json")?;
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
    let raw_json =
        fs::read_to_string("/home/mat/rsonpath/crates/rsonpath-test/tests/documents/toml/twitter_urls.json")?;
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
    let raw_json =
        fs::read_to_string("/home/mat/rsonpath/crates/rsonpath-test/tests/documents/toml/twitter_urls.json")?;
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
    let json_file = fs::File::open("/home/mat/rsonpath/crates/rsonpath-test/tests/documents/toml/twitter_urls.json")?;
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
    let json_file = fs::File::open("/home/mat/rsonpath/crates/rsonpath-test/tests/documents/toml/twitter_urls.json")?;
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
    let json_file = fs::File::open("/home/mat/rsonpath/crates/rsonpath-test/tests/documents/toml/twitter_urls.json")?;
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
    let json_file = fs::File::open("/home/mat/rsonpath/crates/rsonpath-test/tests/documents/toml/twitter_urls.json")?;
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
    let json_file = fs::File::open("/home/mat/rsonpath/crates/rsonpath-test/tests/documents/toml/twitter_urls.json")?;
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
    let json_file = fs::File::open("/home/mat/rsonpath/crates/rsonpath-test/tests/documents/toml/twitter_urls.json")?;
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
    let json_file = fs::File::open("/home/mat/rsonpath/crates/rsonpath-test/tests/documents/toml/twitter_urls.json")?;
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
    let json_file = fs::File::open("/home/mat/rsonpath/crates/rsonpath-test/tests/documents/toml/twitter_urls.json")?;
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
    let raw_json =
        fs::read_to_string("/home/mat/rsonpath/crates/rsonpath-test/tests/documents/toml/twitter_urls.json")?;
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
    let raw_json =
        fs::read_to_string("/home/mat/rsonpath/crates/rsonpath-test/tests/documents/toml/twitter_urls.json")?;
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
    let raw_json =
        fs::read_to_string("/home/mat/rsonpath/crates/rsonpath-test/tests/documents/toml/twitter_urls.json")?;
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
    let raw_json =
        fs::read_to_string("/home/mat/rsonpath/crates/rsonpath-test/tests/documents/toml/twitter_urls.json")?;
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
    let json_file = fs::File::open("/home/mat/rsonpath/crates/rsonpath-test/tests/documents/toml/twitter_urls.json")?;
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
    let json_file = fs::File::open("/home/mat/rsonpath/crates/rsonpath-test/tests/documents/toml/twitter_urls.json")?;
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
    let json_file = fs::File::open("/home/mat/rsonpath/crates/rsonpath-test/tests/documents/toml/twitter_urls.json")?;
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
    let json_file = fs::File::open("/home/mat/rsonpath/crates/rsonpath-test/tests/documents/toml/twitter_urls.json")?;
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
    let json_file = fs::File::open("/home/mat/rsonpath/crates/rsonpath-test/tests/documents/toml/twitter_urls.json")?;
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
    let json_file = fs::File::open("/home/mat/rsonpath/crates/rsonpath-test/tests/documents/toml/twitter_urls.json")?;
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
    let json_file = fs::File::open("/home/mat/rsonpath/crates/rsonpath-test/tests/documents/toml/twitter_urls.json")?;
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
    let json_file = fs::File::open("/home/mat/rsonpath/crates/rsonpath-test/tests/documents/toml/twitter_urls.json")?;
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
    let raw_json =
        fs::read_to_string("/home/mat/rsonpath/crates/rsonpath-test/tests/documents/toml/twitter_urls.json")?;
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
    let raw_json =
        fs::read_to_string("/home/mat/rsonpath/crates/rsonpath-test/tests/documents/toml/twitter_urls.json")?;
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
    let raw_json =
        fs::read_to_string("/home/mat/rsonpath/crates/rsonpath-test/tests/documents/toml/twitter_urls.json")?;
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
    let raw_json =
        fs::read_to_string("/home/mat/rsonpath/crates/rsonpath-test/tests/documents/toml/twitter_urls.json")?;
    let input = OwnedBytes::new(&raw_json.as_bytes())?;
    let engine = RecursiveEngine::compile_query(&jsonpath_query)?;
    let result = engine.run::<_, IndexResult>(&input)?;
    assert_eq!(result.get(), vec![1100usize,], "result != expected");
    Ok(())
}
#[test]
fn label_b_and_b_with_escaped_quote_to_trick_naive_string_comparison_with_query_descendant_search_for_b_with_a_leading_quote_with_buffered_input_and_count_result_using_main_engine(
) -> Result<(), Box<dyn Error>> {
    println ! ("on document memchr_trap.toml running the query $..['\"b'] (descendant search for 'b' with a leading quote) with Input impl BufferedInput and result mode CountResult");
    let jsonpath_query = JsonPathQuery::parse("$..['\"b']")?;
    let json_file = fs::File::open("/home/mat/rsonpath/crates/rsonpath-test/tests/documents/toml/memchr_trap.json")?;
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
    let json_file = fs::File::open("/home/mat/rsonpath/crates/rsonpath-test/tests/documents/toml/memchr_trap.json")?;
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
    let json_file = fs::File::open("/home/mat/rsonpath/crates/rsonpath-test/tests/documents/toml/memchr_trap.json")?;
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
    let json_file = fs::File::open("/home/mat/rsonpath/crates/rsonpath-test/tests/documents/toml/memchr_trap.json")?;
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
    let json_file = fs::File::open("/home/mat/rsonpath/crates/rsonpath-test/tests/documents/toml/memchr_trap.json")?;
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
    let json_file = fs::File::open("/home/mat/rsonpath/crates/rsonpath-test/tests/documents/toml/memchr_trap.json")?;
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
    let json_file = fs::File::open("/home/mat/rsonpath/crates/rsonpath-test/tests/documents/toml/memchr_trap.json")?;
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
    let json_file = fs::File::open("/home/mat/rsonpath/crates/rsonpath-test/tests/documents/toml/memchr_trap.json")?;
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
    let raw_json = fs::read_to_string("/home/mat/rsonpath/crates/rsonpath-test/tests/documents/toml/memchr_trap.json")?;
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
    let raw_json = fs::read_to_string("/home/mat/rsonpath/crates/rsonpath-test/tests/documents/toml/memchr_trap.json")?;
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
    let raw_json = fs::read_to_string("/home/mat/rsonpath/crates/rsonpath-test/tests/documents/toml/memchr_trap.json")?;
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
    let raw_json = fs::read_to_string("/home/mat/rsonpath/crates/rsonpath-test/tests/documents/toml/memchr_trap.json")?;
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
    let json_file = fs::File::open("/home/mat/rsonpath/crates/rsonpath-test/tests/documents/toml/memchr_trap.json")?;
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
    let json_file = fs::File::open("/home/mat/rsonpath/crates/rsonpath-test/tests/documents/toml/memchr_trap.json")?;
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
    let json_file = fs::File::open("/home/mat/rsonpath/crates/rsonpath-test/tests/documents/toml/memchr_trap.json")?;
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
    let json_file = fs::File::open("/home/mat/rsonpath/crates/rsonpath-test/tests/documents/toml/memchr_trap.json")?;
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
    let json_file = fs::File::open("/home/mat/rsonpath/crates/rsonpath-test/tests/documents/toml/memchr_trap.json")?;
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
    let json_file = fs::File::open("/home/mat/rsonpath/crates/rsonpath-test/tests/documents/toml/memchr_trap.json")?;
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
    let json_file = fs::File::open("/home/mat/rsonpath/crates/rsonpath-test/tests/documents/toml/memchr_trap.json")?;
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
    let json_file = fs::File::open("/home/mat/rsonpath/crates/rsonpath-test/tests/documents/toml/memchr_trap.json")?;
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
    let raw_json = fs::read_to_string("/home/mat/rsonpath/crates/rsonpath-test/tests/documents/toml/memchr_trap.json")?;
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
    let raw_json = fs::read_to_string("/home/mat/rsonpath/crates/rsonpath-test/tests/documents/toml/memchr_trap.json")?;
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
    let raw_json = fs::read_to_string("/home/mat/rsonpath/crates/rsonpath-test/tests/documents/toml/memchr_trap.json")?;
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
    let raw_json = fs::read_to_string("/home/mat/rsonpath/crates/rsonpath-test/tests/documents/toml/memchr_trap.json")?;
    let input = OwnedBytes::new(&raw_json.as_bytes())?;
    let engine = RecursiveEngine::compile_query(&jsonpath_query)?;
    let result = engine.run::<_, IndexResult>(&input)?;
    assert_eq!(result.get(), vec![45usize,], "result != expected");
    Ok(())
}
#[test]
fn list_with_mixed_atomic_and_complex_subdocuments_with_query_select_all_elements_on_the_list_with_buffered_input_and_count_result_using_main_engine(
) -> Result<(), Box<dyn Error>> {
    println ! ("on document heterogenous_list.toml running the query $.a.* (select all elements on the list) with Input impl BufferedInput and result mode CountResult");
    let jsonpath_query = JsonPathQuery::parse("$.a.*")?;
    let json_file =
        fs::File::open("/home/mat/rsonpath/crates/rsonpath-test/tests/documents/toml/heterogenous_list.json")?;
    let input = BufferedInput::new(json_file);
    let engine = MainEngine::compile_query(&jsonpath_query)?;
    let result = engine.run::<_, CountResult>(&input)?;
    assert_eq!(result.get(), 3u64, "result != expected");
    Ok(())
}
#[test]
fn list_with_mixed_atomic_and_complex_subdocuments_with_query_select_all_elements_on_the_list_with_buffered_input_and_count_result_using_recursive_engine(
) -> Result<(), Box<dyn Error>> {
    println ! ("on document heterogenous_list.toml running the query $.a.* (select all elements on the list) with Input impl BufferedInput and result mode CountResult");
    let jsonpath_query = JsonPathQuery::parse("$.a.*")?;
    let json_file =
        fs::File::open("/home/mat/rsonpath/crates/rsonpath-test/tests/documents/toml/heterogenous_list.json")?;
    let input = BufferedInput::new(json_file);
    let engine = RecursiveEngine::compile_query(&jsonpath_query)?;
    let result = engine.run::<_, CountResult>(&input)?;
    assert_eq!(result.get(), 3u64, "result != expected");
    Ok(())
}
#[test]
fn list_with_mixed_atomic_and_complex_subdocuments_with_query_select_all_elements_on_the_list_with_buffered_input_and_index_result_using_main_engine(
) -> Result<(), Box<dyn Error>> {
    println ! ("on document heterogenous_list.toml running the query $.a.* (select all elements on the list) with Input impl BufferedInput and result mode IndexResult");
    let jsonpath_query = JsonPathQuery::parse("$.a.*")?;
    let json_file =
        fs::File::open("/home/mat/rsonpath/crates/rsonpath-test/tests/documents/toml/heterogenous_list.json")?;
    let input = BufferedInput::new(json_file);
    let engine = MainEngine::compile_query(&jsonpath_query)?;
    let result = engine.run::<_, IndexResult>(&input)?;
    assert_eq!(result.get(), vec![15usize, 23usize, 50usize,], "result != expected");
    Ok(())
}
#[test]
fn list_with_mixed_atomic_and_complex_subdocuments_with_query_select_all_elements_on_the_list_with_buffered_input_and_index_result_using_recursive_engine(
) -> Result<(), Box<dyn Error>> {
    println ! ("on document heterogenous_list.toml running the query $.a.* (select all elements on the list) with Input impl BufferedInput and result mode IndexResult");
    let jsonpath_query = JsonPathQuery::parse("$.a.*")?;
    let json_file =
        fs::File::open("/home/mat/rsonpath/crates/rsonpath-test/tests/documents/toml/heterogenous_list.json")?;
    let input = BufferedInput::new(json_file);
    let engine = RecursiveEngine::compile_query(&jsonpath_query)?;
    let result = engine.run::<_, IndexResult>(&input)?;
    assert_eq!(result.get(), vec![15usize, 23usize, 50usize,], "result != expected");
    Ok(())
}
#[test]
fn list_with_mixed_atomic_and_complex_subdocuments_with_query_select_all_elements_on_the_list_with_mmap_input_and_count_result_using_main_engine(
) -> Result<(), Box<dyn Error>> {
    println ! ("on document heterogenous_list.toml running the query $.a.* (select all elements on the list) with Input impl MmapInput and result mode CountResult");
    let jsonpath_query = JsonPathQuery::parse("$.a.*")?;
    let json_file =
        fs::File::open("/home/mat/rsonpath/crates/rsonpath-test/tests/documents/toml/heterogenous_list.json")?;
    let input = unsafe { MmapInput::map_file(&json_file)? };
    let engine = MainEngine::compile_query(&jsonpath_query)?;
    let result = engine.run::<_, CountResult>(&input)?;
    assert_eq!(result.get(), 3u64, "result != expected");
    Ok(())
}
#[test]
fn list_with_mixed_atomic_and_complex_subdocuments_with_query_select_all_elements_on_the_list_with_mmap_input_and_count_result_using_recursive_engine(
) -> Result<(), Box<dyn Error>> {
    println ! ("on document heterogenous_list.toml running the query $.a.* (select all elements on the list) with Input impl MmapInput and result mode CountResult");
    let jsonpath_query = JsonPathQuery::parse("$.a.*")?;
    let json_file =
        fs::File::open("/home/mat/rsonpath/crates/rsonpath-test/tests/documents/toml/heterogenous_list.json")?;
    let input = unsafe { MmapInput::map_file(&json_file)? };
    let engine = RecursiveEngine::compile_query(&jsonpath_query)?;
    let result = engine.run::<_, CountResult>(&input)?;
    assert_eq!(result.get(), 3u64, "result != expected");
    Ok(())
}
#[test]
fn list_with_mixed_atomic_and_complex_subdocuments_with_query_select_all_elements_on_the_list_with_mmap_input_and_index_result_using_main_engine(
) -> Result<(), Box<dyn Error>> {
    println ! ("on document heterogenous_list.toml running the query $.a.* (select all elements on the list) with Input impl MmapInput and result mode IndexResult");
    let jsonpath_query = JsonPathQuery::parse("$.a.*")?;
    let json_file =
        fs::File::open("/home/mat/rsonpath/crates/rsonpath-test/tests/documents/toml/heterogenous_list.json")?;
    let input = unsafe { MmapInput::map_file(&json_file)? };
    let engine = MainEngine::compile_query(&jsonpath_query)?;
    let result = engine.run::<_, IndexResult>(&input)?;
    assert_eq!(result.get(), vec![15usize, 23usize, 50usize,], "result != expected");
    Ok(())
}
#[test]
fn list_with_mixed_atomic_and_complex_subdocuments_with_query_select_all_elements_on_the_list_with_mmap_input_and_index_result_using_recursive_engine(
) -> Result<(), Box<dyn Error>> {
    println ! ("on document heterogenous_list.toml running the query $.a.* (select all elements on the list) with Input impl MmapInput and result mode IndexResult");
    let jsonpath_query = JsonPathQuery::parse("$.a.*")?;
    let json_file =
        fs::File::open("/home/mat/rsonpath/crates/rsonpath-test/tests/documents/toml/heterogenous_list.json")?;
    let input = unsafe { MmapInput::map_file(&json_file)? };
    let engine = RecursiveEngine::compile_query(&jsonpath_query)?;
    let result = engine.run::<_, IndexResult>(&input)?;
    assert_eq!(result.get(), vec![15usize, 23usize, 50usize,], "result != expected");
    Ok(())
}
#[test]
fn list_with_mixed_atomic_and_complex_subdocuments_with_query_select_all_elements_on_the_list_with_owned_bytes_and_count_result_using_main_engine(
) -> Result<(), Box<dyn Error>> {
    println ! ("on document heterogenous_list.toml running the query $.a.* (select all elements on the list) with Input impl OwnedBytes and result mode CountResult");
    let jsonpath_query = JsonPathQuery::parse("$.a.*")?;
    let raw_json =
        fs::read_to_string("/home/mat/rsonpath/crates/rsonpath-test/tests/documents/toml/heterogenous_list.json")?;
    let input = OwnedBytes::new(&raw_json.as_bytes())?;
    let engine = MainEngine::compile_query(&jsonpath_query)?;
    let result = engine.run::<_, CountResult>(&input)?;
    assert_eq!(result.get(), 3u64, "result != expected");
    Ok(())
}
#[test]
fn list_with_mixed_atomic_and_complex_subdocuments_with_query_select_all_elements_on_the_list_with_owned_bytes_and_count_result_using_recursive_engine(
) -> Result<(), Box<dyn Error>> {
    println ! ("on document heterogenous_list.toml running the query $.a.* (select all elements on the list) with Input impl OwnedBytes and result mode CountResult");
    let jsonpath_query = JsonPathQuery::parse("$.a.*")?;
    let raw_json =
        fs::read_to_string("/home/mat/rsonpath/crates/rsonpath-test/tests/documents/toml/heterogenous_list.json")?;
    let input = OwnedBytes::new(&raw_json.as_bytes())?;
    let engine = RecursiveEngine::compile_query(&jsonpath_query)?;
    let result = engine.run::<_, CountResult>(&input)?;
    assert_eq!(result.get(), 3u64, "result != expected");
    Ok(())
}
#[test]
fn list_with_mixed_atomic_and_complex_subdocuments_with_query_select_all_elements_on_the_list_with_owned_bytes_and_index_result_using_main_engine(
) -> Result<(), Box<dyn Error>> {
    println ! ("on document heterogenous_list.toml running the query $.a.* (select all elements on the list) with Input impl OwnedBytes and result mode IndexResult");
    let jsonpath_query = JsonPathQuery::parse("$.a.*")?;
    let raw_json =
        fs::read_to_string("/home/mat/rsonpath/crates/rsonpath-test/tests/documents/toml/heterogenous_list.json")?;
    let input = OwnedBytes::new(&raw_json.as_bytes())?;
    let engine = MainEngine::compile_query(&jsonpath_query)?;
    let result = engine.run::<_, IndexResult>(&input)?;
    assert_eq!(result.get(), vec![15usize, 23usize, 50usize,], "result != expected");
    Ok(())
}
#[test]
fn list_with_mixed_atomic_and_complex_subdocuments_with_query_select_all_elements_on_the_list_with_owned_bytes_and_index_result_using_recursive_engine(
) -> Result<(), Box<dyn Error>> {
    println ! ("on document heterogenous_list.toml running the query $.a.* (select all elements on the list) with Input impl OwnedBytes and result mode IndexResult");
    let jsonpath_query = JsonPathQuery::parse("$.a.*")?;
    let raw_json =
        fs::read_to_string("/home/mat/rsonpath/crates/rsonpath-test/tests/documents/toml/heterogenous_list.json")?;
    let input = OwnedBytes::new(&raw_json.as_bytes())?;
    let engine = RecursiveEngine::compile_query(&jsonpath_query)?;
    let result = engine.run::<_, IndexResult>(&input)?;
    assert_eq!(result.get(), vec![15usize, 23usize, 50usize,], "result != expected");
    Ok(())
}
#[test]
fn list_with_mixed_atomic_and_complex_subdocuments_with_query_select_the_first_element_on_the_list_with_buffered_input_and_count_result_using_main_engine(
) -> Result<(), Box<dyn Error>> {
    println ! ("on document heterogenous_list.toml running the query $.a[0] (select the first element on the list) with Input impl BufferedInput and result mode CountResult");
    let jsonpath_query = JsonPathQuery::parse("$.a[0]")?;
    let json_file =
        fs::File::open("/home/mat/rsonpath/crates/rsonpath-test/tests/documents/toml/heterogenous_list.json")?;
    let input = BufferedInput::new(json_file);
    let engine = MainEngine::compile_query(&jsonpath_query)?;
    let result = engine.run::<_, CountResult>(&input)?;
    assert_eq!(result.get(), 1u64, "result != expected");
    Ok(())
}
#[test]
fn list_with_mixed_atomic_and_complex_subdocuments_with_query_select_the_first_element_on_the_list_with_buffered_input_and_count_result_using_recursive_engine(
) -> Result<(), Box<dyn Error>> {
    println ! ("on document heterogenous_list.toml running the query $.a[0] (select the first element on the list) with Input impl BufferedInput and result mode CountResult");
    let jsonpath_query = JsonPathQuery::parse("$.a[0]")?;
    let json_file =
        fs::File::open("/home/mat/rsonpath/crates/rsonpath-test/tests/documents/toml/heterogenous_list.json")?;
    let input = BufferedInput::new(json_file);
    let engine = RecursiveEngine::compile_query(&jsonpath_query)?;
    let result = engine.run::<_, CountResult>(&input)?;
    assert_eq!(result.get(), 1u64, "result != expected");
    Ok(())
}
#[test]
fn list_with_mixed_atomic_and_complex_subdocuments_with_query_select_the_first_element_on_the_list_with_buffered_input_and_index_result_using_main_engine(
) -> Result<(), Box<dyn Error>> {
    println ! ("on document heterogenous_list.toml running the query $.a[0] (select the first element on the list) with Input impl BufferedInput and result mode IndexResult");
    let jsonpath_query = JsonPathQuery::parse("$.a[0]")?;
    let json_file =
        fs::File::open("/home/mat/rsonpath/crates/rsonpath-test/tests/documents/toml/heterogenous_list.json")?;
    let input = BufferedInput::new(json_file);
    let engine = MainEngine::compile_query(&jsonpath_query)?;
    let result = engine.run::<_, IndexResult>(&input)?;
    assert_eq!(result.get(), vec![15usize,], "result != expected");
    Ok(())
}
#[test]
fn list_with_mixed_atomic_and_complex_subdocuments_with_query_select_the_first_element_on_the_list_with_buffered_input_and_index_result_using_recursive_engine(
) -> Result<(), Box<dyn Error>> {
    println ! ("on document heterogenous_list.toml running the query $.a[0] (select the first element on the list) with Input impl BufferedInput and result mode IndexResult");
    let jsonpath_query = JsonPathQuery::parse("$.a[0]")?;
    let json_file =
        fs::File::open("/home/mat/rsonpath/crates/rsonpath-test/tests/documents/toml/heterogenous_list.json")?;
    let input = BufferedInput::new(json_file);
    let engine = RecursiveEngine::compile_query(&jsonpath_query)?;
    let result = engine.run::<_, IndexResult>(&input)?;
    assert_eq!(result.get(), vec![15usize,], "result != expected");
    Ok(())
}
#[test]
fn list_with_mixed_atomic_and_complex_subdocuments_with_query_select_the_first_element_on_the_list_with_mmap_input_and_count_result_using_main_engine(
) -> Result<(), Box<dyn Error>> {
    println ! ("on document heterogenous_list.toml running the query $.a[0] (select the first element on the list) with Input impl MmapInput and result mode CountResult");
    let jsonpath_query = JsonPathQuery::parse("$.a[0]")?;
    let json_file =
        fs::File::open("/home/mat/rsonpath/crates/rsonpath-test/tests/documents/toml/heterogenous_list.json")?;
    let input = unsafe { MmapInput::map_file(&json_file)? };
    let engine = MainEngine::compile_query(&jsonpath_query)?;
    let result = engine.run::<_, CountResult>(&input)?;
    assert_eq!(result.get(), 1u64, "result != expected");
    Ok(())
}
#[test]
fn list_with_mixed_atomic_and_complex_subdocuments_with_query_select_the_first_element_on_the_list_with_mmap_input_and_count_result_using_recursive_engine(
) -> Result<(), Box<dyn Error>> {
    println ! ("on document heterogenous_list.toml running the query $.a[0] (select the first element on the list) with Input impl MmapInput and result mode CountResult");
    let jsonpath_query = JsonPathQuery::parse("$.a[0]")?;
    let json_file =
        fs::File::open("/home/mat/rsonpath/crates/rsonpath-test/tests/documents/toml/heterogenous_list.json")?;
    let input = unsafe { MmapInput::map_file(&json_file)? };
    let engine = RecursiveEngine::compile_query(&jsonpath_query)?;
    let result = engine.run::<_, CountResult>(&input)?;
    assert_eq!(result.get(), 1u64, "result != expected");
    Ok(())
}
#[test]
fn list_with_mixed_atomic_and_complex_subdocuments_with_query_select_the_first_element_on_the_list_with_mmap_input_and_index_result_using_main_engine(
) -> Result<(), Box<dyn Error>> {
    println ! ("on document heterogenous_list.toml running the query $.a[0] (select the first element on the list) with Input impl MmapInput and result mode IndexResult");
    let jsonpath_query = JsonPathQuery::parse("$.a[0]")?;
    let json_file =
        fs::File::open("/home/mat/rsonpath/crates/rsonpath-test/tests/documents/toml/heterogenous_list.json")?;
    let input = unsafe { MmapInput::map_file(&json_file)? };
    let engine = MainEngine::compile_query(&jsonpath_query)?;
    let result = engine.run::<_, IndexResult>(&input)?;
    assert_eq!(result.get(), vec![15usize,], "result != expected");
    Ok(())
}
#[test]
fn list_with_mixed_atomic_and_complex_subdocuments_with_query_select_the_first_element_on_the_list_with_mmap_input_and_index_result_using_recursive_engine(
) -> Result<(), Box<dyn Error>> {
    println ! ("on document heterogenous_list.toml running the query $.a[0] (select the first element on the list) with Input impl MmapInput and result mode IndexResult");
    let jsonpath_query = JsonPathQuery::parse("$.a[0]")?;
    let json_file =
        fs::File::open("/home/mat/rsonpath/crates/rsonpath-test/tests/documents/toml/heterogenous_list.json")?;
    let input = unsafe { MmapInput::map_file(&json_file)? };
    let engine = RecursiveEngine::compile_query(&jsonpath_query)?;
    let result = engine.run::<_, IndexResult>(&input)?;
    assert_eq!(result.get(), vec![15usize,], "result != expected");
    Ok(())
}
#[test]
fn list_with_mixed_atomic_and_complex_subdocuments_with_query_select_the_first_element_on_the_list_with_owned_bytes_and_count_result_using_main_engine(
) -> Result<(), Box<dyn Error>> {
    println ! ("on document heterogenous_list.toml running the query $.a[0] (select the first element on the list) with Input impl OwnedBytes and result mode CountResult");
    let jsonpath_query = JsonPathQuery::parse("$.a[0]")?;
    let raw_json =
        fs::read_to_string("/home/mat/rsonpath/crates/rsonpath-test/tests/documents/toml/heterogenous_list.json")?;
    let input = OwnedBytes::new(&raw_json.as_bytes())?;
    let engine = MainEngine::compile_query(&jsonpath_query)?;
    let result = engine.run::<_, CountResult>(&input)?;
    assert_eq!(result.get(), 1u64, "result != expected");
    Ok(())
}
#[test]
fn list_with_mixed_atomic_and_complex_subdocuments_with_query_select_the_first_element_on_the_list_with_owned_bytes_and_count_result_using_recursive_engine(
) -> Result<(), Box<dyn Error>> {
    println ! ("on document heterogenous_list.toml running the query $.a[0] (select the first element on the list) with Input impl OwnedBytes and result mode CountResult");
    let jsonpath_query = JsonPathQuery::parse("$.a[0]")?;
    let raw_json =
        fs::read_to_string("/home/mat/rsonpath/crates/rsonpath-test/tests/documents/toml/heterogenous_list.json")?;
    let input = OwnedBytes::new(&raw_json.as_bytes())?;
    let engine = RecursiveEngine::compile_query(&jsonpath_query)?;
    let result = engine.run::<_, CountResult>(&input)?;
    assert_eq!(result.get(), 1u64, "result != expected");
    Ok(())
}
#[test]
fn list_with_mixed_atomic_and_complex_subdocuments_with_query_select_the_first_element_on_the_list_with_owned_bytes_and_index_result_using_main_engine(
) -> Result<(), Box<dyn Error>> {
    println ! ("on document heterogenous_list.toml running the query $.a[0] (select the first element on the list) with Input impl OwnedBytes and result mode IndexResult");
    let jsonpath_query = JsonPathQuery::parse("$.a[0]")?;
    let raw_json =
        fs::read_to_string("/home/mat/rsonpath/crates/rsonpath-test/tests/documents/toml/heterogenous_list.json")?;
    let input = OwnedBytes::new(&raw_json.as_bytes())?;
    let engine = MainEngine::compile_query(&jsonpath_query)?;
    let result = engine.run::<_, IndexResult>(&input)?;
    assert_eq!(result.get(), vec![15usize,], "result != expected");
    Ok(())
}
#[test]
fn list_with_mixed_atomic_and_complex_subdocuments_with_query_select_the_first_element_on_the_list_with_owned_bytes_and_index_result_using_recursive_engine(
) -> Result<(), Box<dyn Error>> {
    println ! ("on document heterogenous_list.toml running the query $.a[0] (select the first element on the list) with Input impl OwnedBytes and result mode IndexResult");
    let jsonpath_query = JsonPathQuery::parse("$.a[0]")?;
    let raw_json =
        fs::read_to_string("/home/mat/rsonpath/crates/rsonpath-test/tests/documents/toml/heterogenous_list.json")?;
    let input = OwnedBytes::new(&raw_json.as_bytes())?;
    let engine = RecursiveEngine::compile_query(&jsonpath_query)?;
    let result = engine.run::<_, IndexResult>(&input)?;
    assert_eq!(result.get(), vec![15usize,], "result != expected");
    Ok(())
}
#[test]
fn list_with_mixed_atomic_and_complex_subdocuments_with_query_select_the_second_element_on_the_list_with_buffered_input_and_count_result_using_main_engine(
) -> Result<(), Box<dyn Error>> {
    println ! ("on document heterogenous_list.toml running the query $.a[1] (select the second element on the list) with Input impl BufferedInput and result mode CountResult");
    let jsonpath_query = JsonPathQuery::parse("$.a[1]")?;
    let json_file =
        fs::File::open("/home/mat/rsonpath/crates/rsonpath-test/tests/documents/toml/heterogenous_list.json")?;
    let input = BufferedInput::new(json_file);
    let engine = MainEngine::compile_query(&jsonpath_query)?;
    let result = engine.run::<_, CountResult>(&input)?;
    assert_eq!(result.get(), 1u64, "result != expected");
    Ok(())
}
#[test]
fn list_with_mixed_atomic_and_complex_subdocuments_with_query_select_the_second_element_on_the_list_with_buffered_input_and_count_result_using_recursive_engine(
) -> Result<(), Box<dyn Error>> {
    println ! ("on document heterogenous_list.toml running the query $.a[1] (select the second element on the list) with Input impl BufferedInput and result mode CountResult");
    let jsonpath_query = JsonPathQuery::parse("$.a[1]")?;
    let json_file =
        fs::File::open("/home/mat/rsonpath/crates/rsonpath-test/tests/documents/toml/heterogenous_list.json")?;
    let input = BufferedInput::new(json_file);
    let engine = RecursiveEngine::compile_query(&jsonpath_query)?;
    let result = engine.run::<_, CountResult>(&input)?;
    assert_eq!(result.get(), 1u64, "result != expected");
    Ok(())
}
#[test]
fn list_with_mixed_atomic_and_complex_subdocuments_with_query_select_the_second_element_on_the_list_with_buffered_input_and_index_result_using_main_engine(
) -> Result<(), Box<dyn Error>> {
    println ! ("on document heterogenous_list.toml running the query $.a[1] (select the second element on the list) with Input impl BufferedInput and result mode IndexResult");
    let jsonpath_query = JsonPathQuery::parse("$.a[1]")?;
    let json_file =
        fs::File::open("/home/mat/rsonpath/crates/rsonpath-test/tests/documents/toml/heterogenous_list.json")?;
    let input = BufferedInput::new(json_file);
    let engine = MainEngine::compile_query(&jsonpath_query)?;
    let result = engine.run::<_, IndexResult>(&input)?;
    assert_eq!(result.get(), vec![23usize,], "result != expected");
    Ok(())
}
#[test]
fn list_with_mixed_atomic_and_complex_subdocuments_with_query_select_the_second_element_on_the_list_with_buffered_input_and_index_result_using_recursive_engine(
) -> Result<(), Box<dyn Error>> {
    println ! ("on document heterogenous_list.toml running the query $.a[1] (select the second element on the list) with Input impl BufferedInput and result mode IndexResult");
    let jsonpath_query = JsonPathQuery::parse("$.a[1]")?;
    let json_file =
        fs::File::open("/home/mat/rsonpath/crates/rsonpath-test/tests/documents/toml/heterogenous_list.json")?;
    let input = BufferedInput::new(json_file);
    let engine = RecursiveEngine::compile_query(&jsonpath_query)?;
    let result = engine.run::<_, IndexResult>(&input)?;
    assert_eq!(result.get(), vec![23usize,], "result != expected");
    Ok(())
}
#[test]
fn list_with_mixed_atomic_and_complex_subdocuments_with_query_select_the_second_element_on_the_list_with_mmap_input_and_count_result_using_main_engine(
) -> Result<(), Box<dyn Error>> {
    println ! ("on document heterogenous_list.toml running the query $.a[1] (select the second element on the list) with Input impl MmapInput and result mode CountResult");
    let jsonpath_query = JsonPathQuery::parse("$.a[1]")?;
    let json_file =
        fs::File::open("/home/mat/rsonpath/crates/rsonpath-test/tests/documents/toml/heterogenous_list.json")?;
    let input = unsafe { MmapInput::map_file(&json_file)? };
    let engine = MainEngine::compile_query(&jsonpath_query)?;
    let result = engine.run::<_, CountResult>(&input)?;
    assert_eq!(result.get(), 1u64, "result != expected");
    Ok(())
}
#[test]
fn list_with_mixed_atomic_and_complex_subdocuments_with_query_select_the_second_element_on_the_list_with_mmap_input_and_count_result_using_recursive_engine(
) -> Result<(), Box<dyn Error>> {
    println ! ("on document heterogenous_list.toml running the query $.a[1] (select the second element on the list) with Input impl MmapInput and result mode CountResult");
    let jsonpath_query = JsonPathQuery::parse("$.a[1]")?;
    let json_file =
        fs::File::open("/home/mat/rsonpath/crates/rsonpath-test/tests/documents/toml/heterogenous_list.json")?;
    let input = unsafe { MmapInput::map_file(&json_file)? };
    let engine = RecursiveEngine::compile_query(&jsonpath_query)?;
    let result = engine.run::<_, CountResult>(&input)?;
    assert_eq!(result.get(), 1u64, "result != expected");
    Ok(())
}
#[test]
fn list_with_mixed_atomic_and_complex_subdocuments_with_query_select_the_second_element_on_the_list_with_mmap_input_and_index_result_using_main_engine(
) -> Result<(), Box<dyn Error>> {
    println ! ("on document heterogenous_list.toml running the query $.a[1] (select the second element on the list) with Input impl MmapInput and result mode IndexResult");
    let jsonpath_query = JsonPathQuery::parse("$.a[1]")?;
    let json_file =
        fs::File::open("/home/mat/rsonpath/crates/rsonpath-test/tests/documents/toml/heterogenous_list.json")?;
    let input = unsafe { MmapInput::map_file(&json_file)? };
    let engine = MainEngine::compile_query(&jsonpath_query)?;
    let result = engine.run::<_, IndexResult>(&input)?;
    assert_eq!(result.get(), vec![23usize,], "result != expected");
    Ok(())
}
#[test]
fn list_with_mixed_atomic_and_complex_subdocuments_with_query_select_the_second_element_on_the_list_with_mmap_input_and_index_result_using_recursive_engine(
) -> Result<(), Box<dyn Error>> {
    println ! ("on document heterogenous_list.toml running the query $.a[1] (select the second element on the list) with Input impl MmapInput and result mode IndexResult");
    let jsonpath_query = JsonPathQuery::parse("$.a[1]")?;
    let json_file =
        fs::File::open("/home/mat/rsonpath/crates/rsonpath-test/tests/documents/toml/heterogenous_list.json")?;
    let input = unsafe { MmapInput::map_file(&json_file)? };
    let engine = RecursiveEngine::compile_query(&jsonpath_query)?;
    let result = engine.run::<_, IndexResult>(&input)?;
    assert_eq!(result.get(), vec![23usize,], "result != expected");
    Ok(())
}
#[test]
fn list_with_mixed_atomic_and_complex_subdocuments_with_query_select_the_second_element_on_the_list_with_owned_bytes_and_count_result_using_main_engine(
) -> Result<(), Box<dyn Error>> {
    println ! ("on document heterogenous_list.toml running the query $.a[1] (select the second element on the list) with Input impl OwnedBytes and result mode CountResult");
    let jsonpath_query = JsonPathQuery::parse("$.a[1]")?;
    let raw_json =
        fs::read_to_string("/home/mat/rsonpath/crates/rsonpath-test/tests/documents/toml/heterogenous_list.json")?;
    let input = OwnedBytes::new(&raw_json.as_bytes())?;
    let engine = MainEngine::compile_query(&jsonpath_query)?;
    let result = engine.run::<_, CountResult>(&input)?;
    assert_eq!(result.get(), 1u64, "result != expected");
    Ok(())
}
#[test]
fn list_with_mixed_atomic_and_complex_subdocuments_with_query_select_the_second_element_on_the_list_with_owned_bytes_and_count_result_using_recursive_engine(
) -> Result<(), Box<dyn Error>> {
    println ! ("on document heterogenous_list.toml running the query $.a[1] (select the second element on the list) with Input impl OwnedBytes and result mode CountResult");
    let jsonpath_query = JsonPathQuery::parse("$.a[1]")?;
    let raw_json =
        fs::read_to_string("/home/mat/rsonpath/crates/rsonpath-test/tests/documents/toml/heterogenous_list.json")?;
    let input = OwnedBytes::new(&raw_json.as_bytes())?;
    let engine = RecursiveEngine::compile_query(&jsonpath_query)?;
    let result = engine.run::<_, CountResult>(&input)?;
    assert_eq!(result.get(), 1u64, "result != expected");
    Ok(())
}
#[test]
fn list_with_mixed_atomic_and_complex_subdocuments_with_query_select_the_second_element_on_the_list_with_owned_bytes_and_index_result_using_main_engine(
) -> Result<(), Box<dyn Error>> {
    println ! ("on document heterogenous_list.toml running the query $.a[1] (select the second element on the list) with Input impl OwnedBytes and result mode IndexResult");
    let jsonpath_query = JsonPathQuery::parse("$.a[1]")?;
    let raw_json =
        fs::read_to_string("/home/mat/rsonpath/crates/rsonpath-test/tests/documents/toml/heterogenous_list.json")?;
    let input = OwnedBytes::new(&raw_json.as_bytes())?;
    let engine = MainEngine::compile_query(&jsonpath_query)?;
    let result = engine.run::<_, IndexResult>(&input)?;
    assert_eq!(result.get(), vec![23usize,], "result != expected");
    Ok(())
}
#[test]
fn list_with_mixed_atomic_and_complex_subdocuments_with_query_select_the_second_element_on_the_list_with_owned_bytes_and_index_result_using_recursive_engine(
) -> Result<(), Box<dyn Error>> {
    println ! ("on document heterogenous_list.toml running the query $.a[1] (select the second element on the list) with Input impl OwnedBytes and result mode IndexResult");
    let jsonpath_query = JsonPathQuery::parse("$.a[1]")?;
    let raw_json =
        fs::read_to_string("/home/mat/rsonpath/crates/rsonpath-test/tests/documents/toml/heterogenous_list.json")?;
    let input = OwnedBytes::new(&raw_json.as_bytes())?;
    let engine = RecursiveEngine::compile_query(&jsonpath_query)?;
    let result = engine.run::<_, IndexResult>(&input)?;
    assert_eq!(result.get(), vec![23usize,], "result != expected");
    Ok(())
}
#[test]
fn list_with_mixed_atomic_and_complex_subdocuments_with_query_select_the_third_element_on_the_list_with_buffered_input_and_count_result_using_main_engine(
) -> Result<(), Box<dyn Error>> {
    println ! ("on document heterogenous_list.toml running the query $.a[2] (select the third element on the list) with Input impl BufferedInput and result mode CountResult");
    let jsonpath_query = JsonPathQuery::parse("$.a[2]")?;
    let json_file =
        fs::File::open("/home/mat/rsonpath/crates/rsonpath-test/tests/documents/toml/heterogenous_list.json")?;
    let input = BufferedInput::new(json_file);
    let engine = MainEngine::compile_query(&jsonpath_query)?;
    let result = engine.run::<_, CountResult>(&input)?;
    assert_eq!(result.get(), 1u64, "result != expected");
    Ok(())
}
#[test]
fn list_with_mixed_atomic_and_complex_subdocuments_with_query_select_the_third_element_on_the_list_with_buffered_input_and_count_result_using_recursive_engine(
) -> Result<(), Box<dyn Error>> {
    println ! ("on document heterogenous_list.toml running the query $.a[2] (select the third element on the list) with Input impl BufferedInput and result mode CountResult");
    let jsonpath_query = JsonPathQuery::parse("$.a[2]")?;
    let json_file =
        fs::File::open("/home/mat/rsonpath/crates/rsonpath-test/tests/documents/toml/heterogenous_list.json")?;
    let input = BufferedInput::new(json_file);
    let engine = RecursiveEngine::compile_query(&jsonpath_query)?;
    let result = engine.run::<_, CountResult>(&input)?;
    assert_eq!(result.get(), 1u64, "result != expected");
    Ok(())
}
#[test]
fn list_with_mixed_atomic_and_complex_subdocuments_with_query_select_the_third_element_on_the_list_with_buffered_input_and_index_result_using_main_engine(
) -> Result<(), Box<dyn Error>> {
    println ! ("on document heterogenous_list.toml running the query $.a[2] (select the third element on the list) with Input impl BufferedInput and result mode IndexResult");
    let jsonpath_query = JsonPathQuery::parse("$.a[2]")?;
    let json_file =
        fs::File::open("/home/mat/rsonpath/crates/rsonpath-test/tests/documents/toml/heterogenous_list.json")?;
    let input = BufferedInput::new(json_file);
    let engine = MainEngine::compile_query(&jsonpath_query)?;
    let result = engine.run::<_, IndexResult>(&input)?;
    assert_eq!(result.get(), vec![50usize,], "result != expected");
    Ok(())
}
#[test]
fn list_with_mixed_atomic_and_complex_subdocuments_with_query_select_the_third_element_on_the_list_with_buffered_input_and_index_result_using_recursive_engine(
) -> Result<(), Box<dyn Error>> {
    println ! ("on document heterogenous_list.toml running the query $.a[2] (select the third element on the list) with Input impl BufferedInput and result mode IndexResult");
    let jsonpath_query = JsonPathQuery::parse("$.a[2]")?;
    let json_file =
        fs::File::open("/home/mat/rsonpath/crates/rsonpath-test/tests/documents/toml/heterogenous_list.json")?;
    let input = BufferedInput::new(json_file);
    let engine = RecursiveEngine::compile_query(&jsonpath_query)?;
    let result = engine.run::<_, IndexResult>(&input)?;
    assert_eq!(result.get(), vec![50usize,], "result != expected");
    Ok(())
}
#[test]
fn list_with_mixed_atomic_and_complex_subdocuments_with_query_select_the_third_element_on_the_list_with_mmap_input_and_count_result_using_main_engine(
) -> Result<(), Box<dyn Error>> {
    println ! ("on document heterogenous_list.toml running the query $.a[2] (select the third element on the list) with Input impl MmapInput and result mode CountResult");
    let jsonpath_query = JsonPathQuery::parse("$.a[2]")?;
    let json_file =
        fs::File::open("/home/mat/rsonpath/crates/rsonpath-test/tests/documents/toml/heterogenous_list.json")?;
    let input = unsafe { MmapInput::map_file(&json_file)? };
    let engine = MainEngine::compile_query(&jsonpath_query)?;
    let result = engine.run::<_, CountResult>(&input)?;
    assert_eq!(result.get(), 1u64, "result != expected");
    Ok(())
}
#[test]
fn list_with_mixed_atomic_and_complex_subdocuments_with_query_select_the_third_element_on_the_list_with_mmap_input_and_count_result_using_recursive_engine(
) -> Result<(), Box<dyn Error>> {
    println ! ("on document heterogenous_list.toml running the query $.a[2] (select the third element on the list) with Input impl MmapInput and result mode CountResult");
    let jsonpath_query = JsonPathQuery::parse("$.a[2]")?;
    let json_file =
        fs::File::open("/home/mat/rsonpath/crates/rsonpath-test/tests/documents/toml/heterogenous_list.json")?;
    let input = unsafe { MmapInput::map_file(&json_file)? };
    let engine = RecursiveEngine::compile_query(&jsonpath_query)?;
    let result = engine.run::<_, CountResult>(&input)?;
    assert_eq!(result.get(), 1u64, "result != expected");
    Ok(())
}
#[test]
fn list_with_mixed_atomic_and_complex_subdocuments_with_query_select_the_third_element_on_the_list_with_mmap_input_and_index_result_using_main_engine(
) -> Result<(), Box<dyn Error>> {
    println ! ("on document heterogenous_list.toml running the query $.a[2] (select the third element on the list) with Input impl MmapInput and result mode IndexResult");
    let jsonpath_query = JsonPathQuery::parse("$.a[2]")?;
    let json_file =
        fs::File::open("/home/mat/rsonpath/crates/rsonpath-test/tests/documents/toml/heterogenous_list.json")?;
    let input = unsafe { MmapInput::map_file(&json_file)? };
    let engine = MainEngine::compile_query(&jsonpath_query)?;
    let result = engine.run::<_, IndexResult>(&input)?;
    assert_eq!(result.get(), vec![50usize,], "result != expected");
    Ok(())
}
#[test]
fn list_with_mixed_atomic_and_complex_subdocuments_with_query_select_the_third_element_on_the_list_with_mmap_input_and_index_result_using_recursive_engine(
) -> Result<(), Box<dyn Error>> {
    println ! ("on document heterogenous_list.toml running the query $.a[2] (select the third element on the list) with Input impl MmapInput and result mode IndexResult");
    let jsonpath_query = JsonPathQuery::parse("$.a[2]")?;
    let json_file =
        fs::File::open("/home/mat/rsonpath/crates/rsonpath-test/tests/documents/toml/heterogenous_list.json")?;
    let input = unsafe { MmapInput::map_file(&json_file)? };
    let engine = RecursiveEngine::compile_query(&jsonpath_query)?;
    let result = engine.run::<_, IndexResult>(&input)?;
    assert_eq!(result.get(), vec![50usize,], "result != expected");
    Ok(())
}
#[test]
fn list_with_mixed_atomic_and_complex_subdocuments_with_query_select_the_third_element_on_the_list_with_owned_bytes_and_count_result_using_main_engine(
) -> Result<(), Box<dyn Error>> {
    println ! ("on document heterogenous_list.toml running the query $.a[2] (select the third element on the list) with Input impl OwnedBytes and result mode CountResult");
    let jsonpath_query = JsonPathQuery::parse("$.a[2]")?;
    let raw_json =
        fs::read_to_string("/home/mat/rsonpath/crates/rsonpath-test/tests/documents/toml/heterogenous_list.json")?;
    let input = OwnedBytes::new(&raw_json.as_bytes())?;
    let engine = MainEngine::compile_query(&jsonpath_query)?;
    let result = engine.run::<_, CountResult>(&input)?;
    assert_eq!(result.get(), 1u64, "result != expected");
    Ok(())
}
#[test]
fn list_with_mixed_atomic_and_complex_subdocuments_with_query_select_the_third_element_on_the_list_with_owned_bytes_and_count_result_using_recursive_engine(
) -> Result<(), Box<dyn Error>> {
    println ! ("on document heterogenous_list.toml running the query $.a[2] (select the third element on the list) with Input impl OwnedBytes and result mode CountResult");
    let jsonpath_query = JsonPathQuery::parse("$.a[2]")?;
    let raw_json =
        fs::read_to_string("/home/mat/rsonpath/crates/rsonpath-test/tests/documents/toml/heterogenous_list.json")?;
    let input = OwnedBytes::new(&raw_json.as_bytes())?;
    let engine = RecursiveEngine::compile_query(&jsonpath_query)?;
    let result = engine.run::<_, CountResult>(&input)?;
    assert_eq!(result.get(), 1u64, "result != expected");
    Ok(())
}
#[test]
fn list_with_mixed_atomic_and_complex_subdocuments_with_query_select_the_third_element_on_the_list_with_owned_bytes_and_index_result_using_main_engine(
) -> Result<(), Box<dyn Error>> {
    println ! ("on document heterogenous_list.toml running the query $.a[2] (select the third element on the list) with Input impl OwnedBytes and result mode IndexResult");
    let jsonpath_query = JsonPathQuery::parse("$.a[2]")?;
    let raw_json =
        fs::read_to_string("/home/mat/rsonpath/crates/rsonpath-test/tests/documents/toml/heterogenous_list.json")?;
    let input = OwnedBytes::new(&raw_json.as_bytes())?;
    let engine = MainEngine::compile_query(&jsonpath_query)?;
    let result = engine.run::<_, IndexResult>(&input)?;
    assert_eq!(result.get(), vec![50usize,], "result != expected");
    Ok(())
}
#[test]
fn list_with_mixed_atomic_and_complex_subdocuments_with_query_select_the_third_element_on_the_list_with_owned_bytes_and_index_result_using_recursive_engine(
) -> Result<(), Box<dyn Error>> {
    println ! ("on document heterogenous_list.toml running the query $.a[2] (select the third element on the list) with Input impl OwnedBytes and result mode IndexResult");
    let jsonpath_query = JsonPathQuery::parse("$.a[2]")?;
    let raw_json =
        fs::read_to_string("/home/mat/rsonpath/crates/rsonpath-test/tests/documents/toml/heterogenous_list.json")?;
    let input = OwnedBytes::new(&raw_json.as_bytes())?;
    let engine = RecursiveEngine::compile_query(&jsonpath_query)?;
    let result = engine.run::<_, IndexResult>(&input)?;
    assert_eq!(result.get(), vec![50usize,], "result != expected");
    Ok(())
}
#[test]
fn member_names_with_multiple_whitespace_characters_before_the_colon_with_query_select_most_nested_elements_with_buffered_input_and_count_result_using_main_engine(
) -> Result<(), Box<dyn Error>> {
    println ! ("on document spaced_colon.toml running the query $..a..b..label (select most nested elements) with Input impl BufferedInput and result mode CountResult");
    let jsonpath_query = JsonPathQuery::parse("$..a..b..label")?;
    let json_file = fs::File::open("/home/mat/rsonpath/crates/rsonpath-test/tests/documents/toml/spaced_colon.json")?;
    let input = BufferedInput::new(json_file);
    let engine = MainEngine::compile_query(&jsonpath_query)?;
    let result = engine.run::<_, CountResult>(&input)?;
    assert_eq!(result.get(), 2u64, "result != expected");
    Ok(())
}
#[test]
fn member_names_with_multiple_whitespace_characters_before_the_colon_with_query_select_most_nested_elements_with_buffered_input_and_count_result_using_recursive_engine(
) -> Result<(), Box<dyn Error>> {
    println ! ("on document spaced_colon.toml running the query $..a..b..label (select most nested elements) with Input impl BufferedInput and result mode CountResult");
    let jsonpath_query = JsonPathQuery::parse("$..a..b..label")?;
    let json_file = fs::File::open("/home/mat/rsonpath/crates/rsonpath-test/tests/documents/toml/spaced_colon.json")?;
    let input = BufferedInput::new(json_file);
    let engine = RecursiveEngine::compile_query(&jsonpath_query)?;
    let result = engine.run::<_, CountResult>(&input)?;
    assert_eq!(result.get(), 2u64, "result != expected");
    Ok(())
}
#[test]
fn member_names_with_multiple_whitespace_characters_before_the_colon_with_query_select_most_nested_elements_with_buffered_input_and_index_result_using_main_engine(
) -> Result<(), Box<dyn Error>> {
    println ! ("on document spaced_colon.toml running the query $..a..b..label (select most nested elements) with Input impl BufferedInput and result mode IndexResult");
    let jsonpath_query = JsonPathQuery::parse("$..a..b..label")?;
    let json_file = fs::File::open("/home/mat/rsonpath/crates/rsonpath-test/tests/documents/toml/spaced_colon.json")?;
    let input = BufferedInput::new(json_file);
    let engine = MainEngine::compile_query(&jsonpath_query)?;
    let result = engine.run::<_, IndexResult>(&input)?;
    assert_eq!(result.get(), vec![108usize, 215usize,], "result != expected");
    Ok(())
}
#[test]
fn member_names_with_multiple_whitespace_characters_before_the_colon_with_query_select_most_nested_elements_with_buffered_input_and_index_result_using_recursive_engine(
) -> Result<(), Box<dyn Error>> {
    println ! ("on document spaced_colon.toml running the query $..a..b..label (select most nested elements) with Input impl BufferedInput and result mode IndexResult");
    let jsonpath_query = JsonPathQuery::parse("$..a..b..label")?;
    let json_file = fs::File::open("/home/mat/rsonpath/crates/rsonpath-test/tests/documents/toml/spaced_colon.json")?;
    let input = BufferedInput::new(json_file);
    let engine = RecursiveEngine::compile_query(&jsonpath_query)?;
    let result = engine.run::<_, IndexResult>(&input)?;
    assert_eq!(result.get(), vec![108usize, 215usize,], "result != expected");
    Ok(())
}
#[test]
fn member_names_with_multiple_whitespace_characters_before_the_colon_with_query_select_most_nested_elements_with_mmap_input_and_count_result_using_main_engine(
) -> Result<(), Box<dyn Error>> {
    println ! ("on document spaced_colon.toml running the query $..a..b..label (select most nested elements) with Input impl MmapInput and result mode CountResult");
    let jsonpath_query = JsonPathQuery::parse("$..a..b..label")?;
    let json_file = fs::File::open("/home/mat/rsonpath/crates/rsonpath-test/tests/documents/toml/spaced_colon.json")?;
    let input = unsafe { MmapInput::map_file(&json_file)? };
    let engine = MainEngine::compile_query(&jsonpath_query)?;
    let result = engine.run::<_, CountResult>(&input)?;
    assert_eq!(result.get(), 2u64, "result != expected");
    Ok(())
}
#[test]
fn member_names_with_multiple_whitespace_characters_before_the_colon_with_query_select_most_nested_elements_with_mmap_input_and_count_result_using_recursive_engine(
) -> Result<(), Box<dyn Error>> {
    println ! ("on document spaced_colon.toml running the query $..a..b..label (select most nested elements) with Input impl MmapInput and result mode CountResult");
    let jsonpath_query = JsonPathQuery::parse("$..a..b..label")?;
    let json_file = fs::File::open("/home/mat/rsonpath/crates/rsonpath-test/tests/documents/toml/spaced_colon.json")?;
    let input = unsafe { MmapInput::map_file(&json_file)? };
    let engine = RecursiveEngine::compile_query(&jsonpath_query)?;
    let result = engine.run::<_, CountResult>(&input)?;
    assert_eq!(result.get(), 2u64, "result != expected");
    Ok(())
}
#[test]
fn member_names_with_multiple_whitespace_characters_before_the_colon_with_query_select_most_nested_elements_with_mmap_input_and_index_result_using_main_engine(
) -> Result<(), Box<dyn Error>> {
    println ! ("on document spaced_colon.toml running the query $..a..b..label (select most nested elements) with Input impl MmapInput and result mode IndexResult");
    let jsonpath_query = JsonPathQuery::parse("$..a..b..label")?;
    let json_file = fs::File::open("/home/mat/rsonpath/crates/rsonpath-test/tests/documents/toml/spaced_colon.json")?;
    let input = unsafe { MmapInput::map_file(&json_file)? };
    let engine = MainEngine::compile_query(&jsonpath_query)?;
    let result = engine.run::<_, IndexResult>(&input)?;
    assert_eq!(result.get(), vec![108usize, 215usize,], "result != expected");
    Ok(())
}
#[test]
fn member_names_with_multiple_whitespace_characters_before_the_colon_with_query_select_most_nested_elements_with_mmap_input_and_index_result_using_recursive_engine(
) -> Result<(), Box<dyn Error>> {
    println ! ("on document spaced_colon.toml running the query $..a..b..label (select most nested elements) with Input impl MmapInput and result mode IndexResult");
    let jsonpath_query = JsonPathQuery::parse("$..a..b..label")?;
    let json_file = fs::File::open("/home/mat/rsonpath/crates/rsonpath-test/tests/documents/toml/spaced_colon.json")?;
    let input = unsafe { MmapInput::map_file(&json_file)? };
    let engine = RecursiveEngine::compile_query(&jsonpath_query)?;
    let result = engine.run::<_, IndexResult>(&input)?;
    assert_eq!(result.get(), vec![108usize, 215usize,], "result != expected");
    Ok(())
}
#[test]
fn member_names_with_multiple_whitespace_characters_before_the_colon_with_query_select_most_nested_elements_with_owned_bytes_and_count_result_using_main_engine(
) -> Result<(), Box<dyn Error>> {
    println ! ("on document spaced_colon.toml running the query $..a..b..label (select most nested elements) with Input impl OwnedBytes and result mode CountResult");
    let jsonpath_query = JsonPathQuery::parse("$..a..b..label")?;
    let raw_json =
        fs::read_to_string("/home/mat/rsonpath/crates/rsonpath-test/tests/documents/toml/spaced_colon.json")?;
    let input = OwnedBytes::new(&raw_json.as_bytes())?;
    let engine = MainEngine::compile_query(&jsonpath_query)?;
    let result = engine.run::<_, CountResult>(&input)?;
    assert_eq!(result.get(), 2u64, "result != expected");
    Ok(())
}
#[test]
fn member_names_with_multiple_whitespace_characters_before_the_colon_with_query_select_most_nested_elements_with_owned_bytes_and_count_result_using_recursive_engine(
) -> Result<(), Box<dyn Error>> {
    println ! ("on document spaced_colon.toml running the query $..a..b..label (select most nested elements) with Input impl OwnedBytes and result mode CountResult");
    let jsonpath_query = JsonPathQuery::parse("$..a..b..label")?;
    let raw_json =
        fs::read_to_string("/home/mat/rsonpath/crates/rsonpath-test/tests/documents/toml/spaced_colon.json")?;
    let input = OwnedBytes::new(&raw_json.as_bytes())?;
    let engine = RecursiveEngine::compile_query(&jsonpath_query)?;
    let result = engine.run::<_, CountResult>(&input)?;
    assert_eq!(result.get(), 2u64, "result != expected");
    Ok(())
}
#[test]
fn member_names_with_multiple_whitespace_characters_before_the_colon_with_query_select_most_nested_elements_with_owned_bytes_and_index_result_using_main_engine(
) -> Result<(), Box<dyn Error>> {
    println ! ("on document spaced_colon.toml running the query $..a..b..label (select most nested elements) with Input impl OwnedBytes and result mode IndexResult");
    let jsonpath_query = JsonPathQuery::parse("$..a..b..label")?;
    let raw_json =
        fs::read_to_string("/home/mat/rsonpath/crates/rsonpath-test/tests/documents/toml/spaced_colon.json")?;
    let input = OwnedBytes::new(&raw_json.as_bytes())?;
    let engine = MainEngine::compile_query(&jsonpath_query)?;
    let result = engine.run::<_, IndexResult>(&input)?;
    assert_eq!(result.get(), vec![108usize, 215usize,], "result != expected");
    Ok(())
}
#[test]
fn member_names_with_multiple_whitespace_characters_before_the_colon_with_query_select_most_nested_elements_with_owned_bytes_and_index_result_using_recursive_engine(
) -> Result<(), Box<dyn Error>> {
    println ! ("on document spaced_colon.toml running the query $..a..b..label (select most nested elements) with Input impl OwnedBytes and result mode IndexResult");
    let jsonpath_query = JsonPathQuery::parse("$..a..b..label")?;
    let raw_json =
        fs::read_to_string("/home/mat/rsonpath/crates/rsonpath-test/tests/documents/toml/spaced_colon.json")?;
    let input = OwnedBytes::new(&raw_json.as_bytes())?;
    let engine = RecursiveEngine::compile_query(&jsonpath_query)?;
    let result = engine.run::<_, IndexResult>(&input)?;
    assert_eq!(result.get(), vec![108usize, 215usize,], "result != expected");
    Ok(())
}
#[test]
fn nested_lists_with_a_few_zero_atoms_with_query_path_0_then_any_with_buffered_input_and_count_result_using_main_engine(
) -> Result<(), Box<dyn Error>> {
    println ! ("on document nested_arrays.toml running the query $[0].* (path 0, then any) with Input impl BufferedInput and result mode CountResult");
    let jsonpath_query = JsonPathQuery::parse("$[0].*")?;
    let json_file = fs::File::open("/home/mat/rsonpath/crates/rsonpath-test/tests/documents/toml/nested_arrays.json")?;
    let input = BufferedInput::new(json_file);
    let engine = MainEngine::compile_query(&jsonpath_query)?;
    let result = engine.run::<_, CountResult>(&input)?;
    assert_eq!(result.get(), 1u64, "result != expected");
    Ok(())
}
#[test]
fn nested_lists_with_a_few_zero_atoms_with_query_path_0_then_any_with_buffered_input_and_count_result_using_recursive_engine(
) -> Result<(), Box<dyn Error>> {
    println ! ("on document nested_arrays.toml running the query $[0].* (path 0, then any) with Input impl BufferedInput and result mode CountResult");
    let jsonpath_query = JsonPathQuery::parse("$[0].*")?;
    let json_file = fs::File::open("/home/mat/rsonpath/crates/rsonpath-test/tests/documents/toml/nested_arrays.json")?;
    let input = BufferedInput::new(json_file);
    let engine = RecursiveEngine::compile_query(&jsonpath_query)?;
    let result = engine.run::<_, CountResult>(&input)?;
    assert_eq!(result.get(), 1u64, "result != expected");
    Ok(())
}
#[test]
fn nested_lists_with_a_few_zero_atoms_with_query_path_0_then_any_with_buffered_input_and_index_result_using_main_engine(
) -> Result<(), Box<dyn Error>> {
    println ! ("on document nested_arrays.toml running the query $[0].* (path 0, then any) with Input impl BufferedInput and result mode IndexResult");
    let jsonpath_query = JsonPathQuery::parse("$[0].*")?;
    let json_file = fs::File::open("/home/mat/rsonpath/crates/rsonpath-test/tests/documents/toml/nested_arrays.json")?;
    let input = BufferedInput::new(json_file);
    let engine = MainEngine::compile_query(&jsonpath_query)?;
    let result = engine.run::<_, IndexResult>(&input)?;
    assert_eq!(result.get(), vec![16usize,], "result != expected");
    Ok(())
}
#[test]
fn nested_lists_with_a_few_zero_atoms_with_query_path_0_then_any_with_buffered_input_and_index_result_using_recursive_engine(
) -> Result<(), Box<dyn Error>> {
    println ! ("on document nested_arrays.toml running the query $[0].* (path 0, then any) with Input impl BufferedInput and result mode IndexResult");
    let jsonpath_query = JsonPathQuery::parse("$[0].*")?;
    let json_file = fs::File::open("/home/mat/rsonpath/crates/rsonpath-test/tests/documents/toml/nested_arrays.json")?;
    let input = BufferedInput::new(json_file);
    let engine = RecursiveEngine::compile_query(&jsonpath_query)?;
    let result = engine.run::<_, IndexResult>(&input)?;
    assert_eq!(result.get(), vec![16usize,], "result != expected");
    Ok(())
}
#[test]
fn nested_lists_with_a_few_zero_atoms_with_query_path_0_then_any_with_mmap_input_and_count_result_using_main_engine(
) -> Result<(), Box<dyn Error>> {
    println ! ("on document nested_arrays.toml running the query $[0].* (path 0, then any) with Input impl MmapInput and result mode CountResult");
    let jsonpath_query = JsonPathQuery::parse("$[0].*")?;
    let json_file = fs::File::open("/home/mat/rsonpath/crates/rsonpath-test/tests/documents/toml/nested_arrays.json")?;
    let input = unsafe { MmapInput::map_file(&json_file)? };
    let engine = MainEngine::compile_query(&jsonpath_query)?;
    let result = engine.run::<_, CountResult>(&input)?;
    assert_eq!(result.get(), 1u64, "result != expected");
    Ok(())
}
#[test]
fn nested_lists_with_a_few_zero_atoms_with_query_path_0_then_any_with_mmap_input_and_count_result_using_recursive_engine(
) -> Result<(), Box<dyn Error>> {
    println ! ("on document nested_arrays.toml running the query $[0].* (path 0, then any) with Input impl MmapInput and result mode CountResult");
    let jsonpath_query = JsonPathQuery::parse("$[0].*")?;
    let json_file = fs::File::open("/home/mat/rsonpath/crates/rsonpath-test/tests/documents/toml/nested_arrays.json")?;
    let input = unsafe { MmapInput::map_file(&json_file)? };
    let engine = RecursiveEngine::compile_query(&jsonpath_query)?;
    let result = engine.run::<_, CountResult>(&input)?;
    assert_eq!(result.get(), 1u64, "result != expected");
    Ok(())
}
#[test]
fn nested_lists_with_a_few_zero_atoms_with_query_path_0_then_any_with_mmap_input_and_index_result_using_main_engine(
) -> Result<(), Box<dyn Error>> {
    println ! ("on document nested_arrays.toml running the query $[0].* (path 0, then any) with Input impl MmapInput and result mode IndexResult");
    let jsonpath_query = JsonPathQuery::parse("$[0].*")?;
    let json_file = fs::File::open("/home/mat/rsonpath/crates/rsonpath-test/tests/documents/toml/nested_arrays.json")?;
    let input = unsafe { MmapInput::map_file(&json_file)? };
    let engine = MainEngine::compile_query(&jsonpath_query)?;
    let result = engine.run::<_, IndexResult>(&input)?;
    assert_eq!(result.get(), vec![16usize,], "result != expected");
    Ok(())
}
#[test]
fn nested_lists_with_a_few_zero_atoms_with_query_path_0_then_any_with_mmap_input_and_index_result_using_recursive_engine(
) -> Result<(), Box<dyn Error>> {
    println ! ("on document nested_arrays.toml running the query $[0].* (path 0, then any) with Input impl MmapInput and result mode IndexResult");
    let jsonpath_query = JsonPathQuery::parse("$[0].*")?;
    let json_file = fs::File::open("/home/mat/rsonpath/crates/rsonpath-test/tests/documents/toml/nested_arrays.json")?;
    let input = unsafe { MmapInput::map_file(&json_file)? };
    let engine = RecursiveEngine::compile_query(&jsonpath_query)?;
    let result = engine.run::<_, IndexResult>(&input)?;
    assert_eq!(result.get(), vec![16usize,], "result != expected");
    Ok(())
}
#[test]
fn nested_lists_with_a_few_zero_atoms_with_query_path_0_then_any_with_owned_bytes_and_count_result_using_main_engine(
) -> Result<(), Box<dyn Error>> {
    println ! ("on document nested_arrays.toml running the query $[0].* (path 0, then any) with Input impl OwnedBytes and result mode CountResult");
    let jsonpath_query = JsonPathQuery::parse("$[0].*")?;
    let raw_json =
        fs::read_to_string("/home/mat/rsonpath/crates/rsonpath-test/tests/documents/toml/nested_arrays.json")?;
    let input = OwnedBytes::new(&raw_json.as_bytes())?;
    let engine = MainEngine::compile_query(&jsonpath_query)?;
    let result = engine.run::<_, CountResult>(&input)?;
    assert_eq!(result.get(), 1u64, "result != expected");
    Ok(())
}
#[test]
fn nested_lists_with_a_few_zero_atoms_with_query_path_0_then_any_with_owned_bytes_and_count_result_using_recursive_engine(
) -> Result<(), Box<dyn Error>> {
    println ! ("on document nested_arrays.toml running the query $[0].* (path 0, then any) with Input impl OwnedBytes and result mode CountResult");
    let jsonpath_query = JsonPathQuery::parse("$[0].*")?;
    let raw_json =
        fs::read_to_string("/home/mat/rsonpath/crates/rsonpath-test/tests/documents/toml/nested_arrays.json")?;
    let input = OwnedBytes::new(&raw_json.as_bytes())?;
    let engine = RecursiveEngine::compile_query(&jsonpath_query)?;
    let result = engine.run::<_, CountResult>(&input)?;
    assert_eq!(result.get(), 1u64, "result != expected");
    Ok(())
}
#[test]
fn nested_lists_with_a_few_zero_atoms_with_query_path_0_then_any_with_owned_bytes_and_index_result_using_main_engine(
) -> Result<(), Box<dyn Error>> {
    println ! ("on document nested_arrays.toml running the query $[0].* (path 0, then any) with Input impl OwnedBytes and result mode IndexResult");
    let jsonpath_query = JsonPathQuery::parse("$[0].*")?;
    let raw_json =
        fs::read_to_string("/home/mat/rsonpath/crates/rsonpath-test/tests/documents/toml/nested_arrays.json")?;
    let input = OwnedBytes::new(&raw_json.as_bytes())?;
    let engine = MainEngine::compile_query(&jsonpath_query)?;
    let result = engine.run::<_, IndexResult>(&input)?;
    assert_eq!(result.get(), vec![16usize,], "result != expected");
    Ok(())
}
#[test]
fn nested_lists_with_a_few_zero_atoms_with_query_path_0_then_any_with_owned_bytes_and_index_result_using_recursive_engine(
) -> Result<(), Box<dyn Error>> {
    println ! ("on document nested_arrays.toml running the query $[0].* (path 0, then any) with Input impl OwnedBytes and result mode IndexResult");
    let jsonpath_query = JsonPathQuery::parse("$[0].*")?;
    let raw_json =
        fs::read_to_string("/home/mat/rsonpath/crates/rsonpath-test/tests/documents/toml/nested_arrays.json")?;
    let input = OwnedBytes::new(&raw_json.as_bytes())?;
    let engine = RecursiveEngine::compile_query(&jsonpath_query)?;
    let result = engine.run::<_, IndexResult>(&input)?;
    assert_eq!(result.get(), vec![16usize,], "result != expected");
    Ok(())
}
#[test]
fn nested_lists_with_a_few_zero_atoms_with_query_path_0_with_buffered_input_and_count_result_using_main_engine(
) -> Result<(), Box<dyn Error>> {
    println ! ("on document nested_arrays.toml running the query $[0] (path 0) with Input impl BufferedInput and result mode CountResult");
    let jsonpath_query = JsonPathQuery::parse("$[0]")?;
    let json_file = fs::File::open("/home/mat/rsonpath/crates/rsonpath-test/tests/documents/toml/nested_arrays.json")?;
    let input = BufferedInput::new(json_file);
    let engine = MainEngine::compile_query(&jsonpath_query)?;
    let result = engine.run::<_, CountResult>(&input)?;
    assert_eq!(result.get(), 1u64, "result != expected");
    Ok(())
}
#[test]
fn nested_lists_with_a_few_zero_atoms_with_query_path_0_with_buffered_input_and_count_result_using_recursive_engine(
) -> Result<(), Box<dyn Error>> {
    println ! ("on document nested_arrays.toml running the query $[0] (path 0) with Input impl BufferedInput and result mode CountResult");
    let jsonpath_query = JsonPathQuery::parse("$[0]")?;
    let json_file = fs::File::open("/home/mat/rsonpath/crates/rsonpath-test/tests/documents/toml/nested_arrays.json")?;
    let input = BufferedInput::new(json_file);
    let engine = RecursiveEngine::compile_query(&jsonpath_query)?;
    let result = engine.run::<_, CountResult>(&input)?;
    assert_eq!(result.get(), 1u64, "result != expected");
    Ok(())
}
#[test]
fn nested_lists_with_a_few_zero_atoms_with_query_path_0_with_buffered_input_and_index_result_using_main_engine(
) -> Result<(), Box<dyn Error>> {
    println ! ("on document nested_arrays.toml running the query $[0] (path 0) with Input impl BufferedInput and result mode IndexResult");
    let jsonpath_query = JsonPathQuery::parse("$[0]")?;
    let json_file = fs::File::open("/home/mat/rsonpath/crates/rsonpath-test/tests/documents/toml/nested_arrays.json")?;
    let input = BufferedInput::new(json_file);
    let engine = MainEngine::compile_query(&jsonpath_query)?;
    let result = engine.run::<_, IndexResult>(&input)?;
    assert_eq!(result.get(), vec![6usize,], "result != expected");
    Ok(())
}
#[test]
fn nested_lists_with_a_few_zero_atoms_with_query_path_0_with_buffered_input_and_index_result_using_recursive_engine(
) -> Result<(), Box<dyn Error>> {
    println ! ("on document nested_arrays.toml running the query $[0] (path 0) with Input impl BufferedInput and result mode IndexResult");
    let jsonpath_query = JsonPathQuery::parse("$[0]")?;
    let json_file = fs::File::open("/home/mat/rsonpath/crates/rsonpath-test/tests/documents/toml/nested_arrays.json")?;
    let input = BufferedInput::new(json_file);
    let engine = RecursiveEngine::compile_query(&jsonpath_query)?;
    let result = engine.run::<_, IndexResult>(&input)?;
    assert_eq!(result.get(), vec![6usize,], "result != expected");
    Ok(())
}
#[test]
fn nested_lists_with_a_few_zero_atoms_with_query_path_0_with_mmap_input_and_count_result_using_main_engine(
) -> Result<(), Box<dyn Error>> {
    println ! ("on document nested_arrays.toml running the query $[0] (path 0) with Input impl MmapInput and result mode CountResult");
    let jsonpath_query = JsonPathQuery::parse("$[0]")?;
    let json_file = fs::File::open("/home/mat/rsonpath/crates/rsonpath-test/tests/documents/toml/nested_arrays.json")?;
    let input = unsafe { MmapInput::map_file(&json_file)? };
    let engine = MainEngine::compile_query(&jsonpath_query)?;
    let result = engine.run::<_, CountResult>(&input)?;
    assert_eq!(result.get(), 1u64, "result != expected");
    Ok(())
}
#[test]
fn nested_lists_with_a_few_zero_atoms_with_query_path_0_with_mmap_input_and_count_result_using_recursive_engine(
) -> Result<(), Box<dyn Error>> {
    println ! ("on document nested_arrays.toml running the query $[0] (path 0) with Input impl MmapInput and result mode CountResult");
    let jsonpath_query = JsonPathQuery::parse("$[0]")?;
    let json_file = fs::File::open("/home/mat/rsonpath/crates/rsonpath-test/tests/documents/toml/nested_arrays.json")?;
    let input = unsafe { MmapInput::map_file(&json_file)? };
    let engine = RecursiveEngine::compile_query(&jsonpath_query)?;
    let result = engine.run::<_, CountResult>(&input)?;
    assert_eq!(result.get(), 1u64, "result != expected");
    Ok(())
}
#[test]
fn nested_lists_with_a_few_zero_atoms_with_query_path_0_with_mmap_input_and_index_result_using_main_engine(
) -> Result<(), Box<dyn Error>> {
    println ! ("on document nested_arrays.toml running the query $[0] (path 0) with Input impl MmapInput and result mode IndexResult");
    let jsonpath_query = JsonPathQuery::parse("$[0]")?;
    let json_file = fs::File::open("/home/mat/rsonpath/crates/rsonpath-test/tests/documents/toml/nested_arrays.json")?;
    let input = unsafe { MmapInput::map_file(&json_file)? };
    let engine = MainEngine::compile_query(&jsonpath_query)?;
    let result = engine.run::<_, IndexResult>(&input)?;
    assert_eq!(result.get(), vec![6usize,], "result != expected");
    Ok(())
}
#[test]
fn nested_lists_with_a_few_zero_atoms_with_query_path_0_with_mmap_input_and_index_result_using_recursive_engine(
) -> Result<(), Box<dyn Error>> {
    println ! ("on document nested_arrays.toml running the query $[0] (path 0) with Input impl MmapInput and result mode IndexResult");
    let jsonpath_query = JsonPathQuery::parse("$[0]")?;
    let json_file = fs::File::open("/home/mat/rsonpath/crates/rsonpath-test/tests/documents/toml/nested_arrays.json")?;
    let input = unsafe { MmapInput::map_file(&json_file)? };
    let engine = RecursiveEngine::compile_query(&jsonpath_query)?;
    let result = engine.run::<_, IndexResult>(&input)?;
    assert_eq!(result.get(), vec![6usize,], "result != expected");
    Ok(())
}
#[test]
fn nested_lists_with_a_few_zero_atoms_with_query_path_0_with_owned_bytes_and_count_result_using_main_engine(
) -> Result<(), Box<dyn Error>> {
    println ! ("on document nested_arrays.toml running the query $[0] (path 0) with Input impl OwnedBytes and result mode CountResult");
    let jsonpath_query = JsonPathQuery::parse("$[0]")?;
    let raw_json =
        fs::read_to_string("/home/mat/rsonpath/crates/rsonpath-test/tests/documents/toml/nested_arrays.json")?;
    let input = OwnedBytes::new(&raw_json.as_bytes())?;
    let engine = MainEngine::compile_query(&jsonpath_query)?;
    let result = engine.run::<_, CountResult>(&input)?;
    assert_eq!(result.get(), 1u64, "result != expected");
    Ok(())
}
#[test]
fn nested_lists_with_a_few_zero_atoms_with_query_path_0_with_owned_bytes_and_count_result_using_recursive_engine(
) -> Result<(), Box<dyn Error>> {
    println ! ("on document nested_arrays.toml running the query $[0] (path 0) with Input impl OwnedBytes and result mode CountResult");
    let jsonpath_query = JsonPathQuery::parse("$[0]")?;
    let raw_json =
        fs::read_to_string("/home/mat/rsonpath/crates/rsonpath-test/tests/documents/toml/nested_arrays.json")?;
    let input = OwnedBytes::new(&raw_json.as_bytes())?;
    let engine = RecursiveEngine::compile_query(&jsonpath_query)?;
    let result = engine.run::<_, CountResult>(&input)?;
    assert_eq!(result.get(), 1u64, "result != expected");
    Ok(())
}
#[test]
fn nested_lists_with_a_few_zero_atoms_with_query_path_0_with_owned_bytes_and_index_result_using_main_engine(
) -> Result<(), Box<dyn Error>> {
    println ! ("on document nested_arrays.toml running the query $[0] (path 0) with Input impl OwnedBytes and result mode IndexResult");
    let jsonpath_query = JsonPathQuery::parse("$[0]")?;
    let raw_json =
        fs::read_to_string("/home/mat/rsonpath/crates/rsonpath-test/tests/documents/toml/nested_arrays.json")?;
    let input = OwnedBytes::new(&raw_json.as_bytes())?;
    let engine = MainEngine::compile_query(&jsonpath_query)?;
    let result = engine.run::<_, IndexResult>(&input)?;
    assert_eq!(result.get(), vec![6usize,], "result != expected");
    Ok(())
}
#[test]
fn nested_lists_with_a_few_zero_atoms_with_query_path_0_with_owned_bytes_and_index_result_using_recursive_engine(
) -> Result<(), Box<dyn Error>> {
    println ! ("on document nested_arrays.toml running the query $[0] (path 0) with Input impl OwnedBytes and result mode IndexResult");
    let jsonpath_query = JsonPathQuery::parse("$[0]")?;
    let raw_json =
        fs::read_to_string("/home/mat/rsonpath/crates/rsonpath-test/tests/documents/toml/nested_arrays.json")?;
    let input = OwnedBytes::new(&raw_json.as_bytes())?;
    let engine = RecursiveEngine::compile_query(&jsonpath_query)?;
    let result = engine.run::<_, IndexResult>(&input)?;
    assert_eq!(result.get(), vec![6usize,], "result != expected");
    Ok(())
}
#[test]
fn nested_lists_with_a_few_zero_atoms_with_query_path_2_0_with_buffered_input_and_count_result_using_main_engine(
) -> Result<(), Box<dyn Error>> {
    println ! ("on document nested_arrays.toml running the query $[2][0] (path 2 0) with Input impl BufferedInput and result mode CountResult");
    let jsonpath_query = JsonPathQuery::parse("$[2][0]")?;
    let json_file = fs::File::open("/home/mat/rsonpath/crates/rsonpath-test/tests/documents/toml/nested_arrays.json")?;
    let input = BufferedInput::new(json_file);
    let engine = MainEngine::compile_query(&jsonpath_query)?;
    let result = engine.run::<_, CountResult>(&input)?;
    assert_eq!(result.get(), 1u64, "result != expected");
    Ok(())
}
#[test]
fn nested_lists_with_a_few_zero_atoms_with_query_path_2_0_with_buffered_input_and_count_result_using_recursive_engine(
) -> Result<(), Box<dyn Error>> {
    println ! ("on document nested_arrays.toml running the query $[2][0] (path 2 0) with Input impl BufferedInput and result mode CountResult");
    let jsonpath_query = JsonPathQuery::parse("$[2][0]")?;
    let json_file = fs::File::open("/home/mat/rsonpath/crates/rsonpath-test/tests/documents/toml/nested_arrays.json")?;
    let input = BufferedInput::new(json_file);
    let engine = RecursiveEngine::compile_query(&jsonpath_query)?;
    let result = engine.run::<_, CountResult>(&input)?;
    assert_eq!(result.get(), 1u64, "result != expected");
    Ok(())
}
#[test]
fn nested_lists_with_a_few_zero_atoms_with_query_path_2_0_with_buffered_input_and_index_result_using_main_engine(
) -> Result<(), Box<dyn Error>> {
    println ! ("on document nested_arrays.toml running the query $[2][0] (path 2 0) with Input impl BufferedInput and result mode IndexResult");
    let jsonpath_query = JsonPathQuery::parse("$[2][0]")?;
    let json_file = fs::File::open("/home/mat/rsonpath/crates/rsonpath-test/tests/documents/toml/nested_arrays.json")?;
    let input = BufferedInput::new(json_file);
    let engine = MainEngine::compile_query(&jsonpath_query)?;
    let result = engine.run::<_, IndexResult>(&input)?;
    assert_eq!(result.get(), vec![49usize,], "result != expected");
    Ok(())
}
#[test]
fn nested_lists_with_a_few_zero_atoms_with_query_path_2_0_with_buffered_input_and_index_result_using_recursive_engine(
) -> Result<(), Box<dyn Error>> {
    println ! ("on document nested_arrays.toml running the query $[2][0] (path 2 0) with Input impl BufferedInput and result mode IndexResult");
    let jsonpath_query = JsonPathQuery::parse("$[2][0]")?;
    let json_file = fs::File::open("/home/mat/rsonpath/crates/rsonpath-test/tests/documents/toml/nested_arrays.json")?;
    let input = BufferedInput::new(json_file);
    let engine = RecursiveEngine::compile_query(&jsonpath_query)?;
    let result = engine.run::<_, IndexResult>(&input)?;
    assert_eq!(result.get(), vec![49usize,], "result != expected");
    Ok(())
}
#[test]
fn nested_lists_with_a_few_zero_atoms_with_query_path_2_0_with_mmap_input_and_count_result_using_main_engine(
) -> Result<(), Box<dyn Error>> {
    println ! ("on document nested_arrays.toml running the query $[2][0] (path 2 0) with Input impl MmapInput and result mode CountResult");
    let jsonpath_query = JsonPathQuery::parse("$[2][0]")?;
    let json_file = fs::File::open("/home/mat/rsonpath/crates/rsonpath-test/tests/documents/toml/nested_arrays.json")?;
    let input = unsafe { MmapInput::map_file(&json_file)? };
    let engine = MainEngine::compile_query(&jsonpath_query)?;
    let result = engine.run::<_, CountResult>(&input)?;
    assert_eq!(result.get(), 1u64, "result != expected");
    Ok(())
}
#[test]
fn nested_lists_with_a_few_zero_atoms_with_query_path_2_0_with_mmap_input_and_count_result_using_recursive_engine(
) -> Result<(), Box<dyn Error>> {
    println ! ("on document nested_arrays.toml running the query $[2][0] (path 2 0) with Input impl MmapInput and result mode CountResult");
    let jsonpath_query = JsonPathQuery::parse("$[2][0]")?;
    let json_file = fs::File::open("/home/mat/rsonpath/crates/rsonpath-test/tests/documents/toml/nested_arrays.json")?;
    let input = unsafe { MmapInput::map_file(&json_file)? };
    let engine = RecursiveEngine::compile_query(&jsonpath_query)?;
    let result = engine.run::<_, CountResult>(&input)?;
    assert_eq!(result.get(), 1u64, "result != expected");
    Ok(())
}
#[test]
fn nested_lists_with_a_few_zero_atoms_with_query_path_2_0_with_mmap_input_and_index_result_using_main_engine(
) -> Result<(), Box<dyn Error>> {
    println ! ("on document nested_arrays.toml running the query $[2][0] (path 2 0) with Input impl MmapInput and result mode IndexResult");
    let jsonpath_query = JsonPathQuery::parse("$[2][0]")?;
    let json_file = fs::File::open("/home/mat/rsonpath/crates/rsonpath-test/tests/documents/toml/nested_arrays.json")?;
    let input = unsafe { MmapInput::map_file(&json_file)? };
    let engine = MainEngine::compile_query(&jsonpath_query)?;
    let result = engine.run::<_, IndexResult>(&input)?;
    assert_eq!(result.get(), vec![49usize,], "result != expected");
    Ok(())
}
#[test]
fn nested_lists_with_a_few_zero_atoms_with_query_path_2_0_with_mmap_input_and_index_result_using_recursive_engine(
) -> Result<(), Box<dyn Error>> {
    println ! ("on document nested_arrays.toml running the query $[2][0] (path 2 0) with Input impl MmapInput and result mode IndexResult");
    let jsonpath_query = JsonPathQuery::parse("$[2][0]")?;
    let json_file = fs::File::open("/home/mat/rsonpath/crates/rsonpath-test/tests/documents/toml/nested_arrays.json")?;
    let input = unsafe { MmapInput::map_file(&json_file)? };
    let engine = RecursiveEngine::compile_query(&jsonpath_query)?;
    let result = engine.run::<_, IndexResult>(&input)?;
    assert_eq!(result.get(), vec![49usize,], "result != expected");
    Ok(())
}
#[test]
fn nested_lists_with_a_few_zero_atoms_with_query_path_2_0_with_owned_bytes_and_count_result_using_main_engine(
) -> Result<(), Box<dyn Error>> {
    println ! ("on document nested_arrays.toml running the query $[2][0] (path 2 0) with Input impl OwnedBytes and result mode CountResult");
    let jsonpath_query = JsonPathQuery::parse("$[2][0]")?;
    let raw_json =
        fs::read_to_string("/home/mat/rsonpath/crates/rsonpath-test/tests/documents/toml/nested_arrays.json")?;
    let input = OwnedBytes::new(&raw_json.as_bytes())?;
    let engine = MainEngine::compile_query(&jsonpath_query)?;
    let result = engine.run::<_, CountResult>(&input)?;
    assert_eq!(result.get(), 1u64, "result != expected");
    Ok(())
}
#[test]
fn nested_lists_with_a_few_zero_atoms_with_query_path_2_0_with_owned_bytes_and_count_result_using_recursive_engine(
) -> Result<(), Box<dyn Error>> {
    println ! ("on document nested_arrays.toml running the query $[2][0] (path 2 0) with Input impl OwnedBytes and result mode CountResult");
    let jsonpath_query = JsonPathQuery::parse("$[2][0]")?;
    let raw_json =
        fs::read_to_string("/home/mat/rsonpath/crates/rsonpath-test/tests/documents/toml/nested_arrays.json")?;
    let input = OwnedBytes::new(&raw_json.as_bytes())?;
    let engine = RecursiveEngine::compile_query(&jsonpath_query)?;
    let result = engine.run::<_, CountResult>(&input)?;
    assert_eq!(result.get(), 1u64, "result != expected");
    Ok(())
}
#[test]
fn nested_lists_with_a_few_zero_atoms_with_query_path_2_0_with_owned_bytes_and_index_result_using_main_engine(
) -> Result<(), Box<dyn Error>> {
    println ! ("on document nested_arrays.toml running the query $[2][0] (path 2 0) with Input impl OwnedBytes and result mode IndexResult");
    let jsonpath_query = JsonPathQuery::parse("$[2][0]")?;
    let raw_json =
        fs::read_to_string("/home/mat/rsonpath/crates/rsonpath-test/tests/documents/toml/nested_arrays.json")?;
    let input = OwnedBytes::new(&raw_json.as_bytes())?;
    let engine = MainEngine::compile_query(&jsonpath_query)?;
    let result = engine.run::<_, IndexResult>(&input)?;
    assert_eq!(result.get(), vec![49usize,], "result != expected");
    Ok(())
}
#[test]
fn nested_lists_with_a_few_zero_atoms_with_query_path_2_0_with_owned_bytes_and_index_result_using_recursive_engine(
) -> Result<(), Box<dyn Error>> {
    println ! ("on document nested_arrays.toml running the query $[2][0] (path 2 0) with Input impl OwnedBytes and result mode IndexResult");
    let jsonpath_query = JsonPathQuery::parse("$[2][0]")?;
    let raw_json =
        fs::read_to_string("/home/mat/rsonpath/crates/rsonpath-test/tests/documents/toml/nested_arrays.json")?;
    let input = OwnedBytes::new(&raw_json.as_bytes())?;
    let engine = RecursiveEngine::compile_query(&jsonpath_query)?;
    let result = engine.run::<_, IndexResult>(&input)?;
    assert_eq!(result.get(), vec![49usize,], "result != expected");
    Ok(())
}
#[test]
fn nested_lists_with_a_few_zero_atoms_with_query_path_2_1_0_1_with_buffered_input_and_count_result_using_main_engine(
) -> Result<(), Box<dyn Error>> {
    println ! ("on document nested_arrays.toml running the query $[2][1][0][1] (path 2 1 0 1) with Input impl BufferedInput and result mode CountResult");
    let jsonpath_query = JsonPathQuery::parse("$[2][1][0][1]")?;
    let json_file = fs::File::open("/home/mat/rsonpath/crates/rsonpath-test/tests/documents/toml/nested_arrays.json")?;
    let input = BufferedInput::new(json_file);
    let engine = MainEngine::compile_query(&jsonpath_query)?;
    let result = engine.run::<_, CountResult>(&input)?;
    assert_eq!(result.get(), 1u64, "result != expected");
    Ok(())
}
#[test]
fn nested_lists_with_a_few_zero_atoms_with_query_path_2_1_0_1_with_buffered_input_and_count_result_using_recursive_engine(
) -> Result<(), Box<dyn Error>> {
    println ! ("on document nested_arrays.toml running the query $[2][1][0][1] (path 2 1 0 1) with Input impl BufferedInput and result mode CountResult");
    let jsonpath_query = JsonPathQuery::parse("$[2][1][0][1]")?;
    let json_file = fs::File::open("/home/mat/rsonpath/crates/rsonpath-test/tests/documents/toml/nested_arrays.json")?;
    let input = BufferedInput::new(json_file);
    let engine = RecursiveEngine::compile_query(&jsonpath_query)?;
    let result = engine.run::<_, CountResult>(&input)?;
    assert_eq!(result.get(), 1u64, "result != expected");
    Ok(())
}
#[test]
fn nested_lists_with_a_few_zero_atoms_with_query_path_2_1_0_1_with_buffered_input_and_index_result_using_main_engine(
) -> Result<(), Box<dyn Error>> {
    println ! ("on document nested_arrays.toml running the query $[2][1][0][1] (path 2 1 0 1) with Input impl BufferedInput and result mode IndexResult");
    let jsonpath_query = JsonPathQuery::parse("$[2][1][0][1]")?;
    let json_file = fs::File::open("/home/mat/rsonpath/crates/rsonpath-test/tests/documents/toml/nested_arrays.json")?;
    let input = BufferedInput::new(json_file);
    let engine = MainEngine::compile_query(&jsonpath_query)?;
    let result = engine.run::<_, IndexResult>(&input)?;
    assert_eq!(result.get(), vec![113usize,], "result != expected");
    Ok(())
}
#[test]
fn nested_lists_with_a_few_zero_atoms_with_query_path_2_1_0_1_with_buffered_input_and_index_result_using_recursive_engine(
) -> Result<(), Box<dyn Error>> {
    println ! ("on document nested_arrays.toml running the query $[2][1][0][1] (path 2 1 0 1) with Input impl BufferedInput and result mode IndexResult");
    let jsonpath_query = JsonPathQuery::parse("$[2][1][0][1]")?;
    let json_file = fs::File::open("/home/mat/rsonpath/crates/rsonpath-test/tests/documents/toml/nested_arrays.json")?;
    let input = BufferedInput::new(json_file);
    let engine = RecursiveEngine::compile_query(&jsonpath_query)?;
    let result = engine.run::<_, IndexResult>(&input)?;
    assert_eq!(result.get(), vec![113usize,], "result != expected");
    Ok(())
}
#[test]
fn nested_lists_with_a_few_zero_atoms_with_query_path_2_1_0_1_with_mmap_input_and_count_result_using_main_engine(
) -> Result<(), Box<dyn Error>> {
    println ! ("on document nested_arrays.toml running the query $[2][1][0][1] (path 2 1 0 1) with Input impl MmapInput and result mode CountResult");
    let jsonpath_query = JsonPathQuery::parse("$[2][1][0][1]")?;
    let json_file = fs::File::open("/home/mat/rsonpath/crates/rsonpath-test/tests/documents/toml/nested_arrays.json")?;
    let input = unsafe { MmapInput::map_file(&json_file)? };
    let engine = MainEngine::compile_query(&jsonpath_query)?;
    let result = engine.run::<_, CountResult>(&input)?;
    assert_eq!(result.get(), 1u64, "result != expected");
    Ok(())
}
#[test]
fn nested_lists_with_a_few_zero_atoms_with_query_path_2_1_0_1_with_mmap_input_and_count_result_using_recursive_engine(
) -> Result<(), Box<dyn Error>> {
    println ! ("on document nested_arrays.toml running the query $[2][1][0][1] (path 2 1 0 1) with Input impl MmapInput and result mode CountResult");
    let jsonpath_query = JsonPathQuery::parse("$[2][1][0][1]")?;
    let json_file = fs::File::open("/home/mat/rsonpath/crates/rsonpath-test/tests/documents/toml/nested_arrays.json")?;
    let input = unsafe { MmapInput::map_file(&json_file)? };
    let engine = RecursiveEngine::compile_query(&jsonpath_query)?;
    let result = engine.run::<_, CountResult>(&input)?;
    assert_eq!(result.get(), 1u64, "result != expected");
    Ok(())
}
#[test]
fn nested_lists_with_a_few_zero_atoms_with_query_path_2_1_0_1_with_mmap_input_and_index_result_using_main_engine(
) -> Result<(), Box<dyn Error>> {
    println ! ("on document nested_arrays.toml running the query $[2][1][0][1] (path 2 1 0 1) with Input impl MmapInput and result mode IndexResult");
    let jsonpath_query = JsonPathQuery::parse("$[2][1][0][1]")?;
    let json_file = fs::File::open("/home/mat/rsonpath/crates/rsonpath-test/tests/documents/toml/nested_arrays.json")?;
    let input = unsafe { MmapInput::map_file(&json_file)? };
    let engine = MainEngine::compile_query(&jsonpath_query)?;
    let result = engine.run::<_, IndexResult>(&input)?;
    assert_eq!(result.get(), vec![113usize,], "result != expected");
    Ok(())
}
#[test]
fn nested_lists_with_a_few_zero_atoms_with_query_path_2_1_0_1_with_mmap_input_and_index_result_using_recursive_engine(
) -> Result<(), Box<dyn Error>> {
    println ! ("on document nested_arrays.toml running the query $[2][1][0][1] (path 2 1 0 1) with Input impl MmapInput and result mode IndexResult");
    let jsonpath_query = JsonPathQuery::parse("$[2][1][0][1]")?;
    let json_file = fs::File::open("/home/mat/rsonpath/crates/rsonpath-test/tests/documents/toml/nested_arrays.json")?;
    let input = unsafe { MmapInput::map_file(&json_file)? };
    let engine = RecursiveEngine::compile_query(&jsonpath_query)?;
    let result = engine.run::<_, IndexResult>(&input)?;
    assert_eq!(result.get(), vec![113usize,], "result != expected");
    Ok(())
}
#[test]
fn nested_lists_with_a_few_zero_atoms_with_query_path_2_1_0_1_with_owned_bytes_and_count_result_using_main_engine(
) -> Result<(), Box<dyn Error>> {
    println ! ("on document nested_arrays.toml running the query $[2][1][0][1] (path 2 1 0 1) with Input impl OwnedBytes and result mode CountResult");
    let jsonpath_query = JsonPathQuery::parse("$[2][1][0][1]")?;
    let raw_json =
        fs::read_to_string("/home/mat/rsonpath/crates/rsonpath-test/tests/documents/toml/nested_arrays.json")?;
    let input = OwnedBytes::new(&raw_json.as_bytes())?;
    let engine = MainEngine::compile_query(&jsonpath_query)?;
    let result = engine.run::<_, CountResult>(&input)?;
    assert_eq!(result.get(), 1u64, "result != expected");
    Ok(())
}
#[test]
fn nested_lists_with_a_few_zero_atoms_with_query_path_2_1_0_1_with_owned_bytes_and_count_result_using_recursive_engine(
) -> Result<(), Box<dyn Error>> {
    println ! ("on document nested_arrays.toml running the query $[2][1][0][1] (path 2 1 0 1) with Input impl OwnedBytes and result mode CountResult");
    let jsonpath_query = JsonPathQuery::parse("$[2][1][0][1]")?;
    let raw_json =
        fs::read_to_string("/home/mat/rsonpath/crates/rsonpath-test/tests/documents/toml/nested_arrays.json")?;
    let input = OwnedBytes::new(&raw_json.as_bytes())?;
    let engine = RecursiveEngine::compile_query(&jsonpath_query)?;
    let result = engine.run::<_, CountResult>(&input)?;
    assert_eq!(result.get(), 1u64, "result != expected");
    Ok(())
}
#[test]
fn nested_lists_with_a_few_zero_atoms_with_query_path_2_1_0_1_with_owned_bytes_and_index_result_using_main_engine(
) -> Result<(), Box<dyn Error>> {
    println ! ("on document nested_arrays.toml running the query $[2][1][0][1] (path 2 1 0 1) with Input impl OwnedBytes and result mode IndexResult");
    let jsonpath_query = JsonPathQuery::parse("$[2][1][0][1]")?;
    let raw_json =
        fs::read_to_string("/home/mat/rsonpath/crates/rsonpath-test/tests/documents/toml/nested_arrays.json")?;
    let input = OwnedBytes::new(&raw_json.as_bytes())?;
    let engine = MainEngine::compile_query(&jsonpath_query)?;
    let result = engine.run::<_, IndexResult>(&input)?;
    assert_eq!(result.get(), vec![113usize,], "result != expected");
    Ok(())
}
#[test]
fn nested_lists_with_a_few_zero_atoms_with_query_path_2_1_0_1_with_owned_bytes_and_index_result_using_recursive_engine(
) -> Result<(), Box<dyn Error>> {
    println ! ("on document nested_arrays.toml running the query $[2][1][0][1] (path 2 1 0 1) with Input impl OwnedBytes and result mode IndexResult");
    let jsonpath_query = JsonPathQuery::parse("$[2][1][0][1]")?;
    let raw_json =
        fs::read_to_string("/home/mat/rsonpath/crates/rsonpath-test/tests/documents/toml/nested_arrays.json")?;
    let input = OwnedBytes::new(&raw_json.as_bytes())?;
    let engine = RecursiveEngine::compile_query(&jsonpath_query)?;
    let result = engine.run::<_, IndexResult>(&input)?;
    assert_eq!(result.get(), vec![113usize,], "result != expected");
    Ok(())
}
#[test]
fn nested_lists_with_a_few_zero_atoms_with_query_path_2_1_then_any_with_buffered_input_and_count_result_using_main_engine(
) -> Result<(), Box<dyn Error>> {
    println ! ("on document nested_arrays.toml running the query $[2][1].* (path 2 1, then any) with Input impl BufferedInput and result mode CountResult");
    let jsonpath_query = JsonPathQuery::parse("$[2][1].*")?;
    let json_file = fs::File::open("/home/mat/rsonpath/crates/rsonpath-test/tests/documents/toml/nested_arrays.json")?;
    let input = BufferedInput::new(json_file);
    let engine = MainEngine::compile_query(&jsonpath_query)?;
    let result = engine.run::<_, CountResult>(&input)?;
    assert_eq!(result.get(), 3u64, "result != expected");
    Ok(())
}
#[test]
fn nested_lists_with_a_few_zero_atoms_with_query_path_2_1_then_any_with_buffered_input_and_count_result_using_recursive_engine(
) -> Result<(), Box<dyn Error>> {
    println ! ("on document nested_arrays.toml running the query $[2][1].* (path 2 1, then any) with Input impl BufferedInput and result mode CountResult");
    let jsonpath_query = JsonPathQuery::parse("$[2][1].*")?;
    let json_file = fs::File::open("/home/mat/rsonpath/crates/rsonpath-test/tests/documents/toml/nested_arrays.json")?;
    let input = BufferedInput::new(json_file);
    let engine = RecursiveEngine::compile_query(&jsonpath_query)?;
    let result = engine.run::<_, CountResult>(&input)?;
    assert_eq!(result.get(), 3u64, "result != expected");
    Ok(())
}
#[test]
fn nested_lists_with_a_few_zero_atoms_with_query_path_2_1_then_any_with_buffered_input_and_index_result_using_main_engine(
) -> Result<(), Box<dyn Error>> {
    println ! ("on document nested_arrays.toml running the query $[2][1].* (path 2 1, then any) with Input impl BufferedInput and result mode IndexResult");
    let jsonpath_query = JsonPathQuery::parse("$[2][1].*")?;
    let json_file = fs::File::open("/home/mat/rsonpath/crates/rsonpath-test/tests/documents/toml/nested_arrays.json")?;
    let input = BufferedInput::new(json_file);
    let engine = MainEngine::compile_query(&jsonpath_query)?;
    let result = engine.run::<_, IndexResult>(&input)?;
    assert_eq!(result.get(), vec![75usize, 142usize, 209usize,], "result != expected");
    Ok(())
}
#[test]
fn nested_lists_with_a_few_zero_atoms_with_query_path_2_1_then_any_with_buffered_input_and_index_result_using_recursive_engine(
) -> Result<(), Box<dyn Error>> {
    println ! ("on document nested_arrays.toml running the query $[2][1].* (path 2 1, then any) with Input impl BufferedInput and result mode IndexResult");
    let jsonpath_query = JsonPathQuery::parse("$[2][1].*")?;
    let json_file = fs::File::open("/home/mat/rsonpath/crates/rsonpath-test/tests/documents/toml/nested_arrays.json")?;
    let input = BufferedInput::new(json_file);
    let engine = RecursiveEngine::compile_query(&jsonpath_query)?;
    let result = engine.run::<_, IndexResult>(&input)?;
    assert_eq!(result.get(), vec![75usize, 142usize, 209usize,], "result != expected");
    Ok(())
}
#[test]
fn nested_lists_with_a_few_zero_atoms_with_query_path_2_1_then_any_with_mmap_input_and_count_result_using_main_engine(
) -> Result<(), Box<dyn Error>> {
    println ! ("on document nested_arrays.toml running the query $[2][1].* (path 2 1, then any) with Input impl MmapInput and result mode CountResult");
    let jsonpath_query = JsonPathQuery::parse("$[2][1].*")?;
    let json_file = fs::File::open("/home/mat/rsonpath/crates/rsonpath-test/tests/documents/toml/nested_arrays.json")?;
    let input = unsafe { MmapInput::map_file(&json_file)? };
    let engine = MainEngine::compile_query(&jsonpath_query)?;
    let result = engine.run::<_, CountResult>(&input)?;
    assert_eq!(result.get(), 3u64, "result != expected");
    Ok(())
}
#[test]
fn nested_lists_with_a_few_zero_atoms_with_query_path_2_1_then_any_with_mmap_input_and_count_result_using_recursive_engine(
) -> Result<(), Box<dyn Error>> {
    println ! ("on document nested_arrays.toml running the query $[2][1].* (path 2 1, then any) with Input impl MmapInput and result mode CountResult");
    let jsonpath_query = JsonPathQuery::parse("$[2][1].*")?;
    let json_file = fs::File::open("/home/mat/rsonpath/crates/rsonpath-test/tests/documents/toml/nested_arrays.json")?;
    let input = unsafe { MmapInput::map_file(&json_file)? };
    let engine = RecursiveEngine::compile_query(&jsonpath_query)?;
    let result = engine.run::<_, CountResult>(&input)?;
    assert_eq!(result.get(), 3u64, "result != expected");
    Ok(())
}
#[test]
fn nested_lists_with_a_few_zero_atoms_with_query_path_2_1_then_any_with_mmap_input_and_index_result_using_main_engine(
) -> Result<(), Box<dyn Error>> {
    println ! ("on document nested_arrays.toml running the query $[2][1].* (path 2 1, then any) with Input impl MmapInput and result mode IndexResult");
    let jsonpath_query = JsonPathQuery::parse("$[2][1].*")?;
    let json_file = fs::File::open("/home/mat/rsonpath/crates/rsonpath-test/tests/documents/toml/nested_arrays.json")?;
    let input = unsafe { MmapInput::map_file(&json_file)? };
    let engine = MainEngine::compile_query(&jsonpath_query)?;
    let result = engine.run::<_, IndexResult>(&input)?;
    assert_eq!(result.get(), vec![75usize, 142usize, 209usize,], "result != expected");
    Ok(())
}
#[test]
fn nested_lists_with_a_few_zero_atoms_with_query_path_2_1_then_any_with_mmap_input_and_index_result_using_recursive_engine(
) -> Result<(), Box<dyn Error>> {
    println ! ("on document nested_arrays.toml running the query $[2][1].* (path 2 1, then any) with Input impl MmapInput and result mode IndexResult");
    let jsonpath_query = JsonPathQuery::parse("$[2][1].*")?;
    let json_file = fs::File::open("/home/mat/rsonpath/crates/rsonpath-test/tests/documents/toml/nested_arrays.json")?;
    let input = unsafe { MmapInput::map_file(&json_file)? };
    let engine = RecursiveEngine::compile_query(&jsonpath_query)?;
    let result = engine.run::<_, IndexResult>(&input)?;
    assert_eq!(result.get(), vec![75usize, 142usize, 209usize,], "result != expected");
    Ok(())
}
#[test]
fn nested_lists_with_a_few_zero_atoms_with_query_path_2_1_then_any_with_owned_bytes_and_count_result_using_main_engine(
) -> Result<(), Box<dyn Error>> {
    println ! ("on document nested_arrays.toml running the query $[2][1].* (path 2 1, then any) with Input impl OwnedBytes and result mode CountResult");
    let jsonpath_query = JsonPathQuery::parse("$[2][1].*")?;
    let raw_json =
        fs::read_to_string("/home/mat/rsonpath/crates/rsonpath-test/tests/documents/toml/nested_arrays.json")?;
    let input = OwnedBytes::new(&raw_json.as_bytes())?;
    let engine = MainEngine::compile_query(&jsonpath_query)?;
    let result = engine.run::<_, CountResult>(&input)?;
    assert_eq!(result.get(), 3u64, "result != expected");
    Ok(())
}
#[test]
fn nested_lists_with_a_few_zero_atoms_with_query_path_2_1_then_any_with_owned_bytes_and_count_result_using_recursive_engine(
) -> Result<(), Box<dyn Error>> {
    println ! ("on document nested_arrays.toml running the query $[2][1].* (path 2 1, then any) with Input impl OwnedBytes and result mode CountResult");
    let jsonpath_query = JsonPathQuery::parse("$[2][1].*")?;
    let raw_json =
        fs::read_to_string("/home/mat/rsonpath/crates/rsonpath-test/tests/documents/toml/nested_arrays.json")?;
    let input = OwnedBytes::new(&raw_json.as_bytes())?;
    let engine = RecursiveEngine::compile_query(&jsonpath_query)?;
    let result = engine.run::<_, CountResult>(&input)?;
    assert_eq!(result.get(), 3u64, "result != expected");
    Ok(())
}
#[test]
fn nested_lists_with_a_few_zero_atoms_with_query_path_2_1_then_any_with_owned_bytes_and_index_result_using_main_engine(
) -> Result<(), Box<dyn Error>> {
    println ! ("on document nested_arrays.toml running the query $[2][1].* (path 2 1, then any) with Input impl OwnedBytes and result mode IndexResult");
    let jsonpath_query = JsonPathQuery::parse("$[2][1].*")?;
    let raw_json =
        fs::read_to_string("/home/mat/rsonpath/crates/rsonpath-test/tests/documents/toml/nested_arrays.json")?;
    let input = OwnedBytes::new(&raw_json.as_bytes())?;
    let engine = MainEngine::compile_query(&jsonpath_query)?;
    let result = engine.run::<_, IndexResult>(&input)?;
    assert_eq!(result.get(), vec![75usize, 142usize, 209usize,], "result != expected");
    Ok(())
}
#[test]
fn nested_lists_with_a_few_zero_atoms_with_query_path_2_1_then_any_with_owned_bytes_and_index_result_using_recursive_engine(
) -> Result<(), Box<dyn Error>> {
    println ! ("on document nested_arrays.toml running the query $[2][1].* (path 2 1, then any) with Input impl OwnedBytes and result mode IndexResult");
    let jsonpath_query = JsonPathQuery::parse("$[2][1].*")?;
    let raw_json =
        fs::read_to_string("/home/mat/rsonpath/crates/rsonpath-test/tests/documents/toml/nested_arrays.json")?;
    let input = OwnedBytes::new(&raw_json.as_bytes())?;
    let engine = RecursiveEngine::compile_query(&jsonpath_query)?;
    let result = engine.run::<_, IndexResult>(&input)?;
    assert_eq!(result.get(), vec![75usize, 142usize, 209usize,], "result != expected");
    Ok(())
}
#[test]
fn nested_lists_with_a_few_zero_atoms_with_query_path_2_1_then_recurse_then_1_with_buffered_input_and_count_result_using_main_engine(
) -> Result<(), Box<dyn Error>> {
    println ! ("on document nested_arrays.toml running the query $[2][1]..*[1] (path 2 1, then recurse, then 1) with Input impl BufferedInput and result mode CountResult");
    let jsonpath_query = JsonPathQuery::parse("$[2][1]..*[1]")?;
    let json_file = fs::File::open("/home/mat/rsonpath/crates/rsonpath-test/tests/documents/toml/nested_arrays.json")?;
    let input = BufferedInput::new(json_file);
    let engine = MainEngine::compile_query(&jsonpath_query)?;
    let result = engine.run::<_, CountResult>(&input)?;
    assert_eq!(result.get(), 3u64, "result != expected");
    Ok(())
}
#[test]
fn nested_lists_with_a_few_zero_atoms_with_query_path_2_1_then_recurse_then_1_with_buffered_input_and_count_result_using_recursive_engine(
) -> Result<(), Box<dyn Error>> {
    println ! ("on document nested_arrays.toml running the query $[2][1]..*[1] (path 2 1, then recurse, then 1) with Input impl BufferedInput and result mode CountResult");
    let jsonpath_query = JsonPathQuery::parse("$[2][1]..*[1]")?;
    let json_file = fs::File::open("/home/mat/rsonpath/crates/rsonpath-test/tests/documents/toml/nested_arrays.json")?;
    let input = BufferedInput::new(json_file);
    let engine = RecursiveEngine::compile_query(&jsonpath_query)?;
    let result = engine.run::<_, CountResult>(&input)?;
    assert_eq!(result.get(), 3u64, "result != expected");
    Ok(())
}
#[test]
fn nested_lists_with_a_few_zero_atoms_with_query_path_2_1_then_recurse_then_1_with_buffered_input_and_index_result_using_main_engine(
) -> Result<(), Box<dyn Error>> {
    println ! ("on document nested_arrays.toml running the query $[2][1]..*[1] (path 2 1, then recurse, then 1) with Input impl BufferedInput and result mode IndexResult");
    let jsonpath_query = JsonPathQuery::parse("$[2][1]..*[1]")?;
    let json_file = fs::File::open("/home/mat/rsonpath/crates/rsonpath-test/tests/documents/toml/nested_arrays.json")?;
    let input = BufferedInput::new(json_file);
    let engine = MainEngine::compile_query(&jsonpath_query)?;
    let result = engine.run::<_, IndexResult>(&input)?;
    assert_eq!(result.get(), vec![113usize, 180usize, 247usize,], "result != expected");
    Ok(())
}
#[test]
fn nested_lists_with_a_few_zero_atoms_with_query_path_2_1_then_recurse_then_1_with_buffered_input_and_index_result_using_recursive_engine(
) -> Result<(), Box<dyn Error>> {
    println ! ("on document nested_arrays.toml running the query $[2][1]..*[1] (path 2 1, then recurse, then 1) with Input impl BufferedInput and result mode IndexResult");
    let jsonpath_query = JsonPathQuery::parse("$[2][1]..*[1]")?;
    let json_file = fs::File::open("/home/mat/rsonpath/crates/rsonpath-test/tests/documents/toml/nested_arrays.json")?;
    let input = BufferedInput::new(json_file);
    let engine = RecursiveEngine::compile_query(&jsonpath_query)?;
    let result = engine.run::<_, IndexResult>(&input)?;
    assert_eq!(result.get(), vec![113usize, 180usize, 247usize,], "result != expected");
    Ok(())
}
#[test]
fn nested_lists_with_a_few_zero_atoms_with_query_path_2_1_then_recurse_then_1_with_mmap_input_and_count_result_using_main_engine(
) -> Result<(), Box<dyn Error>> {
    println ! ("on document nested_arrays.toml running the query $[2][1]..*[1] (path 2 1, then recurse, then 1) with Input impl MmapInput and result mode CountResult");
    let jsonpath_query = JsonPathQuery::parse("$[2][1]..*[1]")?;
    let json_file = fs::File::open("/home/mat/rsonpath/crates/rsonpath-test/tests/documents/toml/nested_arrays.json")?;
    let input = unsafe { MmapInput::map_file(&json_file)? };
    let engine = MainEngine::compile_query(&jsonpath_query)?;
    let result = engine.run::<_, CountResult>(&input)?;
    assert_eq!(result.get(), 3u64, "result != expected");
    Ok(())
}
#[test]
fn nested_lists_with_a_few_zero_atoms_with_query_path_2_1_then_recurse_then_1_with_mmap_input_and_count_result_using_recursive_engine(
) -> Result<(), Box<dyn Error>> {
    println ! ("on document nested_arrays.toml running the query $[2][1]..*[1] (path 2 1, then recurse, then 1) with Input impl MmapInput and result mode CountResult");
    let jsonpath_query = JsonPathQuery::parse("$[2][1]..*[1]")?;
    let json_file = fs::File::open("/home/mat/rsonpath/crates/rsonpath-test/tests/documents/toml/nested_arrays.json")?;
    let input = unsafe { MmapInput::map_file(&json_file)? };
    let engine = RecursiveEngine::compile_query(&jsonpath_query)?;
    let result = engine.run::<_, CountResult>(&input)?;
    assert_eq!(result.get(), 3u64, "result != expected");
    Ok(())
}
#[test]
fn nested_lists_with_a_few_zero_atoms_with_query_path_2_1_then_recurse_then_1_with_mmap_input_and_index_result_using_main_engine(
) -> Result<(), Box<dyn Error>> {
    println ! ("on document nested_arrays.toml running the query $[2][1]..*[1] (path 2 1, then recurse, then 1) with Input impl MmapInput and result mode IndexResult");
    let jsonpath_query = JsonPathQuery::parse("$[2][1]..*[1]")?;
    let json_file = fs::File::open("/home/mat/rsonpath/crates/rsonpath-test/tests/documents/toml/nested_arrays.json")?;
    let input = unsafe { MmapInput::map_file(&json_file)? };
    let engine = MainEngine::compile_query(&jsonpath_query)?;
    let result = engine.run::<_, IndexResult>(&input)?;
    assert_eq!(result.get(), vec![113usize, 180usize, 247usize,], "result != expected");
    Ok(())
}
#[test]
fn nested_lists_with_a_few_zero_atoms_with_query_path_2_1_then_recurse_then_1_with_mmap_input_and_index_result_using_recursive_engine(
) -> Result<(), Box<dyn Error>> {
    println ! ("on document nested_arrays.toml running the query $[2][1]..*[1] (path 2 1, then recurse, then 1) with Input impl MmapInput and result mode IndexResult");
    let jsonpath_query = JsonPathQuery::parse("$[2][1]..*[1]")?;
    let json_file = fs::File::open("/home/mat/rsonpath/crates/rsonpath-test/tests/documents/toml/nested_arrays.json")?;
    let input = unsafe { MmapInput::map_file(&json_file)? };
    let engine = RecursiveEngine::compile_query(&jsonpath_query)?;
    let result = engine.run::<_, IndexResult>(&input)?;
    assert_eq!(result.get(), vec![113usize, 180usize, 247usize,], "result != expected");
    Ok(())
}
#[test]
fn nested_lists_with_a_few_zero_atoms_with_query_path_2_1_then_recurse_then_1_with_owned_bytes_and_count_result_using_main_engine(
) -> Result<(), Box<dyn Error>> {
    println ! ("on document nested_arrays.toml running the query $[2][1]..*[1] (path 2 1, then recurse, then 1) with Input impl OwnedBytes and result mode CountResult");
    let jsonpath_query = JsonPathQuery::parse("$[2][1]..*[1]")?;
    let raw_json =
        fs::read_to_string("/home/mat/rsonpath/crates/rsonpath-test/tests/documents/toml/nested_arrays.json")?;
    let input = OwnedBytes::new(&raw_json.as_bytes())?;
    let engine = MainEngine::compile_query(&jsonpath_query)?;
    let result = engine.run::<_, CountResult>(&input)?;
    assert_eq!(result.get(), 3u64, "result != expected");
    Ok(())
}
#[test]
fn nested_lists_with_a_few_zero_atoms_with_query_path_2_1_then_recurse_then_1_with_owned_bytes_and_count_result_using_recursive_engine(
) -> Result<(), Box<dyn Error>> {
    println ! ("on document nested_arrays.toml running the query $[2][1]..*[1] (path 2 1, then recurse, then 1) with Input impl OwnedBytes and result mode CountResult");
    let jsonpath_query = JsonPathQuery::parse("$[2][1]..*[1]")?;
    let raw_json =
        fs::read_to_string("/home/mat/rsonpath/crates/rsonpath-test/tests/documents/toml/nested_arrays.json")?;
    let input = OwnedBytes::new(&raw_json.as_bytes())?;
    let engine = RecursiveEngine::compile_query(&jsonpath_query)?;
    let result = engine.run::<_, CountResult>(&input)?;
    assert_eq!(result.get(), 3u64, "result != expected");
    Ok(())
}
#[test]
fn nested_lists_with_a_few_zero_atoms_with_query_path_2_1_then_recurse_then_1_with_owned_bytes_and_index_result_using_main_engine(
) -> Result<(), Box<dyn Error>> {
    println ! ("on document nested_arrays.toml running the query $[2][1]..*[1] (path 2 1, then recurse, then 1) with Input impl OwnedBytes and result mode IndexResult");
    let jsonpath_query = JsonPathQuery::parse("$[2][1]..*[1]")?;
    let raw_json =
        fs::read_to_string("/home/mat/rsonpath/crates/rsonpath-test/tests/documents/toml/nested_arrays.json")?;
    let input = OwnedBytes::new(&raw_json.as_bytes())?;
    let engine = MainEngine::compile_query(&jsonpath_query)?;
    let result = engine.run::<_, IndexResult>(&input)?;
    assert_eq!(result.get(), vec![113usize, 180usize, 247usize,], "result != expected");
    Ok(())
}
#[test]
fn nested_lists_with_a_few_zero_atoms_with_query_path_2_1_then_recurse_then_1_with_owned_bytes_and_index_result_using_recursive_engine(
) -> Result<(), Box<dyn Error>> {
    println ! ("on document nested_arrays.toml running the query $[2][1]..*[1] (path 2 1, then recurse, then 1) with Input impl OwnedBytes and result mode IndexResult");
    let jsonpath_query = JsonPathQuery::parse("$[2][1]..*[1]")?;
    let raw_json =
        fs::read_to_string("/home/mat/rsonpath/crates/rsonpath-test/tests/documents/toml/nested_arrays.json")?;
    let input = OwnedBytes::new(&raw_json.as_bytes())?;
    let engine = RecursiveEngine::compile_query(&jsonpath_query)?;
    let result = engine.run::<_, IndexResult>(&input)?;
    assert_eq!(result.get(), vec![113usize, 180usize, 247usize,], "result != expected");
    Ok(())
}
#[test]
fn nested_lists_with_a_few_zero_atoms_with_query_path_2_1_with_buffered_input_and_count_result_using_main_engine(
) -> Result<(), Box<dyn Error>> {
    println ! ("on document nested_arrays.toml running the query $[2][1] (path 2 1) with Input impl BufferedInput and result mode CountResult");
    let jsonpath_query = JsonPathQuery::parse("$[2][1]")?;
    let json_file = fs::File::open("/home/mat/rsonpath/crates/rsonpath-test/tests/documents/toml/nested_arrays.json")?;
    let input = BufferedInput::new(json_file);
    let engine = MainEngine::compile_query(&jsonpath_query)?;
    let result = engine.run::<_, CountResult>(&input)?;
    assert_eq!(result.get(), 1u64, "result != expected");
    Ok(())
}
#[test]
fn nested_lists_with_a_few_zero_atoms_with_query_path_2_1_with_buffered_input_and_count_result_using_recursive_engine(
) -> Result<(), Box<dyn Error>> {
    println ! ("on document nested_arrays.toml running the query $[2][1] (path 2 1) with Input impl BufferedInput and result mode CountResult");
    let jsonpath_query = JsonPathQuery::parse("$[2][1]")?;
    let json_file = fs::File::open("/home/mat/rsonpath/crates/rsonpath-test/tests/documents/toml/nested_arrays.json")?;
    let input = BufferedInput::new(json_file);
    let engine = RecursiveEngine::compile_query(&jsonpath_query)?;
    let result = engine.run::<_, CountResult>(&input)?;
    assert_eq!(result.get(), 1u64, "result != expected");
    Ok(())
}
#[test]
fn nested_lists_with_a_few_zero_atoms_with_query_path_2_1_with_buffered_input_and_index_result_using_main_engine(
) -> Result<(), Box<dyn Error>> {
    println ! ("on document nested_arrays.toml running the query $[2][1] (path 2 1) with Input impl BufferedInput and result mode IndexResult");
    let jsonpath_query = JsonPathQuery::parse("$[2][1]")?;
    let json_file = fs::File::open("/home/mat/rsonpath/crates/rsonpath-test/tests/documents/toml/nested_arrays.json")?;
    let input = BufferedInput::new(json_file);
    let engine = MainEngine::compile_query(&jsonpath_query)?;
    let result = engine.run::<_, IndexResult>(&input)?;
    assert_eq!(result.get(), vec![61usize,], "result != expected");
    Ok(())
}
#[test]
fn nested_lists_with_a_few_zero_atoms_with_query_path_2_1_with_buffered_input_and_index_result_using_recursive_engine(
) -> Result<(), Box<dyn Error>> {
    println ! ("on document nested_arrays.toml running the query $[2][1] (path 2 1) with Input impl BufferedInput and result mode IndexResult");
    let jsonpath_query = JsonPathQuery::parse("$[2][1]")?;
    let json_file = fs::File::open("/home/mat/rsonpath/crates/rsonpath-test/tests/documents/toml/nested_arrays.json")?;
    let input = BufferedInput::new(json_file);
    let engine = RecursiveEngine::compile_query(&jsonpath_query)?;
    let result = engine.run::<_, IndexResult>(&input)?;
    assert_eq!(result.get(), vec![61usize,], "result != expected");
    Ok(())
}
#[test]
fn nested_lists_with_a_few_zero_atoms_with_query_path_2_1_with_mmap_input_and_count_result_using_main_engine(
) -> Result<(), Box<dyn Error>> {
    println ! ("on document nested_arrays.toml running the query $[2][1] (path 2 1) with Input impl MmapInput and result mode CountResult");
    let jsonpath_query = JsonPathQuery::parse("$[2][1]")?;
    let json_file = fs::File::open("/home/mat/rsonpath/crates/rsonpath-test/tests/documents/toml/nested_arrays.json")?;
    let input = unsafe { MmapInput::map_file(&json_file)? };
    let engine = MainEngine::compile_query(&jsonpath_query)?;
    let result = engine.run::<_, CountResult>(&input)?;
    assert_eq!(result.get(), 1u64, "result != expected");
    Ok(())
}
#[test]
fn nested_lists_with_a_few_zero_atoms_with_query_path_2_1_with_mmap_input_and_count_result_using_recursive_engine(
) -> Result<(), Box<dyn Error>> {
    println ! ("on document nested_arrays.toml running the query $[2][1] (path 2 1) with Input impl MmapInput and result mode CountResult");
    let jsonpath_query = JsonPathQuery::parse("$[2][1]")?;
    let json_file = fs::File::open("/home/mat/rsonpath/crates/rsonpath-test/tests/documents/toml/nested_arrays.json")?;
    let input = unsafe { MmapInput::map_file(&json_file)? };
    let engine = RecursiveEngine::compile_query(&jsonpath_query)?;
    let result = engine.run::<_, CountResult>(&input)?;
    assert_eq!(result.get(), 1u64, "result != expected");
    Ok(())
}
#[test]
fn nested_lists_with_a_few_zero_atoms_with_query_path_2_1_with_mmap_input_and_index_result_using_main_engine(
) -> Result<(), Box<dyn Error>> {
    println ! ("on document nested_arrays.toml running the query $[2][1] (path 2 1) with Input impl MmapInput and result mode IndexResult");
    let jsonpath_query = JsonPathQuery::parse("$[2][1]")?;
    let json_file = fs::File::open("/home/mat/rsonpath/crates/rsonpath-test/tests/documents/toml/nested_arrays.json")?;
    let input = unsafe { MmapInput::map_file(&json_file)? };
    let engine = MainEngine::compile_query(&jsonpath_query)?;
    let result = engine.run::<_, IndexResult>(&input)?;
    assert_eq!(result.get(), vec![61usize,], "result != expected");
    Ok(())
}
#[test]
fn nested_lists_with_a_few_zero_atoms_with_query_path_2_1_with_mmap_input_and_index_result_using_recursive_engine(
) -> Result<(), Box<dyn Error>> {
    println ! ("on document nested_arrays.toml running the query $[2][1] (path 2 1) with Input impl MmapInput and result mode IndexResult");
    let jsonpath_query = JsonPathQuery::parse("$[2][1]")?;
    let json_file = fs::File::open("/home/mat/rsonpath/crates/rsonpath-test/tests/documents/toml/nested_arrays.json")?;
    let input = unsafe { MmapInput::map_file(&json_file)? };
    let engine = RecursiveEngine::compile_query(&jsonpath_query)?;
    let result = engine.run::<_, IndexResult>(&input)?;
    assert_eq!(result.get(), vec![61usize,], "result != expected");
    Ok(())
}
#[test]
fn nested_lists_with_a_few_zero_atoms_with_query_path_2_1_with_owned_bytes_and_count_result_using_main_engine(
) -> Result<(), Box<dyn Error>> {
    println ! ("on document nested_arrays.toml running the query $[2][1] (path 2 1) with Input impl OwnedBytes and result mode CountResult");
    let jsonpath_query = JsonPathQuery::parse("$[2][1]")?;
    let raw_json =
        fs::read_to_string("/home/mat/rsonpath/crates/rsonpath-test/tests/documents/toml/nested_arrays.json")?;
    let input = OwnedBytes::new(&raw_json.as_bytes())?;
    let engine = MainEngine::compile_query(&jsonpath_query)?;
    let result = engine.run::<_, CountResult>(&input)?;
    assert_eq!(result.get(), 1u64, "result != expected");
    Ok(())
}
#[test]
fn nested_lists_with_a_few_zero_atoms_with_query_path_2_1_with_owned_bytes_and_count_result_using_recursive_engine(
) -> Result<(), Box<dyn Error>> {
    println ! ("on document nested_arrays.toml running the query $[2][1] (path 2 1) with Input impl OwnedBytes and result mode CountResult");
    let jsonpath_query = JsonPathQuery::parse("$[2][1]")?;
    let raw_json =
        fs::read_to_string("/home/mat/rsonpath/crates/rsonpath-test/tests/documents/toml/nested_arrays.json")?;
    let input = OwnedBytes::new(&raw_json.as_bytes())?;
    let engine = RecursiveEngine::compile_query(&jsonpath_query)?;
    let result = engine.run::<_, CountResult>(&input)?;
    assert_eq!(result.get(), 1u64, "result != expected");
    Ok(())
}
#[test]
fn nested_lists_with_a_few_zero_atoms_with_query_path_2_1_with_owned_bytes_and_index_result_using_main_engine(
) -> Result<(), Box<dyn Error>> {
    println ! ("on document nested_arrays.toml running the query $[2][1] (path 2 1) with Input impl OwnedBytes and result mode IndexResult");
    let jsonpath_query = JsonPathQuery::parse("$[2][1]")?;
    let raw_json =
        fs::read_to_string("/home/mat/rsonpath/crates/rsonpath-test/tests/documents/toml/nested_arrays.json")?;
    let input = OwnedBytes::new(&raw_json.as_bytes())?;
    let engine = MainEngine::compile_query(&jsonpath_query)?;
    let result = engine.run::<_, IndexResult>(&input)?;
    assert_eq!(result.get(), vec![61usize,], "result != expected");
    Ok(())
}
#[test]
fn nested_lists_with_a_few_zero_atoms_with_query_path_2_1_with_owned_bytes_and_index_result_using_recursive_engine(
) -> Result<(), Box<dyn Error>> {
    println ! ("on document nested_arrays.toml running the query $[2][1] (path 2 1) with Input impl OwnedBytes and result mode IndexResult");
    let jsonpath_query = JsonPathQuery::parse("$[2][1]")?;
    let raw_json =
        fs::read_to_string("/home/mat/rsonpath/crates/rsonpath-test/tests/documents/toml/nested_arrays.json")?;
    let input = OwnedBytes::new(&raw_json.as_bytes())?;
    let engine = RecursiveEngine::compile_query(&jsonpath_query)?;
    let result = engine.run::<_, IndexResult>(&input)?;
    assert_eq!(result.get(), vec![61usize,], "result != expected");
    Ok(())
}
#[test]
fn nested_lists_with_a_few_zero_atoms_with_query_path_2_then_any_then_1_with_buffered_input_and_count_result_using_main_engine(
) -> Result<(), Box<dyn Error>> {
    println ! ("on document nested_arrays.toml running the query $[2].*[1] (path 2, then any, then 1) with Input impl BufferedInput and result mode CountResult");
    let jsonpath_query = JsonPathQuery::parse("$[2].*[1]")?;
    let json_file = fs::File::open("/home/mat/rsonpath/crates/rsonpath-test/tests/documents/toml/nested_arrays.json")?;
    let input = BufferedInput::new(json_file);
    let engine = MainEngine::compile_query(&jsonpath_query)?;
    let result = engine.run::<_, CountResult>(&input)?;
    assert_eq!(result.get(), 1u64, "result != expected");
    Ok(())
}
#[test]
fn nested_lists_with_a_few_zero_atoms_with_query_path_2_then_any_then_1_with_buffered_input_and_count_result_using_recursive_engine(
) -> Result<(), Box<dyn Error>> {
    println ! ("on document nested_arrays.toml running the query $[2].*[1] (path 2, then any, then 1) with Input impl BufferedInput and result mode CountResult");
    let jsonpath_query = JsonPathQuery::parse("$[2].*[1]")?;
    let json_file = fs::File::open("/home/mat/rsonpath/crates/rsonpath-test/tests/documents/toml/nested_arrays.json")?;
    let input = BufferedInput::new(json_file);
    let engine = RecursiveEngine::compile_query(&jsonpath_query)?;
    let result = engine.run::<_, CountResult>(&input)?;
    assert_eq!(result.get(), 1u64, "result != expected");
    Ok(())
}
#[test]
fn nested_lists_with_a_few_zero_atoms_with_query_path_2_then_any_then_1_with_buffered_input_and_index_result_using_main_engine(
) -> Result<(), Box<dyn Error>> {
    println ! ("on document nested_arrays.toml running the query $[2].*[1] (path 2, then any, then 1) with Input impl BufferedInput and result mode IndexResult");
    let jsonpath_query = JsonPathQuery::parse("$[2].*[1]")?;
    let json_file = fs::File::open("/home/mat/rsonpath/crates/rsonpath-test/tests/documents/toml/nested_arrays.json")?;
    let input = BufferedInput::new(json_file);
    let engine = MainEngine::compile_query(&jsonpath_query)?;
    let result = engine.run::<_, IndexResult>(&input)?;
    assert_eq!(result.get(), vec![142usize,], "result != expected");
    Ok(())
}
#[test]
fn nested_lists_with_a_few_zero_atoms_with_query_path_2_then_any_then_1_with_buffered_input_and_index_result_using_recursive_engine(
) -> Result<(), Box<dyn Error>> {
    println ! ("on document nested_arrays.toml running the query $[2].*[1] (path 2, then any, then 1) with Input impl BufferedInput and result mode IndexResult");
    let jsonpath_query = JsonPathQuery::parse("$[2].*[1]")?;
    let json_file = fs::File::open("/home/mat/rsonpath/crates/rsonpath-test/tests/documents/toml/nested_arrays.json")?;
    let input = BufferedInput::new(json_file);
    let engine = RecursiveEngine::compile_query(&jsonpath_query)?;
    let result = engine.run::<_, IndexResult>(&input)?;
    assert_eq!(result.get(), vec![142usize,], "result != expected");
    Ok(())
}
#[test]
fn nested_lists_with_a_few_zero_atoms_with_query_path_2_then_any_then_1_with_mmap_input_and_count_result_using_main_engine(
) -> Result<(), Box<dyn Error>> {
    println ! ("on document nested_arrays.toml running the query $[2].*[1] (path 2, then any, then 1) with Input impl MmapInput and result mode CountResult");
    let jsonpath_query = JsonPathQuery::parse("$[2].*[1]")?;
    let json_file = fs::File::open("/home/mat/rsonpath/crates/rsonpath-test/tests/documents/toml/nested_arrays.json")?;
    let input = unsafe { MmapInput::map_file(&json_file)? };
    let engine = MainEngine::compile_query(&jsonpath_query)?;
    let result = engine.run::<_, CountResult>(&input)?;
    assert_eq!(result.get(), 1u64, "result != expected");
    Ok(())
}
#[test]
fn nested_lists_with_a_few_zero_atoms_with_query_path_2_then_any_then_1_with_mmap_input_and_count_result_using_recursive_engine(
) -> Result<(), Box<dyn Error>> {
    println ! ("on document nested_arrays.toml running the query $[2].*[1] (path 2, then any, then 1) with Input impl MmapInput and result mode CountResult");
    let jsonpath_query = JsonPathQuery::parse("$[2].*[1]")?;
    let json_file = fs::File::open("/home/mat/rsonpath/crates/rsonpath-test/tests/documents/toml/nested_arrays.json")?;
    let input = unsafe { MmapInput::map_file(&json_file)? };
    let engine = RecursiveEngine::compile_query(&jsonpath_query)?;
    let result = engine.run::<_, CountResult>(&input)?;
    assert_eq!(result.get(), 1u64, "result != expected");
    Ok(())
}
#[test]
fn nested_lists_with_a_few_zero_atoms_with_query_path_2_then_any_then_1_with_mmap_input_and_index_result_using_main_engine(
) -> Result<(), Box<dyn Error>> {
    println ! ("on document nested_arrays.toml running the query $[2].*[1] (path 2, then any, then 1) with Input impl MmapInput and result mode IndexResult");
    let jsonpath_query = JsonPathQuery::parse("$[2].*[1]")?;
    let json_file = fs::File::open("/home/mat/rsonpath/crates/rsonpath-test/tests/documents/toml/nested_arrays.json")?;
    let input = unsafe { MmapInput::map_file(&json_file)? };
    let engine = MainEngine::compile_query(&jsonpath_query)?;
    let result = engine.run::<_, IndexResult>(&input)?;
    assert_eq!(result.get(), vec![142usize,], "result != expected");
    Ok(())
}
#[test]
fn nested_lists_with_a_few_zero_atoms_with_query_path_2_then_any_then_1_with_mmap_input_and_index_result_using_recursive_engine(
) -> Result<(), Box<dyn Error>> {
    println ! ("on document nested_arrays.toml running the query $[2].*[1] (path 2, then any, then 1) with Input impl MmapInput and result mode IndexResult");
    let jsonpath_query = JsonPathQuery::parse("$[2].*[1]")?;
    let json_file = fs::File::open("/home/mat/rsonpath/crates/rsonpath-test/tests/documents/toml/nested_arrays.json")?;
    let input = unsafe { MmapInput::map_file(&json_file)? };
    let engine = RecursiveEngine::compile_query(&jsonpath_query)?;
    let result = engine.run::<_, IndexResult>(&input)?;
    assert_eq!(result.get(), vec![142usize,], "result != expected");
    Ok(())
}
#[test]
fn nested_lists_with_a_few_zero_atoms_with_query_path_2_then_any_then_1_with_owned_bytes_and_count_result_using_main_engine(
) -> Result<(), Box<dyn Error>> {
    println ! ("on document nested_arrays.toml running the query $[2].*[1] (path 2, then any, then 1) with Input impl OwnedBytes and result mode CountResult");
    let jsonpath_query = JsonPathQuery::parse("$[2].*[1]")?;
    let raw_json =
        fs::read_to_string("/home/mat/rsonpath/crates/rsonpath-test/tests/documents/toml/nested_arrays.json")?;
    let input = OwnedBytes::new(&raw_json.as_bytes())?;
    let engine = MainEngine::compile_query(&jsonpath_query)?;
    let result = engine.run::<_, CountResult>(&input)?;
    assert_eq!(result.get(), 1u64, "result != expected");
    Ok(())
}
#[test]
fn nested_lists_with_a_few_zero_atoms_with_query_path_2_then_any_then_1_with_owned_bytes_and_count_result_using_recursive_engine(
) -> Result<(), Box<dyn Error>> {
    println ! ("on document nested_arrays.toml running the query $[2].*[1] (path 2, then any, then 1) with Input impl OwnedBytes and result mode CountResult");
    let jsonpath_query = JsonPathQuery::parse("$[2].*[1]")?;
    let raw_json =
        fs::read_to_string("/home/mat/rsonpath/crates/rsonpath-test/tests/documents/toml/nested_arrays.json")?;
    let input = OwnedBytes::new(&raw_json.as_bytes())?;
    let engine = RecursiveEngine::compile_query(&jsonpath_query)?;
    let result = engine.run::<_, CountResult>(&input)?;
    assert_eq!(result.get(), 1u64, "result != expected");
    Ok(())
}
#[test]
fn nested_lists_with_a_few_zero_atoms_with_query_path_2_then_any_then_1_with_owned_bytes_and_index_result_using_main_engine(
) -> Result<(), Box<dyn Error>> {
    println ! ("on document nested_arrays.toml running the query $[2].*[1] (path 2, then any, then 1) with Input impl OwnedBytes and result mode IndexResult");
    let jsonpath_query = JsonPathQuery::parse("$[2].*[1]")?;
    let raw_json =
        fs::read_to_string("/home/mat/rsonpath/crates/rsonpath-test/tests/documents/toml/nested_arrays.json")?;
    let input = OwnedBytes::new(&raw_json.as_bytes())?;
    let engine = MainEngine::compile_query(&jsonpath_query)?;
    let result = engine.run::<_, IndexResult>(&input)?;
    assert_eq!(result.get(), vec![142usize,], "result != expected");
    Ok(())
}
#[test]
fn nested_lists_with_a_few_zero_atoms_with_query_path_2_then_any_then_1_with_owned_bytes_and_index_result_using_recursive_engine(
) -> Result<(), Box<dyn Error>> {
    println ! ("on document nested_arrays.toml running the query $[2].*[1] (path 2, then any, then 1) with Input impl OwnedBytes and result mode IndexResult");
    let jsonpath_query = JsonPathQuery::parse("$[2].*[1]")?;
    let raw_json =
        fs::read_to_string("/home/mat/rsonpath/crates/rsonpath-test/tests/documents/toml/nested_arrays.json")?;
    let input = OwnedBytes::new(&raw_json.as_bytes())?;
    let engine = RecursiveEngine::compile_query(&jsonpath_query)?;
    let result = engine.run::<_, IndexResult>(&input)?;
    assert_eq!(result.get(), vec![142usize,], "result != expected");
    Ok(())
}
#[test]
fn nested_lists_with_a_few_zero_atoms_with_query_path_2_then_recurse_with_buffered_input_and_count_result_using_main_engine(
) -> Result<(), Box<dyn Error>> {
    println ! ("on document nested_arrays.toml running the query $[2]..* (path 2, then recurse) with Input impl BufferedInput and result mode CountResult");
    let jsonpath_query = JsonPathQuery::parse("$[2]..*")?;
    let json_file = fs::File::open("/home/mat/rsonpath/crates/rsonpath-test/tests/documents/toml/nested_arrays.json")?;
    let input = BufferedInput::new(json_file);
    let engine = MainEngine::compile_query(&jsonpath_query)?;
    let result = engine.run::<_, CountResult>(&input)?;
    assert_eq!(result.get(), 11u64, "result != expected");
    Ok(())
}
#[test]
fn nested_lists_with_a_few_zero_atoms_with_query_path_2_then_recurse_with_buffered_input_and_count_result_using_recursive_engine(
) -> Result<(), Box<dyn Error>> {
    println ! ("on document nested_arrays.toml running the query $[2]..* (path 2, then recurse) with Input impl BufferedInput and result mode CountResult");
    let jsonpath_query = JsonPathQuery::parse("$[2]..*")?;
    let json_file = fs::File::open("/home/mat/rsonpath/crates/rsonpath-test/tests/documents/toml/nested_arrays.json")?;
    let input = BufferedInput::new(json_file);
    let engine = RecursiveEngine::compile_query(&jsonpath_query)?;
    let result = engine.run::<_, CountResult>(&input)?;
    assert_eq!(result.get(), 11u64, "result != expected");
    Ok(())
}
#[test]
fn nested_lists_with_a_few_zero_atoms_with_query_path_2_then_recurse_with_buffered_input_and_index_result_using_main_engine(
) -> Result<(), Box<dyn Error>> {
    println ! ("on document nested_arrays.toml running the query $[2]..* (path 2, then recurse) with Input impl BufferedInput and result mode IndexResult");
    let jsonpath_query = JsonPathQuery::parse("$[2]..*")?;
    let json_file = fs::File::open("/home/mat/rsonpath/crates/rsonpath-test/tests/documents/toml/nested_arrays.json")?;
    let input = BufferedInput::new(json_file);
    let engine = MainEngine::compile_query(&jsonpath_query)?;
    let result = engine.run::<_, IndexResult>(&input)?;
    assert_eq!(
        result.get(),
        vec![49usize, 61usize, 75usize, 93usize, 113usize, 142usize, 160usize, 180usize, 209usize, 227usize, 247usize,],
        "result != expected"
    );
    Ok(())
}
#[test]
fn nested_lists_with_a_few_zero_atoms_with_query_path_2_then_recurse_with_buffered_input_and_index_result_using_recursive_engine(
) -> Result<(), Box<dyn Error>> {
    println ! ("on document nested_arrays.toml running the query $[2]..* (path 2, then recurse) with Input impl BufferedInput and result mode IndexResult");
    let jsonpath_query = JsonPathQuery::parse("$[2]..*")?;
    let json_file = fs::File::open("/home/mat/rsonpath/crates/rsonpath-test/tests/documents/toml/nested_arrays.json")?;
    let input = BufferedInput::new(json_file);
    let engine = RecursiveEngine::compile_query(&jsonpath_query)?;
    let result = engine.run::<_, IndexResult>(&input)?;
    assert_eq!(
        result.get(),
        vec![49usize, 61usize, 75usize, 93usize, 113usize, 142usize, 160usize, 180usize, 209usize, 227usize, 247usize,],
        "result != expected"
    );
    Ok(())
}
#[test]
fn nested_lists_with_a_few_zero_atoms_with_query_path_2_then_recurse_with_mmap_input_and_count_result_using_main_engine(
) -> Result<(), Box<dyn Error>> {
    println ! ("on document nested_arrays.toml running the query $[2]..* (path 2, then recurse) with Input impl MmapInput and result mode CountResult");
    let jsonpath_query = JsonPathQuery::parse("$[2]..*")?;
    let json_file = fs::File::open("/home/mat/rsonpath/crates/rsonpath-test/tests/documents/toml/nested_arrays.json")?;
    let input = unsafe { MmapInput::map_file(&json_file)? };
    let engine = MainEngine::compile_query(&jsonpath_query)?;
    let result = engine.run::<_, CountResult>(&input)?;
    assert_eq!(result.get(), 11u64, "result != expected");
    Ok(())
}
#[test]
fn nested_lists_with_a_few_zero_atoms_with_query_path_2_then_recurse_with_mmap_input_and_count_result_using_recursive_engine(
) -> Result<(), Box<dyn Error>> {
    println ! ("on document nested_arrays.toml running the query $[2]..* (path 2, then recurse) with Input impl MmapInput and result mode CountResult");
    let jsonpath_query = JsonPathQuery::parse("$[2]..*")?;
    let json_file = fs::File::open("/home/mat/rsonpath/crates/rsonpath-test/tests/documents/toml/nested_arrays.json")?;
    let input = unsafe { MmapInput::map_file(&json_file)? };
    let engine = RecursiveEngine::compile_query(&jsonpath_query)?;
    let result = engine.run::<_, CountResult>(&input)?;
    assert_eq!(result.get(), 11u64, "result != expected");
    Ok(())
}
#[test]
fn nested_lists_with_a_few_zero_atoms_with_query_path_2_then_recurse_with_mmap_input_and_index_result_using_main_engine(
) -> Result<(), Box<dyn Error>> {
    println ! ("on document nested_arrays.toml running the query $[2]..* (path 2, then recurse) with Input impl MmapInput and result mode IndexResult");
    let jsonpath_query = JsonPathQuery::parse("$[2]..*")?;
    let json_file = fs::File::open("/home/mat/rsonpath/crates/rsonpath-test/tests/documents/toml/nested_arrays.json")?;
    let input = unsafe { MmapInput::map_file(&json_file)? };
    let engine = MainEngine::compile_query(&jsonpath_query)?;
    let result = engine.run::<_, IndexResult>(&input)?;
    assert_eq!(
        result.get(),
        vec![49usize, 61usize, 75usize, 93usize, 113usize, 142usize, 160usize, 180usize, 209usize, 227usize, 247usize,],
        "result != expected"
    );
    Ok(())
}
#[test]
fn nested_lists_with_a_few_zero_atoms_with_query_path_2_then_recurse_with_mmap_input_and_index_result_using_recursive_engine(
) -> Result<(), Box<dyn Error>> {
    println ! ("on document nested_arrays.toml running the query $[2]..* (path 2, then recurse) with Input impl MmapInput and result mode IndexResult");
    let jsonpath_query = JsonPathQuery::parse("$[2]..*")?;
    let json_file = fs::File::open("/home/mat/rsonpath/crates/rsonpath-test/tests/documents/toml/nested_arrays.json")?;
    let input = unsafe { MmapInput::map_file(&json_file)? };
    let engine = RecursiveEngine::compile_query(&jsonpath_query)?;
    let result = engine.run::<_, IndexResult>(&input)?;
    assert_eq!(
        result.get(),
        vec![49usize, 61usize, 75usize, 93usize, 113usize, 142usize, 160usize, 180usize, 209usize, 227usize, 247usize,],
        "result != expected"
    );
    Ok(())
}
#[test]
fn nested_lists_with_a_few_zero_atoms_with_query_path_2_then_recurse_with_owned_bytes_and_count_result_using_main_engine(
) -> Result<(), Box<dyn Error>> {
    println ! ("on document nested_arrays.toml running the query $[2]..* (path 2, then recurse) with Input impl OwnedBytes and result mode CountResult");
    let jsonpath_query = JsonPathQuery::parse("$[2]..*")?;
    let raw_json =
        fs::read_to_string("/home/mat/rsonpath/crates/rsonpath-test/tests/documents/toml/nested_arrays.json")?;
    let input = OwnedBytes::new(&raw_json.as_bytes())?;
    let engine = MainEngine::compile_query(&jsonpath_query)?;
    let result = engine.run::<_, CountResult>(&input)?;
    assert_eq!(result.get(), 11u64, "result != expected");
    Ok(())
}
#[test]
fn nested_lists_with_a_few_zero_atoms_with_query_path_2_then_recurse_with_owned_bytes_and_count_result_using_recursive_engine(
) -> Result<(), Box<dyn Error>> {
    println ! ("on document nested_arrays.toml running the query $[2]..* (path 2, then recurse) with Input impl OwnedBytes and result mode CountResult");
    let jsonpath_query = JsonPathQuery::parse("$[2]..*")?;
    let raw_json =
        fs::read_to_string("/home/mat/rsonpath/crates/rsonpath-test/tests/documents/toml/nested_arrays.json")?;
    let input = OwnedBytes::new(&raw_json.as_bytes())?;
    let engine = RecursiveEngine::compile_query(&jsonpath_query)?;
    let result = engine.run::<_, CountResult>(&input)?;
    assert_eq!(result.get(), 11u64, "result != expected");
    Ok(())
}
#[test]
fn nested_lists_with_a_few_zero_atoms_with_query_path_2_then_recurse_with_owned_bytes_and_index_result_using_main_engine(
) -> Result<(), Box<dyn Error>> {
    println ! ("on document nested_arrays.toml running the query $[2]..* (path 2, then recurse) with Input impl OwnedBytes and result mode IndexResult");
    let jsonpath_query = JsonPathQuery::parse("$[2]..*")?;
    let raw_json =
        fs::read_to_string("/home/mat/rsonpath/crates/rsonpath-test/tests/documents/toml/nested_arrays.json")?;
    let input = OwnedBytes::new(&raw_json.as_bytes())?;
    let engine = MainEngine::compile_query(&jsonpath_query)?;
    let result = engine.run::<_, IndexResult>(&input)?;
    assert_eq!(
        result.get(),
        vec![49usize, 61usize, 75usize, 93usize, 113usize, 142usize, 160usize, 180usize, 209usize, 227usize, 247usize,],
        "result != expected"
    );
    Ok(())
}
#[test]
fn nested_lists_with_a_few_zero_atoms_with_query_path_2_then_recurse_with_owned_bytes_and_index_result_using_recursive_engine(
) -> Result<(), Box<dyn Error>> {
    println ! ("on document nested_arrays.toml running the query $[2]..* (path 2, then recurse) with Input impl OwnedBytes and result mode IndexResult");
    let jsonpath_query = JsonPathQuery::parse("$[2]..*")?;
    let raw_json =
        fs::read_to_string("/home/mat/rsonpath/crates/rsonpath-test/tests/documents/toml/nested_arrays.json")?;
    let input = OwnedBytes::new(&raw_json.as_bytes())?;
    let engine = RecursiveEngine::compile_query(&jsonpath_query)?;
    let result = engine.run::<_, IndexResult>(&input)?;
    assert_eq!(
        result.get(),
        vec![49usize, 61usize, 75usize, 93usize, 113usize, 142usize, 160usize, 180usize, 209usize, 227usize, 247usize,],
        "result != expected"
    );
    Ok(())
}
#[test]
fn nested_lists_with_a_few_zero_atoms_with_query_path_any_then_0_with_buffered_input_and_count_result_using_main_engine(
) -> Result<(), Box<dyn Error>> {
    println ! ("on document nested_arrays.toml running the query $.*[0] (path any, then 0) with Input impl BufferedInput and result mode CountResult");
    let jsonpath_query = JsonPathQuery::parse("$.*[0]")?;
    let json_file = fs::File::open("/home/mat/rsonpath/crates/rsonpath-test/tests/documents/toml/nested_arrays.json")?;
    let input = BufferedInput::new(json_file);
    let engine = MainEngine::compile_query(&jsonpath_query)?;
    let result = engine.run::<_, CountResult>(&input)?;
    assert_eq!(result.get(), 2u64, "result != expected");
    Ok(())
}
#[test]
fn nested_lists_with_a_few_zero_atoms_with_query_path_any_then_0_with_buffered_input_and_count_result_using_recursive_engine(
) -> Result<(), Box<dyn Error>> {
    println ! ("on document nested_arrays.toml running the query $.*[0] (path any, then 0) with Input impl BufferedInput and result mode CountResult");
    let jsonpath_query = JsonPathQuery::parse("$.*[0]")?;
    let json_file = fs::File::open("/home/mat/rsonpath/crates/rsonpath-test/tests/documents/toml/nested_arrays.json")?;
    let input = BufferedInput::new(json_file);
    let engine = RecursiveEngine::compile_query(&jsonpath_query)?;
    let result = engine.run::<_, CountResult>(&input)?;
    assert_eq!(result.get(), 2u64, "result != expected");
    Ok(())
}
#[test]
fn nested_lists_with_a_few_zero_atoms_with_query_path_any_then_0_with_buffered_input_and_index_result_using_main_engine(
) -> Result<(), Box<dyn Error>> {
    println ! ("on document nested_arrays.toml running the query $.*[0] (path any, then 0) with Input impl BufferedInput and result mode IndexResult");
    let jsonpath_query = JsonPathQuery::parse("$.*[0]")?;
    let json_file = fs::File::open("/home/mat/rsonpath/crates/rsonpath-test/tests/documents/toml/nested_arrays.json")?;
    let input = BufferedInput::new(json_file);
    let engine = MainEngine::compile_query(&jsonpath_query)?;
    let result = engine.run::<_, IndexResult>(&input)?;
    assert_eq!(result.get(), vec![16usize, 49usize,], "result != expected");
    Ok(())
}
#[test]
fn nested_lists_with_a_few_zero_atoms_with_query_path_any_then_0_with_buffered_input_and_index_result_using_recursive_engine(
) -> Result<(), Box<dyn Error>> {
    println ! ("on document nested_arrays.toml running the query $.*[0] (path any, then 0) with Input impl BufferedInput and result mode IndexResult");
    let jsonpath_query = JsonPathQuery::parse("$.*[0]")?;
    let json_file = fs::File::open("/home/mat/rsonpath/crates/rsonpath-test/tests/documents/toml/nested_arrays.json")?;
    let input = BufferedInput::new(json_file);
    let engine = RecursiveEngine::compile_query(&jsonpath_query)?;
    let result = engine.run::<_, IndexResult>(&input)?;
    assert_eq!(result.get(), vec![16usize, 49usize,], "result != expected");
    Ok(())
}
#[test]
fn nested_lists_with_a_few_zero_atoms_with_query_path_any_then_0_with_mmap_input_and_count_result_using_main_engine(
) -> Result<(), Box<dyn Error>> {
    println ! ("on document nested_arrays.toml running the query $.*[0] (path any, then 0) with Input impl MmapInput and result mode CountResult");
    let jsonpath_query = JsonPathQuery::parse("$.*[0]")?;
    let json_file = fs::File::open("/home/mat/rsonpath/crates/rsonpath-test/tests/documents/toml/nested_arrays.json")?;
    let input = unsafe { MmapInput::map_file(&json_file)? };
    let engine = MainEngine::compile_query(&jsonpath_query)?;
    let result = engine.run::<_, CountResult>(&input)?;
    assert_eq!(result.get(), 2u64, "result != expected");
    Ok(())
}
#[test]
fn nested_lists_with_a_few_zero_atoms_with_query_path_any_then_0_with_mmap_input_and_count_result_using_recursive_engine(
) -> Result<(), Box<dyn Error>> {
    println ! ("on document nested_arrays.toml running the query $.*[0] (path any, then 0) with Input impl MmapInput and result mode CountResult");
    let jsonpath_query = JsonPathQuery::parse("$.*[0]")?;
    let json_file = fs::File::open("/home/mat/rsonpath/crates/rsonpath-test/tests/documents/toml/nested_arrays.json")?;
    let input = unsafe { MmapInput::map_file(&json_file)? };
    let engine = RecursiveEngine::compile_query(&jsonpath_query)?;
    let result = engine.run::<_, CountResult>(&input)?;
    assert_eq!(result.get(), 2u64, "result != expected");
    Ok(())
}
#[test]
fn nested_lists_with_a_few_zero_atoms_with_query_path_any_then_0_with_mmap_input_and_index_result_using_main_engine(
) -> Result<(), Box<dyn Error>> {
    println ! ("on document nested_arrays.toml running the query $.*[0] (path any, then 0) with Input impl MmapInput and result mode IndexResult");
    let jsonpath_query = JsonPathQuery::parse("$.*[0]")?;
    let json_file = fs::File::open("/home/mat/rsonpath/crates/rsonpath-test/tests/documents/toml/nested_arrays.json")?;
    let input = unsafe { MmapInput::map_file(&json_file)? };
    let engine = MainEngine::compile_query(&jsonpath_query)?;
    let result = engine.run::<_, IndexResult>(&input)?;
    assert_eq!(result.get(), vec![16usize, 49usize,], "result != expected");
    Ok(())
}
#[test]
fn nested_lists_with_a_few_zero_atoms_with_query_path_any_then_0_with_mmap_input_and_index_result_using_recursive_engine(
) -> Result<(), Box<dyn Error>> {
    println ! ("on document nested_arrays.toml running the query $.*[0] (path any, then 0) with Input impl MmapInput and result mode IndexResult");
    let jsonpath_query = JsonPathQuery::parse("$.*[0]")?;
    let json_file = fs::File::open("/home/mat/rsonpath/crates/rsonpath-test/tests/documents/toml/nested_arrays.json")?;
    let input = unsafe { MmapInput::map_file(&json_file)? };
    let engine = RecursiveEngine::compile_query(&jsonpath_query)?;
    let result = engine.run::<_, IndexResult>(&input)?;
    assert_eq!(result.get(), vec![16usize, 49usize,], "result != expected");
    Ok(())
}
#[test]
fn nested_lists_with_a_few_zero_atoms_with_query_path_any_then_0_with_owned_bytes_and_count_result_using_main_engine(
) -> Result<(), Box<dyn Error>> {
    println ! ("on document nested_arrays.toml running the query $.*[0] (path any, then 0) with Input impl OwnedBytes and result mode CountResult");
    let jsonpath_query = JsonPathQuery::parse("$.*[0]")?;
    let raw_json =
        fs::read_to_string("/home/mat/rsonpath/crates/rsonpath-test/tests/documents/toml/nested_arrays.json")?;
    let input = OwnedBytes::new(&raw_json.as_bytes())?;
    let engine = MainEngine::compile_query(&jsonpath_query)?;
    let result = engine.run::<_, CountResult>(&input)?;
    assert_eq!(result.get(), 2u64, "result != expected");
    Ok(())
}
#[test]
fn nested_lists_with_a_few_zero_atoms_with_query_path_any_then_0_with_owned_bytes_and_count_result_using_recursive_engine(
) -> Result<(), Box<dyn Error>> {
    println ! ("on document nested_arrays.toml running the query $.*[0] (path any, then 0) with Input impl OwnedBytes and result mode CountResult");
    let jsonpath_query = JsonPathQuery::parse("$.*[0]")?;
    let raw_json =
        fs::read_to_string("/home/mat/rsonpath/crates/rsonpath-test/tests/documents/toml/nested_arrays.json")?;
    let input = OwnedBytes::new(&raw_json.as_bytes())?;
    let engine = RecursiveEngine::compile_query(&jsonpath_query)?;
    let result = engine.run::<_, CountResult>(&input)?;
    assert_eq!(result.get(), 2u64, "result != expected");
    Ok(())
}
#[test]
fn nested_lists_with_a_few_zero_atoms_with_query_path_any_then_0_with_owned_bytes_and_index_result_using_main_engine(
) -> Result<(), Box<dyn Error>> {
    println ! ("on document nested_arrays.toml running the query $.*[0] (path any, then 0) with Input impl OwnedBytes and result mode IndexResult");
    let jsonpath_query = JsonPathQuery::parse("$.*[0]")?;
    let raw_json =
        fs::read_to_string("/home/mat/rsonpath/crates/rsonpath-test/tests/documents/toml/nested_arrays.json")?;
    let input = OwnedBytes::new(&raw_json.as_bytes())?;
    let engine = MainEngine::compile_query(&jsonpath_query)?;
    let result = engine.run::<_, IndexResult>(&input)?;
    assert_eq!(result.get(), vec![16usize, 49usize,], "result != expected");
    Ok(())
}
#[test]
fn nested_lists_with_a_few_zero_atoms_with_query_path_any_then_0_with_owned_bytes_and_index_result_using_recursive_engine(
) -> Result<(), Box<dyn Error>> {
    println ! ("on document nested_arrays.toml running the query $.*[0] (path any, then 0) with Input impl OwnedBytes and result mode IndexResult");
    let jsonpath_query = JsonPathQuery::parse("$.*[0]")?;
    let raw_json =
        fs::read_to_string("/home/mat/rsonpath/crates/rsonpath-test/tests/documents/toml/nested_arrays.json")?;
    let input = OwnedBytes::new(&raw_json.as_bytes())?;
    let engine = RecursiveEngine::compile_query(&jsonpath_query)?;
    let result = engine.run::<_, IndexResult>(&input)?;
    assert_eq!(result.get(), vec![16usize, 49usize,], "result != expected");
    Ok(())
}
#[test]
fn nested_lists_with_a_few_zero_atoms_with_query_recurse_any_then_0_with_buffered_input_and_count_result_using_main_engine(
) -> Result<(), Box<dyn Error>> {
    println ! ("on document nested_arrays.toml running the query $..*[0] (recurse, any, then 0) with Input impl BufferedInput and result mode CountResult");
    let jsonpath_query = JsonPathQuery::parse("$..*[0]")?;
    let json_file = fs::File::open("/home/mat/rsonpath/crates/rsonpath-test/tests/documents/toml/nested_arrays.json")?;
    let input = BufferedInput::new(json_file);
    let engine = MainEngine::compile_query(&jsonpath_query)?;
    let result = engine.run::<_, CountResult>(&input)?;
    assert_eq!(result.get(), 7u64, "result != expected");
    Ok(())
}
#[test]
fn nested_lists_with_a_few_zero_atoms_with_query_recurse_any_then_0_with_buffered_input_and_count_result_using_recursive_engine(
) -> Result<(), Box<dyn Error>> {
    println ! ("on document nested_arrays.toml running the query $..*[0] (recurse, any, then 0) with Input impl BufferedInput and result mode CountResult");
    let jsonpath_query = JsonPathQuery::parse("$..*[0]")?;
    let json_file = fs::File::open("/home/mat/rsonpath/crates/rsonpath-test/tests/documents/toml/nested_arrays.json")?;
    let input = BufferedInput::new(json_file);
    let engine = RecursiveEngine::compile_query(&jsonpath_query)?;
    let result = engine.run::<_, CountResult>(&input)?;
    assert_eq!(result.get(), 7u64, "result != expected");
    Ok(())
}
#[test]
fn nested_lists_with_a_few_zero_atoms_with_query_recurse_any_then_0_with_buffered_input_and_index_result_using_main_engine(
) -> Result<(), Box<dyn Error>> {
    println ! ("on document nested_arrays.toml running the query $..*[0] (recurse, any, then 0) with Input impl BufferedInput and result mode IndexResult");
    let jsonpath_query = JsonPathQuery::parse("$..*[0]")?;
    let json_file = fs::File::open("/home/mat/rsonpath/crates/rsonpath-test/tests/documents/toml/nested_arrays.json")?;
    let input = BufferedInput::new(json_file);
    let engine = MainEngine::compile_query(&jsonpath_query)?;
    let result = engine.run::<_, IndexResult>(&input)?;
    assert_eq!(
        result.get(),
        vec![16usize, 17usize, 49usize, 75usize, 93usize, 160usize, 227usize,],
        "result != expected"
    );
    Ok(())
}
#[test]
fn nested_lists_with_a_few_zero_atoms_with_query_recurse_any_then_0_with_buffered_input_and_index_result_using_recursive_engine(
) -> Result<(), Box<dyn Error>> {
    println ! ("on document nested_arrays.toml running the query $..*[0] (recurse, any, then 0) with Input impl BufferedInput and result mode IndexResult");
    let jsonpath_query = JsonPathQuery::parse("$..*[0]")?;
    let json_file = fs::File::open("/home/mat/rsonpath/crates/rsonpath-test/tests/documents/toml/nested_arrays.json")?;
    let input = BufferedInput::new(json_file);
    let engine = RecursiveEngine::compile_query(&jsonpath_query)?;
    let result = engine.run::<_, IndexResult>(&input)?;
    assert_eq!(
        result.get(),
        vec![16usize, 17usize, 49usize, 75usize, 93usize, 160usize, 227usize,],
        "result != expected"
    );
    Ok(())
}
#[test]
fn nested_lists_with_a_few_zero_atoms_with_query_recurse_any_then_0_with_mmap_input_and_count_result_using_main_engine(
) -> Result<(), Box<dyn Error>> {
    println ! ("on document nested_arrays.toml running the query $..*[0] (recurse, any, then 0) with Input impl MmapInput and result mode CountResult");
    let jsonpath_query = JsonPathQuery::parse("$..*[0]")?;
    let json_file = fs::File::open("/home/mat/rsonpath/crates/rsonpath-test/tests/documents/toml/nested_arrays.json")?;
    let input = unsafe { MmapInput::map_file(&json_file)? };
    let engine = MainEngine::compile_query(&jsonpath_query)?;
    let result = engine.run::<_, CountResult>(&input)?;
    assert_eq!(result.get(), 7u64, "result != expected");
    Ok(())
}
#[test]
fn nested_lists_with_a_few_zero_atoms_with_query_recurse_any_then_0_with_mmap_input_and_count_result_using_recursive_engine(
) -> Result<(), Box<dyn Error>> {
    println ! ("on document nested_arrays.toml running the query $..*[0] (recurse, any, then 0) with Input impl MmapInput and result mode CountResult");
    let jsonpath_query = JsonPathQuery::parse("$..*[0]")?;
    let json_file = fs::File::open("/home/mat/rsonpath/crates/rsonpath-test/tests/documents/toml/nested_arrays.json")?;
    let input = unsafe { MmapInput::map_file(&json_file)? };
    let engine = RecursiveEngine::compile_query(&jsonpath_query)?;
    let result = engine.run::<_, CountResult>(&input)?;
    assert_eq!(result.get(), 7u64, "result != expected");
    Ok(())
}
#[test]
fn nested_lists_with_a_few_zero_atoms_with_query_recurse_any_then_0_with_mmap_input_and_index_result_using_main_engine(
) -> Result<(), Box<dyn Error>> {
    println ! ("on document nested_arrays.toml running the query $..*[0] (recurse, any, then 0) with Input impl MmapInput and result mode IndexResult");
    let jsonpath_query = JsonPathQuery::parse("$..*[0]")?;
    let json_file = fs::File::open("/home/mat/rsonpath/crates/rsonpath-test/tests/documents/toml/nested_arrays.json")?;
    let input = unsafe { MmapInput::map_file(&json_file)? };
    let engine = MainEngine::compile_query(&jsonpath_query)?;
    let result = engine.run::<_, IndexResult>(&input)?;
    assert_eq!(
        result.get(),
        vec![16usize, 17usize, 49usize, 75usize, 93usize, 160usize, 227usize,],
        "result != expected"
    );
    Ok(())
}
#[test]
fn nested_lists_with_a_few_zero_atoms_with_query_recurse_any_then_0_with_mmap_input_and_index_result_using_recursive_engine(
) -> Result<(), Box<dyn Error>> {
    println ! ("on document nested_arrays.toml running the query $..*[0] (recurse, any, then 0) with Input impl MmapInput and result mode IndexResult");
    let jsonpath_query = JsonPathQuery::parse("$..*[0]")?;
    let json_file = fs::File::open("/home/mat/rsonpath/crates/rsonpath-test/tests/documents/toml/nested_arrays.json")?;
    let input = unsafe { MmapInput::map_file(&json_file)? };
    let engine = RecursiveEngine::compile_query(&jsonpath_query)?;
    let result = engine.run::<_, IndexResult>(&input)?;
    assert_eq!(
        result.get(),
        vec![16usize, 17usize, 49usize, 75usize, 93usize, 160usize, 227usize,],
        "result != expected"
    );
    Ok(())
}
#[test]
fn nested_lists_with_a_few_zero_atoms_with_query_recurse_any_then_0_with_owned_bytes_and_count_result_using_main_engine(
) -> Result<(), Box<dyn Error>> {
    println ! ("on document nested_arrays.toml running the query $..*[0] (recurse, any, then 0) with Input impl OwnedBytes and result mode CountResult");
    let jsonpath_query = JsonPathQuery::parse("$..*[0]")?;
    let raw_json =
        fs::read_to_string("/home/mat/rsonpath/crates/rsonpath-test/tests/documents/toml/nested_arrays.json")?;
    let input = OwnedBytes::new(&raw_json.as_bytes())?;
    let engine = MainEngine::compile_query(&jsonpath_query)?;
    let result = engine.run::<_, CountResult>(&input)?;
    assert_eq!(result.get(), 7u64, "result != expected");
    Ok(())
}
#[test]
fn nested_lists_with_a_few_zero_atoms_with_query_recurse_any_then_0_with_owned_bytes_and_count_result_using_recursive_engine(
) -> Result<(), Box<dyn Error>> {
    println ! ("on document nested_arrays.toml running the query $..*[0] (recurse, any, then 0) with Input impl OwnedBytes and result mode CountResult");
    let jsonpath_query = JsonPathQuery::parse("$..*[0]")?;
    let raw_json =
        fs::read_to_string("/home/mat/rsonpath/crates/rsonpath-test/tests/documents/toml/nested_arrays.json")?;
    let input = OwnedBytes::new(&raw_json.as_bytes())?;
    let engine = RecursiveEngine::compile_query(&jsonpath_query)?;
    let result = engine.run::<_, CountResult>(&input)?;
    assert_eq!(result.get(), 7u64, "result != expected");
    Ok(())
}
#[test]
fn nested_lists_with_a_few_zero_atoms_with_query_recurse_any_then_0_with_owned_bytes_and_index_result_using_main_engine(
) -> Result<(), Box<dyn Error>> {
    println ! ("on document nested_arrays.toml running the query $..*[0] (recurse, any, then 0) with Input impl OwnedBytes and result mode IndexResult");
    let jsonpath_query = JsonPathQuery::parse("$..*[0]")?;
    let raw_json =
        fs::read_to_string("/home/mat/rsonpath/crates/rsonpath-test/tests/documents/toml/nested_arrays.json")?;
    let input = OwnedBytes::new(&raw_json.as_bytes())?;
    let engine = MainEngine::compile_query(&jsonpath_query)?;
    let result = engine.run::<_, IndexResult>(&input)?;
    assert_eq!(
        result.get(),
        vec![16usize, 17usize, 49usize, 75usize, 93usize, 160usize, 227usize,],
        "result != expected"
    );
    Ok(())
}
#[test]
fn nested_lists_with_a_few_zero_atoms_with_query_recurse_any_then_0_with_owned_bytes_and_index_result_using_recursive_engine(
) -> Result<(), Box<dyn Error>> {
    println ! ("on document nested_arrays.toml running the query $..*[0] (recurse, any, then 0) with Input impl OwnedBytes and result mode IndexResult");
    let jsonpath_query = JsonPathQuery::parse("$..*[0]")?;
    let raw_json =
        fs::read_to_string("/home/mat/rsonpath/crates/rsonpath-test/tests/documents/toml/nested_arrays.json")?;
    let input = OwnedBytes::new(&raw_json.as_bytes())?;
    let engine = RecursiveEngine::compile_query(&jsonpath_query)?;
    let result = engine.run::<_, IndexResult>(&input)?;
    assert_eq!(
        result.get(),
        vec![16usize, 17usize, 49usize, 75usize, 93usize, 160usize, 227usize,],
        "result != expected"
    );
    Ok(())
}
#[test]
fn nested_lists_with_a_few_zero_atoms_with_query_recurse_then_0_including_immediate_with_buffered_input_and_count_result_using_main_engine(
) -> Result<(), Box<dyn Error>> {
    println ! ("on document nested_arrays.toml running the query $..[0] (recurse, then 0, including immediate) with Input impl BufferedInput and result mode CountResult");
    let jsonpath_query = JsonPathQuery::parse("$..[0]")?;
    let json_file = fs::File::open("/home/mat/rsonpath/crates/rsonpath-test/tests/documents/toml/nested_arrays.json")?;
    let input = BufferedInput::new(json_file);
    let engine = MainEngine::compile_query(&jsonpath_query)?;
    let result = engine.run::<_, CountResult>(&input)?;
    assert_eq!(result.get(), 8u64, "result != expected");
    Ok(())
}
#[test]
fn nested_lists_with_a_few_zero_atoms_with_query_recurse_then_0_including_immediate_with_buffered_input_and_count_result_using_recursive_engine(
) -> Result<(), Box<dyn Error>> {
    println ! ("on document nested_arrays.toml running the query $..[0] (recurse, then 0, including immediate) with Input impl BufferedInput and result mode CountResult");
    let jsonpath_query = JsonPathQuery::parse("$..[0]")?;
    let json_file = fs::File::open("/home/mat/rsonpath/crates/rsonpath-test/tests/documents/toml/nested_arrays.json")?;
    let input = BufferedInput::new(json_file);
    let engine = RecursiveEngine::compile_query(&jsonpath_query)?;
    let result = engine.run::<_, CountResult>(&input)?;
    assert_eq!(result.get(), 8u64, "result != expected");
    Ok(())
}
#[test]
fn nested_lists_with_a_few_zero_atoms_with_query_recurse_then_0_including_immediate_with_buffered_input_and_index_result_using_main_engine(
) -> Result<(), Box<dyn Error>> {
    println ! ("on document nested_arrays.toml running the query $..[0] (recurse, then 0, including immediate) with Input impl BufferedInput and result mode IndexResult");
    let jsonpath_query = JsonPathQuery::parse("$..[0]")?;
    let json_file = fs::File::open("/home/mat/rsonpath/crates/rsonpath-test/tests/documents/toml/nested_arrays.json")?;
    let input = BufferedInput::new(json_file);
    let engine = MainEngine::compile_query(&jsonpath_query)?;
    let result = engine.run::<_, IndexResult>(&input)?;
    assert_eq!(
        result.get(),
        vec![6usize, 16usize, 17usize, 49usize, 75usize, 93usize, 160usize, 227usize,],
        "result != expected"
    );
    Ok(())
}
#[test]
fn nested_lists_with_a_few_zero_atoms_with_query_recurse_then_0_including_immediate_with_buffered_input_and_index_result_using_recursive_engine(
) -> Result<(), Box<dyn Error>> {
    println ! ("on document nested_arrays.toml running the query $..[0] (recurse, then 0, including immediate) with Input impl BufferedInput and result mode IndexResult");
    let jsonpath_query = JsonPathQuery::parse("$..[0]")?;
    let json_file = fs::File::open("/home/mat/rsonpath/crates/rsonpath-test/tests/documents/toml/nested_arrays.json")?;
    let input = BufferedInput::new(json_file);
    let engine = RecursiveEngine::compile_query(&jsonpath_query)?;
    let result = engine.run::<_, IndexResult>(&input)?;
    assert_eq!(
        result.get(),
        vec![6usize, 16usize, 17usize, 49usize, 75usize, 93usize, 160usize, 227usize,],
        "result != expected"
    );
    Ok(())
}
#[test]
fn nested_lists_with_a_few_zero_atoms_with_query_recurse_then_0_including_immediate_with_mmap_input_and_count_result_using_main_engine(
) -> Result<(), Box<dyn Error>> {
    println ! ("on document nested_arrays.toml running the query $..[0] (recurse, then 0, including immediate) with Input impl MmapInput and result mode CountResult");
    let jsonpath_query = JsonPathQuery::parse("$..[0]")?;
    let json_file = fs::File::open("/home/mat/rsonpath/crates/rsonpath-test/tests/documents/toml/nested_arrays.json")?;
    let input = unsafe { MmapInput::map_file(&json_file)? };
    let engine = MainEngine::compile_query(&jsonpath_query)?;
    let result = engine.run::<_, CountResult>(&input)?;
    assert_eq!(result.get(), 8u64, "result != expected");
    Ok(())
}
#[test]
fn nested_lists_with_a_few_zero_atoms_with_query_recurse_then_0_including_immediate_with_mmap_input_and_count_result_using_recursive_engine(
) -> Result<(), Box<dyn Error>> {
    println ! ("on document nested_arrays.toml running the query $..[0] (recurse, then 0, including immediate) with Input impl MmapInput and result mode CountResult");
    let jsonpath_query = JsonPathQuery::parse("$..[0]")?;
    let json_file = fs::File::open("/home/mat/rsonpath/crates/rsonpath-test/tests/documents/toml/nested_arrays.json")?;
    let input = unsafe { MmapInput::map_file(&json_file)? };
    let engine = RecursiveEngine::compile_query(&jsonpath_query)?;
    let result = engine.run::<_, CountResult>(&input)?;
    assert_eq!(result.get(), 8u64, "result != expected");
    Ok(())
}
#[test]
fn nested_lists_with_a_few_zero_atoms_with_query_recurse_then_0_including_immediate_with_mmap_input_and_index_result_using_main_engine(
) -> Result<(), Box<dyn Error>> {
    println ! ("on document nested_arrays.toml running the query $..[0] (recurse, then 0, including immediate) with Input impl MmapInput and result mode IndexResult");
    let jsonpath_query = JsonPathQuery::parse("$..[0]")?;
    let json_file = fs::File::open("/home/mat/rsonpath/crates/rsonpath-test/tests/documents/toml/nested_arrays.json")?;
    let input = unsafe { MmapInput::map_file(&json_file)? };
    let engine = MainEngine::compile_query(&jsonpath_query)?;
    let result = engine.run::<_, IndexResult>(&input)?;
    assert_eq!(
        result.get(),
        vec![6usize, 16usize, 17usize, 49usize, 75usize, 93usize, 160usize, 227usize,],
        "result != expected"
    );
    Ok(())
}
#[test]
fn nested_lists_with_a_few_zero_atoms_with_query_recurse_then_0_including_immediate_with_mmap_input_and_index_result_using_recursive_engine(
) -> Result<(), Box<dyn Error>> {
    println ! ("on document nested_arrays.toml running the query $..[0] (recurse, then 0, including immediate) with Input impl MmapInput and result mode IndexResult");
    let jsonpath_query = JsonPathQuery::parse("$..[0]")?;
    let json_file = fs::File::open("/home/mat/rsonpath/crates/rsonpath-test/tests/documents/toml/nested_arrays.json")?;
    let input = unsafe { MmapInput::map_file(&json_file)? };
    let engine = RecursiveEngine::compile_query(&jsonpath_query)?;
    let result = engine.run::<_, IndexResult>(&input)?;
    assert_eq!(
        result.get(),
        vec![6usize, 16usize, 17usize, 49usize, 75usize, 93usize, 160usize, 227usize,],
        "result != expected"
    );
    Ok(())
}
#[test]
fn nested_lists_with_a_few_zero_atoms_with_query_recurse_then_0_including_immediate_with_owned_bytes_and_count_result_using_main_engine(
) -> Result<(), Box<dyn Error>> {
    println ! ("on document nested_arrays.toml running the query $..[0] (recurse, then 0, including immediate) with Input impl OwnedBytes and result mode CountResult");
    let jsonpath_query = JsonPathQuery::parse("$..[0]")?;
    let raw_json =
        fs::read_to_string("/home/mat/rsonpath/crates/rsonpath-test/tests/documents/toml/nested_arrays.json")?;
    let input = OwnedBytes::new(&raw_json.as_bytes())?;
    let engine = MainEngine::compile_query(&jsonpath_query)?;
    let result = engine.run::<_, CountResult>(&input)?;
    assert_eq!(result.get(), 8u64, "result != expected");
    Ok(())
}
#[test]
fn nested_lists_with_a_few_zero_atoms_with_query_recurse_then_0_including_immediate_with_owned_bytes_and_count_result_using_recursive_engine(
) -> Result<(), Box<dyn Error>> {
    println ! ("on document nested_arrays.toml running the query $..[0] (recurse, then 0, including immediate) with Input impl OwnedBytes and result mode CountResult");
    let jsonpath_query = JsonPathQuery::parse("$..[0]")?;
    let raw_json =
        fs::read_to_string("/home/mat/rsonpath/crates/rsonpath-test/tests/documents/toml/nested_arrays.json")?;
    let input = OwnedBytes::new(&raw_json.as_bytes())?;
    let engine = RecursiveEngine::compile_query(&jsonpath_query)?;
    let result = engine.run::<_, CountResult>(&input)?;
    assert_eq!(result.get(), 8u64, "result != expected");
    Ok(())
}
#[test]
fn nested_lists_with_a_few_zero_atoms_with_query_recurse_then_0_including_immediate_with_owned_bytes_and_index_result_using_main_engine(
) -> Result<(), Box<dyn Error>> {
    println ! ("on document nested_arrays.toml running the query $..[0] (recurse, then 0, including immediate) with Input impl OwnedBytes and result mode IndexResult");
    let jsonpath_query = JsonPathQuery::parse("$..[0]")?;
    let raw_json =
        fs::read_to_string("/home/mat/rsonpath/crates/rsonpath-test/tests/documents/toml/nested_arrays.json")?;
    let input = OwnedBytes::new(&raw_json.as_bytes())?;
    let engine = MainEngine::compile_query(&jsonpath_query)?;
    let result = engine.run::<_, IndexResult>(&input)?;
    assert_eq!(
        result.get(),
        vec![6usize, 16usize, 17usize, 49usize, 75usize, 93usize, 160usize, 227usize,],
        "result != expected"
    );
    Ok(())
}
#[test]
fn nested_lists_with_a_few_zero_atoms_with_query_recurse_then_0_including_immediate_with_owned_bytes_and_index_result_using_recursive_engine(
) -> Result<(), Box<dyn Error>> {
    println ! ("on document nested_arrays.toml running the query $..[0] (recurse, then 0, including immediate) with Input impl OwnedBytes and result mode IndexResult");
    let jsonpath_query = JsonPathQuery::parse("$..[0]")?;
    let raw_json =
        fs::read_to_string("/home/mat/rsonpath/crates/rsonpath-test/tests/documents/toml/nested_arrays.json")?;
    let input = OwnedBytes::new(&raw_json.as_bytes())?;
    let engine = RecursiveEngine::compile_query(&jsonpath_query)?;
    let result = engine.run::<_, IndexResult>(&input)?;
    assert_eq!(
        result.get(),
        vec![6usize, 16usize, 17usize, 49usize, 75usize, 93usize, 160usize, 227usize,],
        "result != expected"
    );
    Ok(())
}
#[test]
fn object_with_two_labels_x_and_x_with_a_preceding_escaped_double_quote_with_query_select_x_directly_with_buffered_input_and_count_result_using_main_engine(
) -> Result<(), Box<dyn Error>> {
    println ! ("on document quote_escape.toml running the query $['x'] (select 'x' directly) with Input impl BufferedInput and result mode CountResult");
    let jsonpath_query = JsonPathQuery::parse("$['x']")?;
    let json_file = fs::File::open("/home/mat/rsonpath/crates/rsonpath-test/tests/documents/toml/quote_escape.json")?;
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
    let json_file = fs::File::open("/home/mat/rsonpath/crates/rsonpath-test/tests/documents/toml/quote_escape.json")?;
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
    let json_file = fs::File::open("/home/mat/rsonpath/crates/rsonpath-test/tests/documents/toml/quote_escape.json")?;
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
    let json_file = fs::File::open("/home/mat/rsonpath/crates/rsonpath-test/tests/documents/toml/quote_escape.json")?;
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
    let json_file = fs::File::open("/home/mat/rsonpath/crates/rsonpath-test/tests/documents/toml/quote_escape.json")?;
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
    let json_file = fs::File::open("/home/mat/rsonpath/crates/rsonpath-test/tests/documents/toml/quote_escape.json")?;
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
    let json_file = fs::File::open("/home/mat/rsonpath/crates/rsonpath-test/tests/documents/toml/quote_escape.json")?;
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
    let json_file = fs::File::open("/home/mat/rsonpath/crates/rsonpath-test/tests/documents/toml/quote_escape.json")?;
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
    let raw_json =
        fs::read_to_string("/home/mat/rsonpath/crates/rsonpath-test/tests/documents/toml/quote_escape.json")?;
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
    let raw_json =
        fs::read_to_string("/home/mat/rsonpath/crates/rsonpath-test/tests/documents/toml/quote_escape.json")?;
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
    let raw_json =
        fs::read_to_string("/home/mat/rsonpath/crates/rsonpath-test/tests/documents/toml/quote_escape.json")?;
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
    let raw_json =
        fs::read_to_string("/home/mat/rsonpath/crates/rsonpath-test/tests/documents/toml/quote_escape.json")?;
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
    let json_file = fs::File::open("/home/mat/rsonpath/crates/rsonpath-test/tests/documents/toml/quote_escape.json")?;
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
    let json_file = fs::File::open("/home/mat/rsonpath/crates/rsonpath-test/tests/documents/toml/quote_escape.json")?;
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
    let json_file = fs::File::open("/home/mat/rsonpath/crates/rsonpath-test/tests/documents/toml/quote_escape.json")?;
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
    let json_file = fs::File::open("/home/mat/rsonpath/crates/rsonpath-test/tests/documents/toml/quote_escape.json")?;
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
    let json_file = fs::File::open("/home/mat/rsonpath/crates/rsonpath-test/tests/documents/toml/quote_escape.json")?;
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
    let json_file = fs::File::open("/home/mat/rsonpath/crates/rsonpath-test/tests/documents/toml/quote_escape.json")?;
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
    let json_file = fs::File::open("/home/mat/rsonpath/crates/rsonpath-test/tests/documents/toml/quote_escape.json")?;
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
    let json_file = fs::File::open("/home/mat/rsonpath/crates/rsonpath-test/tests/documents/toml/quote_escape.json")?;
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
    let raw_json =
        fs::read_to_string("/home/mat/rsonpath/crates/rsonpath-test/tests/documents/toml/quote_escape.json")?;
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
    let raw_json =
        fs::read_to_string("/home/mat/rsonpath/crates/rsonpath-test/tests/documents/toml/quote_escape.json")?;
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
    let raw_json =
        fs::read_to_string("/home/mat/rsonpath/crates/rsonpath-test/tests/documents/toml/quote_escape.json")?;
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
    let raw_json =
        fs::read_to_string("/home/mat/rsonpath/crates/rsonpath-test/tests/documents/toml/quote_escape.json")?;
    let input = OwnedBytes::new(&raw_json.as_bytes())?;
    let engine = RecursiveEngine::compile_query(&jsonpath_query)?;
    let result = engine.run::<_, IndexResult>(&input)?;
    assert_eq!(result.get(), vec![13usize,], "result != expected");
    Ok(())
}
#[test]
fn short_json_with_objects_and_lists_given_as_an_example_on_jsonpath_com_with_query_descendant_search_for_number_with_buffered_input_and_count_result_using_main_engine(
) -> Result<(), Box<dyn Error>> {
    println ! ("on document jsonpath_com_example.toml running the query $..number (descendant search for 'number') with Input impl BufferedInput and result mode CountResult");
    let jsonpath_query = JsonPathQuery::parse("$..number")?;
    let json_file =
        fs::File::open("/home/mat/rsonpath/crates/rsonpath-test/tests/documents/toml/jsonpath_com_example.json")?;
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
    let json_file =
        fs::File::open("/home/mat/rsonpath/crates/rsonpath-test/tests/documents/toml/jsonpath_com_example.json")?;
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
    let json_file =
        fs::File::open("/home/mat/rsonpath/crates/rsonpath-test/tests/documents/toml/jsonpath_com_example.json")?;
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
    let json_file =
        fs::File::open("/home/mat/rsonpath/crates/rsonpath-test/tests/documents/toml/jsonpath_com_example.json")?;
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
    let json_file =
        fs::File::open("/home/mat/rsonpath/crates/rsonpath-test/tests/documents/toml/jsonpath_com_example.json")?;
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
    let json_file =
        fs::File::open("/home/mat/rsonpath/crates/rsonpath-test/tests/documents/toml/jsonpath_com_example.json")?;
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
    let json_file =
        fs::File::open("/home/mat/rsonpath/crates/rsonpath-test/tests/documents/toml/jsonpath_com_example.json")?;
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
    let json_file =
        fs::File::open("/home/mat/rsonpath/crates/rsonpath-test/tests/documents/toml/jsonpath_com_example.json")?;
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
    let raw_json =
        fs::read_to_string("/home/mat/rsonpath/crates/rsonpath-test/tests/documents/toml/jsonpath_com_example.json")?;
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
    let raw_json =
        fs::read_to_string("/home/mat/rsonpath/crates/rsonpath-test/tests/documents/toml/jsonpath_com_example.json")?;
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
    let raw_json =
        fs::read_to_string("/home/mat/rsonpath/crates/rsonpath-test/tests/documents/toml/jsonpath_com_example.json")?;
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
    let raw_json =
        fs::read_to_string("/home/mat/rsonpath/crates/rsonpath-test/tests/documents/toml/jsonpath_com_example.json")?;
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
    let json_file =
        fs::File::open("/home/mat/rsonpath/crates/rsonpath-test/tests/documents/toml/jsonpath_com_example.json")?;
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
    let json_file =
        fs::File::open("/home/mat/rsonpath/crates/rsonpath-test/tests/documents/toml/jsonpath_com_example.json")?;
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
    let json_file =
        fs::File::open("/home/mat/rsonpath/crates/rsonpath-test/tests/documents/toml/jsonpath_com_example.json")?;
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
    let json_file =
        fs::File::open("/home/mat/rsonpath/crates/rsonpath-test/tests/documents/toml/jsonpath_com_example.json")?;
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
    let json_file =
        fs::File::open("/home/mat/rsonpath/crates/rsonpath-test/tests/documents/toml/jsonpath_com_example.json")?;
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
    let json_file =
        fs::File::open("/home/mat/rsonpath/crates/rsonpath-test/tests/documents/toml/jsonpath_com_example.json")?;
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
    let json_file =
        fs::File::open("/home/mat/rsonpath/crates/rsonpath-test/tests/documents/toml/jsonpath_com_example.json")?;
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
    let json_file =
        fs::File::open("/home/mat/rsonpath/crates/rsonpath-test/tests/documents/toml/jsonpath_com_example.json")?;
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
    let raw_json =
        fs::read_to_string("/home/mat/rsonpath/crates/rsonpath-test/tests/documents/toml/jsonpath_com_example.json")?;
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
    let raw_json =
        fs::read_to_string("/home/mat/rsonpath/crates/rsonpath-test/tests/documents/toml/jsonpath_com_example.json")?;
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
    let raw_json =
        fs::read_to_string("/home/mat/rsonpath/crates/rsonpath-test/tests/documents/toml/jsonpath_com_example.json")?;
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
    let raw_json =
        fs::read_to_string("/home/mat/rsonpath/crates/rsonpath-test/tests/documents/toml/jsonpath_com_example.json")?;
    let input = OwnedBytes::new(&raw_json.as_bytes())?;
    let engine = RecursiveEngine::compile_query(&jsonpath_query)?;
    let result = engine.run::<_, IndexResult>(&input)?;
    assert_eq!(result.get(), vec![239usize,], "result != expected");
    Ok(())
}
#[test]
fn twitter_json_from_simdjson_github_example_with_query_descendant_for_entities_then_directly_url_with_buffered_input_and_count_result_using_main_engine(
) -> Result<(), Box<dyn Error>> {
    println ! ("on document twitter.toml running the query $..user..entities.url (descendant for entities, then directly url) with Input impl BufferedInput and result mode CountResult");
    let jsonpath_query = JsonPathQuery::parse("$..user..entities.url")?;
    let json_file = fs::File::open("/home/mat/rsonpath/crates/rsonpath-test/tests/documents/toml/twitter.json")?;
    let input = BufferedInput::new(json_file);
    let engine = MainEngine::compile_query(&jsonpath_query)?;
    let result = engine.run::<_, CountResult>(&input)?;
    assert_eq!(result.get(), 18u64, "result != expected");
    Ok(())
}
#[test]
fn twitter_json_from_simdjson_github_example_with_query_descendant_for_entities_then_directly_url_with_buffered_input_and_count_result_using_recursive_engine(
) -> Result<(), Box<dyn Error>> {
    println ! ("on document twitter.toml running the query $..user..entities.url (descendant for entities, then directly url) with Input impl BufferedInput and result mode CountResult");
    let jsonpath_query = JsonPathQuery::parse("$..user..entities.url")?;
    let json_file = fs::File::open("/home/mat/rsonpath/crates/rsonpath-test/tests/documents/toml/twitter.json")?;
    let input = BufferedInput::new(json_file);
    let engine = RecursiveEngine::compile_query(&jsonpath_query)?;
    let result = engine.run::<_, CountResult>(&input)?;
    assert_eq!(result.get(), 18u64, "result != expected");
    Ok(())
}
#[test]
fn twitter_json_from_simdjson_github_example_with_query_descendant_for_entities_then_directly_url_with_buffered_input_and_index_result_using_main_engine(
) -> Result<(), Box<dyn Error>> {
    println ! ("on document twitter.toml running the query $..user..entities.url (descendant for entities, then directly url) with Input impl BufferedInput and result mode IndexResult");
    let jsonpath_query = JsonPathQuery::parse("$..user..entities.url")?;
    let json_file = fs::File::open("/home/mat/rsonpath/crates/rsonpath-test/tests/documents/toml/twitter.json")?;
    let input = BufferedInput::new(json_file);
    let engine = MainEngine::compile_query(&jsonpath_query)?;
    let result = engine.run::<_, IndexResult>(&input)?;
    assert_eq!(
        result.get(),
        vec![
            5465usize,
            18496usize,
            23338usize,
            89785usize,
            112198usize,
            134220usize,
            201055usize,
            205281usize,
            333130usize,
            352432usize,
            357000usize,
            399785usize,
            475584usize,
            511442usize,
            516538usize,
            728252usize,
            743602usize,
            762797usize,
        ],
        "result != expected"
    );
    Ok(())
}
#[test]
fn twitter_json_from_simdjson_github_example_with_query_descendant_for_entities_then_directly_url_with_buffered_input_and_index_result_using_recursive_engine(
) -> Result<(), Box<dyn Error>> {
    println ! ("on document twitter.toml running the query $..user..entities.url (descendant for entities, then directly url) with Input impl BufferedInput and result mode IndexResult");
    let jsonpath_query = JsonPathQuery::parse("$..user..entities.url")?;
    let json_file = fs::File::open("/home/mat/rsonpath/crates/rsonpath-test/tests/documents/toml/twitter.json")?;
    let input = BufferedInput::new(json_file);
    let engine = RecursiveEngine::compile_query(&jsonpath_query)?;
    let result = engine.run::<_, IndexResult>(&input)?;
    assert_eq!(
        result.get(),
        vec![
            5465usize,
            18496usize,
            23338usize,
            89785usize,
            112198usize,
            134220usize,
            201055usize,
            205281usize,
            333130usize,
            352432usize,
            357000usize,
            399785usize,
            475584usize,
            511442usize,
            516538usize,
            728252usize,
            743602usize,
            762797usize,
        ],
        "result != expected"
    );
    Ok(())
}
#[test]
fn twitter_json_from_simdjson_github_example_with_query_descendant_for_entities_then_directly_url_with_mmap_input_and_count_result_using_main_engine(
) -> Result<(), Box<dyn Error>> {
    println ! ("on document twitter.toml running the query $..user..entities.url (descendant for entities, then directly url) with Input impl MmapInput and result mode CountResult");
    let jsonpath_query = JsonPathQuery::parse("$..user..entities.url")?;
    let json_file = fs::File::open("/home/mat/rsonpath/crates/rsonpath-test/tests/documents/toml/twitter.json")?;
    let input = unsafe { MmapInput::map_file(&json_file)? };
    let engine = MainEngine::compile_query(&jsonpath_query)?;
    let result = engine.run::<_, CountResult>(&input)?;
    assert_eq!(result.get(), 18u64, "result != expected");
    Ok(())
}
#[test]
fn twitter_json_from_simdjson_github_example_with_query_descendant_for_entities_then_directly_url_with_mmap_input_and_count_result_using_recursive_engine(
) -> Result<(), Box<dyn Error>> {
    println ! ("on document twitter.toml running the query $..user..entities.url (descendant for entities, then directly url) with Input impl MmapInput and result mode CountResult");
    let jsonpath_query = JsonPathQuery::parse("$..user..entities.url")?;
    let json_file = fs::File::open("/home/mat/rsonpath/crates/rsonpath-test/tests/documents/toml/twitter.json")?;
    let input = unsafe { MmapInput::map_file(&json_file)? };
    let engine = RecursiveEngine::compile_query(&jsonpath_query)?;
    let result = engine.run::<_, CountResult>(&input)?;
    assert_eq!(result.get(), 18u64, "result != expected");
    Ok(())
}
#[test]
fn twitter_json_from_simdjson_github_example_with_query_descendant_for_entities_then_directly_url_with_mmap_input_and_index_result_using_main_engine(
) -> Result<(), Box<dyn Error>> {
    println ! ("on document twitter.toml running the query $..user..entities.url (descendant for entities, then directly url) with Input impl MmapInput and result mode IndexResult");
    let jsonpath_query = JsonPathQuery::parse("$..user..entities.url")?;
    let json_file = fs::File::open("/home/mat/rsonpath/crates/rsonpath-test/tests/documents/toml/twitter.json")?;
    let input = unsafe { MmapInput::map_file(&json_file)? };
    let engine = MainEngine::compile_query(&jsonpath_query)?;
    let result = engine.run::<_, IndexResult>(&input)?;
    assert_eq!(
        result.get(),
        vec![
            5465usize,
            18496usize,
            23338usize,
            89785usize,
            112198usize,
            134220usize,
            201055usize,
            205281usize,
            333130usize,
            352432usize,
            357000usize,
            399785usize,
            475584usize,
            511442usize,
            516538usize,
            728252usize,
            743602usize,
            762797usize,
        ],
        "result != expected"
    );
    Ok(())
}
#[test]
fn twitter_json_from_simdjson_github_example_with_query_descendant_for_entities_then_directly_url_with_mmap_input_and_index_result_using_recursive_engine(
) -> Result<(), Box<dyn Error>> {
    println ! ("on document twitter.toml running the query $..user..entities.url (descendant for entities, then directly url) with Input impl MmapInput and result mode IndexResult");
    let jsonpath_query = JsonPathQuery::parse("$..user..entities.url")?;
    let json_file = fs::File::open("/home/mat/rsonpath/crates/rsonpath-test/tests/documents/toml/twitter.json")?;
    let input = unsafe { MmapInput::map_file(&json_file)? };
    let engine = RecursiveEngine::compile_query(&jsonpath_query)?;
    let result = engine.run::<_, IndexResult>(&input)?;
    assert_eq!(
        result.get(),
        vec![
            5465usize,
            18496usize,
            23338usize,
            89785usize,
            112198usize,
            134220usize,
            201055usize,
            205281usize,
            333130usize,
            352432usize,
            357000usize,
            399785usize,
            475584usize,
            511442usize,
            516538usize,
            728252usize,
            743602usize,
            762797usize,
        ],
        "result != expected"
    );
    Ok(())
}
#[test]
fn twitter_json_from_simdjson_github_example_with_query_descendant_for_entities_then_directly_url_with_owned_bytes_and_count_result_using_main_engine(
) -> Result<(), Box<dyn Error>> {
    println ! ("on document twitter.toml running the query $..user..entities.url (descendant for entities, then directly url) with Input impl OwnedBytes and result mode CountResult");
    let jsonpath_query = JsonPathQuery::parse("$..user..entities.url")?;
    let raw_json = fs::read_to_string("/home/mat/rsonpath/crates/rsonpath-test/tests/documents/toml/twitter.json")?;
    let input = OwnedBytes::new(&raw_json.as_bytes())?;
    let engine = MainEngine::compile_query(&jsonpath_query)?;
    let result = engine.run::<_, CountResult>(&input)?;
    assert_eq!(result.get(), 18u64, "result != expected");
    Ok(())
}
#[test]
fn twitter_json_from_simdjson_github_example_with_query_descendant_for_entities_then_directly_url_with_owned_bytes_and_count_result_using_recursive_engine(
) -> Result<(), Box<dyn Error>> {
    println ! ("on document twitter.toml running the query $..user..entities.url (descendant for entities, then directly url) with Input impl OwnedBytes and result mode CountResult");
    let jsonpath_query = JsonPathQuery::parse("$..user..entities.url")?;
    let raw_json = fs::read_to_string("/home/mat/rsonpath/crates/rsonpath-test/tests/documents/toml/twitter.json")?;
    let input = OwnedBytes::new(&raw_json.as_bytes())?;
    let engine = RecursiveEngine::compile_query(&jsonpath_query)?;
    let result = engine.run::<_, CountResult>(&input)?;
    assert_eq!(result.get(), 18u64, "result != expected");
    Ok(())
}
#[test]
fn twitter_json_from_simdjson_github_example_with_query_descendant_for_entities_then_directly_url_with_owned_bytes_and_index_result_using_main_engine(
) -> Result<(), Box<dyn Error>> {
    println ! ("on document twitter.toml running the query $..user..entities.url (descendant for entities, then directly url) with Input impl OwnedBytes and result mode IndexResult");
    let jsonpath_query = JsonPathQuery::parse("$..user..entities.url")?;
    let raw_json = fs::read_to_string("/home/mat/rsonpath/crates/rsonpath-test/tests/documents/toml/twitter.json")?;
    let input = OwnedBytes::new(&raw_json.as_bytes())?;
    let engine = MainEngine::compile_query(&jsonpath_query)?;
    let result = engine.run::<_, IndexResult>(&input)?;
    assert_eq!(
        result.get(),
        vec![
            5465usize,
            18496usize,
            23338usize,
            89785usize,
            112198usize,
            134220usize,
            201055usize,
            205281usize,
            333130usize,
            352432usize,
            357000usize,
            399785usize,
            475584usize,
            511442usize,
            516538usize,
            728252usize,
            743602usize,
            762797usize,
        ],
        "result != expected"
    );
    Ok(())
}
#[test]
fn twitter_json_from_simdjson_github_example_with_query_descendant_for_entities_then_directly_url_with_owned_bytes_and_index_result_using_recursive_engine(
) -> Result<(), Box<dyn Error>> {
    println ! ("on document twitter.toml running the query $..user..entities.url (descendant for entities, then directly url) with Input impl OwnedBytes and result mode IndexResult");
    let jsonpath_query = JsonPathQuery::parse("$..user..entities.url")?;
    let raw_json = fs::read_to_string("/home/mat/rsonpath/crates/rsonpath-test/tests/documents/toml/twitter.json")?;
    let input = OwnedBytes::new(&raw_json.as_bytes())?;
    let engine = RecursiveEngine::compile_query(&jsonpath_query)?;
    let result = engine.run::<_, IndexResult>(&input)?;
    assert_eq!(
        result.get(),
        vec![
            5465usize,
            18496usize,
            23338usize,
            89785usize,
            112198usize,
            134220usize,
            201055usize,
            205281usize,
            333130usize,
            352432usize,
            357000usize,
            399785usize,
            475584usize,
            511442usize,
            516538usize,
            728252usize,
            743602usize,
            762797usize,
        ],
        "result != expected"
    );
    Ok(())
}
#[test]
fn twitter_json_from_simdjson_github_example_with_query_descendant_search_for_count_with_buffered_input_and_count_result_using_main_engine(
) -> Result<(), Box<dyn Error>> {
    println ! ("on document twitter.toml running the query $..count (descendant search for count) with Input impl BufferedInput and result mode CountResult");
    let jsonpath_query = JsonPathQuery::parse("$..count")?;
    let json_file = fs::File::open("/home/mat/rsonpath/crates/rsonpath-test/tests/documents/toml/twitter.json")?;
    let input = BufferedInput::new(json_file);
    let engine = MainEngine::compile_query(&jsonpath_query)?;
    let result = engine.run::<_, CountResult>(&input)?;
    assert_eq!(result.get(), 1u64, "result != expected");
    Ok(())
}
#[test]
fn twitter_json_from_simdjson_github_example_with_query_descendant_search_for_count_with_buffered_input_and_count_result_using_recursive_engine(
) -> Result<(), Box<dyn Error>> {
    println ! ("on document twitter.toml running the query $..count (descendant search for count) with Input impl BufferedInput and result mode CountResult");
    let jsonpath_query = JsonPathQuery::parse("$..count")?;
    let json_file = fs::File::open("/home/mat/rsonpath/crates/rsonpath-test/tests/documents/toml/twitter.json")?;
    let input = BufferedInput::new(json_file);
    let engine = RecursiveEngine::compile_query(&jsonpath_query)?;
    let result = engine.run::<_, CountResult>(&input)?;
    assert_eq!(result.get(), 1u64, "result != expected");
    Ok(())
}
#[test]
fn twitter_json_from_simdjson_github_example_with_query_descendant_search_for_count_with_buffered_input_and_index_result_using_main_engine(
) -> Result<(), Box<dyn Error>> {
    println ! ("on document twitter.toml running the query $..count (descendant search for count) with Input impl BufferedInput and result mode IndexResult");
    let jsonpath_query = JsonPathQuery::parse("$..count")?;
    let json_file = fs::File::open("/home/mat/rsonpath/crates/rsonpath-test/tests/documents/toml/twitter.json")?;
    let input = BufferedInput::new(json_file);
    let engine = MainEngine::compile_query(&jsonpath_query)?;
    let result = engine.run::<_, IndexResult>(&input)?;
    assert_eq!(result.get(), vec![767233usize,], "result != expected");
    Ok(())
}
#[test]
fn twitter_json_from_simdjson_github_example_with_query_descendant_search_for_count_with_buffered_input_and_index_result_using_recursive_engine(
) -> Result<(), Box<dyn Error>> {
    println ! ("on document twitter.toml running the query $..count (descendant search for count) with Input impl BufferedInput and result mode IndexResult");
    let jsonpath_query = JsonPathQuery::parse("$..count")?;
    let json_file = fs::File::open("/home/mat/rsonpath/crates/rsonpath-test/tests/documents/toml/twitter.json")?;
    let input = BufferedInput::new(json_file);
    let engine = RecursiveEngine::compile_query(&jsonpath_query)?;
    let result = engine.run::<_, IndexResult>(&input)?;
    assert_eq!(result.get(), vec![767233usize,], "result != expected");
    Ok(())
}
#[test]
fn twitter_json_from_simdjson_github_example_with_query_descendant_search_for_count_with_mmap_input_and_count_result_using_main_engine(
) -> Result<(), Box<dyn Error>> {
    println ! ("on document twitter.toml running the query $..count (descendant search for count) with Input impl MmapInput and result mode CountResult");
    let jsonpath_query = JsonPathQuery::parse("$..count")?;
    let json_file = fs::File::open("/home/mat/rsonpath/crates/rsonpath-test/tests/documents/toml/twitter.json")?;
    let input = unsafe { MmapInput::map_file(&json_file)? };
    let engine = MainEngine::compile_query(&jsonpath_query)?;
    let result = engine.run::<_, CountResult>(&input)?;
    assert_eq!(result.get(), 1u64, "result != expected");
    Ok(())
}
#[test]
fn twitter_json_from_simdjson_github_example_with_query_descendant_search_for_count_with_mmap_input_and_count_result_using_recursive_engine(
) -> Result<(), Box<dyn Error>> {
    println ! ("on document twitter.toml running the query $..count (descendant search for count) with Input impl MmapInput and result mode CountResult");
    let jsonpath_query = JsonPathQuery::parse("$..count")?;
    let json_file = fs::File::open("/home/mat/rsonpath/crates/rsonpath-test/tests/documents/toml/twitter.json")?;
    let input = unsafe { MmapInput::map_file(&json_file)? };
    let engine = RecursiveEngine::compile_query(&jsonpath_query)?;
    let result = engine.run::<_, CountResult>(&input)?;
    assert_eq!(result.get(), 1u64, "result != expected");
    Ok(())
}
#[test]
fn twitter_json_from_simdjson_github_example_with_query_descendant_search_for_count_with_mmap_input_and_index_result_using_main_engine(
) -> Result<(), Box<dyn Error>> {
    println ! ("on document twitter.toml running the query $..count (descendant search for count) with Input impl MmapInput and result mode IndexResult");
    let jsonpath_query = JsonPathQuery::parse("$..count")?;
    let json_file = fs::File::open("/home/mat/rsonpath/crates/rsonpath-test/tests/documents/toml/twitter.json")?;
    let input = unsafe { MmapInput::map_file(&json_file)? };
    let engine = MainEngine::compile_query(&jsonpath_query)?;
    let result = engine.run::<_, IndexResult>(&input)?;
    assert_eq!(result.get(), vec![767233usize,], "result != expected");
    Ok(())
}
#[test]
fn twitter_json_from_simdjson_github_example_with_query_descendant_search_for_count_with_mmap_input_and_index_result_using_recursive_engine(
) -> Result<(), Box<dyn Error>> {
    println ! ("on document twitter.toml running the query $..count (descendant search for count) with Input impl MmapInput and result mode IndexResult");
    let jsonpath_query = JsonPathQuery::parse("$..count")?;
    let json_file = fs::File::open("/home/mat/rsonpath/crates/rsonpath-test/tests/documents/toml/twitter.json")?;
    let input = unsafe { MmapInput::map_file(&json_file)? };
    let engine = RecursiveEngine::compile_query(&jsonpath_query)?;
    let result = engine.run::<_, IndexResult>(&input)?;
    assert_eq!(result.get(), vec![767233usize,], "result != expected");
    Ok(())
}
#[test]
fn twitter_json_from_simdjson_github_example_with_query_descendant_search_for_count_with_owned_bytes_and_count_result_using_main_engine(
) -> Result<(), Box<dyn Error>> {
    println ! ("on document twitter.toml running the query $..count (descendant search for count) with Input impl OwnedBytes and result mode CountResult");
    let jsonpath_query = JsonPathQuery::parse("$..count")?;
    let raw_json = fs::read_to_string("/home/mat/rsonpath/crates/rsonpath-test/tests/documents/toml/twitter.json")?;
    let input = OwnedBytes::new(&raw_json.as_bytes())?;
    let engine = MainEngine::compile_query(&jsonpath_query)?;
    let result = engine.run::<_, CountResult>(&input)?;
    assert_eq!(result.get(), 1u64, "result != expected");
    Ok(())
}
#[test]
fn twitter_json_from_simdjson_github_example_with_query_descendant_search_for_count_with_owned_bytes_and_count_result_using_recursive_engine(
) -> Result<(), Box<dyn Error>> {
    println ! ("on document twitter.toml running the query $..count (descendant search for count) with Input impl OwnedBytes and result mode CountResult");
    let jsonpath_query = JsonPathQuery::parse("$..count")?;
    let raw_json = fs::read_to_string("/home/mat/rsonpath/crates/rsonpath-test/tests/documents/toml/twitter.json")?;
    let input = OwnedBytes::new(&raw_json.as_bytes())?;
    let engine = RecursiveEngine::compile_query(&jsonpath_query)?;
    let result = engine.run::<_, CountResult>(&input)?;
    assert_eq!(result.get(), 1u64, "result != expected");
    Ok(())
}
#[test]
fn twitter_json_from_simdjson_github_example_with_query_descendant_search_for_count_with_owned_bytes_and_index_result_using_main_engine(
) -> Result<(), Box<dyn Error>> {
    println ! ("on document twitter.toml running the query $..count (descendant search for count) with Input impl OwnedBytes and result mode IndexResult");
    let jsonpath_query = JsonPathQuery::parse("$..count")?;
    let raw_json = fs::read_to_string("/home/mat/rsonpath/crates/rsonpath-test/tests/documents/toml/twitter.json")?;
    let input = OwnedBytes::new(&raw_json.as_bytes())?;
    let engine = MainEngine::compile_query(&jsonpath_query)?;
    let result = engine.run::<_, IndexResult>(&input)?;
    assert_eq!(result.get(), vec![767233usize,], "result != expected");
    Ok(())
}
#[test]
fn twitter_json_from_simdjson_github_example_with_query_descendant_search_for_count_with_owned_bytes_and_index_result_using_recursive_engine(
) -> Result<(), Box<dyn Error>> {
    println ! ("on document twitter.toml running the query $..count (descendant search for count) with Input impl OwnedBytes and result mode IndexResult");
    let jsonpath_query = JsonPathQuery::parse("$..count")?;
    let raw_json = fs::read_to_string("/home/mat/rsonpath/crates/rsonpath-test/tests/documents/toml/twitter.json")?;
    let input = OwnedBytes::new(&raw_json.as_bytes())?;
    let engine = RecursiveEngine::compile_query(&jsonpath_query)?;
    let result = engine.run::<_, IndexResult>(&input)?;
    assert_eq!(result.get(), vec![767233usize,], "result != expected");
    Ok(())
}
#[test]
fn twitter_json_from_simdjson_github_example_with_query_descendant_search_for_search_metadata_count_with_buffered_input_and_count_result_using_main_engine(
) -> Result<(), Box<dyn Error>> {
    println ! ("on document twitter.toml running the query $..search_metadata..count (descendant search for search_metadata count) with Input impl BufferedInput and result mode CountResult");
    let jsonpath_query = JsonPathQuery::parse("$..search_metadata..count")?;
    let json_file = fs::File::open("/home/mat/rsonpath/crates/rsonpath-test/tests/documents/toml/twitter.json")?;
    let input = BufferedInput::new(json_file);
    let engine = MainEngine::compile_query(&jsonpath_query)?;
    let result = engine.run::<_, CountResult>(&input)?;
    assert_eq!(result.get(), 1u64, "result != expected");
    Ok(())
}
#[test]
fn twitter_json_from_simdjson_github_example_with_query_descendant_search_for_search_metadata_count_with_buffered_input_and_count_result_using_recursive_engine(
) -> Result<(), Box<dyn Error>> {
    println ! ("on document twitter.toml running the query $..search_metadata..count (descendant search for search_metadata count) with Input impl BufferedInput and result mode CountResult");
    let jsonpath_query = JsonPathQuery::parse("$..search_metadata..count")?;
    let json_file = fs::File::open("/home/mat/rsonpath/crates/rsonpath-test/tests/documents/toml/twitter.json")?;
    let input = BufferedInput::new(json_file);
    let engine = RecursiveEngine::compile_query(&jsonpath_query)?;
    let result = engine.run::<_, CountResult>(&input)?;
    assert_eq!(result.get(), 1u64, "result != expected");
    Ok(())
}
#[test]
fn twitter_json_from_simdjson_github_example_with_query_descendant_search_for_search_metadata_count_with_buffered_input_and_index_result_using_main_engine(
) -> Result<(), Box<dyn Error>> {
    println ! ("on document twitter.toml running the query $..search_metadata..count (descendant search for search_metadata count) with Input impl BufferedInput and result mode IndexResult");
    let jsonpath_query = JsonPathQuery::parse("$..search_metadata..count")?;
    let json_file = fs::File::open("/home/mat/rsonpath/crates/rsonpath-test/tests/documents/toml/twitter.json")?;
    let input = BufferedInput::new(json_file);
    let engine = MainEngine::compile_query(&jsonpath_query)?;
    let result = engine.run::<_, IndexResult>(&input)?;
    assert_eq!(result.get(), vec![767233usize,], "result != expected");
    Ok(())
}
#[test]
fn twitter_json_from_simdjson_github_example_with_query_descendant_search_for_search_metadata_count_with_buffered_input_and_index_result_using_recursive_engine(
) -> Result<(), Box<dyn Error>> {
    println ! ("on document twitter.toml running the query $..search_metadata..count (descendant search for search_metadata count) with Input impl BufferedInput and result mode IndexResult");
    let jsonpath_query = JsonPathQuery::parse("$..search_metadata..count")?;
    let json_file = fs::File::open("/home/mat/rsonpath/crates/rsonpath-test/tests/documents/toml/twitter.json")?;
    let input = BufferedInput::new(json_file);
    let engine = RecursiveEngine::compile_query(&jsonpath_query)?;
    let result = engine.run::<_, IndexResult>(&input)?;
    assert_eq!(result.get(), vec![767233usize,], "result != expected");
    Ok(())
}
#[test]
fn twitter_json_from_simdjson_github_example_with_query_descendant_search_for_search_metadata_count_with_mmap_input_and_count_result_using_main_engine(
) -> Result<(), Box<dyn Error>> {
    println ! ("on document twitter.toml running the query $..search_metadata..count (descendant search for search_metadata count) with Input impl MmapInput and result mode CountResult");
    let jsonpath_query = JsonPathQuery::parse("$..search_metadata..count")?;
    let json_file = fs::File::open("/home/mat/rsonpath/crates/rsonpath-test/tests/documents/toml/twitter.json")?;
    let input = unsafe { MmapInput::map_file(&json_file)? };
    let engine = MainEngine::compile_query(&jsonpath_query)?;
    let result = engine.run::<_, CountResult>(&input)?;
    assert_eq!(result.get(), 1u64, "result != expected");
    Ok(())
}
#[test]
fn twitter_json_from_simdjson_github_example_with_query_descendant_search_for_search_metadata_count_with_mmap_input_and_count_result_using_recursive_engine(
) -> Result<(), Box<dyn Error>> {
    println ! ("on document twitter.toml running the query $..search_metadata..count (descendant search for search_metadata count) with Input impl MmapInput and result mode CountResult");
    let jsonpath_query = JsonPathQuery::parse("$..search_metadata..count")?;
    let json_file = fs::File::open("/home/mat/rsonpath/crates/rsonpath-test/tests/documents/toml/twitter.json")?;
    let input = unsafe { MmapInput::map_file(&json_file)? };
    let engine = RecursiveEngine::compile_query(&jsonpath_query)?;
    let result = engine.run::<_, CountResult>(&input)?;
    assert_eq!(result.get(), 1u64, "result != expected");
    Ok(())
}
#[test]
fn twitter_json_from_simdjson_github_example_with_query_descendant_search_for_search_metadata_count_with_mmap_input_and_index_result_using_main_engine(
) -> Result<(), Box<dyn Error>> {
    println ! ("on document twitter.toml running the query $..search_metadata..count (descendant search for search_metadata count) with Input impl MmapInput and result mode IndexResult");
    let jsonpath_query = JsonPathQuery::parse("$..search_metadata..count")?;
    let json_file = fs::File::open("/home/mat/rsonpath/crates/rsonpath-test/tests/documents/toml/twitter.json")?;
    let input = unsafe { MmapInput::map_file(&json_file)? };
    let engine = MainEngine::compile_query(&jsonpath_query)?;
    let result = engine.run::<_, IndexResult>(&input)?;
    assert_eq!(result.get(), vec![767233usize,], "result != expected");
    Ok(())
}
#[test]
fn twitter_json_from_simdjson_github_example_with_query_descendant_search_for_search_metadata_count_with_mmap_input_and_index_result_using_recursive_engine(
) -> Result<(), Box<dyn Error>> {
    println ! ("on document twitter.toml running the query $..search_metadata..count (descendant search for search_metadata count) with Input impl MmapInput and result mode IndexResult");
    let jsonpath_query = JsonPathQuery::parse("$..search_metadata..count")?;
    let json_file = fs::File::open("/home/mat/rsonpath/crates/rsonpath-test/tests/documents/toml/twitter.json")?;
    let input = unsafe { MmapInput::map_file(&json_file)? };
    let engine = RecursiveEngine::compile_query(&jsonpath_query)?;
    let result = engine.run::<_, IndexResult>(&input)?;
    assert_eq!(result.get(), vec![767233usize,], "result != expected");
    Ok(())
}
#[test]
fn twitter_json_from_simdjson_github_example_with_query_descendant_search_for_search_metadata_count_with_owned_bytes_and_count_result_using_main_engine(
) -> Result<(), Box<dyn Error>> {
    println ! ("on document twitter.toml running the query $..search_metadata..count (descendant search for search_metadata count) with Input impl OwnedBytes and result mode CountResult");
    let jsonpath_query = JsonPathQuery::parse("$..search_metadata..count")?;
    let raw_json = fs::read_to_string("/home/mat/rsonpath/crates/rsonpath-test/tests/documents/toml/twitter.json")?;
    let input = OwnedBytes::new(&raw_json.as_bytes())?;
    let engine = MainEngine::compile_query(&jsonpath_query)?;
    let result = engine.run::<_, CountResult>(&input)?;
    assert_eq!(result.get(), 1u64, "result != expected");
    Ok(())
}
#[test]
fn twitter_json_from_simdjson_github_example_with_query_descendant_search_for_search_metadata_count_with_owned_bytes_and_count_result_using_recursive_engine(
) -> Result<(), Box<dyn Error>> {
    println ! ("on document twitter.toml running the query $..search_metadata..count (descendant search for search_metadata count) with Input impl OwnedBytes and result mode CountResult");
    let jsonpath_query = JsonPathQuery::parse("$..search_metadata..count")?;
    let raw_json = fs::read_to_string("/home/mat/rsonpath/crates/rsonpath-test/tests/documents/toml/twitter.json")?;
    let input = OwnedBytes::new(&raw_json.as_bytes())?;
    let engine = RecursiveEngine::compile_query(&jsonpath_query)?;
    let result = engine.run::<_, CountResult>(&input)?;
    assert_eq!(result.get(), 1u64, "result != expected");
    Ok(())
}
#[test]
fn twitter_json_from_simdjson_github_example_with_query_descendant_search_for_search_metadata_count_with_owned_bytes_and_index_result_using_main_engine(
) -> Result<(), Box<dyn Error>> {
    println ! ("on document twitter.toml running the query $..search_metadata..count (descendant search for search_metadata count) with Input impl OwnedBytes and result mode IndexResult");
    let jsonpath_query = JsonPathQuery::parse("$..search_metadata..count")?;
    let raw_json = fs::read_to_string("/home/mat/rsonpath/crates/rsonpath-test/tests/documents/toml/twitter.json")?;
    let input = OwnedBytes::new(&raw_json.as_bytes())?;
    let engine = MainEngine::compile_query(&jsonpath_query)?;
    let result = engine.run::<_, IndexResult>(&input)?;
    assert_eq!(result.get(), vec![767233usize,], "result != expected");
    Ok(())
}
#[test]
fn twitter_json_from_simdjson_github_example_with_query_descendant_search_for_search_metadata_count_with_owned_bytes_and_index_result_using_recursive_engine(
) -> Result<(), Box<dyn Error>> {
    println ! ("on document twitter.toml running the query $..search_metadata..count (descendant search for search_metadata count) with Input impl OwnedBytes and result mode IndexResult");
    let jsonpath_query = JsonPathQuery::parse("$..search_metadata..count")?;
    let raw_json = fs::read_to_string("/home/mat/rsonpath/crates/rsonpath-test/tests/documents/toml/twitter.json")?;
    let input = OwnedBytes::new(&raw_json.as_bytes())?;
    let engine = RecursiveEngine::compile_query(&jsonpath_query)?;
    let result = engine.run::<_, IndexResult>(&input)?;
    assert_eq!(result.get(), vec![767233usize,], "result != expected");
    Ok(())
}
#[test]
fn twitter_json_from_simdjson_github_example_with_query_descendant_search_for_search_metadata_then_direct_count_with_buffered_input_and_count_result_using_main_engine(
) -> Result<(), Box<dyn Error>> {
    println ! ("on document twitter.toml running the query $..search_metadata.count (descendant search for search_metadata then direct count) with Input impl BufferedInput and result mode CountResult");
    let jsonpath_query = JsonPathQuery::parse("$..search_metadata.count")?;
    let json_file = fs::File::open("/home/mat/rsonpath/crates/rsonpath-test/tests/documents/toml/twitter.json")?;
    let input = BufferedInput::new(json_file);
    let engine = MainEngine::compile_query(&jsonpath_query)?;
    let result = engine.run::<_, CountResult>(&input)?;
    assert_eq!(result.get(), 1u64, "result != expected");
    Ok(())
}
#[test]
fn twitter_json_from_simdjson_github_example_with_query_descendant_search_for_search_metadata_then_direct_count_with_buffered_input_and_count_result_using_recursive_engine(
) -> Result<(), Box<dyn Error>> {
    println ! ("on document twitter.toml running the query $..search_metadata.count (descendant search for search_metadata then direct count) with Input impl BufferedInput and result mode CountResult");
    let jsonpath_query = JsonPathQuery::parse("$..search_metadata.count")?;
    let json_file = fs::File::open("/home/mat/rsonpath/crates/rsonpath-test/tests/documents/toml/twitter.json")?;
    let input = BufferedInput::new(json_file);
    let engine = RecursiveEngine::compile_query(&jsonpath_query)?;
    let result = engine.run::<_, CountResult>(&input)?;
    assert_eq!(result.get(), 1u64, "result != expected");
    Ok(())
}
#[test]
fn twitter_json_from_simdjson_github_example_with_query_descendant_search_for_search_metadata_then_direct_count_with_buffered_input_and_index_result_using_main_engine(
) -> Result<(), Box<dyn Error>> {
    println ! ("on document twitter.toml running the query $..search_metadata.count (descendant search for search_metadata then direct count) with Input impl BufferedInput and result mode IndexResult");
    let jsonpath_query = JsonPathQuery::parse("$..search_metadata.count")?;
    let json_file = fs::File::open("/home/mat/rsonpath/crates/rsonpath-test/tests/documents/toml/twitter.json")?;
    let input = BufferedInput::new(json_file);
    let engine = MainEngine::compile_query(&jsonpath_query)?;
    let result = engine.run::<_, IndexResult>(&input)?;
    assert_eq!(result.get(), vec![767233usize,], "result != expected");
    Ok(())
}
#[test]
fn twitter_json_from_simdjson_github_example_with_query_descendant_search_for_search_metadata_then_direct_count_with_buffered_input_and_index_result_using_recursive_engine(
) -> Result<(), Box<dyn Error>> {
    println ! ("on document twitter.toml running the query $..search_metadata.count (descendant search for search_metadata then direct count) with Input impl BufferedInput and result mode IndexResult");
    let jsonpath_query = JsonPathQuery::parse("$..search_metadata.count")?;
    let json_file = fs::File::open("/home/mat/rsonpath/crates/rsonpath-test/tests/documents/toml/twitter.json")?;
    let input = BufferedInput::new(json_file);
    let engine = RecursiveEngine::compile_query(&jsonpath_query)?;
    let result = engine.run::<_, IndexResult>(&input)?;
    assert_eq!(result.get(), vec![767233usize,], "result != expected");
    Ok(())
}
#[test]
fn twitter_json_from_simdjson_github_example_with_query_descendant_search_for_search_metadata_then_direct_count_with_mmap_input_and_count_result_using_main_engine(
) -> Result<(), Box<dyn Error>> {
    println ! ("on document twitter.toml running the query $..search_metadata.count (descendant search for search_metadata then direct count) with Input impl MmapInput and result mode CountResult");
    let jsonpath_query = JsonPathQuery::parse("$..search_metadata.count")?;
    let json_file = fs::File::open("/home/mat/rsonpath/crates/rsonpath-test/tests/documents/toml/twitter.json")?;
    let input = unsafe { MmapInput::map_file(&json_file)? };
    let engine = MainEngine::compile_query(&jsonpath_query)?;
    let result = engine.run::<_, CountResult>(&input)?;
    assert_eq!(result.get(), 1u64, "result != expected");
    Ok(())
}
#[test]
fn twitter_json_from_simdjson_github_example_with_query_descendant_search_for_search_metadata_then_direct_count_with_mmap_input_and_count_result_using_recursive_engine(
) -> Result<(), Box<dyn Error>> {
    println ! ("on document twitter.toml running the query $..search_metadata.count (descendant search for search_metadata then direct count) with Input impl MmapInput and result mode CountResult");
    let jsonpath_query = JsonPathQuery::parse("$..search_metadata.count")?;
    let json_file = fs::File::open("/home/mat/rsonpath/crates/rsonpath-test/tests/documents/toml/twitter.json")?;
    let input = unsafe { MmapInput::map_file(&json_file)? };
    let engine = RecursiveEngine::compile_query(&jsonpath_query)?;
    let result = engine.run::<_, CountResult>(&input)?;
    assert_eq!(result.get(), 1u64, "result != expected");
    Ok(())
}
#[test]
fn twitter_json_from_simdjson_github_example_with_query_descendant_search_for_search_metadata_then_direct_count_with_mmap_input_and_index_result_using_main_engine(
) -> Result<(), Box<dyn Error>> {
    println ! ("on document twitter.toml running the query $..search_metadata.count (descendant search for search_metadata then direct count) with Input impl MmapInput and result mode IndexResult");
    let jsonpath_query = JsonPathQuery::parse("$..search_metadata.count")?;
    let json_file = fs::File::open("/home/mat/rsonpath/crates/rsonpath-test/tests/documents/toml/twitter.json")?;
    let input = unsafe { MmapInput::map_file(&json_file)? };
    let engine = MainEngine::compile_query(&jsonpath_query)?;
    let result = engine.run::<_, IndexResult>(&input)?;
    assert_eq!(result.get(), vec![767233usize,], "result != expected");
    Ok(())
}
#[test]
fn twitter_json_from_simdjson_github_example_with_query_descendant_search_for_search_metadata_then_direct_count_with_mmap_input_and_index_result_using_recursive_engine(
) -> Result<(), Box<dyn Error>> {
    println ! ("on document twitter.toml running the query $..search_metadata.count (descendant search for search_metadata then direct count) with Input impl MmapInput and result mode IndexResult");
    let jsonpath_query = JsonPathQuery::parse("$..search_metadata.count")?;
    let json_file = fs::File::open("/home/mat/rsonpath/crates/rsonpath-test/tests/documents/toml/twitter.json")?;
    let input = unsafe { MmapInput::map_file(&json_file)? };
    let engine = RecursiveEngine::compile_query(&jsonpath_query)?;
    let result = engine.run::<_, IndexResult>(&input)?;
    assert_eq!(result.get(), vec![767233usize,], "result != expected");
    Ok(())
}
#[test]
fn twitter_json_from_simdjson_github_example_with_query_descendant_search_for_search_metadata_then_direct_count_with_owned_bytes_and_count_result_using_main_engine(
) -> Result<(), Box<dyn Error>> {
    println ! ("on document twitter.toml running the query $..search_metadata.count (descendant search for search_metadata then direct count) with Input impl OwnedBytes and result mode CountResult");
    let jsonpath_query = JsonPathQuery::parse("$..search_metadata.count")?;
    let raw_json = fs::read_to_string("/home/mat/rsonpath/crates/rsonpath-test/tests/documents/toml/twitter.json")?;
    let input = OwnedBytes::new(&raw_json.as_bytes())?;
    let engine = MainEngine::compile_query(&jsonpath_query)?;
    let result = engine.run::<_, CountResult>(&input)?;
    assert_eq!(result.get(), 1u64, "result != expected");
    Ok(())
}
#[test]
fn twitter_json_from_simdjson_github_example_with_query_descendant_search_for_search_metadata_then_direct_count_with_owned_bytes_and_count_result_using_recursive_engine(
) -> Result<(), Box<dyn Error>> {
    println ! ("on document twitter.toml running the query $..search_metadata.count (descendant search for search_metadata then direct count) with Input impl OwnedBytes and result mode CountResult");
    let jsonpath_query = JsonPathQuery::parse("$..search_metadata.count")?;
    let raw_json = fs::read_to_string("/home/mat/rsonpath/crates/rsonpath-test/tests/documents/toml/twitter.json")?;
    let input = OwnedBytes::new(&raw_json.as_bytes())?;
    let engine = RecursiveEngine::compile_query(&jsonpath_query)?;
    let result = engine.run::<_, CountResult>(&input)?;
    assert_eq!(result.get(), 1u64, "result != expected");
    Ok(())
}
#[test]
fn twitter_json_from_simdjson_github_example_with_query_descendant_search_for_search_metadata_then_direct_count_with_owned_bytes_and_index_result_using_main_engine(
) -> Result<(), Box<dyn Error>> {
    println ! ("on document twitter.toml running the query $..search_metadata.count (descendant search for search_metadata then direct count) with Input impl OwnedBytes and result mode IndexResult");
    let jsonpath_query = JsonPathQuery::parse("$..search_metadata.count")?;
    let raw_json = fs::read_to_string("/home/mat/rsonpath/crates/rsonpath-test/tests/documents/toml/twitter.json")?;
    let input = OwnedBytes::new(&raw_json.as_bytes())?;
    let engine = MainEngine::compile_query(&jsonpath_query)?;
    let result = engine.run::<_, IndexResult>(&input)?;
    assert_eq!(result.get(), vec![767233usize,], "result != expected");
    Ok(())
}
#[test]
fn twitter_json_from_simdjson_github_example_with_query_descendant_search_for_search_metadata_then_direct_count_with_owned_bytes_and_index_result_using_recursive_engine(
) -> Result<(), Box<dyn Error>> {
    println ! ("on document twitter.toml running the query $..search_metadata.count (descendant search for search_metadata then direct count) with Input impl OwnedBytes and result mode IndexResult");
    let jsonpath_query = JsonPathQuery::parse("$..search_metadata.count")?;
    let raw_json = fs::read_to_string("/home/mat/rsonpath/crates/rsonpath-test/tests/documents/toml/twitter.json")?;
    let input = OwnedBytes::new(&raw_json.as_bytes())?;
    let engine = RecursiveEngine::compile_query(&jsonpath_query)?;
    let result = engine.run::<_, IndexResult>(&input)?;
    assert_eq!(result.get(), vec![767233usize,], "result != expected");
    Ok(())
}
#[test]
fn twitter_json_from_simdjson_github_example_with_query_descendant_user_entities_url_with_buffered_input_and_count_result_using_main_engine(
) -> Result<(), Box<dyn Error>> {
    println ! ("on document twitter.toml running the query $..user..entities..url (descendant user entities url) with Input impl BufferedInput and result mode CountResult");
    let jsonpath_query = JsonPathQuery::parse("$..user..entities..url")?;
    let json_file = fs::File::open("/home/mat/rsonpath/crates/rsonpath-test/tests/documents/toml/twitter.json")?;
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
    let json_file = fs::File::open("/home/mat/rsonpath/crates/rsonpath-test/tests/documents/toml/twitter.json")?;
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
    let json_file = fs::File::open("/home/mat/rsonpath/crates/rsonpath-test/tests/documents/toml/twitter.json")?;
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
    let json_file = fs::File::open("/home/mat/rsonpath/crates/rsonpath-test/tests/documents/toml/twitter.json")?;
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
    let json_file = fs::File::open("/home/mat/rsonpath/crates/rsonpath-test/tests/documents/toml/twitter.json")?;
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
    let json_file = fs::File::open("/home/mat/rsonpath/crates/rsonpath-test/tests/documents/toml/twitter.json")?;
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
    let json_file = fs::File::open("/home/mat/rsonpath/crates/rsonpath-test/tests/documents/toml/twitter.json")?;
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
    let json_file = fs::File::open("/home/mat/rsonpath/crates/rsonpath-test/tests/documents/toml/twitter.json")?;
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
    let raw_json = fs::read_to_string("/home/mat/rsonpath/crates/rsonpath-test/tests/documents/toml/twitter.json")?;
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
    let raw_json = fs::read_to_string("/home/mat/rsonpath/crates/rsonpath-test/tests/documents/toml/twitter.json")?;
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
    let raw_json = fs::read_to_string("/home/mat/rsonpath/crates/rsonpath-test/tests/documents/toml/twitter.json")?;
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
    let raw_json = fs::read_to_string("/home/mat/rsonpath/crates/rsonpath-test/tests/documents/toml/twitter.json")?;
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
fn twitter_json_from_simdjson_github_example_with_query_direct_search_for_search_metadata_count_with_buffered_input_and_count_result_using_main_engine(
) -> Result<(), Box<dyn Error>> {
    println ! ("on document twitter.toml running the query $.search_metadata.count (direct search for search_metadata count) with Input impl BufferedInput and result mode CountResult");
    let jsonpath_query = JsonPathQuery::parse("$.search_metadata.count")?;
    let json_file = fs::File::open("/home/mat/rsonpath/crates/rsonpath-test/tests/documents/toml/twitter.json")?;
    let input = BufferedInput::new(json_file);
    let engine = MainEngine::compile_query(&jsonpath_query)?;
    let result = engine.run::<_, CountResult>(&input)?;
    assert_eq!(result.get(), 1u64, "result != expected");
    Ok(())
}
#[test]
fn twitter_json_from_simdjson_github_example_with_query_direct_search_for_search_metadata_count_with_buffered_input_and_count_result_using_recursive_engine(
) -> Result<(), Box<dyn Error>> {
    println ! ("on document twitter.toml running the query $.search_metadata.count (direct search for search_metadata count) with Input impl BufferedInput and result mode CountResult");
    let jsonpath_query = JsonPathQuery::parse("$.search_metadata.count")?;
    let json_file = fs::File::open("/home/mat/rsonpath/crates/rsonpath-test/tests/documents/toml/twitter.json")?;
    let input = BufferedInput::new(json_file);
    let engine = RecursiveEngine::compile_query(&jsonpath_query)?;
    let result = engine.run::<_, CountResult>(&input)?;
    assert_eq!(result.get(), 1u64, "result != expected");
    Ok(())
}
#[test]
fn twitter_json_from_simdjson_github_example_with_query_direct_search_for_search_metadata_count_with_buffered_input_and_index_result_using_main_engine(
) -> Result<(), Box<dyn Error>> {
    println ! ("on document twitter.toml running the query $.search_metadata.count (direct search for search_metadata count) with Input impl BufferedInput and result mode IndexResult");
    let jsonpath_query = JsonPathQuery::parse("$.search_metadata.count")?;
    let json_file = fs::File::open("/home/mat/rsonpath/crates/rsonpath-test/tests/documents/toml/twitter.json")?;
    let input = BufferedInput::new(json_file);
    let engine = MainEngine::compile_query(&jsonpath_query)?;
    let result = engine.run::<_, IndexResult>(&input)?;
    assert_eq!(result.get(), vec![767233usize,], "result != expected");
    Ok(())
}
#[test]
fn twitter_json_from_simdjson_github_example_with_query_direct_search_for_search_metadata_count_with_buffered_input_and_index_result_using_recursive_engine(
) -> Result<(), Box<dyn Error>> {
    println ! ("on document twitter.toml running the query $.search_metadata.count (direct search for search_metadata count) with Input impl BufferedInput and result mode IndexResult");
    let jsonpath_query = JsonPathQuery::parse("$.search_metadata.count")?;
    let json_file = fs::File::open("/home/mat/rsonpath/crates/rsonpath-test/tests/documents/toml/twitter.json")?;
    let input = BufferedInput::new(json_file);
    let engine = RecursiveEngine::compile_query(&jsonpath_query)?;
    let result = engine.run::<_, IndexResult>(&input)?;
    assert_eq!(result.get(), vec![767233usize,], "result != expected");
    Ok(())
}
#[test]
fn twitter_json_from_simdjson_github_example_with_query_direct_search_for_search_metadata_count_with_mmap_input_and_count_result_using_main_engine(
) -> Result<(), Box<dyn Error>> {
    println ! ("on document twitter.toml running the query $.search_metadata.count (direct search for search_metadata count) with Input impl MmapInput and result mode CountResult");
    let jsonpath_query = JsonPathQuery::parse("$.search_metadata.count")?;
    let json_file = fs::File::open("/home/mat/rsonpath/crates/rsonpath-test/tests/documents/toml/twitter.json")?;
    let input = unsafe { MmapInput::map_file(&json_file)? };
    let engine = MainEngine::compile_query(&jsonpath_query)?;
    let result = engine.run::<_, CountResult>(&input)?;
    assert_eq!(result.get(), 1u64, "result != expected");
    Ok(())
}
#[test]
fn twitter_json_from_simdjson_github_example_with_query_direct_search_for_search_metadata_count_with_mmap_input_and_count_result_using_recursive_engine(
) -> Result<(), Box<dyn Error>> {
    println ! ("on document twitter.toml running the query $.search_metadata.count (direct search for search_metadata count) with Input impl MmapInput and result mode CountResult");
    let jsonpath_query = JsonPathQuery::parse("$.search_metadata.count")?;
    let json_file = fs::File::open("/home/mat/rsonpath/crates/rsonpath-test/tests/documents/toml/twitter.json")?;
    let input = unsafe { MmapInput::map_file(&json_file)? };
    let engine = RecursiveEngine::compile_query(&jsonpath_query)?;
    let result = engine.run::<_, CountResult>(&input)?;
    assert_eq!(result.get(), 1u64, "result != expected");
    Ok(())
}
#[test]
fn twitter_json_from_simdjson_github_example_with_query_direct_search_for_search_metadata_count_with_mmap_input_and_index_result_using_main_engine(
) -> Result<(), Box<dyn Error>> {
    println ! ("on document twitter.toml running the query $.search_metadata.count (direct search for search_metadata count) with Input impl MmapInput and result mode IndexResult");
    let jsonpath_query = JsonPathQuery::parse("$.search_metadata.count")?;
    let json_file = fs::File::open("/home/mat/rsonpath/crates/rsonpath-test/tests/documents/toml/twitter.json")?;
    let input = unsafe { MmapInput::map_file(&json_file)? };
    let engine = MainEngine::compile_query(&jsonpath_query)?;
    let result = engine.run::<_, IndexResult>(&input)?;
    assert_eq!(result.get(), vec![767233usize,], "result != expected");
    Ok(())
}
#[test]
fn twitter_json_from_simdjson_github_example_with_query_direct_search_for_search_metadata_count_with_mmap_input_and_index_result_using_recursive_engine(
) -> Result<(), Box<dyn Error>> {
    println ! ("on document twitter.toml running the query $.search_metadata.count (direct search for search_metadata count) with Input impl MmapInput and result mode IndexResult");
    let jsonpath_query = JsonPathQuery::parse("$.search_metadata.count")?;
    let json_file = fs::File::open("/home/mat/rsonpath/crates/rsonpath-test/tests/documents/toml/twitter.json")?;
    let input = unsafe { MmapInput::map_file(&json_file)? };
    let engine = RecursiveEngine::compile_query(&jsonpath_query)?;
    let result = engine.run::<_, IndexResult>(&input)?;
    assert_eq!(result.get(), vec![767233usize,], "result != expected");
    Ok(())
}
#[test]
fn twitter_json_from_simdjson_github_example_with_query_direct_search_for_search_metadata_count_with_owned_bytes_and_count_result_using_main_engine(
) -> Result<(), Box<dyn Error>> {
    println ! ("on document twitter.toml running the query $.search_metadata.count (direct search for search_metadata count) with Input impl OwnedBytes and result mode CountResult");
    let jsonpath_query = JsonPathQuery::parse("$.search_metadata.count")?;
    let raw_json = fs::read_to_string("/home/mat/rsonpath/crates/rsonpath-test/tests/documents/toml/twitter.json")?;
    let input = OwnedBytes::new(&raw_json.as_bytes())?;
    let engine = MainEngine::compile_query(&jsonpath_query)?;
    let result = engine.run::<_, CountResult>(&input)?;
    assert_eq!(result.get(), 1u64, "result != expected");
    Ok(())
}
#[test]
fn twitter_json_from_simdjson_github_example_with_query_direct_search_for_search_metadata_count_with_owned_bytes_and_count_result_using_recursive_engine(
) -> Result<(), Box<dyn Error>> {
    println ! ("on document twitter.toml running the query $.search_metadata.count (direct search for search_metadata count) with Input impl OwnedBytes and result mode CountResult");
    let jsonpath_query = JsonPathQuery::parse("$.search_metadata.count")?;
    let raw_json = fs::read_to_string("/home/mat/rsonpath/crates/rsonpath-test/tests/documents/toml/twitter.json")?;
    let input = OwnedBytes::new(&raw_json.as_bytes())?;
    let engine = RecursiveEngine::compile_query(&jsonpath_query)?;
    let result = engine.run::<_, CountResult>(&input)?;
    assert_eq!(result.get(), 1u64, "result != expected");
    Ok(())
}
#[test]
fn twitter_json_from_simdjson_github_example_with_query_direct_search_for_search_metadata_count_with_owned_bytes_and_index_result_using_main_engine(
) -> Result<(), Box<dyn Error>> {
    println ! ("on document twitter.toml running the query $.search_metadata.count (direct search for search_metadata count) with Input impl OwnedBytes and result mode IndexResult");
    let jsonpath_query = JsonPathQuery::parse("$.search_metadata.count")?;
    let raw_json = fs::read_to_string("/home/mat/rsonpath/crates/rsonpath-test/tests/documents/toml/twitter.json")?;
    let input = OwnedBytes::new(&raw_json.as_bytes())?;
    let engine = MainEngine::compile_query(&jsonpath_query)?;
    let result = engine.run::<_, IndexResult>(&input)?;
    assert_eq!(result.get(), vec![767233usize,], "result != expected");
    Ok(())
}
#[test]
fn twitter_json_from_simdjson_github_example_with_query_direct_search_for_search_metadata_count_with_owned_bytes_and_index_result_using_recursive_engine(
) -> Result<(), Box<dyn Error>> {
    println ! ("on document twitter.toml running the query $.search_metadata.count (direct search for search_metadata count) with Input impl OwnedBytes and result mode IndexResult");
    let jsonpath_query = JsonPathQuery::parse("$.search_metadata.count")?;
    let raw_json = fs::read_to_string("/home/mat/rsonpath/crates/rsonpath-test/tests/documents/toml/twitter.json")?;
    let input = OwnedBytes::new(&raw_json.as_bytes())?;
    let engine = RecursiveEngine::compile_query(&jsonpath_query)?;
    let result = engine.run::<_, IndexResult>(&input)?;
    assert_eq!(result.get(), vec![767233usize,], "result != expected");
    Ok(())
}
#[test]
fn twitter_json_from_simdjson_github_example_with_query_direct_search_for_search_metadata_then_descendant_count_with_buffered_input_and_count_result_using_main_engine(
) -> Result<(), Box<dyn Error>> {
    println ! ("on document twitter.toml running the query $.search_metadata..count (direct search for search_metadata then descendant count) with Input impl BufferedInput and result mode CountResult");
    let jsonpath_query = JsonPathQuery::parse("$.search_metadata..count")?;
    let json_file = fs::File::open("/home/mat/rsonpath/crates/rsonpath-test/tests/documents/toml/twitter.json")?;
    let input = BufferedInput::new(json_file);
    let engine = MainEngine::compile_query(&jsonpath_query)?;
    let result = engine.run::<_, CountResult>(&input)?;
    assert_eq!(result.get(), 1u64, "result != expected");
    Ok(())
}
#[test]
fn twitter_json_from_simdjson_github_example_with_query_direct_search_for_search_metadata_then_descendant_count_with_buffered_input_and_count_result_using_recursive_engine(
) -> Result<(), Box<dyn Error>> {
    println ! ("on document twitter.toml running the query $.search_metadata..count (direct search for search_metadata then descendant count) with Input impl BufferedInput and result mode CountResult");
    let jsonpath_query = JsonPathQuery::parse("$.search_metadata..count")?;
    let json_file = fs::File::open("/home/mat/rsonpath/crates/rsonpath-test/tests/documents/toml/twitter.json")?;
    let input = BufferedInput::new(json_file);
    let engine = RecursiveEngine::compile_query(&jsonpath_query)?;
    let result = engine.run::<_, CountResult>(&input)?;
    assert_eq!(result.get(), 1u64, "result != expected");
    Ok(())
}
#[test]
fn twitter_json_from_simdjson_github_example_with_query_direct_search_for_search_metadata_then_descendant_count_with_buffered_input_and_index_result_using_main_engine(
) -> Result<(), Box<dyn Error>> {
    println ! ("on document twitter.toml running the query $.search_metadata..count (direct search for search_metadata then descendant count) with Input impl BufferedInput and result mode IndexResult");
    let jsonpath_query = JsonPathQuery::parse("$.search_metadata..count")?;
    let json_file = fs::File::open("/home/mat/rsonpath/crates/rsonpath-test/tests/documents/toml/twitter.json")?;
    let input = BufferedInput::new(json_file);
    let engine = MainEngine::compile_query(&jsonpath_query)?;
    let result = engine.run::<_, IndexResult>(&input)?;
    assert_eq!(result.get(), vec![767233usize,], "result != expected");
    Ok(())
}
#[test]
fn twitter_json_from_simdjson_github_example_with_query_direct_search_for_search_metadata_then_descendant_count_with_buffered_input_and_index_result_using_recursive_engine(
) -> Result<(), Box<dyn Error>> {
    println ! ("on document twitter.toml running the query $.search_metadata..count (direct search for search_metadata then descendant count) with Input impl BufferedInput and result mode IndexResult");
    let jsonpath_query = JsonPathQuery::parse("$.search_metadata..count")?;
    let json_file = fs::File::open("/home/mat/rsonpath/crates/rsonpath-test/tests/documents/toml/twitter.json")?;
    let input = BufferedInput::new(json_file);
    let engine = RecursiveEngine::compile_query(&jsonpath_query)?;
    let result = engine.run::<_, IndexResult>(&input)?;
    assert_eq!(result.get(), vec![767233usize,], "result != expected");
    Ok(())
}
#[test]
fn twitter_json_from_simdjson_github_example_with_query_direct_search_for_search_metadata_then_descendant_count_with_mmap_input_and_count_result_using_main_engine(
) -> Result<(), Box<dyn Error>> {
    println ! ("on document twitter.toml running the query $.search_metadata..count (direct search for search_metadata then descendant count) with Input impl MmapInput and result mode CountResult");
    let jsonpath_query = JsonPathQuery::parse("$.search_metadata..count")?;
    let json_file = fs::File::open("/home/mat/rsonpath/crates/rsonpath-test/tests/documents/toml/twitter.json")?;
    let input = unsafe { MmapInput::map_file(&json_file)? };
    let engine = MainEngine::compile_query(&jsonpath_query)?;
    let result = engine.run::<_, CountResult>(&input)?;
    assert_eq!(result.get(), 1u64, "result != expected");
    Ok(())
}
#[test]
fn twitter_json_from_simdjson_github_example_with_query_direct_search_for_search_metadata_then_descendant_count_with_mmap_input_and_count_result_using_recursive_engine(
) -> Result<(), Box<dyn Error>> {
    println ! ("on document twitter.toml running the query $.search_metadata..count (direct search for search_metadata then descendant count) with Input impl MmapInput and result mode CountResult");
    let jsonpath_query = JsonPathQuery::parse("$.search_metadata..count")?;
    let json_file = fs::File::open("/home/mat/rsonpath/crates/rsonpath-test/tests/documents/toml/twitter.json")?;
    let input = unsafe { MmapInput::map_file(&json_file)? };
    let engine = RecursiveEngine::compile_query(&jsonpath_query)?;
    let result = engine.run::<_, CountResult>(&input)?;
    assert_eq!(result.get(), 1u64, "result != expected");
    Ok(())
}
#[test]
fn twitter_json_from_simdjson_github_example_with_query_direct_search_for_search_metadata_then_descendant_count_with_mmap_input_and_index_result_using_main_engine(
) -> Result<(), Box<dyn Error>> {
    println ! ("on document twitter.toml running the query $.search_metadata..count (direct search for search_metadata then descendant count) with Input impl MmapInput and result mode IndexResult");
    let jsonpath_query = JsonPathQuery::parse("$.search_metadata..count")?;
    let json_file = fs::File::open("/home/mat/rsonpath/crates/rsonpath-test/tests/documents/toml/twitter.json")?;
    let input = unsafe { MmapInput::map_file(&json_file)? };
    let engine = MainEngine::compile_query(&jsonpath_query)?;
    let result = engine.run::<_, IndexResult>(&input)?;
    assert_eq!(result.get(), vec![767233usize,], "result != expected");
    Ok(())
}
#[test]
fn twitter_json_from_simdjson_github_example_with_query_direct_search_for_search_metadata_then_descendant_count_with_mmap_input_and_index_result_using_recursive_engine(
) -> Result<(), Box<dyn Error>> {
    println ! ("on document twitter.toml running the query $.search_metadata..count (direct search for search_metadata then descendant count) with Input impl MmapInput and result mode IndexResult");
    let jsonpath_query = JsonPathQuery::parse("$.search_metadata..count")?;
    let json_file = fs::File::open("/home/mat/rsonpath/crates/rsonpath-test/tests/documents/toml/twitter.json")?;
    let input = unsafe { MmapInput::map_file(&json_file)? };
    let engine = RecursiveEngine::compile_query(&jsonpath_query)?;
    let result = engine.run::<_, IndexResult>(&input)?;
    assert_eq!(result.get(), vec![767233usize,], "result != expected");
    Ok(())
}
#[test]
fn twitter_json_from_simdjson_github_example_with_query_direct_search_for_search_metadata_then_descendant_count_with_owned_bytes_and_count_result_using_main_engine(
) -> Result<(), Box<dyn Error>> {
    println ! ("on document twitter.toml running the query $.search_metadata..count (direct search for search_metadata then descendant count) with Input impl OwnedBytes and result mode CountResult");
    let jsonpath_query = JsonPathQuery::parse("$.search_metadata..count")?;
    let raw_json = fs::read_to_string("/home/mat/rsonpath/crates/rsonpath-test/tests/documents/toml/twitter.json")?;
    let input = OwnedBytes::new(&raw_json.as_bytes())?;
    let engine = MainEngine::compile_query(&jsonpath_query)?;
    let result = engine.run::<_, CountResult>(&input)?;
    assert_eq!(result.get(), 1u64, "result != expected");
    Ok(())
}
#[test]
fn twitter_json_from_simdjson_github_example_with_query_direct_search_for_search_metadata_then_descendant_count_with_owned_bytes_and_count_result_using_recursive_engine(
) -> Result<(), Box<dyn Error>> {
    println ! ("on document twitter.toml running the query $.search_metadata..count (direct search for search_metadata then descendant count) with Input impl OwnedBytes and result mode CountResult");
    let jsonpath_query = JsonPathQuery::parse("$.search_metadata..count")?;
    let raw_json = fs::read_to_string("/home/mat/rsonpath/crates/rsonpath-test/tests/documents/toml/twitter.json")?;
    let input = OwnedBytes::new(&raw_json.as_bytes())?;
    let engine = RecursiveEngine::compile_query(&jsonpath_query)?;
    let result = engine.run::<_, CountResult>(&input)?;
    assert_eq!(result.get(), 1u64, "result != expected");
    Ok(())
}
#[test]
fn twitter_json_from_simdjson_github_example_with_query_direct_search_for_search_metadata_then_descendant_count_with_owned_bytes_and_index_result_using_main_engine(
) -> Result<(), Box<dyn Error>> {
    println ! ("on document twitter.toml running the query $.search_metadata..count (direct search for search_metadata then descendant count) with Input impl OwnedBytes and result mode IndexResult");
    let jsonpath_query = JsonPathQuery::parse("$.search_metadata..count")?;
    let raw_json = fs::read_to_string("/home/mat/rsonpath/crates/rsonpath-test/tests/documents/toml/twitter.json")?;
    let input = OwnedBytes::new(&raw_json.as_bytes())?;
    let engine = MainEngine::compile_query(&jsonpath_query)?;
    let result = engine.run::<_, IndexResult>(&input)?;
    assert_eq!(result.get(), vec![767233usize,], "result != expected");
    Ok(())
}
#[test]
fn twitter_json_from_simdjson_github_example_with_query_direct_search_for_search_metadata_then_descendant_count_with_owned_bytes_and_index_result_using_recursive_engine(
) -> Result<(), Box<dyn Error>> {
    println ! ("on document twitter.toml running the query $.search_metadata..count (direct search for search_metadata then descendant count) with Input impl OwnedBytes and result mode IndexResult");
    let jsonpath_query = JsonPathQuery::parse("$.search_metadata..count")?;
    let raw_json = fs::read_to_string("/home/mat/rsonpath/crates/rsonpath-test/tests/documents/toml/twitter.json")?;
    let input = OwnedBytes::new(&raw_json.as_bytes())?;
    let engine = RecursiveEngine::compile_query(&jsonpath_query)?;
    let result = engine.run::<_, IndexResult>(&input)?;
    assert_eq!(result.get(), vec![767233usize,], "result != expected");
    Ok(())
}
#[test]
fn very_long_path_with_nested_member_names_with_query_find_a_select_nodes_at_depth_two_find_b_within_that_and_select_nodes_at_depth_two_from_there_with_buffered_input_and_count_result_using_main_engine(
) -> Result<(), Box<dyn Error>> {
    println ! ("on document long_paths.toml running the query $..a.*.*..b.*.* (find a, select nodes at depth two, find b within that and select nodes at depth two from there) with Input impl BufferedInput and result mode CountResult");
    let jsonpath_query = JsonPathQuery::parse("$..a.*.*..b.*.*")?;
    let json_file = fs::File::open("/home/mat/rsonpath/crates/rsonpath-test/tests/documents/toml/long_paths.json")?;
    let input = BufferedInput::new(json_file);
    let engine = MainEngine::compile_query(&jsonpath_query)?;
    let result = engine.run::<_, CountResult>(&input)?;
    assert_eq!(result.get(), 9u64, "result != expected");
    Ok(())
}
#[test]
fn very_long_path_with_nested_member_names_with_query_find_a_select_nodes_at_depth_two_find_b_within_that_and_select_nodes_at_depth_two_from_there_with_buffered_input_and_count_result_using_recursive_engine(
) -> Result<(), Box<dyn Error>> {
    println ! ("on document long_paths.toml running the query $..a.*.*..b.*.* (find a, select nodes at depth two, find b within that and select nodes at depth two from there) with Input impl BufferedInput and result mode CountResult");
    let jsonpath_query = JsonPathQuery::parse("$..a.*.*..b.*.*")?;
    let json_file = fs::File::open("/home/mat/rsonpath/crates/rsonpath-test/tests/documents/toml/long_paths.json")?;
    let input = BufferedInput::new(json_file);
    let engine = RecursiveEngine::compile_query(&jsonpath_query)?;
    let result = engine.run::<_, CountResult>(&input)?;
    assert_eq!(result.get(), 9u64, "result != expected");
    Ok(())
}
#[test]
fn very_long_path_with_nested_member_names_with_query_find_a_select_nodes_at_depth_two_find_b_within_that_and_select_nodes_at_depth_two_from_there_with_buffered_input_and_index_result_using_main_engine(
) -> Result<(), Box<dyn Error>> {
    println ! ("on document long_paths.toml running the query $..a.*.*..b.*.* (find a, select nodes at depth two, find b within that and select nodes at depth two from there) with Input impl BufferedInput and result mode IndexResult");
    let jsonpath_query = JsonPathQuery::parse("$..a.*.*..b.*.*")?;
    let json_file = fs::File::open("/home/mat/rsonpath/crates/rsonpath-test/tests/documents/toml/long_paths.json")?;
    let input = BufferedInput::new(json_file);
    let engine = MainEngine::compile_query(&jsonpath_query)?;
    let result = engine.run::<_, IndexResult>(&input)?;
    assert_eq!(
        result.get(),
        vec![654usize, 711usize, 751usize, 793usize, 857usize, 901usize, 1715usize, 1813usize, 1878usize,],
        "result != expected"
    );
    Ok(())
}
#[test]
fn very_long_path_with_nested_member_names_with_query_find_a_select_nodes_at_depth_two_find_b_within_that_and_select_nodes_at_depth_two_from_there_with_buffered_input_and_index_result_using_recursive_engine(
) -> Result<(), Box<dyn Error>> {
    println ! ("on document long_paths.toml running the query $..a.*.*..b.*.* (find a, select nodes at depth two, find b within that and select nodes at depth two from there) with Input impl BufferedInput and result mode IndexResult");
    let jsonpath_query = JsonPathQuery::parse("$..a.*.*..b.*.*")?;
    let json_file = fs::File::open("/home/mat/rsonpath/crates/rsonpath-test/tests/documents/toml/long_paths.json")?;
    let input = BufferedInput::new(json_file);
    let engine = RecursiveEngine::compile_query(&jsonpath_query)?;
    let result = engine.run::<_, IndexResult>(&input)?;
    assert_eq!(
        result.get(),
        vec![654usize, 711usize, 751usize, 793usize, 857usize, 901usize, 1715usize, 1813usize, 1878usize,],
        "result != expected"
    );
    Ok(())
}
#[test]
fn very_long_path_with_nested_member_names_with_query_find_a_select_nodes_at_depth_two_find_b_within_that_and_select_nodes_at_depth_two_from_there_with_mmap_input_and_count_result_using_main_engine(
) -> Result<(), Box<dyn Error>> {
    println ! ("on document long_paths.toml running the query $..a.*.*..b.*.* (find a, select nodes at depth two, find b within that and select nodes at depth two from there) with Input impl MmapInput and result mode CountResult");
    let jsonpath_query = JsonPathQuery::parse("$..a.*.*..b.*.*")?;
    let json_file = fs::File::open("/home/mat/rsonpath/crates/rsonpath-test/tests/documents/toml/long_paths.json")?;
    let input = unsafe { MmapInput::map_file(&json_file)? };
    let engine = MainEngine::compile_query(&jsonpath_query)?;
    let result = engine.run::<_, CountResult>(&input)?;
    assert_eq!(result.get(), 9u64, "result != expected");
    Ok(())
}
#[test]
fn very_long_path_with_nested_member_names_with_query_find_a_select_nodes_at_depth_two_find_b_within_that_and_select_nodes_at_depth_two_from_there_with_mmap_input_and_count_result_using_recursive_engine(
) -> Result<(), Box<dyn Error>> {
    println ! ("on document long_paths.toml running the query $..a.*.*..b.*.* (find a, select nodes at depth two, find b within that and select nodes at depth two from there) with Input impl MmapInput and result mode CountResult");
    let jsonpath_query = JsonPathQuery::parse("$..a.*.*..b.*.*")?;
    let json_file = fs::File::open("/home/mat/rsonpath/crates/rsonpath-test/tests/documents/toml/long_paths.json")?;
    let input = unsafe { MmapInput::map_file(&json_file)? };
    let engine = RecursiveEngine::compile_query(&jsonpath_query)?;
    let result = engine.run::<_, CountResult>(&input)?;
    assert_eq!(result.get(), 9u64, "result != expected");
    Ok(())
}
#[test]
fn very_long_path_with_nested_member_names_with_query_find_a_select_nodes_at_depth_two_find_b_within_that_and_select_nodes_at_depth_two_from_there_with_mmap_input_and_index_result_using_main_engine(
) -> Result<(), Box<dyn Error>> {
    println ! ("on document long_paths.toml running the query $..a.*.*..b.*.* (find a, select nodes at depth two, find b within that and select nodes at depth two from there) with Input impl MmapInput and result mode IndexResult");
    let jsonpath_query = JsonPathQuery::parse("$..a.*.*..b.*.*")?;
    let json_file = fs::File::open("/home/mat/rsonpath/crates/rsonpath-test/tests/documents/toml/long_paths.json")?;
    let input = unsafe { MmapInput::map_file(&json_file)? };
    let engine = MainEngine::compile_query(&jsonpath_query)?;
    let result = engine.run::<_, IndexResult>(&input)?;
    assert_eq!(
        result.get(),
        vec![654usize, 711usize, 751usize, 793usize, 857usize, 901usize, 1715usize, 1813usize, 1878usize,],
        "result != expected"
    );
    Ok(())
}
#[test]
fn very_long_path_with_nested_member_names_with_query_find_a_select_nodes_at_depth_two_find_b_within_that_and_select_nodes_at_depth_two_from_there_with_mmap_input_and_index_result_using_recursive_engine(
) -> Result<(), Box<dyn Error>> {
    println ! ("on document long_paths.toml running the query $..a.*.*..b.*.* (find a, select nodes at depth two, find b within that and select nodes at depth two from there) with Input impl MmapInput and result mode IndexResult");
    let jsonpath_query = JsonPathQuery::parse("$..a.*.*..b.*.*")?;
    let json_file = fs::File::open("/home/mat/rsonpath/crates/rsonpath-test/tests/documents/toml/long_paths.json")?;
    let input = unsafe { MmapInput::map_file(&json_file)? };
    let engine = RecursiveEngine::compile_query(&jsonpath_query)?;
    let result = engine.run::<_, IndexResult>(&input)?;
    assert_eq!(
        result.get(),
        vec![654usize, 711usize, 751usize, 793usize, 857usize, 901usize, 1715usize, 1813usize, 1878usize,],
        "result != expected"
    );
    Ok(())
}
#[test]
fn very_long_path_with_nested_member_names_with_query_find_a_select_nodes_at_depth_two_find_b_within_that_and_select_nodes_at_depth_two_from_there_with_owned_bytes_and_count_result_using_main_engine(
) -> Result<(), Box<dyn Error>> {
    println ! ("on document long_paths.toml running the query $..a.*.*..b.*.* (find a, select nodes at depth two, find b within that and select nodes at depth two from there) with Input impl OwnedBytes and result mode CountResult");
    let jsonpath_query = JsonPathQuery::parse("$..a.*.*..b.*.*")?;
    let raw_json = fs::read_to_string("/home/mat/rsonpath/crates/rsonpath-test/tests/documents/toml/long_paths.json")?;
    let input = OwnedBytes::new(&raw_json.as_bytes())?;
    let engine = MainEngine::compile_query(&jsonpath_query)?;
    let result = engine.run::<_, CountResult>(&input)?;
    assert_eq!(result.get(), 9u64, "result != expected");
    Ok(())
}
#[test]
fn very_long_path_with_nested_member_names_with_query_find_a_select_nodes_at_depth_two_find_b_within_that_and_select_nodes_at_depth_two_from_there_with_owned_bytes_and_count_result_using_recursive_engine(
) -> Result<(), Box<dyn Error>> {
    println ! ("on document long_paths.toml running the query $..a.*.*..b.*.* (find a, select nodes at depth two, find b within that and select nodes at depth two from there) with Input impl OwnedBytes and result mode CountResult");
    let jsonpath_query = JsonPathQuery::parse("$..a.*.*..b.*.*")?;
    let raw_json = fs::read_to_string("/home/mat/rsonpath/crates/rsonpath-test/tests/documents/toml/long_paths.json")?;
    let input = OwnedBytes::new(&raw_json.as_bytes())?;
    let engine = RecursiveEngine::compile_query(&jsonpath_query)?;
    let result = engine.run::<_, CountResult>(&input)?;
    assert_eq!(result.get(), 9u64, "result != expected");
    Ok(())
}
#[test]
fn very_long_path_with_nested_member_names_with_query_find_a_select_nodes_at_depth_two_find_b_within_that_and_select_nodes_at_depth_two_from_there_with_owned_bytes_and_index_result_using_main_engine(
) -> Result<(), Box<dyn Error>> {
    println ! ("on document long_paths.toml running the query $..a.*.*..b.*.* (find a, select nodes at depth two, find b within that and select nodes at depth two from there) with Input impl OwnedBytes and result mode IndexResult");
    let jsonpath_query = JsonPathQuery::parse("$..a.*.*..b.*.*")?;
    let raw_json = fs::read_to_string("/home/mat/rsonpath/crates/rsonpath-test/tests/documents/toml/long_paths.json")?;
    let input = OwnedBytes::new(&raw_json.as_bytes())?;
    let engine = MainEngine::compile_query(&jsonpath_query)?;
    let result = engine.run::<_, IndexResult>(&input)?;
    assert_eq!(
        result.get(),
        vec![654usize, 711usize, 751usize, 793usize, 857usize, 901usize, 1715usize, 1813usize, 1878usize,],
        "result != expected"
    );
    Ok(())
}
#[test]
fn very_long_path_with_nested_member_names_with_query_find_a_select_nodes_at_depth_two_find_b_within_that_and_select_nodes_at_depth_two_from_there_with_owned_bytes_and_index_result_using_recursive_engine(
) -> Result<(), Box<dyn Error>> {
    println ! ("on document long_paths.toml running the query $..a.*.*..b.*.* (find a, select nodes at depth two, find b within that and select nodes at depth two from there) with Input impl OwnedBytes and result mode IndexResult");
    let jsonpath_query = JsonPathQuery::parse("$..a.*.*..b.*.*")?;
    let raw_json = fs::read_to_string("/home/mat/rsonpath/crates/rsonpath-test/tests/documents/toml/long_paths.json")?;
    let input = OwnedBytes::new(&raw_json.as_bytes())?;
    let engine = RecursiveEngine::compile_query(&jsonpath_query)?;
    let result = engine.run::<_, IndexResult>(&input)?;
    assert_eq!(
        result.get(),
        vec![654usize, 711usize, 751usize, 793usize, 857usize, 901usize, 1715usize, 1813usize, 1878usize,],
        "result != expected"
    );
    Ok(())
}
#[test]
fn whitespace_separators_between_structurals_to_test_correctness_of_index_result_handling_with_query_select_a_by_child_with_buffered_input_and_count_result_using_main_engine(
) -> Result<(), Box<dyn Error>> {
    println ! ("on document index_result.toml running the query $.a (select a by child) with Input impl BufferedInput and result mode CountResult");
    let jsonpath_query = JsonPathQuery::parse("$.a")?;
    let json_file = fs::File::open("/home/mat/rsonpath/crates/rsonpath-test/tests/documents/toml/index_result.json")?;
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
    let json_file = fs::File::open("/home/mat/rsonpath/crates/rsonpath-test/tests/documents/toml/index_result.json")?;
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
    let json_file = fs::File::open("/home/mat/rsonpath/crates/rsonpath-test/tests/documents/toml/index_result.json")?;
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
    let json_file = fs::File::open("/home/mat/rsonpath/crates/rsonpath-test/tests/documents/toml/index_result.json")?;
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
    let json_file = fs::File::open("/home/mat/rsonpath/crates/rsonpath-test/tests/documents/toml/index_result.json")?;
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
    let json_file = fs::File::open("/home/mat/rsonpath/crates/rsonpath-test/tests/documents/toml/index_result.json")?;
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
    let json_file = fs::File::open("/home/mat/rsonpath/crates/rsonpath-test/tests/documents/toml/index_result.json")?;
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
    let json_file = fs::File::open("/home/mat/rsonpath/crates/rsonpath-test/tests/documents/toml/index_result.json")?;
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
    let raw_json =
        fs::read_to_string("/home/mat/rsonpath/crates/rsonpath-test/tests/documents/toml/index_result.json")?;
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
    let raw_json =
        fs::read_to_string("/home/mat/rsonpath/crates/rsonpath-test/tests/documents/toml/index_result.json")?;
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
    let raw_json =
        fs::read_to_string("/home/mat/rsonpath/crates/rsonpath-test/tests/documents/toml/index_result.json")?;
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
    let raw_json =
        fs::read_to_string("/home/mat/rsonpath/crates/rsonpath-test/tests/documents/toml/index_result.json")?;
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
    let json_file = fs::File::open("/home/mat/rsonpath/crates/rsonpath-test/tests/documents/toml/index_result.json")?;
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
    let json_file = fs::File::open("/home/mat/rsonpath/crates/rsonpath-test/tests/documents/toml/index_result.json")?;
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
    let json_file = fs::File::open("/home/mat/rsonpath/crates/rsonpath-test/tests/documents/toml/index_result.json")?;
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
    let json_file = fs::File::open("/home/mat/rsonpath/crates/rsonpath-test/tests/documents/toml/index_result.json")?;
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
    let json_file = fs::File::open("/home/mat/rsonpath/crates/rsonpath-test/tests/documents/toml/index_result.json")?;
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
    let json_file = fs::File::open("/home/mat/rsonpath/crates/rsonpath-test/tests/documents/toml/index_result.json")?;
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
    let json_file = fs::File::open("/home/mat/rsonpath/crates/rsonpath-test/tests/documents/toml/index_result.json")?;
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
    let json_file = fs::File::open("/home/mat/rsonpath/crates/rsonpath-test/tests/documents/toml/index_result.json")?;
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
    let raw_json =
        fs::read_to_string("/home/mat/rsonpath/crates/rsonpath-test/tests/documents/toml/index_result.json")?;
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
    let raw_json =
        fs::read_to_string("/home/mat/rsonpath/crates/rsonpath-test/tests/documents/toml/index_result.json")?;
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
    let raw_json =
        fs::read_to_string("/home/mat/rsonpath/crates/rsonpath-test/tests/documents/toml/index_result.json")?;
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
    let raw_json =
        fs::read_to_string("/home/mat/rsonpath/crates/rsonpath-test/tests/documents/toml/index_result.json")?;
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
    let json_file = fs::File::open("/home/mat/rsonpath/crates/rsonpath-test/tests/documents/toml/index_result.json")?;
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
    let json_file = fs::File::open("/home/mat/rsonpath/crates/rsonpath-test/tests/documents/toml/index_result.json")?;
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
    let json_file = fs::File::open("/home/mat/rsonpath/crates/rsonpath-test/tests/documents/toml/index_result.json")?;
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
    let json_file = fs::File::open("/home/mat/rsonpath/crates/rsonpath-test/tests/documents/toml/index_result.json")?;
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
    let json_file = fs::File::open("/home/mat/rsonpath/crates/rsonpath-test/tests/documents/toml/index_result.json")?;
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
    let json_file = fs::File::open("/home/mat/rsonpath/crates/rsonpath-test/tests/documents/toml/index_result.json")?;
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
    let json_file = fs::File::open("/home/mat/rsonpath/crates/rsonpath-test/tests/documents/toml/index_result.json")?;
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
    let json_file = fs::File::open("/home/mat/rsonpath/crates/rsonpath-test/tests/documents/toml/index_result.json")?;
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
    let raw_json =
        fs::read_to_string("/home/mat/rsonpath/crates/rsonpath-test/tests/documents/toml/index_result.json")?;
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
    let raw_json =
        fs::read_to_string("/home/mat/rsonpath/crates/rsonpath-test/tests/documents/toml/index_result.json")?;
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
    let raw_json =
        fs::read_to_string("/home/mat/rsonpath/crates/rsonpath-test/tests/documents/toml/index_result.json")?;
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
    let raw_json =
        fs::read_to_string("/home/mat/rsonpath/crates/rsonpath-test/tests/documents/toml/index_result.json")?;
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
    let json_file = fs::File::open("/home/mat/rsonpath/crates/rsonpath-test/tests/documents/toml/index_result.json")?;
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
    let json_file = fs::File::open("/home/mat/rsonpath/crates/rsonpath-test/tests/documents/toml/index_result.json")?;
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
    let json_file = fs::File::open("/home/mat/rsonpath/crates/rsonpath-test/tests/documents/toml/index_result.json")?;
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
    let json_file = fs::File::open("/home/mat/rsonpath/crates/rsonpath-test/tests/documents/toml/index_result.json")?;
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
    let json_file = fs::File::open("/home/mat/rsonpath/crates/rsonpath-test/tests/documents/toml/index_result.json")?;
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
    let json_file = fs::File::open("/home/mat/rsonpath/crates/rsonpath-test/tests/documents/toml/index_result.json")?;
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
    let json_file = fs::File::open("/home/mat/rsonpath/crates/rsonpath-test/tests/documents/toml/index_result.json")?;
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
    let json_file = fs::File::open("/home/mat/rsonpath/crates/rsonpath-test/tests/documents/toml/index_result.json")?;
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
    let raw_json =
        fs::read_to_string("/home/mat/rsonpath/crates/rsonpath-test/tests/documents/toml/index_result.json")?;
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
    let raw_json =
        fs::read_to_string("/home/mat/rsonpath/crates/rsonpath-test/tests/documents/toml/index_result.json")?;
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
    let raw_json =
        fs::read_to_string("/home/mat/rsonpath/crates/rsonpath-test/tests/documents/toml/index_result.json")?;
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
    let raw_json =
        fs::read_to_string("/home/mat/rsonpath/crates/rsonpath-test/tests/documents/toml/index_result.json")?;
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
    let json_file =
        fs::File::open("/home/mat/rsonpath/crates/rsonpath-test/tests/documents/toml/atomic_descendant.json")?;
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
    let json_file =
        fs::File::open("/home/mat/rsonpath/crates/rsonpath-test/tests/documents/toml/atomic_descendant.json")?;
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
    let json_file =
        fs::File::open("/home/mat/rsonpath/crates/rsonpath-test/tests/documents/toml/atomic_descendant.json")?;
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
    let json_file =
        fs::File::open("/home/mat/rsonpath/crates/rsonpath-test/tests/documents/toml/atomic_descendant.json")?;
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
    let json_file =
        fs::File::open("/home/mat/rsonpath/crates/rsonpath-test/tests/documents/toml/atomic_descendant.json")?;
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
    let json_file =
        fs::File::open("/home/mat/rsonpath/crates/rsonpath-test/tests/documents/toml/atomic_descendant.json")?;
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
    let json_file =
        fs::File::open("/home/mat/rsonpath/crates/rsonpath-test/tests/documents/toml/atomic_descendant.json")?;
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
    let json_file =
        fs::File::open("/home/mat/rsonpath/crates/rsonpath-test/tests/documents/toml/atomic_descendant.json")?;
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
    let raw_json =
        fs::read_to_string("/home/mat/rsonpath/crates/rsonpath-test/tests/documents/toml/atomic_descendant.json")?;
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
    let raw_json =
        fs::read_to_string("/home/mat/rsonpath/crates/rsonpath-test/tests/documents/toml/atomic_descendant.json")?;
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
    let raw_json =
        fs::read_to_string("/home/mat/rsonpath/crates/rsonpath-test/tests/documents/toml/atomic_descendant.json")?;
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
    let raw_json =
        fs::read_to_string("/home/mat/rsonpath/crates/rsonpath-test/tests/documents/toml/atomic_descendant.json")?;
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
    let json_file =
        fs::File::open("/home/mat/rsonpath/crates/rsonpath-test/tests/documents/toml/atomic_descendant.json")?;
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
    let json_file =
        fs::File::open("/home/mat/rsonpath/crates/rsonpath-test/tests/documents/toml/atomic_descendant.json")?;
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
    let json_file =
        fs::File::open("/home/mat/rsonpath/crates/rsonpath-test/tests/documents/toml/atomic_descendant.json")?;
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
    let json_file =
        fs::File::open("/home/mat/rsonpath/crates/rsonpath-test/tests/documents/toml/atomic_descendant.json")?;
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
    let json_file =
        fs::File::open("/home/mat/rsonpath/crates/rsonpath-test/tests/documents/toml/atomic_descendant.json")?;
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
    let json_file =
        fs::File::open("/home/mat/rsonpath/crates/rsonpath-test/tests/documents/toml/atomic_descendant.json")?;
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
    let json_file =
        fs::File::open("/home/mat/rsonpath/crates/rsonpath-test/tests/documents/toml/atomic_descendant.json")?;
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
    let json_file =
        fs::File::open("/home/mat/rsonpath/crates/rsonpath-test/tests/documents/toml/atomic_descendant.json")?;
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
    let raw_json =
        fs::read_to_string("/home/mat/rsonpath/crates/rsonpath-test/tests/documents/toml/atomic_descendant.json")?;
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
    let raw_json =
        fs::read_to_string("/home/mat/rsonpath/crates/rsonpath-test/tests/documents/toml/atomic_descendant.json")?;
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
    let raw_json =
        fs::read_to_string("/home/mat/rsonpath/crates/rsonpath-test/tests/documents/toml/atomic_descendant.json")?;
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
    let raw_json =
        fs::read_to_string("/home/mat/rsonpath/crates/rsonpath-test/tests/documents/toml/atomic_descendant.json")?;
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
    let json_file =
        fs::File::open("/home/mat/rsonpath/crates/rsonpath-test/tests/documents/toml/atomic_descendant.json")?;
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
    let json_file =
        fs::File::open("/home/mat/rsonpath/crates/rsonpath-test/tests/documents/toml/atomic_descendant.json")?;
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
    let json_file =
        fs::File::open("/home/mat/rsonpath/crates/rsonpath-test/tests/documents/toml/atomic_descendant.json")?;
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
    let json_file =
        fs::File::open("/home/mat/rsonpath/crates/rsonpath-test/tests/documents/toml/atomic_descendant.json")?;
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
    let json_file =
        fs::File::open("/home/mat/rsonpath/crates/rsonpath-test/tests/documents/toml/atomic_descendant.json")?;
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
    let json_file =
        fs::File::open("/home/mat/rsonpath/crates/rsonpath-test/tests/documents/toml/atomic_descendant.json")?;
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
    let json_file =
        fs::File::open("/home/mat/rsonpath/crates/rsonpath-test/tests/documents/toml/atomic_descendant.json")?;
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
    let json_file =
        fs::File::open("/home/mat/rsonpath/crates/rsonpath-test/tests/documents/toml/atomic_descendant.json")?;
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
    let raw_json =
        fs::read_to_string("/home/mat/rsonpath/crates/rsonpath-test/tests/documents/toml/atomic_descendant.json")?;
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
    let raw_json =
        fs::read_to_string("/home/mat/rsonpath/crates/rsonpath-test/tests/documents/toml/atomic_descendant.json")?;
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
    let raw_json =
        fs::read_to_string("/home/mat/rsonpath/crates/rsonpath-test/tests/documents/toml/atomic_descendant.json")?;
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
    let raw_json =
        fs::read_to_string("/home/mat/rsonpath/crates/rsonpath-test/tests/documents/toml/atomic_descendant.json")?;
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
    let json_file =
        fs::File::open("/home/mat/rsonpath/crates/rsonpath-test/tests/documents/toml/atomic_descendant.json")?;
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
    let json_file =
        fs::File::open("/home/mat/rsonpath/crates/rsonpath-test/tests/documents/toml/atomic_descendant.json")?;
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
    let json_file =
        fs::File::open("/home/mat/rsonpath/crates/rsonpath-test/tests/documents/toml/atomic_descendant.json")?;
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
    let json_file =
        fs::File::open("/home/mat/rsonpath/crates/rsonpath-test/tests/documents/toml/atomic_descendant.json")?;
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
    let json_file =
        fs::File::open("/home/mat/rsonpath/crates/rsonpath-test/tests/documents/toml/atomic_descendant.json")?;
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
    let json_file =
        fs::File::open("/home/mat/rsonpath/crates/rsonpath-test/tests/documents/toml/atomic_descendant.json")?;
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
    let json_file =
        fs::File::open("/home/mat/rsonpath/crates/rsonpath-test/tests/documents/toml/atomic_descendant.json")?;
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
    let json_file =
        fs::File::open("/home/mat/rsonpath/crates/rsonpath-test/tests/documents/toml/atomic_descendant.json")?;
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
    let raw_json =
        fs::read_to_string("/home/mat/rsonpath/crates/rsonpath-test/tests/documents/toml/atomic_descendant.json")?;
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
    let raw_json =
        fs::read_to_string("/home/mat/rsonpath/crates/rsonpath-test/tests/documents/toml/atomic_descendant.json")?;
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
    let raw_json =
        fs::read_to_string("/home/mat/rsonpath/crates/rsonpath-test/tests/documents/toml/atomic_descendant.json")?;
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
    let raw_json =
        fs::read_to_string("/home/mat/rsonpath/crates/rsonpath-test/tests/documents/toml/atomic_descendant.json")?;
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
    let json_file =
        fs::File::open("/home/mat/rsonpath/crates/rsonpath-test/tests/documents/toml/atomic_descendant.json")?;
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
    let json_file =
        fs::File::open("/home/mat/rsonpath/crates/rsonpath-test/tests/documents/toml/atomic_descendant.json")?;
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
    let json_file =
        fs::File::open("/home/mat/rsonpath/crates/rsonpath-test/tests/documents/toml/atomic_descendant.json")?;
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
    let json_file =
        fs::File::open("/home/mat/rsonpath/crates/rsonpath-test/tests/documents/toml/atomic_descendant.json")?;
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
    let json_file =
        fs::File::open("/home/mat/rsonpath/crates/rsonpath-test/tests/documents/toml/atomic_descendant.json")?;
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
    let json_file =
        fs::File::open("/home/mat/rsonpath/crates/rsonpath-test/tests/documents/toml/atomic_descendant.json")?;
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
    let json_file =
        fs::File::open("/home/mat/rsonpath/crates/rsonpath-test/tests/documents/toml/atomic_descendant.json")?;
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
    let json_file =
        fs::File::open("/home/mat/rsonpath/crates/rsonpath-test/tests/documents/toml/atomic_descendant.json")?;
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
    let raw_json =
        fs::read_to_string("/home/mat/rsonpath/crates/rsonpath-test/tests/documents/toml/atomic_descendant.json")?;
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
    let raw_json =
        fs::read_to_string("/home/mat/rsonpath/crates/rsonpath-test/tests/documents/toml/atomic_descendant.json")?;
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
    let raw_json =
        fs::read_to_string("/home/mat/rsonpath/crates/rsonpath-test/tests/documents/toml/atomic_descendant.json")?;
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
    let raw_json =
        fs::read_to_string("/home/mat/rsonpath/crates/rsonpath-test/tests/documents/toml/atomic_descendant.json")?;
    let input = OwnedBytes::new(&raw_json.as_bytes())?;
    let engine = RecursiveEngine::compile_query(&jsonpath_query)?;
    let result = engine.run::<_, IndexResult>(&input)?;
    assert_eq!(result.get(), vec![34usize,], "result != expected");
    Ok(())
}
