# Introduction

_This part of the book is a work in progress._

```rust
# extern crate rsonpath;
use rsonpath::engine::{Compiler, Engine, RsonpathEngine};
use rsonpath::input::BorrowedBytes;
use rsonpath::query::JsonPathQuery;

# fn main() -> Result<(), Box<dyn std::error::Error>> {
let query = JsonPathQuery::parse("$..phoneNumbers[*].number")?;

let contents = r#"
{
  "person": {
    "name": "John",
    "surname": "Doe",
    "phoneNumbers": [
      {
        "type": "Home",
        "number": "111-222-333"
      },
      {
        "type": "Work",
        "number": "123-456-789"
      }
    ]
  }
}"#;

let input = BorrowedBytes::new(contents.as_bytes());
let engine = RsonpathEngine::compile_query(&query)?;
let count = engine.count(&input)?;

assert_eq!(2, count);
# Ok(())
# }
```
