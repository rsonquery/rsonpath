use aligners::AlignedBytes;
use rsonpath::classify::classify_structural_characters;
use rsonpath::classify::Structural;
use rsonpath::quotes::classify_quoted_sequences;

fn classify_string(json: &str) -> Vec<Structural> {
    let bytes = AlignedBytes::new_padded(json.as_bytes());
    let quotes_classifier = classify_quoted_sequences(&bytes);
    let structural_classifier = classify_structural_characters(quotes_classifier);

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
        Structural::Opening(0),
        Structural::Colon(4),
        Structural::Opening(6),
        Structural::Comma(8),
        Structural::Comma(11),
        Structural::Closing(14),
        Structural::Comma(15),
        Structural::Colon(20),
        Structural::Comma(30),
        Structural::Colon(35),
        Structural::Opening(37),
        Structural::Colon(41),
        Structural::Comma(45),
        Structural::Colon(50),
        Structural::Closing(54),
        Structural::Closing(55),
    ];

    let result = classify_string(json);

    assert_eq!(expected, result);
}

#[test]
fn json_with_escapes() {
    let json = r#"{"a": "Hello, World!", "b": "\"{Hello, [World]!}\""}"#;
    let expected: &[Structural] = &[
        Structural::Opening(0),
        Structural::Colon(4),
        Structural::Comma(21),
        Structural::Colon(26),
        Structural::Closing(51),
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

mod prop_test {
    use super::{classify_string, Structural};
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
                    Token::OpeningBrace | Token::OpeningBracket => Some(Structural::Opening(*idx)),
                    Token::ClosingBrace | Token::ClosingBracket => Some(Structural::Closing(*idx)),
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
