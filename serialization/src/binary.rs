use num::Num;
use num::cast::ToPrimitive;
use bytes::{BytesMut, BufMut};
use varint;

use serializer::Serializer;

/// A serilaizer to serialize structures to binary.
#[derive(Debug)]
pub struct BinarySerializer {
    pub bytes: BytesMut
}

impl BinarySerializer {
    /// Creates a new serializer.
    pub fn new() -> BinarySerializer {
        BinarySerializer {
            bytes: BytesMut::new(),
        }
    }

    /// Take the underlying `BytesMut`.
    pub fn bytes(self) -> BytesMut {
        self.bytes
    }
}

impl Serializer for BinarySerializer {
    fn serialize_num<T: Num + ToPrimitive + Sized>(&mut self, v: T) {
        use std::mem::size_of;
        let size = size_of::<T>();

        let mut v = v.to_u64().unwrap();
        self.bytes.reserve(size);
        for _ in 0..size {
            self.bytes.put_u8((v & 0xff) as u8);
            if 1 < size { v >>= 8 }
        }
    }

    fn serialize_uvarint<T: ToPrimitive>(&mut self, v: T) {
        varint::write(&mut self.bytes, v);
    }

    fn serialize_varint<T: ToPrimitive>(&mut self, v: T) {
        self.serialize_uvarint(v);
    }

    fn serialize_blob<T: AsRef<[u8]>>(&mut self, v: &T) {
        self.bytes.extend_from_slice(v.as_ref());
    }

    fn serialize_tag(&mut self, _tag: &str) {}
}
