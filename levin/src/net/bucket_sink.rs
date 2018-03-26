// Copyright 2018 Jean Pierre Dudey <jeandudey@hotmail.com>
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

use std::io;
use std::mem::replace;

use futures::{Future, Poll, Async, AsyncSink, StartSend};
use futures::sink::Sink;

use tokio_io::io::{WriteAll, write_all};
use tokio_io::AsyncWrite;

use crossbeam::sync::MsQueue;

use bytes::Bytes;

use bucket::Bucket;

/// Creates the bucket sink.
pub fn bucket_sink<A>(a: A) -> BucketSink<A>
    where A: AsyncWrite
{
    BucketSink {
        a: Some(a),
        current_write: None,
        queue: MsQueue::new(),
    }
}

/// A sink of buckets.
#[derive(Debug)]
pub struct BucketSink<A: AsyncWrite> {
    a: Option<A>,
    current_write: Option<WriteAll<A, Bytes>>,
    queue: MsQueue<Bucket>,
}

impl<A> BucketSink<A>
    where A: AsyncWrite
{
    /// Consumes the sink and returns the inner `AsyncWrite`
    /// writer.
    /// 
    /// It returns `Some(writer)` if all items have been flushed
    /// otherwise returns `None`.
    pub fn inner(self) -> Option<A> {
        self.a
    }
}

impl<A> Sink for BucketSink<A>
    where A: AsyncWrite
{
    type SinkItem = Bucket;
    type SinkError = io::Error;

    fn start_send(&mut self, item: Self::SinkItem) -> StartSend<Self::SinkItem, Self::SinkError> {
        if self.current_write.is_none() && self.queue.is_empty() {
            let a = replace(&mut self.a, None).expect("invalid state");
            let mut future = write_all(a, item.to_bytes());

            // start writing the data inmediately and
            // see if it's done, otherwise poll_completed should
            // do this.
            match future.poll()? {
                Async::Ready((a, _)) => {
                    self.a = Some(a);
                    Ok(AsyncSink::Ready)
                }
                Async::NotReady => {
                    self.current_write = Some(future);
                    Ok(AsyncSink::Ready)
                }
            }
        } else if self.current_write.is_none() && !self.queue.is_empty() {
            // the pop method shouldn't block here because the queue is not
            // empty, however try_pop().unwrap() isn't used because in highly
            // threaded environments the queue could get empty.
            let bucket = self.queue.pop();
            self.queue.push(item);

            let a = replace(&mut self.a, None).expect("invalid state");
            let mut future = write_all(a, bucket.to_bytes());

            // TODO: DRY, same as above
            match future.poll()? {
                Async::Ready((a, _)) => {
                    self.a = Some(a);
                    Ok(AsyncSink::Ready)
                }
                Async::NotReady => {
                    self.current_write = Some(future);
                    Ok(AsyncSink::Ready)
                }
            }
        } else {
            // a write operation is already being done,
            // so we put this bucket into the queue.
            self.queue.push(item);

            Ok(AsyncSink::Ready)
        }
    }

    fn poll_complete(&mut self) -> Poll<(), Self::SinkError> {
        if let Some(ref mut future) = self.current_write {
            let (a, _) = try_ready!(future.poll());
            self.a = Some(a);
        }

        // check if there are remaining items to write
        if let Some(bucket) = self.queue.try_pop() {
            let a = replace(&mut self.a, None).expect("invalid state");
            let mut future = write_all(a, bucket.to_bytes());

            match future.poll()? {
                Async::Ready((a, _)) => {
                    self.a = Some(a);

                    // now that the item is writing try to write other
                    // items calling this function recursively once this is
                    // done.
                    return self.poll_complete();
                }
                Async::NotReady => {
                    self.current_write = Some(future);
                    return Ok(Async::NotReady);
                }
            }
        } else {
            return Ok(Async::Ready(()));
        }
    }
}
