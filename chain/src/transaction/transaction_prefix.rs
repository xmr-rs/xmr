use transaction::{TxIn, TxOut};
use primitives::H256;
use format::{Deserialize, DeserializerStream, Error, Serialize, SerializerStream, to_binary};

/// Transaction prefix.
#[derive(Debug, Clone)]
pub struct TransactionPrefix {
    pub version: u8,
    pub unlock_time: u64,
    pub vin: Vec<TxIn>,
    pub vout: Vec<TxOut>,
    pub extra: Vec<u8>,
}

impl TransactionPrefix {
    pub fn hash(&self) -> H256 {
        H256::fast_hash(to_binary(self))
    }
}

impl Deserialize for TransactionPrefix {
    fn deserialize(deserializer: &mut DeserializerStream) -> Result<Self, Error> {
        let version = deserializer.get_u8_varint()?;
        let unlock_time = deserializer.get_u64_varint()?;

        let vin_length = deserializer.get_u64_varint()? as usize;
        let mut vin = Vec::with_capacity(vin_length);

        for _ in 0..vin_length {
            vin.push(deserializer.get_deserializable()?);
        }

        let vout_length = deserializer.get_u64_varint()? as usize;
        let mut vout = Vec::with_capacity(vout_length);

        for _ in 0..vout_length {
            vout.push(deserializer.get_deserializable()?);
        }

        let extra_length = deserializer.get_u64_varint()? as usize;
        let extra = deserializer.get_blob(extra_length)?;

        Ok(TransactionPrefix {
               version,
               unlock_time,
               vin,
               vout,
               extra,
           })
    }
}

impl Serialize for TransactionPrefix {
    fn serialize(&self, mut serializer: SerializerStream) {
        serializer.put_u8_varint(self.version);
        serializer.put_u64_varint(self.unlock_time);

        serializer.put_u64_varint(self.vin.len() as u64);
        for tx in self.vin.iter() {
            serializer.put_serializable(tx);
        }

        serializer.put_u64_varint(self.vout.len() as u64);
        for tx in self.vout.iter() {
            serializer.put_serializable(tx);
        }

        serializer.put_u64_varint(self.extra.len() as u64);
        serializer.put_blob(self.extra.as_slice());
    }

    fn len(&self) -> usize {
        use varint;

        let mut sum = 0;
        sum += varint::length(self.version);
        sum += varint::length(self.unlock_time);
        sum += varint::length(self.vin.len());
        for tx in self.vin.iter() {
            sum += tx.len();
        }
        sum += varint::length(self.vout.len());
        for tx in self.vout.iter() {
            sum += tx.len();
        }
        sum += varint::length(self.extra.len());
        sum += self.extra.len();
        sum
    }
}
