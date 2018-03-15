use std::sync::Arc;

use parking_lot::Mutex;

use storage::SharedStore;

use local_node::LocalNode;
use synchronization_client::SynchronizationClient;
use synchronization_client_core::SynchronizationClientCore;
use synchronization_executor::LocalSynchronizationTaskExecutor;
use synchronization_peers::Peers;

pub type LocalNodeRef = Arc<LocalNode>;
pub type ClientRef = Arc<SynchronizationClient>;
pub type ClientCoreRef = Arc<Mutex<SynchronizationClientCore>>;
pub type PeersRef = Arc<Peers>;
pub type ExecutorRef = Arc<LocalSynchronizationTaskExecutor>;
pub type StorageRef = SharedStore;
