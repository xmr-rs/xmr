use std::sync::Arc;

use parking_lot::Mutex;

use network::Network;
use p2p::types::PeerId;
use p2p::types::cn::cmd::{NewBlock, NewFluffyBlock, NewTransactions, RequestChain,
                          RequestFluffyMissingTx, RequestGetObjects, ResponseChainEntry,
                          ResponseGetObjects};

use synchronization_client_core::{ClientCore, SynchronizationClientCore};
use types::{ClientCoreRef, ExecutorRef, PeersRef, StorageRef};

/// 1.) Verify peer synchronization data.
/// 1.1.) Send a RequestChain notification to the peer.
pub trait Client: Send + Sync + 'static {
    fn on_connect(&self, peer_id: PeerId);
    fn on_new_block(&self, peer_id: PeerId, arg: &NewBlock);
    fn on_new_fluffy_block(&self, peer_id: PeerId, arg: &NewFluffyBlock);
    fn on_new_transactions(&self, peer_id: PeerId, arg: &NewTransactions);
    fn on_request_chain(&self, peer_id: PeerId, arg: &RequestChain);
    fn on_request_fluffy_missing_tx(&self, peer_id: PeerId, arg: &RequestFluffyMissingTx);
    fn on_request_get_objects(&self, peer_id: PeerId, arg: &RequestGetObjects);
    fn on_response_chain_entry(&self, peer_id: PeerId, arg: &ResponseChainEntry);
    fn on_response_get_objects(&self, peer_id: PeerId, arg: &ResponseGetObjects);
    fn on_support_flags(&self, peer_id: PeerId, arg: u32);
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

    fn on_new_block(&self, peer_id: PeerId, arg: &NewBlock) {
        self.core
            .lock()
            .on_new_block(peer_id, arg);
    }

    fn on_new_fluffy_block(&self, peer_id: PeerId, arg: &NewFluffyBlock) {
        self.core
            .lock()
            .on_new_fluffy_block(peer_id, arg);
    }

    fn on_new_transactions(&self, peer_id: PeerId, arg: &NewTransactions) {
        self.core
            .lock()
            .on_new_transactions(peer_id, arg);
    }

    fn on_request_chain(&self, peer_id: PeerId, arg: &RequestChain) {
        self.core
            .lock()
            .on_request_chain(peer_id, arg);
    }

    fn on_request_fluffy_missing_tx(&self, peer_id: PeerId, arg: &RequestFluffyMissingTx) {
        self.core
            .lock()
            .on_request_fluffy_missing_tx(peer_id, arg);
    }

    fn on_request_get_objects(&self, peer_id: PeerId, arg: &RequestGetObjects) {
        self.core
            .lock()
            .on_request_get_objects(peer_id, arg);
    }

    fn on_response_chain_entry(&self, peer_id: PeerId, arg: &ResponseChainEntry) {
        self.core
            .lock()
            .on_response_chain_entry(peer_id, arg);
    }

    fn on_response_get_objects(&self, peer_id: PeerId, arg: &ResponseGetObjects) {
        self.core
            .lock()
            .on_response_get_objects(peer_id, arg);
    }

    fn on_support_flags(&self, peer_id: PeerId, arg: u32) {
        self.core
            .lock()
            .on_support_flags(peer_id, arg);
    }
}
