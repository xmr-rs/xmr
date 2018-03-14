use primitives::H256;

#[derive(Debug)]
pub enum BlockRef {
    Height(u64),
    Id(H256),
}

impl From<u64> for BlockRef {
    fn from(height: u64) -> BlockRef {
        BlockRef::Height(height)
    }
}


impl From<H256> for BlockRef {
    fn from(id: H256) -> BlockRef {
        BlockRef::Id(id)
    }
}
