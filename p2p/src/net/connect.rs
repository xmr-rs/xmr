use std::io;
use std::sync::Arc;
use std::net::SocketAddr;

use futures::{Future, Poll};
use tokio_core::reactor::Handle;
use tokio_core::net::{TcpStream, TcpStreamNew};

use uuid::Uuid;

use p2p::Context;
use levin::{LevinError, DefaultEndian, Command, Invoke, invoke};
use protocol::handshake::CryptoNoteHandshake;

pub type Request = <CryptoNoteHandshake as Command>::Request;
pub type Response = <CryptoNoteHandshake as Command>::Response;

pub fn connect(address: &SocketAddr,
               handle: &Handle,
               context: Arc<Context>,
               request: Request) -> Connect {
    Connect {
        context,
        state: ConnectState::TcpConnect {
            future: TcpStream::connect(address, handle),
            request,
        }
    }
}

enum ConnectState {
    TcpConnect {
        future: TcpStreamNew,
        request: Request,
    },
    InvokeHandshake {
        future: Invoke<TcpStream>,
    }
}

pub struct Connect {
    state: ConnectState,
    context: Arc<Context>,
}

impl Future for Connect {
    type Item = (TcpStream, Result<Response, ConnectError>);
    type Error = io::Error;

    fn poll(&mut self) -> Poll<Self::Item, Self::Error> {
        loop {
            let next_state = match self.state {
                ConnectState::TcpConnect { ref mut future, ref request } => {
                    let stream = try_ready!(future.poll());
                    ConnectState::InvokeHandshake {
                        future: invoke::<CryptoNoteHandshake, TcpStream, DefaultEndian>(stream, request),
                    }
                },
                ConnectState::InvokeHandshake { ref mut future } => {
                    let stream = try_ready!(future.poll());

                    return Ok((stream, Err(ConnectError::SamePeerId)).into())
                },
            };
            self.state = next_state;
        }
    }
}

#[derive(Debug)]
pub enum ConnectError {
    /// A levin error.
    LevinError(LevinError),
    /// Wrong network Id.
    WrongNetwork(Uuid),
    /// The peer has the same peer id, probably connected to self.
    SamePeerId,
}

impl From<LevinError> for ConnectError {
    fn from(e: LevinError) -> ConnectError {
        ConnectError::LevinError(e)
    }
}
