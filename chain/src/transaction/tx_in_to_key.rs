use keys::{KeyImage, KEY_IMAGE_LENGTH};
use format::{
    Deserialize,
    DeserializerStream,
    Error,
    Serialize,
    SerializerStream
};

#[derive(Debug)]
pub struct TxInToKey {
    pub amount: u64,
    pub key_offsets: Vec<u64>,
    pub k_image: KeyImage,
}

impl Deserialize for TxInToKey {
    fn deserialize(mut deserializer: DeserializerStream) -> Result<Self, Error> {
        let amount = deserializer.get_u64_varint()?;

        let key_offsets_length = deserializer.get_u64_varint()? as usize;
        let mut key_offsets = Vec::with_capacity(key_offsets_length);
        for _ in 0..key_offsets_length {
            key_offsets.push(deserializer.get_u64_varint()?);
        }

        let k_image = KeyImage::from_bytes(deserializer.get_blob(KEY_IMAGE_LENGTH)?);

        Ok(TxInToKey {
            amount,
            key_offsets,
            k_image,
        })
    }
}

impl Serialize for TxInToKey {
    fn serialize(&self, mut serializer: SerializerStream) {
        serializer.put_u64_varint(self.amount);

        serializer.put_u64_varint(self.key_offsets.len() as u64);
        for offset in self.key_offsets.iter() {
            serializer.put_u64_varint(*offset);
        }

        serializer.put_blob(self.k_image.as_bytes())
    }
}
