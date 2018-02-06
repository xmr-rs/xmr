use num::Num;
use num::cast::NumCast;
use std::io::Cursor;

pub trait Deserialize: Default {
    fn deserialize<'buf, T: Deserializer<'buf>>(deserializer: &'buf mut T) -> Self;
}

pub trait DeserializeBlob: Sized {
    fn deserialize_blob(buf: &mut Cursor<&[u8]>) -> Self;
}

/// A trait to deserialize formats.
pub trait Deserializer<'buf> {
    /// Deserialize a number, be it signed or unsigned.
    fn deserialize_num<T: Num + NumCast + Sized>(&mut self) -> T;

    /// Deserialize a variable-length unsigned integer.
    fn deserialize_uvarint<T: NumCast>(&mut self) -> T;

    /// Deserialize a variable-length signed integer.
    fn deserialize_varint<T: NumCast>(&mut self) -> T;

    /// Deserialize a binary blob.
    fn deserialize_blob<T: DeserializeBlob>(&mut self) -> T;
}
