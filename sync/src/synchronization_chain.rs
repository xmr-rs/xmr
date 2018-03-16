use primitives::H256;

use types::StorageRef;

pub struct Chain {
    storage: StorageRef,
}

impl Chain {
    pub fn new(storage: StorageRef) -> Chain {
        Chain { storage }
    }

    pub fn storage(&self) -> StorageRef {
        self.storage.clone()
    }

    pub fn have_block(&self, id: H256) -> bool {
        self.storage.indexed_block(id.into()).is_some()
    }

    pub fn height(&self) -> u64 {
        self.storage.height()
    }
}
