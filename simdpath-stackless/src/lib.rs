use simdpath_core::bytes::*;

pub fn run_simdpath3(contents: &str, word1: &str, word2: &str, word3: &str) -> usize {
    let word1 = word1.as_bytes();
    let word2 = word2.as_bytes();
    let word3 = word3.as_bytes();
    let mut reg1: usize = 0;
    let mut reg2: usize = 0;
    let mut depth: usize = 0;
    let mut state = 0;
    let mut bytes = contents.as_bytes();
    let mut count = 0;

    loop {
        match state {
            0 => {
                if let Some(i) = find_non_whitespace(bytes) {
                    match bytes[i] {
                        b'{' => {
                            depth += 1;
                            bytes = &bytes[i + 1..];
                        }
                        b'}' => {
                            depth -= 1;
                            bytes = &bytes[i + 1..];
                        }
                        b'[' => {
                            depth += 1;
                            bytes = &bytes[i + 1..];
                        }
                        b']' => {
                            depth -= 1;
                            bytes = &bytes[i + 1..];
                        }
                        b'\\' => {
                            bytes = &bytes[i + 2..];
                        }
                        b'"' => {
                            bytes = &bytes[i + 1..];
                            let closing_quote = find_unescaped_byte(b'"', bytes)
                                .expect("Malformed JSON: closing quote missing.");

                            let label = &bytes[..closing_quote];
                            bytes = &bytes[closing_quote + 1..];
                            let next = find_non_whitespace(bytes).unwrap();

                            if bytes[next] == b':' && label == word1 {
                                state = 1;
                                reg1 = depth;
                                bytes = &bytes[next + 1..];
                            } else {
                                bytes = &bytes[next..];
                            }
                        }
                        _ => {
                            bytes = &bytes[i + 1..];
                        }
                    }
                } else {
                    break;
                }
            }
            1 => {
                if let Some(i) = find_non_whitespace(bytes) {
                    match bytes[i] {
                        b'{' => {
                            depth += 1;
                            bytes = &bytes[i + 1..];
                        }
                        b'}' => {
                            depth -= 1;
                            bytes = &bytes[i + 1..];
                            if depth == reg1 {
                                state = 0;
                            }
                        }
                        b'[' => {
                            depth += 1;
                            bytes = &bytes[i + 1..];
                        }
                        b']' => {
                            depth -= 1;
                            bytes = &bytes[i + 1..];
                            if depth == reg1 {
                                state = 0;
                            }
                        }
                        b'\\' => {
                            bytes = &bytes[i + 2..];
                        }
                        b'"' => {
                            bytes = &bytes[i + 1..];
                            let closing_quote = find_unescaped_byte(b'"', bytes)
                                .expect("Malformed JSON: closing quote missing.");

                            let label = &bytes[..closing_quote];
                            bytes = &bytes[closing_quote + 1..];
                            let next = find_non_whitespace(bytes).unwrap();

                            if bytes[next] == b':' && label == word2 {
                                state = 2;
                                reg2 = depth;
                                bytes = &bytes[next + 1..];
                            } else {
                                bytes = &bytes[next..];
                            }
                        }
                        _ => {
                            bytes = &bytes[i + 1..];
                        }
                    }
                } else {
                    break;
                }
            }
            2 => {
                if let Some(i) = find_non_whitespace(bytes) {
                    match bytes[i] {
                        b'{' => {
                            depth += 1;
                            bytes = &bytes[i + 1..];
                        }
                        b'}' => {
                            depth -= 1;
                            bytes = &bytes[i + 1..];
                            if depth == reg2 {
                                state = 1;
                            }
                        }
                        b'[' => {
                            depth += 1;
                            bytes = &bytes[i + 1..];
                        }
                        b']' => {
                            depth -= 1;
                            bytes = &bytes[i + 1..];
                            if depth == reg2 {
                                state = 1;
                            }
                        }
                        b'\\' => {
                            bytes = &bytes[i + 2..];
                        }
                        b'"' => {
                            bytes = &bytes[i + 1..];
                            let closing_quote = find_unescaped_byte(b'"', bytes)
                                .expect("Malformed JSON: closing quote missing.");

                            let label = &bytes[..closing_quote];
                            bytes = &bytes[closing_quote + 1..];
                            let next = find_non_whitespace(bytes).unwrap();

                            if bytes[next] == b':' && label == word3 {
                                count += 1;
                                bytes = &bytes[next + 1..];
                            } else {
                                bytes = &bytes[next..];
                            }
                        }
                        _ => {
                            bytes = &bytes[i + 1..];
                        }
                    }
                } else {
                    break;
                }
            }
            _ => unreachable! {},
        }
    }

    count
}
