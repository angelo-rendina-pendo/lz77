use super::Code;

pub fn decode<T>(encoded: &[Code<T>]) -> Vec<T>
where T: Copy {
    let mut capacity: usize = 0;
    for code in encoded.iter() {
        capacity += code.length + 1;
    }
    let mut decoded = Vec::<T>::with_capacity(capacity);
    let mut position: usize = 0;
    for code in encoded.iter() {
        if code.length > 0 {
            let segment_start = position - code.offset;
            let segment_end = segment_start + code.length;
            let segment = &mut decoded[segment_start..segment_end].to_owned();
            decoded.append(segment);
            position += code.length;
        }
        decoded.push(code.literal);
        position += 1;
    }
    return decoded;
}

pub fn to_string(encoded: &[Code<char>]) -> String {
    let decoded = decode(encoded);
    return decoded.into_iter().collect();
}

#[cfg(test)]
mod to_string {
    use super::*;

    #[test]
    fn empty_encoding() {
        let encoding: [Code<char>; 0] = [];
        let expected_decoding = "";
        let found_decoding = to_string(&encoding);
        assert_eq!(&found_decoding, &expected_decoding);
    }

    #[test]
    fn only_literals() {
        let encoding = vec![
            Code::<char> { offset: 0, length: 0, literal: 'a' },
            Code::<char> { offset: 0, length: 0, literal: 'b' },
            Code::<char> { offset: 0, length: 0, literal: 'c' },
            Code::<char> { offset: 0, length: 0, literal: 'd' },
            Code::<char> { offset: 0, length: 0, literal: 'e' },
        ];
        let expected_decoding = "abcde";
        let found_decoding = to_string(&encoding);
        assert_eq!(&found_decoding, &expected_decoding);
    }

    #[test]
    fn some_matches() {
        let encoding = vec![
            Code::<char> { offset: 0, length: 0, literal: 'a' },
            Code::<char> { offset: 0, length: 0, literal: 'b' },
            Code::<char> { offset: 2, length: 2, literal: 'c' },
            Code::<char> { offset: 3, length: 3, literal: 'd' },
            Code::<char> { offset: 4, length: 3, literal: 'd' },
        ];
        let expected_decoding = "ababcabcdabcd";
        let found_decoding = to_string(&encoding);
        assert_eq!(&found_decoding, &expected_decoding);
    }
}

#[cfg(test)]
mod decode {
    use super::*;

    #[test]
    fn only_literals() {
        let encoding = vec![
            Code::<u8> { offset: 0, length: 0, literal: 0 },
            Code::<u8> { offset: 0, length: 0, literal: 1 },
            Code::<u8> { offset: 0, length: 0, literal: 2 },
            Code::<u8> { offset: 0, length: 0, literal: 3 },
            Code::<u8> { offset: 0, length: 0, literal: 4 },
        ];
        let expected_decoding = [0u8, 1, 2, 3, 4];
        let found_decoding = decode(&encoding);
        assert_eq!(&found_decoding, &expected_decoding);
    }

    #[test]
    fn literals_and_segments() {
        let encoding = vec![
            Code::<u8> { offset: 0, length: 0, literal: 0 },
            Code::<u8> { offset: 1, length: 1, literal: 1 },
            Code::<u8> { offset: 0, length: 0, literal: 2 },
            Code::<u8> { offset: 2, length: 2, literal: 3 },
            Code::<u8> { offset: 4, length: 2, literal: 4 },
        ];
        let expected_decoding = [0u8, 0, 1, 2, 1, 2, 3, 2, 1, 4];
        let found_decoding = decode(&encoding);
        assert_eq!(&found_decoding, &expected_decoding);
    }

    #[test]
    fn literals_and_segments_with_chars() {
        let encoding = vec![
            Code::<char> { offset: 0, length: 0, literal: 'a' },
            Code::<char> { offset: 1, length: 1, literal: 'b' },
            Code::<char> { offset: 0, length: 0, literal: 'c' },
            Code::<char> { offset: 2, length: 2, literal: 'd' },
            Code::<char> { offset: 4, length: 2, literal: 'e' },
        ];
        let expected_decoding = ['a', 'a', 'b', 'c', 'b', 'c', 'd', 'c', 'b', 'e'];
        let found_decoding = decode(&encoding);
        assert_eq!(&found_decoding, &expected_decoding);
    }
}