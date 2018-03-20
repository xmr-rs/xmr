use std::net::SocketAddr;

use clap::ArgMatches;
use failure::Error;
use network::Network;
use peers::default_peers;
use storage::SharedStore;
use utils;

pub struct Config {
    pub network: Network,
    pub peers: Vec<SocketAddr>,
    pub threads: usize,
    pub listen_port: Option<u16>,
    pub hide_my_port: bool,
    pub out_peers: u32,
    pub in_peers: u32,
    pub db: SharedStore,
}

pub fn parse(matches: &ArgMatches) -> Result<Config, Error> {
    let network = match matches.is_present("testnet") {
        true => Network::Testnet,
        false => Network::Mainnet,
    };

    let peers = match value_t!(matches.value_of("connect"), SocketAddr) {
        Ok(addr) => {
            let mut peers = Vec::with_capacity(1);
            peers.push(addr);
            peers
        }
        Err(_e) => default_peers(network),
    };

    let threads = value_t!(matches.value_of("threads"), usize).unwrap_or(1);

    let listen_port = value_t!(matches.value_of("listenport"), u16).ok();

    let hide_my_port = matches.is_present("hidemyport");

    let out_peers = value_t!(matches.value_of("outpeers"), u32).unwrap_or(10);
    let in_peers = value_t!(matches.value_of("inpeers"), u32).unwrap_or(10);

    let db = utils::open_db();

    Ok(Config {
           network,
           peers,
           threads,
           listen_port,
           hide_my_port,
           out_peers,
           in_peers,
           db,
       })
}
