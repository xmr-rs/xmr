use levin::COMMAND_BASE_ID;

pub const P2P_COMMAND_BASE_ID: u32 = COMMAND_BASE_ID;

pub mod cmd;

mod basic_node_data;
mod ipv4_address;
mod peerid;
mod peerlist_entry;

pub use self::basic_node_data::BasicNodeData;
pub use self::ipv4_address::Ipv4Address;
pub use self::peerid::PeerId;
pub use self::peerlist_entry::PeerlistEntry;
