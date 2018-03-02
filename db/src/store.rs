use block_chain::BlockChain;
use block_provider::{BlockProvider, IndexedBlockProvider};
use best_block::BestBlock;
use chain::BlockHeader;
use std::sync::Arc;

pub trait CanonStore: Store {
    fn as_store(&self) -> &Store;
}

/// Blockchain storage interface.
pub trait Store: AsSubstore {
    /// Get the best block.
    fn best_block(&self) -> BestBlock;

    /// Get the best block header.
    fn best_header(&self) -> BlockHeader;
}

/// Allows casting Arc<Store> to reference to any substore type
pub trait AsSubstore: BlockChain + IndexedBlockProvider {
	fn as_block_provider(&self) -> &BlockProvider;
}

impl<T> AsSubstore for T where T: BlockChain + IndexedBlockProvider {
	fn as_block_provider(&self) -> &BlockProvider {
		&*self
	}
}

pub type SharedStore = Arc<CanonStore + Send + Sync>;
