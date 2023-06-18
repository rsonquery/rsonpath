use crate::model;
use std::{
    fs, io,
    path::{Path, PathBuf},
};

pub fn discover<P: AsRef<Path>>(directory_path: P) -> Result<impl IntoIterator<Item = model::NamedDocument>, io::Error> {
    let all_document_files = get_document_files(directory_path.as_ref())?;
    Ok(all_document_files.into_iter().map(read_document))
}

fn get_document_files(dir_path: &Path) -> Result<impl IntoIterator<Item = PathBuf>, io::Error> {
    let dir_path = dir_path.canonicalize()?;
    let dir = fs::read_dir(dir_path)?;
    Ok(dir
        .into_iter()
        .filter_map(|x| x.ok())
        .filter(|x| x.file_type().is_ok_and(|t| t.is_file()))
        .map(|x| x.path()))
}

fn read_document<P: AsRef<Path>>(f: P) -> model::NamedDocument {
    let file_name = f.as_ref().file_name().unwrap().to_string_lossy();
    let contents = fs::read_to_string(f.as_ref()).unwrap();

    let document: model::Document = toml::from_str(&contents).unwrap();

    model::NamedDocument {
        name: file_name.to_string(),
        document,
    }
}
