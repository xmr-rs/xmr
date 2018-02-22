use bytes::BytesMut;

#[derive(Debug, Clone, Fail)]
pub enum Error {
    #[fail(display = "invalid length ({})", _0)]
    InvalidLength(usize),
}

/// A stl container element.
pub trait StlElement: Sized {
    /// The length in bytes of this element.
    const LENGTH: usize;

    fn from_bytes(v: &[u8]) -> Result<Self, Error>;
    fn to_bytes(&self, buf: &mut BytesMut);
}

mod linked_list;

pub use self::linked_list::StlLinkedList;
