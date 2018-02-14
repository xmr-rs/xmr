use hash::H256;

pub trait BlockProvider {
    fn block_hash(&self, height: u64) -> Option<H256>;
}
