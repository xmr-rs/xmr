use app_dirs::{AppDataType, app_dir};
use std::sync::Arc;
use db::{SharedStore, BlockChainDatabase};
use chain::IndexedBlock;
use config::Config;
use APP_INFO;

pub fn open_db() -> SharedStore {
    let path = app_dir(AppDataType::UserData, &APP_INFO, "db")
        .expect("couldn't get user data location");

    let db = BlockChainDatabase::open(path)
        .expect("couldn't open blockchain database");

    Arc::new(db)
}

pub fn init_db(cfg: &Config) {
    let genesis_block: IndexedBlock = cfg.network.genesis_block().into();

    match cfg.db.block_id(0) {
        Some(ref id) if id != genesis_block.id() =>
            panic!("trying to open database with incompatible genesis block"),
        Some(_) => {},
        None => {
            let id = genesis_block.id().clone();
            cfg.db.insert(genesis_block)
                .expect("couldn't insert genesis block");

            cfg.db.canonize(&id)
                .expect("couldn't canonize genesis block");
        }
    }
}
