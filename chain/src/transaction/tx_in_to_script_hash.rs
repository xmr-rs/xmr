use primitives::H256;
use transaction::TxOutToScript;
use format::{Deserialize, DeserializerStream, Error, Serialize, SerializerStream};

#[derive(Debug, Clone)]
pub struct TxInToScriptHash {
    pub prev: H256,
    pub prevout: u64,
    pub script: TxOutToScript,
    pub sigset: Vec<u8>,
}

impl Deserialize for TxInToScriptHash {
    fn deserialize(deserializer: &mut DeserializerStream) -> Result<Self, Error> {
        let prev = deserializer.get_deserializable()?;
        let prevout = deserializer.get_u64_varint()?;
        let script = deserializer.get_deserializable()?;

        let sigset_length = deserializer.get_u64_varint()? as usize;
        let sigset = deserializer.get_blob(sigset_length)?;

        Ok(TxInToScriptHash {
               prev,
               prevout,
               script,
               sigset,
           })
    }
}

impl Serialize for TxInToScriptHash {
    fn serialize(&self, mut serializer: SerializerStream) {
        serializer.put_serializable(&self.prev);
        serializer.put_u64_varint(self.prevout);
        serializer.put_serializable(&self.script);

        serializer.put_u64_varint(self.sigset.len() as u64);
        serializer.put_blob(self.sigset.as_slice());
    }

    fn len(&self) -> usize {
        use varint;

        let mut sum = 0;
        sum += self.prev.len();
        sum += varint::length(self.prevout);
        sum += self.script.len();
        sum += varint::length(self.sigset.len());
        sum += self.sigset.len();
        sum
    }
}
