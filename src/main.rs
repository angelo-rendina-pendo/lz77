mod lz77;

fn main() {
    let input = "12341234abcabcabcabcXXXXXXXXXXXXX";
    println!("{} is the input.", input);
    let encoding = lz77::encode::encode(input, 1024);
    let decoding = lz77::decode::decode(&encoding);
    println!("{} was obtained by encoding and decoding.", decoding);
    println!("Input has {} characters, encoded in {} codes.", input.len(), encoding.len());
    println!("Input size: {} bytes.", std::mem::size_of_val(input));
    println!("Encoding size: {} bytes.", std::mem::size_of::<lz77::Code>() * encoding.len());
}