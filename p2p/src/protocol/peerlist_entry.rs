use bytes::{BytesMut, Buf, BufMut, LittleEndian, IntoBuf};
use portable_storage_utils::stl::{StlElement, Error};
use protocol::{Ipv4Address, PeerId};
use std::cmp::{Eq, PartialEq};

#[derive(Debug, Default, Clone)]
pub struct PeerlistEntry {
    pub adr: Ipv4Address,
    pub id: PeerId,
    pub last_seen: i64,
}

impl PartialEq for PeerlistEntry {
    fn eq(&self, other: &PeerlistEntry) -> bool {
        self.id == other.id && self.adr == other.adr
    }
}

impl Eq for PeerlistEntry {}

impl StlElement for PeerlistEntry {
    const LENGTH: usize = Ipv4Address::LENGTH + 8 + 8;

    fn from_bytes(v: &[u8]) -> Result<PeerlistEntry, Error> {
        if v.len() != Self::LENGTH {
            return Err(Error::InvalidLength(v.len()))
        }

        let adr = Ipv4Address::from_bytes(&v[..Ipv4Address::LENGTH])?;

        let mut buf = (&v[Ipv4Address::LENGTH..]).into_buf();

        let id = buf.get_u64::<LittleEndian>().into();
        let last_seen = buf.get_i64::<LittleEndian>();

        Ok(PeerlistEntry { adr, id, last_seen })
    }

    fn to_bytes(&self, buf: &mut BytesMut) {
        buf.reserve(Self::LENGTH);

        self.adr.to_bytes(buf);
        buf.put_u64::<LittleEndian>(self.id.into());
        buf.put_i64::<LittleEndian>(self.last_seen);
    }
}
