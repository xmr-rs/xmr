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

use levin::COMMAND_BASE_ID;

pub const P2P_COMMAND_BASE_ID: u32 = COMMAND_BASE_ID;

pub mod cmd;
pub mod cn;

mod basic_node_data;
mod ipv4_address;
mod peerid;
mod peerlist_entry;

pub use self::basic_node_data::BasicNodeData;
pub use self::ipv4_address::Ipv4Address;
pub use self::peerid::PeerId;
pub use self::peerlist_entry::PeerlistEntry;
