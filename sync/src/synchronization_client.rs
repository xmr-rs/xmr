use std::sync::Arc;

use parking_lot::Mutex;

use network::Network;
use p2p::types::PeerId;
use p2p::types::cn::cmd::ResponseChainEntry;

use synchronization_client_core::{ClientCore, SynchronizationClientCore};
use types::{ClientCoreRef, ExecutorRef, PeersRef, StorageRef};

/// 1.) Verify peer synchronization data.
/// 1.1.) Send a RequestChain notification to the peer.
pub trait Client: Send + Sync + 'static {
    fn on_connect(&self, peer_id: PeerId);
    fn on_response_chain_entry(&self, peer_id: PeerId, notification: &ResponseChainEntry);
}

pub struct SynchronizationClient {
    core: ClientCoreRef,
}

impl SynchronizationClient {
    pub fn new(executor: ExecutorRef,
               storage: StorageRef,
               network: Network,
               peers: PeersRef)
               -> SynchronizationClient {
        SynchronizationClient {
            core: Arc::new(Mutex::new(SynchronizationClientCore::new(executor,
                                                                     storage,
                                                                     network,
                                                                     peers))),
        }
    }
}

impl Client for SynchronizationClient {
    fn on_connect(&self, peer_id: PeerId) {
        self.core.lock().on_connect(peer_id)
    }

    fn on_response_chain_entry(&self, peer_id: PeerId, notification: &ResponseChainEntry) {
        self.core
            .lock()
            .on_response_chain_entry(peer_id, notification)
    }
}
