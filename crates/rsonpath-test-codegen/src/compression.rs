use crate::{
    files::Files,
    model::{self},
    DiscoveredDocument,
};
use std::{io, string::FromUtf8Error};

pub(crate) fn generate_compressed_documents(files: &mut Files) -> Result<(), io::Error> {
    let original_documents: Vec<_> = files.documents().into_iter().cloned().collect();

    original_documents
        .into_iter()
        .filter(|doc| !doc.document.input.is_compressed)
        .try_for_each(|doc| compress_document(files, &doc))
}

fn compress_document(files: &mut Files, doc: &DiscoveredDocument) -> Result<(), io::Error> {
    let (required_file, contents) = match &doc.document.input.source {
        model::InputSource::LargeFile(f) => (Some(f), files.read_json(f)?),
        model::InputSource::JsonString(c) => (None, c.clone()),
    };

    let compressed_input = CompressedInput::new(&contents);
    let queries = doc
        .document
        .queries
        .iter()
        .map(|q| {
            let compressed_results = compressed_input.transform_results(&q.results);

            model::Query {
                results: compressed_results,
                description: q.description.clone(),
                query: q.query.clone(),
            }
        })
        .collect();

    let json_string = compressed_input.into_string();
    let source = if let Some(f) = required_file {
        let relative_path = files.add_compressed_large_json(f, json_string);
        model::InputSource::LargeFile(relative_path)
    } else {
        model::InputSource::JsonString(json_string)
    };

    let compressed_doc = model::Document {
        input: model::Input {
            description: format!("{} (compressed)", doc.document.input.description),
            is_compressed: true,
            source,
        },
        queries,
    };

    files.add_compressed_document(&doc.relative_path, doc.name.clone(), compressed_doc);

    Ok(())
}

struct CompressedInput {
    json_string: JsonString,
}

#[derive(Clone, Copy)]
enum JsonByte {
    Significant(u8),
    Whitespace,
}

struct JsonString(Vec<JsonByte>);

impl From<JsonString> for Vec<u8> {
    fn from(value: JsonString) -> Self {
        value.0.into_iter().filter_map(|x| x.byte()).collect()
    }
}

impl TryFrom<JsonString> for String {
    type Error = FromUtf8Error;

    fn try_from(value: JsonString) -> Result<Self, Self::Error> {
        let bytes: Vec<u8> = value.into();
        String::from_utf8(bytes)
    }
}

impl CompressedInput {
    fn new(json: &str) -> Self {
        Self {
            json_string: JsonString::new(json),
        }
    }

    fn into_string(self) -> String {
        let bytes: Vec<u8> = self.json_string.into();
        String::from_utf8(bytes).expect("valid utf8 should be valid after compression")
    }

    fn transform_results(&self, original_results: &model::Results) -> model::Results {
        let count = original_results.count; // Count is obviously unchanged.
        let bytes = original_results.bytes.as_ref().map(|r| self.transform_byte_results(r));
        let nodes = original_results.nodes.as_ref().map(|n| self.transform_node_results(n));

        model::Results { count, bytes, nodes }
    }

    fn transform_byte_results(&self, bytes: &[usize]) -> Vec<usize> {
        let mut st = 0_usize;
        let new_indices: Vec<_> = self
            .json_string
            .0
            .iter()
            .copied()
            .map(|x| {
                let res = st;
                if x.is_significant() {
                    st += 1
                }
                res
            })
            .collect();

        bytes.iter().copied().map(|b| new_indices[b]).collect()
    }

    fn transform_node_results(&self, nodes: &[String]) -> Vec<String> {
        nodes
            .iter()
            .map(|n| {
                let bytes: Vec<u8> = JsonString::new(n).into();
                String::from_utf8(bytes).expect("valid utf8 should be valid after compression")
            })
            .collect()
    }
}

impl JsonByte {
    fn is_significant(&self) -> bool {
        matches!(self, JsonByte::Significant(_))
    }

    fn is_whitespace_byte(byte: u8) -> bool {
        const JSON_WHITESPACE: [u8; 4] = [0x20, 0x09, 0x0A, 0x0D];

        JSON_WHITESPACE.contains(&byte)
    }

    fn byte(self) -> Option<u8> {
        match self {
            JsonByte::Significant(b) => Some(b),
            JsonByte::Whitespace => None,
        }
    }
}

impl JsonString {
    fn new(json: &str) -> Self {
        struct State {
            is_escaped: bool,
            within_string: bool,
        }
        let mut st = State {
            is_escaped: false,
            within_string: false,
        };

        let vec = json
            .as_bytes()
            .iter()
            .copied()
            .map(|b| {
                if b == b'"' && !st.is_escaped {
                    st.within_string = !st.within_string;
                }

                st.is_escaped = b == b'\\' && !st.is_escaped;

                if !st.within_string && JsonByte::is_whitespace_byte(b) {
                    JsonByte::Whitespace
                } else {
                    JsonByte::Significant(b)
                }
            })
            .collect();

        JsonString(vec)
    }
}

#[cfg(test)]
mod tests {
    use super::JsonString;
    use pretty_assertions::assert_eq;

    #[test]
    fn compress_json_with_escaped_quotes() {
        let json = r#"
{"\"b": 42,
 "b": 43 
}"#;
        let expected = r#"{"\"b":42,"b":43}"#;

        let json_string = JsonString::new(json);
        let result: String = json_string.try_into().unwrap();

        assert_eq!(expected, result)
    }
}
