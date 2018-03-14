#[macro_use]
extern crate log;
extern crate parking_lot;

extern crate xmr_p2p as p2p;

mod connection_factory;
mod local_node;
mod synchronization_peers;

pub use connection_factory::ConnectionFactory;
pub use local_node::LocalNode;
pub use synchronization_peers::{Peers, PeersRef, Peer, PeersImpl};

pub fn create_local_node() -> LocalNode {
    LocalNode::new()
}

pub fn create_local_sync_node(peers: PeersRef) -> p2p::protocol::LocalSyncNodeRef {
    ConnectionFactory::new(peers).boxed()
}
