mod new_block;
mod new_fluffy_block;
mod new_transactions;

pub use self::new_block::{NewBlock, NewBlockRequest};
pub use self::new_fluffy_block::{NewFluffyBlock, NewFluffyBlockRequest};
pub use self::new_transactions::{NewTransactions, NewTransactionsRequest};
