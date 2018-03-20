use std::io;
use std::mem::replace;

use futures::{Future, Poll, Async};
use futures::stream::Stream;

use tokio_io::AsyncRead;

use bucket::{Bucket, Receive};
use error::Result;

/// Creates the bucket stream.
pub fn bucket_stream<A>(a: A) -> BucketStream<A>
    where A: AsyncRead
{
    BucketStream { future: Bucket::receive_future(a) }
}

/// A stream of buckets.
#[derive(Debug)]
pub struct BucketStream<A: AsyncRead> {
    future: Receive<A>,
}

impl<A> Stream for BucketStream<A>
    where A: AsyncRead
{
    type Item = Result<Bucket>;
    type Error = io::Error;

    fn poll(&mut self) -> Poll<Option<Self::Item>, io::Error> {
        let (stream, result) = try_ready!(self.future.poll());

        replace(&mut self.future, Bucket::receive_future(stream));

        Ok(Async::Ready(Some(result)))
    }
}
