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

use std::sync::Arc;
use std::sync::atomic::{AtomicUsize, Ordering};

use p2p::protocol::{LocalSyncNode, LocalSyncNodeRef, OutboundSyncConnectionRef,
                    InboundSyncConnectionRef};
use p2p::types::cn::CoreSyncData;

use inbound_connection::InboundConnection;
use types::{LocalNodeRef, PeersRef};

pub struct ConnectionFactory {
    peers: PeersRef,
    local_node: LocalNodeRef,
    counter: AtomicUsize,
}

impl ConnectionFactory {
    pub fn new(local_node: LocalNodeRef) -> ConnectionFactory {
        ConnectionFactory {
            peers: local_node.peers(),
            local_node,
            counter: AtomicUsize::new(0),
        }
    }

    pub fn boxed(self) -> LocalSyncNodeRef {
        Box::new(self)
    }
}

impl LocalSyncNode for ConnectionFactory {
    fn new_sync_connection(&self,
                           sync_data: &CoreSyncData,
                           connection: OutboundSyncConnectionRef)
                           -> InboundSyncConnectionRef {
		let peer_index = self.counter.fetch_add(1, Ordering::SeqCst) + 1;
        info!("Connecting with peer #{}", peer_index);

        self.peers.insert(peer_index, sync_data, connection);
        self.local_node.on_connect(peer_index);

        Arc::new(InboundConnection::new(peer_index, self.peers.clone(), self.local_node.clone()))
    }
}
