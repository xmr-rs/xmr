use primitives::{H256, H256_LENGTH};
use bytes::{BytesMut, BufMut};

#[derive(Debug, Clone, Fail)]
pub enum Error {
    #[fail(display = "invalid length ({})", _0)]
    InvalidLength(usize),
}

/// A stl container element.
pub trait StlElement: Sized {
    /// The length in bytes of this element.
    const LENGTH: usize;

    // TODO: `v.len()` should always be == to LENGTH
    // making Error::InvalidLength.
    fn from_bytes(v: &[u8]) -> Result<Self, Error>;
    fn to_bytes(&self, buf: &mut BytesMut);
}

impl StlElement for H256 {
    const LENGTH: usize = H256_LENGTH;

    fn from_bytes(v: &[u8]) -> Result<Self, Error> {
        if v.len() != H256_LENGTH {
            return Err(Error::InvalidLength(v.len()));
        }

        Ok(H256::from_bytes(v))
    }

    fn to_bytes(&self, buf: &mut BytesMut) {
        buf.put(self.as_bytes())
    }
}
