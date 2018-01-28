use std::fmt::{self, Debug, Display, Formatter};
use failure::{Fail, Error};

pub type Result<T> = ::std::result::Result<T, Error>;

#[derive(Clone, Copy)]
pub struct UnexpectedEob {
    pub needed: usize,
}

impl Debug for UnexpectedEob {
    fn fmt(&self, fmt: &mut Formatter) -> fmt::Result {
        if self.needed != 0 {
            write!(fmt, "unexpected end of buffer, needed {} bytes", self.needed)
        } else {
            write!(fmt, "unexpected end of buffer")
        }
    }
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

impl Fail for UnexpectedEob { }

#[macro_export]
macro_rules! ensure_eob {
    ($buf:expr, $needed:expr) => {
        ensure!($buf.remaining() >= $needed, $crate::errors::UnexpectedEob { needed: $needed });
    };
}

#[derive(Clone, Copy)]
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

impl Debug for InvalidStorageEntry {
    fn fmt(&self, fmt: &mut Formatter) -> fmt::Result {
        write!(fmt, "unexpected portable-storage entry, expected {}", self.expected)
    }
}

impl Display for InvalidStorageEntry {
    fn fmt(&self, fmt: &mut Formatter) -> fmt::Result {
        write!(fmt, "unexpected portable-storage entry, expected {}", self.expected)
    }
}

impl Fail for InvalidStorageEntry { }
