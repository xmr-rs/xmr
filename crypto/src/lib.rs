mod ffi;

pub const FAST_HASH_LENGTH: usize = 32;
pub const SLOW_HASH_LENGTH: usize = 32;

pub fn fast_hash(data: &[u8]) -> [u8; FAST_HASH_LENGTH] {
    use std::os::raw::{c_void, c_char};
    use ffi::{cn_fast_hash, size_t};

    debug_assert!(FAST_HASH_LENGTH == ffi::HASH_SIZE);

    let mut output = [0u8; FAST_HASH_LENGTH];    
    unsafe {
        cn_fast_hash(
            data.as_ptr() as *const c_void,
            data.len() as size_t,
            (&mut output).as_mut_ptr() as *mut c_char
        )
    }

    output
}

pub fn slow_hash(data: &[u8]) -> [u8; SLOW_HASH_LENGTH] {
    use std::os::raw::{c_void, c_char};
    use ffi::{cn_slow_hash, size_t};

    debug_assert!(SLOW_HASH_LENGTH == ffi::HASH_SIZE);

    let mut output = [0u8; SLOW_HASH_LENGTH];    
    unsafe {
        cn_slow_hash(
            data.as_ptr() as *const c_void,
            data.len() as size_t,
            (&mut output).as_mut_ptr() as *mut c_char
        )
    }

    output
}
