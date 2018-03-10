use levin::Notify;
use types::cn::{CN_COMMAND_BASE_ID, BlockCompleteEntry};

pub struct NotifyNewBlock;

impl Notify for NotifyNewBlock {
    type Request = NotifyNewBlockRequest;

    const ID: u32 = CN_COMMAND_BASE_ID + 1;
}

#[derive(Debug, Deserialize, Serialize)]
pub struct NotifyNewBlockRequest {
    pub b: BlockCompleteEntry,
    pub current_blockchain_height: u64,
}
