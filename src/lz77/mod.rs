pub mod encode;
pub mod decode;

#[derive(Debug, PartialEq)]
pub struct Code<T> {
    offset: usize,
    length: usize,
    literal: T,
}