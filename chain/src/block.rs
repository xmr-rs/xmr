use block_header::BlockHeader;
use transaction::Transaction;
use hash::{H256, H256_LENGTH};
use format::{
    Deserialize,
    DeserializerStream,
    Error,
    Serialize,
    SerializerStream,
};

/// A block.
pub struct Block {
    pub header: BlockHeader,
    pub miner_tx: Transaction,
    pub tx_hashes: Vec<H256>,
}

impl Block {
    pub fn transaction_tree_hash(&self) -> H256 {
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
