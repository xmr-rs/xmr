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
