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

use parking_lot::Mutex;

use network::Network;
use p2p::types::cn::cmd::{NewBlock, NewFluffyBlock, NewTransactions, RequestChain,
                          RequestFluffyMissingTx, RequestGetObjects, ResponseChainEntry,
                          ResponseGetObjects};

use synchronization_client_core::{ClientCore, SynchronizationClientCore};
use types::{ClientCoreRef, ExecutorRef, PeersRef, StorageRef, PeerIndex};

/// 1.) Verify peer synchronization data.
/// 1.1.) Send a RequestChain notification to the peer.
pub trait Client: Send + Sync + 'static {
    fn on_connect(&self, peer_index: PeerIndex);
    fn on_new_block(&self, peer_index: PeerIndex, arg: &NewBlock);
    fn on_new_fluffy_block(&self, peer_index: PeerIndex, arg: &NewFluffyBlock);
    fn on_new_transactions(&self, peer_index: PeerIndex, arg: &NewTransactions);
    fn on_request_chain(&self, peer_index: PeerIndex, arg: &RequestChain);
    fn on_request_fluffy_missing_tx(&self, peer_index: PeerIndex, arg: &RequestFluffyMissingTx);
    fn on_request_get_objects(&self, peer_index: PeerIndex, arg: &RequestGetObjects);
    fn on_response_chain_entry(&self, peer_index: PeerIndex, arg: &ResponseChainEntry);
    fn on_response_get_objects(&self, peer_index: PeerIndex, arg: &ResponseGetObjects);
    fn on_support_flags(&self, peer_index: PeerIndex, arg: u32);
}

pub struct SynchronizationClient {
    core: ClientCoreRef,
}

impl SynchronizationClient {
    pub fn new(executor: ExecutorRef,
               storage: StorageRef,
               network: Network,
               peers: PeersRef)
               -> SynchronizationClient {
        SynchronizationClient {
            core: Arc::new(Mutex::new(SynchronizationClientCore::new(executor,
                                                                     storage,
                                                                     network,
                                                                     peers))),
        }
    }
}

impl Client for SynchronizationClient {
    fn on_connect(&self, peer_index: PeerIndex) {
        self.core.lock().on_connect(peer_index)
    }

    fn on_new_block(&self, peer_index: PeerIndex, arg: &NewBlock) {
        self.core
            .lock()
            .on_new_block(peer_index, arg);
    }

    fn on_new_fluffy_block(&self, peer_index: PeerIndex, arg: &NewFluffyBlock) {
        self.core
            .lock()
            .on_new_fluffy_block(peer_index, arg);
    }

    fn on_new_transactions(&self, peer_index: PeerIndex, arg: &NewTransactions) {
        self.core
            .lock()
            .on_new_transactions(peer_index, arg);
    }

    fn on_request_chain(&self, peer_index: PeerIndex, arg: &RequestChain) {
        self.core
            .lock()
            .on_request_chain(peer_index, arg);
    }

    fn on_request_fluffy_missing_tx(&self, peer_index: PeerIndex, arg: &RequestFluffyMissingTx) {
        self.core
            .lock()
            .on_request_fluffy_missing_tx(peer_index, arg);
    }

    fn on_request_get_objects(&self, peer_index: PeerIndex, arg: &RequestGetObjects) {
        self.core
            .lock()
            .on_request_get_objects(peer_index, arg);
    }

    fn on_response_chain_entry(&self, peer_index: PeerIndex, arg: &ResponseChainEntry) {
        self.core
            .lock()
            .on_response_chain_entry(peer_index, arg);
    }

    fn on_response_get_objects(&self, peer_index: PeerIndex, arg: &ResponseGetObjects) {
        self.core
            .lock()
            .on_response_get_objects(peer_index, arg);
    }

    fn on_support_flags(&self, peer_index: PeerIndex, arg: u32) {
        self.core
            .lock()
            .on_support_flags(peer_index, arg);
    }
}
