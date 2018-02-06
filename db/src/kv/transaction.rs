use bytes::Bytes;
use hash::H256;
use chain::BlockHeader;
use serialization::{binary_serialize as serialize, binary_deserialize as deserialize};

pub const COL_BLOCKS: usize = 0;

#[derive(Debug)]
pub enum Operation {
	Insert(KeyValue),
	Delete(Key),
}

#[derive(Debug)]
pub enum KeyValue {
    /// The block header.
    Block(H256, BlockHeader),
}

#[derive(Debug)]
pub enum Key {
    /// The block hash height.
    Block(H256),
}

#[derive(Debug, Clone)]
pub enum Value {
    /// The block hash.
    Block(BlockHeader),
}

impl Value {
    pub fn for_key(key: &Key, bytes: &[u8]) -> Value {
        match *key {
            Key::Block(_) => Value::Block(deserialize(&bytes)),
        }
    }
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
    pub location: usize,
    pub key: Bytes,
    pub value: Bytes,
}

impl<'a> From<&'a KeyValue> for RawKeyValue {
    fn from(kv: &'a KeyValue) -> RawKeyValue {
        let (location, key, value) = match *kv {
            KeyValue::Block(ref k, ref v) => (COL_BLOCKS, Bytes::from(k.as_bytes()), serialize(v))
        };
        
        RawKeyValue {
            location,
            key,
            value,
        }
    }
}

#[derive(Debug)]
pub struct RawKey {
    pub location: usize,
    pub key: Bytes,
}

impl<'a> From<&'a Key> for RawKey {
    fn from(k: &'a Key) -> RawKey {
        let (location, key) = match *k {
            Key::Block(ref k) => (COL_BLOCKS, Bytes::from(k.as_bytes())),
        };

        RawKey {
            location,
            key,
        }
    }
}
