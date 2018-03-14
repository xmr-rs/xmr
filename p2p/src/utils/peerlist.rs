use std::collections::HashMap;
use std::net::SocketAddr;
use types::PeerlistEntry;

#[derive(Debug)]
pub struct Peerlist {
    pub list: HashMap<SocketAddr, PeerlistEntry>,
}

impl Peerlist {
    pub fn new() -> Peerlist {
        Peerlist { list: HashMap::new() }
    }

    pub fn insert(&mut self, address: SocketAddr, entry: PeerlistEntry) {
        self.list.insert(address, entry);
    }
}
