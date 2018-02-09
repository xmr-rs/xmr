use chain::BlockHeader;
use best_block::BestBlock;

/// Blockchain storage interface.
pub trait Store {
    /// Get the best block.
    fn best_block(&self) -> BestBlock;

    /// Get the best block header.
    fn best_header(&self) -> BlockHeader;
}
