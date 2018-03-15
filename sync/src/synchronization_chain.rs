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
}
