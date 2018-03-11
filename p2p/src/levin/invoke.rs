use std::io;

use futures::{Future, Poll};
use tokio_io:: AsyncWrite;
use tokio_io::io::{WriteAll, write_all};
use bytes::BytesMut;

use portable_storage;

use levin::{
    Command,
    Storage,
    BucketHead,
    invoke_bucket,
};

/// Invoke a command.
pub fn invoke<C, A>(a: A, request: &C::Request) -> Invoke<A>
    where C: Command,
          A: AsyncWrite,
{
    trace!("invoke - creating future");

    let section = request.to_section().unwrap();

    trace!("invoke - section created: {:?}", section);

    let mut command_buf = BytesMut::new();
    portable_storage::write(&mut command_buf, &section);

    let mut buf = BytesMut::new();
    BucketHead::write(&mut buf, &invoke_bucket(C::ID, command_buf.len()));
    buf.unsplit(command_buf);

    Invoke {
        writer: write_all::<A, _>(a, buf),
    }
}

pub struct Invoke<A> {
    writer: WriteAll<A, BytesMut>,
}

impl<A> Future for Invoke<A> where A: AsyncWrite {
    type Item = A;
    type Error = io::Error;

    fn poll(&mut self) -> Poll<Self::Item, Self::Error> {
        trace!("invoke poll - writing");
        let (stream, _) = try_ready!(self.writer.poll());
        Ok(stream.into())
    }
}
