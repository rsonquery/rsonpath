use crate::{discovery::DiscoveredDocument, model, paths};

pub(crate) fn get_compressed_toml_files(
    docs: &[DiscoveredDocument],
) -> impl IntoIterator<Item = DiscoveredDocument> + '_ {
    docs.iter().map(|doc| {
        let path = paths::get_path_to_compressed(doc);
        let name = format!("{}_compressed", doc.name);
        let compressed_doc = compress_document(&doc.document);

        DiscoveredDocument {
            name,
            relative_path: path,
            document: compressed_doc,
        }
    })
}

fn compress_document(doc: &model::Document) -> model::Document {
    doc.clone()
}
