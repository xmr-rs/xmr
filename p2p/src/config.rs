use std::net::SocketAddr;
use network::Network;

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
    pub listen_port: Option<u32>,
    /// Hide my port.
    pub hide_my_port: bool,
}

pub const P2P_SUPPORT_FLAG_FLUFFY_BLOCKS: u32 = 0x01;
pub const P2P_SUPPORT_FLAGS: u32 = P2P_SUPPORT_FLAG_FLUFFY_BLOCKS;
