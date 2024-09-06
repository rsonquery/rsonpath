use std::path::Path;

/// Extracts the file name (without extension) from a given file path.
///
/// # Arguments
/// * `path` - A string slice representing the file path.
///
/// # Returns
/// * `String` - The file name (without extension) as a `String`.
///
/// # Panics
/// This function will panic if the file name cannot be extracted from the path.
///
/// # Example
/// ```
/// let filename = get_filename_from_path("/path/to/file.txt");
/// assert_eq!(filename, "file");
/// ```
#[inline]
#[must_use]
pub fn get_filename_from_path(path: &str) -> String {
    let path = std::path::Path::new(path);
    let filename = path.file_stem().expect("Failed to extract filename");
    filename.to_string_lossy().into_owned()
}

/// Extracts the file extension (without the dot) from a given file path.
///
/// # Arguments
/// * `path` - A string slice representing the file path.
///
/// # Returns
/// * `String` - The file extension as a `String`, or an empty string if no extension exists.
///
/// # Example
/// ```
/// let filetype = get_filetype_from_path("/path/to/file.txt");
/// assert_eq!(filetype, "txt");
/// ```
#[inline]
#[must_use]
pub fn get_filetype_from_path(path: &str) -> String {
    let path = Path::new(path);
    match path.extension() {
        Some(ext) => ext.to_string_lossy().into_owned(),
        None => String::new(),
    }
}
