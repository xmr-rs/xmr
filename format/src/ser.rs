use bytes::{BufMut, Bytes, BytesMut};
use varint;

pub fn to_binary<T: Serialize>(v: &T) -> Bytes {
    let mut bytes = BytesMut::new();
    bytes.reserve(v.len());
    v.serialize(SerializerStream::new(&mut bytes));
    bytes.freeze()
}

pub trait Serialize {
    fn serialize(&self, serializer: SerializerStream);
    fn len(&self) -> usize;
}

pub struct SerializerStream<'buf>(&'buf mut BytesMut);

impl<'buf> SerializerStream<'buf> {
    pub fn new(bytes: &'buf mut BytesMut) -> SerializerStream<'buf> {
        SerializerStream(bytes)
    }

    pub fn put_u8(&mut self, v: u8) {
        self.0.put_u8(v);
    }

    pub fn put_u32(&mut self, mut v: u32) {
        use std::mem::size_of;

        let bytes = size_of::<u32>();
        for _ in 0..bytes {
            self.0.put_u8((v & 0xff) as u8);
            v >>= 8;
        }
    }

    pub fn put_u64(&mut self, mut v: u64) {
        use std::mem::size_of;

        let bytes = size_of::<u64>();
        for _ in 0..bytes {
            self.0.put_u8((v & 0xff) as u8);
            v >>= 8;
        }
    }

    pub fn put_u8_varint(&mut self, v: u8) {
        varint::write(self.0, v)
    }

    pub fn put_u64_varint(&mut self, v: u64) {
        varint::write(self.0, v)
    }

    pub fn put_blob(&mut self, v: &[u8]) {
        self.0.put(v)
    }

    pub fn put_serializable<T: Serialize>(&mut self, v: &T) {
        v.serialize(SerializerStream::new(self.0))
    }
}
