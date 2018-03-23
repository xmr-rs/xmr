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
    fn on_new_block(&self, _arg: &NewBlock) {}
    fn on_new_fluffy_block(&self, _arg: &NewFluffyBlock) {}
    fn on_new_transactions(&self, _arg: &NewTransactions) {}
    fn on_request_chain(&self, _arg: &RequestChain) {}
    fn on_request_fluffy_missing_tx(&self, _arg: &RequestFluffyMissingTx) {}
    fn on_request_get_objects(&self, _arg: &RequestGetObjects) {}
    fn on_response_chain_entry(&self, arg: &ResponseChainEntry) {
        self.local_node
            .on_response_chain_entry(self.peer_id, arg);
    }
    fn on_response_get_objects(&self, _arg: &ResponseGetObjects) {}
    fn on_support_flags(&self, _arg: u32) {}
}
