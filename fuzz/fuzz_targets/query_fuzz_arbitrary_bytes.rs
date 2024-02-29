//! This is as pure as fuzzing goes - throw random UTF8 at the parser and make sure it doesn't panic.
#![no_main]

use libfuzzer_sys::fuzz_target;

fuzz_target!(|data: &str| {
    let _ = rsonpath_syntax::parse(data);
});
