use levin::Command;
use portable_storage_utils::stl::StlLinkedList;
use primitives::H256;

use types::cn::CN_COMMAND_BASE_ID;

#[derive(Debug, Deserialize, Serialize)]
pub struct RequestChain {
    pub block_ids: StlLinkedList<H256>,
}

impl Command for RequestChain {
    const ID: u32 = CN_COMMAND_BASE_ID + 6;
}
