#[macro_use]
extern crate clap;
#[macro_use]
extern crate failure;
extern crate common_failures;
#[macro_use]
extern crate log;
extern crate env_logger;
extern crate p2p;
extern crate db;

use std::sync::Arc;

mod network;
mod config;
mod peers;

fn main() {
    env_logger::init();

    let matches  = clap_app!(dxmr =>
        (version: "0.1.0")
        (author: "Jean Pierre Dudey <jeandudey@hotmail.com>")
        (about: "Monero client")
        (@arg testnet: --testnet "Use the test network")
    ).get_matches();
    
    // TODO: no unwrap
    let cfg = config::parse(&matches).unwrap();

    start(cfg)
}

fn start(cfg: config::Config) {
    let mut el = p2p::event_loop();

    let config = p2p::Config {
        // TODO: Add option.
        threads: 2,
    };

    let blockchain = Arc::new(db::BlockChainDatabase);

    let p2p = p2p::P2P::new(config, el.handle(), blockchain);

    p2p.run();
    el.run(p2p::forever());
}
