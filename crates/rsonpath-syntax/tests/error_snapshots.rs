use insta::assert_snapshot;
use rsonpath_syntax::parse;

#[test]
fn empty_string_is_invalid() {
    let src = "";
    let result = parse(src).expect_err("should fail to parse");
    assert_snapshot!(result, @r###"
    error: query not starting with the root identifier '$'

      
      ^ the '$' character missing before here
      (byte 0)


    suggestion: did you mean `$` ?
    "###);
}

// This is a regression test. There was a bug where the error handling loop would try to resume
// parsing at the next byte after an invalid character, which is invalid and causes a panic
// if the character takes more than one byte - strings can be indexed only at char boundaries.
#[test]
fn error_handling_across_unicode_values() {
    // Ferris has 4 bytes of encoding.
    let input = "🦀.";
    let result = parse(input).expect_err("should fail to parse");

    assert_snapshot!(result, @r###"
    error: query not starting with the root identifier '$'

      🦀.
      ^^ the '$' character missing before here
      (bytes 0-3)


    error: invalid segment syntax

      🦀.
      ^^^ not a valid segment syntax
      (bytes 0-4)

    note: valid segments are: member name shorthands like `.name`/`..name`; or child/descendant bracketed selections like `[<segments>]`/`..[<segments>]`
    "###);
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
        assert_snapshot!(result, @r###"
        error: invalid unescaped control character

          $[' ']
          (byte 3)


        suggestion: did you mean `$['\u0000']` ?
        "###);
    }

    #[test]
    fn u0019_control_must_be_escaped() {
        let src = "\u{0019}";
        let result = parse_single_quoted_name_selector(src).expect_err("should fail to parse");
        assert_snapshot!(result, @r###"
        error: invalid unescaped control character

          $['']
          (byte 3)


        suggestion: did you mean `$['\u0019']` ?
        "###);
    }

    #[test]
    fn single_quote_in_single_quoted_string_must_be_escaped() {
        let src = "unescaped ' quote";
        let result = parse_single_quoted_name_selector(src).expect_err("should fail to parse");
        assert_snapshot!(result, @r###"
        error: selectors not separated with commas

          $['unescaped ' quote']
                         ^ expected a comma separator before this character
          (byte 15)


        error: invalid selector syntax

          $['unescaped ' quote']
                         ^^^^^^ not a valid selector
          (bytes 15-20)


        "###);
    }

    #[test]
    fn double_quote_in_double_quoted_string_must_be_escaped() {
        let src = r#"unescaped " quote"#;
        let query_string = format!(r#"$["{src}"]"#);
        let err = parse(&query_string).expect_err("should fail to parse");
        assert_snapshot!(err, @r###"
        error: selectors not separated with commas

          $["unescaped " quote"]
                         ^ expected a comma separator before this character
          (byte 15)


        error: invalid selector syntax

          $["unescaped " quote"]
                         ^^^^^^ not a valid selector
          (bytes 15-20)


        "###);
    }

    #[test]
    fn u0020_space_must_not_be_escaped() {
        let src = r"escape \ a space";
        let result = parse_single_quoted_name_selector(src).expect_err("should fail to parse");
        assert_snapshot!(result, @r###"
        error: invalid escape sequence

          $['escape \ a space']
                    ^^ not a valid escape sequence
          (bytes 10-11)

        note: the only valid escape sequences are \n, \r, \t, \f, \b, \\, \/, \' (in single quoted names), \" (in double quoted names), and \uXXXX where X are hex digits
        "###);
    }

    #[test]
    fn backslash_must_be_escaped() {
        let src = r"\";
        let result = parse_single_quoted_name_selector(src).expect_err("should fail to parse");
        assert_snapshot!(result, @r###"
        error: single-quoted name selector is not closed

          $['\']
                ^ expected a single quote `'`
          (byte 6)


        error: bracketed selection is not closed

          $['\']
                ^ expected a closing bracket ']'
          (byte 6)


        suggestion: did you mean `$['\']']` ?
        "###);
    }

    #[test]
    fn unicode_escape_must_be_lowercase() {
        let src = r"\U0012";
        let result = parse_single_quoted_name_selector(src).expect_err("should fail to parse");
        assert_snapshot!(result, @r###"
        error: invalid escape sequence

          $['\U0012']
             ^^ not a valid escape sequence
          (bytes 3-4)

        note: unicode escape sequences must use a lowercase 'u'
        suggestion: did you mean `$['\u0012']` ?
        "###);
    }

    #[test]
    fn high_surrogate_must_be_paired() {
        let src = r"escape \uD800 and that is it";
        let result = parse_single_quoted_name_selector(src).expect_err("should fail to parse");
        assert_snapshot!(result, @r###"
        error: invalid unicode escape sequence - unpaired high surrogate

          $['escape \uD800 and that is it']
                    ^^^^^^ this high surrogate is unpaired
          (bytes 10-15)

        note: a UTF-16 high surrogate has to be followed by a low surrogate to encode a valid Unicode character
        note: for more information about UTF-16 surrogate pairs see https://en.wikipedia.org/wiki/UTF-16#Code_points_from_U+010000_to_U+10FFFF
        "###);
    }

    #[test]
    fn high_surrogate_must_be_paired_with_a_low_surrogate_and_not_high() {
        let src = r"escape \uD800\uD801 and that is it";
        let result = parse_single_quoted_name_selector(src).expect_err("should fail to parse");
        assert_snapshot!(result, @r###"
        error: invalid unicode escape sequence - unpaired high surrogate

          $['escape \uD800\uD801 and that is it']
                    ^^^^^^ this high surrogate is unpaired
          (bytes 10-15)

        note: a UTF-16 high surrogate has to be followed by a low surrogate to encode a valid Unicode character
        note: for more information about UTF-16 surrogate pairs see https://en.wikipedia.org/wiki/UTF-16#Code_points_from_U+010000_to_U+10FFFF
        "###);
    }

    #[test]
    fn high_surrogate_must_be_paired_with_a_low_surrogate_and_not_newline() {
        let src = r"escape \uD800\n and that is it";
        let result = parse_single_quoted_name_selector(src).expect_err("should fail to parse");
        assert_snapshot!(result, @r###"
        error: invalid unicode escape sequence - unpaired high surrogate

          $['escape \uD800\n and that is it']
                    ^^^^^^ this high surrogate is unpaired
          (bytes 10-15)

        note: a UTF-16 high surrogate has to be followed by a low surrogate to encode a valid Unicode character
        note: for more information about UTF-16 surrogate pairs see https://en.wikipedia.org/wiki/UTF-16#Code_points_from_U+010000_to_U+10FFFF
        "###);
    }

    #[test]
    fn high_surrogate_must_be_paired_with_a_low_surrogate_and_not_regular_unicode_escape() {
        let src = r"escape \uD800\uCC01 and that is it";
        let result = parse_single_quoted_name_selector(src).expect_err("should fail to parse");
        assert_snapshot!(result, @r###"
        error: invalid unicode escape sequence - unpaired high surrogate

          $['escape \uD800\uCC01 and that is it']
                    ^^^^^^ this high surrogate is unpaired
          (bytes 10-15)

        note: a UTF-16 high surrogate has to be followed by a low surrogate to encode a valid Unicode character
        note: for more information about UTF-16 surrogate pairs see https://en.wikipedia.org/wiki/UTF-16#Code_points_from_U+010000_to_U+10FFFF
        "###);
    }

    #[test]
    fn low_surrogate_must_be_paired() {
        let src = r"escape \uDC01 and that is it";
        let result = parse_single_quoted_name_selector(src).expect_err("should fail to parse");
        assert_snapshot!(result, @r###"
        error: invalid unicode escape sequence - unpaired low surrogate

          $['escape \uDC01 and that is it']
                    ^^^^^^ this low surrogate is unpaired
          (bytes 10-15)

        note: a UTF-16 low surrogate has to be preceded by a high surrogate to encode a valid Unicode character
        note: for more information about UTF-16 surrogate pairs see https://en.wikipedia.org/wiki/UTF-16#Code_points_from_U+010000_to_U+10FFFF
        "###);
    }

    #[test]
    fn low_surrogate_must_be_paired_with_a_high_surrogate_and_not_low() {
        let src = r"escape \uDC01\uDC02 and that is it";
        let result = parse_single_quoted_name_selector(src).expect_err("should fail to parse");
        assert_snapshot!(result, @r###"
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
        "###);
    }

    #[test]
    fn low_surrogate_must_be_paired_with_a_high_surrogate_and_not_newline() {
        let src = r"escape \n\uDC01\n and that is it";
        let result = parse_single_quoted_name_selector(src).expect_err("should fail to parse");
        assert_snapshot!(result, @r###"
        error: invalid unicode escape sequence - unpaired low surrogate

          $['escape \n\uDC01\n and that is it']
                      ^^^^^^ this low surrogate is unpaired
          (bytes 12-17)

        note: a UTF-16 low surrogate has to be preceded by a high surrogate to encode a valid Unicode character
        note: for more information about UTF-16 surrogate pairs see https://en.wikipedia.org/wiki/UTF-16#Code_points_from_U+010000_to_U+10FFFF
        "###);
    }

    #[test]
    fn low_surrogate_must_be_paired_with_a_high_surrogate_and_not_regular_unicode_escape() {
        let src = r"escape \uCC01\uDC01\uCC01 and that is it";
        let result = parse_single_quoted_name_selector(src).expect_err("should fail to parse");
        assert_snapshot!(result, @r###"
        error: invalid unicode escape sequence - unpaired low surrogate

          $['escape \uCC01\uDC01\uCC01 and that is it']
                          ^^^^^^ this low surrogate is unpaired
          (bytes 16-21)

        note: a UTF-16 low surrogate has to be preceded by a high surrogate to encode a valid Unicode character
        note: for more information about UTF-16 surrogate pairs see https://en.wikipedia.org/wiki/UTF-16#Code_points_from_U+010000_to_U+10FFFF
        "###);
    }

    #[test]
    fn backslash_u_alone_is_not_valid() {
        let src = r"\u";
        let result = parse_single_quoted_name_selector(src).expect_err("should fail to parse");
        assert_snapshot!(result, @r###"
        error: invalid unicode escape sequence - invalid hex digit

          $['\u']
               ^ not a hex digit
          (byte 5)

        note: valid hex digits are 0 through 9 and A through F (case-insensitive)
        "###);
    }

    #[test]
    fn unicode_escape_must_have_four_hex_digits_not_zero() {
        let src = r"escape \u and that is it";
        let result = parse_single_quoted_name_selector(src).expect_err("should fail to parse");
        assert_snapshot!(result, @r###"
        error: invalid unicode escape sequence - invalid hex digit

          $['escape \u and that is it']
                      ^ not a hex digit
          (byte 12)

        note: valid hex digits are 0 through 9 and A through F (case-insensitive)
        "###);
    }

    #[test]
    fn unicode_escape_must_have_four_hex_digits_not_one() {
        let src = r"escape \u1 and that is it";
        let result = parse_single_quoted_name_selector(src).expect_err("should fail to parse");
        assert_snapshot!(result, @r###"
        error: invalid unicode escape sequence - invalid hex digit

          $['escape \u1 and that is it']
                       ^ not a hex digit
          (byte 13)

        note: valid hex digits are 0 through 9 and A through F (case-insensitive)
        "###);
    }

    #[test]
    fn unicode_escape_must_have_four_hex_digits_not_two() {
        let src = r"escape \u12 and that is it";
        let result = parse_single_quoted_name_selector(src).expect_err("should fail to parse");
        assert_snapshot!(result, @r###"
        error: invalid unicode escape sequence - invalid hex digit

          $['escape \u12 and that is it']
                        ^ not a hex digit
          (byte 14)

        note: valid hex digits are 0 through 9 and A through F (case-insensitive)
        "###);
    }

    #[test]
    fn unicode_escape_must_have_four_hex_digits_not_three() {
        let src = r"escape \u123 and that is it";
        let result = parse_single_quoted_name_selector(src).expect_err("should fail to parse");
        assert_snapshot!(result, @r###"
        error: invalid unicode escape sequence - invalid hex digit

          $['escape \u123 and that is it']
                         ^ not a hex digit
          (byte 15)

        note: valid hex digits are 0 through 9 and A through F (case-insensitive)
        "###);
    }

    #[test]
    fn g_is_not_a_valid_first_hex_digit() {
        let src = r"escape \uG234 and that is it";
        let result = parse_single_quoted_name_selector(src).expect_err("should fail to parse");
        assert_snapshot!(result, @r###"
        error: invalid unicode escape sequence - invalid hex digit

          $['escape \uG234 and that is it']
                      ^ not a hex digit
          (byte 12)

        note: valid hex digits are 0 through 9 and A through F (case-insensitive)
        "###);
    }

    #[test]
    fn g_is_not_a_valid_second_hex_digit() {
        let src = r"escape \u1G34 and that is it";
        let result = parse_single_quoted_name_selector(src).expect_err("should fail to parse");
        assert_snapshot!(result, @r###"
        error: invalid unicode escape sequence - invalid hex digit

          $['escape \u1G34 and that is it']
                       ^ not a hex digit
          (byte 13)

        note: valid hex digits are 0 through 9 and A through F (case-insensitive)
        "###);
    }

    #[test]
    fn g_is_not_a_valid_third_hex_digit() {
        let src = r"escape \u12G4 and that is it";
        let result = parse_single_quoted_name_selector(src).expect_err("should fail to parse");
        assert_snapshot!(result, @r###"
        error: invalid unicode escape sequence - invalid hex digit

          $['escape \u12G4 and that is it']
                        ^ not a hex digit
          (byte 14)

        note: valid hex digits are 0 through 9 and A through F (case-insensitive)
        "###);
    }

    #[test]
    fn g_is_not_a_valid_fourth_hex_digit() {
        let src = r"escape \u123G and that is it";
        let result = parse_single_quoted_name_selector(src).expect_err("should fail to parse");
        assert_snapshot!(result, @r###"
        error: invalid unicode escape sequence - invalid hex digit

          $['escape \u123G and that is it']
                         ^ not a hex digit
          (byte 15)

        note: valid hex digits are 0 through 9 and A through F (case-insensitive)
        "###);
    }

    #[test]
    fn high_surrogate_must_be_paired_with_a_low_surrogate_not_zero_digit_unicode_escape() {
        let src = r"escape \uD800\u and that is it";
        let result = parse_single_quoted_name_selector(src).expect_err("should fail to parse");
        assert_snapshot!(result, @r###"
        error: invalid unicode escape sequence - invalid hex digit

          $['escape \uD800\u and that is it']
                            ^ not a hex digit
          (byte 18)

        note: valid hex digits are 0 through 9 and A through F (case-insensitive)
        "###);
    }

    #[test]
    fn high_surrogate_must_be_paired_with_a_low_surrogate_not_one_digit_unicode_escape() {
        let src = r"escape \uD800\uD and that is it";
        let result = parse_single_quoted_name_selector(src).expect_err("should fail to parse");
        assert_snapshot!(result, @r###"
        error: invalid unicode escape sequence - invalid hex digit

          $['escape \uD800\uD and that is it']
                             ^ not a hex digit
          (byte 19)

        note: valid hex digits are 0 through 9 and A through F (case-insensitive)
        "###);
    }

    #[test]
    fn high_surrogate_must_be_paired_with_a_low_surrogate_not_two_digit_unicode_escape() {
        let src = r"escape \uD800\uDC and that is it";
        let result = parse_single_quoted_name_selector(src).expect_err("should fail to parse");
        assert_snapshot!(result, @r###"
        error: invalid unicode escape sequence - invalid hex digit

          $['escape \uD800\uDC and that is it']
                              ^ not a hex digit
          (byte 20)

        note: valid hex digits are 0 through 9 and A through F (case-insensitive)
        "###);
    }

    #[test]
    fn high_surrogate_must_be_paired_with_a_low_surrogate_not_three_digit_unicode_escape() {
        let src = r"escape \uD800\uDC0 and that is it";
        let result = parse_single_quoted_name_selector(src).expect_err("should fail to parse");
        assert_snapshot!(result, @r###"
        error: invalid unicode escape sequence - invalid hex digit

          $['escape \uD800\uDC0 and that is it']
                               ^ not a hex digit
          (byte 21)

        note: valid hex digits are 0 through 9 and A through F (case-insensitive)
        "###);
    }

    #[test]
    fn high_surrogate_must_be_paired_with_a_low_surrogate_not_invalid_unicode_escape() {
        let src = r"escape \uD800\uDC0X and that is it";
        let result = parse_single_quoted_name_selector(src).expect_err("should fail to parse");
        assert_snapshot!(result, @r###"
        error: invalid unicode escape sequence - invalid hex digit

          $['escape \uD800\uDC0X and that is it']
                               ^ not a hex digit
          (byte 21)

        note: valid hex digits are 0 through 9 and A through F (case-insensitive)
        "###);
    }

    #[test]
    fn error_following_wide_letters_should_be_properly_highlighted() {
        let src = r"Ｈｅｌｌｏ, ｗｏｒｌｄ!\u222X";
        let result = parse_single_quoted_name_selector(src).expect_err("should fail to parse");
        assert_snapshot!(result, @r###"
        error: invalid unicode escape sequence - invalid hex digit

          $['Ｈｅｌｌｏ, ｗｏｒｌｄ!\u222X']
                                         ^ not a hex digit
          (byte 41)

        note: valid hex digits are 0 through 9 and A through F (case-insensitive)
        "###);
    }

    #[test]
    fn error_following_multibyte_grapheme_cluster_should_be_properly_highlighted() {
        let src = r"क्\u12G4";
        let result = parse_single_quoted_name_selector(src).expect_err("should fail to parse");
        assert_snapshot!(result, @r###"
        error: invalid unicode escape sequence - invalid hex digit

          $['क्\u12G4']
                  ^ not a hex digit
          (byte 13)

        note: valid hex digits are 0 through 9 and A through F (case-insensitive)
        "###);
    }

    #[test]
    fn error_following_ligature_emoji_should_be_properly_highlighted() {
        let src = r"👩‍🔬\u222X";
        let result = parse_single_quoted_name_selector(src).expect_err("should fail to parse");
        assert_snapshot!(result, @r###"
        error: invalid unicode escape sequence - invalid hex digit

          $['👩‍🔬\u222X']
                      ^ not a hex digit
          (byte 19)

        note: valid hex digits are 0 through 9 and A through F (case-insensitive)
        "###);
    }
}
