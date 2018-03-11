#[macro_use]
extern crate clap;
extern crate failure;
extern crate log;
extern crate env_logger;
extern crate app_dirs;
extern crate p2p;
extern crate db;
extern crate network;
extern crate chain;

mod config;
mod peers;
mod utils;

use failure::Error;
use app_dirs::AppInfo;

pub const APP_INFO: AppInfo = AppInfo { name: "dxmr", author: "Jean Pierre Dudey" };

fn main() {
    env_logger::init();

    let matches  = clap_app!(dxmr =>
        (version: "0.1.0")
        (author: "Jean Pierre Dudey <jeandudey@hotmail.com>")
        (about: "Monero client")
        (@arg threads: --threads +takes_value "Number of threads")
        (@arg testnet: --testnet "Use the test network")
        (@arg connect: --connect +takes_value "Connect only to the given peer")
        (@arg listenport: --listenport +takes_value )
        (@arg hidemyport: --hidemyport)
        (@arg outpeers: --outpeers +takes_value "Maximum of outbound peers")
        (@arg inpeers: --inpeers +takes_value "Maximum of outbound peers")
    ).get_matches();
    
    // TODO: no unwrap
    let cfg = config::parse(&matches).unwrap();

    if let Err(e) = start(cfg) {
        println!("{}", e);
        return;
    }
}

fn start(cfg: config::Config) -> Result<(), Error> {
    utils::init_db(&cfg);

    let mut el = p2p::event_loop();

    let config = p2p::Config {
        threads: cfg.threads,
        network: cfg.network,
        peers: cfg.peers,
        listen_port: cfg.listen_port,
        hide_my_port: cfg.hide_my_port,
        out_peers: cfg.out_peers,
        in_peers: cfg.in_peers,
    };

    let p2p = p2p::P2P::new(config, el.handle());

    p2p.run(cfg.db.clone())
        .expect("couldn't start p2p");

    el.run(p2p::forever())
        .expect("couldn't run event loop");

    Ok(())
}
