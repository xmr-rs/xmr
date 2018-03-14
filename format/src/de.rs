use std::fmt::Display;
use bytes::{IntoBuf, Buf};
use varint;

pub fn from_binary<T: Deserialize>(v: &[u8]) -> Result<T, Error> {
    let mut deserializer = DeserializerStream::new(v);
    T::deserialize(&mut deserializer)
}

pub trait Deserialize: Sized {
    fn deserialize(stream: &mut DeserializerStream) -> Result<Self, Error>;
}

#[derive(Debug, Fail)]
pub enum Error {
    #[fail(display = "couldn't read varint: {}", _0)]
    VarInt(varint::ReadError),
    #[fail(display = "unexpected EOF, needed {}", _0)]
    UnexpectedEof(usize),
    #[fail(display = "{}", _0)]
    Custom(String),
}

impl Error {
    pub fn custom<D: Display>(error: D) -> Error {
        Error::Custom(format!("{}", error))
    }
}

#[doc(hidden)]
impl From<varint::ReadError> for Error {
    fn from(e: varint::ReadError) -> Error {
        Error::VarInt(e)
    }
}

#[derive(Debug)]
pub struct DeserializerStream<'buf>(<&'buf [u8] as IntoBuf>::Buf);

impl<'buf> DeserializerStream<'buf> {
    fn new(v: &'buf [u8]) -> DeserializerStream<'buf> {
        DeserializerStream(v.into_buf())
    }

    pub fn get_u8(&mut self) -> Result<u8, Error> {
        if self.0.remaining() < 1 {
            Err(Error::UnexpectedEof(1))
        } else {
            Ok(self.0.get_u8())
        }
    }

    pub fn get_u32(&mut self) -> Result<u32, Error> {
        use std::mem::size_of;

        let bytes = size_of::<u32>();

        if self.0.remaining() < bytes {
            Err(Error::UnexpectedEof(bytes))
        } else {
            let mut res = 0;
            let mut shift = 0;
            for _ in 0..bytes {
                res += (self.0.get_u8() as u32) << shift;
                shift += 8;
            }
            Ok(res)
        }
    }

    pub fn get_u8_varint(&mut self) -> Result<u8, Error> {
        varint::read(&mut self.0)
            .map_err(Error::from)
            .and_then(|v| if v > u8::max_value() as u64 {
                          Err(varint::ReadError::Overflow.into())
                      } else {
                          Ok(v as u8)
                      })
    }

    pub fn get_u64_varint(&mut self) -> Result<u64, Error> {
        varint::read(&mut self.0).map_err(Error::from)
    }

    pub fn get_blob(&mut self, length: usize) -> Result<Vec<u8>, Error> {
        if self.0.remaining() < length {
            return Err(Error::UnexpectedEof(length));
        }

        let blob = (&(self.0.bytes()[..length])).to_vec();

        self.0.advance(length);

        Ok(blob)
    }

    pub fn get_deserializable<T: Deserialize>(&mut self) -> Result<T, Error> {
        T::deserialize(self)
    }
}
