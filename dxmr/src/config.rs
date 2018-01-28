use std::net::{SocketAddr, ToSocketAddrs};

use clap::ArgMatches;
use failure::Error;
use common_failures::io::{IoContextExt, Target, Operation};
use network::Network;
use peers::default_peers;

#[derive(Debug)]
pub struct Config {
    pub network: Network,
    pub peers: Vec<SocketAddr>,
}

pub fn parse(matches: &ArgMatches) -> Result<Config, Error> {
    let network = match matches.is_present("testnet") {
        true => Network::Testnet,
        false => Network::Mainnet,
    };

    let default_peers = default_peers(network);
    let mut peers: Vec<SocketAddr> = Vec::with_capacity(default_peers.len());
    for seed in default_peers {
        let s = seed.to_socket_addrs()
            .io_context(Operation::Other, Target::Other("address".to_string()));
        match s {
            Ok(s) => {
                for addr in s {
                    match addr { 
                        SocketAddr::V4(a) => peers.push(SocketAddr::V4(a)),
                        // TODO: Handle IPv6 Addresses
                        SocketAddr::V6(_) => {},
                    }
                }
            }
            Err(e) => warn!("{}", e),
        }
    }

    Ok(Config {
        network,
        peers,
    })
}
