use bytes::LittleEndian;

pub type DefaultEndian = LittleEndian;

mod command;
pub use self::command::{COMMAND_BASE_ID, Command};

mod bucket;
pub use self::bucket::{BUCKET_HEAD_LENGTH, BucketHead, invoke_bucket};

mod invoke;
pub use self::invoke::{Invoke, invoke};

mod result;
pub use self::result::{LevinResult, LevinError, BucketHeadError};
