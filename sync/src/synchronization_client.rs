use std::sync::Arc;

use parking_lot::Mutex;

use p2p::types::PeerId;

use synchronization_client_core::{ClientCore, SynchronizationClientCore};
use types::{ClientCoreRef, ExecutorRef, StorageRef};

pub trait Client: Send + Sync + 'static  {
    fn on_connect(&self, peer_id: PeerId);
}

pub struct SynchronizationClient {
    core: ClientCoreRef,
}

impl SynchronizationClient {
    pub fn new(executor: ExecutorRef, storage: StorageRef) -> SynchronizationClient {
        SynchronizationClient {
            core: Arc::new(Mutex::new(SynchronizationClientCore::new(executor, storage)))
        }
    }
}

impl Client for SynchronizationClient {
    fn on_connect(&self, peer_id: PeerId) {
        self.core.lock().on_connect(peer_id)
    } 
}
