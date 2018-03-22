use std::sync::Arc;

use network::Network;
use p2p::types::PeerId;
use p2p::types::cn::cmd::ResponseChainEntryRequest;

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

    pub fn on_response_chain_entry(&self,
                                   peer_id: PeerId,
                                   notification: &ResponseChainEntryRequest) {
        self.client.on_response_chain_entry(peer_id, notification)
    }
}
