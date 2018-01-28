extern crate portable_storage;
extern crate failure;

use portable_storage::ser::ToUnderlying;
use portable_storage::errors::InvalidStorageEntry;
use portable_storage::{Result, StorageEntry};
use failure::Error;

/// H256 length in bytes.
pub const H256_LENGTH: usize = 32;

/// A 256-bit hash.
#[derive(Debug, Default, Clone)]
pub struct H256(pub [u8; H256_LENGTH]);

impl H256 {
    pub fn new() -> H256 {
        H256::default()
    }

    pub fn from_bytes<B: AsRef<[u8]>>(bytes: &B) -> H256 {
        let bytes = bytes.as_ref();
        assert!(bytes.len() == H256_LENGTH, "invalid hash length");

        let mut h = Self::new();
        h.0.clone_from_slice(bytes);
        h
    }

    pub fn as_bytes(&self) -> &[u8; H256_LENGTH] {
        &self.0
    }
}

impl ToUnderlying for H256 {
    fn to_underlying(entry: &StorageEntry) -> Result<H256> {
        match entry {
            &StorageEntry::Buf(ref v) => {
                // TODO: Add error handling, this panics on invalid slice length
                Ok(H256::from_bytes(v))
            }
            _ => Err(Error::from(InvalidStorageEntry::new("StorageEntry::Buf")))
        }
    }
}

impl From<H256> for StorageEntry {
    fn from(v: H256) -> StorageEntry {
        StorageEntry::Buf(v.as_bytes().to_vec())
    }
}
