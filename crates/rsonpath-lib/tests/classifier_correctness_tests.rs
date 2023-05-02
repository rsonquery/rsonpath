use rsonpath_lib::classification::quotes::classify_quoted_sequences;
use rsonpath_lib::classification::structural::{
    classify_structural_characters, BracketType, Structural, StructuralIterator,
};
use rsonpath_lib::input::OwnedBytes;

fn classify_string(json: &str) -> Vec<Structural> {
    let json_string = json.to_owned();
    let bytes = OwnedBytes::new(&json_string);
    let quotes_classifier = classify_quoted_sequences(&bytes);
    let mut structural_classifier = classify_structural_characters(quotes_classifier);
    structural_classifier.turn_commas_on(0);
    structural_classifier.turn_colons_on(0);

    structural_classifier.collect()
}

#[test]
fn empty_string() {
    let result = classify_string("");

    assert_eq!(Vec::<Structural>::default(), result);
}

#[test]
fn json() {
    let json = r#"{"a": [1, 2, 3], "b": "string", "c": {"d": 42, "e": 17}}"#;
    let expected: &[Structural] = &[
        Structural::Opening(BracketType::Curly, 0),
        Structural::Colon(4),
        Structural::Opening(BracketType::Square, 6),
        Structural::Comma(8),
        Structural::Comma(11),
        Structural::Closing(BracketType::Square, 14),
        Structural::Comma(15),
        Structural::Colon(20),
        Structural::Comma(30),
        Structural::Colon(35),
        Structural::Opening(BracketType::Curly, 37),
        Structural::Colon(41),
        Structural::Comma(45),
        Structural::Colon(50),
        Structural::Closing(BracketType::Curly, 54),
        Structural::Closing(BracketType::Curly, 55),
    ];

    let result = classify_string(json);

    assert_eq!(expected, result);
}

#[test]
fn json_with_escapes() {
    let json = r#"{"a": "Hello, World!", "b": "\"{Hello, [World]!}\""}"#;
    let expected: &[Structural] = &[
        Structural::Opening(BracketType::Curly, 0),
        Structural::Colon(4),
        Structural::Comma(21),
        Structural::Colon(26),
        Structural::Closing(BracketType::Curly, 51),
    ];

    let result = classify_string(json);

    assert_eq!(expected, result);
}

#[test]
fn reverse_exclamation_point() {
    let wtf = "¡";
    let expected: &[Structural] = &[];

    let result = classify_string(wtf);

    assert_eq!(expected, result);
}

#[test]
fn block_boundary() {
    use Structural::*;

    let wtf = r##",,#;0a#0,#a#0#0aa ;a0 0a,"A"#a~A#0a~A##a0|a0#0aaa~ 0#;A|~|"a"A-|;#0 Aa,,"0","A"A0,,,,,,,,,,,,,,,"a",AA;#|#|a;AAA;a A~;aA;A##A#~a ,,,,,,0^A-AA0aa;- ~0,,,#;A;aA#A#0 a-, a;0aaa0|a 0aA -A#a,,,,"\\","##;
    let expected: &[Structural] = &[
        Comma(0),
        Comma(1),
        Comma(8),
        Comma(24),
        Comma(70),
        Comma(71),
        Comma(75),
        Comma(81),
        Comma(82),
        Comma(83),
        Comma(84),
        Comma(85),
        Comma(86),
        Comma(87),
        Comma(88),
        Comma(89),
        Comma(90),
        Comma(91),
        Comma(92),
        Comma(93),
        Comma(94),
        Comma(95),
        Comma(99),
        Comma(129),
        Comma(130),
        Comma(131),
        Comma(132),
        Comma(133),
        Comma(134),
        Comma(149),
        Comma(150),
        Comma(151),
        Comma(165),
        Comma(185),
        Comma(186),
        Comma(187),
        Comma(188),
        Comma(193),
    ];

    let result = classify_string(wtf);

    assert_eq!(expected, result);
}

mod prop_test {
    use super::{classify_string, BracketType, Structural};
    use proptest::{self, collection, prelude::*};

    #[derive(Debug, Clone)]
    enum Token {
        Comma,
        Colon,
        OpeningBrace,
        OpeningBracket,
        ClosingBrace,
        ClosingBracket,
        Garbage(String),
    }

    fn token_strategy() -> impl Strategy<Value = Token> {
        prop_oneof![
            Just(Token::Comma),
            Just(Token::Colon),
            Just(Token::OpeningBrace),
            Just(Token::OpeningBracket),
            Just(Token::ClosingBrace),
            Just(Token::ClosingBracket),
            r#"[^"\\,:{\[\}\]]+"#.prop_map(Token::Garbage)
        ]
    }

    fn tokens_strategy() -> impl Strategy<Value = Vec<Token>> {
        collection::vec(token_strategy(), collection::SizeRange::default())
    }

    fn tokens_into_string(tokens: &[Token]) -> String {
        tokens
            .iter()
            .map(|x| match x {
                Token::Comma => ",",
                Token::Colon => ":",
                Token::OpeningBrace => "{",
                Token::OpeningBracket => "[",
                Token::ClosingBrace => "}",
                Token::ClosingBracket => "]",
                Token::Garbage(string) => string,
            })
            .collect::<String>()
    }

    fn tokens_into_structurals(tokens: &[Token]) -> Vec<Structural> {
        tokens
            .iter()
            .scan(0usize, |idx, x| {
                let expected = match x {
                    Token::Comma => Some(Structural::Comma(*idx)),
                    Token::Colon => Some(Structural::Colon(*idx)),
                    Token::OpeningBrace => Some(Structural::Opening(BracketType::Curly, *idx)),
                    Token::OpeningBracket => Some(Structural::Opening(BracketType::Square, *idx)),
                    Token::ClosingBrace => Some(Structural::Closing(BracketType::Curly, *idx)),
                    Token::ClosingBracket => Some(Structural::Closing(BracketType::Square, *idx)),
                    _ => None,
                };
                match x {
                    Token::Garbage(string) => *idx += string.len(),
                    _ => *idx += 1,
                }
                Some(expected)
            })
            .flatten()
            .collect::<Vec<_>>()
    }

    fn input_string() -> impl Strategy<Value = (String, Vec<Structural>)> {
        tokens_strategy().prop_map(|x| (tokens_into_string(&x), tokens_into_structurals(&x)))
    }

    proptest! {
        #[test]
        fn classifies_correctly((input, expected) in input_string()) {
            let result = classify_string(&input);

            assert_eq!(expected, result);
        }
    }
}
