// Xmr, Monero node.
// Copyright (C) 2018  Jean Pierre Dudey
//
// This program is free software: you can redistribute it and/or modify
// it under the terms of the GNU General Public License as published by
// the Free Software Foundation, either version 3 of the License, or
// (at your option) any later version.
//
// This program is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
// GNU General Public License for more details.
//
// You should have received a copy of the GNU General Public License
// along with this program.  If not, see <http://www.gnu.org/licenses/>.

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
