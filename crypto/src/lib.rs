extern crate libc;

use std::mem::transmute;
use libc::{c_void, c_char, size_t};

mod ffi;

pub const FAST_HASH_LENGTH: usize = 32;
pub const SLOW_HASH_LENGTH: usize = 32;

pub fn fast_hash(data: &[u8]) -> [u8; FAST_HASH_LENGTH] {
    use ffi::cn_fast_hash;

    debug_assert!(FAST_HASH_LENGTH == ffi::HASH_SIZE);

    let output = &mut [0u8; FAST_HASH_LENGTH];    
    unsafe {
        cn_fast_hash(
            data.as_ptr() as *const c_void,
            data.len() as size_t,
            transmute::<*mut u8, *mut c_char>(output.as_mut_ptr()),
        )
    }

    *output
}

pub fn slow_hash(data: &[u8]) -> [u8; SLOW_HASH_LENGTH] {
    use ffi::cn_slow_hash;

    debug_assert!(SLOW_HASH_LENGTH == ffi::HASH_SIZE);

    let output = &mut [0u8; FAST_HASH_LENGTH];    
    unsafe {
        cn_slow_hash(
            data.as_ptr() as *const c_void,
            data.len() as size_t,
            transmute::<*mut u8, *mut c_char>(output.as_mut_ptr()),
        )
    }

    *output
}
