use hash::{H256, H256_LENGTH};
use format::{
    Deserialize,
    DeserializerStream,
    Error,
    Serialize,
    SerializerStream,
};

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
    pub nonce: [u8; 4],
}

impl Deserialize for BlockHeader {
    fn deserialize(mut deserializer: DeserializerStream) -> Result<Self, Error> {
        let major_version = deserializer.get_u8_varint()?;
        let minor_version = deserializer.get_u8_varint()?;
        let timestamp = deserializer.get_u64_varint()?;
        let prev_id = H256::from_bytes(deserializer.get_blob(H256_LENGTH)?);
        let nonce = match deserializer.get_blob(4) {
            Ok(v) => {
                let mut n = [0u8; 4];
                n.copy_from_slice(v);
                n
            }
            Err(e) => return Err(e),
        };

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
        serializer.put_blob(self.prev_id.as_ref());
        serializer.put_blob(&self.nonce)
    }
}
