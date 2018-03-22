use levin::Command;
use portable_storage_utils::Blob;

use types::cn::CN_COMMAND_BASE_ID;

#[derive(Debug, Deserialize, Serialize)]
pub struct NewTransactions {
    pub txs: Vec<Blob>,
}

impl Command for NewTransactions {
    const ID: u32 = CN_COMMAND_BASE_ID + 2;
}
