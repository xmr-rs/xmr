use primitives::H256;
use chain::IndexedBlock;
use block_ref::BlockRef;

pub trait BlockProvider {
    fn block_id(&self, height: u64) -> Option<H256>;
}

pub trait IndexedBlockProvider: BlockProvider {
    fn indexed_block(&self, block_ref: BlockRef) -> Option<IndexedBlock>;
}
