use levin::Notify;
use types::cn::{CN_COMMAND_BASE_ID, BlockCompleteEntry};

pub struct NewBlock;

impl Notify for NewBlock {
    type Request = NewBlockRequest;

    const ID: u32 = CN_COMMAND_BASE_ID + 1;
}

#[derive(Debug, Deserialize, Serialize)]
pub struct NewBlockRequest {
    pub b: BlockCompleteEntry,
    pub current_blockchain_height: u64,
}
