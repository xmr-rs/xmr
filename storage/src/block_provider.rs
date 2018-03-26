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

use primitives::H256;
use chain::IndexedBlock;
use block_ref::BlockRef;

pub trait BlockProvider {
    fn block_id(&self, height: u64) -> Option<H256>;
}

pub trait IndexedBlockProvider: BlockProvider {
    fn indexed_block(&self, block_ref: BlockRef) -> Option<IndexedBlock>;
}
