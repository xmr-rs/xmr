use bytes::{BytesMut, Buf, ByteOrder};
use failure::Error;
use codec::Codec;

pub const COMMAND_BASE_ID: usize = 1000;

/// A protocol command.
pub trait Command<T: ByteOrder> {
    const ID: usize;
    type Request: Codec<T>;
    type Response: Codec<T>;

    fn encode_request(buf: &mut BytesMut, req: &Self::Request) -> Result<(), Error> {
        req.encode(buf)
    }

    fn encode_response(buf: &mut BytesMut, res: &Self::Response) -> Result<(), Error> {
        res.encode(buf)
    }

    fn decode_request<B: Buf>(buf: &mut B) -> Result<Self::Request, Error> {
        Self::Request::decode::<B>(buf)
    }

    fn decode_response<B: Buf>(buf: &mut B) -> Result<Self::Response, Error> {
        Self::Response::decode::<B>(buf)
    }
}
