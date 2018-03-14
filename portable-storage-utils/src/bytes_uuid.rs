use std::fmt;

use uuid;

use serde::de::{Deserialize, Deserializer, Error, Visitor};
use serde::ser::{Serialize, Serializer};

#[derive(Debug, Default, Clone, Eq, PartialEq, Hash)]
pub struct BytesUuid(pub uuid::Uuid);

impl From<uuid::Uuid> for BytesUuid {
    fn from(v: uuid::Uuid) -> BytesUuid {
        BytesUuid(v)
    }
}

impl<'de> Deserialize<'de> for BytesUuid {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
        where D: Deserializer<'de>
    {
        struct UuidVisitor;

        impl<'de> Visitor<'de> for UuidVisitor {
            type Value = BytesUuid;

            fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
                write!(formatter, "an uuid")
            }

            fn visit_bytes<E>(self, v: &[u8]) -> Result<Self::Value, E>
                where E: Error
            {
                uuid::Uuid::from_bytes(v)
                    .map(BytesUuid::from)
                    .map_err(E::custom)
            }
        }

        deserializer.deserialize_bytes(UuidVisitor)
    }
}

impl Serialize for BytesUuid {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
        where S: Serializer
    {
        serializer.serialize_bytes(self.0.as_bytes())
    }
}
