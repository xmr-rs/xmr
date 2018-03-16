use std::sync::Arc;
use types::PeerlistEntry;
use p2p::Context;

use futures::Future;
use levin::bucket::Bucket;
use levin::Notify;

pub struct PeerContext {
    context: Arc<Context>,
    info: PeerlistEntry,
}

impl PeerContext {
    pub fn new(context: Arc<Context>, info: PeerlistEntry) -> PeerContext {
        PeerContext { context, info }
    }

    pub fn notify<N>(&self, req: &N::Request)
        where N: Notify
    {
        trace!("peer context notify - {:?}", self.info);
        let context = self.context.clone();

        let channel = if let Some(c) = context.connections.channel(&self.info.id) {
            c
        } else {
            warn!("couldn't get peer channel, closed connection? (peer id: {:?})",
                  self.info.id);
            return;
        };

        let future = Box::new(Bucket::notify_future::<_, N>(channel, req)
                                  .map_err(|_| ())
                                  .map(|_| ()));

        context
            .remote
            .clone()
            .spawn(move |_| context.pool.clone().spawn(future))
    }

    pub fn close(&self) {
        Context::close(self.context.clone(), self.info.id);
    }
}
