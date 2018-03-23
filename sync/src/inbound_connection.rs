use p2p::protocol::InboundSyncConnection;

use p2p::types::PeerId;
use p2p::types::cn::cmd::{NewBlock, NewFluffyBlock, NewTransactions, RequestChain,
                          RequestFluffyMissingTx, RequestGetObjects, ResponseChainEntry,
                          ResponseGetObjects};

use types::{PeersRef, LocalNodeRef};

pub struct InboundConnection {
    peer_id: PeerId,
    peers: PeersRef,
    local_node: LocalNodeRef,
}

impl InboundConnection {
    pub fn new(peer_id: PeerId, peers: PeersRef, local_node: LocalNodeRef) -> InboundConnection {
        InboundConnection {
            peer_id,
            peers,
            local_node,
        }
    }
}

impl InboundSyncConnection for InboundConnection {
    fn on_new_block(&self, arg: &NewBlock) {
        self.local_node
            .on_new_block(self.peer_id, arg);
    }

    fn on_new_fluffy_block(&self, arg: &NewFluffyBlock) {
        self.local_node
            .on_new_fluffy_block(self.peer_id, arg);
    }

    fn on_new_transactions(&self, arg: &NewTransactions) {
        self.local_node
            .on_new_transactions(self.peer_id, arg);
    }

    fn on_request_chain(&self, arg: &RequestChain) {
        self.local_node
            .on_request_chain(self.peer_id, arg);
    }

    fn on_request_fluffy_missing_tx(&self, arg: &RequestFluffyMissingTx) {
        self.local_node
            .on_request_fluffy_missing_tx(self.peer_id, arg);
    }

    fn on_request_get_objects(&self, arg: &RequestGetObjects) {
        self.local_node
            .on_request_get_objects(self.peer_id, arg);
    }

    fn on_response_chain_entry(&self, arg: &ResponseChainEntry) {
        self.local_node
            .on_response_chain_entry(self.peer_id, arg);
    }

    fn on_response_get_objects(&self, arg: &ResponseGetObjects) {
        self.local_node
            .on_response_get_objects(self.peer_id, arg);
    }

    fn on_support_flags(&self, arg: u32) {
        self.local_node
            .on_support_flags(self.peer_id, arg)
    }
}
