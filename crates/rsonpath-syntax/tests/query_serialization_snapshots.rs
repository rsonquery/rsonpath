#[cfg(feature = "serde")]
mod ron {
    use insta::assert_ron_snapshot;
    use rsonpath_syntax::parse;
    use std::error::Error;

    #[test]
    fn empty_query() -> Result<(), Box<dyn Error>> {
        assert_ron_snapshot!(&parse("$")?);
        Ok(())
    }

    #[test]
    fn readme_query() -> Result<(), Box<dyn Error>> {
        assert_ron_snapshot!(&parse("$.jsonpath[*]")?);
        Ok(())
    }

    #[test]
    fn jsonpath_example_query() -> Result<(), Box<dyn Error>> {
        assert_ron_snapshot!(&parse("$..phoneNumbers[*].number")?);
        Ok(())
    }

    #[test]
    fn real_life_query() -> Result<(), Box<dyn Error>> {
        assert_ron_snapshot!(&parse("$.personal.details.contact.information.phones.home")?);
        Ok(())
    }

    #[test]
    fn slice() -> Result<(), Box<dyn Error>> {
        assert_ron_snapshot!(&parse("$..entries[3:5:7]")?);
        Ok(())
    }

    #[test]
    fn multiple_selectors() -> Result<(), Box<dyn Error>> {
        assert_ron_snapshot!(&parse("$..entries['abc', 4, 7:10:13]")?);
        Ok(())
    }

    #[test]
    fn filter() -> Result<(), Box<dyn Error>> {
        assert_ron_snapshot!(&parse("$..user[?@.id == 'value']..entities..url")?);
        Ok(())
    }

    #[test]
    fn nested_filter() -> Result<(), Box<dyn Error>> {
        assert_ron_snapshot!(&parse(r#"$.a[?@.b == "abc" && $[*][?@.c > 3.13]]"#)?);
        Ok(())
    }
}
