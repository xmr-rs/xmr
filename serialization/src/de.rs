use num::Num;
use num::cast::NumCast;
use std::io::Cursor;

pub trait Deserialize: Default {
    fn deserialize<T: Deserializer>(deserializer: &mut T) -> Self;
}

pub trait DeserializeBlob: Sized {
    fn deserialize_blob(buf: &mut Cursor<&[u8]>) -> Self;
}

/// A trait to deserialize formats.
pub trait Deserializer: Sized {
    /// Deserialize a number, be it signed or unsigned.
    fn deserialize_num<T: Num + NumCast + Sized>(&mut self) -> T;

    /// Deserialize a variable-length unsigned integer.
    fn deserialize_uvarint<T: NumCast>(&mut self) -> T;

    /// Deserialize a variable-length signed integer.
    fn deserialize_varint<T: NumCast>(&mut self) -> T;

    /// Deserialize a binary blob.
    fn deserialize_blob<T: DeserializeBlob>(&mut self) -> T;

    /// Deserialize an array.
    fn deserialize_array<T: Deserialize>(&mut self) -> Vec<T>;

    /// Deserialize an struct that implements `Deserialize`.
    fn deserialize_struct<T: Deserialize>(&mut self) -> T {
        T::deserialize(self)
    }
}

macro_rules! impl_deserialize_num {
    ($ty:ty) => {
        impl Deserialize for $ty {
            fn deserialize<T: Deserializer>(deserializer: &mut T) -> $ty {
                deserializer.deserialize_num::<$ty>()
            }
        }
    }
}

impl_deserialize_num!(u8);
impl_deserialize_num!(i8);
impl_deserialize_num!(u16);
impl_deserialize_num!(i16);
impl_deserialize_num!(u32);
impl_deserialize_num!(i32);
impl_deserialize_num!(u64);
impl_deserialize_num!(i64);
impl_deserialize_num!(usize);
impl_deserialize_num!(isize);
