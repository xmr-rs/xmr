use std::sync::Arc;

use portable_storage::to_section;

use net::PeerContext;

use types::PeerId;
use types::cn::CoreSyncData;
use types::cn::cmd::{NewBlock, NewFluffyBlock, NewTransactions, RequestChain,
                     RequestFluffyMissingTx, RequestGetObjects, ResponseChainEntry,
                     ResponseGetObjects};

pub trait LocalSyncNode: Send + Sync + 'static {
    fn new_sync_connection(&self,
                           peer_id: PeerId,
                           sync_data: &CoreSyncData,
                           connection: OutboundSyncConnectionRef)
                           -> InboundSyncConnectionRef;
}

pub type LocalSyncNodeRef = Box<LocalSyncNode>;

pub trait OutboundSyncConnection: Send + Sync {
    fn notify_new_block(&self, arg: &NewBlock);
    fn notify_new_fluffy_block(&self, arg: &NewFluffyBlock);
    fn notify_new_transactions(&self, arg: &NewTransactions);
    fn notify_request_chain(&self, arg: &RequestChain);
    fn notify_request_fluffy_missing_tx(&self, arg: &RequestFluffyMissingTx);
    fn notify_request_get_objects(&self, arg: &RequestGetObjects);
    fn notify_response_chain_entry(&self, arg: &ResponseChainEntry);
    fn notify_response_get_objects(&self, arg: &ResponseGetObjects);
    fn close(&self);
}

pub type OutboundSyncConnectionRef = Arc<OutboundSyncConnection>;

pub struct OutboundSync {
    context: PeerContext,
}

impl OutboundSync {
    pub fn new(context: PeerContext) -> OutboundSync {
        OutboundSync { context }
    }
}

impl OutboundSyncConnection for OutboundSync {
    fn notify_new_block(&self, arg: &NewBlock) {
        self.context.notify::<NewBlock>(to_section(arg).unwrap())
    }


    fn notify_new_fluffy_block(&self, arg: &NewFluffyBlock) {
        self.context
            .notify::<NewFluffyBlock>(to_section(arg).unwrap())
    }

    fn notify_new_transactions(&self, arg: &NewTransactions) {
        self.context
            .notify::<NewTransactions>(to_section(arg).unwrap())
    }

    fn notify_request_chain(&self, arg: &RequestChain) {
        self.context
            .notify::<RequestChain>(to_section(arg).unwrap())
    }

    fn notify_request_fluffy_missing_tx(&self, arg: &RequestFluffyMissingTx) {
        self.context
            .notify::<RequestFluffyMissingTx>(to_section(arg).unwrap())
    }

    fn notify_request_get_objects(&self, arg: &RequestGetObjects) {
        self.context
            .notify::<RequestGetObjects>(to_section(arg).unwrap())
    }

    fn notify_response_chain_entry(&self, arg: &ResponseChainEntry) {
        self.context
            .notify::<ResponseChainEntry>(to_section(arg).unwrap())
    }

    fn notify_response_get_objects(&self, arg: &ResponseGetObjects) {
        self.context
            .notify::<ResponseGetObjects>(to_section(arg).unwrap())
    }

    fn close(&self) {
        self.context.close();
    }
}

pub trait InboundSyncConnection: Send + Sync + 'static {
    fn on_new_block(&self, arg: &NewBlock);
    fn on_new_fluffy_block(&self, arg: &NewFluffyBlock);
    fn on_new_transactions(&self, arg: &NewTransactions);
    fn on_request_chain(&self, arg: &RequestChain);
    fn on_request_fluffy_missing_tx(&self, arg: &RequestFluffyMissingTx);
    fn on_request_get_objects(&self, arg: &RequestGetObjects);
    fn on_response_chain_entry(&self, arg: &ResponseChainEntry);
    fn on_response_get_objects(&self, arg: &ResponseGetObjects);

    /// # Notes:
    ///
    /// This isn't a notification, it is called when a `RequestSupportFlags`
    /// response is received.
    fn on_support_flags(&self, arg: u32);
}

pub type InboundSyncConnectionRef = Arc<InboundSyncConnection>;
