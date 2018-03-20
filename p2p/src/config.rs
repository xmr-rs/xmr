use std::net::SocketAddr;
use network::Network;

use types::PeerId;

/// P2P configuration.
#[derive(Debug, Clone)]
pub struct Config {
    /// Number of threads.
    pub threads: usize,
    /// The network id.
    pub network: Network,
    /// Peers to connect.
    pub peers: Vec<SocketAddr>,
    /// Listening port.
    pub listen_port: Option<u16>,
    /// Hide my port.
    pub hide_my_port: bool,
    /// Maximum of outbound peers.
    pub out_peers: u32,
    /// Maximum of inbound peers.
    pub in_peers: u32,
    /// The peer ID.
    pub peer_id: PeerId,
}
