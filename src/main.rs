mod lz77;

fn main() {
    let input = "12341234abcabcabcabcXXXXXXXXXXXXX";
    println!("{} is the input.", input);
    let encoding = lz77::encode::encode(input, 1024);
    let decoding = lz77::decode::decode(&encoding);
    println!("{} was obtained by encoding and decoding.", decoding);
}