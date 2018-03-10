use portable_storage_utils::BytesUuid;
use types::PeerId;

/// Basic information about a node.
#[derive(Debug, Default, Clone, Deserialize, Serialize)]
pub struct BasicNodeData {
    /// The peer's local time
    pub local_time: u64,

    /// The peer's listening port.
    pub my_port: u32,

    /// The network UUID, should be the same for all peers.
    pub network_id: BytesUuid,

    /// The peer's id.
    pub peer_id: PeerId,
}
