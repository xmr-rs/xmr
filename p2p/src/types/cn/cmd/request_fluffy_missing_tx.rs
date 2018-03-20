use levin::Command;
use portable_storage_utils::stl::StlVector;
use primitives::H256;

use types::cn::CN_COMMAND_BASE_ID;

pub struct RequestFluffyMissingTx;

impl Command for RequestFluffyMissingTx {
    const ID: u32 = CN_COMMAND_BASE_ID + 9;
}

#[derive(Debug, Deserialize, Serialize)]
pub struct RequestFluffyMissingTxRequest {
    pub block_hash: H256,
    pub current_blockchain_length: u64,
    pub missing_tx_indices: StlVector<u64>,
}
