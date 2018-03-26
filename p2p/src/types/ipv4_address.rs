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

use std::net;
use bytes::{BytesMut, Buf, BufMut, IntoBuf, LittleEndian};

use portable_storage_utils::stl::{StlElement, Error};

/// An IPv4 address
#[derive(Debug, Default, Clone, Eq, PartialEq, Hash)]
pub struct Ipv4Address {
    pub ip: u32,
    pub port: u16,
}

impl From<net::SocketAddrV4> for Ipv4Address {
    fn from(addr: net::SocketAddrV4) -> Ipv4Address {
        Ipv4Address {
            ip: addr.ip().clone().into(),
            port: addr.port(),
        }
    }
}

impl<'a> From<&'a net::SocketAddrV4> for Ipv4Address {
    fn from(addr: &'a net::SocketAddrV4) -> Ipv4Address {
        Ipv4Address {
            ip: addr.ip().clone().into(),
            port: addr.port(),
        }
    }
}

impl StlElement for Ipv4Address {
    const LENGTH: usize = 4 + 2;

    fn from_bytes(v: &[u8]) -> Result<Ipv4Address, Error> {
        if v.len() != Self::LENGTH {
            return Err(Error::InvalidLength(v.len()));
        }

        let mut buf = v.into_buf();

        Ok(Ipv4Address {
               ip: buf.get_u32::<LittleEndian>(),
               port: buf.get_u16::<LittleEndian>(),
           })
    }

    fn to_bytes(&self, buf: &mut BytesMut) {
        buf.reserve(Self::LENGTH);
        buf.put_u32::<LittleEndian>(self.ip);
        buf.put_u16::<LittleEndian>(self.port);
    }
}
