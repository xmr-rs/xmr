use std::io;

use bytes::{Bytes, BytesMut, IntoBuf};

use futures::{Future, Poll};
use tokio_io::{AsyncWrite, AsyncRead};
use tokio_io::io::{Read, WriteAll, read, write_all};

use levin::{Command, Notify, Storage, LevinResult, LevinError};
use levin::bucket::bucket_head::{BucketHead, LEVIN_SIGNATURE, LEVIN_PROTOCOL_VER_1, LEVIN_OK,
                                 LEVIN_PACKET_REQUEST, LEVIN_PACKET_RESPONSE, BUCKET_HEAD_LENGTH};

use portable_storage::{self, Section};

pub struct Bucket {
    pub head: BucketHead,
    pub body: BytesMut,
}

impl Bucket {
    pub fn request<C>(body: &C::Request) -> Bucket
        where C: Command
    {
        let body_section = body.to_section().expect("invalid portable storage type");
        let mut body_buf = BytesMut::new();
        portable_storage::write(&mut body_buf, &body_section);

        Bucket {
            head: BucketHead {
                signature: LEVIN_SIGNATURE,
                cb: body_buf.len() as u64,
                have_to_return_data: true,
                command: C::ID,
                return_code: LEVIN_OK,
                protocol_version: LEVIN_PROTOCOL_VER_1,
                flags: LEVIN_PACKET_REQUEST,
            },
            body: body_buf,
        }
    }

    pub fn response<C>(body: &C::Response) -> Bucket
        where C: Command
    {
        let body_section = body.to_section().expect("invalid portable storage type");
        let mut body_buf = BytesMut::new();
        portable_storage::write(&mut body_buf, &body_section);

        Bucket {
            head: BucketHead {
                signature: LEVIN_SIGNATURE,
                cb: body_buf.len() as u64,
                have_to_return_data: false,
                command: C::ID,
                return_code: LEVIN_OK,
                protocol_version: LEVIN_PROTOCOL_VER_1,
                flags: LEVIN_PACKET_RESPONSE,
            },
            body: body_buf,
        }
    }

    pub fn notify<N>(body: &N::Request) -> Bucket
        where N: Notify
    {
        let body_section = body.to_section().expect("invalid portable storage type");
        let mut body_buf = BytesMut::new();
        portable_storage::write(&mut body_buf, &body_section);

        Bucket {
            head: BucketHead {
                signature: LEVIN_SIGNATURE,
                cb: body_buf.len() as u64,
                have_to_return_data: false,
                command: N::ID,
                return_code: LEVIN_OK,
                protocol_version: LEVIN_PROTOCOL_VER_1,
                flags: LEVIN_PACKET_REQUEST,
            },
            body: body_buf,
        }
    }

    pub fn request_future<A, C>(a: A, body: &C::Request) -> Request<A>
        where A: AsyncWrite,
              C: Command
    {
        Request { future: write_all(a, Self::request::<C>(body).to_bytes()) }
    }

    pub fn response_future<A, C>(a: A, body: &C::Response) -> Response<A>
        where A: AsyncWrite,
              C: Command
    {
        Response { future: write_all(a, Self::response::<C>(body).to_bytes()) }
    }

    pub fn notify_future<A, N>(a: A, body: &N::Request) -> Request<A>
        where A: AsyncWrite,
              N: Notify
    {
        Request { future: write_all(a, Self::notify::<N>(body).to_bytes()) }
    }

    pub fn receive_future<A>(a: A) -> Receive<A>
        where A: AsyncRead
    {
        let buf = vec![0u8; BUCKET_HEAD_LENGTH];
        Receive { state: ReceiveState::ReadBucket { reader: read(a, buf) } }
    }

    pub fn into_request<C>(&self) -> LevinResult<C::Request>
        where C: Command
    {
        if C::ID != self.head.command {
            return Err(LevinError::InvalidCommandId(self.head.command));
        }

        let section = self.body_into_section();

        // TODO: remove unwrap and add error to LevinError.
        let req = C::Request::from_section(section).unwrap();

        Ok(req)
    }

    pub fn into_response<C>(&self) -> LevinResult<C::Response>
        where C: Command
    {
        if C::ID != self.head.command {
            return Err(LevinError::InvalidCommandId(self.head.command));
        }

        let section = self.body_into_section();

        // TODO: remove unwrap and add error to LevinError.
        let req = C::Response::from_section(section).unwrap();

        Ok(req)
    }

    pub fn into_notify<N>(&self) -> LevinResult<N::Request>
        where N: Notify
    {
        if N::ID != self.head.command {
            return Err(LevinError::InvalidCommandId(self.head.command));
        }

        let section = self.body_into_section();

        // TODO: remove unwrap and add error to LevinError.
        let req = N::Request::from_section(section).unwrap();

        Ok(req)
    }

    pub fn to_bytes(self) -> Bytes {
        let mut blob = BytesMut::with_capacity(self.body.len() + BUCKET_HEAD_LENGTH);
        BucketHead::write(&mut blob, &self.head);

        // unsplit is a bad and confusing name for this :(,
        // in this context it means "concatenate".
        blob.unsplit(self.body);

        blob.freeze()
    }

    fn body_into_section(&self) -> Section {
        use std::io::Cursor;
        // TODO: remove unwrap and add error to LevinError.
        let mut buf = Cursor::new(self.body.as_ref());
        portable_storage::read(&mut buf).unwrap()
    }
}

pub struct Request<A> {
    future: WriteAll<A, Bytes>,
}

impl<A> Future for Request<A>
    where A: AsyncWrite
{
    type Item = (A, Bytes);
    type Error = io::Error;

    fn poll(&mut self) -> Poll<Self::Item, Self::Error> {
        self.future.poll()
    }
}

pub struct Response<A> {
    future: WriteAll<A, Bytes>,
}

impl<A> Future for Response<A>
    where A: AsyncWrite
{
    type Item = (A, Bytes);
    type Error = io::Error;

    fn poll(&mut self) -> Poll<Self::Item, Self::Error> {
        self.future.poll()
    }
}

#[derive(Debug)]
pub struct Receive<A: AsyncRead> {
    state: ReceiveState<A>,
}

#[derive(Debug)]
enum ReceiveState<A> {
    ReadBucket { reader: Read<A, Vec<u8>> },
    ReadStorage {
        bucket_head: BucketHead,
        reader: Read<A, Vec<u8>>,
    },
}

impl<A> Future for Receive<A>
    where A: AsyncRead
{
    type Item = (A, LevinResult<Bucket>);
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
                        }
                    };

                    trace!("receive poll - bucket received: {:?}", bucket_head);

                    let buf = vec![0u8; bucket_head.cb as usize];
                    ReceiveState::ReadStorage {
                        bucket_head,
                        reader: read(stream, buf),
                    }
                }
                ReceiveState::ReadStorage {
                    ref bucket_head,
                    ref mut reader,
                } => {
                    trace!("receive poll - reading response");

                    let (stream, buf, size) = try_ready!(reader.poll());
                    if buf.len() != size {
                        return Ok((stream, Err(LevinError::UnfinishedRead(buf.len()))).into());
                    }

                    let bucket = Bucket {
                        head: bucket_head.clone(),
                        body: buf.into(),
                    };

                    return Ok((stream, Ok(bucket)).into());
                }
            };

            self.state = next_state;
        }
    }
}
