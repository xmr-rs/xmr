use app_dirs::{AppDataType, app_dir};
use std::sync::Arc;
use db::{SharedStore, BlockChainDatabase};
use APP_INFO;

pub fn open_db() -> SharedStore {
    let path = app_dir(AppDataType::UserData, &APP_INFO, "db")
        .expect("couldn't get user data location");

    let db = BlockChainDatabase::open(path)
        .expect("couldn't open blockchain database");

    Arc::new(db)
}
