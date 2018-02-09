use std::path::Path;

use parking_lot::RwLock;

use chain::BlockHeader;
use hash::H256;
use serialization::binary_deserialize as deserialize;

use block_chain::BlockChain;
use kv::{Key, Value, KeyState, KeyValueDatabase, DiskDb};

use store::Store;
use best_block::BestBlock;

const KEY_BEST_BLOCK_HEIGHT: &'static str = "best_block_height";
const KEY_BEST_BLOCK_ID: &'static str = "best_block_id";

/// A blockchain database.
pub struct BlockChainDatabase<DB: KeyValueDatabase> {
    db: DB,
    best_block: RwLock<BestBlock>,
}

impl BlockChainDatabase<DiskDb> {
    pub fn open<P: AsRef<Path>>(path: P) -> BlockChainDatabase<DiskDb> {
        // TODO: unwrap unwrap unwrap
        let db = DiskDb::open(path).unwrap();
        let best_block = RwLock::new(Self::read_best_block(&db).unwrap_or_default());
        BlockChainDatabase {
            db,
            best_block,
        }
    }
}

impl<DB> BlockChainDatabase<DB> where DB: KeyValueDatabase {
	fn read_best_block(db: &DB) -> Option<BestBlock> {
		let best_height = db.get(&Key::Meta(KEY_BEST_BLOCK_HEIGHT))
            .map(KeyState::into_option)
            .map(|x| x.and_then(Value::as_meta));
		let best_id = db.get(&Key::Meta(KEY_BEST_BLOCK_ID))
            .map(KeyState::into_option)
            .map(|x| x.and_then(Value::as_meta));

		match (best_height, best_id) {
			(Ok(None), Ok(None)) => None,
			(Ok(Some(height)), Ok(Some(id))) => Some(BestBlock {
				height: deserialize(&height),
				id: deserialize(&id),
			}),
			_ => panic!("Inconsistent DB"),
		}
	}
}

impl<DB> BlockChain for BlockChainDatabase<DB> where DB: KeyValueDatabase {
    fn top_id(&self) -> H256 {
        unimplemented!();
    }
}

impl<DB> Store for BlockChainDatabase<DB> where DB: KeyValueDatabase {
    fn best_block(&self) -> BestBlock {
        self.best_block.read().clone()
    }

    fn best_header(&self) -> BlockHeader {
        BlockHeader::default()
    }
}
