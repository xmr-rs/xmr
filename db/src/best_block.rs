use hash::H256;

/// The best block in the blockchain.
#[derive(Debug, Default, Clone, Eq, PartialEq)]
pub struct BestBlock {
    /// This block's height.
    pub height: u64,
    /// This block's id.
    pub id: H256,
}
