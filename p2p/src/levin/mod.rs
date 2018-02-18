use bytes::LittleEndian;

pub type DefaultEndian = LittleEndian;

mod bucket;
mod command;
mod invoke;
mod result;
mod receive;

pub use self::bucket::{BUCKET_HEAD_LENGTH, BucketHead, invoke_bucket};
pub use self::command::{COMMAND_BASE_ID, Command};
pub use self::invoke::{Invoke, invoke};
pub use self::result::{LevinResult, LevinError, BucketHeadError};
pub use self::receive::{Receive, receive};
