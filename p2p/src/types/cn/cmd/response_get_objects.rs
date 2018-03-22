use levin::Command;
use primitives::H256;
use portable_storage_utils::Blob;

use types::cn::{CN_COMMAND_BASE_ID, BlockCompleteEntry};

#[derive(Debug, Deserialize, Serialize)]
pub struct ResponseGetObjects {
    pub txs: Vec<Blob>,
    pub blocks: Vec<BlockCompleteEntry>,
    pub missed_ids: Vec<H256>,
    pub current_blockchain_height: u64,
}

impl Command for ResponseGetObjects {
    const ID: u32 = CN_COMMAND_BASE_ID + 4;
}
