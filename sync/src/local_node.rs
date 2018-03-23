use std::sync::Arc;

use network::Network;
use p2p::types::PeerId;
use p2p::types::cn::cmd::{NewBlock, NewFluffyBlock, NewTransactions, RequestChain,
                          RequestFluffyMissingTx, RequestGetObjects, ResponseChainEntry,
                          ResponseGetObjects};

use synchronization_client::{Client, SynchronizationClient};
use synchronization_executor::LocalSynchronizationTaskExecutor;
use synchronization_peers::PeersImpl;
use types::{ClientRef, PeersRef, ExecutorRef, StorageRef};

pub struct LocalNode {
    client: ClientRef,
    executor: ExecutorRef,
    peers: PeersRef,
}

impl LocalNode {
    pub fn new(storage: StorageRef, network: Network) -> LocalNode {
        let peers = Arc::new(PeersImpl::new());
        let executor = Arc::new(LocalSynchronizationTaskExecutor::new(peers.clone()));
        let client =
            Arc::new(SynchronizationClient::new(executor.clone(), storage, network, peers.clone()));

        LocalNode {
            peers,
            executor,
            client,
        }
    }

    pub fn peers(&self) -> PeersRef {
        self.peers.clone()
    }

    pub fn on_connect(&self, peer_id: PeerId) {
        self.client.on_connect(peer_id);
    }

    pub fn on_new_block(&self, peer_id: PeerId, arg: &NewBlock) {
        self.client
            .on_new_block(peer_id, arg);
    }

    pub fn on_new_fluffy_block(&self, peer_id: PeerId, arg: &NewFluffyBlock) {
        self.client
            .on_new_fluffy_block(peer_id, arg);
    }

    pub fn on_new_transactions(&self, peer_id: PeerId, arg: &NewTransactions) {
        self.client
            .on_new_transactions(peer_id, arg);
    }

    pub fn on_request_chain(&self, peer_id: PeerId, arg: &RequestChain) {
        self.client
            .on_request_chain(peer_id, arg);
    }

    pub fn on_request_fluffy_missing_tx(&self, peer_id: PeerId, arg: &RequestFluffyMissingTx) {
        self.client
            .on_request_fluffy_missing_tx(peer_id, arg);
    }

    pub fn on_request_get_objects(&self, peer_id: PeerId, arg: &RequestGetObjects) {
        self.client
            .on_request_get_objects(peer_id, arg);
    }

    pub fn on_response_chain_entry(&self, peer_id: PeerId, arg: &ResponseChainEntry) {
        self.client
            .on_response_chain_entry(peer_id, arg);
    }

    pub fn on_response_get_objects(&self, peer_id: PeerId, arg: &ResponseGetObjects) {
        self.client
            .on_response_get_objects(peer_id, arg);
    }

    pub fn on_support_flags(&self, peer_id: PeerId, arg: u32) {
        self.client
            .on_support_flags(peer_id, arg);
    }
}
