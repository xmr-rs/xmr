use portable_storage_utils::stl::StlLinkedList;
use primitives::H256;
use levin::Notify;
use types::cn::CN_COMMAND_BASE_ID;

pub struct ResponseChainEntry;

impl Notify for ResponseChainEntry {
    type Request = ResponseChainEntryRequest;

    const ID: u32 = CN_COMMAND_BASE_ID + 7;
}

#[derive(Debug, Deserialize, Serialize)]
pub struct ResponseChainEntryRequest {
    pub start_height: u64,
    pub total_height: u64,
    pub cummulative_difficulty: u64,
    pub m_block_ids: StlLinkedList<H256>,
}