use std::io;

use futures::{Future, Poll};
use tokio_io:: AsyncWrite;
use tokio_io::io::{WriteAll, write_all};
use bytes::BytesMut;

use portable_storage;

use levin::{Notify, Storage};
use levin::bucket::{BucketHead, notify_bucket};

pub fn notify<N, A>(a: A, request: &N::Request) -> NotifyFuture<A>
    where N: Notify,
          A: AsyncWrite,
{
    trace!("notify - creating future");

    let section = request.to_section().unwrap();

    trace!("notify - section created: {:?}", section);

    let mut notify_buf = BytesMut::new();
    portable_storage::write(&mut notify_buf, &section);

    let mut buf = BytesMut::new();
    BucketHead::write(&mut buf, &notify_bucket(N::ID, notify_buf.len()));
    buf.unsplit(notify_buf);

    NotifyFuture {
        writer: write_all::<A, _>(a, buf),
    }
}

pub struct NotifyFuture<A> {
    writer: WriteAll<A, BytesMut>,
}

impl<A> Future for NotifyFuture<A> where A: AsyncWrite {
    type Item = A;
    type Error = io::Error;

    fn poll(&mut self) -> Poll<Self::Item, Self::Error> {
        trace!("notify poll - writing");
        let (stream, _) = try_ready!(self.writer.poll());
        Ok(stream.into())
    }
}
