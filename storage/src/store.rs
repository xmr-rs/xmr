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

use std::sync::Arc;
use std::collections::LinkedList;

use primitives::H256;
use best_block::BestBlock;
use block_chain::BlockChain;
use block_provider::{BlockProvider, IndexedBlockProvider};

pub trait CanonStore: Store {
    fn as_store(&self) -> &Store;
}

/// Blockchain storage interface.
pub trait Store: AsSubstore {
    /// Get the best block.
    fn best_block(&self) -> BestBlock;

    /// Get the blockchain height.
    fn height(&self) -> u64;

    fn short_chain_history(&self) -> LinkedList<H256>;
}

/// Allows casting Arc<Store> to reference to any substore type
pub trait AsSubstore: BlockChain + IndexedBlockProvider {
    fn as_block_provider(&self) -> &BlockProvider;
}

impl<T> AsSubstore for T
    where T: BlockChain + IndexedBlockProvider
{
    fn as_block_provider(&self) -> &BlockProvider {
        &*self
    }
}

pub type SharedStore = Arc<CanonStore + Send + Sync>;
