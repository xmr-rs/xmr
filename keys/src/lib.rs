use std::fmt;

/// Key image length.
pub const KEY_IMAGE_LENGTH: usize = 32;
/// Signature lenght.
pub const SIGNATURE_LENGTH: usize = 64;
/// Public Key length in bytes.
pub const PUBLIC_KEY_LENGTH: usize = 32;
/// Secret Key length in bytes.
pub const SECRET_KEY_LENGTH: usize = 32;

#[derive(Default, Clone)]
pub struct KeyImage(pub [u8; KEY_IMAGE_LENGTH]);

impl KeyImage {
    pub fn new() -> KeyImage {
        KeyImage::default()
    }

    pub fn from_bytes<B: AsRef<[u8]>>(bytes: B) -> KeyImage {
        let bytes = bytes.as_ref();
        assert!(bytes.len() == KEY_IMAGE_LENGTH, "invalid key image length");

        let mut h = Self::new();
        h.0.clone_from_slice(bytes);
        h
    }

    pub fn as_bytes(&self) -> &[u8] {
        &self.0
    }
}

impl AsRef<[u8]> for KeyImage {
    fn as_ref(&self) -> &[u8] {
        &self.0
    }
}

impl fmt::Debug for KeyImage {
    fn fmt(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
        fmt_byte_slice(&self.0, fmt)
    }
}

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

impl fmt::Debug for Signature {
    fn fmt(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
        fmt_byte_slice(&self.0, fmt)
    }
}

impl Clone for Signature {
    fn clone(&self) -> Signature {
        let mut s = Signature::new();
        s.0.copy_from_slice(&self.0);
        s
    }
}

#[derive(Default, Clone)]
pub struct PublicKey(pub [u8; PUBLIC_KEY_LENGTH]);

impl PublicKey {
    pub fn new() -> PublicKey {
        PublicKey::default()
    }

    pub fn from_bytes<B: AsRef<[u8]>>(bytes: B) -> PublicKey {
        let bytes = bytes.as_ref();
        assert!(bytes.len() == PUBLIC_KEY_LENGTH, "invalid public key length");

        let mut h = Self::new();
        h.0.clone_from_slice(bytes);
        h
    }

    pub fn as_bytes(&self) -> &[u8] {
        &self.0
    }
}

impl From<[u8; PUBLIC_KEY_LENGTH]> for PublicKey {
    fn from(key: [u8; PUBLIC_KEY_LENGTH]) -> PublicKey {
        PublicKey(key)
    }
}

impl fmt::Debug for PublicKey {
    fn fmt(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
        fmt_byte_slice(&self.0, fmt)
    }
}

impl AsRef<[u8]> for PublicKey {
    fn as_ref(&self) -> &[u8] {
        &self.0
    }
}

#[derive(Default, Clone)]
pub struct SecretKey(pub [u8; SECRET_KEY_LENGTH]);

impl AsRef<[u8]> for SecretKey {
    fn as_ref(&self) -> &[u8] {
        &self.0
    }
}

impl fmt::Debug for SecretKey {
    fn fmt(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
        fmt_byte_slice(&self.0, fmt)
    }
}

fn fmt_byte_slice(slice: &[u8], fmt: &mut fmt::Formatter) -> fmt::Result {
    write!(fmt, "\"")?;

    for b in slice.iter() {
        write!(fmt, "{:02x}", b)?;
    }

    write!(fmt, "\"")
}

#[cfg(test)]
pub mod tests {
    use super::*;

    use std::fmt;

    #[test]
    fn fmt_byte_slice_() {
        struct Blob([u8; 32]);
        impl fmt::Debug for Blob {
            fn fmt(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
                fmt_byte_slice(&self.0, fmt)
            }
        }

        let blob = Blob([0u8; 32]);


        let blob_str = format!("{:?}", blob);

        assert_eq!(&*blob_str,
                   "\"0000000000000000000000000000000000000000000000000000000000000000\"");
    }
}
