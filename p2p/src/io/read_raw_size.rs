use std::io;
use std::marker::PhantomData;

use bytes::ByteOrder;
use tokio_io::AsyncRead;
use tokio_io::io::{ReadExact, read_exact};
use futures::{Future, Poll};

use portable_storage::raw_size::{
    PORTABLE_RAW_SIZE_MARK_MASK,
    PORTABLE_RAW_SIZE_MARK_BYTE,
    PORTABLE_RAW_SIZE_MARK_WORD,
    PORTABLE_RAW_SIZE_MARK_DWORD,
    PORTABLE_RAW_SIZE_MARK_INT64,
};

/// Read a variable-sized unsigned integer.
pub fn read_raw_size<A, E>(a: A) -> ReadRawSize<A, E>
    where A: AsyncRead,
          E: ByteOrder,
{
    ReadRawSize {
        state: ReadRawSizeState::ReadMark {
            reader: read_exact(a, [0u8; 1]),
        },
        _endian: PhantomData,
    }
}

enum ReadRawSizeState<A> {
    /// Read the byte containing the mark.
    ReadMark {
        reader: ReadExact<A, [u8; 1]>,
    },
    /// Read the remaining byte for a `u16`.
    ReadOne {
        first_byte: u8,
        reader: ReadExact<A, [u8; 1]>,
    },
    /// Read the remaining three bytes for a `u32`.
    ReadThree {
        first_byte: u8,
        reader: ReadExact<A, [u8; 3]>,
    },
    /// Read the remaining seven bytes for a `u64`.
    ReadSeven {
        first_byte: u8,
        reader: ReadExact<A, [u8; 7]>,
    }
}

pub struct ReadRawSize<A, E> {
    state: ReadRawSizeState<A>,
    _endian: PhantomData<E>,
}

impl<A, E> Future for ReadRawSize<A, E>
    where A: AsyncRead,
          E: ByteOrder,
{
    type Item = (A, usize);
    type Error = io::Error;

	fn poll(&mut self) -> Poll<Self::Item, Self::Error> {
        loop {
            let next_state = match self.state {
                ReadRawSizeState::ReadMark { ref mut reader } => {
                    let (stream, data) = try_ready!(reader.poll());
                    let mark = data[0] & PORTABLE_RAW_SIZE_MARK_MASK;
                    match mark {
                        PORTABLE_RAW_SIZE_MARK_BYTE => {
                            return Ok((stream, (data[0] >> 2) as usize).into())
                        },
                        PORTABLE_RAW_SIZE_MARK_WORD => {
                            ReadRawSizeState::ReadOne {
                                first_byte: data[0],
                                reader: read_exact(stream, [0u8; 1]),
                            }
                        },
                        PORTABLE_RAW_SIZE_MARK_DWORD => {
                            ReadRawSizeState::ReadThree {
                                first_byte: data[0],
                                reader: read_exact(stream, [0u8; 3]),
                            }
                        },
                        PORTABLE_RAW_SIZE_MARK_INT64 => {
                            ReadRawSizeState::ReadSeven {
                                first_byte: data[0],
                                reader: read_exact(stream, [0u8; 7]),
                            }
                        },
                        _ => unreachable!(),
                    }
                },
                ReadRawSizeState::ReadOne { ref first_byte, ref mut reader } => {
                    let (stream, data) = try_ready!(reader.poll());
                    let mut buf = [0u8; 2];
                    buf[0] = *first_byte;
                    buf[1] = data[0];
                    return Ok((stream, (E::read_u16(&buf) >> 2) as usize).into());
                },
                ReadRawSizeState::ReadThree { ref first_byte, ref mut reader } => {
                    let (stream, data) = try_ready!(reader.poll());
                    let mut buf = [0u8; 4];
                    buf[0] = *first_byte;
                    buf[1] = data[0];
                    buf[2] = data[1];
                    buf[3] = data[2];
                    return Ok((stream, (E::read_u32(&buf) >> 2) as usize).into());
                },
                ReadRawSizeState::ReadSeven { ref first_byte, ref mut reader } => {
                    let (stream, data) = try_ready!(reader.poll());
                    let mut buf = [0u8; 4];
                    buf[0] = *first_byte;
                    buf[1] = data[0];
                    buf[2] = data[1];
                    buf[3] = data[2];
                    buf[4] = data[3];
                    buf[5] = data[4];
                    buf[6] = data[5];
                    buf[7] = data[6];
                    return Ok((stream, (E::read_u64(&buf) >> 2) as usize).into());
                },
            };
            self.state = next_state;
        }
	}
}
