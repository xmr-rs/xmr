use num::Num;
use num::cast::NumCast;
use std::io::Cursor;
use std::borrow::Cow;

/// A trait to deserialize formats.
pub trait Deserializer {
    /// Deserialize a number, be it signed or unsigned.
    fn deserialize_num<T: Num + NumCast + Sized>(&mut self) -> T;

    /// Deserialize a variable-length unsigned integer.
    fn deserialize_uvarint<T: NumCast>(&mut self) -> T;

    /// Deserialize a variable-length signed integer.
    fn deserialize_varint<T: NumCast>(&mut self) -> T;

    // TODO: Use TryFrom
    /// Deserialize a binary blob.
    fn deserialize_blob<'a, T: From<&'a Cursor<&'a [u8]>>>(&'a mut self) -> T;

    /// Deserialize a tag.
    fn deserialize_tag<'a>(&'a mut self) -> Option<Cow<'a, str>>;
}
