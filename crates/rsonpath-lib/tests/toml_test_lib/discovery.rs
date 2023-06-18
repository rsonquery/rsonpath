use crate::model;
use std::{
    fs, io,
    path::{Path, PathBuf},
};

const CASE_DIRECTORY_PATH: &str = "./tests/end_to_end/cases";

pub fn discover() -> Result<impl IntoIterator<Item = model::NamedCase>, io::Error> {
    let all_case_files = get_case_files()?;
    Ok(all_case_files.into_iter().map(read_case))
}

fn get_case_files() -> Result<impl IntoIterator<Item = PathBuf>, io::Error> {
    let dir_path: &Path = CASE_DIRECTORY_PATH.as_ref();
    let dir_path = dir_path.canonicalize()?;
    let dir = fs::read_dir(dir_path)?;
    Ok(dir
        .into_iter()
        .filter_map(|x| x.ok())
        .filter(|x| x.file_type().is_ok_and(|t| t.is_file()))
        .map(|x| x.path()))
}

fn read_case<P: AsRef<Path>>(f: P) -> model::NamedCase {
    let file_name = f.as_ref().file_name().unwrap().to_string_lossy();
    let contents = fs::read_to_string(f.as_ref()).unwrap();

    let case: model::Case = toml::from_str(&contents).unwrap();

    model::NamedCase {
        name: file_name.to_string(),
        case,
    }
}
