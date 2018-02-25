extern crate varint;
extern crate bytes;
extern crate num;

use bytes::Bytes;

pub mod de;
pub mod ser;
pub mod binary;
mod macros;

pub fn binary_serialize<T: serializer::Serialize>(v: &T) -> Bytes {
    let mut ser = binary::BinarySerializer::new();
    v.serialize(&mut ser);
    ser.bytes().freeze()
}

pub fn binary_deserialize<T: deserializer::Deserialize, B: AsRef<[u8]>>(buf: &B) -> T {
    let mut deser = binary::BinaryDeserializer::new(buf);
    T::deserialize(&mut deser)
}
