#[macro_use]
extern crate log;
extern crate parking_lot;

extern crate xmr_p2p as p2p;
extern crate xmr_db as db;
extern crate xmr_storage as storage;

mod connection_factory;
mod local_node;
mod synchronization_chain;
mod synchronization_client;
mod synchronization_client_core;
mod synchronization_executor;
mod synchronization_peers;
mod types;

pub use connection_factory::ConnectionFactory;
pub use local_node::LocalNode;
pub use synchronization_chain::Chain;
pub use synchronization_client::{Client, SynchronizationClient};
pub use synchronization_client_core::{ClientCore, SynchronizationClientCore};
pub use synchronization_executor::{TaskExecutor, LocalSynchronizationTaskExecutor};
pub use synchronization_peers::{Peers, Peer, PeersImpl};
pub use types::{LocalNodeRef, ClientRef, ClientCoreRef, ExecutorRef, PeersRef, StorageRef};

pub fn create_local_node(storage: StorageRef) -> LocalNodeRef {
    use std::sync::Arc;

    Arc::new(LocalNode::new(storage))
}

pub fn create_local_sync_node(local_node: LocalNodeRef) -> p2p::protocol::LocalSyncNodeRef {
    ConnectionFactory::new(local_node).boxed()
}
