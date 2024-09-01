use std::path::Path;

#[inline]
pub fn get_filename_from_path(path: &str) -> String {
    let path = std::path::Path::new(path);
    let filename = path.file_stem().expect("Failed to extract filename");
    filename.to_string_lossy().into_owned()
}

#[inline]
pub fn get_filetype_from_path(path: &str) -> String {
    let path = Path::new(path);
    match path.extension() {
        Some(ext) => ext.to_string_lossy().into_owned(),
        None => String::new(), // Return an empty string if there's no extension
    }
}
