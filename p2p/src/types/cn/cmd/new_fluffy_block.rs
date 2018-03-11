use levin::Notify;
use types::cn::{CN_COMMAND_BASE_ID, BlockCompleteEntry};

pub struct NewFluffyBlock;

impl Notify for NewFluffyBlock {
    type Request = NewFluffyBlockRequest;

    const ID: u32 = CN_COMMAND_BASE_ID + 8;
}

#[derive(Debug, Deserialize, Serialize)]
pub struct NewFluffyBlockRequest {
    pub b: BlockCompleteEntry,
    pub current_blockchain_height: u64,
}
