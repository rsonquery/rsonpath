use crate::discovery::DiscoveredDocument;
use std::path::{Path, PathBuf};

pub(crate) fn get_path_to_compressed(doc: &DiscoveredDocument) -> PathBuf {
    let file_name = doc
        .relative_path
        .file_name()
        .expect("toml document must have a file name");
    let dir_path = doc.relative_path.parent().expect("toml document must have a parent");

    let new_dir_path = Path::join(dir_path, "compressed");

    Path::join(&new_dir_path, file_name)
}

pub(crate) fn get_path_of_json_for_document<P: AsRef<Path>>(json_dir: P, document: &DiscoveredDocument) -> PathBuf {
    let mut new_path = json_dir.as_ref().to_path_buf();
    new_path.push(&document.relative_path);
    new_path.set_extension("json");

    new_path
}
