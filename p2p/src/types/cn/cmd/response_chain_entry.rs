use levin::Command;
use portable_storage_utils::stl::StlLinkedList;
use primitives::H256;

use types::cn::CN_COMMAND_BASE_ID;

#[derive(Debug, Deserialize, Serialize)]
pub struct ResponseChainEntry {
    pub start_height: u64,
    pub total_height: u64,
    pub cummulative_difficulty: u64,
    #[serde(rename = "m_block_ids")]
    pub block_ids: StlLinkedList<H256>,
}

impl Command for ResponseChainEntry {
    const ID: u32 = CN_COMMAND_BASE_ID + 7;
}
