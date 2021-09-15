use super::result::*;

pub trait Runner {
    fn count(&self, input: &str) -> CountResult;
}
