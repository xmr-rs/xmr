use num::Num;
use num::cast::ToPrimitive;

pub trait Serialize {
    fn serialize<T: Serializer>(&self, serializer: &mut T);
}

/// Serializer trait.
pub trait Serializer {
    /// Serialize a number, be it signed or unsigned.
    fn serialize_num<T: Num + ToPrimitive + Sized>(&mut self, v: T);

    /// Serialize an variable-length unsigned integer.
    fn serialize_uvarint<I: ToPrimitive>(&mut self, v: I);

    /// Serialize a variable-length signed integer.
    fn serialize_varint<I: ToPrimitive>(&mut self, v: I);

    /// Serialize the field name.
    fn tag(&mut self, tag: &str);
}
