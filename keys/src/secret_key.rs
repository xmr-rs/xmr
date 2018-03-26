// Copyright 2018 Jean Pierre Dudey <jeandudey@hotmail.com>
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

use std::fmt::{self, Debug, Formatter};

use utils::fmt_byte_slice;

/// Secret Key length in bytes.
pub const SECRET_KEY_LENGTH: usize = 32;

#[derive(Default, Clone)]
pub struct SecretKey(pub [u8; SECRET_KEY_LENGTH]);

impl AsRef<[u8]> for SecretKey {
    fn as_ref(&self) -> &[u8] {
        &self.0
    }
}

impl Debug for SecretKey {
    fn fmt(&self, fmt: &mut Formatter) -> fmt::Result {
        fmt_byte_slice(&self.0, fmt)
    }
}
