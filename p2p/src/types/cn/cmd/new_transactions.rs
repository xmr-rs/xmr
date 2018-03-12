use portable_storage_utils::Blob;
use levin::Notify;
use types::cn::CN_COMMAND_BASE_ID;

pub struct NewTransactions;

impl Notify for NewTransactions {
    type Request = NewTransactionsRequest;

    const ID: u32 = CN_COMMAND_BASE_ID + 2;
}

#[derive(Debug, Deserialize, Serialize)]
pub struct NewTransactionsRequest {
    pub txs: Vec<Blob>,
}
