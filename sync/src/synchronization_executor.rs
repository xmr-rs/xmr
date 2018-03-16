use p2p::types::{cn, PeerId};

use types::PeersRef;

pub trait TaskExecutor: Send + Sync + 'static {
    fn execute(&self, task: Task);
}

pub enum Task {
    RequestChain(PeerId, cn::cmd::RequestChainRequest),
}

pub struct LocalSynchronizationTaskExecutor {
    peers: PeersRef,
}

impl LocalSynchronizationTaskExecutor {
    pub fn new(peers: PeersRef) -> LocalSynchronizationTaskExecutor {
        LocalSynchronizationTaskExecutor { peers }
    }

    fn execute_requestchain(&self, peer_id: PeerId, request: cn::cmd::RequestChainRequest) {
        debug!("Executing RequestChain request - {:?} - {:?}",
               peer_id,
               request);

        self.peers
            .connection(peer_id)
            .map(|connection| { connection.notify_request_chain(&request); });
    }
}

impl TaskExecutor for LocalSynchronizationTaskExecutor {
    fn execute(&self, task: Task) {
        match task {
            Task::RequestChain(peer_id, req) => self.execute_requestchain(peer_id, req),
        }
    }
}
