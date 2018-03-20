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

        let res = self.context.command_streams.read().get(&self.addr).cloned();
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
