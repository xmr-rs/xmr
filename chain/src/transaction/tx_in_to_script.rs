use primitives::{H256, H256_LENGTH};
use format::{
    Deserialize,
    DeserializerStream,
    Error,
    Serialize,
    SerializerStream
};

#[derive(Debug, Clone)]
pub struct TxInToScript {
    pub prev: H256,
    pub prevout: u64,
    pub sigset: Vec<u8>,
}

impl Deserialize for TxInToScript {
    fn deserialize(deserializer: &mut DeserializerStream) -> Result<Self, Error> {
        let prev = H256::from_bytes(deserializer.get_blob(H256_LENGTH)?);
        let prevout = deserializer.get_u64_varint()?;

        let sigset_length = deserializer.get_u64_varint()? as usize;
        let sigset = deserializer.get_blob(sigset_length)?.to_vec();

        Ok(TxInToScript {
            prev,
            prevout,
            sigset,
        })
    }
}

impl Serialize for TxInToScript {
    fn serialize(&self, mut serializer: SerializerStream) {
        serializer.put_blob(self.prev.as_bytes());
        serializer.put_u64_varint(self.prevout);

        serializer.put_u64_varint(self.sigset.len() as u64);
        serializer.put_blob(self.sigset.as_slice());
    }
}
