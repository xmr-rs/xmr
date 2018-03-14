use std::sync::Arc;
use synchronization_peers::{PeersRef, PeersImpl};

pub struct LocalNode {
    peers: PeersRef,
}

impl LocalNode {
    pub fn new() -> LocalNode {
        LocalNode {
            peers: Arc::new(PeersImpl::new()),
        }
    }

    pub fn peers(&self) -> PeersRef {
        self.peers.clone()
    }
}
