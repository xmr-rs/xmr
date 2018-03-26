// Copyright 2018 Jean Pierre Dudey <jeandudey@hotmail.com>
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

use primitives::{H256, H256_LENGTH};
use format::{Deserialize, DeserializerStream, Error, Serialize, SerializerStream};

#[derive(Debug, Clone)]
pub struct TxOutToScriptHash {
    pub hash: H256,
}

impl Deserialize for TxOutToScriptHash {
    fn deserialize(deserializer: &mut DeserializerStream) -> Result<Self, Error> {
        deserializer
            .get_deserializable()
            .map(|hash| TxOutToScriptHash { hash })
    }
}

impl Serialize for TxOutToScriptHash {
    fn serialize(&self, mut serializer: SerializerStream) {
        serializer.put_serializable(&self.hash);
    }

    fn len(&self) -> usize {
        H256_LENGTH
    }
}
