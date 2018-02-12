use uuid;

use portable_storage::ser::{ToUnderlying, Error, invalid_storage_entry};
use portable_storage::StorageEntry;

#[derive(Debug, Default, Clone, Eq, PartialEq, Hash)]
pub struct SerializableUuid(pub uuid::Uuid);

impl From<uuid::Uuid> for SerializableUuid {
    fn from(v: uuid::Uuid) -> SerializableUuid {
        SerializableUuid(v)
    }
}

impl ToUnderlying for SerializableUuid {
    fn to_underlying(entry: &StorageEntry) -> Result<SerializableUuid, Error> {
        match *entry {
            StorageEntry::Buf(ref v) => {
                // TODO: Convert to failure
                Ok(SerializableUuid(uuid::Uuid::from_bytes(&*v).unwrap()))
            }
            _ => Err(invalid_storage_entry("StorageEntry::Buf"))
        }
    }
}

impl From<SerializableUuid> for StorageEntry {
    fn from(v: SerializableUuid) -> StorageEntry {
        StorageEntry::Buf(v.0.as_bytes().to_vec())
    }
}
