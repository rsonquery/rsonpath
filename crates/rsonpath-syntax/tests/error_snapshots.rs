use insta::assert_snapshot;
use rsonpath_syntax::parse;

#[test]
fn empty_string_is_invalid() {
    let src = "";
    let result = parse(src).expect_err("should fail to parse");
    assert_snapshot!(result, @"
    error: query not starting with the root identifier '$'

      
      ^ the '$' character missing before here
      (byte 0)


    suggestion: did you mean `$` ?
    ");
}

/// This is a regression test. There was a bug where the error handling loop would try to resume
/// parsing at the next byte after an invalid character, which is invalid and causes a panic
/// if the character takes more than one byte - strings can be indexed only at char boundaries.
#[test]
fn error_handling_across_unicode_values() {
    // Ferris has 4 bytes of encoding.
    let input = "🦀.";
    let result = parse(input).expect_err("should fail to parse");

    assert_snapshot!(result, @"
    error: query not starting with the root identifier '$'

      🦀.
      ^^ the '$' character missing before here
      (bytes 0-3)


    error: invalid segment syntax

      🦀.
      ^^ not a valid segment syntax
      (bytes 0-3)

    note: valid segments are: member name shorthands like `.name`/`..name`; or child/descendant bracketed selections like `[<segments>]`/`..[<segments>]`
    error: invalid selector - empty

      🦀.
        ^^ expected a selector here, but found nothing
      (bytes 4-5)

    note: if you meant to match any value, you should use the wildcard selector `*`
    ");
}

/// This is an important user comfort feature. People new to JSONPath commonly mix single dots accesses with brackets,
/// i.e. instead of `$.a` or `$['a']` they use `$.['a']`. This can also be mixed with the missing-quotes error,
/// as in `$.[a]`. These tests make sure we emit accurate suggestions as to how rewrite the query to be correct.
mod invalid_short_name_syntax_suggestions {
    use insta::assert_snapshot;
    use rsonpath_syntax::parse;

    #[test]
    fn just_brackets() {
        let input = "$.[a]";
        let result = parse(input).expect_err("should fail to parse");

        assert_snapshot!(result, @"
        error: invalid short member name syntax

          $.[a]
            ^^^ not a valid name shorthand
          (bytes 2-4)


        suggestion: did you mean `$['a']` ?
        ");
    }

    #[test]
    fn brackets_and_single_quotes() {
        let input = "$.['a']";
        let result = parse(input).expect_err("should fail to parse");

        assert_snapshot!(result, @"
        error: invalid short member name syntax

          $.['a']
            ^^^^^ not a valid name shorthand
          (bytes 2-6)


        suggestion: did you mean `$['a']` ?
        ");
    }

    #[test]
    fn brackets_and_double_quotes() {
        let input = "$.[\"a\"]";
        let result = parse(input).expect_err("should fail to parse");

        assert_snapshot!(result, @r#"
        error: invalid short member name syntax

          $.["a"]
            ^^^^^ not a valid name shorthand
          (bytes 2-6)


        suggestion: did you mean `$["a"]` ?
        "#);
    }

    #[test]
    fn just_brackets_but_cannot_use_single_quotes() {
        let input = "$.[a'b]";
        let result = parse(input).expect_err("should fail to parse");

        assert_snapshot!(result, @r#"
        error: invalid short member name syntax

          $.[a'b]
            ^^^^^ not a valid name shorthand
          (bytes 2-6)


        suggestion: did you mean `$["a'b"]` ?
        "#);
    }

    #[test]
    fn just_brackets_but_cannot_use_double_quotes() {
        let input = "$.[a\"b]";
        let result = parse(input).expect_err("should fail to parse");

        assert_snapshot!(result, @r#"
        error: invalid short member name syntax

          $.[a"b]
            ^^^^^ not a valid name shorthand
          (bytes 2-6)


        suggestion: did you mean `$['a"b']` ?
        "#);
    }

    #[test]
    fn just_brackets_but_cannot_either_quotes_and_needs_escaping() {
        let input = "$.[a'\"b]";
        let result = parse(input).expect_err("should fail to parse");

        assert_snapshot!(result, @r#"
        error: invalid short member name syntax

          $.[a'"b]
            ^^^^^^ not a valid name shorthand
          (bytes 2-7)


        suggestion: did you mean `$['a\'"b']` ?
        "#);
    }
}

/// Suggestions for missing quotes, for example `$[a]` should suggest `$['a']`.
/// This needs to handle multiple selectors as well as quote escaping.
mod missing_quotes_suggestions {
    use insta::assert_snapshot;
    use rsonpath_syntax::parse;

    #[test]
    fn simple() {
        let input = "$[a]";
        let result = parse(input).expect_err("should fail to parse");
        assert_snapshot!(result, @"
        error: invalid selector syntax

          $[a]
            ^ not a valid selector
          (byte 2)


        suggestion: did you mean `$['a']` ?
        ");
    }

    #[test]
    fn contains_a_single_quote() {
        let input = "$[a'b]";
        let result = parse(input).expect_err("should fail to parse");
        assert_snapshot!(result, @r#"
        error: invalid selector syntax

          $[a'b]
            ^^^ not a valid selector
          (bytes 2-4)


        suggestion: did you mean `$["a'b"]` ?
        "#);
    }

    #[test]
    fn contains_a_double_quote() {
        let input = "$[a\"b]";
        let result = parse(input).expect_err("should fail to parse");
        assert_snapshot!(result, @r#"
        error: invalid selector syntax

          $[a"b]
            ^^^ not a valid selector
          (bytes 2-4)


        suggestion: did you mean `$['a"b']` ?
        "#);
    }

    #[test]
    fn contains_mixed_quote() {
        let input = "$[a'\"b]";
        let result = parse(input).expect_err("should fail to parse");
        assert_snapshot!(result, @r#"
        error: invalid selector syntax

          $[a'"b]
            ^^^^ not a valid selector
          (bytes 2-5)


        suggestion: did you mean `$['a\'"b']` ?
        "#);
    }

    #[test]
    fn multiple_broken_selectors() {
        let input = "$[a, b]";
        let result = parse(input).expect_err("should fail to parse");
        assert_snapshot!(result, @"
        error: invalid selector syntax

          $[a, b]
            ^ not a valid selector
          (byte 2)


        error: invalid selector syntax

          $[a, b]
               ^ not a valid selector
          (byte 5)


        suggestion: did you mean `$['a', 'b']` ?
        ");
    }

    #[test]
    fn multiple_broken_selectors_but_some_are_not_broken() {
        let input = "$['a', b, c, 'd', e]";
        let result = parse(input).expect_err("should fail to parse");
        assert_snapshot!(result, @"
        error: invalid selector syntax

          $['a', b, c, 'd', e]
                 ^ not a valid selector
          (byte 7)


        error: invalid selector syntax

          $['a', b, c, 'd', e]
                    ^ not a valid selector
          (byte 10)


        error: invalid selector syntax

          $['a', b, c, 'd', e]
                            ^ not a valid selector
          (byte 18)


        suggestion: did you mean `$['a', 'b', 'c', 'd', 'e']` ?
        ");
    }
}

mod name_selector {
    use insta::assert_snapshot;
    use rsonpath_syntax::{parse, JsonPathQuery, Result};

    fn parse_single_quoted_name_selector(src: &str) -> Result<JsonPathQuery> {
        let query_string = format!("$['{src}']");
        parse(&query_string)
    }

    #[test]
    fn null_byte_must_be_escaped() {
        let src = "\u{0000}";
        let result = parse_single_quoted_name_selector(src).expect_err("should fail to parse");
        assert_snapshot!(result, @"error: invalid unescaped control character\n\n  $['\0']\n  (byte 3)\n\n\nsuggestion: did you mean `$['\\u0000']` ?");
    }

    #[test]
    fn u0019_control_must_be_escaped() {
        let src = "\u{0019}";
        let result = parse_single_quoted_name_selector(src).expect_err("should fail to parse");
        assert_snapshot!(result, @"error: invalid unescaped control character\n\n  $['\u{19}']\n  (byte 3)\n\n\nsuggestion: did you mean `$['\\u0019']` ?");
    }

    #[test]
    fn single_quote_in_single_quoted_string_must_be_escaped() {
        let src = "unescaped ' quote";
        let result = parse_single_quoted_name_selector(src).expect_err("should fail to parse");
        assert_snapshot!(result, @"
        error: selectors not separated with commas

          $['unescaped ' quote']
                         ^ expected a comma separator before this character
          (byte 15)


        error: invalid selector syntax

          $['unescaped ' quote']
                         ^^^^^^ not a valid selector
          (bytes 15-20)
        ");
    }

    #[test]
    fn double_quote_in_double_quoted_string_must_be_escaped() {
        let src = r#"unescaped " quote"#;
        let query_string = format!(r#"$["{src}"]"#);
        let err = parse(&query_string).expect_err("should fail to parse");
        assert_snapshot!(err, @r#"
        error: selectors not separated with commas

          $["unescaped " quote"]
                         ^ expected a comma separator before this character
          (byte 15)


        error: invalid selector syntax

          $["unescaped " quote"]
                         ^^^^^^ not a valid selector
          (bytes 15-20)
        "#);
    }

    #[test]
    fn u0020_space_must_not_be_escaped() {
        let src = r"escape \ a space";
        let result = parse_single_quoted_name_selector(src).expect_err("should fail to parse");
        assert_snapshot!(result, @r#"
        error: invalid escape sequence

          $['escape \ a space']
                    ^^ not a valid escape sequence
          (bytes 10-11)

        note: the only valid escape sequences are \n, \r, \t, \f, \b, \\, \/, \' (in single quoted names), \" (in double quoted names), and \uXXXX where X are hex digits
        note: if you meant to match a literal backslash, you need to escape it with \\
        suggestion: did you mean `$['escape \\ a space']` ?
        "#);
    }

    #[test]
    fn backslash_must_be_escaped() {
        let src = r"\";
        let result = parse_single_quoted_name_selector(src).expect_err("should fail to parse");
        assert_snapshot!(result, @r"
        error: single-quoted name selector is not closed

          $['\']
                ^ expected a single quote `'`
          (byte 6)


        error: bracketed selection is not closed

          $['\']
                ^ expected a closing bracket ']'
          (byte 6)


        suggestion: did you mean `$['\']']` ?
        ");
    }

    #[test]
    fn unicode_escape_must_be_lowercase() {
        let src = r"\U0012";
        let result = parse_single_quoted_name_selector(src).expect_err("should fail to parse");
        assert_snapshot!(result, @r"
        error: invalid escape sequence

          $['\U0012']
             ^^ not a valid escape sequence
          (bytes 3-4)

        note: unicode escape sequences must use a lowercase 'u'
        suggestion: did you mean `$['\u0012']` ?
        ");
    }

    #[test]
    fn high_surrogate_must_be_paired() {
        let src = r"escape \uD800 and that is it";
        let result = parse_single_quoted_name_selector(src).expect_err("should fail to parse");
        assert_snapshot!(result, @r"
        error: invalid unicode escape sequence - unpaired high surrogate

          $['escape \uD800 and that is it']
                    ^^^^^^ this high surrogate is unpaired
          (bytes 10-15)

        note: a UTF-16 high surrogate has to be followed by a low surrogate to encode a valid Unicode character
        note: for more information about UTF-16 surrogate pairs see https://en.wikipedia.org/wiki/UTF-16#Code_points_from_U+010000_to_U+10FFFF
        ");
    }

    #[test]
    fn high_surrogate_must_be_paired_with_a_low_surrogate_and_not_high() {
        let src = r"escape \uD800\uD801 and that is it";
        let result = parse_single_quoted_name_selector(src).expect_err("should fail to parse");
        assert_snapshot!(result, @r"
        error: invalid unicode escape sequence - unpaired high surrogate

          $['escape \uD800\uD801 and that is it']
                    ^^^^^^ this high surrogate is unpaired
          (bytes 10-15)

        note: a UTF-16 high surrogate has to be followed by a low surrogate to encode a valid Unicode character
        note: for more information about UTF-16 surrogate pairs see https://en.wikipedia.org/wiki/UTF-16#Code_points_from_U+010000_to_U+10FFFF
        ");
    }

    #[test]
    fn high_surrogate_must_be_paired_with_a_low_surrogate_and_not_newline() {
        let src = r"escape \uD800\n and that is it";
        let result = parse_single_quoted_name_selector(src).expect_err("should fail to parse");
        assert_snapshot!(result, @r"
        error: invalid unicode escape sequence - unpaired high surrogate

          $['escape \uD800\n and that is it']
                    ^^^^^^ this high surrogate is unpaired
          (bytes 10-15)

        note: a UTF-16 high surrogate has to be followed by a low surrogate to encode a valid Unicode character
        note: for more information about UTF-16 surrogate pairs see https://en.wikipedia.org/wiki/UTF-16#Code_points_from_U+010000_to_U+10FFFF
        ");
    }

    #[test]
    fn high_surrogate_must_be_paired_with_a_low_surrogate_and_not_regular_unicode_escape() {
        let src = r"escape \uD800\uCC01 and that is it";
        let result = parse_single_quoted_name_selector(src).expect_err("should fail to parse");
        assert_snapshot!(result, @r"
        error: invalid unicode escape sequence - unpaired high surrogate

          $['escape \uD800\uCC01 and that is it']
                    ^^^^^^ this high surrogate is unpaired
          (bytes 10-15)

        note: a UTF-16 high surrogate has to be followed by a low surrogate to encode a valid Unicode character
        note: for more information about UTF-16 surrogate pairs see https://en.wikipedia.org/wiki/UTF-16#Code_points_from_U+010000_to_U+10FFFF
        ");
    }

    #[test]
    fn low_surrogate_must_be_paired() {
        let src = r"escape \uDC01 and that is it";
        let result = parse_single_quoted_name_selector(src).expect_err("should fail to parse");
        assert_snapshot!(result, @r"
        error: invalid unicode escape sequence - unpaired low surrogate

          $['escape \uDC01 and that is it']
                    ^^^^^^ this low surrogate is unpaired
          (bytes 10-15)

        note: a UTF-16 low surrogate has to be preceded by a high surrogate to encode a valid Unicode character
        note: for more information about UTF-16 surrogate pairs see https://en.wikipedia.org/wiki/UTF-16#Code_points_from_U+010000_to_U+10FFFF
        ");
    }

    #[test]
    fn low_surrogate_must_be_paired_with_a_high_surrogate_and_not_low() {
        let src = r"escape \uDC01\uDC02 and that is it";
        let result = parse_single_quoted_name_selector(src).expect_err("should fail to parse");
        assert_snapshot!(result, @r"
        error: invalid unicode escape sequence - unpaired low surrogate

          $['escape \uDC01\uDC02 and that is it']
                    ^^^^^^ this low surrogate is unpaired
          (bytes 10-15)

        note: a UTF-16 low surrogate has to be preceded by a high surrogate to encode a valid Unicode character
        note: for more information about UTF-16 surrogate pairs see https://en.wikipedia.org/wiki/UTF-16#Code_points_from_U+010000_to_U+10FFFF
        error: invalid unicode escape sequence - unpaired low surrogate

          $['escape \uDC01\uDC02 and that is it']
                          ^^^^^^ this low surrogate is unpaired
          (bytes 16-21)

        note: a UTF-16 low surrogate has to be preceded by a high surrogate to encode a valid Unicode character
        note: for more information about UTF-16 surrogate pairs see https://en.wikipedia.org/wiki/UTF-16#Code_points_from_U+010000_to_U+10FFFF
        ");
    }

    #[test]
    fn low_surrogate_must_be_paired_with_a_high_surrogate_and_not_newline() {
        let src = r"escape \n\uDC01\n and that is it";
        let result = parse_single_quoted_name_selector(src).expect_err("should fail to parse");
        assert_snapshot!(result, @r"
        error: invalid unicode escape sequence - unpaired low surrogate

          $['escape \n\uDC01\n and that is it']
                      ^^^^^^ this low surrogate is unpaired
          (bytes 12-17)

        note: a UTF-16 low surrogate has to be preceded by a high surrogate to encode a valid Unicode character
        note: for more information about UTF-16 surrogate pairs see https://en.wikipedia.org/wiki/UTF-16#Code_points_from_U+010000_to_U+10FFFF
        ");
    }

    #[test]
    fn low_surrogate_must_be_paired_with_a_high_surrogate_and_not_regular_unicode_escape() {
        let src = r"escape \uCC01\uDC01\uCC01 and that is it";
        let result = parse_single_quoted_name_selector(src).expect_err("should fail to parse");
        assert_snapshot!(result, @r"
        error: invalid unicode escape sequence - unpaired low surrogate

          $['escape \uCC01\uDC01\uCC01 and that is it']
                          ^^^^^^ this low surrogate is unpaired
          (bytes 16-21)

        note: a UTF-16 low surrogate has to be preceded by a high surrogate to encode a valid Unicode character
        note: for more information about UTF-16 surrogate pairs see https://en.wikipedia.org/wiki/UTF-16#Code_points_from_U+010000_to_U+10FFFF
        ");
    }

    #[test]
    fn backslash_u_alone_is_not_valid() {
        let src = r"\u";
        let result = parse_single_quoted_name_selector(src).expect_err("should fail to parse");
        assert_snapshot!(result, @r"
        error: invalid unicode escape sequence - invalid hex digit

          $['\u']
               ^ not a hex digit
          (byte 5)

        note: valid hex digits are 0 through 9 and A through F (case-insensitive)
        ");
    }

    #[test]
    fn unicode_escape_must_have_four_hex_digits_not_zero() {
        let src = r"escape \u and that is it";
        let result = parse_single_quoted_name_selector(src).expect_err("should fail to parse");
        assert_snapshot!(result, @r"
        error: invalid unicode escape sequence - invalid hex digit

          $['escape \u and that is it']
                      ^ not a hex digit
          (byte 12)

        note: valid hex digits are 0 through 9 and A through F (case-insensitive)
        ");
    }

    #[test]
    fn unicode_escape_must_have_four_hex_digits_not_one() {
        let src = r"escape \u1 and that is it";
        let result = parse_single_quoted_name_selector(src).expect_err("should fail to parse");
        assert_snapshot!(result, @r"
        error: invalid unicode escape sequence - invalid hex digit

          $['escape \u1 and that is it']
                       ^ not a hex digit
          (byte 13)

        note: valid hex digits are 0 through 9 and A through F (case-insensitive)
        ");
    }

    #[test]
    fn unicode_escape_must_have_four_hex_digits_not_two() {
        let src = r"escape \u12 and that is it";
        let result = parse_single_quoted_name_selector(src).expect_err("should fail to parse");
        assert_snapshot!(result, @r"
        error: invalid unicode escape sequence - invalid hex digit

          $['escape \u12 and that is it']
                        ^ not a hex digit
          (byte 14)

        note: valid hex digits are 0 through 9 and A through F (case-insensitive)
        ");
    }

    #[test]
    fn unicode_escape_must_have_four_hex_digits_not_three() {
        let src = r"escape \u123 and that is it";
        let result = parse_single_quoted_name_selector(src).expect_err("should fail to parse");
        assert_snapshot!(result, @r"
        error: invalid unicode escape sequence - invalid hex digit

          $['escape \u123 and that is it']
                         ^ not a hex digit
          (byte 15)

        note: valid hex digits are 0 through 9 and A through F (case-insensitive)
        ");
    }

    #[test]
    fn g_is_not_a_valid_first_hex_digit() {
        let src = r"escape \uG234 and that is it";
        let result = parse_single_quoted_name_selector(src).expect_err("should fail to parse");
        assert_snapshot!(result, @r"
        error: invalid unicode escape sequence - invalid hex digit

          $['escape \uG234 and that is it']
                      ^ not a hex digit
          (byte 12)

        note: valid hex digits are 0 through 9 and A through F (case-insensitive)
        ");
    }

    #[test]
    fn g_is_not_a_valid_second_hex_digit() {
        let src = r"escape \u1G34 and that is it";
        let result = parse_single_quoted_name_selector(src).expect_err("should fail to parse");
        assert_snapshot!(result, @r"
        error: invalid unicode escape sequence - invalid hex digit

          $['escape \u1G34 and that is it']
                       ^ not a hex digit
          (byte 13)

        note: valid hex digits are 0 through 9 and A through F (case-insensitive)
        ");
    }

    #[test]
    fn g_is_not_a_valid_third_hex_digit() {
        let src = r"escape \u12G4 and that is it";
        let result = parse_single_quoted_name_selector(src).expect_err("should fail to parse");
        assert_snapshot!(result, @r"
        error: invalid unicode escape sequence - invalid hex digit

          $['escape \u12G4 and that is it']
                        ^ not a hex digit
          (byte 14)

        note: valid hex digits are 0 through 9 and A through F (case-insensitive)
        ");
    }

    #[test]
    fn g_is_not_a_valid_fourth_hex_digit() {
        let src = r"escape \u123G and that is it";
        let result = parse_single_quoted_name_selector(src).expect_err("should fail to parse");
        assert_snapshot!(result, @r"
        error: invalid unicode escape sequence - invalid hex digit

          $['escape \u123G and that is it']
                         ^ not a hex digit
          (byte 15)

        note: valid hex digits are 0 through 9 and A through F (case-insensitive)
        ");
    }

    #[test]
    fn high_surrogate_must_be_paired_with_a_low_surrogate_not_zero_digit_unicode_escape() {
        let src = r"escape \uD800\u and that is it";
        let result = parse_single_quoted_name_selector(src).expect_err("should fail to parse");
        assert_snapshot!(result, @r"
        error: invalid unicode escape sequence - invalid hex digit

          $['escape \uD800\u and that is it']
                            ^ not a hex digit
          (byte 18)

        note: valid hex digits are 0 through 9 and A through F (case-insensitive)
        ");
    }

    #[test]
    fn high_surrogate_must_be_paired_with_a_low_surrogate_not_one_digit_unicode_escape() {
        let src = r"escape \uD800\uD and that is it";
        let result = parse_single_quoted_name_selector(src).expect_err("should fail to parse");
        assert_snapshot!(result, @r"
        error: invalid unicode escape sequence - invalid hex digit

          $['escape \uD800\uD and that is it']
                             ^ not a hex digit
          (byte 19)

        note: valid hex digits are 0 through 9 and A through F (case-insensitive)
        ");
    }

    #[test]
    fn high_surrogate_must_be_paired_with_a_low_surrogate_not_two_digit_unicode_escape() {
        let src = r"escape \uD800\uDC and that is it";
        let result = parse_single_quoted_name_selector(src).expect_err("should fail to parse");
        assert_snapshot!(result, @r"
        error: invalid unicode escape sequence - invalid hex digit

          $['escape \uD800\uDC and that is it']
                              ^ not a hex digit
          (byte 20)

        note: valid hex digits are 0 through 9 and A through F (case-insensitive)
        ");
    }

    #[test]
    fn high_surrogate_must_be_paired_with_a_low_surrogate_not_three_digit_unicode_escape() {
        let src = r"escape \uD800\uDC0 and that is it";
        let result = parse_single_quoted_name_selector(src).expect_err("should fail to parse");
        assert_snapshot!(result, @r"
        error: invalid unicode escape sequence - invalid hex digit

          $['escape \uD800\uDC0 and that is it']
                               ^ not a hex digit
          (byte 21)

        note: valid hex digits are 0 through 9 and A through F (case-insensitive)
        ");
    }

    #[test]
    fn high_surrogate_must_be_paired_with_a_low_surrogate_not_invalid_unicode_escape() {
        let src = r"escape \uD800\uDC0X and that is it";
        let result = parse_single_quoted_name_selector(src).expect_err("should fail to parse");
        assert_snapshot!(result, @r"
        error: invalid unicode escape sequence - invalid hex digit

          $['escape \uD800\uDC0X and that is it']
                               ^ not a hex digit
          (byte 21)

        note: valid hex digits are 0 through 9 and A through F (case-insensitive)
        ");
    }

    #[test]
    fn error_following_wide_letters_should_be_properly_highlighted() {
        let src = r"Ｈｅｌｌｏ, ｗｏｒｌｄ!\u222X";
        let result = parse_single_quoted_name_selector(src).expect_err("should fail to parse");
        assert_snapshot!(result, @r"
        error: invalid unicode escape sequence - invalid hex digit

          $['Ｈｅｌｌｏ, ｗｏｒｌｄ!\u222X']
                                         ^ not a hex digit
          (byte 41)

        note: valid hex digits are 0 through 9 and A through F (case-insensitive)
        ");
    }

    #[test]
    fn error_following_multibyte_grapheme_cluster_should_be_properly_highlighted() {
        let src = r"क्\u12G4";
        let result = parse_single_quoted_name_selector(src).expect_err("should fail to parse");
        assert_snapshot!(result, @r"
        error: invalid unicode escape sequence - invalid hex digit

          $['क्\u12G4']
                  ^ not a hex digit
          (byte 13)

        note: valid hex digits are 0 through 9 and A through F (case-insensitive)
        ");
    }

    #[test]
    fn error_following_ligature_emoji_should_be_properly_highlighted() {
        let src = r"👩‍🔬\u222X";
        let result = parse_single_quoted_name_selector(src).expect_err("should fail to parse");
        assert_snapshot!(result, @r"
        error: invalid unicode escape sequence - invalid hex digit

          $['👩‍🔬\u222X']
                      ^ not a hex digit
          (byte 19)

        note: valid hex digits are 0 through 9 and A through F (case-insensitive)
        ");
    }
}

/// Suggestions for fixing invalid escape sequence or unescaped characters in strings.
mod escaped_sequence_suggestions {
    use insta::assert_snapshot;
    use rsonpath_syntax::parse;

    #[test]
    fn backslash() {
        let src = r"$['abc\def']";
        let result = parse(src).expect_err("should fail to parse");
        assert_snapshot!(result, @r#"
        error: invalid escape sequence

          $['abc\def']
                ^^ not a valid escape sequence
          (bytes 6-7)

        note: the only valid escape sequences are \n, \r, \t, \f, \b, \\, \/, \' (in single quoted names), \" (in double quoted names), and \uXXXX where X are hex digits
        note: if you meant to match a literal backslash, you need to escape it with \\
        suggestion: did you mean `$['abc\\def']` ?
        "#);
    }

    #[test]
    fn newline() {
        let src = "$['abc\ndef']";
        let result = parse(src).expect_err("should fail to parse");
        assert_snapshot!(result, @r"
        error: invalid unescaped control character

           1 | $['abc
             (byte 6)


        suggestion: did you mean `$['abc\ndef']` ?
        ");
    }
}

/// Verify that a filter expression error produced at end of input is properly highlighted
mod filter_error_at_end_of_input {
    use insta::assert_snapshot;
    use rsonpath_syntax::parse;

    #[test]
    fn incomplete_filter_selector() {
        let result = parse("$.a[?").expect_err("should fail to parse");
        assert_snapshot!(result, @"
        error: invalid filter expression syntax

          $.a[?
               ^ not a valid filter expression
          (byte 5)


        error: bracketed selection is not closed

          $.a[?
               ^ expected a closing bracket ']'
          (byte 5)
        ");
    }

    #[test]
    fn filter_comparison_missing_rhs() {
        let result = parse("$[?@.b ==").expect_err("should fail to parse");
        assert_snapshot!(result, @"
        error: invalid right-hand side of comparison

          $[?@.b ==
                   ^ expected a literal or a filter query here
          (byte 9)


        error: bracketed selection is not closed

          $[?@.b ==
                   ^ expected a closing bracket ']'
          (byte 9)
        ");
    }
}

mod missing_at_end {
    use insta::assert_snapshot;
    use rsonpath_syntax::parse;

    #[test]
    fn missing_after_child() {
        let src = "$.";
        let result = parse(src).expect_err("should fail to parse");

        assert_snapshot!(result, @"
        error: invalid selector - empty

          $.
           ^^ expected a selector here, but found nothing
          (bytes 1-2)

        note: if you meant to match any value, you should use the wildcard selector `*`
        suggestion: did you mean `$.*` ?
        ");
    }

    #[test]
    fn missing_after_descendant() {
        let src = "$..";
        let result = parse(src).expect_err("should fail to parse");

        assert_snapshot!(result, @"
        error: invalid selector - empty

          $..
            ^^ expected a selector here, but found nothing
          (bytes 2-3)

        note: if you meant to match any value, you should use the wildcard selector `*`
        suggestion: did you mean `$..*` ?
        ");
    }

    #[test]
    fn missing_after_bracket() {
        let src = "$[";
        let result = parse(src).expect_err("should fail to parse");

        assert_snapshot!(result, @"
        error: invalid selector - empty

          $[
           ^^ expected a selector here, but found nothing
          (bytes 1-2)

        note: if you meant to match any value, you should use the wildcard selector `*`
        error: bracketed selection is not closed

          $[
            ^ expected a closing bracket ']'
          (byte 2)


        suggestion: did you mean `$[*]` ?
        ");
    }

    #[test]
    fn missing_in_brackets() {
        let src = "$[]";
        let result = parse(src).expect_err("should fail to parse");

        assert_snapshot!(result, @"
        error: invalid selector - empty

          $[]
           ^^ expected a selector here, but found nothing
          (bytes 1-2)

        note: if you meant to match any value, you should use the wildcard selector `*`
        suggestion: did you mean `$[*]` ?
        ");
    }

    #[test]
    fn missing_after_paren() {
        let src = "$[?(";
        let result = parse(src).expect_err("should fail to parse");

        assert_snapshot!(result, @"
        error: invalid filter expression syntax

          $[?(
              ^ not a valid filter expression
          (byte 4)


        error: bracketed selection is not closed

          $[?(
              ^ expected a closing bracket ']'
          (byte 4)
        ");
    }

    #[test]
    fn missing_in_parens() {
        let src = "$[?()]";
        let result = parse(src).expect_err("should fail to parse");

        assert_snapshot!(result, @"
        error: invalid filter expression syntax

          $[?()]
              ^ not a valid filter expression
          (byte 4)
        ");
    }
}

mod multiline {
    // These are too long to be useful so we use the out-of-line snapshots.
    use insta::assert_snapshot;
    use rsonpath_syntax::parse;

    #[test]
    fn mixed_errors() {
        let src = r#"$.thisIsFine["and so is this"]
.hereWeErr[100.0]
.butThisIsFine.*
.nowFilter[?@.a < @.thisWillBeInvalid
.becauseItIsNotSingular
.*
.see]"#;
        let result = parse(src).expect_err("should fail to parse");
        assert_snapshot!(result);
    }
}

/// Tests for inputs that are too long to be fully printed out as context, activating the truncation machinery.
// Truncation Machinery is a great band name.
mod long_inputs_truncation {
    // These are too long to be useful so we use the out-of-line snapshots.
    use insta::assert_snapshot;
    use rsonpath_syntax::parse;

    #[test]
    fn error_on_a_long_single_line() {
        let src_base = "['a'].";
        let src = src_base.repeat(30); // 180 chars
        let result = parse(&src).expect_err("should fail to parse");
        assert_snapshot!(result);
    }

    #[test]
    fn error_on_long_multiple_line() {
        let src_base = "['a'].";
        let src_line = src_base.repeat(30); // 180 chars
        let mut src = String::with_capacity(3 * src_line.len() + 2);
        src.push_str(&src_line);
        src.push('\n');
        src.push_str(&src_line);
        src.push('\n');
        src.push_str(&src_line);
        let result = parse(&src).expect_err("should fail to parse");
        assert_snapshot!(result);
    }

    #[test]
    fn error_on_mixed_short_and_long_lines() {
        let src_base = ".[a]";
        let src_line = src_base.repeat(30); // 180 chars
        let mut src = String::with_capacity(3 * src_base.len() + 4 + src_line.len());
        src.push_str(src_base);
        src.push('\n');
        src.push_str(src_base);
        src.push('\n');
        src.push_str(src_base);
        src.push('\n');
        src.push_str(&src_line);
        src.push('\n');
        let result = parse(&src).expect_err("should fail to parse");
        assert_snapshot!(result);
    }
}
