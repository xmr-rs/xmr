// Copyright 2018 Jean Pierre Dudey <jeandudey@hotmail.com>
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

use format::{Deserialize, DeserializerStream, Error, Serialize, SerializerStream};

#[derive(Debug, Clone)]
pub struct TxInGen {
    pub height: u64,
}

impl Deserialize for TxInGen {
    fn deserialize(deserializer: &mut DeserializerStream) -> Result<Self, Error> {
        deserializer
            .get_u64_varint()
            .map(|height| TxInGen { height })
    }
}

impl Serialize for TxInGen {
    fn serialize(&self, mut serializer: SerializerStream) {
        serializer.put_u64_varint(self.height)
    }

    fn len(&self) -> usize {
        use varint;

        varint::length(self.height)
    }
}
