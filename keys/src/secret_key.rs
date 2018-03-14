use std::fmt::{self, Debug, Formatter};

use utils::fmt_byte_slice;

/// Secret Key length in bytes.
pub const SECRET_KEY_LENGTH: usize = 32;

#[derive(Default, Clone)]
pub struct SecretKey(pub [u8; SECRET_KEY_LENGTH]);

impl AsRef<[u8]> for SecretKey {
    fn as_ref(&self) -> &[u8] {
        &self.0
    }
}

impl Debug for SecretKey {
    fn fmt(&self, fmt: &mut Formatter) -> fmt::Result {
        fmt_byte_slice(&self.0, fmt)
    }
}
