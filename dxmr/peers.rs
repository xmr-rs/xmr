use std::net::{SocketAddr, ToSocketAddrs};
use network::Network;

pub fn default_peers(network: Network) -> Vec<SocketAddr> {
    let addrs: &[&'static str] = match network {
        Network::Testnet => {
            &["212.83.175.67:28080",
              "5.9.100.248:28080",
              "163.172.182.165:28080",
              "195.154.123.123:28080",
              "212.83.172.165:28080"]
        }
        Network::Mainnet => {
            &["107.152.130.98:18080",
              "212.83.175.67:18080",
              "5.9.100.248:18080",
              "163.172.182.165:18080",
              "161.67.132.39:18080",
              "198.74.231.92:18080",
              "195.154.123.123:28080",
              "212.83.172.165:28080"]
        }
    };

    // TODO: use expect instead of unwrap.
    addrs
        .iter()
        .map(|addr| (*addr).to_socket_addrs().unwrap().next().unwrap())
        .collect()
}

// TODO: Add support for seed nodes
