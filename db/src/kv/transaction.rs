use bytes::{BytesMut, Bytes, Buf, BufMut, IntoBuf, LittleEndian};
use primitives::H256;
use chain::Block;
use format::{to_binary, from_binary};

pub const COL_META: usize = 0;
pub const COL_BLOCKS: usize = 1;
pub const COL_BLOCK_HEIGHTS: usize = 2;
pub const COL_BLOCK_HASHES: usize = 2;

#[derive(Debug)]
pub enum Operation {
	Insert(KeyValue),
	Delete(Key),
}

#[derive(Debug)]
pub enum KeyValue {
    Meta(&'static str, Bytes),
    Block(H256, Block),
    BlockHeight(H256, u64),
    BlockId(u64, H256),
}

#[derive(Debug)]
pub enum Key {
    Meta(&'static str),
    Block(H256),
    BlockHeight(H256),
    BlockId(u64),
}

#[derive(Debug, Clone)]
pub enum Value {
    Meta(Bytes),
    Block(Block),
    BlockHeight(u64),
    BlockId(H256),
}

impl Value {
    pub fn for_key(key: &Key, bytes: &[u8]) -> Value {
        match *key {
            Key::Meta(_) => Value::Meta(bytes.into()),
            Key::Block(_) => Value::Block(from_binary(bytes).unwrap()),
            Key::BlockHeight(_) => {
                if bytes.len() != 8 {
                    unimplemented!();
                }

                let mut buf = bytes.into_buf();
                Value::BlockHeight(buf.get_u64::<LittleEndian>())
            },
            Key::BlockId(_) => Value::BlockId(H256::from_bytes(&bytes)),
        }
    }

    pub fn as_meta(self) -> Option<Bytes> {
        match self {
            Value::Meta(bytes) => Some(bytes),
            _ => None,
        }
    }

    pub fn as_block(self) -> Option<Block> {
        match self {
            Value::Block(block) => Some(block),
            _ => None,
        }
    }

    pub fn as_block_height(self) -> Option<u64> {
        match self {
            Value::BlockHeight(height) => Some(height),
            _ => None,
        }
    }

    pub fn as_block_id(self) -> Option<H256> {
        match self {
            Value::BlockId(hash) => Some(hash),
            _ => None,
        }
    }
}

#[derive(Debug, Clone)]
pub enum KeyState<V> {
	Insert(V),
	Delete,
	Unknown,
}

impl<V> KeyState<V> {
	pub fn into_option(self) -> Option<V> {
		match self {
			KeyState::Insert(value) => Some(value),
			KeyState::Delete => None,
			KeyState::Unknown => None,
		}
	}
}

/// A list of operations to be done.
#[derive(Debug)]
pub struct Transaction {
	pub operations: Vec<Operation>,
}

impl Transaction {
    /// Creates a new `Transaction`.
    pub fn new() -> Transaction {
        Transaction {
            operations: Vec::new(),
        }
    }

    /// Inserts a key-value pair onto the database.
    pub fn insert(&mut self, kv: KeyValue) {
        self.operations.push(Operation::Insert(kv));
    }

    /// Deletes a key-value pair given a key.
    pub fn delete(&mut self, k: Key) {
        self.operations.push(Operation::Delete(k));
    }
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
            KeyValue::Meta(ref k, ref v) => (COL_META, Bytes::from(k.as_bytes()), v.clone()),
            KeyValue::Block(ref k, ref v) => (COL_BLOCKS, Bytes::from(k.as_bytes()), to_binary(v)),
            KeyValue::BlockHeight(ref k, ref v) => {
                let mut buf = BytesMut::with_capacity(8);
                buf.put_u64::<LittleEndian>(*v);
                (COL_BLOCK_HEIGHTS, Bytes::from(k.as_bytes()), buf.freeze())
            },
            KeyValue::BlockId(ref k, ref v) => {
                let mut buf = BytesMut::with_capacity(8);
                buf.put_u64::<LittleEndian>(*k);
                (COL_BLOCK_HASHES, buf.freeze(), Bytes::from(v.as_bytes()))
            }
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
            Key::Meta(ref k) => (COL_META, Bytes::from(k.as_bytes())),
            Key::Block(ref k) => (COL_BLOCKS, Bytes::from(k.as_bytes())),
            Key::BlockHeight(ref k) => (COL_BLOCK_HEIGHTS, Bytes::from(k.as_bytes())),
            Key::BlockId(ref k) => {
                let mut buf = BytesMut::with_capacity(8);
                buf.put_u64::<LittleEndian>(*k);
                (COL_BLOCK_HASHES, buf.freeze())
            },
        };

        RawKey {
            location,
            key,
        }
    }
}
