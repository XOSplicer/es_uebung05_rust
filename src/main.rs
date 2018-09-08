#[macro_use]
extern crate nom;

mod packet;
mod fletcher_16;

#[derive(Debug, Clone, Copy, PartialEq, Eq,)]
pub struct BufferTooShortError {
    expected: usize,
    actual: usize
}

pub fn main() {

}