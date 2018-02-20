use levin::COMMAND_BASE_ID;

use ser::SerializableUuid;

pub const P2P_COMMAND_BASE_ID: u32 = COMMAND_BASE_ID;

pub mod handshake;
pub mod request_support_flags;
pub mod timedsync;

pub mod peerlist;
pub use self::peerlist::PeerId;

mod ipv4_address;
pub use self::ipv4_address::Ipv4Address;

/// Basic information about a node.
#[derive(Debug, Default, Clone)]
pub struct BasicNodeData {
    /// The network UUID, should be the same for all peers.
    pub network_id: SerializableUuid,

    /// The peer's local time
    pub local_time: u64,

    /// The peer's listening port.
    pub my_port: u32,

    /// The peer's id.
    pub peer_id: PeerId,
}

serializable! {
    BasicNodeData {
        local_time,
        my_port,
        network_id,
        peer_id,
    }
}
