use p2p::types::PeerId;

use synchronization_chain::Chain;
use synchronization_executor::{Task, TaskExecutor};
use types::{ExecutorRef, StorageRef};

pub trait ClientCore: Send + Sync + 'static  {
    fn on_connect(&self, peer_id: PeerId);
}

pub struct SynchronizationClientCore {
    executor: ExecutorRef,
    chain: Chain,
}

impl SynchronizationClientCore {
    pub fn new(executor: ExecutorRef, storage: StorageRef) -> SynchronizationClientCore {
        SynchronizationClientCore {
            executor,
            chain: Chain::new(storage),
        }
    }
}

impl ClientCore for SynchronizationClientCore {
    fn on_connect(&self, peer_id: PeerId) {
        use p2p::types::cn::cmd::RequestChainRequest;

        info!("Synchronizing with peer \"{}\"", peer_id);

        let block_ids = self.chain.storage().short_chain_history();
        let request = RequestChainRequest { block_ids: block_ids.into() };
        self.executor.execute(Task::RequestChain(peer_id, request))
    }
}
