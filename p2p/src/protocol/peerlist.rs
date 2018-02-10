use bytes::{BytesMut, Buf, BufMut, ByteOrder};

use rand::Rng;

use portable_storage::ser::bytes::SerializeBytes;
use portable_storage::ser::{Serializable, ToUnderlying, Error, invalid_storage_entry};
use portable_storage::StorageEntry;

use protocol::Ipv4Address;
use levin::DefaultEndian;

#[derive(Debug, Default, Clone, Copy, Eq, PartialEq, Hash)]
pub struct PeerId(u64);

impl PeerId {
    pub fn random<R: Rng>(rng: &mut R) -> PeerId {
        PeerId(rng.next_u64())
    }
}

impl Into<u64> for PeerId {
    fn into(self) -> u64 {
        self.0
    }
}

impl From<u64> for PeerId {
    fn from(v: u64) -> PeerId {
        PeerId(v)
    }
}

impl From<PeerId> for StorageEntry {
    fn from(v: PeerId) -> StorageEntry {
        StorageEntry::U64(v.into())
    }
}

impl ToUnderlying for PeerId {
    fn to_underlying(entry: &StorageEntry) -> Result<PeerId, Error> {
        match *entry {
            StorageEntry::U64(ref v) => Ok(PeerId::from(*v)),
            _ => Err(invalid_storage_entry("StorageEntry::U64"))
        }
    }
}

pub type PeerlistEntry = PeerlistEntryBase<Ipv4Address>;

#[derive(Debug, Default, Clone)]
pub struct PeerlistEntryBase<A: Serializable + SerializeBytes> {
    pub adr: A,
    pub id: PeerId,
    pub last_seen: i64,
}

serializable! {
    PeerlistEntryBase<A> where (A: Serializable + SerializeBytes) {
        adr,
        id,
        last_seen,
    }
}

impl<A: Serializable + SerializeBytes> SerializeBytes for PeerlistEntryBase<A> {
    fn to_bytes<T: ByteOrder>(&self, buf: &mut BytesMut) {
        self.adr.to_bytes::<T>(buf);
        buf.reserve(16);
        buf.put_u64::<T>(self.id.into());
        buf.put_i64::<T>(self.last_seen);
    }

    fn from_bytes<T: ByteOrder, B: Buf>(buf: &mut B) -> Result<PeerlistEntryBase<A>, ::failure::Error> {
        let adr = A::from_bytes::<T, B>(buf)?;
        assert!(buf.remaining() >= 16);
        let id = buf.get_u64::<T>().into();
        let last_seen= buf.get_i64::<T>();
        Ok(PeerlistEntryBase { adr, id, last_seen })
    }
}
