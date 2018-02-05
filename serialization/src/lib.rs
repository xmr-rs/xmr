extern crate varint;
extern crate bytes;
extern crate num;

use bytes::Bytes;

pub mod serializer;
pub mod binary;
mod macros;

pub fn binary_serialize<T: serializer::Serialize>(v: &T) -> Bytes {
    let mut ser = binary::BinarySerializer::new();
    v.serialize(&mut ser);
    ser.bytes().freeze()
}
