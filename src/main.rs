extern crate num_traits;

mod lz77;
mod io;

fn main() {
    let args: Vec<String> = std::env::args().collect();
    if args.len() > 1 {
        let filename = &args[1];
        let input = io::stdin::file_bytes(filename);
        let encoded = lz77::encode::encode::<u8, u8>(&input);
        println!("File is {} bytes long.", input.len());
        println!("Encoding size: {} bytes.", std::mem::size_of::<lz77::Code<u8, u8>>() * encoded.len());
    } else {
        let input = "12341234abcabcabcabcXXXXXXXXXXXXX";
        let encoded = lz77::encode::from_string::<u8>(&input);
        let decoded = lz77::decode::to_string(&encoded);
        println!("Usage: lz77 [file]");
        println!("If file provided, it encodes it and prints the compressed size.");
        println!("No file given, so printing an example run now.");
        println!("{} is the input.", input);
        println!("{} is obtained by encoding and decoding.", &decoded);
        println!("Input has {} characters, encoded in {} codes.", input.len(), encoded.len());
        println!("Input size: {} bytes.", std::mem::size_of_val(input));
        println!("Encoding size: {} bytes.", std::mem::size_of::<lz77::Code<char, u8>>() * encoded.len());
    }
}