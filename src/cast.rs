use std::slice;
use std::mem;
pub unsafe fn cast_slice_to_bytes<'a, T>(buf: &'a [T]) -> &'a [u8] {
    std::slice::from_raw_parts(
        buf.as_ptr() as *const u8,
        buf.len() * mem::size_of::<T>()
    )
}