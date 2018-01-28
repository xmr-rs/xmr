use std::io;

use bytes::{ByteOrder, Buf, BufMut, BytesMut};

use levin::{LevinResult, LevinError, BucketHeadError};

/// BucketHead signature.
pub const LEVIN_SIGNATURE: u64 = 0x0101010101012101;

/// Identifies a command as a request.
pub const LEVIN_PACKET_REQUEST: u32 = 0x00000001;
/// Identifies a command as a response.
pub const LEVIN_PACKET_RESPONSE: u32 = 0x00000002;

/// Levin maximum packet size. It's default is 100 *MB*.
pub const LEVIN_DEFAULT_MAX_PACKET_SIZE: u64 = 100000000;
  
/// Current levin protocol version.
pub const LEVIN_PROTOCOL_VER_1: u32 = 1;

/// Size in bytes of `BucketHead`.
pub const BUCKET_HEAD_LENGTH: usize = 33;

pub const LEVIN_OK: i32 = 0;
pub const LEVIN_ERROR_CONNECTION: i32 = -1;
pub const LEVIN_ERROR_CONNECTION_NOT_FOUND: i32 = -2;
pub const LEVIN_ERROR_CONNECTION_DESTROYED: i32 = -3;
pub const LEVIN_ERROR_CONNECTION_TIMEDOUT: i32 = -4;
pub const LEVIN_ERROR_CONNECTION_NO_DUPLEX_PROTOCOL: i32 = -5;
pub const LEVIN_ERROR_CONNECTION_HANDLER_NOT_DEFINED: i32 = -6;
pub const LEVIN_ERROR_FORMAT: i32 = -7;

/// Header of all the levin protocol operations.
#[derive(Debug)]
pub struct BucketHead {
    /// This identifies the stream as a valid header.
    pub signature: u64,
    /// Size in bytes of the rest.
    pub cb: u64,
    /// Specifies if a command has to return a response.
    pub have_to_return_data: bool,
    /// The command ID.
    pub command: u32,
    /// The return code.
    pub return_code: i32,
    /// Flags of this header.
    pub flags: u32,
    /// The levin protocol version.
    pub protocol_version: u32,
}

impl BucketHead {
    /// Read a `BucketHead` from a buffer.
    pub fn read<T: ByteOrder, B: Buf>(buf: &mut B) -> LevinResult<Self> {
        if buf.remaining() < BUCKET_HEAD_LENGTH {
            return Err(LevinError::UnexpectedEob);
        }

        let bucket_head = BucketHead {
            signature: buf.get_u64::<T>(),
            cb: buf.get_u64::<T>(),
            have_to_return_data: buf.get_u8() != 0,
            command: buf.get_u32::<T>(),
            return_code: buf.get_i32::<T>(),
            flags: buf.get_u32::<T>(),
            protocol_version: buf.get_u32::<T>(),
        };

        if bucket_head.signature != LEVIN_SIGNATURE { 
            return Err(BucketHeadError::InvalidSignature(bucket_head.signature).into());
        } else if bucket_head.protocol_version != LEVIN_PROTOCOL_VER_1 {
            return Err(BucketHeadError::InvalidProtocolVersion(bucket_head.protocol_version).into());
        } else if bucket_head.cb > LEVIN_DEFAULT_MAX_PACKET_SIZE {
            return Err(BucketHeadError::TooBig(bucket_head.cb).into());
        }

        Ok(bucket_head)
    }

    /// Write a `BucketHead` to a buffer.
    pub fn write<T: ByteOrder>(buf: &mut BytesMut, bucket_head: BucketHead) {
        buf.reserve(BUCKET_HEAD_LENGTH);

        buf.put_u64::<T>(bucket_head.signature);
        buf.put_u64::<T>(bucket_head.cb);
        buf.put_u8(if bucket_head.have_to_return_data { 1u8 } else { 0u8 });
        buf.put_u32::<T>(bucket_head.command);
        buf.put_i32::<T>(bucket_head.return_code);
        buf.put_u32::<T>(bucket_head.flags);
        buf.put_u32::<T>(bucket_head.protocol_version);
    }
}

/// Create a `BucketHead` used for invoke.
pub fn invoke_bucket(command: u32, cb: usize) -> BucketHead {
    BucketHead {
        signature: LEVIN_SIGNATURE,
        cb: cb as u64,
        have_to_return_data: true,
        command,
        return_code: LEVIN_OK,
        protocol_version: LEVIN_PROTOCOL_VER_1,
        flags: LEVIN_PACKET_REQUEST,
    }
}
