#![no_main]

use libfuzzer_sys::{arbitrary::Arbitrary, fuzz_target, Corpus};
use rsonpath::input::BorrowedBytes;
use rsonpath::{
    engine::{Compiler, Engine, RsonpathEngine},
    query::error::CompilerError,
};
use rsonpath_syntax::JsonPathQuery;
use std::{
    collections::HashMap,
    fmt::{Debug, Display},
};

#[derive(Debug, Arbitrary)]
struct FuzzData {
    query: JsonPathQuery,
    json: Json,
}

fuzz_target!(|data: FuzzData| -> Corpus {
    let json_string = data.json.to_string();
    let bytes = BorrowedBytes::new(json_string.as_bytes());
    let engine = match RsonpathEngine::compile_query(&data.query) {
        Ok(x) => x,
        Err(CompilerError::QueryTooComplex(_)) => return Corpus::Reject,
        Err(err) => panic!("error compiling query: {err}"),
    };
    let mut sink = vec![];

    let _ = engine.matches(&bytes, &mut sink);

    Corpus::Keep
});

#[derive(Debug)]
struct Json(serde_json::Value);

impl Display for Json {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl<'a> Arbitrary<'a> for Json {
    fn arbitrary(u: &mut arbitrary::Unstructured<'a>) -> arbitrary::Result<Self> {
        #[derive(Arbitrary)]
        enum RawValue {
            Null,
            Bool(bool),
            Integer(i64),
            Float(f64),
            String(String),
            Array(Vec<RawValue>),
            Object(HashMap<String, RawValue>),
        }

        impl From<RawValue> for serde_json::Value {
            fn from(value: RawValue) -> Self {
                match value {
                    RawValue::Null => serde_json::Value::Null,
                    RawValue::Bool(b) => serde_json::Value::Bool(b),
                    RawValue::Integer(n) => serde_json::Value::from(n),
                    RawValue::Float(f) => serde_json::Value::from(f),
                    RawValue::String(s) => serde_json::Value::String(s),
                    RawValue::Array(arr) => serde_json::Value::Array(arr.into_iter().map(|x| x.into()).collect()),
                    RawValue::Object(obj) => {
                        serde_json::Value::Object(obj.into_iter().map(|x| (x.0, x.1.into())).collect())
                    }
                }
            }
        }

        Ok(Json(u.arbitrary::<RawValue>()?.into()))
    }
}
