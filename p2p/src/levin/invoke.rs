use std::io;
use std::marker::PhantomData;

use futures::{Future, Poll, Async};
use tokio_io::{AsyncRead, AsyncWrite};
use tokio_io::io::{ReadExact, Read, WriteAll, write_all, read_exact, read};
use bytes::{BytesMut, ByteOrder, IntoBuf, Buf};

use portable_storage::{self, Serialize, Deserialize};

use levin::{
    BUCKET_HEAD_LENGTH,
    Command,
    BucketHead,
    LevinResult,
    LevinError,
    invoke_bucket,
};

/// Invoke a command.
pub fn invoke<C, A, E>(a: A, request: &C::Request) -> Invoke<C, A, E>
    where C: Command,
          A: AsyncRead + AsyncWrite,
          E: ByteOrder,
{
    let section = request.serialize();
    let mut command_buf = BytesMut::new();
    portable_storage::write::<E>(&mut command_buf, &section);

    let mut buf = BytesMut::new();
    BucketHead::write::<E>(&mut buf, invoke_bucket(C::ID, command_buf.len()));
    buf.unsplit(command_buf);

    Invoke {
        state: InvokeState::WriteRequest {
            writer: write_all::<A, _>(a, buf),
        },
        _phantom: PhantomData,
    }
}

pub struct Invoke<C, A, E> {
    state: InvokeState<A>,
    _phantom: PhantomData<(C, E)>,
}

enum InvokeState<A> {
    WriteRequest {
        writer: WriteAll<A, BytesMut>
    },
    ReadResponseBucket {
        reader: Read<A, Vec<u8>>,
    },
    ReadResponseBody {
        reader: Read<A, Vec<u8>>,
    }
}

impl<C, A, E> Future for Invoke<C, A, E>
    where C: Command,
          A: AsyncRead + AsyncWrite,
          E: ByteOrder,
{
    type Item = (A, LevinResult<C::Response>);
    type Error = io::Error;

    fn poll(&mut self) -> Poll<Self::Item, Self::Error> {
        loop {
            let next_state = match self.state {
                InvokeState::WriteRequest { ref mut writer } => {
                    let (stream, _) = try_ready!(writer.poll());
                    // TODO: read_exact doesn't work with [u8; 33].
                    InvokeState::ReadResponseBucket {
                        reader: read(stream, vec![0u8; BUCKET_HEAD_LENGTH]),
                    }
                },
                InvokeState::ReadResponseBucket { ref mut reader } => {
                    let (stream, data, length) = try_ready!(reader.poll());
                    if data.len() != length {
                        return Ok((stream, Err(LevinError::UnfinishedRead(data.len()))).into());
                    }
                    let mut buf = data.into_buf();
                    match BucketHead::read::<E, _>(&mut buf) {
                        Ok(bucket_head) => {
                            let buf = vec![0u8; bucket_head.cb as usize];
                            if bucket_head.command != C::ID {
                                return Ok((stream, Err(LevinError::InvalidCommandId(bucket_head.command))).into());
                            }

                            // TODO: add other verifications.

                            InvokeState::ReadResponseBody {
                                reader: read(stream, buf),
                            }
                        },
                        Err(e) => return Ok((stream, Err(e)).into()),
                    }
                },
                InvokeState::ReadResponseBody { ref mut reader } => {
                    let (stream, data, length) = try_ready!(reader.poll());
                    if data.len() != length {
                        return Ok((stream, Err(LevinError::UnfinishedRead(data.len()))).into());
                    }

                    let mut buf = data.into_buf();
                    let section = portable_storage::read::<E, _>(&mut buf).unwrap();
                    let response = C::Response::deserialize(&section).unwrap();

                    return Ok((stream, Ok(response)).into());
                }
            };
            self.state = next_state;
        }
    }
}
