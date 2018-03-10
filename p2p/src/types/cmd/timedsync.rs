use portable_storage_utils::stl::StlLinkedList;

use types::{P2P_COMMAND_BASE_ID, PeerlistEntry};
use cryptonote::CoreSyncData;
use levin::Command;

#[derive(Debug)]
pub struct TimedSync;

impl Command for TimedSync {
    type Request = TimedSyncRequest;
    type Response = TimedSyncResponse;
    
    const ID: u32 = P2P_COMMAND_BASE_ID + 2;
}

#[derive(Debug, Default, Clone, Deserialize, Serialize)]
pub struct TimedSyncRequest {
    pub payload_data: CoreSyncData,
}

#[derive(Debug, Default, Clone, Deserialize, Serialize)]
pub struct TimedSyncResponse {
    pub local_time: u64,
    pub payload_data: CoreSyncData,
    pub local_peerlist: StlLinkedList<PeerlistEntry>,
}
