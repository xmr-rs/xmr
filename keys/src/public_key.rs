use std::fmt::{self, Debug, Formatter};

use format::{Deserialize, DeserializerStream, Error, Serialize, SerializerStream};

use utils::fmt_byte_slice;

/// Public Key length in bytes.
pub const PUBLIC_KEY_LENGTH: usize = 32;

#[derive(Default, Clone)]
pub struct PublicKey(pub [u8; PUBLIC_KEY_LENGTH]);

impl PublicKey {
    pub fn new() -> PublicKey {
        PublicKey::default()
    }

    pub fn from_bytes<B: AsRef<[u8]>>(bytes: B) -> PublicKey {
        let bytes = bytes.as_ref();
        assert!(bytes.len() == PUBLIC_KEY_LENGTH,
                "invalid public key length");

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


impl AsRef<[u8]> for PublicKey {
    fn as_ref(&self) -> &[u8] {
        &self.0
    }
}

impl Deserialize for PublicKey {
    fn deserialize(deserializer: &mut DeserializerStream) -> Result<Self, Error> {
        deserializer
            .get_blob(PUBLIC_KEY_LENGTH)
            .map(PublicKey::from_bytes)
    }
}

impl Serialize for PublicKey {
    fn serialize(&self, mut serializer: SerializerStream) {
        serializer.put_blob(self.as_bytes())
    }

    fn len(&self) -> usize {
        PUBLIC_KEY_LENGTH
    }
}

impl Debug for PublicKey {
    fn fmt(&self, fmt: &mut Formatter) -> fmt::Result {
        fmt_byte_slice(&self.0, fmt)
    }
}
