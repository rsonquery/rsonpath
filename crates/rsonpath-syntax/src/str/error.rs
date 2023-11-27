use std::fmt::Display;

use thiserror::Error;

#[derive(Debug, Error)]
pub struct JsonStringParseError {}

impl Display for JsonStringParseError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        todo!()
    }
}
