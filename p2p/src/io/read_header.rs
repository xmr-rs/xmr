use tokio_io::AsyncRead;
use tokio_io::io::{ReadExact, read_exact};
use futures::{Future, Poll, Async};
use portable_storage::header::{
    PORTABLE_STORAGE_BLOCK_HEADER_LENGTH as HEADER_LENGTH,
    StorageBlockHeader
};
use bytes::{ByteOrder, IntoBuf};
use std::marker::PhantomData;
use std::io;

/// Read a portable storage block header.
pub fn read_header<A, E>(a: A) -> ReadHeader<A, E>
    where A: AsyncRead,
          E: ByteOrder,
{
    ReadHeader {
        reader: read_exact(a, [0u8; HEADER_LENGTH]),
        _endian: PhantomData,
    }
}

#[derive(Debug)]
pub struct ReadHeader<A, E> {
    reader: ReadExact<A, [u8; HEADER_LENGTH]>,
    _endian: PhantomData<E> 
}

impl<A, E> Future for ReadHeader<A, E>
    where A: AsyncRead,
          E: ByteOrder,
{
    type Item = (A, StorageBlockHeader);
    type Error = io::Error;

	fn poll(&mut self) -> Poll<Self::Item, Self::Error> {
		let (read, data) = try_ready!(self.reader.poll());
        let mut data = data.into_buf();
		let header = StorageBlockHeader::read::<E, _>(&mut data).unwrap();
		Ok(Async::Ready((read, header)))
	}
}
