/// Key image length.
pub const KEY_IMAGE_LENGTH: usize = 32;
/// Signature lenght.
pub const SIGNATURE_LENGTH: usize = 64;
/// Public Key length in bytes.
pub const PUBLIC_KEY_LENGTH: usize = 32;
/// Secret Key length in bytes.
pub const SECRET_KEY_LENGTH: usize = 32;

#[derive(Debug, Default, Clone)]
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

#[derive(Debug, Default, Clone)]
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

impl AsRef<[u8]> for PublicKey {
    fn as_ref(&self) -> &[u8] {
        &self.0
    }
}

#[derive(Debug, Default, Clone)]
pub struct SecretKey(pub [u8; SECRET_KEY_LENGTH]);

impl AsRef<[u8]> for SecretKey {
    fn as_ref(&self) -> &[u8] {
        &self.0
    }
}
