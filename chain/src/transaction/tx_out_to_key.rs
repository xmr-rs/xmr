use keys::{PublicKey, PUBLIC_KEY_LENGTH};
use format::{Deserialize, DeserializerStream, Error, Serialize, SerializerStream};

#[derive(Debug, Clone)]
pub struct TxOutToKey {
    pub key: PublicKey,
}

impl Deserialize for TxOutToKey {
    fn deserialize(deserializer: &mut DeserializerStream) -> Result<Self, Error> {
        deserializer
            .get_deserializable()
            .map(|key| TxOutToKey { key })
    }
}

impl Serialize for TxOutToKey {
    fn serialize(&self, mut serializer: SerializerStream) {
        serializer.put_serializable(&self.key);
    }

    fn len(&self) -> usize {
        PUBLIC_KEY_LENGTH
    }
}
