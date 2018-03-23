use transaction::TxOutTarget;
use format::{Deserialize, DeserializerStream, Error, Serialize, SerializerStream};

/// Transaction output.
#[derive(Debug, Clone)]
pub struct TxOut {
    pub amount: u64,
    pub target: TxOutTarget,
}

impl Deserialize for TxOut {
    fn deserialize(deserializer: &mut DeserializerStream) -> Result<Self, Error> {
        let amount = deserializer.get_u64_varint()?;
        let target = deserializer.get_deserializable()?;

        Ok(TxOut { amount, target })
    }
}

impl Serialize for TxOut {
    fn serialize(&self, mut serializer: SerializerStream) {
        serializer.put_u64_varint(self.amount);
        serializer.put_serializable(&self.target);
    }

    fn len(&self) -> usize {
        use varint;

        varint::length(self.amount) + self.target.len()
    }
}
