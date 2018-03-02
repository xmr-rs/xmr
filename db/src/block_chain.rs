use primitives::H256;
use chain::IndexedBlock;
use error::Error;

/// A BlockChain interface.
pub trait BlockChain {
    fn insert(&self, block: IndexedBlock) -> Result<(), Error>;

    fn canonize(&self, id: &H256) -> Result<(), Error>;
}
