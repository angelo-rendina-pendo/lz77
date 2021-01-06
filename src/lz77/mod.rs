pub mod encode;
pub mod decode;

#[derive(Debug, PartialEq)]
pub struct Code {
    offset: usize,
    length: usize,
    literal: char,
}