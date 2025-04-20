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
    #[cfg(target_pointer_width = "64")]
    fn readme_query_64() -> Result<(), Box<dyn Error>> {
        assert_ron_snapshot!(&engine("$.jsonpath[*]")?);
        Ok(())
    }

    #[test]
    #[cfg(target_pointer_width = "32")]
    fn readme_query_32() -> Result<(), Box<dyn Error>> {
        assert_ron_snapshot!(&engine("$.jsonpath[*]")?);
        Ok(())
    }

    #[test]
    #[cfg(target_pointer_width = "64")]
    fn jsonpath_example_query_64() -> Result<(), Box<dyn Error>> {
        assert_ron_snapshot!(&engine("$..phoneNumbers[*].number")?);
        Ok(())
    }

    #[test]
    #[cfg(target_pointer_width = "32")]
    fn jsonpath_example_query_32() -> Result<(), Box<dyn Error>> {
        assert_ron_snapshot!(&engine("$..phoneNumbers[*].number")?);
        Ok(())
    }

    #[test]
    #[cfg(target_pointer_width = "64")]
    fn real_life_query_64() -> Result<(), Box<dyn Error>> {
        assert_ron_snapshot!(&engine("$.personal.details.contact.information.phones.home")?);
        Ok(())
    }

    #[test]
    #[cfg(target_pointer_width = "32")]
    fn real_life_query_32() -> Result<(), Box<dyn Error>> {
        assert_ron_snapshot!(&engine("$.personal.details.contact.information.phones.home")?);
        Ok(())
    }

    #[test]
    #[cfg(target_pointer_width = "64")]
    fn slice_64() -> Result<(), Box<dyn Error>> {
        assert_ron_snapshot!(&engine("$..entries[3:5:7]")?);
        Ok(())
    }

    #[test]
    #[cfg(target_pointer_width = "32")]
    fn slice_32() -> Result<(), Box<dyn Error>> {
        assert_ron_snapshot!(&engine("$..entries[3:5:7]")?);
        Ok(())
    }
}
