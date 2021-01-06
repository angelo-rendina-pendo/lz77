mod lz77;

fn main() {
    let input = "12341234abcabcabcabcXXXXXXXXXXXXX";
    let encoded = lz77::encode::from_string(&input, 1024);
    let decoded = lz77::decode::to_string(&encoded);
    println!("{} is the input.", input);
    println!("{} is obtained by encoding and decoding.", &decoded);
    println!("Input has {} characters, encoded in {} codes.", input.len(), encoded.len());
    println!("Input size: {} bytes.", std::mem::size_of_val(input));
    println!("Encoding size: {} bytes.", std::mem::size_of::<lz77::Code<u8>>() * encoded.len());
}