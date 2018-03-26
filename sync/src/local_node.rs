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

use network::Network;
use p2p::types::cn::cmd::{NewBlock, NewFluffyBlock, NewTransactions, RequestChain,
                          RequestFluffyMissingTx, RequestGetObjects, ResponseChainEntry,
                          ResponseGetObjects};

use synchronization_client::{Client, SynchronizationClient};
use synchronization_executor::LocalSynchronizationTaskExecutor;
use synchronization_peers::PeersImpl;
use types::{ClientRef, PeersRef, ExecutorRef, StorageRef, PeerIndex};

pub struct LocalNode {
    client: ClientRef,
    executor: ExecutorRef,
    peers: PeersRef,
}

impl LocalNode {
    pub fn new(storage: StorageRef, network: Network) -> LocalNode {
        let peers = Arc::new(PeersImpl::new());
        let executor = Arc::new(LocalSynchronizationTaskExecutor::new(peers.clone()));
        let client =
            Arc::new(SynchronizationClient::new(executor.clone(), storage, network, peers.clone()));

        LocalNode {
            peers,
            executor,
            client,
        }
    }

    pub fn peers(&self) -> PeersRef {
        self.peers.clone()
    }

    pub fn on_connect(&self, peer_index: PeerIndex) {
        self.client.on_connect(peer_index);
    }

    pub fn on_new_block(&self, peer_index: PeerIndex, arg: &NewBlock) {
        self.client
            .on_new_block(peer_index, arg);
    }

    pub fn on_new_fluffy_block(&self, peer_index: PeerIndex, arg: &NewFluffyBlock) {
        self.client
            .on_new_fluffy_block(peer_index, arg);
    }

    pub fn on_new_transactions(&self, peer_index: PeerIndex, arg: &NewTransactions) {
        self.client
            .on_new_transactions(peer_index, arg);
    }

    pub fn on_request_chain(&self, peer_index: PeerIndex, arg: &RequestChain) {
        self.client
            .on_request_chain(peer_index, arg);
    }

    pub fn on_request_fluffy_missing_tx(&self, peer_index: PeerIndex, arg: &RequestFluffyMissingTx) {
        self.client
            .on_request_fluffy_missing_tx(peer_index, arg);
    }

    pub fn on_request_get_objects(&self, peer_index: PeerIndex, arg: &RequestGetObjects) {
        self.client
            .on_request_get_objects(peer_index, arg);
    }

    pub fn on_response_chain_entry(&self, peer_index: PeerIndex, arg: &ResponseChainEntry) {
        self.client
            .on_response_chain_entry(peer_index, arg);
    }

    pub fn on_response_get_objects(&self, peer_index: PeerIndex, arg: &ResponseGetObjects) {
        self.client
            .on_response_get_objects(peer_index, arg);
    }

    pub fn on_support_flags(&self, peer_index: PeerIndex, arg: u32) {
        self.client
            .on_support_flags(peer_index, arg);
    }
}
