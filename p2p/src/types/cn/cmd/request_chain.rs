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

use levin::Command;
use portable_storage_utils::stl::StlLinkedList;
use primitives::H256;

use types::cn::CN_COMMAND_BASE_ID;

#[derive(Debug, Deserialize, Serialize)]
pub struct RequestChain {
    pub block_ids: StlLinkedList<H256>,
}

impl Command for RequestChain {
    const ID: u32 = CN_COMMAND_BASE_ID + 6;
}
