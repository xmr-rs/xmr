use keys::{PublicKey, PUBLIC_KEY_LENGTH};
use format::{
    Deserialize,
    DeserializerStream,
    Error,
    Serialize,
    SerializerStream
};

#[derive(Debug, Clone)]
pub struct TxOutToScript {
    pub keys: Vec<PublicKey>,
    pub script: Vec<u8>,
}

impl Deserialize for TxOutToScript {
    fn deserialize(deserializer: &mut DeserializerStream) -> Result<Self, Error> {
        let keys_length = deserializer.get_u64_varint()?;
        let mut keys = Vec::new();
        keys.reserve(keys_length as usize);

        for _ in 0..keys_length {
            let key = deserializer.get_blob(PUBLIC_KEY_LENGTH)?;
            keys.push(PublicKey::from_bytes(key));
        }

        let script_length = deserializer.get_u64_varint()? as usize;
        let script = deserializer.get_blob(script_length)?.to_vec();


        Ok(TxOutToScript {
            keys,
            script,
        })
    }
}


impl Serialize for TxOutToScript {
    fn serialize(&self, mut serializer: SerializerStream) {
        serializer.put_u64_varint(self.keys.len() as u64);

        for key in self.keys.iter() {
            serializer.put_blob(key.as_bytes());
        }

        serializer.put_u64_varint(self.script.len() as u64);
        serializer.put_blob(self.script.as_slice());
    }
}
