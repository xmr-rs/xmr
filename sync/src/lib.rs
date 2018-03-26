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

#[macro_use]
extern crate log;
extern crate parking_lot;

extern crate xmr_db as db;
extern crate xmr_network as network;
extern crate xmr_p2p as p2p;
extern crate xmr_primitives as primitives;
extern crate xmr_storage as storage;

mod connection_factory;
mod inbound_connection;
mod local_node;
mod synchronization_chain;
mod synchronization_client;
mod synchronization_client_core;
mod synchronization_executor;
mod synchronization_peers;
mod types;

pub use connection_factory::ConnectionFactory;
pub use inbound_connection::InboundConnection;
pub use local_node::LocalNode;
pub use synchronization_chain::Chain;
pub use synchronization_client::{Client, SynchronizationClient};
pub use synchronization_client_core::{ClientCore, SynchronizationClientCore};
pub use synchronization_executor::{TaskExecutor, LocalSynchronizationTaskExecutor};
pub use synchronization_peers::{Peers, Peer, PeersImpl};
pub use types::{LocalNodeRef, ClientRef, ClientCoreRef, ExecutorRef, PeersRef, StorageRef, PeerIndex};

pub fn create_local_node(storage: StorageRef, network: network::Network) -> LocalNodeRef {
    use std::sync::Arc;

    Arc::new(LocalNode::new(storage, network))
}

pub fn create_local_sync_node(local_node: LocalNodeRef) -> p2p::protocol::LocalSyncNodeRef {
    ConnectionFactory::new(local_node).boxed()
}
