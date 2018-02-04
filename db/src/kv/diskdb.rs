use std::path::Path;
use std::cmp::{max, Ordering};
use std::slice::{from_raw_parts_mut, from_raw_parts};
use std::iter::{Empty, empty};
use std::borrow::Cow;

use rand::OsRng;
use sanakirja::{Env, Error, MutTxn, Commit};
use sanakirja::value::UnsafeValue;

use kv::{KeyValueDatabase, KeyState, Key, Value as TxValue, Transaction};
use kv::transaction::RawOperation;

/// A database stored in disk.
pub struct DiskDb {
    /// Sanakirja environment.
    env: Env,
}

impl DiskDb {
    /// Open a database.
    pub fn open<P>(path: P) -> Result<DiskDb, Error>
        where P: AsRef<Path>,
    {
        let size = Self::db_size(path.as_ref());
        let env = Env::new::<P>(path, size)?;
        
        Ok(DiskDb {
            env: env,
        })
    }

    /// Query the database file size.
    fn db_size<P>(path: P) -> u64 where P: AsRef<Path> {
        // XXX: Is this the best default?
        const MIN_DB_SIZE: u64 = 4096;

        Env::file_size(path.as_ref())
            .map(|size| max(size, MIN_DB_SIZE))
            .unwrap_or(MIN_DB_SIZE)
    }
}

impl KeyValueDatabase for DiskDb {
    // TODO: Unwraps to errors.
	fn write(&self, tx: Transaction) -> Result<(), String> {
        let mut txn = self.env.mut_txn_begin().unwrap();
        let mut db = txn.create_db::<UnsafeValue, UnsafeValue>().unwrap();
        let mut prng = OsRng::new().unwrap();

        for op in tx.operations.iter() {
            let op = op.into();
            match op {
                RawOperation::Insert(ref kv) => {
                    let k = UnsafeValue::from_slice(kv.key.as_ref());
                    let v = UnsafeValue::from_slice(kv.value.as_ref());
                    txn.put::<_, _, UnsafeValue>(&mut prng, &mut db, k, v).unwrap();
                },
                RawOperation::Delete(ref k) => {
                    let k = UnsafeValue::from_slice(k.key.as_ref());
                    txn.del::<_, _, UnsafeValue>(&mut prng, &mut db, k, None).unwrap();
                },
            }
        }

        txn.commit().unwrap();

        Ok(())
    }

	fn get(&self, key: &Key) -> Result<KeyState<TxValue>, String> { Err("".into()) }
}
