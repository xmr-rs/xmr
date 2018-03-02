use block_header::BlockHeader;
use transaction::Transaction;
use primitives::{H256, H256_LENGTH};
use format::{
    Deserialize,
    DeserializerStream,
    Error,
    Serialize,
    SerializerStream,
    to_binary,
};
use bytes::{Bytes, BytesMut, BufMut};
use varint;

/// A block.
#[derive(Clone)]
pub struct Block {
    pub header: BlockHeader,
    pub miner_tx: Transaction,
    pub tx_hashes: Vec<H256>,
}

impl Block {
    /// Calculate block PoW (CryptoNight) hash.
    pub fn hash(&self) -> H256 {
        H256::slow_hash(self.hashable_blob())
    }

    /// Calculate the block identifier.
    pub fn id(&self) -> H256 {
        H256::fast_hash(self.hashable_blob())
    }

    fn hashable_blob(&self) -> Bytes {
        let mut buf: BytesMut = to_binary(&self.header).into();
        buf.put(self.transaction_tree_hash().as_bytes());
        varint::write(&mut buf, self.tx_hashes.len() + 1);
        buf.freeze()
    }

    fn transaction_tree_hash(&self) -> H256 {
        H256::tree_hash(self.build_tree_ids())
    }

    fn build_tree_ids(&self) -> Vec<H256> {
        let mut txids = Vec::with_capacity(self.tx_hashes.len() + 1);
        txids.push(self.miner_tx.hash());
        txids.extend_from_slice(self.tx_hashes.as_slice());
        txids
    }
}

impl Deserialize for Block {
    fn deserialize(mut deserializer: DeserializerStream) -> Result<Self, Error> {
        let header = deserializer.get_deserializable()?;
        let miner_tx = deserializer.get_deserializable()?;

        let tx_hashes_length = deserializer.get_u64_varint().map(|v| v as usize)?;
        let mut tx_hashes = Vec::with_capacity(tx_hashes_length);

        for _ in 0..tx_hashes_length {
            let hash = deserializer.get_blob(H256_LENGTH)?;
            tx_hashes.push(H256::from_bytes(hash));
        }

        Ok(Block {
            header,
            miner_tx,
            tx_hashes,
        })
    }
}

impl Serialize for Block {
    fn serialize(&self, mut serializer: SerializerStream) {
        serializer.put_serializable(&self.header);
        serializer.put_serializable(&self.miner_tx);

        serializer.put_u64_varint(self.tx_hashes.len() as u64);
        for txid in self.tx_hashes.iter() {
            serializer.put_blob(txid.as_bytes());
        }
    }
}
