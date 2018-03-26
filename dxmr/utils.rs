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

use app_dirs::{AppDataType, app_dir};

use chain::IndexedBlock;
use db::BlockChainDatabase;
use storage::SharedStore;

use config::Config;

pub fn open_db() -> SharedStore {
    use APP_INFO;

    let path =
        app_dir(AppDataType::UserData, &APP_INFO, "db").expect("couldn't get user data location");

    let db = BlockChainDatabase::open(path).expect("couldn't open blockchain database");

    Arc::new(db)
}

pub fn init_db(cfg: &Config) {
    let genesis_block: IndexedBlock = cfg.network.genesis_block().into();

    match cfg.db.block_id(0) {
        Some(ref id) => {
            if id != genesis_block.id() {
                panic!("trying to open database with incompatible genesis block")
            }
        }
        None => {
            let id = genesis_block.id().clone();
            cfg.db
                .insert(genesis_block)
                .expect("couldn't insert genesis block");

            cfg.db
                .canonize(&id)
                .expect("couldn't canonize genesis block");
        }
    }
}
