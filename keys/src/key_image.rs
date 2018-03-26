// Copyright 2018 Jean Pierre Dudey <jeandudey@hotmail.com>
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

use std::fmt::{self, Debug, Formatter};

use format::{Deserialize, DeserializerStream, Error, Serialize, SerializerStream};

use utils::fmt_byte_slice;

/// Key image length.
pub const KEY_IMAGE_LENGTH: usize = 32;

#[derive(Default, Clone)]
pub struct KeyImage(pub [u8; KEY_IMAGE_LENGTH]);

impl KeyImage {
    pub fn new() -> KeyImage {
        KeyImage::default()
    }

    pub fn from_bytes<B: AsRef<[u8]>>(bytes: B) -> KeyImage {
        let bytes = bytes.as_ref();
        assert!(bytes.len() == KEY_IMAGE_LENGTH, "invalid key image length");

        let mut h = Self::new();
        h.0.clone_from_slice(bytes);
        h
    }

    pub fn as_bytes(&self) -> &[u8] {
        &self.0
    }
}

impl AsRef<[u8]> for KeyImage {
    fn as_ref(&self) -> &[u8] {
        &self.0
    }
}

impl Deserialize for KeyImage {
    fn deserialize(deserializer: &mut DeserializerStream) -> Result<Self, Error> {
        deserializer
            .get_blob(KEY_IMAGE_LENGTH)
            .map(KeyImage::from_bytes)
    }
}

impl Serialize for KeyImage {
    fn serialize(&self, mut serializer: SerializerStream) {
        serializer.put_blob(self.as_bytes())
    }

    fn len(&self) -> usize {
        KEY_IMAGE_LENGTH
    }
}

impl Debug for KeyImage {
    fn fmt(&self, fmt: &mut Formatter) -> fmt::Result {
        fmt_byte_slice(&self.0, fmt)
    }
}
