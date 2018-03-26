// Xmr, Monero node.
// Copyright (C) 2018  Jean Pierre Dudey
//
// This program is free software: you can redistribute it and/or modify
// it under the terms of the GNU General Public License as published by
// the Free Software Foundation, either version 3 of the License, or
// (at your option) any later version.
//
// This program is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
// GNU General Public License for more details.
//
// You should have received a copy of the GNU General Public License
// along with this program.  If not, see <http://www.gnu.org/licenses/>.

extern crate app_dirs;
#[macro_use]
extern crate clap;

extern crate env_logger;
extern crate log;

extern crate failure;

extern crate rand;

extern crate xmr_chain as chain;
extern crate xmr_db as db;
extern crate xmr_network as network;
extern crate xmr_p2p as p2p;
extern crate xmr_storage as storage;
extern crate xmr_sync as sync;

mod config;
mod peers;
mod utils;

use failure::Error;
use app_dirs::AppInfo;

pub const APP_INFO: AppInfo = AppInfo {
    name: "dxmr",
    author: "Jean Pierre Dudey",
};

fn main() {
    env_logger::init();

    let matches = clap_app!(dxmr =>
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
    )
            .get_matches();

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

    let local_node = sync::create_local_node(cfg.db.clone(), cfg.network);
    let local_sync_node = sync::create_local_sync_node(local_node.clone());

    let mut rng = rand::OsRng::new().expect("couldn't open OS random");

    let config = p2p::Config {
        threads: cfg.threads,
        network: cfg.network,
        peers: cfg.peers,
        listen_port: cfg.listen_port,
        hide_my_port: cfg.hide_my_port,
        out_peers: cfg.out_peers,
        in_peers: cfg.in_peers,
        peer_id: p2p::types::PeerId::random(&mut rng),
    };

    let p2p = p2p::P2P::new(config, el.handle(), cfg.db.clone(), local_sync_node);

    p2p.run().expect("couldn't start p2p");

    el.run(p2p::forever()).expect("couldn't run event loop");

    Ok(())
}
