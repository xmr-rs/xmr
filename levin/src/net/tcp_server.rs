// Copyright 2018 Jean Pierre Dudey <jeandudey@hotmail.com>
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

use std::io;
use std::net::SocketAddr;
use std::sync::Arc;

use futures::{Future, future};
use futures::stream::Stream;

use tokio_core::net::TcpListener;
use tokio_core::reactor::Handle;
use tokio_io::{AsyncRead, AsyncWrite};

use net::io::IoHandlerRef;
use net::handlers::RemoteHandler;
use net::bucket_stream::bucket_stream;
use net::bucket_sink::bucket_sink;
use net::tcp_client::Commands;

/// A levin server.
#[allow(missing_debug_implementations)]
pub struct TcpServer {
    listener: TcpListener,
    io_handler: IoHandlerRef,
    connection_handler: ConnectionHandlerRef,
}

impl TcpServer {
    /// Creates a new server.
    pub fn bind(addr: &SocketAddr,
                handle: &Handle,
                io_handler: IoHandlerRef,
                connection_handler: ConnectionHandlerRef)
                -> io::Result<TcpServer> {
        Ok(TcpServer {
               listener: TcpListener::bind(addr, handle)?,
               io_handler,
               connection_handler,
           })
    }

    /// Creates a future that will run the server.
    pub fn run(self) -> Box<Future<Item = (), Error = io::Error> + Send + Sync + 'static> {
        let io_handler = self.io_handler;
        let connection_handler = self.connection_handler;
        Box::new(self.listener
            .incoming()
            .for_each(move |(stream, addr)| {
                let io_handler = io_handler.clone();
                let connection_handler = connection_handler.clone();

                let commands = Commands::new();
                connection_handler.on_connect(addr.clone(), commands.clone());

                let (read_half, write_half) = stream.split();

                let buckets = bucket_stream(read_half);

                let commands_ = commands.clone();
                let receiver = buckets.for_each(move |bucket| {
                    let io_handler = io_handler.clone();
                    let commands = commands_.clone();

                    let bucket = match bucket {
                        Ok(b) => b,
                        Err(e) => {
                            warn!("received invalid bucket: {}", e);
                            return future::ok::<(), io::Error>(());
                        }
                    };

                    let id = bucket.head.command;
                    let section = match bucket.into_section() {
                        Ok(s) => s,
                        Err(e) => {
                            warn!("received bucket with invalid portable-storage section: {}",
                                  e);
                            commands.error_response(id, -1);
                            return future::ok::<(), io::Error>(());
                        }
                    };

                    if bucket.head.is_request() {
                        match io_handler.get(id) {
                            Some(RemoteHandler::Invokation(handler)) => {
                                let response = handler.call(addr.clone(), section);
                                match response {
                                    Ok(Some(r)) => commands.invokation_response(id, r),
                                    Ok(None) => { /* do nothing, the command stream is closed */},
                                    Err(e) => commands.error_response(id, e),
                                }
                            }
                            Some(RemoteHandler::Notification(handler)) => handler.call(addr.clone(), section),
                            None => {
                                warn!("received bucket with ID #{} but a handler isn't defined.",
                                      id);
                                commands.error_response(id, -1);
                                return future::ok::<(), io::Error>(());
                            }
                        }
                    } else {
                        if let Some((handler_id, handler)) = commands.current_handler() {
                            if id != handler_id {
                                warn!("response id #{} doesn't match handler id #{}",
                                      id,
                                      handler_id);
                                commands.error_response(id, -1);
                                return future::ok::<(), io::Error>(());
                            }

                            handler.call(section);
                        } else {
                            warn!("received response with ID #{}, but no handler is defined.",
                                  id);
                            commands.error_response(id, -1);
                            return future::ok::<(), io::Error>(());
                        }
                    }

                    future::ok::<(), io::Error>(())
                });

                let bucket_sender = bucket_sink(write_half);

                let sender = commands.forward(bucket_sender)
                    .map(|(_, sender)| sender.inner().unwrap().shutdown());

                receiver.join(sender).map(|_| ())
            }))
    }
}

/// A container trait to handle connections.
pub trait ConnectionHandler: Send + Sync + 'static {
    /// A new connection is made
    fn on_connect(&self, addr: SocketAddr, commands: Commands);
}

/// A reference to a `ConnectionHandler`.
pub type ConnectionHandlerRef = Arc<ConnectionHandler>;
