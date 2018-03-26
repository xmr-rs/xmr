// Xmr, Monero node.
// Copyright (C) 2018  Jean Pierre Dudey
//
// This program is free software: you can redistribute it and/or modify
// it under the terms of the GNU General Public License as published by
// the Free Software Foundation, either version 3 of the License, or
// (at your option) any later version.
//
// This program is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
// GNU General Public License for more details.
//
// You should have received a copy of the GNU General Public License
// along with this program.  If not, see <http://www.gnu.org/licenses/>.

use bytes::{BytesMut, Buf, BufMut, LittleEndian, IntoBuf};
use portable_storage_utils::stl::{StlElement, Error};
use types::{Ipv4Address, PeerId};
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
            return Err(Error::InvalidLength(v.len()));
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
