use std::path::Path;
use std::collections::LinkedList;

use sanakirja;
use parking_lot::RwLock;
use bytes::{Buf, IntoBuf, LittleEndian};

use chain::IndexedBlock;
use format::to_binary;
use primitives::H256;
use storage::{BestBlock, BlockChain, BlockProvider, IndexedBlockProvider, BlockRef, Store,
              CanonStore};

use kv::{Key, Value, KeyValue, KeyState, KeyValueDatabase, DiskDb, Transaction};
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

        Ok(BlockChainDatabase { db, best_block })
    }
}

impl<DB> BlockChainDatabase<DB>
    where DB: KeyValueDatabase
{
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

                Some(BestBlock { height, id })
            }
            _ => panic!("Inconsistent DB"),
        }
    }

    fn get(&self, key: Key) -> Option<Value> {
        self.db
            .get(&key)
            .expect("db value to be fine")
            .into_option()
    }

    pub fn insert(&self, block: IndexedBlock) -> Result<(), Error> {
        if self.contains_block(block.id().clone().into()) {
            return Ok(());
        }

        let parent_id = block.raw.header.prev_id.clone();
        if !self.contains_block(parent_id.clone().into()) && !parent_id.is_zero() {
            return Err(Error::UnknownParent);
        }

        let mut update = Transaction::new();
        update.insert(KeyValue::Block(block.id().clone(), block.raw.clone()));

        // TODO: transactions?

        self.db.write(update).map_err(Error::DatabaseError)
    }

    fn contains_block(&self, block_ref: BlockRef) -> bool {
        self.resolve_id(block_ref)
            .and_then(|id| self.get(Key::Block(id)))
            .is_some()
    }

    pub fn canonize(&self, id: &H256) -> Result<(), Error> {
        let mut best_block = self.best_block.write();
        let block = match self.indexed_block(id.clone().into()) {
            Some(block) => block,
            None => return Err(Error::CannotCanonize),
        };

        if best_block.id != block.raw.header.prev_id {
            return Err(Error::CannotCanonize);
        }

        let new_best_block = BestBlock {
            id: id.clone(),
            height: if block.raw.header.prev_id.is_zero() {
                assert_eq!(best_block.height, 0);
                0
            } else {
                best_block.height + 1
            },
        };

        let mut update = Transaction::new();
        update.insert(KeyValue::BlockId(new_best_block.height, new_best_block.id.clone()));
        update.insert(KeyValue::BlockHeight(new_best_block.id.clone(), new_best_block.height));
		update.insert(KeyValue::Meta(KEY_BEST_BLOCK_ID, to_binary(&new_best_block.id)));
		update.insert(KeyValue::Meta(KEY_BEST_BLOCK_HEIGHT, to_binary(&new_best_block.height)));

        // TODO: transactions

        self.db.write(update).map_err(Error::DatabaseError)?;
        *best_block = new_best_block;
        Ok(())
    }


    fn resolve_id(&self, block_ref: BlockRef) -> Option<H256> {
        match block_ref {
            BlockRef::Height(height) => self.block_id(height),
            BlockRef::Id(id) => Some(id),
        }
    }
}

impl<DB> BlockChain for BlockChainDatabase<DB>
    where DB: KeyValueDatabase
{
    fn insert(&self, block: IndexedBlock) -> Result<(), String> {
        BlockChainDatabase::insert(self, block).map_err(|e| format!("{}", e))
    }

    fn canonize(&self, id: &H256) -> Result<(), String> {
        BlockChainDatabase::canonize(self, id).map_err(|e| format!("{}", e))
    }
}

impl<DB> Store for BlockChainDatabase<DB>
    where DB: KeyValueDatabase
{
    fn best_block(&self) -> BestBlock {
        self.best_block.read().clone()
    }

    fn height(&self) -> u64 {
        self.best_block.read().height + 1
    }

    fn short_chain_history(&self) -> LinkedList<H256> {
        let height = self.height();
        if height == 0 {
            return LinkedList::new();
        }

        let mut ids = LinkedList::new();
        let mut i = 0u64;
        let mut current_multiplier = 1u64;
        let mut genesis_included = false;
        let mut current_back_offset = 1;
        while current_back_offset < height {
            let id = self.block_id(height - current_back_offset).unwrap();
            ids.push_back(id);

            if height - current_back_offset == 0 {
                genesis_included = true;
            }

            if i < 10 {
                current_back_offset += 1;
            } else {
                current_multiplier *= 2;
                current_back_offset += current_multiplier;
            }
            i += 1;
        }

        if !genesis_included {
            ids.push_back(self.block_id(0).unwrap());
        }

        ids
    }
}

impl<DB> CanonStore for BlockChainDatabase<DB>
    where DB: KeyValueDatabase
{
    fn as_store(&self) -> &Store {
        &*self
    }
}

impl<DB> BlockProvider for BlockChainDatabase<DB>
    where DB: KeyValueDatabase
{
    fn block_id(&self, height: u64) -> Option<H256> {
        self.get(Key::BlockId(height))
            .and_then(Value::as_block_id)
    }
}

impl<DB> IndexedBlockProvider for BlockChainDatabase<DB>
    where DB: KeyValueDatabase
{
    fn indexed_block(&self, block_ref: BlockRef) -> Option<IndexedBlock> {
        self.resolve_id(block_ref)
            .and_then(|id| {
                          self.get(Key::Block(id.clone()))
                              .and_then(Value::as_block)
                              .map(|block| IndexedBlock::new(block, id))
                      })
    }
}
