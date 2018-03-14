use std::cmp::PartialEq;

use primitives::H256;

use block::Block;

pub struct IndexedBlock {
    pub id: H256,
    pub raw: Block,
}

impl IndexedBlock {
    pub fn new(block: Block, id: H256) -> IndexedBlock {
        IndexedBlock { id, raw: block }
    }

    pub fn id(&self) -> &H256 {
        &self.id
    }
}

impl From<Block> for IndexedBlock {
    fn from(block: Block) -> IndexedBlock {
        IndexedBlock {
            id: block.id(),
            raw: block,
        }
    }
}

impl PartialEq for IndexedBlock {
    fn eq(&self, other: &IndexedBlock) -> bool {
        self.id == other.id
    }
}

impl Eq for IndexedBlock {}
