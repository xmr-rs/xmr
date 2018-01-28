use failure::Error;
use bytes::{BytesMut, Buf, ByteOrder};
use portable_storage::{self, Serialize, Deserialize};

pub trait Codec<T: ByteOrder>: Sized {
    fn encode(&self, buf: &mut BytesMut) -> Result<(), Error>;
    fn decode<B: Buf>(buf: &mut B) -> Result<Self, Error>;
}

impl<T, S> Codec<T> for S
    where T: ByteOrder,
          S: Deserialize + Serialize
{
    fn encode(&self, buf: &mut BytesMut) -> Result<(), Error> {
        let section = S::serialize(self);
        Ok(portable_storage::write::<T>(buf, &section))
    }

    fn decode<B: Buf>(buf: &mut B) -> Result<Self, Error> {
        let section = portable_storage::read::<T, B>(buf)?;
        S::deserialize(&section)
    }
}
