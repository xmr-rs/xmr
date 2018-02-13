use std::net::SocketAddr;
use uuid::Uuid;

/// P2P configuration.
#[derive(Debug, Clone)]
pub struct Config {
    /// Number of threads.
    pub threads: usize,
    /// The network id.
    pub network_id: Uuid,
    /// Peers to connect.
    pub peers: Vec<SocketAddr>,
    /// Listening port.
    pub listen_port: u32,
    /// Hide my port.
    pub hide_my_port: bool,
}
