//! Filesystem context for registering files that need to be written.
//!
//! The decision to create a new file can be taken in many different places during codegen,
//! so we pass around a [`Files`] context that can register those requests. Then the file writing
//! is performed all at once at the end of the generation.
use crate::{
    gen,
    model::{self, ConfigurationError},
    DiscoveredDocument, DocumentName,
};
use proc_macro2::TokenStream;
use std::{
    collections::HashMap,
    error::Error,
    fmt::Display,
    fs, io,
    path::{Path, PathBuf},
};
use walkdir::WalkDir;

struct FileToWrite {
    full_path: PathBuf,
    contents: String,
}

/// Filesystem context.
pub(crate) struct Files {
    json_dir: PathBuf,
    test_dir: PathBuf,
    toml_dir: PathBuf,
    toml_documents: HashMap<DocumentName, DiscoveredDocument>,
    file_buf: Vec<FileToWrite>,
}

pub(crate) struct Stats {
    total_documents: usize,
    total_queries: usize,
}

impl Stats {
    pub(crate) fn number_of_documents(&self) -> usize {
        self.total_documents
    }

    pub(crate) fn number_of_queries(&self) -> usize {
        self.total_queries
    }
}

impl Files {
    /// Create a new context that can read and write files to the TOML and JSON dirs.
    pub(crate) fn new<P1: AsRef<Path>, P2: AsRef<Path>, P3: AsRef<Path>>(
        json_dir: P1,
        toml_dir: P2,
        test_dir: P3,
    ) -> Result<Self, CombinedError> {
        let all_document_files = get_document_files(toml_dir.as_ref());
        let discovery = all_document_files
            .into_iter()
            .map(|doc| (read_document(toml_dir.as_ref(), &doc), doc));

        let mut oks = HashMap::new();
        let mut errs = CombinedError::new();

        for (res, path) in discovery {
            match res {
                Ok(ok) => {
                    oks.insert(ok.name.clone(), ok);
                }
                Err(err) => {
                    errs.add(path, err);
                }
            };
        }

        if !errs.is_empty() {
            Err(errs)
        } else {
            Ok(Self {
                json_dir: json_dir.as_ref().to_path_buf(),
                test_dir: test_dir.as_ref().to_path_buf(),
                toml_dir: toml_dir.as_ref().to_path_buf(),
                toml_documents: oks,
                file_buf: vec![],
            })
        }
    }

    /// Returns a list of all available TOML configurations parsed into [`DiscoveredDocument`].
    pub(crate) fn documents(&self) -> impl IntoIterator<Item = &DiscoveredDocument> {
        self.toml_documents.values()
    }

    pub(crate) fn stats(&self) -> Stats {
        let total_documents = self.toml_documents.len();
        let total_queries = self.toml_documents.iter().map(|x| x.1.document.queries.len()).sum();

        Stats {
            total_documents,
            total_queries,
        }
    }

    /// Read a JSON using a path relative to the JSON directory.
    pub(crate) fn read_json<P: AsRef<Path>>(&self, relative_path: P) -> Result<String, io::Error> {
        let full_path = Path::join(&self.json_dir, relative_path);
        fs::read_to_string(full_path)
    }

    /// Get the path to a file-based input source from a relative path.
    pub(crate) fn get_json_source_path<P: AsRef<Path>>(&self, relative_path: P) -> PathBuf {
        // This is a bit of a hack, compressed files are passed with a non-relative path
        // by `compression`, and we need to support that. So we only append the JSON base dir
        // if it is not already there.
        if !relative_path.as_ref().starts_with(&self.json_dir) {
            Path::join(&self.json_dir, relative_path)
        } else {
            relative_path.as_ref().to_path_buf()
        }
    }

    /// Register a JSON file to write that is a compressed version of the file at `original_path`.
    pub(crate) fn add_compressed_large_json<P: AsRef<Path>>(
        &mut self,
        original_path: P,
        json_string: String,
    ) -> PathBuf {
        let file_name = original_path
            .as_ref()
            .file_name()
            .expect("all documents should have a file path");
        let mut new_path = self.compressed_large_json_dir();
        new_path.push(file_name);
        new_path.set_extension("json");

        self.file_buf.push(FileToWrite {
            full_path: new_path.clone(),
            contents: json_string,
        });

        new_path
    }

    /// Register a JSON file to write that is a copy of the inline json string in the `doc`.
    pub(crate) fn add_json_source(&mut self, doc: &DiscoveredDocument, json_string: String) -> PathBuf {
        let file_name = doc
            .relative_path
            .file_name()
            .expect("all documents should have a file path");
        let dir = if doc.document.input.is_compressed {
            Path::join(&self.json_dir, "compressed")
        } else {
            self.json_dir.clone()
        };
        let mut new_path = Path::join(&dir, file_name);
        new_path.set_extension("json");

        self.file_buf.push(FileToWrite {
            full_path: new_path.clone(),
            contents: json_string,
        });

        new_path
    }

    /// Register a generated group of tests for a single document.
    pub(crate) fn add_test_group(&mut self, test_group: &gen::DocumentTestGroup) {
        let mut directory_name = test_group.name.file_path();
        directory_name.set_extension("");

        for query_group in &test_group.query_test_groups {
            let file_name = query_group.name.clone() + ".rs";
            let file_path = Path::join(&directory_name, file_name);
            self.add_rust_file(file_path, &query_group.source);
        }
    }

    pub(crate) fn add_rust_file<P: AsRef<Path>>(&mut self, relative_path: P, source: &TokenStream) {
        let directory_path = Path::join(&self.test_dir, relative_path.as_ref());
        let source = format!("{}", source).replace("\r\n", "\n");

        self.file_buf.push(FileToWrite {
            full_path: directory_path,
            contents: source,
        })
    }

    /// Register a TOML file to write that is a version of an existing TOML file but with compressed input.
    pub(crate) fn add_compressed_document<P: AsRef<Path>>(
        &mut self,
        relative_path: P,
        name: &DocumentName,
        compressed_doc: model::Document,
    ) -> PathBuf {
        let file_name = relative_path
            .as_ref()
            .file_name()
            .expect("toml document must have a file name");
        let new_dir_path = self.compressed_toml_dir();
        let new_path = Path::join(&new_dir_path, file_name);
        let new_name = name.as_compressed();

        self.file_buf.push(FileToWrite {
            full_path: new_path.clone(),
            contents: model::serialize(&compressed_doc),
        });
        let new_doc = DiscoveredDocument {
            document: compressed_doc,
            name: new_name,
            relative_path: new_path.clone(),
        };
        self.toml_documents.insert(new_doc.name.clone(), new_doc);

        new_path
    }

    /// Write all registered files to the filesystem.
    pub(crate) fn flush(&mut self) -> Result<(), io::Error> {
        for file_to_write in self.file_buf.drain(..) {
            write_file(file_to_write.full_path, file_to_write.contents)?;
        }

        Ok(())
    }

    fn compressed_large_json_dir(&self) -> PathBuf {
        Path::join(&self.json_dir, "large/compressed")
    }

    fn compressed_toml_dir(&self) -> PathBuf {
        Path::join(&self.toml_dir, "compressed")
    }
}

fn get_document_files(dir_path: &Path) -> impl IntoIterator<Item = PathBuf> {
    WalkDir::new(dir_path)
        .into_iter()
        .filter_map(Result::ok)
        .filter(|x| x.file_type().is_file() && x.path().extension().is_some_and(|e| e == "toml"))
        .map(|x| x.path().to_path_buf())
}

fn read_document<P1: AsRef<Path>, P2: AsRef<Path>>(
    base_dir: P1,
    f: P2,
) -> Result<DiscoveredDocument, ConfigurationError> {
    let file_name = f.as_ref().file_stem().unwrap().to_string_lossy();
    let contents = fs::read_to_string(f.as_ref()).unwrap();

    let document: model::Document = model::deserialize(contents)?;

    let relative_path = f
        .as_ref()
        .strip_prefix(base_dir)
        .expect("all discovered docs should be within the base dir")
        .to_owned();

    let name = if document.input.is_compressed {
        DocumentName::compressed(file_name)
    } else {
        DocumentName::uncompressed(file_name)
    };

    Ok(DiscoveredDocument {
        name,
        relative_path,
        document,
    })
}

fn write_file<P: AsRef<Path>, D: Display>(path: P, contents: D) -> Result<(), io::Error> {
    create_parent_dirs(&path)?;

    println!("writing to {}...", path.as_ref().to_string_lossy());
    fs::write(path, contents.to_string())
}

fn create_parent_dirs<P: AsRef<Path>>(path: P) -> Result<(), io::Error> {
    let dir = path.as_ref().parent().expect("generated files must have a parent");
    fs::create_dir_all(dir)
}

#[derive(Debug)]
pub struct CombinedError {
    errs: Vec<(PathBuf, ConfigurationError)>,
}

impl CombinedError {
    fn new() -> Self {
        Self { errs: vec![] }
    }

    fn add(&mut self, path: PathBuf, err: ConfigurationError) {
        self.errs.push((path, err));
    }

    fn is_empty(&self) -> bool {
        self.errs.is_empty()
    }
}

impl Display for CombinedError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for err in &self.errs {
            writeln!(f, "error in test configuration {}:\n    {}", err.0.display(), err.1)?;
        }

        Ok(())
    }
}

impl Error for CombinedError {}
