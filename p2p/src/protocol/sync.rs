use net::PeerContext;

use types::cn::cmd::{NewBlock, NewBlockRequest};
use types::cn::cmd::{NewFluffyBlock, NewFluffyBlockRequest};
use types::cn::cmd::{NewTransactions, NewTransactionsRequest};
use types::cn::cmd::{RequestChain, RequestChainRequest};
use types::cn::cmd::{RequestGetObjects, RequestGetObjectsRequest};
use types::cn::cmd::{ResponseChainEntry, ResponseChainEntryRequest};
use types::cn::cmd::{ResponseGetObjects, ResponseGetObjectsRequest};

pub trait OutboundSyncConnection {
    fn notify_new_block(&self, req: &NewBlockRequest);
    fn notify_new_fluffy_block(&self, req: &NewFluffyBlockRequest);
    fn notify_new_transactions(&self, req: &NewTransactionsRequest);
    fn notify_request_chain(&self, req: &RequestChainRequest);
    fn notify_request_get_objects(&self, req: &RequestGetObjectsRequest);
    fn notify_response_chain_entry(&self, req: &ResponseChainEntryRequest);
    fn notify_response_get_objects(&self, req: &ResponseGetObjectsRequest);
}

pub struct OutboundSync {
    context: PeerContext,
}

impl OutboundSyncConnection for OutboundSync {
    fn notify_new_block(&self, req: &NewBlockRequest) {
        trace!("outbound sync - {:?}", req);
        self.context.notify::<NewBlock>(req)
    }


    fn notify_new_fluffy_block(&self, req: &NewFluffyBlockRequest) {
        trace!("outbound sync - {:?}", req);
        self.context.notify::<NewFluffyBlock>(req)
    }

    fn notify_new_transactions(&self, req: &NewTransactionsRequest) {
        trace!("outbound sync - {:?}", req);
        self.context.notify::<NewTransactions>(req)
    }

    fn notify_request_chain(&self, req: &RequestChainRequest) {
        trace!("outbound sync - {:?}", req);
        self.context.notify::<RequestChain>(req)
    }

    fn notify_request_get_objects(&self, req: &RequestGetObjectsRequest) {
        trace!("outbound sync - {:?}", req);
        self.context.notify::<RequestGetObjects>(req)
    }

    fn notify_response_chain_entry(&self, req: &ResponseChainEntryRequest) {
        trace!("outbound sync - {:?}", req);
        self.context.notify::<ResponseChainEntry>(req)
    }

    fn notify_response_get_objects(&self, req: &ResponseGetObjectsRequest) {
        trace!("outbound sync - {:?}", req);
        self.context.notify::<ResponseGetObjects>(req)
    }
}
