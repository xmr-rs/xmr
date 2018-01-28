use bytes::{BytesMut, Buf, BufMut, ByteOrder};

use portable_storage::ser::bytes::SerializeBytes;
use portable_storage::ser::Serializable;
use portable_storage::Result;

use protocol::Ipv4Address;
use levin::DefaultEndian;

pub type PeerId = u64;

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
        buf.put_u64::<T>(self.id);
        buf.put_i64::<T>(self.last_seen);
    }

    fn from_bytes<T: ByteOrder, B: Buf>(buf: &mut B) -> Result<PeerlistEntryBase<A>> {
        let adr = A::from_bytes::<T, B>(buf)?;
        assert!(buf.remaining() >= 16);
        let id = buf.get_u64::<T>();
        let last_seen= buf.get_i64::<T>();
        Ok(PeerlistEntryBase { adr, id, last_seen })
    }
}
