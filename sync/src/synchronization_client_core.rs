use std::collections::HashMap;

use parking_lot::RwLock;

use network::Network;

use p2p::types::PeerId;
use p2p::types::cn::cmd::{NewBlock, NewFluffyBlock, NewTransactions, RequestChain,
                          RequestFluffyMissingTx, RequestGetObjects, ResponseChainEntry,
                          ResponseGetObjects};

use synchronization_chain::Chain;
use synchronization_executor::{Task, TaskExecutor};
use types::{ExecutorRef, PeersRef, StorageRef};

pub trait ClientCore: Send + Sync + 'static {
    fn on_connect(&self, peer_id: PeerId);
    fn on_new_block(&self, peer_id: PeerId, arg: &NewBlock);
    fn on_new_fluffy_block(&self, peer_id: PeerId, arg: &NewFluffyBlock);
    fn on_new_transactions(&self, peer_id: PeerId, arg: &NewTransactions);
    fn on_request_chain(&self, peer_id: PeerId, arg: &RequestChain);
    fn on_request_fluffy_missing_tx(&self, peer_id: PeerId, arg: &RequestFluffyMissingTx);
    fn on_request_get_objects(&self, peer_id: PeerId, arg: &RequestGetObjects);
    fn on_response_chain_entry(&self, peer_id: PeerId, arg: &ResponseChainEntry);
    fn on_response_get_objects(&self, peer_id: PeerId, arg: &ResponseGetObjects);
    fn on_support_flags(&self, peer_id: PeerId, arg: u32);
}

pub struct SynchronizationClientCore {
    executor: ExecutorRef,
    chain: Chain,
    network: Network,
    peers: PeersRef,
    contexes: RwLock<HashMap<PeerId, Context>>,
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

    fn context_write<F>(&self, peer_id: &PeerId, f: F)
        where F: FnOnce(&mut Context)
    {
        let mut contexes = self.contexes.write();
        let context = contexes.get_mut(peer_id).expect("context should exist");
        f(context)
    }

    fn misbehaving(&self, peer_id: PeerId, reason: &str) {
        self.peers.misbehaving(peer_id, reason);
        self.contexes.write().remove(&peer_id);
    }

    fn verify_sync_data(&self, peer_id: PeerId) -> Option<SyncState> {
        let sync_data = self.peers
            .last_sync_data(peer_id)
            .expect("peer id should be valid");

        info!("Verifying peer {} synchronization data", peer_id);
        debug!("Peer {} sync data: {:?}", peer_id, sync_data);

        if sync_data.current_height > 0 {
            let peer_top = sync_data.current_height - 1;
            let version = self.network
                .hard_forks()
                .ideal_version_for_height(peer_top);

            if version >= 6 && sync_data.top_version != version {
                warn!("Peer {} claims higher version than we think (v{} for block height {}) -
                      we may be forked from the network and a software upgrade is needed.",
                      peer_id,
                      sync_data.top_version,
                      peer_top);

                self.misbehaving(peer_id, "peer uses different version than us");

                return None;
            }
        }

        let context = Context {
            remote_blockchain_height: sync_data.current_height,
            last_response_height: None,
            support_flags: None,
        };

        self.contexes.write().insert(peer_id, context);

        if self.chain.have_block(sync_data.top_id) {
            let our_height = self.chain.height();
            if our_height == sync_data.current_height {
                info!("Peer {} is synchronized with us.", peer_id);
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
    fn on_connect(&self, peer_id: PeerId) {
        use p2p::types::cn::cmd::RequestChain;

        info!("Synchronizing with peer \"{}\"", peer_id);

        let state = self.verify_sync_data(peer_id);
        match state {
            Some(SyncState::Synchronizing) => {
                let block_ids = self.chain.storage().short_chain_history();
                let request = RequestChain { block_ids: block_ids.into() };
                self.executor
                    .execute(Task::RequestChain(peer_id, request));
            }
            Some(SyncState::Synchronized) => {}
            None => { /* not valid sync info */ }
        }
    }

    fn on_new_block(&self, _peer_id: PeerId, _arg: &NewBlock) {
    }

    fn on_new_fluffy_block(&self, _peer_id: PeerId, _arg: &NewFluffyBlock) {
    }

    fn on_new_transactions(&self, _peer_id: PeerId, _arg: &NewTransactions) {
    }

    fn on_request_chain(&self, _peer_id: PeerId, _arg: &RequestChain) {
    }

    fn on_request_fluffy_missing_tx(&self, _peer_id: PeerId, _arg: &RequestFluffyMissingTx) {
    }

    fn on_request_get_objects(&self, _peer_id: PeerId, _arg: &RequestGetObjects) {
    }

    fn on_response_chain_entry(&self, peer_id: PeerId, arg: &ResponseChainEntry) {
        if arg.block_ids.len() == 0 {
            self.misbehaving(peer_id, "peer sent empty `block_ids` field");
            return;
        }

        if arg.total_height < arg.block_ids.len() as u64 ||
           arg.start_height > arg.total_height - arg.block_ids.len() as u64 {
            self.misbehaving(peer_id, "peer sent invalid start/nblocks/height.");
            return;
        }

        let remote_blockchain_height = arg.total_height;
        let last_response_height = arg.start_height + arg.block_ids.len() as u64 - 1;

        self.context_write(&peer_id, move |context| {
            context.remote_blockchain_height = remote_blockchain_height;
            context.last_response_height = Some(last_response_height);
        });

        if last_response_height > remote_blockchain_height {
            let reason = "peer sent `ResponseChainEntry` with invalid height information.";
            self.misbehaving(peer_id, reason);
            return;
        }
    }

    fn on_response_get_objects(&self, _peer_id: PeerId, _arg: &ResponseGetObjects) {
    }

    fn on_support_flags(&self, peer_id: PeerId, arg: u32) {
        self.context_write(&peer_id, move |context| {
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
