use primitives::{H256, H256_LENGTH};
use format::{Deserialize, DeserializerStream, Error, Serialize, SerializerStream};

#[derive(Debug, Clone)]
pub struct TxOutToScriptHash {
    pub hash: H256,
}

impl Deserialize for TxOutToScriptHash {
    fn deserialize(deserializer: &mut DeserializerStream) -> Result<Self, Error> {
        deserializer
            .get_deserializable()
            .map(|hash| TxOutToScriptHash { hash })
    }
}

impl Serialize for TxOutToScriptHash {
    fn serialize(&self, mut serializer: SerializerStream) {
        serializer.put_serializable(&self.hash);
    }

    fn len(&self) -> usize {
        H256_LENGTH
    }
}
