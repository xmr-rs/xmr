use keys::{PublicKey, PUBLIC_KEY_LENGTH};
use format::{Deserialize, DeserializerStream, Error, Serialize, SerializerStream};

#[derive(Debug, Clone)]
pub struct TxOutToKey {
    pub key: PublicKey,
}

impl Deserialize for TxOutToKey {
    fn deserialize(deserializer: &mut DeserializerStream) -> Result<Self, Error> {
        let key = PublicKey::from_bytes(deserializer.get_blob(PUBLIC_KEY_LENGTH)?);

        Ok(TxOutToKey { key })
    }
}

impl Serialize for TxOutToKey {
    fn serialize(&self, mut serializer: SerializerStream) {
        serializer.put_blob(self.key.as_bytes());
    }
}
