use chain::IndexedBlock;
use primitives::H256;

/// A BlockChain interface.
pub trait BlockChain {
    fn insert(&self, block: IndexedBlock) -> Result<(), String>;

    fn canonize(&self, id: &H256) -> Result<(), String>;
}
