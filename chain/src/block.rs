use block_header::BlockHeader;
use transaction::Transaction;
use hash::H256;

/// A block.
#[derive(Debug)]
pub struct Block {
    pub header: BlockHeader,
    pub miner_tx: Transaction,
    pub tx_hashes: Vec<H256>,
}
