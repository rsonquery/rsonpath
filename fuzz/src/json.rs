use arbitrary::{Arbitrary, Unstructured};
use std::collections::HashMap;
use std::fmt::Display;

#[derive(Debug)]
pub struct ArbitraryJson<const MAX_SIZE: usize>(serde_json::Value);

impl<const MAX_SIZE: usize> Display for ArbitraryJson<MAX_SIZE> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl<'a, const MAX_SIZE: usize> Arbitrary<'a> for ArbitraryJson<MAX_SIZE> {
    fn arbitrary(u: &mut Unstructured<'a>) -> arbitrary::Result<Self> {
        enum RawValue {
            Leaf(LeafValue),
            Nested(NestedValue),
        }

        #[derive(Arbitrary)]
        enum LeafValue {
            Null,
            Bool(bool),
            Integer(i64),
            Float(f64),
            String(String),
        }

        enum NestedValue {
            Array(Vec<RawValue>),
            Object(HashMap<String, RawValue>),
        }

        impl From<RawValue> for serde_json::Value {
            fn from(value: RawValue) -> Self {
                match value {
                    RawValue::Leaf(LeafValue::Null) => serde_json::Value::Null,
                    RawValue::Leaf(LeafValue::Bool(b)) => serde_json::Value::Bool(b),
                    RawValue::Leaf(LeafValue::Integer(n)) => serde_json::Value::from(n),
                    RawValue::Leaf(LeafValue::Float(f)) => serde_json::Value::from(f),
                    RawValue::Leaf(LeafValue::String(s)) => serde_json::Value::String(s),
                    RawValue::Nested(NestedValue::Array(arr)) => {
                        serde_json::Value::Array(arr.into_iter().map(|x| x.into()).collect())
                    }
                    RawValue::Nested(NestedValue::Object(obj)) => {
                        serde_json::Value::Object(obj.into_iter().map(|x| (x.0, x.1.into())).collect())
                    }
                }
            }
        }

        fn generate_json_with_size(u: &mut Unstructured, size: usize) -> arbitrary::Result<RawValue> {
            if size == 1 && u.arbitrary::<bool>()? {
                Ok(RawValue::Leaf(u.arbitrary::<LeafValue>()?))
            } else {
                let mut rem_size = size - 1;
                if u.arbitrary::<bool>()? {
                    // Array.
                    let mut nested_values = vec![];
                    while rem_size > 0 {
                        let nested_size = u.int_in_range(1..=rem_size)?;
                        rem_size -= nested_size;
                        nested_values.push(generate_json_with_size(u, nested_size)?);
                    }
                    Ok(RawValue::Nested(NestedValue::Array(nested_values)))
                } else {
                    // Object.
                    // We generate arbitrary labels and values.
                    // We can't guarantee Unstructured won't start returning the same repeat label at some point.
                    // In that case, we most likely ran out of bytes in the source. We apply a "good-enough-effort"
                    // strategy - append the index number to the end of the label, insert it, if it overwrites something
                    // then too bad.
                    let mut object = HashMap::new();
                    let mut i = 0;
                    while rem_size > 0 {
                        let nested_size = u.int_in_range(1..=rem_size)?;
                        rem_size -= nested_size;
                        let value = generate_json_with_size(u, nested_size)?;
                        let mut key = u.arbitrary::<String>()?;
                        if object.contains_key(&key) {
                            key += &i.to_string();
                        }
                        object.insert(key, value);
                        i += 1;
                    }
                    Ok(RawValue::Nested(NestedValue::Object(object)))
                }
            }
        }

        let size = u.int_in_range(1..=MAX_SIZE)?;
        Ok(ArbitraryJson(generate_json_with_size(u, size)?.into()))
    }
}
