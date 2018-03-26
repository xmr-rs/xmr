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

use p2p::protocol::InboundSyncConnection;

use p2p::types::cn::cmd::{NewBlock, NewFluffyBlock, NewTransactions, RequestChain,
                          RequestFluffyMissingTx, RequestGetObjects, ResponseChainEntry,
                          ResponseGetObjects};

use types::{PeersRef, PeerIndex, LocalNodeRef};

pub struct InboundConnection {
    peer_index: PeerIndex,
    peers: PeersRef,
    local_node: LocalNodeRef,
}

impl InboundConnection {
    pub fn new(peer_index: PeerIndex, peers: PeersRef, local_node: LocalNodeRef) -> InboundConnection {
        InboundConnection {
            peer_index,
            peers,
            local_node,
        }
    }
}

impl InboundSyncConnection for InboundConnection {
    fn on_new_block(&self, arg: &NewBlock) {
        self.local_node
            .on_new_block(self.peer_index, arg);
    }

    fn on_new_fluffy_block(&self, arg: &NewFluffyBlock) {
        self.local_node
            .on_new_fluffy_block(self.peer_index, arg);
    }

    fn on_new_transactions(&self, arg: &NewTransactions) {
        self.local_node
            .on_new_transactions(self.peer_index, arg);
    }

    fn on_request_chain(&self, arg: &RequestChain) {
        self.local_node
            .on_request_chain(self.peer_index, arg);
    }

    fn on_request_fluffy_missing_tx(&self, arg: &RequestFluffyMissingTx) {
        self.local_node
            .on_request_fluffy_missing_tx(self.peer_index, arg);
    }

    fn on_request_get_objects(&self, arg: &RequestGetObjects) {
        self.local_node
            .on_request_get_objects(self.peer_index, arg);
    }

    fn on_response_chain_entry(&self, arg: &ResponseChainEntry) {
        self.local_node
            .on_response_chain_entry(self.peer_index, arg);
    }

    fn on_response_get_objects(&self, arg: &ResponseGetObjects) {
        self.local_node
            .on_response_get_objects(self.peer_index, arg);
    }

    fn on_support_flags(&self, arg: u32) {
        self.local_node
            .on_support_flags(self.peer_index, arg)
    }
}
