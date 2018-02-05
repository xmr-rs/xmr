//! # varint
//! XZ variable length integers reading/writing

extern crate bytes;
extern crate num;

use std::mem::size_of;

use bytes::{BytesMut, Buf, BufMut};
use num::cast::ToPrimitive;

/// An error occurred during reading.
#[derive(Debug)]
pub enum ReadError {
    /// The integer is too large to fit in the current type.
    Overflow,
    /// The integer cannot be represented.
    Represent,
}

pub fn read<B: Buf>(buf: &mut B) -> Result<usize, ReadError> {
    let bits = size_of::<usize>() * 8;
    let mut output = 0usize;
    let mut shift = 0usize;
    loop {
        let byte = buf.get_u8();
        
        if shift + 7 >= bits && byte >= 1 << (bits - shift) {
            return Err(ReadError::Overflow);
        }

        if byte == 0 && shift != 0 {
            return Err(ReadError::Represent);
        }
        
        // Does the actualy placing into output, stripping the first bit
        output |= ((byte & 0x7f) as usize) << shift as usize;

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
