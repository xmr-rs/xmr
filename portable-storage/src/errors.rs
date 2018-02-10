use std::fmt::{self, Debug, Display, Formatter};
use failure::Error;

pub type Result<T> = ::std::result::Result<T, Error>;

#[derive(Debug, Clone, Copy, Fail)]
pub struct UnexpectedEob {
    pub needed: usize,
}

impl Display for UnexpectedEob {
    fn fmt(&self, fmt: &mut Formatter) -> fmt::Result {
        if self.needed != 0 {
            write!(fmt, "reached the end of buffer, more data needs to be read ({} bytes)", self.needed)
        } else {
            write!(fmt, "reached the end of buffer, more data needs to be read")
        }
    }
}

#[macro_export]
macro_rules! ensure_eob {
    ($buf:expr, $needed:expr) => {
        ensure!($buf.remaining() >= $needed, $crate::errors::UnexpectedEob { needed: $needed });
    };
}

#[derive(Debug, Clone, Copy, Fail)]
#[fail(display = "unexpected portable-storage entry, expected {}", expected)]
pub struct InvalidStorageEntry {
    pub expected: &'static str,
}

impl InvalidStorageEntry {
    pub fn new(expected: &'static str) -> InvalidStorageEntry {
        InvalidStorageEntry {
            expected
        }
    }
}
