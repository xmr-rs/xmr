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

use network::Network;

use p2p::types::cn::cmd::{NewBlock, NewFluffyBlock, NewTransactions, RequestChain,
                          RequestFluffyMissingTx, RequestGetObjects, ResponseChainEntry,
                          ResponseGetObjects};

use synchronization_chain::Chain;
use synchronization_executor::{Task, TaskExecutor};
use types::{ExecutorRef, PeersRef, StorageRef, PeerIndex};

pub trait ClientCore: Send + Sync + 'static {
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

pub struct SynchronizationClientCore {
    executor: ExecutorRef,
    chain: Chain,
    network: Network,
    peers: PeersRef,
    contexes: RwLock<HashMap<PeerIndex, Context>>,
}

impl SynchronizationClientCore {
    pub fn new(executor: ExecutorRef,
               storage: StorageRef,
               network: Network,
               peers: PeersRef)
               -> SynchronizationClientCore {
        SynchronizationClientCore {
            executor,
            chain: Chain::new(storage),
            network,
            peers,
            contexes: RwLock::new(HashMap::new()),
        }
    }

    fn context_write<F>(&self, peer_index: &PeerIndex, f: F)
        where F: FnOnce(&mut Context)
    {
        let mut contexes = self.contexes.write();
        let context = contexes.get_mut(peer_index).expect("context should exist");
        f(context)
    }

    fn misbehaving(&self, peer_index: PeerIndex, reason: &str) {
        self.peers.misbehaving(peer_index, reason);
        self.contexes.write().remove(&peer_index);
    }

    fn verify_sync_data(&self, peer_index: PeerIndex) -> Option<SyncState> {
        let sync_data = self.peers
            .last_sync_data(peer_index)
            .expect("peer id should be valid");

        info!("Verifying peer #{} synchronization data", peer_index);
        debug!("Peer {} sync data: {:?}", peer_index, sync_data);

        if sync_data.current_height > 0 {
            let peer_top = sync_data.current_height - 1;
            let version = self.network
                .hard_forks()
                .ideal_version_for_height(peer_top);

            if version >= 6 && sync_data.top_version != version {
                warn!("Peer #{} claims higher version than we think (v{} for block height {}) -
                      we may be forked from the network and a software upgrade is needed.",
                      peer_index,
                      sync_data.top_version,
                      peer_top);

                self.misbehaving(peer_index, "peer uses different version than us");

                return None;
            }
        }

        let context = Context {
            remote_blockchain_height: sync_data.current_height,
            last_response_height: None,
            support_flags: None,
        };

        self.contexes.write().insert(peer_index, context);

        if self.chain.have_block(sync_data.top_id) {
            let our_height = self.chain.height();
            if our_height == sync_data.current_height {
                info!("Peer #{} is synchronized with us.", peer_index);
                Some(SyncState::Synchronized)
            } else {
                Some(SyncState::Synchronizing)
            }
        } else {
            Some(SyncState::Synchronizing)
        }
    }
}

impl ClientCore for SynchronizationClientCore {
    fn on_connect(&self, peer_index: PeerIndex) {
        use p2p::types::cn::cmd::RequestChain;

        info!("Synchronizing with peer #{}", peer_index);

        let state = self.verify_sync_data(peer_index);
        match state {
            Some(SyncState::Synchronizing) => {
                let block_ids = self.chain.storage().short_chain_history();
                let request = RequestChain { block_ids: block_ids.into() };
                self.executor
                    .execute(Task::RequestChain(peer_index, request));
            }
            Some(SyncState::Synchronized) => {}
            None => { /* not valid sync info */ }
        }
    }

    fn on_new_block(&self, _peer_index: PeerIndex, _arg: &NewBlock) {
    }

    fn on_new_fluffy_block(&self, _peer_index: PeerIndex, _arg: &NewFluffyBlock) {
    }

    fn on_new_transactions(&self, _peer_index: PeerIndex, _arg: &NewTransactions) {
    }

    fn on_request_chain(&self, _peer_index: PeerIndex, _arg: &RequestChain) {
    }

    fn on_request_fluffy_missing_tx(&self, _peer_index: PeerIndex, _arg: &RequestFluffyMissingTx) {
    }

    fn on_request_get_objects(&self, _peer_index: PeerIndex, _arg: &RequestGetObjects) {
    }

    fn on_response_chain_entry(&self, peer_index: PeerIndex, arg: &ResponseChainEntry) {
        if arg.block_ids.len() == 0 {
            self.misbehaving(peer_index, "peer sent empty `block_ids` field");
            return;
        }

        if arg.total_height < arg.block_ids.len() as u64 ||
           arg.start_height > arg.total_height - arg.block_ids.len() as u64 {
            self.misbehaving(peer_index, "peer sent invalid start/nblocks/height.");
            return;
        }

        let remote_blockchain_height = arg.total_height;
        let last_response_height = arg.start_height + arg.block_ids.len() as u64 - 1;

        self.context_write(&peer_index, move |context| {
            context.remote_blockchain_height = remote_blockchain_height;
            context.last_response_height = Some(last_response_height);
        });

        if last_response_height > remote_blockchain_height {
            let reason = "peer sent `ResponseChainEntry` with invalid height information.";
            self.misbehaving(peer_index, reason);
            return;
        }
    }

    fn on_response_get_objects(&self, _peer_index: PeerIndex, _arg: &ResponseGetObjects) {
    }

    fn on_support_flags(&self, peer_index: PeerIndex, arg: u32) {
        self.context_write(&peer_index, move |context| {
            context.support_flags = Some(arg);
        });
    }
}

#[derive(PartialEq, Eq)]
pub enum SyncState {
    Synchronizing,
    Synchronized,
}

pub struct Context {
    pub remote_blockchain_height: u64,
    pub last_response_height: Option<u64>,
    pub support_flags: Option<u32>,
}
