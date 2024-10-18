use std::path::Path;

#[inline]
#[must_use]
pub fn extract_filename(path: &str) -> String {
    let path = std::path::Path::new(path);
    let filename = path.file_stem().expect("Failed to extract filename");
    filename.to_string_lossy().into_owned()
}

#[inline]
#[must_use]
pub fn get_filetype_from_path(path: &str) -> String {
    let path = Path::new(path);
    match path.extension() {
        Some(ext) => ext.to_string_lossy().into_owned(),
        None => String::new(),
    }
}
