use hash::H256;

use block_chain::BlockChain;

/// A blockchain database.
pub struct BlockChainDatabase;

impl BlockChain for BlockChainDatabase {
    fn top_id() -> H256 {
        unimplemented!();
    }
}
