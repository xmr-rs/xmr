extern crate serde;

extern crate crypto;

use crypto::{fast_hash, slow_hash};

/// H256 length in bytes.
pub const H256_LENGTH: usize = 32;

/// A 256-bit hash.
#[derive(Debug, Default, Clone, Eq, PartialEq)]
pub struct H256(pub [u8; H256_LENGTH]);

impl H256 {
    pub fn new() -> H256 {
        H256::default()
    }

    pub fn fast_hash<T: AsRef<[u8]>>(input: T) -> H256 {
        H256(fast_hash(input.as_ref()))
    }

    pub fn slow_hash<T: AsRef<[u8]>>(input: T) -> H256 {
        H256(slow_hash(input.as_ref()))
    }

    pub fn from_bytes<B: AsRef<[u8]>>(bytes: B) -> H256 {
        let bytes = bytes.as_ref();
        assert!(bytes.len() == H256_LENGTH, "invalid hash length");

        let mut h = Self::new();
        h.0.clone_from_slice(bytes);
        h
    }

    pub fn as_bytes(&self) -> &[u8] {
        &self.0
    }
}

impl From<[u8; 32]> for H256 {
    fn from(v: [u8; 32]) -> H256 {
        H256(v)
    }
}

impl<'de> serde::Deserialize<'de> for H256 {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
        where D: serde::Deserializer<'de>
    {
        impl<'de> serde::de::Visitor<'de> for H256 {
            type Value = H256;

            fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
                write!(formatter, "a {} bytes slice", H256_LENGTH)
            }

            fn visit_bytes<E>(mut self, v: &[u8]) -> Result<Self::Value, E>
                where E: serde::de::Error {
                if v.len() != H256_LENGTH {
                    Err(E::custom(format!("slice length isn't {} bytes", H256_LENGTH)))
                } else {
                    self.0.copy_from_slice(v);
                    Ok(self)
                }
            }
        }
        deserializer.deserialize_bytes(H256::default())
    }
}

impl serde::Serialize for H256 {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
        where S: serde::Serializer
    {
        serializer.serialize_bytes(&self.0)
    }
}

impl AsRef<[u8]> for H256 {
    fn as_ref(&self) -> &[u8] {
        self.as_bytes()
    }
}
