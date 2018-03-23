use primitives::H256;
use format::{Deserialize, DeserializerStream, Error, Serialize, SerializerStream};

/// The metadata at the beginning of each block.
#[derive(Debug, Default, Clone)]
pub struct BlockHeader {
    /// Major block header version.
    pub major_version: u8,
    /// Minor block header version, now used as a voting mechanism.
    pub minor_version: u8,
    /// Block creation time (UNIX timestamps).
    pub timestamp: u64,
    /// Identifier of the previous block.
    pub prev_id: H256,
    /// Any value which is used in the network consensus algorithm.
    pub nonce: u32,
}

impl Deserialize for BlockHeader {
    fn deserialize(deserializer: &mut DeserializerStream) -> Result<Self, Error> {
        let major_version = deserializer.get_u8_varint()?;
        let minor_version = deserializer.get_u8_varint()?;
        let timestamp = deserializer.get_u64_varint()?;
        let prev_id = deserializer.get_deserializable()?;
        let nonce = deserializer.get_u32()?;

        Ok(BlockHeader {
               major_version,
               minor_version,
               timestamp,
               prev_id,
               nonce,
           })
    }
}

impl Serialize for BlockHeader {
    fn serialize(&self, mut serializer: SerializerStream) {
        serializer.put_u8_varint(self.major_version);
        serializer.put_u8_varint(self.minor_version);
        serializer.put_u64_varint(self.timestamp);
        serializer.put_serializable(&self.prev_id);
        serializer.put_u32(self.nonce)
    }

    fn len(&self) -> usize {
        use varint;

        let mut sum = 0;

        sum += varint::length(self.major_version);
        sum += varint::length(self.minor_version);
        sum += varint::length(self.timestamp);
        sum += self.prev_id.len();
        sum += 4;
        sum
    }
}
