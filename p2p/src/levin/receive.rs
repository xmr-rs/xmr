use std::io;
use std::marker::PhantomData;

use bytes::{ByteOrder, IntoBuf};
use futures::{Future, Poll};

use tokio_io::AsyncRead;
use tokio_io::io::{Read, read};

use portable_storage::{self, Deserialize};

use levin::{
    BUCKET_HEAD_LENGTH,
    BucketHead,
    Command,
    LevinResult,
    LevinError,
};

pub fn receive<A, E, C>(a: A) -> Receive<A, E, C>
    where A: AsyncRead,
          E: ByteOrder,
          C: Command, {
    let buf = vec![0u8; BUCKET_HEAD_LENGTH];
    Receive {
        state: ReceiveState::ReadBucket {
            reader: read(a, buf),
        },
        _phantom_data: PhantomData,
    }
}

#[derive(Debug)]
pub struct Receive<A: AsyncRead, E: ByteOrder, C: Command> {
    state: ReceiveState<A>,
    _phantom_data: PhantomData<(E, C)>,
}

#[derive(Debug)]
enum ReceiveState<A> {
    ReadBucket {
        reader: Read<A, Vec<u8>>,
    },
    ReadResponse {
        reader: Read<A, Vec<u8>>,
    },
}

impl<A, E, C> Future for Receive<A, E, C>
    where A: AsyncRead,
          E: ByteOrder,
          C: Command,
{
    type Item = (A, LevinResult<C::Response>);
    type Error = io::Error;

    fn poll(&mut self) -> Poll<Self::Item, Self::Error> {
        loop {
            let next_state = match self.state {
                ReceiveState::ReadBucket { ref mut reader } => {
                    let (stream, buf, size) = try_ready!(reader.poll());
                    if buf.len() != size {
                        return Ok((stream, Err(LevinError::UnfinishedRead(buf.len()))).into());
                    }

                    let mut buf = buf.into_buf();
                    let bucket_head = match BucketHead::read::<E, _>(&mut buf) { 
                        Ok(b) => b,
                        Err(e) => {
                            return Ok((stream, Err(e)).into());
                        },
                    };

                    if bucket_head.command != C::ID {
                        return Ok((stream, Err(LevinError::InvalidCommandId(bucket_head.command))).into());
                    }

                    let mut response_buf = vec![0u8; bucket_head.cb as usize];
                    ReceiveState::ReadResponse {
                        reader: read(stream, response_buf)
                    }
                },
                ReceiveState::ReadResponse { ref mut reader } => {
                    let (stream, buf, size) = try_ready!(reader.poll());
                    if buf.len() != size {
                        return Ok((stream, Err(LevinError::UnfinishedRead(buf.len()))).into());
                    }

                    let mut buf = buf.into_buf();
                    let section = portable_storage::read::<E, _>(&mut buf).unwrap();
                    let response = C::Response::deserialize(&section).unwrap();

                    return Ok((stream, Ok(response)).into())
                },
            };

            self.state = next_state;
        }
    }
}
