//! Public reexports of the JSONPath query object model, parsing and building
//! entrypoints.

pub use crate::{
    builder::{JsonPathQueryBuilder, SliceBuilder},
    error::ParseError,
    num::{JsonFloat, JsonInt, JsonNumber, JsonUInt},
    parse as parse_json_path_query,
    str::JsonString,
    Comparable, ComparisonExpr, ComparisonOp, Index, JsonPathQuery, Literal, LogicalExpr, Parser as JsonPathParser,
    ParserBuilder as JsonPathParserBuilder, Segment, Selector, Selectors, SingularJsonPathQuery, SingularSegment,
    Slice, Step, TestExpr,
};
