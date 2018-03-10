//! CryptoNote types.

pub const CN_COMMAND_BASE_ID: u32 = 2000;

pub mod cmd;

mod block_complete_entry;
mod core_sync_data;

pub use self::block_complete_entry::BlockCompleteEntry;
pub use self::core_sync_data::CoreSyncData;
