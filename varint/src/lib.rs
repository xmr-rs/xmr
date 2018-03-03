//! # varint
//! XZ variable length integers reading/writing

extern crate bytes;
extern crate num;

use std::mem::size_of;

use bytes::{BytesMut, Buf, BufMut};
use num::cast::ToPrimitive;

/// An error occurred during reading.
#[derive(Debug, Clone, Copy)]
pub enum ReadError {
    /// The integer is too large to fit in the current type.
    Overflow,
    /// The integer cannot be represented.
    Represent,
}

impl std::fmt::Display for ReadError {
    fn fmt(&self, fmt: &mut std::fmt::Formatter) -> std::fmt::Result {
        match *self {
            ReadError::Overflow => write!(fmt, "the integer is too large"),
            ReadError::Represent => write!(fmt, "the integer cannot be represented"),
        }
    }
}

pub fn read<B: Buf>(buf: &mut B) -> Result<u64, ReadError> {
    let bits = (size_of::<u64>() * 8) as u64;
    let mut output = 0u64;
    let mut shift = 0u64;
    loop {
        let byte = buf.get_u8();
        
        if shift + 7 >= bits && byte >= 1 << (bits - shift) {
            return Err(ReadError::Overflow);
        }

        if byte == 0 && shift != 0 {
            return Err(ReadError::Represent);
        }
        
        // Does the actualy placing into output, stripping the first bit
        output |= ((byte & 0x7f) as u64) << shift;

        /* If there is no next */
        if (byte & 0x80) == 0 {
            break;
        }

        shift += 7;
    }

    Ok(output)
}

pub fn write<I: ToPrimitive>(output: &mut BytesMut, i: I) {
    // XXX: Benchmark this, then try to optimize allocations,
    // then benchmark again.
    let mut i = i.to_u64().unwrap();
    while i >= 0x80 {
        output.reserve(1);
        output.put((i & 0x7f) as u8 | 0x80);
        i >>= 7;
    }
    output.reserve(1);
    output.put_u8(i as u8);
}
