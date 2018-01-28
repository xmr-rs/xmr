use std::result;

/// A levin result.
pub type LevinResult<T> = result::Result<T, LevinError>;

pub enum LevinError {
    /// An error when reading `BucketHead`.
    BucketHeadError(BucketHeadError),
    /// Unexpected End-Of-Buffer.
    UnexpectedEob,
    /// Expected to read more bytes.
    UnfinishedRead(usize),
    /// The command is invalid.
    InvalidCommandId(u32),
}

impl From<BucketHeadError> for LevinError {
    fn from(e: BucketHeadError) -> LevinError {
        LevinError::BucketHeadError(e)
    }
}

pub enum BucketHeadError {
    /// The version isn't supported.
    InvalidProtocolVersion(u32),
    /// Invalid signature
    InvalidSignature(u64),
    /// Packet too big.
    TooBig(u64),
}
