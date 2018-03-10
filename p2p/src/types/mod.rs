use portable_storage_utils::BytesUuid;
use levin::COMMAND_BASE_ID;

pub const P2P_COMMAND_BASE_ID: u32 = COMMAND_BASE_ID;

pub mod handshake;
pub mod request_support_flags;
pub mod ping;
pub mod timedsync;

mod peerid;
mod peerlist_entry;
mod ipv4_address;

pub use self::peerid::PeerId;
pub use self::ipv4_address::Ipv4Address;
pub use self::peerlist_entry::PeerlistEntry;

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
