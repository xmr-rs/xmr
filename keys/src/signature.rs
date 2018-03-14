use std::fmt::{self, Debug, Formatter};

use utils::fmt_byte_slice;

/// Signature lenght.
pub const SIGNATURE_LENGTH: usize = 64;

pub struct Signature([u8; SIGNATURE_LENGTH]);

impl Signature {
    pub fn new() -> Signature {
        Signature([0u8; 64])
    }

    pub fn from_bytes<B: AsRef<[u8]>>(bytes: B) -> Signature {
        let bytes = bytes.as_ref();
        assert!(bytes.len() == SIGNATURE_LENGTH, "invalid signature length");

        let mut h = Self::new();
        h.0.clone_from_slice(bytes);
        h
    }

    pub fn as_bytes(&self) -> &[u8] {
        &self.0
    }
}

impl Clone for Signature {
    fn clone(&self) -> Signature {
        let mut s = Signature::new();
        s.0.copy_from_slice(&self.0);
        s
    }
}

impl Debug for Signature {
    fn fmt(&self, fmt: &mut Formatter) -> fmt::Result {
        fmt_byte_slice(&self.0, fmt)
    }
}
