use std::io;

use bytes::BytesMut;
use futures::{Future, Poll};

use tokio_io::AsyncWrite;
use tokio_io::io::{WriteAll, write_all};

use portable_storage;

use levin::{
    BUCKET_HEAD_LENGTH,
    BucketHead,
    Command,
    Storage,
    response_bucket,
};

pub fn response<A, C>(a: A, response: C::Response) -> Response<A>
    where A: AsyncWrite,
          C: Command, {
    trace!("response - creating future");
    let section = response.to_section().unwrap();

    let mut response_buf = BytesMut::new();
    portable_storage::write(&mut response_buf, &section);

    let mut buf = BytesMut::with_capacity(BUCKET_HEAD_LENGTH);
    BucketHead::write(&mut buf, response_bucket(C::ID, response_buf.len()));

    // XXX: unsplit is a bad and confusing name. In this context it mean's
    // "concatenate".
    buf.unsplit(response_buf);

    Response {
        writer: write_all(a, buf),
    }
}

#[derive(Debug)]
pub struct Response<A: AsyncWrite> {
    writer: WriteAll<A, BytesMut>
}

impl<A> Future for Response<A>
    where A: AsyncWrite,
{
    type Item = A;
    type Error = io::Error;

    fn poll(&mut self) -> Poll<Self::Item, Self::Error> {
        trace!("response poll - writing");
        let (stream, _) = try_ready!(self.writer.poll());
        Ok(stream.into())
    }
}
