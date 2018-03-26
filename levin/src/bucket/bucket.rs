// Copyright 2018 Jean Pierre Dudey <jeandudey@hotmail.com>
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

use std::io;

use futures::{Future, Poll};
use tokio_io::AsyncRead;
use tokio_io::io::{ReadExact, read_exact};

use bytes::{Bytes, BytesMut, IntoBuf};

use portable_storage::{self, Section};

use bucket::bucket_head::{BucketHead, LEVIN_SIGNATURE, LEVIN_PROTOCOL_VER_1, LEVIN_OK,
                          LEVIN_PACKET_REQUEST, LEVIN_PACKET_RESPONSE, BUCKET_HEAD_LENGTH};

use command::Id;
use error::Result;

/// A levin bucket, this is the packet of information
/// that carries commands in the levin protocol.
///
/// Every bucket starts with a header called [`BucketHead`][1],
/// this header contains information about the bucket, it's size
/// and the command it cointains.
///
/// This structure is just a container of the header and the command
/// stored in a `BytesMut` container.
///
/// [1]: struct.BucketHead.html
#[derive(Debug)]
pub struct Bucket {
    /// The bucket header.
    pub head: BucketHead,
    /// The bucket data.
    pub body: BytesMut,
}

impl Bucket {
    /// Create a bucket used to send a notification.
    pub fn notification(id: Id, request: Section) -> Bucket {
        let mut body_buf = BytesMut::new();
        portable_storage::write(&mut body_buf, &request);

        Bucket {
            head: BucketHead {
                signature: LEVIN_SIGNATURE,
                cb: body_buf.len() as u64,
                have_to_return_data: false,
                command: id,
                return_code: LEVIN_OK,
                protocol_version: LEVIN_PROTOCOL_VER_1,
                flags: LEVIN_PACKET_REQUEST,
            },
            body: body_buf,
        }
    }

    /// Create a bucket used to send an invokation.
    pub fn invokation(id: Id, request: Section) -> Bucket {
        let mut body_buf = BytesMut::new();
        portable_storage::write(&mut body_buf, &request);

        Bucket {
            head: BucketHead {
                signature: LEVIN_SIGNATURE,
                cb: body_buf.len() as u64,
                have_to_return_data: true,
                command: id,
                return_code: LEVIN_OK,
                protocol_version: LEVIN_PROTOCOL_VER_1,
                flags: LEVIN_PACKET_REQUEST,
            },
            body: body_buf,
        }
    }

    /// Create a bucket used to send a command response.
    pub fn invokation_response(id: Id, response: Section) -> Bucket {
        let mut body_buf = BytesMut::new();
        portable_storage::write(&mut body_buf, &response);

        Bucket {
            head: BucketHead {
                signature: LEVIN_SIGNATURE,
                cb: body_buf.len() as u64,
                have_to_return_data: false,
                command: id,
                return_code: LEVIN_OK,
                protocol_version: LEVIN_PROTOCOL_VER_1,
                flags: LEVIN_PACKET_RESPONSE,
            },
            body: body_buf,
        }
    }

    /// Create a bucket used to send an error response.
    pub fn error_response(id: Id, return_code: i32) -> Bucket {
        Bucket {
            head: BucketHead {
                signature: LEVIN_SIGNATURE,
                cb: 0,
                have_to_return_data: false,
                command: id,
                return_code,
                protocol_version: LEVIN_PROTOCOL_VER_1,
                flags: LEVIN_PACKET_RESPONSE,
            },
            body: BytesMut::new(),
        }
    }

    /// Creates a future that will read a bucket from the provided stream.
    pub fn receive_future<A>(a: A) -> Receive<A>
        where A: AsyncRead
    {
        let buf = vec![0u8; BUCKET_HEAD_LENGTH];
        Receive { state: ReceiveState::ReadBucket { reader: read_exact(a, buf) } }
    }

    /// Convert the body of this bucket into a portable storage section.
    pub fn into_section(&self) -> ::std::result::Result<Section, portable_storage::Error> {
        use std::io::Cursor;

        let mut buf = Cursor::new(self.body.as_ref());
        portable_storage::read(&mut buf)
    }

    /// Consumes this bucket and returns a `Bytes` container
    /// with the data of it.
    pub fn to_bytes(self) -> Bytes {
        let mut blob = BytesMut::with_capacity(self.body.len() + BUCKET_HEAD_LENGTH);
        BucketHead::write(&mut blob, &self.head);

        // unsplit is a bad and confusing name for this :(,
        // in this context it means "concatenate".
        blob.unsplit(self.body);

        blob.freeze()
    }
}

/// A future that will receive a bucket.
#[derive(Debug)]
pub struct Receive<A: AsyncRead> {
    state: ReceiveState<A>,
}

#[derive(Debug)]
enum ReceiveState<A> {
    ReadBucket { reader: ReadExact<A, Vec<u8>> },
    ReadStorage {
        bucket_head: BucketHead,
        reader: ReadExact<A, Vec<u8>>,
    },
}

impl<A> Future for Receive<A>
    where A: AsyncRead
{
    type Item = (A, Result<Bucket>);
    type Error = io::Error;

    fn poll(&mut self) -> Poll<Self::Item, Self::Error> {
        loop {
            let next_state = match self.state {
                ReceiveState::ReadBucket { ref mut reader } => {
                    trace!("receive poll - reading bucket");
                    let (stream, buf) = try_ready!(reader.poll());

                    let mut buf = buf.into_buf();
                    let bucket_head = match BucketHead::read(&mut buf) {
                        Ok(b) => b,
                        Err(e) => {
                            return Ok((stream, Err(e)).into());
                        }
                    };

                    trace!("receive poll - bucket received: {:?}", bucket_head);

                    let buf = vec![0u8; bucket_head.cb as usize];
                    ReceiveState::ReadStorage {
                        bucket_head,
                        reader: read_exact(stream, buf),
                    }
                }
                ReceiveState::ReadStorage {
                    ref bucket_head,
                    ref mut reader,
                } => {
                    trace!("receive poll - reading response");

                    let (stream, buf) = try_ready!(reader.poll());

                    let bucket = Bucket {
                        head: bucket_head.clone(),
                        body: buf.into(),
                    };

                    return Ok((stream, Ok(bucket)).into());
                }
            };

            self.state = next_state;
        }
    }
}
