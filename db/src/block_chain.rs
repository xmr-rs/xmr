use std::sync::Arc;

use hash::H256;

/// A BlockChain interface.
pub trait BlockChain {
    /// Returns the blockchain's highest block id.
    fn top_id(&self) -> H256;
}

pub type SharedBlockChain = Arc<BlockChain>;
