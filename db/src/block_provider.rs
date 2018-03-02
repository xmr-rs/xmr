use primitives::H256;

pub trait BlockProvider {
    fn block_id(&self, height: u64) -> Option<H256>;
}
