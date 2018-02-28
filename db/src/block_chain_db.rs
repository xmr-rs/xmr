use std::path::Path;

use sanakirja;

use parking_lot::RwLock;

use chain::BlockHeader;
use hash::H256;
use bytes::{Buf, IntoBuf, LittleEndian};

use block_chain::BlockChain;
use block_provider::BlockProvider;

use kv::{Key, Value, KeyState, KeyValueDatabase, DiskDb};

use store::Store;
use best_block::BestBlock;

use error::Error;

const KEY_BEST_BLOCK_HEIGHT: &'static str = "best_block_height";
const KEY_BEST_BLOCK_ID: &'static str = "best_block_id";

/// A blockchain database.
#[derive(Debug)]
pub struct BlockChainDatabase<DB: KeyValueDatabase> {
    db: DB,
    best_block: RwLock<BestBlock>,
}

impl BlockChainDatabase<DiskDb> {
    pub fn open<P: AsRef<Path>>(path: P) -> Result<BlockChainDatabase<DiskDb>, Error> {
        let db = match DiskDb::open(path) {
            Ok(db) => db,
            Err(sanakirja::Error::IO(e)) => return Err(Error::Io(e)),
            Err(sanakirja::Error::NotEnoughSpace) => panic!("couldn't \"mmap\" database"),
            Err(sanakirja::Error::Poison) => return Err(Error::AlreadyOpen),
        };

        let best_block = RwLock::new(Self::read_best_block(&db).unwrap_or_default());

        Ok(BlockChainDatabase {
            db,
            best_block,
        })
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
			(Ok(Some(height)), Ok(Some(id))) => {
                let mut buf = height.into_buf();
                let height = buf.get_u64::<LittleEndian>();

                let id = H256::from_bytes(id);

                Some(BestBlock {
                    height,
                    id,
                })
            },
			_ => panic!("Inconsistent DB"),
		}
	}
    
    fn get(&self, key: Key) -> Option<Value> {
        self.db.get(&key).expect("db value to be fine").into_option()
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

impl<DB> BlockProvider for BlockChainDatabase<DB> where DB: KeyValueDatabase {
    fn block_hash(&self, height: u64) -> Option<H256> {
        self.get(Key::BlockHash(height))
            .and_then(Value::as_block_hash)
    }
}
