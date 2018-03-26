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
use std::net::SocketAddr;

use levin::Command;
use portable_storage::Section;

use p2p::Context;

pub struct PeerContext {
    context: Arc<Context>,
    addr: SocketAddr,
}

impl PeerContext {
    pub fn new(context: Arc<Context>, addr: SocketAddr) -> PeerContext {
        PeerContext { context, addr }
    }

    pub fn notify<C>(&self, request: Section)
        where C: Command
    {
        trace!("peer ({}) context notify - {:?} ", self.addr, request);

        let res = self.context
            .command_streams
            .read()
            .get(&self.addr)
            .cloned();
        if let Some(command_stream) = res {
            command_stream.notify::<C>(request)
        } else {
            warn!("couldn't get command stream, closed connection? address {}",
                  self.addr);
            return;
        };
    }

    pub fn close(&self) {
        Context::close(self.context.clone(), &self.addr);
    }
}
