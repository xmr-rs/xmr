use uuid;
use failure::Error;

use portable_storage::errors::InvalidStorageEntry;
use portable_storage::ser::ToUnderlying;
use portable_storage::{Result, StorageEntry};

#[derive(Debug, Default, Clone, Eq, PartialEq, Hash)]
pub struct SerializableUuid(uuid::Uuid);

impl ToUnderlying for SerializableUuid {
    fn to_underlying(entry: &StorageEntry) -> Result<SerializableUuid> {
        match entry {
            &StorageEntry::Buf(ref v) => {
                // TODO: Convert to failure
                Ok(SerializableUuid(uuid::Uuid::from_bytes(&*v).unwrap()))
            }
            _ => Err(Error::from(InvalidStorageEntry::new("StorageEntry::Buf")))
        }
    }
}

impl From<SerializableUuid> for StorageEntry {
    fn from(v: SerializableUuid) -> StorageEntry {
        StorageEntry::Buf(v.0.as_bytes().to_vec())
    }
}
