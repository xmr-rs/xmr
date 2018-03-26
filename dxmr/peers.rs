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
