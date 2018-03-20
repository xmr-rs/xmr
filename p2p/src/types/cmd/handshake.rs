use portable_storage_utils::stl::StlLinkedList;

use types::{P2P_COMMAND_BASE_ID, BasicNodeData, PeerlistEntry};
use types::cn::CoreSyncData;
use levin::Command;

/// The handshake command.
#[derive(Debug, Clone, Copy)]
pub struct Handshake;

impl Command for Handshake {
    const ID: u32 = P2P_COMMAND_BASE_ID + 1;
}

/// The handshake command request.
#[derive(Debug, Default, Clone, Deserialize, Serialize)]
pub struct HandshakeRequest {
    pub node_data: BasicNodeData,
    pub payload_data: CoreSyncData,
}

/// The handshake command response.
#[derive(Debug, Default, Clone, Deserialize, Serialize)]
pub struct HandshakeResponse {
    pub node_data: BasicNodeData,
    pub payload_data: CoreSyncData,
    pub local_peerlist: StlLinkedList<PeerlistEntry>,
}
