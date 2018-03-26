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

use std::net::SocketAddr;
use network::Network;

use types::PeerId;

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
    pub listen_port: Option<u16>,
    /// Hide my port.
    pub hide_my_port: bool,
    /// Maximum of outbound peers.
    pub out_peers: u32,
    /// Maximum of inbound peers.
    pub in_peers: u32,
    /// The peer ID.
    pub peer_id: PeerId,
}
