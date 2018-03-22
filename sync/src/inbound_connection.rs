use p2p::protocol::InboundSyncConnection;
use p2p::types::PeerId;

use p2p::types::cn::cmd::{NewBlockRequest, NewFluffyBlockRequest, NewTransactionsRequest, RequestChainRequest, RequestFluffyMissingTxRequest, RequestGetObjectsRequest, ResponseChainEntryRequest, ResponseGetObjectsRequest};

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
    fn on_new_block(&self, _req: &NewBlockRequest) {}
    fn on_new_fluffy_block(&self, _req: &NewFluffyBlockRequest) {}
    fn on_new_transactions(&self, _req: &NewTransactionsRequest) {}
    fn on_request_chain(&self, _req: &RequestChainRequest) {}
    fn on_request_fluffy_missing_tx(&self, _req: &RequestFluffyMissingTxRequest) {}
    fn on_request_get_objects(&self, _req: &RequestGetObjectsRequest) {}
    fn on_response_chain_entry(&self, req: &ResponseChainEntryRequest) {
        self.local_node.on_response_chain_entry(self.peer_id, req);
    }
    fn on_response_get_objects(&self, _req: &ResponseGetObjectsRequest) {}
}
