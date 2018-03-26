// Copyright 2018 Jean Pierre Dudey <jeandudey@hotmail.com>
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

//! Buckets
//!
//! Buckets are the packet of information that the levin protocol use
//! to send and receive commands.

mod bucket;
mod bucket_head;

pub use self::bucket::{Bucket, Receive};
pub use self::bucket_head::{BucketHead, LEVIN_SIGNATURE, LEVIN_PACKET_REQUEST,
                            LEVIN_PACKET_RESPONSE, LEVIN_PROTOCOL_VER_1, BUCKET_HEAD_LENGTH,
                            LEVIN_OK, LEVIN_DEFAULT_MAX_PACKET_SIZE};
