pub mod encode;
pub mod decode;

#[repr(C)]
#[derive(Debug, PartialEq)]
pub struct Code<T, U> {
    pub offset: U,
    pub length: U,
    pub literal: T,
}

impl<T, U> Code<T, U> {
    pub fn mem_size() -> usize {
        return 2 * std::mem::size_of::<U>() + std::mem::size_of::<T>();
    }
}