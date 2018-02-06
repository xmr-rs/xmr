use num::Num;
use num::cast::ToPrimitive;

/// A trait to serialize a structure.
pub trait Serialize {
    fn serialize<T: Serializer>(&self, serializer: &mut T);
}

/// A trait to serialize a structure to a given format.
pub trait Serializer {
    /// Serialize a number, be it signed or unsigned.
    fn serialize_num<T: Num + ToPrimitive + Sized>(&mut self, v: T);

    /// Serialize an variable-length unsigned integer.
    fn serialize_uvarint<I: ToPrimitive>(&mut self, v: I);

    /// Serialize a variable-length signed integer.
    fn serialize_varint<I: ToPrimitive>(&mut self, v: I);

    /// Serialize a binary blob.
    fn serialize_blob<T: AsRef<[u8]>>(&mut self, v: &T);
}
