use std::io::Cursor;
use std::mem::size_of;

use num::Num;
use num::cast::{ToPrimitive, NumCast};
use bytes::{BytesMut, Buf, BufMut};
use varint;

use serializer::{Serialize, Serializer};
use deserializer::{Deserialize, Deserializer, DeserializeBlob};

/// A serilaizer to serialize structures to binary.
#[derive(Debug)]
pub struct BinarySerializer {
    pub bytes: BytesMut,
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

    fn serialize_array<T: Serialize, A: AsRef<[T]>>(&mut self, v: &A) {
        let v = v.as_ref();
        self.serialize_uvarint(v.len());
        for elem in v.iter() {
            self.serialize_struct(elem);
        }
    }
}

/// A serilaizer to serialize structures to binary.
#[derive(Debug)]
pub struct BinaryDeserializer<'buf> {
    pub bytes: Cursor<&'buf [u8]>,
}

impl<'buf> BinaryDeserializer<'buf> {
    /// Creates a new serializer.
    pub fn new<B: AsRef<[u8]>>(bytes: &'buf B) -> BinaryDeserializer<'buf> {
        BinaryDeserializer {
            bytes: Cursor::new(bytes.as_ref()),
        }
    }
}

impl<'buf> Deserializer for BinaryDeserializer<'buf> {
    fn deserialize_num<T: Num + NumCast + Sized>(&mut self) -> T {
        let size = size_of::<T>();
        let mut ret = 0u64;
        let mut shift= 0u64;
        for _ in 0..size {
            let b = self.bytes.get_u8() as u64;

            ret += b << shift;
            shift += 8;
        }

        // TODO: When NumCast::from returns None? what can i do about it?
        T::from(ret).unwrap()
    }

    fn deserialize_uvarint<T: NumCast>(&mut self) -> T {
        // TODO: double-kill, remove these unwraps and add a sane API that returns errors.
        T::from(varint::read(&mut self.bytes).unwrap()).unwrap()
    }

    fn deserialize_varint<T: NumCast>(&mut self) -> T {
        // TODO: unwrap unwrap unwrap.
        T::from(varint::read(&mut self.bytes).unwrap()).unwrap()
    }

    fn deserialize_blob<T: DeserializeBlob>(&mut self) -> T
    {
        // TODO: error error error.
        T::deserialize_blob(&mut self.bytes)
    }

    fn deserialize_array<T: Deserialize>(&mut self) -> Vec<T> {
        let len = self.deserialize_uvarint();
        let mut v = Vec::with_capacity(len);
        for _ in 0..len {
            v.push(self.deserialize_struct());
        }
        v
    }
}
