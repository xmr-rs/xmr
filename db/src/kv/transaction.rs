use bytes::Bytes;
use hash::H256;

#[derive(Debug)]
pub enum Operation {
	Insert(KeyValue),
	Delete(Key),
}

#[derive(Debug)]
pub enum KeyValue {
    /// The block hash, the key is the block height and the value is the hash.
    BlockHash(u64, H256),
}

#[derive(Debug)]
pub enum Key {
    /// The block hash height.
    BlockHash(u64),
}

#[derive(Debug, Clone)]
pub enum Value {
    /// The block hash.
    BlockHash(H256),
}

#[derive(Debug, Clone)]
pub enum KeyState<V> {
	Insert(V),
	Delete,
	Unknown,
}

#[derive(Debug)]
pub struct Transaction {
	pub operations: Vec<Operation>,
}

#[derive(Debug)]
pub enum RawOperation {
    Insert(RawKeyValue),
    Delete(RawKey)
}

impl<'a> From<&'a Operation> for RawOperation {
    fn from(v: &'a Operation) -> RawOperation {
        match *v {
            Operation::Insert(ref kv) => RawOperation::Insert(kv.into()),
            Operation::Delete(ref k) => RawOperation::Delete(k.into()),
        }
    }
}

#[derive(Debug)]
pub struct RawKeyValue {
    pub key: Bytes,
    pub value: Bytes,
}

impl<'a> From<&'a KeyValue> for RawKeyValue {
    fn from(kv: &'a KeyValue) -> RawKeyValue {
        match *kv {
            KeyValue::BlockHash(ref key, ref value) => unimplemented!(),
        }
    }
}

#[derive(Debug)]
pub struct RawKey {
    pub key: Bytes,
}

impl<'a> From<&'a Key> for RawKey {
    fn from(k: &'a Key) -> RawKey {
        match *k {
            Key::BlockHash(ref key) => unimplemented!(),
        }
    }
}
