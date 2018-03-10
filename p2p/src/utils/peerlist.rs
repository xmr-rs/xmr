use std::collections::HashMap;
use std::net::SocketAddr;
use types::PeerlistEntry;

#[derive(Debug)]
pub struct Peerlist {
    pub list: HashMap<SocketAddr, PeerlistEntry>,
}

impl Peerlist {
    pub fn new() -> Peerlist {
        Peerlist {
            list: HashMap::new(),
        }
    }
}