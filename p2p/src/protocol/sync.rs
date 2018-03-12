use net::PeerContext;

use types::cn::cmd::{NewBlock, NewBlockRequest};
use types::cn::cmd::{NewFluffyBlock, NewFluffyBlockRequest};
use types::cn::cmd::{NewTransactions, NewTransactionsRequest};

pub trait OutboundSyncConnection {
    fn notify_new_block(&self, req: &NewBlockRequest);
    fn notify_new_fluffy_block(&self, req: &NewFluffyBlockRequest);
    fn notify_new_transactions(&self, req: &NewTransactionsRequest);
}

pub struct OutboundSync {
    context: PeerContext,
}

impl OutboundSyncConnection for OutboundSync {
    fn notify_new_block(&self, req: &NewBlockRequest) {
        trace!("outbound sync - notify new block - {:?}", req);
        self.context.notify::<NewBlock>(req)
    }


    fn notify_new_fluffy_block(&self, req: &NewFluffyBlockRequest) {
        trace!("outbound sync - notify new fluffy block - {:?}", req);
        self.context.notify::<NewFluffyBlock>(req)
    }

    fn notify_new_transactions(&self, req: &NewTransactionsRequest) {
        trace!("outbound sync - notify new fluffy block - {:?}", req);
        self.context.notify::<NewTransactions>(req)
    }
}
