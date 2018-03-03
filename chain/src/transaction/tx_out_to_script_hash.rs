use primitives::{H256, H256_LENGTH};
use format::{
    Deserialize,
    DeserializerStream,
    Error,
    Serialize,
    SerializerStream
};

#[derive(Debug, Clone)]
pub struct TxOutToScriptHash {
    pub hash: H256,
}

impl Deserialize for TxOutToScriptHash {
    fn deserialize(deserializer: &mut DeserializerStream) -> Result<Self, Error> {
        let hash = H256::from_bytes(deserializer.get_blob(H256_LENGTH)?);

        Ok(TxOutToScriptHash { hash })
    }
}

impl Serialize for TxOutToScriptHash {
    fn serialize(&self, mut serializer: SerializerStream) {
        serializer.put_blob(self.hash.as_bytes());
    }
}
