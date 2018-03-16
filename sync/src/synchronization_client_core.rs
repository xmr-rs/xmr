use network::Network;

use p2p::types::PeerId;

use synchronization_chain::Chain;
use synchronization_executor::{Task, TaskExecutor};
use types::{ExecutorRef, PeersRef, StorageRef};

pub trait ClientCore: Send + Sync + 'static {
    fn on_connect(&self, peer_id: PeerId);
}

pub struct SynchronizationClientCore {
    executor: ExecutorRef,
    chain: Chain,
    network: Network,
    peers: PeersRef,
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
        }
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

                self.peers
                    .misbehaving(peer_id, "peer uses different version than us");

                return None;
            }
        }

        let our_height = self.chain.height();

        if self.chain.have_block(sync_data.top_id) {
            if our_height == sync_data.current_height {
                info!("Peer {} is synchronized with us.", peer_id);
                return Some(SyncState::Synchronized);
            }
        }

        return Some(SyncState::Synchronizing);
    }
}

impl ClientCore for SynchronizationClientCore {
    fn on_connect(&self, peer_id: PeerId) {
        use p2p::types::cn::cmd::RequestChainRequest;

        info!("Synchronizing with peer \"{}\"", peer_id);

        let state = self.verify_sync_data(peer_id);
        match state {
            Some(SyncState::Synchronizing) => {
                let block_ids = self.chain.storage().short_chain_history();
                let request = RequestChainRequest { block_ids: block_ids.into() };
                self.executor
                    .execute(Task::RequestChain(peer_id, request));
            }
            _ => {}
        }
    }
}

#[derive(PartialEq, Eq)]
pub enum SyncState {
    Synchronizing,
    Synchronized,
}
