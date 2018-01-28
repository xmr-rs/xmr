use std::marker::PhantomData;

use portable_storage::ser::Serializable;

use protocol::P2P_COMMAND_BASE_ID;
use protocol::peerlist::PeerlistEntry;
use cryptonote::CoreSyncData;
use levin::Command;

use ser::DefaultSerializableLinkedList;

/// Handshake with `cryptonote::CoreSyncData` as the payload data.
pub type CryptoNoteTimedSync = TimedSync<CoreSyncData>;

#[derive(Debug)]
pub struct TimedSync<P: Serializable>(PhantomData<P>);

impl<P: Serializable> Command for TimedSync<P> {
    type Request = TimedSyncRequest<P>;
    type Response = TimedSyncResponse<P>;
    
    const ID: u32 = P2P_COMMAND_BASE_ID + 2;
}

#[derive(Debug, Default, Clone)]
pub struct TimedSyncRequest<P: Serializable> {
    pub payload_data: P,
}

serializable! {
    TimedSyncRequest<P> where (P: Serializable) {
        payload_data,
    }
}

#[derive(Debug, Default, Clone)]
pub struct TimedSyncResponse<P: Serializable> {
    pub local_time: u64,
    pub payload_data: P,
    pub local_peerlist_new: DefaultSerializableLinkedList<PeerlistEntry>,
}

serializable! {
    TimedSyncResponse<P> where (P: Serializable) {
        local_time,
        payload_data,
        local_peerlist_new,
    }
}
