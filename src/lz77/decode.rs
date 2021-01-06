use super::Code;

pub fn to_string<U>(encoded: &[Code<char, U>]) -> String
where U: Into<usize> + Copy {
    let decoded = decode::<char, U>(encoded);
    return decoded.into_iter().collect();
}

pub fn decode<T, U>(encoded: &[Code<T, U>]) -> Vec<T>
where T: Copy, U: Into<usize> + Copy {
    let mut capacity: usize = 0;
    for code in encoded.iter() {
        capacity += code.length.into() + 1;
    }
    let mut decoded = Vec::<T>::with_capacity(capacity);
    let mut position: usize = 0;
    for code in encoded.iter() {
        if code.length.into() > 0 {
            let segment_start = position - code.offset.into();
            let segment_end = segment_start + code.length.into();
            let segment = &mut decoded[segment_start..segment_end].to_owned();
            decoded.append(segment);
            position += code.length.into();
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
            Code::<u8, u8> { offset: 0, length: 0, literal: 0 },
            Code::<u8, u8> { offset: 0, length: 0, literal: 1 },
            Code::<u8, u8> { offset: 0, length: 0, literal: 2 },
            Code::<u8, u8> { offset: 0, length: 0, literal: 3 },
            Code::<u8, u8> { offset: 0, length: 0, literal: 4 },
        ];
        let expected_decoding = [0u8, 1, 2, 3, 4];
        let found_decoding = decode::<u8,u8>(&encoding);
        assert_eq!(&found_decoding, &expected_decoding);
    }

    #[test]
    fn literals_and_segments() {
        let encoding = vec![
            Code::<u8, u8> { offset: 0, length: 0, literal: 0 },
            Code::<u8, u8> { offset: 1, length: 1, literal: 1 },
            Code::<u8, u8> { offset: 0, length: 0, literal: 2 },
            Code::<u8, u8> { offset: 2, length: 2, literal: 3 },
            Code::<u8, u8> { offset: 4, length: 2, literal: 4 },
        ];
        let expected_decoding = [0u8, 0, 1, 2, 1, 2, 3, 2, 1, 4];
        let found_decoding = decode::<u8, u8>(&encoding);
        assert_eq!(&found_decoding, &expected_decoding);
    }

    #[test]
    fn literals_and_segments_with_chars() {
        let encoding = vec![
            Code::<char, u8> { offset: 0, length: 0, literal: 'a' },
            Code::<char, u8> { offset: 1, length: 1, literal: 'b' },
            Code::<char, u8> { offset: 0, length: 0, literal: 'c' },
            Code::<char, u8> { offset: 2, length: 2, literal: 'd' },
            Code::<char, u8> { offset: 4, length: 2, literal: 'e' },
        ];
        let expected_decoding = ['a', 'a', 'b', 'c', 'b', 'c', 'd', 'c', 'b', 'e'];
        let found_decoding = decode::<char, u8>(&encoding);
        assert_eq!(&found_decoding, &expected_decoding);
    }
}

#[cfg(test)]
mod to_string {
    use super::*;

    #[test]
    fn empty_encoding() {
        let encoding: [Code<char, u8>; 0] = [];
        let expected_decoding = "";
        let found_decoding = to_string::<u8>(&encoding);
        assert_eq!(&found_decoding, &expected_decoding);
    }

    #[test]
    fn only_literals() {
        let encoding = vec![
            Code::<char, u8> { offset: 0, length: 0, literal: 'a' },
            Code::<char, u8> { offset: 0, length: 0, literal: 'b' },
            Code::<char, u8> { offset: 0, length: 0, literal: 'c' },
            Code::<char, u8> { offset: 0, length: 0, literal: 'd' },
            Code::<char, u8> { offset: 0, length: 0, literal: 'e' },
        ];
        let expected_decoding = "abcde";
        let found_decoding = to_string::<u8>(&encoding);
        assert_eq!(&found_decoding, &expected_decoding);
    }

    #[test]
    fn some_matches() {
        let encoding = vec![
            Code::<char, u8> { offset: 0, length: 0, literal: 'a' },
            Code::<char, u8> { offset: 0, length: 0, literal: 'b' },
            Code::<char, u8> { offset: 2, length: 2, literal: 'c' },
            Code::<char, u8> { offset: 3, length: 3, literal: 'd' },
            Code::<char, u8> { offset: 4, length: 3, literal: 'd' },
        ];
        let expected_decoding = "ababcabcdabcd";
        let found_decoding = to_string::<u8>(&encoding);
        assert_eq!(&found_decoding, &expected_decoding);
    }
}