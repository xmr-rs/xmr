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

extern crate xmr_chain as chain;
extern crate xmr_primitives as primitives;

mod best_block;
mod block_chain;
mod block_provider;
mod block_ref;
mod store;

pub use best_block::BestBlock;
pub use block_chain::BlockChain;
pub use block_provider::{BlockProvider, IndexedBlockProvider};
pub use block_ref::BlockRef;
pub use store::{AsSubstore, CanonStore, Store, SharedStore};
