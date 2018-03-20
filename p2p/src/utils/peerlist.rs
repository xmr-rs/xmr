use std::collections::{HashMap, LinkedList};
use std::net::SocketAddr;

use portable_storage_utils::stl::StlLinkedList;

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

    pub fn remove(&mut self, addr: &SocketAddr) -> Option<PeerlistEntry> {
        self.list.remove(&addr)
    }

    pub fn stl_peerlist(&self) -> StlLinkedList<PeerlistEntry> {
        let mut ll = LinkedList::new();
        for peer in self.list.iter() {
            ll.push_back(peer.1.clone())
        }

        ll.into()
    }
}
