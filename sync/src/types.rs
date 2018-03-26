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
