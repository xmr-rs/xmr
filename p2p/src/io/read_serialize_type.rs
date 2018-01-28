use std::io;

use tokio_io::AsyncRead;
use tokio_io::io::{ReadExact, read_exact};
use futures::{Future, Poll};

// TODO: Strong typing?

/// Read the serialize type for a portable storage entry.
pub fn read_serialize_type<A>(a: A) -> ReadSerializeType<A>
    where A: AsyncRead,
{
    ReadSerializeType {
        reader: read_exact(a, [0u8; 1]),
    }
}

#[derive(Debug)]
pub struct ReadSerializeType<A> {
    reader: ReadExact<A, [u8; 1]>,
}

impl<A> Future for ReadSerializeType<A>
    where A: AsyncRead,
{
    type Item = (A, u8);
    type Error = io::Error;

	fn poll(&mut self) -> Poll<Self::Item, Self::Error> {
        let (stream, data) = try_ready!(self.reader.poll());
        Ok((stream, data[0]).into())
	}
}
