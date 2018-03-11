use levin::Notify;
use types::cn::{CN_COMMAND_BASE_ID, BlockCompleteEntry};

pub struct NotifyNewFluffyBlock;

impl Notify for NotifyNewFluffyBlock {
    type Request = NotifyNewFluffyBlockRequest;

    const ID: u32 = CN_COMMAND_BASE_ID + 8;
}

#[derive(Debug, Deserialize, Serialize)]
pub struct NotifyNewFluffyBlockRequest {
    pub b: BlockCompleteEntry,
    pub current_blockchain_height: u64,
}
