use super::Code;

pub fn encode(input: &str, max_window_length: usize) -> Vec<Code> {
    let mut encoded: Vec<Code> = Vec::with_capacity(input.len());
    let mut position: usize = 0;
    while position < input.len() {
        let lookahead = &input[position..];
        let window_start = position.saturating_sub(max_window_length);
        let window = &input[window_start..position];
        let code = find_code(&window, &lookahead);
        position += code.length + 1;
        encoded.push(code);
    }
    return encoded;
}

fn find_code(window: &str, lookahead: &str) -> Code {
    let mut lookahead_chars = lookahead.chars();
    let mut code = Code {
        offset: 0,
        length: 0,
        literal: lookahead_chars.next().unwrap(),
    };
    let mut search_length: usize = 1;
    while search_length < lookahead.len() {
        let search = &lookahead[..search_length];
        let rightmost_match = window.rfind(search);
        if rightmost_match == None {
            break;
        }
        code.offset = window.len() - rightmost_match.unwrap();
        code.length = search_length;
        code.literal = lookahead_chars.next().unwrap();
        search_length += 1;
    }
    return code;
}

#[cfg(test)]
mod find_code {
    use super::*;

    #[test]
    fn no_match() {
        let lookahead = "abc";
        let window = "xyz";
        let expected_code = Code {
            offset: 0,
            length: 0,
            literal: 'a',
        };
        let found_code = find_code(window, lookahead);
        assert_eq!(found_code, expected_code);
    }

    #[test]
    fn trivial_match_at_end() {
        let lookahead = "a";
        let window = "a";
        let expected_code = Code {
            offset: 0,
            length: 0,
            literal: 'a',
        };
        let found_code = find_code(window, lookahead);
        assert_eq!(found_code, expected_code);
    }

    #[test]
    fn trivial_match_with_next() {
        let lookahead = "ab";
        let window = "a";
        let expected_code = Code {
            offset: 1,
            length: 1,
            literal: 'b',
        };
        let found_code = find_code(window, lookahead);
        assert_eq!(found_code, expected_code);
    }

    #[test]
    fn match_is_longest() {
        let lookahead = "abcdef";
        let window = "abcdeabcdabcaba";
        let expected_code = Code {
            offset: 15,
            length: 5,
            literal: 'f',
        };
        let found_code = find_code(window, lookahead);
        assert_eq!(found_code, expected_code);
    }

    #[test]
    fn match_is_rightmost() {
        let lookahead = "abc";
        let window = "ababcabc";
        let expected_code = Code {
            offset: 3,
            length: 2,
            literal: 'c',
        };
        let found_code = find_code(window, lookahead);
        assert_eq!(found_code, expected_code);
    }
}

#[cfg(test)]
mod encode {
    use super::*;

    #[test]
    fn no_matches() {
        let input = "abcde";
        let max_window_length: usize = 16;
        let expected_encoding = vec![
            Code { offset: 0, length: 0, literal: 'a' },
            Code { offset: 0, length: 0, literal: 'b' },
            Code { offset: 0, length: 0, literal: 'c' },
            Code { offset: 0, length: 0, literal: 'd' },
            Code { offset: 0, length: 0, literal: 'e' },
        ];
        let found_encoding = encode(input, max_window_length);
        assert_eq!(found_encoding, expected_encoding);
    }

    #[test]
    fn some_matches() {
        let input = "abcababc";
        let max_window_length: usize = 16;
        let expected_encoding = vec![
            Code { offset: 0, length: 0, literal: 'a' },
            Code { offset: 0, length: 0, literal: 'b' },
            Code { offset: 0, length: 0, literal: 'c' },
            Code { offset: 3, length: 2, literal: 'a' },
            Code { offset: 2, length: 1, literal: 'c' },
        ];
        let found_encoding = encode(input, max_window_length);
        assert_eq!(found_encoding, expected_encoding);
    }
}