use std::collections::HashMap;
use std::net::SocketAddr;
use protocol::PeerlistEntry;

#[derive(Debug)]
pub struct Peer {
    pub entry: PeerlistEntry,
}

#[derive(Debug)]
pub struct Peerlist {
    pub list: HashMap<SocketAddr, Peer>,
}

impl Peerlist {
    pub fn new() -> Peerlist {
        Peerlist {
            list: HashMap::new(),
        }
    }
}
