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
    DoHandshake {
        future: Invoke<CryptoNoteHandshake, TcpStream, DefaultEndian>,
    }
}

pub struct Connect {
    state: ConnectState,
    context: Arc<Context>,
}

impl Future for Connect {
    type Item = Result<(TcpStream, Response), ConnectError>;
    type Error = io::Error;

    fn poll(&mut self) -> Poll<Self::Item, Self::Error> {
        loop {
            let next_state = match self.state {
                ConnectState::TcpConnect { ref mut future, ref request } => {
                    let stream = try_ready!(future.poll());
                    ConnectState::DoHandshake {
                        future: invoke::<CryptoNoteHandshake, TcpStream, DefaultEndian>(stream, request),
                    }
                },
                ConnectState::DoHandshake { ref mut future } => {
                    let (stream, response) = try_ready!(future.poll());
                    if response.is_err() {
                        return Ok(Err(response.map_err(|e| ConnectError::from(e)).unwrap_err()).into())
                    }
                    let response = response.unwrap();
                    if response.node_data.network_id.0 != self.context.config.network_id {
                        let uuid = response.node_data.network_id.0;
                        return Ok(Err(ConnectError::WrongNetwork(uuid)).into());
                    }

                    if response.node_data.peer_id == self.context.peer_id {
                        return Ok(Err(ConnectError::SamePeerId).into());
                    }

                    return Ok(Ok((stream, response)).into());
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
