use std::path::Path;
use std::cmp::max;

use rand::OsRng;
use sanakirja::{Env, Error, MutTxn, Commit, Db, Transaction as SanakirjaTransaction};
use sanakirja::value::UnsafeValue;

use kv::{KeyValueDatabase, KeyState, Key, Value, Transaction};
use kv::transaction::{RawOperation, RawKey};

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
        let mut prng = OsRng::new().unwrap();

        // XXX: probably not the best performant kv db out there, but... who cares?
        for op in tx.operations.iter() {
            let op = op.into();
            match op {
                RawOperation::Insert(ref kv) => {
                    let mut db = open_db(&mut txn, kv.location);
                    let k = UnsafeValue::from_slice(kv.key.as_ref());
                    let v = UnsafeValue::from_slice(kv.value.as_ref());
                    txn.put::<_, _, UnsafeValue>(&mut prng, &mut db, k, v).unwrap();
                    txn.set_root(kv.location, db);
                },
                RawOperation::Delete(ref k) => {
                    let mut db = open_db(&mut txn, k.location);
                    let key = UnsafeValue::from_slice(k.key.as_ref());
                    txn.del::<_, _, UnsafeValue>(&mut prng, &mut db, key, None).unwrap();
                    txn.set_root(k.location, db);
                },
            }
        }

        txn.commit().unwrap();

        Ok(())
    }

	fn get(&self, key: &Key) -> Result<KeyState<Value>, String> {
        let raw_key: RawKey = key.into();
        let mut txn = self.env.txn_begin().unwrap();
        let db = match txn.root(raw_key.location) {
            Some(db) => db,
            None => {
                return Err("Db doesn't exists".into())
            }
        };

        let key_val = UnsafeValue::from_slice(&raw_key.key);
        let val = txn.get::<_, UnsafeValue>(&db, key_val, None).ok_or("key doesn't exists".to_owned());
        if let Ok(val) = val {
            Ok(KeyState::Insert(Value::for_key(key, unsafe { val.as_slice() })))
        } else {
            Ok(KeyState::Delete)
        }
    }
}

fn open_db(txn: &mut MutTxn<()>, root: usize) -> Db<UnsafeValue, UnsafeValue> {
    if let Some(db) = txn.root(root) {
        db
    } else {
        // TODO: no unwrap
        txn.create_db().unwrap()
    }
}
