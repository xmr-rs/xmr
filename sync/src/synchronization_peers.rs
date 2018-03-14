use std::sync::Arc;
use std::collections::HashMap;

use parking_lot::RwLock;

use p2p::types::PeerId;
use p2p::types::cn::CoreSyncData;
use p2p::protocol::OutboundSyncConnectionRef;

pub trait Peers: Send + Sync {
    fn insert(&self,
              peer_id: PeerId,
              sync_data: &CoreSyncData,
              connection: OutboundSyncConnectionRef);
}

pub type PeersRef = Arc<Peers>;

pub struct Peer {
    connection: OutboundSyncConnectionRef,
    last_sync_data: CoreSyncData,
}

pub struct PeersImpl {
    peers: RwLock<HashMap<PeerId, Peer>>,
}

impl PeersImpl {
    pub fn new() -> PeersImpl {
        PeersImpl { peers: RwLock::new(HashMap::new()) }
    }
}

impl Peers for PeersImpl {
    fn insert(&self,
              peer_id: PeerId,
              sync_data: &CoreSyncData,
              connection: OutboundSyncConnectionRef) {
        trace!("peer insertion - id - {:?}", peer_id);
        trace!("peer insertion - sync data - {:?}", sync_data);

        let peer = Peer {
            connection,
            last_sync_data: sync_data.clone(),
        };

        self.peers.write().insert(peer_id, peer);
    }
}
