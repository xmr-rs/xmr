use p2p::protocol::{LocalSyncNode, LocalSyncNodeRef, OutboundSyncConnectionRef};
use p2p::types::cn::CoreSyncData;
use p2p::types::PeerId;

use synchronization_peers::PeersRef;

pub struct ConnectionFactory {
    peers: PeersRef,
}

impl ConnectionFactory {
    pub fn new(peers: PeersRef) -> ConnectionFactory {
        ConnectionFactory { peers }
    }

    pub fn boxed(self) -> LocalSyncNodeRef {
        Box::new(self)
    }
}

impl LocalSyncNode for ConnectionFactory {
    fn new_sync_connection(&self,
                           peer_id: PeerId,
                           sync_data: &CoreSyncData,
                           connection: OutboundSyncConnectionRef) {
        self.peers.insert(peer_id, sync_data, connection);
    }
}
