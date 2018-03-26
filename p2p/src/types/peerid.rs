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

use std::fmt::{self, Formatter, Debug, Display};

use rand::Rng;

use serde::de::{Deserialize, Deserializer, Error, Visitor};
use serde::ser::{Serialize, Serializer};

#[derive(Default, Clone, Copy, Eq, PartialEq, Hash)]
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

impl<'de> Deserialize<'de> for PeerId {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
        where D: Deserializer<'de>
    {
        struct PeerIdVisitor;

        impl<'de> Visitor<'de> for PeerIdVisitor {
            type Value = PeerId;

            fn expecting(&self, formatter: &mut Formatter) -> fmt::Result {
                write!(formatter, "a peer id")
            }

            fn visit_u64<E>(self, v: u64) -> Result<Self::Value, E>
                where E: Error
            {
                Ok(PeerId(v))
            }
        }

        deserializer.deserialize_u64(PeerIdVisitor)
    }
}

impl Serialize for PeerId {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
        where S: Serializer
    {
        serializer.serialize_u64(self.0)
    }
}

impl Debug for PeerId {
    fn fmt(&self, fmt: &mut Formatter) -> fmt::Result {
        write!(fmt, "PeerId(0x{:08x})", self.0)
    }
}

impl Display for PeerId {
    fn fmt(&self, fmt: &mut Formatter) -> fmt::Result {
        write!(fmt, "{:08x}", self.0)
    }
}
