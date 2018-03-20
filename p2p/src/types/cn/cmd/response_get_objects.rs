use levin::Command;
use primitives::H256;
use portable_storage_utils::Blob;

use types::cn::{CN_COMMAND_BASE_ID, BlockCompleteEntry};

pub struct ResponseGetObjects;

impl Command for ResponseGetObjects {
    const ID: u32 = CN_COMMAND_BASE_ID + 4;
}

#[derive(Debug, Deserialize, Serialize)]
pub struct ResponseGetObjectsRequest {
    pub txs: Vec<Blob>,
    pub blocks: Vec<BlockCompleteEntry>,
    pub missed_ids: Vec<H256>,
    pub current_blockchain_height: u64,
}
