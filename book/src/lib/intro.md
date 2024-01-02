# Introduction

_This part of the book is a work in progress._

```rust
# extern crate rsonpath;
# extern crate rsonpath_syntax;
use rsonpath::engine::{Compiler, Engine, RsonpathEngine};
use rsonpath::input::BorrowedBytes;
use rsonpath::result::count::CountRecorder;
# use std::error::Error;

# fn main() -> Result<(), Box<dyn Error>> {
// Parse a JSONPath query from string.
let query = rsonpath_syntax::parse("$..phoneNumbers[*].number")?;
// Convert the contents to the Input type required by the Engines.
let mut contents = r#"
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
}
"#;
let input = BorrowedBytes::new(contents.as_bytes());
// Compile the query. The engine can be reused to run the same query on different contents.
let engine = RsonpathEngine::compile_query(&query)?;
// Count the number of occurrences of elements satisfying the query.
let count = engine.count(&input)?;

assert_eq!(2, count);
# Ok(())
# }
```
