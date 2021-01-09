mod lz77;
use num_traits::Bounded;
use std::convert::{TryFrom, TryInto};

use wasm_bindgen::prelude::*;

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_namespace = console)]
    fn log(s: &str);
}

#[wasm_bindgen(start)]
pub fn run() {
    log("Hello from Rust! WASM is now loaded.");
}

#[wasm_bindgen]
pub struct Data {
    pub address: usize,
    pub length: usize,
}

#[wasm_bindgen]
pub fn encode_8(symbols: &[u8]) -> Data {
    return encode::<u8>(symbols);
}
#[wasm_bindgen]
pub fn encode_16(symbols: &[u16]) -> Data {
    return encode::<u16>(symbols);
}
#[wasm_bindgen]
pub fn encode_32(symbols: &[u32]) -> Data {
    return encode::<u32>(symbols);
}

#[wasm_bindgen]
pub fn decode_8(symbols: &[u8]) -> Data {
    return decode::<u8>(symbols);
}
#[wasm_bindgen]
pub fn decode_16(symbols: &[u16]) -> Data {
    return decode::<u16>(symbols);
}
#[wasm_bindgen]
pub fn decode_32(symbols: &[u32]) -> Data {
    return decode::<u32>(symbols);
}

pub fn decode<U>(symbols: &[U]) -> Data
where U: TryInto<usize> + TryFrom<usize> + Bounded + Copy + Eq {
    let decoded_length = symbols.len() / 3;
    let mut encoded = Vec::<lz77::Code<U,U>>::with_capacity(decoded_length);
    let mut symbols_iterator = symbols.iter();
    loop {
        match symbols_iterator.next() {
            Some(offset) => {
                encoded.push(lz77::Code::<U,U> {
                    offset: *offset,
                    length: *symbols_iterator.next().unwrap(),
                    literal: *symbols_iterator.next().unwrap(),
                });
            },
            None => {
                break;
            },
        }
    }
    let decoded = lz77::decode::decode::<U, U>(&encoded);
    return Data {
        address: unsafe { std::mem::transmute::<*const U, usize>(decoded.as_ptr()) } ,
        length: decoded_length,
    };
}

pub fn encode<U>(symbols: &[U]) -> Data
where U: TryFrom<usize> + TryInto<usize> + Bounded + Copy + Eq {
    let encoded = lz77::encode::encode::<U, U>(symbols);
    let encoded_length = encoded.len() * 3;
    let mut encoded_iterator = encoded.iter();
    let mut encoded_symbols = Vec::<U>::with_capacity(encoded_length);
    loop {
        match encoded_iterator.next() {
            Some(code) => {
                encoded_symbols.push(code.offset);
                encoded_symbols.push(code.length);
                encoded_symbols.push(code.literal);
            },
            None => {
                break;
            },
        }
    }
    return Data {
        address: unsafe { std::mem::transmute::<*const U, usize>(encoded_symbols.as_ptr()) },
        length: 3 * encoded.len(),
    };
}