use levin::Command;
use portable_storage_utils::stl::StlVector;
use primitives::H256;

use types::cn::CN_COMMAND_BASE_ID;

#[derive(Debug, Deserialize, Serialize)]
pub struct RequestFluffyMissingTx {
    pub block_hash: H256,
    pub current_blockchain_length: u64,
    pub missing_tx_indices: StlVector<u64>,
}

impl Command for RequestFluffyMissingTx {
    const ID: u32 = CN_COMMAND_BASE_ID + 9;
}
