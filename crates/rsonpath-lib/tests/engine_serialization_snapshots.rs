#[cfg(feature = "serde")]
mod ron {
    use insta::assert_ron_snapshot;
    use rsonpath::engine::{Compiler, RsonpathEngine};
    use std::error::Error;

    fn engine(string: &str) -> Result<RsonpathEngine, Box<dyn Error>> {
        let query = rsonpath_syntax::parse(string)?;
        let engine = RsonpathEngine::compile_query(&query)?;
        Ok(engine)
    }

    #[test]
    fn empty_query() -> Result<(), Box<dyn Error>> {
        assert_ron_snapshot!(&engine("$")?);
        Ok(())
    }

    #[test]
    fn readme_query() -> Result<(), Box<dyn Error>> {
        assert_ron_snapshot!(&engine("$.jsonpath[*]")?);
        Ok(())
    }

    #[test]
    fn jsonpath_example_query() -> Result<(), Box<dyn Error>> {
        assert_ron_snapshot!(&engine("$..phoneNumbers[*].number")?);
        Ok(())
    }

    #[test]
    fn real_life_query() -> Result<(), Box<dyn Error>> {
        assert_ron_snapshot!(&engine("$.personal.details.contact.information.phones.home")?);
        Ok(())
    }

    #[test]
    fn slice() -> Result<(), Box<dyn Error>> {
        assert_ron_snapshot!(&engine("$..entries[3:5:7]")?);
        Ok(())
    }
}
