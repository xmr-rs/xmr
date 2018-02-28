use chain::BlockHeader;
use best_block::BestBlock;
use std::sync::Arc;

pub trait CanonStore: Store {
    fn as_store(&self) -> &Store;
}

/// Blockchain storage interface.
pub trait Store {
    /// Get the best block.
    fn best_block(&self) -> BestBlock;

    /// Get the best block header.
    fn best_header(&self) -> BlockHeader;
}

pub type SharedStore = Arc<CanonStore + Send + Sync>;
