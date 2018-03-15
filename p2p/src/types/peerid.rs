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
