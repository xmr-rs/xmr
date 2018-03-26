// Copyright 2018 Jean Pierre Dudey <jeandudey@hotmail.com>
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

extern crate bytes;

extern crate failure;
#[macro_use]
extern crate failure_derive;

extern crate xmr_varint as varint;

mod de;
mod ser;

pub use de::{Deserialize, DeserializerStream, Error, from_binary};
pub use ser::{Serialize, SerializerStream, to_binary};

mod impls;

pub use impls::*;
