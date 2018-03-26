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

use std::collections::HashMap;

use parking_lot::RwLock;

use p2p::types::cn::CoreSyncData;
use p2p::protocol::OutboundSyncConnectionRef;

use types::PeerIndex;

pub trait Peers: Send + Sync {
    fn insert(&self,
              peer_index: PeerIndex,
              sync_data: &CoreSyncData,
              connection: OutboundSyncConnectionRef);
    fn last_sync_data(&self, peer_index: PeerIndex) -> Option<CoreSyncData>;
    fn connection(&self, peer_index: PeerIndex) -> Option<OutboundSyncConnectionRef>;

    fn misbehaving(&self, peer_index: PeerIndex, reason: &str);
}

pub struct Peer {
    connection: OutboundSyncConnectionRef,
    last_sync_data: CoreSyncData,
}

pub struct PeersImpl {
    peers: RwLock<HashMap<PeerIndex, Peer>>,
}

impl PeersImpl {
    pub fn new() -> PeersImpl {
        PeersImpl { peers: RwLock::new(HashMap::new()) }
    }
}

impl Peers for PeersImpl {
    fn insert(&self,
              peer_index: PeerIndex,
              sync_data: &CoreSyncData,
              connection: OutboundSyncConnectionRef) {
        trace!("peer insertion - #{}", peer_index);
        trace!("peer insertion - sync data - {:?}", sync_data);

        let peer = Peer {
            connection,
            last_sync_data: sync_data.clone(),
        };

        self.peers.write().insert(peer_index, peer);
    }

    fn last_sync_data(&self, peer_index: PeerIndex) -> Option<CoreSyncData> {
        self.peers
            .read()
            .get(&peer_index)
            .map(|peer| peer.last_sync_data.clone())
    }

    fn connection(&self, peer_index: PeerIndex) -> Option<OutboundSyncConnectionRef> {
        self.peers
            .read()
            .get(&peer_index)
            .map(|peer| peer.connection.clone())
    }

    fn misbehaving(&self, peer_index: PeerIndex, reason: &str) {
        if let Some(peer) = self.peers.write().remove(&peer_index) {
            warn!("Disconnecting from peer #{} due to misbehaviour: {}",
                  peer_index,
                  reason);
            peer.connection.close();
        }
    }
}
