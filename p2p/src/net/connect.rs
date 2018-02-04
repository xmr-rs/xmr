use std::io;
use std::sync::Arc;
use std::net::SocketAddr;

use futures::{Future, Poll};
use tokio_core::reactor::Handle;
use tokio_core::net::{TcpStream, TcpStreamNew};

use uuid::Uuid;

use p2p::Context;
use levin::{LevinError, DefaultEndian, Invoke, invoke};
use protocol::handshake::CryptoNoteHandshake;

pub fn connect(address: &SocketAddr, handle: &Handle, context: Arc<Context>) -> Connect {
    Connect {
        context,
        state: ConnectState::TcpConnect {
            future: TcpStream::connect(address, handle),
        }
    }
}

enum ConnectState {
    TcpConnect {
        future: TcpStreamNew,
    },
    Handshake {
        future: Invoke<CryptoNoteHandshake, TcpStream, DefaultEndian>,
    }
}

pub struct Connect {
    state: ConnectState,
    context: Arc<Context>,
}

impl Future for Connect {
    type Item = Result<TcpStream, ConnectError>;
    type Error = io::Error;

    fn poll(&mut self) -> Poll<Self::Item, Self::Error> {
        loop {
            let next_state = match self.state {
                ConnectState::TcpConnect { ref mut future } => {
                    let stream = try_ready!(future.poll());
                    ConnectState::Handhsake {
                        future: invoke::<Handshake, TcpStream, DefaultEndian>(),
                    }
                },
                ConnectState::Handshake { ref mut future } => {
                    let (stream, response) = try_ready!(future.poll());
                    let response = response?;
                    if response.node_data.network_id != self.context.config.network_id {
                        let uuid = response.node_data.network_id;
                        return Ok(Err(ConnectError::WrongNetwork(uuid)).into());
                    }
                },
            };
            self.state = next_state;
        }
    }
}

pub enum ConnectError {
    /// A levin error.
    LevinError(LevinError),
    /// Wrong network Id.
    WrongNetwork(Uuid)
}

impl From<LevinError> for ConnectError {
    fn from(e: LevinError) -> ConnectError {
        ConnectError::LevinError(e)
    }
}
