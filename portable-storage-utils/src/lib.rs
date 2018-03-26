// Copyright 2018 Jean Pierre Dudey <jeandudey@hotmail.com>
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

extern crate uuid;
extern crate serde;
extern crate bytes;

extern crate failure;
#[macro_use]
extern crate failure_derive;

extern crate xmr_primitives as primitives;

pub mod stl;

mod blob;
mod bytes_uuid;

pub use blob::Blob;
pub use bytes_uuid::BytesUuid;
