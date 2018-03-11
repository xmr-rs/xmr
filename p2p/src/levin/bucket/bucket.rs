use std::io;

use levin::{Command, Storage};
use bytes::{Bytes, BytesMut};

use futures::{Future, Poll};
use tokio_io::AsyncWrite;
use tokio_io::io::{WriteAll, write_all};

use levin::bucket::bucket_head::{BucketHead, LEVIN_SIGNATURE, LEVIN_PROTOCOL_VER_1, LEVIN_OK,
                                 LEVIN_PACKET_REQUEST, LEVIN_PACKET_RESPONSE,
                                 BUCKET_HEAD_LENGTH};

use portable_storage;

pub struct Bucket {
    pub head: BucketHead,
    pub body: BytesMut,
}

impl Bucket {
    pub fn request<C>(body: &C::Request) -> Bucket where C: Command, {
        let body_section = body.to_section().expect("invalid portable storage type");
        let mut body_buf = BytesMut::new();
        portable_storage::write(&mut body_buf, &body_section);

        Bucket {
            head: BucketHead {
                signature: LEVIN_SIGNATURE,
                cb: body_buf.len() as u64,
                have_to_return_data: true,
                command: C::ID,
                return_code: LEVIN_OK,
                protocol_version: LEVIN_PROTOCOL_VER_1,
                flags: LEVIN_PACKET_REQUEST,
            },
            body: body_buf,
        }
    }

    pub fn response<C>(body: &C::Response) -> Bucket where C: Command {
        let body_section = body.to_section().expect("invalid portable storage type");
        let mut body_buf = BytesMut::new();
        portable_storage::write(&mut body_buf, &body_section);

        Bucket {
            head: BucketHead {
                signature: LEVIN_SIGNATURE,
                cb: body_buf.len() as u64,
                have_to_return_data: false,
                command: C::ID,
                return_code: LEVIN_OK,
                protocol_version: LEVIN_PROTOCOL_VER_1,
                flags: LEVIN_PACKET_RESPONSE,
            },
            body: body_buf,
        }
    }

    pub fn request_future<A, C>(a: A, body: &C::Request) -> Request<A>
        where A: AsyncWrite,
              C: Command,
    {
        Request {
            future: write_all(a, Self::request::<C>(body).to_bytes())
        }
    }

    pub fn response_future<A, C>(a: A, body: &C::Response) -> Response<A>
        where A: AsyncWrite,
              C: Command,
    {
        Response {
            future: write_all(a, Self::response::<C>(body).to_bytes())
        }
    }

    pub fn to_bytes(self) -> Bytes {
        let mut blob = BytesMut::with_capacity(self.body.len() + BUCKET_HEAD_LENGTH);
        BucketHead::write(&mut blob, &self.head);

        // unsplit is a bad and confusing name for this :(,
        // in this context it means "concatenate".
        blob.unsplit(self.body);

        blob.freeze()
    }
}

pub struct Request<A> {
    future: WriteAll<A, Bytes>,
}

impl<A> Future for Request<A>
    where A: AsyncWrite,
{
    type Item = (A, Bytes);
    type Error = io::Error;

    fn poll(&mut self) -> Poll<Self::Item, Self::Error> {
        self.future.poll()
    }
}

pub struct Response<A> {
    future: WriteAll<A, Bytes>,
}

impl<A> Future for Response<A>
    where A: AsyncWrite,
{
    type Item = (A, Bytes);
    type Error = io::Error;

    fn poll(&mut self) -> Poll<Self::Item, Self::Error> {
        self.future.poll()
    }
}

/// A levin bucket used to send notify commands.
pub fn notify_bucket(command: u32, cb: usize) -> BucketHead {
    BucketHead {
        signature: LEVIN_SIGNATURE,
        cb: cb as u64,
        have_to_return_data: false,
        command,
        return_code: LEVIN_OK,
        protocol_version: LEVIN_PROTOCOL_VER_1,
        flags: LEVIN_PACKET_REQUEST,
    }
}
