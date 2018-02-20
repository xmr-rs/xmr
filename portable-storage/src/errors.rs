use std::fmt::{self, Debug, Display, Formatter};

pub type Result<T> = ::std::result::Result<T, UnexpectedEob>;

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
        if $buf.remaining() < $needed {
            return Err($crate::errors::UnexpectedEob { needed: $needed });
        }
    };
}
