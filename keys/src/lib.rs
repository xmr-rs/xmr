extern crate serialization;
extern crate bytes;

use std::io::Cursor;
use bytes::Buf;

use serialization::serializer::{Serialize, Serializer};
use serialization::deserializer::{Deserialize, DeserializeBlob, Deserializer};


/// Key image length.
pub const KEY_IMAGE_LENGTH: usize = 32;
/// Public Key length in bytes.
pub const PUBLIC_KEY_LENGTH: usize = 32;
/// Secret Key length in bytes.
pub const SECRET_KEY_LENGTH: usize = 32;

#[derive(Debug, Default, Clone)]
pub struct KeyImage(pub [u8; KEY_IMAGE_LENGTH]);

impl AsRef<[u8]> for KeyImage {
    fn as_ref(&self) -> &[u8] {
        &self.0
    }
}

impl DeserializeBlob for KeyImage {
    fn deserialize_blob(v: &mut Cursor<&[u8]>) -> KeyImage {
        let mut key = KeyImage::default();
        v.copy_to_slice(&mut key.0);
        key
    }
}

impl Serialize for KeyImage {
    fn serialize<T: Serializer>(&self, serializer: &mut T) {
        serializer.serialize_blob(self);
    }
}

impl Deserialize for KeyImage {
    fn deserialize<T: Deserializer>(deserializer: &mut T) -> Self {
        deserializer.deserialize_blob()
    }
}

#[derive(Debug, Default, Clone)]
pub struct Signature;

#[derive(Debug, Default, Clone)]
pub struct PublicKey(pub [u8; PUBLIC_KEY_LENGTH]);

impl AsRef<[u8]> for PublicKey {
    fn as_ref(&self) -> &[u8] {
        &self.0
    }
}

impl DeserializeBlob for PublicKey {
    fn deserialize_blob(v: &mut Cursor<&[u8]>) -> PublicKey {
        let mut key = PublicKey::default();
        v.copy_to_slice(&mut key.0);
        key
    }
}

impl Serialize for PublicKey {
    fn serialize<T: Serializer>(&self, serializer: &mut T) {
        serializer.serialize_blob(self);
    }
}

impl Deserialize for PublicKey {
    fn deserialize<T: Deserializer>(deserializer: &mut T) -> Self {
        deserializer.deserialize_blob()
    }
}

#[derive(Debug, Default, Clone)]
pub struct SecretKey(pub [u8; SECRET_KEY_LENGTH]);

impl AsRef<[u8]> for SecretKey {
    fn as_ref(&self) -> &[u8] {
        &self.0
    }
}

impl DeserializeBlob for SecretKey {
    fn deserialize_blob(v: &mut Cursor<&[u8]>) -> SecretKey {
        let mut key = SecretKey::default();
        v.copy_to_slice(&mut key.0);
        key
    }
}

impl Serialize for SecretKey {
    fn serialize<T: Serializer>(&self, serializer: &mut T) {
        serializer.serialize_blob(self);
    }
}

impl Deserialize for SecretKey {
    fn deserialize<T: Deserializer>(deserializer: &mut T) -> Self {
        deserializer.deserialize_blob()
    }
}
