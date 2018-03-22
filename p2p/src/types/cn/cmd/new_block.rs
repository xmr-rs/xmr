use levin::Command;

use types::cn::{CN_COMMAND_BASE_ID, BlockCompleteEntry};

#[derive(Debug, Deserialize, Serialize)]
pub struct NewBlock {
    pub b: BlockCompleteEntry,
    pub current_blockchain_height: u64,
}

impl Command for NewBlock {
    const ID: u32 = CN_COMMAND_BASE_ID + 1;
}
