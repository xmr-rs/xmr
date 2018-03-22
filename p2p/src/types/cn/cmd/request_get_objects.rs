use levin::Command;
use primitives::H256;

use types::cn::CN_COMMAND_BASE_ID;

#[derive(Debug, Deserialize, Serialize)]
pub struct RequestGetObjects {
    pub txs: Vec<H256>,
    pub blocks: Vec<H256>,
}

impl Command for RequestGetObjects {
    const ID: u32 = CN_COMMAND_BASE_ID + 3;
}
