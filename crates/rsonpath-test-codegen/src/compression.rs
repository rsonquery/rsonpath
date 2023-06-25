use std::string::FromUtf8Error;

use crate::{
    discovery::DiscoveredDocument,
    model::{self, Input},
    paths,
};

pub(crate) fn get_compressed_toml_files(
    docs: &[DiscoveredDocument],
) -> impl IntoIterator<Item = DiscoveredDocument> + '_ {
    docs.iter().filter(|doc| !doc.document.input.is_compressed).map(|doc| {
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
    let compressed_input = CompressedInput::new(&doc.input);
    let queries = doc
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

    model::Document {
        input: compressed_input.into_model(),
        queries,
    }
}

struct CompressedInput {
    json_string: JsonString,
    description: String,
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
    fn new(input: &Input) -> Self {
        Self {
            json_string: JsonString::new(&input.json),
            description: input.description.clone(),
        }
    }

    fn into_model(self) -> model::Input {
        let bytes: Vec<u8> = self.json_string.into();
        let json = String::from_utf8(bytes).expect("valid utf8 should be valid after compression");
        model::Input {
            description: self.description,
            is_compressed: true,
            json,
        }
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