use levin::Command;
use portable_storage_utils::Blob;

use types::cn::CN_COMMAND_BASE_ID;

pub struct NewTransactions;

impl Command for NewTransactions {
    const ID: u32 = CN_COMMAND_BASE_ID + 2;
}

#[derive(Debug, Deserialize, Serialize)]
pub struct NewTransactionsRequest {
    pub txs: Vec<Blob>,
}
