// Copyright 2018 Jean Pierre Dudey <jeandudey@hotmail.com>
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

use de::{Deserialize, DeserializerStream, Error};
use ser::{Serialize, SerializerStream};

impl Deserialize for u64 {
    fn deserialize(deserializer: &mut DeserializerStream) -> Result<u64, Error> {
        deserializer.get_u64()
    }
}

impl Serialize for u64 {
    fn serialize(&self, mut serializer: SerializerStream) {
        serializer.put_u64(*self)
    }

    fn len(&self) -> usize {
        8
    }
}
