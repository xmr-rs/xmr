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

extern crate bytes;
extern crate parking_lot;
extern crate sanakirja;
extern crate rand;

extern crate failure;
#[macro_use]
extern crate failure_derive;

extern crate xmr_chain as chain;
extern crate xmr_format as format;
extern crate xmr_primitives as primitives;
extern crate xmr_storage as storage;

pub mod kv;

mod block_chain_db;
mod error;

pub use self::block_chain_db::BlockChainDatabase;
pub use self::error::Error;
