use types::cn::cmd::{NotifyNewBlock, NotifyNewBlockRequest};
use net::PeerContext;

pub trait OutboundSyncConnection {
    fn notify_new_block(&self, req: &NotifyNewBlockRequest);
}

pub struct OutboundSync {
    context: PeerContext,
}

impl OutboundSyncConnection for OutboundSync {
    fn notify_new_block(&self, req: &NotifyNewBlockRequest) {
        trace!("outbound sync - notify new block - {:?}", req);
        self.context.notify::<NotifyNewBlock>(req)
    } 
}
