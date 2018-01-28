use std::io;

use tokio_io::AsyncRead;
use tokio_io::io::{ReadExact, Read, read_exact, read};
use futures::{Future, Poll};

/// Read a section's entry name.
pub fn read_name<A>(a: A) -> ReadName<A>
    where A: AsyncRead,
{
    ReadName {
        state: ReadNameState::ReadLength {
            reader: read_exact(a, [0u8; 1]),
        },
    }
}

enum ReadNameState<A> {
    /// Read the name length
    ReadLength {
        reader: ReadExact<A, [u8; 1]>,
    },
    /// Read the remaining bytes given a length
    ReadName {
        reader: Read<A, Vec<u8>>,
    },
}

pub struct ReadName<A> {
    state: ReadNameState<A>,
}

impl<A> Future for ReadName<A>
    where A: AsyncRead,
{
    type Item = (A, String);
    type Error = io::Error;

	fn poll(&mut self) -> Poll<Self::Item, Self::Error> {
        loop {
            let next_state = match self.state {
                ReadNameState::ReadLength { ref mut reader } => {
                    let (stream, data) = try_ready!(reader.poll());
                    let buf = vec![0u8; data[0] as usize];
                    ReadNameState::ReadName { reader: read(stream, buf) }
                },
                ReadNameState::ReadName { ref mut reader } => {
                    let (stream, data, length) = try_ready!(reader.poll());
                    // TODO: failure
                    assert_eq!(data.len(), length);
                    return Ok((stream, String::from_utf8_lossy(data.as_slice()).into_owned()).into());
                },
            };
            self.state = next_state;
        }
	}
}
