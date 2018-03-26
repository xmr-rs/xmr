// Copyright 2018 Jean Pierre Dudey <jeandudey@hotmail.com>
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

use keys::KeyImage;
use format::{Deserialize, DeserializerStream, Error, Serialize, SerializerStream};

#[derive(Debug, Clone)]
pub struct TxInToKey {
    pub amount: u64,
    pub key_offsets: Vec<u64>,
    pub k_image: KeyImage,
}

impl Deserialize for TxInToKey {
    fn deserialize(deserializer: &mut DeserializerStream) -> Result<Self, Error> {
        let amount = deserializer.get_u64_varint()?;

        let key_offsets_length = deserializer.get_u64_varint()? as usize;
        let mut key_offsets = Vec::with_capacity(key_offsets_length);
        for _ in 0..key_offsets_length {
            key_offsets.push(deserializer.get_u64_varint()?);
        }

        let k_image = deserializer.get_deserializable()?;

        Ok(TxInToKey {
               amount,
               key_offsets,
               k_image,
           })
    }
}

impl Serialize for TxInToKey {
    fn serialize(&self, mut serializer: SerializerStream) {
        serializer.put_u64_varint(self.amount);

        serializer.put_u64_varint(self.key_offsets.len() as u64);
        for offset in self.key_offsets.iter() {
            serializer.put_u64_varint(*offset);
        }

        serializer.put_serializable(&self.k_image)
    }

    fn len(&self) -> usize {
        use varint;

        let mut sum = 0;
        sum += varint::length(self.amount);
        sum += varint::length(self.key_offsets.len());
        for offset in self.key_offsets.iter() {
            sum += varint::length(*offset);
        }
        sum += self.k_image.len();
        sum
    }
}
