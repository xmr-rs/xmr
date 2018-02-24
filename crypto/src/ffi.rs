#![allow(non_camel_case_types)]

use std::os::raw::{c_void, c_char, c_ulonglong};

pub type size_t = c_ulonglong;

pub const HASH_SIZE: usize = 32;

extern "C" {
    pub fn cn_fast_hash(data: *const c_void, length: size_t, hash: *mut c_char);
    pub fn cn_slow_hash(data: *const c_void, length: size_t, hash: *mut c_char);
}

#[cfg(test)]
pub mod tests {
    use super::*;

    fn f<T>(_t: T) {}

    #[test]
    fn link() {
        f(cn_fast_hash);
        f(cn_slow_hash);
    }
}
