use primitives::H256;
use levin::Notify;
use types::cn::CN_COMMAND_BASE_ID;

pub struct RequestGetObjects;

impl Notify for RequestGetObjects {
    type Request = RequestGetObjectsRequest;

    const ID: u32 = CN_COMMAND_BASE_ID + 3;
}

#[derive(Debug, Deserialize, Serialize)]
pub struct RequestGetObjectsRequest {
    pub txs: Vec<H256>,
    pub blocks: Vec<H256>,
}
