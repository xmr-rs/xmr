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

macro_rules! impl_serialize_num {
    ($ty:ty) => {
        impl Serialize for $ty {
            fn serialize<T: Serializer>(&self, serializer: &mut T) {
                serializer.serialize_num(*self)
            }
        }
    }
}

impl_serialize_num!(u8);
impl_serialize_num!(i8);
impl_serialize_num!(u16);
impl_serialize_num!(i16);
impl_serialize_num!(u32);
impl_serialize_num!(i32);
impl_serialize_num!(u64);
impl_serialize_num!(i64);
impl_serialize_num!(usize);
impl_serialize_num!(isize);
