use bytes::{BytesMut, Buf, BufMut, ByteOrder};

use portable_storage::ser::bytes::SerializeBytes;
use portable_storage::errors::UnexpectedEob;

/// An IPv4 address
#[derive(Debug, Default, Clone, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub struct Ipv4Address {
    pub ip: u32,
    pub port: u16,
}

serializable! {
    Ipv4Address {
        ip,
        port,
    }
}

impl SerializeBytes for Ipv4Address {
    fn to_bytes<T: ByteOrder>(&self, buf: &mut BytesMut) {
        buf.reserve(6);
        buf.put_u32::<T>(self.ip);
        buf.put_u16::<T>(self.port);
    }

    fn from_bytes<T: ByteOrder, B: Buf>(buf: &mut B) -> Result<Ipv4Address, UnexpectedEob> {
        ensure_eob!(buf, 6);

        Ok(Ipv4Address {
            ip: buf.get_u32::<T>(),
            port: buf.get_u16::<T>(),
        })
    }
}
