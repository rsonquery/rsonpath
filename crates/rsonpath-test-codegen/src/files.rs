use crate::{model, DiscoveredDocument};
use std::{
    collections::HashMap,
    fmt::Display,
    fs, io,
    path::{Path, PathBuf},
};
use walkdir::WalkDir;

struct FileToWrite {
    full_path: PathBuf,
    contents: String,
}

pub(crate) struct Files {
    json_dir: PathBuf,
    toml_dir: PathBuf,
    toml_documents: HashMap<String, DiscoveredDocument>,
    file_buf: Vec<FileToWrite>,
}

pub(crate) struct Stats {
    total_documents: usize,
    total_queries: usize,
}

impl Stats {
    pub fn number_of_documents(&self) -> usize {
        self.total_documents
    }

    pub fn number_of_queries(&self) -> usize {
        self.total_queries
    }
}

impl Files {
    pub(crate) fn new<P1: AsRef<Path>, P2: AsRef<Path>>(json_dir: P1, toml_dir: P2) -> Result<Self, io::Error> {
        let all_document_files = get_document_files(toml_dir.as_ref());
        let discovery = all_document_files
            .into_iter()
            .map(|doc| read_document(toml_dir.as_ref(), doc))
            .map(|d| (d.name.clone(), d))
            .collect();

        Ok(Self {
            json_dir: json_dir.as_ref().to_path_buf(),
            toml_dir: toml_dir.as_ref().to_path_buf(),
            toml_documents: discovery,
            file_buf: vec![],
        })
    }

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

    pub(crate) fn read_json<P: AsRef<Path>>(&self, relative_path: P) -> Result<String, io::Error> {
        let full_path = Path::join(&self.json_dir, relative_path);
        fs::read_to_string(full_path)
    }

    pub(crate) fn get_json_source_path<P: AsRef<Path>>(&self, relative_path: P) -> PathBuf {
        if !relative_path.as_ref().starts_with(&self.json_dir) {
            Path::join(&self.json_dir, relative_path)
        } else {
            relative_path.as_ref().to_path_buf()
        }
    }

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

    pub(crate) fn add_compressed_document<P: AsRef<Path>>(
        &mut self,
        relative_path: P,
        name: String,
        compressed_doc: model::Document,
    ) -> PathBuf {
        let file_name = relative_path
            .as_ref()
            .file_name()
            .expect("toml document must have a file name");
        let new_dir_path = self.compressed_toml_dir();
        let new_path = Path::join(&new_dir_path, file_name);

        self.file_buf.push(FileToWrite {
            full_path: new_path.clone(),
            contents: model::serialize(&compressed_doc),
        });
        let new_doc = DiscoveredDocument {
            document: compressed_doc,
            name: format!("compressed/{}", name),
            relative_path: new_path.clone(),
        };
        self.toml_documents.insert(new_doc.name.clone(), new_doc);

        new_path
    }

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
        .filter_map(|x| x.ok())
        .filter(|x| x.file_type().is_file() && x.path().extension().is_some_and(|e| e == "toml"))
        .map(|x| x.path().to_path_buf())
}

fn read_document<P1: AsRef<Path>, P2: AsRef<Path>>(base_dir: P1, f: P2) -> DiscoveredDocument {
    let file_name = f.as_ref().file_name().unwrap().to_string_lossy();
    let contents = fs::read_to_string(f.as_ref()).unwrap();

    let document: model::Document = model::deserialize(contents);
    let relative_path = f
        .as_ref()
        .strip_prefix(base_dir)
        .expect("all discovered docs should be within the base dir")
        .to_owned();

    let name = if document.input.is_compressed {
        format!("compressed/{}", file_name)
    } else {
        file_name.to_string()
    };

    DiscoveredDocument {
        name,
        relative_path,
        document,
    }
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
