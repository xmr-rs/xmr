// Copyright 2018 Jean Pierre Dudey <jeandudey@hotmail.com>
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

use std::fmt;

use serde::de::{Deserialize, Deserializer, Error, Visitor};
use serde::ser::{Serialize, Serializer};

#[derive(Debug, Default, Clone, Eq, PartialEq, Hash)]
pub struct Blob(pub Vec<u8>);

impl From<Vec<u8>> for Blob {
    fn from(v: Vec<u8>) -> Blob {
        Blob(v)
    }
}

impl From<&'static [u8]> for Blob {
    fn from(v: &'static [u8]) -> Blob {
        Blob(v.to_vec())
    }
}

impl<'de> Deserialize<'de> for Blob {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
        where D: Deserializer<'de>
    {
        struct BlobVisitor;

        impl<'de> Visitor<'de> for BlobVisitor {
            type Value = Blob;

            fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
                write!(formatter, "a binary blob")
            }

            fn visit_bytes<E>(self, v: &[u8]) -> Result<Self::Value, E>
                where E: Error
            {
                Ok(Blob(v.to_vec()))
            }
        }

        deserializer.deserialize_bytes(BlobVisitor)
    }
}

impl Serialize for Blob {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
        where S: Serializer
    {
        serializer.serialize_bytes(self.0.as_slice())
    }
}
