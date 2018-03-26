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

extern crate futures;
extern crate futures_cpupool;
extern crate tokio_core;
extern crate tokio_io;

extern crate bytes;
extern crate parking_lot;
extern crate rand;
extern crate uuid;

extern crate serde;
#[macro_use]
extern crate serde_derive;

extern crate failure;
#[macro_use]
extern crate log;

extern crate xmr_levin as levin;
extern crate xmr_network as network;
extern crate xmr_portable_storage as portable_storage;
extern crate xmr_portable_storage_utils as portable_storage_utils;
extern crate xmr_primitives as primitives;
extern crate xmr_storage as storage;

pub mod event_loop;
pub mod types;
pub mod p2p;
pub mod net;
pub mod protocol;
pub mod config;
pub mod utils;

pub use p2p::P2P;
pub use event_loop::{event_loop, forever};
pub use config::Config;
