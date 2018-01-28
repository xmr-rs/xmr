use std::slice::{Iter, IterMut};

#[derive(Debug)]
pub enum ReadError {
    /// The varint is too large to fit in the current type.
    Overflow,
    /// The varint cannot be represented
    Represent,
}

pub trait VarInt: Sized {
    fn read_varint(buf: &mut Iter<u8>) -> Result<(Self, usize), ReadError>;
    fn write_varint(output: &mut IterMut<u8>, i: Self) -> Result<usize, ()>;
}


macro_rules! impl_varint {
    ($ty:ty) => {
        impl VarInt for $ty {
            fn read_varint(buf: &mut Iter<u8>) -> Result<($ty, usize), ReadError> {
                let bits = std::mem::size_of::<$ty>() * 8;
                let mut output: $ty = 0;
                let mut read = 0;
                let mut shift = 0usize;
                loop {
                    let byte = if let Some(byte) = buf.next() {
                        byte
                    } else {
                        return Ok((0, read));
                    };
                    
                    read += 1;
                    if shift + 7 >= bits && *byte >= 1 << (bits - shift) {
                        return Err(ReadError::Overflow);
                    }

                    if *byte == 0 && shift != 0 {
                        return Err(ReadError::Represent);
                    }
                    
                    // Does the actualy placing into output, stripping the first bit
                    output |= ((byte & 0x7f) as $ty) << shift as $ty;

                    /* If there is no next */
                    if (byte & 0x80) == 0 {
                        break;
                    }

                    shift += 7;
                }

                Ok((output, read))
            }

            fn write_varint(output: &mut IterMut<u8>, mut i: $ty) -> Result<usize, ()> {
                let mut count = 0;
                while i >= 0x80 {
                    let byte = if let Some(b) = output.next() { b } else { return Err(()) };

                    *byte = (i as u8 & 0x7f) | 0x80;
                    count += 1;
                    i >>= 7;
                }

                let byte = if let Some(b) = output.next() { b } else { return Err(()) };
                *byte = i as u8;
                count += 1;
                Ok(count)
            }
        }
    }
}

impl_varint!(u8);
impl_varint!(u16);
impl_varint!(u32);
impl_varint!(u64);
impl_varint!(usize);

#[test]
pub fn test_varint() {
    for idx in 0..65537u64 {
        let mut buf = [0u8; 12];
        let (idx, write) = u64::write_varint(&mut buf.iter_mut()).unwrap();
        assert!(write <= 12);

        let mut idx2 = 0u64;
        let read = u64::read_varint(&mut buf.iter(), &mut idx2).unwrap();
        assert!(read == write);
        assert!(idx2 == idx)
    }
}
