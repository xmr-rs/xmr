// Copyright 2018 Jean Pierre Dudey <jeandudey@hotmail.com>
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

use keys::{PublicKey, PUBLIC_KEY_LENGTH};
use format::{Deserialize, DeserializerStream, Error, Serialize, SerializerStream};

#[derive(Debug, Clone)]
pub struct TxOutToKey {
    pub key: PublicKey,
}

impl Deserialize for TxOutToKey {
    fn deserialize(deserializer: &mut DeserializerStream) -> Result<Self, Error> {
        deserializer
            .get_deserializable()
            .map(|key| TxOutToKey { key })
    }
}

impl Serialize for TxOutToKey {
    fn serialize(&self, mut serializer: SerializerStream) {
        serializer.put_serializable(&self.key);
    }

    fn len(&self) -> usize {
        PUBLIC_KEY_LENGTH
    }
}
