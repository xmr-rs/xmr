use std::result;

/// A levin result.
pub type LevinResult<T> = result::Result<T, LevinError>;

#[derive(Debug)]
pub enum LevinError {
    /// An error when reading `BucketHead`.
    BucketHead(BucketHeadError),
    /// Unexpected End-Of-Buffer.
    UnexpectedEob,
    /// Expected to read more bytes.
    UnfinishedRead(usize),
    /// The command is invalid.
    InvalidCommandId(u32),
}

impl From<BucketHeadError> for LevinError {
    fn from(e: BucketHeadError) -> LevinError {
        LevinError::BucketHead(e)
    }
}

#[derive(Debug)]
pub enum BucketHeadError {
    /// The version isn't supported.
    InvalidProtocolVersion(u32),
    /// Invalid signature
    InvalidSignature(u64),
    /// An error code was returned.
    ReturnCode(i32),
    /// Packet too big.
    TooBig(u64),
}
