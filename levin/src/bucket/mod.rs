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
