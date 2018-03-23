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

/// Read a varint.
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

/// Calcuate how many bytes a varint occupies in memory.
pub fn length<I: ToPrimitive>(i: I) -> usize {
    let mut i = i.to_u64().unwrap();
    let mut count = 1;
    while i >= 0x80 {
        count += 1;
        i >>= 7;
    }
    count
}

/// Write an integer as a varint.
pub fn write<I: ToPrimitive>(output: &mut BytesMut, i: I) {
    let mut i = i.to_u64().unwrap();
    while i >= 0x80 {
        output.put((i & 0x7f) as u8 | 0x80);
        i >>= 7;
    }
    output.put_u8(i as u8);
}

#[cfg(test)]
pub mod tests {
    use super::*;
    use std::u16::MAX;
    use bytes::{BytesMut, IntoBuf};

    #[test]
    fn read_write_is_equal() {
        let mut write_buf = BytesMut::with_capacity(64);
        for input in 0..MAX {
            write(&mut write_buf, input as u16);

            {
                let mut read_buf = write_buf.as_ref().into_buf();
                let output = read(&mut read_buf).expect("reading should be fine") as u16;
                assert_eq!(input, output);
            }

            write_buf.clear();
        }
    }
}
