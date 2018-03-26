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

use p2p::types::cn;

use types::{PeersRef, PeerIndex};

pub trait TaskExecutor: Send + Sync + 'static {
    fn execute(&self, task: Task);
}

pub enum Task {
    RequestChain(PeerIndex, cn::cmd::RequestChain),
}

pub struct LocalSynchronizationTaskExecutor {
    peers: PeersRef,
}

impl LocalSynchronizationTaskExecutor {
    pub fn new(peers: PeersRef) -> LocalSynchronizationTaskExecutor {
        LocalSynchronizationTaskExecutor { peers }
    }

    fn execute_requestchain(&self, peer_index: PeerIndex, request: cn::cmd::RequestChain) {
        debug!("Executing RequestChain request for peer #{} - {:?}",
               peer_index,
               request);

        self.peers
            .connection(peer_index)
            .map(|connection| { connection.notify_request_chain(&request); });
    }
}

impl TaskExecutor for LocalSynchronizationTaskExecutor {
    fn execute(&self, task: Task) {
        match task {
            Task::RequestChain(peer_index, req) => self.execute_requestchain(peer_index, req),
        }
    }
}
