mod lz77;

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
    pub address: *const u8,
    pub length: usize,
}

#[wasm_bindgen]
pub fn encode(bytes: &[u8]) -> Data {
    let encoded = lz77::encode::encode::<u8, u8>(bytes);
    let encoded_length = encoded.len() * 3;
    let mut encoded_iterator = encoded.iter();
    let mut encoded_bytes = Vec::<u8>::with_capacity(encoded_length);
    loop {
        match encoded_iterator.next() {
            Some(code) => {
                encoded_bytes.push(code.offset);
                encoded_bytes.push(code.length);
                encoded_bytes.push(code.literal);
            },
            None => {
                break;
            },
        }
    }
    return Data {
        address: encoded_bytes.as_ptr(),
        length: 3 * encoded.len(),
    };
}

#[wasm_bindgen]
pub fn decode(bytes: &[u8]) -> Data {
    let decoded_length = bytes.len() / 3;
    let mut encoded = Vec::<lz77::Code<u8,u8>>::with_capacity(decoded_length);
    let mut bytes_iterator = bytes.iter();
    loop {
        match bytes_iterator.next() {
            Some(offset) => {
                encoded.push(lz77::Code::<u8,u8> {
                    offset: *offset,
                    length: *bytes_iterator.next().unwrap(),
                    literal: *bytes_iterator.next().unwrap(),
                });
            },
            None => {
                break;
            },
        }
    }
    let decoded = lz77::decode::decode::<u8, u8>(&encoded);
    return Data {
        address: decoded.as_ptr(),
        length: decoded_length,
    };
}