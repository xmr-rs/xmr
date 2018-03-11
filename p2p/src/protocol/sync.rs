use types::cn::cmd::{NotifyNewBlock, NotifyNewBlockRequest, NotifyNewFluffyBlock, NotifyNewFluffyBlockRequest};
use net::PeerContext;

pub trait OutboundSyncConnection {
    fn notify_new_block(&self, req: &NotifyNewBlockRequest);
    fn notify_new_fluffy_block(&self, req: &NotifyNewFluffyBlockRequest);
}

pub struct OutboundSync {
    context: PeerContext,
}

impl OutboundSyncConnection for OutboundSync {
    fn notify_new_block(&self, req: &NotifyNewBlockRequest) {
        trace!("outbound sync - notify new block - {:?}", req);
        self.context.notify::<NotifyNewBlock>(req)
    }


    fn notify_new_fluffy_block(&self, req: &NotifyNewFluffyBlockRequest) {
        trace!("outbound sync - notify new fluffy block - {:?}", req);
        self.context.notify::<NotifyNewFluffyBlock>(req)
    }
}
