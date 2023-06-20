use crate::model;
use std::{
    fs, io,
    path::{Path, PathBuf},
};

pub(crate) struct DiscoveredDocument {
    pub name: String,
    pub path: PathBuf,
    pub document: model::Document,
}

pub(crate) fn discover<P: AsRef<Path>>(
    directory_path: P,
) -> Result<impl IntoIterator<Item = DiscoveredDocument>, io::Error> {
    let all_document_files = get_document_files(directory_path.as_ref())?;
    Ok(all_document_files.into_iter().map(read_document))
}

fn get_document_files(dir_path: &Path) -> Result<impl IntoIterator<Item = PathBuf>, io::Error> {
    let dir_path = dir_path.canonicalize()?;
    let dir = fs::read_dir(dir_path)?;
    Ok(dir
        .into_iter()
        .filter_map(|x| x.ok())
        .filter(|x| x.file_type().is_ok_and(|t| t.is_file()) && x.path().extension().is_some_and(|e| e == "toml"))
        .map(|x| x.path()))
}

fn read_document<P: AsRef<Path>>(f: P) -> DiscoveredDocument {
    let file_name = f.as_ref().file_name().unwrap().to_string_lossy();
    let contents = fs::read_to_string(f.as_ref()).unwrap();

    let document: model::Document = toml::from_str(&contents).unwrap();

    DiscoveredDocument {
        name: file_name.to_string(),
        path: f.as_ref().to_owned(),
        document,
    }
}
