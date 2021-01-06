use super::Code;

pub fn decode(encoded: &Vec<Code>) -> String {
    let mut capacity: usize = 0;
    for code in encoded.iter() {
        capacity += code.length + 1;
    }
    let mut decoded = String::with_capacity(capacity);
    let mut position: usize = 0;
    for code in encoded.iter() {
        if code.length > 0 {
            let segment_start = position - code.offset;
            let segment_end = segment_start + code.length;
            let segment = &decoded[segment_start..segment_end].to_owned();
            decoded.push_str(segment);
            position += code.length;
        }
        decoded.push(code.literal);
        position += 1;
    }
    return decoded;
}

#[cfg(test)]
mod decode {
    use super::*;

    #[test]
    fn only_literals() {
        let encoding = vec![
            Code { offset: 0, length: 0, literal: 'a' },
            Code { offset: 0, length: 0, literal: 'b' },
            Code { offset: 0, length: 0, literal: 'c' },
            Code { offset: 0, length: 0, literal: 'd' },
            Code { offset: 0, length: 0, literal: 'e' },
        ];
        let expected_decoding = "abcde";
        let found_decoding = decode(&encoding);
        assert_eq!(found_decoding, expected_decoding);
    }

    #[test]
    fn literals_and_segments() {
        let encoding = vec![
            Code { offset: 0, length: 0, literal: 'a' },
            Code { offset: 0, length: 0, literal: 'b' },
            Code { offset: 2, length: 2, literal: 'c' },
            Code { offset: 0, length: 0, literal: 'd' },
            Code { offset: 4, length: 3, literal: 'e' },
        ];
        let expected_decoding = "ababcdabce";
        let found_decoding = decode(&encoding);
        assert_eq!(found_decoding, expected_decoding);
    }
}