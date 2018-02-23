use std::io;
use std::marker::PhantomData;

use bytes::IntoBuf;
use futures::{Future, Poll};

use tokio_io::AsyncRead;
use tokio_io::io::{Read, read};

use portable_storage;

use levin::{
    BUCKET_HEAD_LENGTH,
    BucketHead,
    Storage,
    LevinResult,
    LevinError,
};

pub fn receive<A, S>(a: A) -> Receive<A, S>
    where A: AsyncRead,
          S: Storage, {
    trace!("receive - creating future");
    let buf = vec![0u8; BUCKET_HEAD_LENGTH];
    Receive {
        state: ReceiveState::ReadBucket {
            reader: read(a, buf),
        },
        _phantom_data: PhantomData,
    }
}

#[derive(Debug)]
pub struct Receive<A: AsyncRead, S: Storage> {
    state: ReceiveState<A>,
    _phantom_data: PhantomData<S>,
}

#[derive(Debug)]
enum ReceiveState<A> {
    ReadBucket {
        reader: Read<A, Vec<u8>>,
    },
    ReadStorage {
        bucket_head: BucketHead,
        reader: Read<A, Vec<u8>>,
    },
}

impl<A, S> Future for Receive<A, S>
    where A: AsyncRead,
          S: Storage,
{
    type Item = (A, LevinResult<(BucketHead, S)>);
    type Error = io::Error;

    fn poll(&mut self) -> Poll<Self::Item, Self::Error> {
        loop {
            let next_state = match self.state {
                ReceiveState::ReadBucket { ref mut reader } => {
                    trace!("receive poll - reading bucket");
                    let (stream, buf, size) = try_ready!(reader.poll());
                    if buf.len() != size {
                        return Ok((stream, Err(LevinError::UnfinishedRead(buf.len()))).into());
                    }

                    let mut buf = buf.into_buf();
                    let bucket_head = match BucketHead::read(&mut buf) { 
                        Ok(b) => b,
                        Err(e) => {
                            return Ok((stream, Err(e)).into());
                        },
                    };

                    trace!("receive poll - bucket received: {:?}", bucket_head);

                    let buf = vec![0u8; bucket_head.cb as usize];
                    ReceiveState::ReadStorage {
                        bucket_head,
                        reader: read(stream, buf)
                    }
                },
                ReceiveState::ReadStorage { ref bucket_head, ref mut reader } => {
                    trace!("receive poll - reading response");

                    let (stream, buf, size) = try_ready!(reader.poll());
                    if buf.len() != size {
                        return Ok((stream, Err(LevinError::UnfinishedRead(buf.len()))).into());
                    }

                    let mut buf = buf.into_buf();
                    let section = portable_storage::read(&mut buf).unwrap();
                    trace!("receive poll - received: {:?}", section);
                    let response = S::from_section(section).unwrap();

                    return Ok((stream, Ok((bucket_head.clone(), response))).into())
                },
            };

            self.state = next_state;
        }
    }
}
