use std::path::Path;

use hash::H256;

use block_chain::BlockChain;
use kv::{KeyValueDatabase, DiskDb};

/// A blockchain database.
pub struct BlockChainDatabase<DB: KeyValueDatabase> {
    db: DB,
}

impl BlockChainDatabase<DiskDb> {
    pub fn from_path<P: AsRef<Path>>(path: P) -> BlockChainDatabase<DiskDb> {
        // TODO: unwrap unwrap unwrap
        BlockChainDatabase {
            db: DiskDb::open(path).unwrap(),
        }
    }
}

impl<DB> BlockChain for BlockChainDatabase<DB> where DB: KeyValueDatabase {
    fn top_id(&self) -> H256 {
        unimplemented!();
    }
}
