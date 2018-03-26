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

use futures::{Async, Future, Poll, future};
use futures::stream::Stream;
use futures::task::{self, Task};

use tokio_core::net::TcpStream;
use tokio_core::reactor::Handle;
use tokio_io::{AsyncRead, AsyncWrite};

use crossbeam::sync::MsQueue;
use parking_lot::RwLock;

use portable_storage::Section;

use bucket::Bucket;
use command::{Command, Id};
use net::io::IoHandlerRef;
use net::handlers::RemoteHandler;
use net::bucket_stream::bucket_stream;
use net::bucket_sink::bucket_sink;

/// Connects to a levin server.
pub fn connect(addr: &SocketAddr,
               handle: &Handle,
               io_handler: IoHandlerRef,
               commands: Commands)
               -> Box<Future<Item = (), Error = io::Error> + Send + Sync + 'static> {
    let addr = addr.clone();
    Box::new(TcpStream::connect(&addr, handle).and_then(move |stream| {
        let io_handler = io_handler.clone();
        let commands = commands.clone();

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

pub trait InvokationResponseHandler: Send + Sync + 'static {
    fn call(&self, response: Section);
}

impl<F> InvokationResponseHandler for F
    where F: Send + Sync + 'static + Fn(Section)
{
    fn call(&self, response: Section) {
        self(response)
    }
}

/// A command queue stream.
#[allow(missing_debug_implementations)]
#[derive(Clone)]
pub struct Commands {
    queue: Arc<MsQueue<Bucket>>,
    handler_queue: Arc<MsQueue<(Id, Arc<InvokationResponseHandler>)>>,
    task: Arc<RwLock<Option<Task>>>,
    shutdown: Arc<RwLock<bool>>,
}

impl Commands {
    /// Creates an empty `Commands` strucutre.
    pub fn new() -> Commands {
        Commands {
            queue: Arc::new(MsQueue::new()),
            handler_queue: Arc::new(MsQueue::new()),
            task: Arc::new(RwLock::new(None)),
            shutdown: Arc::new(RwLock::new(false)),
        }
    }

    /// Adds an invokation to the queue.
    pub fn invoke<C, F>(&self, request: Section, command: F)
        where C: Command,
              F: InvokationResponseHandler + 'static
    {
        let bucket = Bucket::invokation(C::ID, request);

        self.queue.push(bucket);
        self.handler_queue.push((C::ID, Arc::new(command)));
        if let Some(ref task) = *self.task.read() {
            task.notify()
        }
    }

    /// Adds a notification to the queue.
    pub fn notify<C>(&self, request: Section)
        where C: Command
    {
        let bucket = Bucket::notification(C::ID, request);

        self.queue.push(bucket);
        if let Some(ref task) = *self.task.read() {
            task.notify()
        }
    }

    /// Adds an invokation response to the queue.
    pub fn invokation_response(&self, id: Id, response: Section) {
        let bucket = Bucket::invokation_response(id, response);

        self.queue.push(bucket);
        if let Some(ref task) = *self.task.read() {
            task.notify()
        }
    }

    /// Adds an error response bucket to the queue.
    pub fn error_response(&self, id: Id, return_code: i32) {
        let bucket = Bucket::error_response(id, return_code);

        self.queue.push(bucket);
        if let Some(ref task) = *self.task.read() {
            task.notify()
        }
    }

    pub(crate) fn current_handler(&self) -> Option<(Id, Arc<InvokationResponseHandler>)> {
        self.handler_queue.try_pop()
    }

    /// Stop streaming commands.
    pub fn shutdown(&self) {
        *self.shutdown.write() = true;
    }
}

impl Stream for Commands {
    type Item = Bucket;
    type Error = io::Error;

    fn poll(&mut self) -> Poll<Option<Self::Item>, Self::Error> {
        let shutdown = *self.shutdown.read();
        if let Some(bucket) = self.queue.try_pop() {
            if shutdown {
                Ok(Async::Ready(None))
            } else {
                *self.task.write() = None;
                Ok(Async::Ready(Some(bucket)))
            }
        } else {
            if shutdown {
                Ok(Async::Ready(None))
            } else {
                *self.task.write() = Some(task::current());
                Ok(Async::NotReady)
            }
        }
    }
}
