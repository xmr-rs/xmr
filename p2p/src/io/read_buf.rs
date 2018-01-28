use std::io;

use bytes::ByteOrder;
use tokio_io::AsyncRead;
use tokio_io::io::{Read, read};
use futures::{Future, Poll};

use io::{ReadRawSize, read_raw_size};

/// Read a portable storage buffer entry.
pub fn read_buf<A, E>(a: A) -> ReadBuf<A, E>
    where A: AsyncRead,
          E: ByteOrder,
{
    ReadBuf {
        state: ReadBufState::ReadLength {
            future: read_raw_size(a),
        },
    }
}

enum ReadBufState<A, E> {
    /// Read the buffer length
    ReadLength {
        future: ReadRawSize<A, E>,
    },
    /// Read the buffer
    ReadBuffer {
        reader: Read<A, Vec<u8>>,
    },
}

pub struct ReadBuf<A, E> {
    state: ReadBufState<A, E>,
}

impl<A, E> Future for ReadBuf<A, E>
    where A: AsyncRead,
          E: ByteOrder,
{
    type Item = (A, Vec<u8>);
    type Error = io::Error;

	fn poll(&mut self) -> Poll<Self::Item, Self::Error> {
        loop {
            let next_state = match self.state {
                ReadBufState::ReadLength { ref mut future } => {
                    let (stream, length) = try_ready!(future.poll());
                    let buf = vec![0u8; length];
                    ReadBufState::ReadBuffer { reader: read(stream, buf) }
                },
                ReadBufState::ReadBuffer { ref mut reader } => {
                    let (stream, buffer, count) = try_ready!(reader.poll());
                    assert_eq!(buffer.len(), count); // TODO: Error
                    return Ok((stream, buffer).into());
                },
            };
            self.state = next_state;
        }
	}
}
