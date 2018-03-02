use format::{
    Deserialize,
    DeserializerStream,
    Error,
    Serialize,
    SerializerStream
};

#[derive(Debug, Clone)]
pub struct TxInGen {
    pub height: u64,
}

impl Deserialize for TxInGen {
    fn deserialize(mut deserializer: DeserializerStream) -> Result<Self, Error> {
        deserializer.get_u64_varint()
            .map(|height| TxInGen { height })
    }
}

impl Serialize for TxInGen {
    fn serialize(&self, mut serializer: SerializerStream) {
        serializer.put_u64_varint(self.height)
    }
}
