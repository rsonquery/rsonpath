use proptest::{arbitrary::Arbitrary, collection, prelude::*, prop_oneof, strategy::Strategy};
use rsonpath::query::{builder::JsonPathQueryBuilder, JsonPathQuery, JsonString};
use std::{cmp, fmt::Display, rc::Rc};

#[derive(Debug)]
pub(crate) struct Json(serde_json::Value);

impl From<serde_json::Value> for Json {
    fn from(value: serde_json::Value) -> Self {
        Json(value)
    }
}

#[derive(Debug)]
pub(crate) struct Query(pub(crate) JsonPathQuery);

#[derive(Debug)]
pub(crate) struct JsonAndQuery(pub(crate) Json, pub(crate) Query);

#[derive(Debug, Clone, Hash, PartialEq, Eq)]
pub(crate) struct JsonStringWrap(JsonString);

#[derive(Debug)]
struct JsonStringPool(Rc<Vec<JsonString>>);

impl Display for JsonAndQuery {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        writeln!(f, "{}", self.1)?;
        writeln!(f, "{}", self.0)
    }
}

impl From<Vec<JsonString>> for JsonStringPool {
    fn from(value: Vec<JsonString>) -> Self {
        Self(Rc::new(value))
    }
}

impl JsonStringPool {
    fn strategy(&self) -> impl Strategy<Value = JsonString> {
        let range = 0..self.0.len();
        let vec = self.0.clone();

        range.prop_map(move |i| vec[i].clone())
    }
}

#[derive(Debug, Clone, Copy)]
struct NonNegativeArrayIndex(rsonpath::query::NonNegativeArrayIndex);

impl From<JsonStringWrap> for JsonString {
    fn from(value: JsonStringWrap) -> Self {
        value.0
    }
}

impl From<NonNegativeArrayIndex> for rsonpath::query::NonNegativeArrayIndex {
    fn from(value: NonNegativeArrayIndex) -> Self {
        value.0
    }
}

impl From<rsonpath::query::JsonString> for JsonStringWrap {
    fn from(value: rsonpath::query::JsonString) -> Self {
        Self(value)
    }
}

impl From<rsonpath::query::NonNegativeArrayIndex> for NonNegativeArrayIndex {
    fn from(value: rsonpath::query::NonNegativeArrayIndex) -> Self {
        Self(value)
    }
}

pub(crate) struct JsonParameters {
    depth: u32,
    size: u32,
    branching: u32,
    string_strategy: BoxedStrategy<JsonString>,
}

impl Default for JsonParameters {
    fn default() -> Self {
        Self {
            depth: 32,
            size: 1024,
            branching: 64,
            string_strategy: json_string_arbitrary().boxed(),
        }
    }
}

pub(crate) struct QueryParameters {
    min_len: usize,
    max_len: usize,
    max_idx: u64,
    string_strategy: BoxedStrategy<JsonString>,
}

impl Default for QueryParameters {
    fn default() -> Self {
        Self {
            min_len: 0,
            max_len: 8,
            max_idx: 64,
            string_strategy: json_string_arbitrary().boxed(),
        }
    }
}

impl Arbitrary for Json {
    type Parameters = JsonParameters;

    fn arbitrary_with(args: Self::Parameters) -> Self::Strategy {
        #[derive(Debug, Clone)]
        enum RawJsonLeaf {
            Null,
            Bool(bool),
            Integer(i64),
            Float(f64),
            String(JsonString),
        }

        impl From<RawJsonLeaf> for serde_json::Value {
            fn from(value: RawJsonLeaf) -> Self {
                match value {
                    RawJsonLeaf::Null => serde_json::Value::Null,
                    RawJsonLeaf::Bool(b) => serde_json::Value::from(b),
                    RawJsonLeaf::Integer(i) => serde_json::Value::from(i),
                    RawJsonLeaf::Float(f) => serde_json::Value::from(f),
                    RawJsonLeaf::String(s) => serde_json::Value::String(s.to_string()),
                }
            }
        }

        impl Arbitrary for RawJsonLeaf {
            type Parameters = Option<BoxedStrategy<JsonString>>;

            fn arbitrary_with(args: Self::Parameters) -> Self::Strategy {
                let string_strategy = args.unwrap_or(json_string_arbitrary().boxed());
                prop_oneof![
                    Just(RawJsonLeaf::Null),
                    proptest::bool::ANY.prop_map(RawJsonLeaf::Bool),
                    proptest::num::i64::ANY.prop_map(RawJsonLeaf::Integer),
                    proptest::num::f64::ANY.prop_map(RawJsonLeaf::Float),
                    string_strategy.prop_map(RawJsonLeaf::String)
                ]
                .boxed()
            }

            type Strategy = BoxedStrategy<Self>;
        }

        let json_leaf = RawJsonLeaf::arbitrary().prop_map_into::<serde_json::Value>();

        let json_tree = json_leaf.prop_recursive(args.depth, args.size, args.branching, move |json| {
            prop_oneof![
                prop::collection::vec(json.clone(), 0..=(args.branching as usize)).prop_map_into::<serde_json::Value>(),
                prop::collection::hash_map(args.string_strategy.clone(), json, 0..=(args.branching as usize))
                    .prop_map(serde_json::Value::from_iter),
            ]
        });

        json_tree.prop_map_into().boxed()
    }

    type Strategy = BoxedStrategy<Json>;
}

impl Arbitrary for Query {
    type Parameters = QueryParameters;

    fn arbitrary_with(args: Self::Parameters) -> Self::Strategy {
        #[derive(Debug, Clone)]
        enum RawQueryNode {
            Child(JsonString),
            AnyChild,
            Descendant(JsonString),
            AnyDescendant,
            ArrayIndexChild(NonNegativeArrayIndex),
            ArrayIndexDescendant(NonNegativeArrayIndex),
        }

        impl Arbitrary for RawQueryNode {
            type Parameters = Option<(BoxedStrategy<JsonString>, u64)>;

            fn arbitrary_with(args: Self::Parameters) -> Self::Strategy {
                let (string_strategy, max_idx) = args.unwrap_or((json_string_arbitrary().boxed(), 128));
                let index_strategy = NonNegativeArrayIndex::arbitrary_with(Some((0, max_idx)));
                prop_oneof![
                    string_strategy.clone().prop_map(RawQueryNode::Child),
                    Just(RawQueryNode::AnyChild),
                    string_strategy.prop_map(RawQueryNode::Descendant),
                    Just(RawQueryNode::AnyDescendant),
                    index_strategy.clone().prop_map(RawQueryNode::ArrayIndexChild),
                    index_strategy.prop_map(RawQueryNode::ArrayIndexDescendant),
                ]
                .boxed()
            }

            type Strategy = BoxedStrategy<Self>;
        }

        let nodes = collection::vec(
            RawQueryNode::arbitrary_with(Some((args.string_strategy, args.max_idx))),
            args.min_len..args.max_len,
        )
        .prop_map(|x| {
            let mut builder = JsonPathQueryBuilder::new();

            for node in x {
                builder = match node {
                    RawQueryNode::Child(s) => builder.child(s.into()),
                    RawQueryNode::AnyChild => builder.any_child(),
                    RawQueryNode::Descendant(s) => builder.descendant(s.into()),
                    RawQueryNode::AnyDescendant => builder.any_descendant(),
                    RawQueryNode::ArrayIndexChild(i) => builder.array_index_child(i.into()),
                    RawQueryNode::ArrayIndexDescendant(i) => builder.array_index_descendant(i.into()),
                };
            }

            Query(builder.build())
        });

        nodes.boxed()
    }

    type Strategy = BoxedStrategy<Self>;
}

pub(crate) fn json_string_arbitrary() -> impl Strategy<Value = JsonString> {
    json_string_from_string(r"[^\u{00}-\u{1F}&&]*")
}

pub(crate) fn json_string_ascii() -> impl Strategy<Value = JsonString> {
    json_string_from_string(r"[a-zA-Z0-9_\-]")
}

fn json_string_from_string<S: Strategy<Value = String>>(base_strategy: S) -> impl Strategy<Value = JsonString> {
    base_strategy.prop_map(|x| {
        x.chars().fold(String::new(), |mut s, x| {
            match x {
                '\"' => s += r#"\""#,
                '\\' => s += r"\\",
                '\u{0}'..='\u{1F}' => s += &format!("\\u{:0>4X}", u32::from(x)),
                _ => s.push(x),
            };
            s
        });
        JsonString::new(&x)
    })
}

impl Arbitrary for NonNegativeArrayIndex {
    type Parameters = Option<(u64, u64)>;

    fn arbitrary_with(args: Self::Parameters) -> Self::Strategy {
        let (min, max) = args.unwrap_or((
            rsonpath::query::NonNegativeArrayIndex::ZERO.get_index(),
            rsonpath::query::NonNegativeArrayIndex::MAX.get_index(),
        ));

        low_skewed_range(min, max)
            .prop_map(|x| NonNegativeArrayIndex(x.try_into().unwrap()))
            .boxed()
    }

    type Strategy = BoxedStrategy<Self>;
}

fn low_skewed_range(min: u64, max: u64) -> impl Strategy<Value = u64> {
    (min..=max, min..=max).prop_map(|(x, y)| cmp::min(x, y))
}

pub(crate) fn json_and_query<S: Strategy<Value = JsonString>>(json_string: S) -> impl Strategy<Value = (Json, Query)> {
    collection::vec(json_string, 1..=128)
        .prop_map_into()
        .prop_flat_map(move |pool: JsonStringPool| {
            let json_parameters = JsonParameters {
                string_strategy: pool.strategy().boxed(),
                ..Default::default()
            };
            let query_parameters = QueryParameters {
                string_strategy: pool.strategy().boxed(),
                ..Default::default()
            };

            (
                Json::arbitrary_with(json_parameters),
                Query::arbitrary_with(query_parameters),
            )
        })
}

impl Display for Json {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl Display for Query {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

pub(crate) fn crawl(json: &Json) -> impl Iterator<Item = JsonValuePointer> {
    use serde_json::Value as JsonValue;

    let mut result = vec![];

    run(&mut result, &json.0, JsonPointer { segments: vec![] });

    return result.into_iter();

    fn run<'a>(result: &mut Vec<JsonValuePointer<'a>>, json: &'a JsonValue, ptr: JsonPointer) {
        let local_ptr = ptr.clone();
        result.push(JsonValuePointer {
            value: json,
            pointer: ptr,
        });

        match json {
            JsonValue::Array(list) => {
                for (i, val) in list.iter().enumerate() {
                    let mut local_ptr = local_ptr.clone();
                    local_ptr.segments.push(PointerSegment::ListItem(
                        rsonpath::query::NonNegativeArrayIndex::try_from(i as u64).unwrap(),
                    ));
                    run(result, val, local_ptr);
                }
            }
            JsonValue::Object(obj) => {
                for (key, val) in obj.iter() {
                    let mut local_ptr = local_ptr.clone();
                    local_ptr.segments.push(PointerSegment::Child(key.into()));
                    run(result, val, local_ptr);
                }
            }
            _ => (),
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) enum PointerSegment {
    Child(rsonpath::query::JsonString),
    ListItem(rsonpath::query::NonNegativeArrayIndex),
}

#[derive(Debug, Clone)]
pub(crate) struct JsonValuePointer<'a> {
    value: &'a serde_json::Value,
    pointer: JsonPointer,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) struct JsonPointer {
    segments: Vec<PointerSegment>,
}

impl JsonPointer {
    pub(crate) fn matches(&self, query: &JsonPathQuery) -> bool {
        use rsonpath::query::JsonPathQueryNode;

        let node = query.root().child();
        let segments = &self.segments;

        return try_match(node, segments);

        fn try_match(node: Option<&JsonPathQueryNode>, ptr: &[PointerSegment]) -> bool {
            match node {
                None => ptr.is_empty(),
                Some(node) => {
                    if ptr.is_empty() {
                        return false;
                    }

                    let segment = &ptr[0];
                    let ptr_rest = &ptr[1..];

                    match (node, segment) {
                        (JsonPathQueryNode::Child(s1, _), PointerSegment::Child(s2)) if s1 == s2 => {
                            try_match(node.child(), ptr_rest)
                        }
                        (JsonPathQueryNode::AnyChild(_), _) => try_match(node.child(), ptr_rest),
                        (JsonPathQueryNode::Descendant(s1, _), PointerSegment::Child(s2)) if s1 == s2 => {
                            try_match(node.child(), ptr_rest) || try_match(Some(node), ptr_rest)
                        }
                        (JsonPathQueryNode::Descendant(_, _), _) => try_match(Some(node), ptr_rest),
                        (JsonPathQueryNode::AnyDescendant(_), _) => {
                            try_match(node.child(), ptr_rest) || try_match(Some(node), ptr_rest)
                        }
                        (JsonPathQueryNode::ArrayIndexChild(i1, _), PointerSegment::ListItem(i2)) if i1 == i2 => {
                            try_match(node.child(), ptr_rest)
                        }
                        (JsonPathQueryNode::ArrayIndexDescendant(i1, _), PointerSegment::ListItem(i2)) if i1 == i2 => {
                            try_match(node.child(), ptr_rest) || try_match(Some(node), ptr_rest)
                        }
                        (JsonPathQueryNode::ArrayIndexDescendant(_, _), _) => try_match(Some(node), ptr_rest),
                        _ => false,
                    }
                }
            }
        }
    }
}

impl<'a> JsonValuePointer<'a> {
    pub(crate) fn value(&self) -> &serde_json::Value {
        self.value
    }

    pub(crate) fn matches(&self, query: &Query) -> bool {
        self.pointer.matches(&query.0)
    }
}

impl Display for JsonPointer {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "$")?;

        for seg in &self.segments {
            match seg {
                PointerSegment::Child(c) => write!(f, "['{c}']")?,
                PointerSegment::ListItem(i) => write!(f, "[{i}]")?,
            }
        }

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::Value as JsonValue;

    mod crawl {
        use super::*;

        #[test]
        fn null_label_null_value() {
            let root = JsonValue::from_iter([("\0", JsonValue::Null)]);
            let pointer1 = JsonPointer { segments: vec![] };
            let pointer2 = JsonPointer {
                segments: vec![PointerSegment::Child(JsonString::new("\0"))],
            };

            let json = Json(root.clone());
            let values: Vec<_> = crawl(&json).collect();

            assert_eq!(values.len(), 2);
            assert_eq!(values[0].pointer, pointer1);
            assert_eq!(values[1].pointer, pointer2);
            assert_eq!(values[0].value, &root);
            assert_eq!(values[1].value, &JsonValue::Null);
        }
    }

    mod matches {
        use super::*;

        #[test]
        fn empty_descendant() {
            let query = JsonPathQueryBuilder::new().descendant(JsonString::new("")).build();
            let pointer1 = JsonPointer { segments: vec![] };
            let pointer2 = JsonPointer {
                segments: vec![PointerSegment::Child(JsonString::new("\0"))],
            };
            let pointer3 = JsonPointer {
                segments: vec![PointerSegment::Child(JsonString::new(""))],
            };

            assert!(!pointer1.matches(&query));
            assert!(!pointer2.matches(&query));
            assert!(pointer3.matches(&query));
        }
    }
}
