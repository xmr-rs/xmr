// Copyright 2018 Jean Pierre Dudey <jeandudey@hotmail.com>
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

use primitives::{H256, H256_LENGTH};
use bytes::{BytesMut, BufMut, Buf, IntoBuf, LittleEndian};

#[derive(Debug, Clone, Fail)]
pub enum Error {
    #[fail(display = "invalid length ({})", _0)]
    InvalidLength(usize),
}

/// A stl container element.
pub trait StlElement: Sized {
    /// The length in bytes of this element.
    const LENGTH: usize;

    // TODO: `v.len()` should always be == to LENGTH
    // making Error::InvalidLength.
    fn from_bytes(v: &[u8]) -> Result<Self, Error>;
    fn to_bytes(&self, buf: &mut BytesMut);
}

impl StlElement for u64 {
    const LENGTH: usize = 8;

    fn from_bytes(v: &[u8]) -> Result<Self, Error> {
        if v.len() != 8 {
            return Err(Error::InvalidLength(v.len()));
        }

        Ok(v.into_buf().get_u64::<LittleEndian>())
    }

    fn to_bytes(&self, buf: &mut BytesMut) {
        buf.put_u64::<LittleEndian>(*self)
    }
}

impl StlElement for H256 {
    const LENGTH: usize = H256_LENGTH;

    fn from_bytes(v: &[u8]) -> Result<Self, Error> {
        if v.len() != H256_LENGTH {
            return Err(Error::InvalidLength(v.len()));
        }

        Ok(H256::from_bytes(v))
    }

    fn to_bytes(&self, buf: &mut BytesMut) {
        buf.put(self.as_bytes())
    }
}
