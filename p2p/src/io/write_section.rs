use std::io;
use std::mem::replace;

use bytes::{ByteOrder, BytesMut};
use tokio_io::AsyncWrite;
use tokio_io::io::{WriteAll, write_all};
use futures::{Future, Poll};

use portable_storage::{self, Section};

/// Write a portable storage section.
pub fn write_section<A, E>(a: A, section: Section) -> WriteSection<A>
    where A: AsyncWrite,
          E: ByteOrder,
{
    let mut bytes = BytesMut::new();
    portable_storage::write::<E>(&mut bytes, &section);

    WriteSection {
        section,
        writer: write_all(a, bytes)
    }
}

pub struct WriteSection<A> {
    section: Section,
    writer: WriteAll<A, BytesMut>,
}

impl<A> Future for WriteSection<A>
    where A: AsyncWrite,
{
    type Item = (A, Section);
    type Error = io::Error;

	fn poll(&mut self) -> Poll<Self::Item, Self::Error> {
        let (stream, _) = try_ready!(self.writer.poll());

        Ok((stream, replace(&mut self.section, Section::new())).into())
	}
}
