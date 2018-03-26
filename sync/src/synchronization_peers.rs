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

use p2p::types::PeerId;
use p2p::types::cn::CoreSyncData;
use p2p::protocol::OutboundSyncConnectionRef;

pub trait Peers: Send + Sync {
    fn insert(&self,
              peer_id: PeerId,
              sync_data: &CoreSyncData,
              connection: OutboundSyncConnectionRef);
    fn last_sync_data(&self, peer_id: PeerId) -> Option<CoreSyncData>;
    fn connection(&self, peer_id: PeerId) -> Option<OutboundSyncConnectionRef>;

    fn misbehaving(&self, peer_id: PeerId, reason: &str);
}

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

        let is_inserted = self.peers.write().get(&peer_id).is_some();

        if is_inserted {
            let reason = "peer did a double handshake.";
            self.misbehaving(peer_id, reason);
        } else {
            let peer = Peer {
                connection,
                last_sync_data: sync_data.clone(),
            };

            self.peers.write().insert(peer_id, peer);
        }
    }

    fn last_sync_data(&self, peer_id: PeerId) -> Option<CoreSyncData> {
        self.peers
            .read()
            .get(&peer_id)
            .map(|peer| peer.last_sync_data.clone())
    }

    fn connection(&self, peer_id: PeerId) -> Option<OutboundSyncConnectionRef> {
        self.peers
            .read()
            .get(&peer_id)
            .map(|peer| peer.connection.clone())
    }

    fn misbehaving(&self, peer_id: PeerId, reason: &str) {
        if let Some(peer) = self.peers.write().remove(&peer_id) {
            warn!("Disconnecting from peer {} due to misbehaviour: {}",
                  peer_id,
                  reason);
            peer.connection.close();
        }
    }
}
