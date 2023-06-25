use crate::model;
use std::{
    fs, io,
    path::{Path, PathBuf},
};
use walkdir::WalkDir;

pub(crate) struct DiscoveredDocument {
    pub name: String,
    pub relative_path: PathBuf,
    pub document: model::Document,
}

pub(crate) fn discover<P: AsRef<Path>>(
    directory_path: P,
) -> Result<impl IntoIterator<Item = DiscoveredDocument>, io::Error> {
    let dir_path = directory_path.as_ref().canonicalize()?;
    let all_document_files = get_document_files(&dir_path);
    Ok(all_document_files
        .into_iter()
        .map(move |doc| read_document(&dir_path, doc)))
}

fn get_document_files(dir_path: &Path) -> impl IntoIterator<Item = PathBuf> {
    WalkDir::new(dir_path)
        .into_iter()
        .filter_map(|x| x.ok())
        .filter(|x| x.file_type().is_file() && x.path().extension().is_some_and(|e| e == "toml"))
        .map(|x| x.path().to_path_buf())
}

fn read_document<P1: AsRef<Path>, P2: AsRef<Path>>(base_dir: P1, f: P2) -> DiscoveredDocument {
    let file_name = f.as_ref().file_name().unwrap().to_string_lossy();
    let contents = fs::read_to_string(f.as_ref()).unwrap();

    let document: model::Document = toml::from_str(&contents).unwrap();
    let relative_path = f
        .as_ref()
        .strip_prefix(base_dir)
        .expect("all discovered docs should be within the base dir")
        .to_owned();

    DiscoveredDocument {
        name: file_name.to_string(),
        relative_path,
        document,
    }
}
