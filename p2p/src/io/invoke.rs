use std::io;
use std::marker::PhantomData;

use futures::{Future, Poll, Async};
use tokio_io::{AsyncRead, AsyncWrite};
use bytes::{BytesMut, ByteOrder};

use portable_storage::{Serialize, Deserialize};

use io::{
    WriteSection,
    ReadSection,
    ReadHeader,

    write_section,
    read_section,
    read_header
};
use command::Command;

/// Invoke a command.
pub fn invoke<C, A, E>(a: A, request: C::Request) -> Invoke<C, A, E>
    where C: Command,
          A: AsyncRead + AsyncWrite,
          E: ByteOrder,
{
    let section = request.serialize();

    Invoke {
        state: InvokeState::WriteRequest {
            future: write_section::<A, E>(a, section),
        },
        _phantom: PhantomData,
    }
}

pub struct Invoke<C, A, E> {
    state: InvokeState<A, E>,
    _phantom: PhantomData<C>,
}

enum InvokeState<A, E> {
    WriteRequest {
        future: WriteSection<A>,
    },
    ReadHeader {
        future: ReadHeader<A, E>,
    },
    ReadResponse {
        future: ReadSection<A, E>,
    },
}

impl<C, A, E> Future for Invoke<C, A, E>
    where C: Command,
          A: AsyncRead + AsyncWrite,
          E: ByteOrder,
{
    type Item = (A, C::Response);
    type Error = io::Error;

    fn poll(&mut self) -> Poll<Self::Item, Self::Error> {
        loop {
            let next_state = match self.state {
                InvokeState::WriteRequest { ref mut future } => {
                    let (stream, _) = try_ready!(future.poll());
                    InvokeState::ReadHeader {
                        future: read_header(stream),
                    }
                },
                InvokeState::ReadHeader { ref mut future } => {
                    let (stream, _) = try_ready!(future.poll());
                    InvokeState::ReadResponse {
                        future: read_section(stream),
                    }
                },
                InvokeState::ReadResponse { ref mut future } => {
                    let (stream, section) = try_ready!(future.poll());
                    // TODO: better error handling.
                    let response = C::Response::deserialize(&section).unwrap();
                    return Ok((stream, response).into());
                },
            };
            self.state = next_state;
        }
    }
}
