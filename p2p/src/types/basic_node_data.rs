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

use portable_storage_utils::BytesUuid;
use types::PeerId;

/// Basic information about a node.
#[derive(Debug, Default, Clone, Deserialize, Serialize)]
pub struct BasicNodeData {
    /// The peer's local time
    pub local_time: u64,

    /// The peer's listening port.
    pub my_port: u32,

    /// The network UUID, should be the same for all peers.
    pub network_id: BytesUuid,

    /// The peer's id.
    pub peer_id: PeerId,
}
