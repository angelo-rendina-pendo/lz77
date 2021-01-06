pub mod encode;
pub mod decode;

#[repr(C)]
#[derive(Debug, PartialEq)]
pub struct Code<T, U> {
    offset: U,
    length: U,
    literal: T,
}