use keys::{Signature, SIGNATURE_LENGTH};
use rct::Signature as RctSignature;
use transaction::TransactionPrefix;
use primitives::H256;
use format::{
    Deserialize,
    DeserializerStream,
    Error,
    Serialize,
    SerializerStream,
    from_binary,
    to_binary,
};

/// A transaction.
pub struct Transaction {
    pub prefix: TransactionPrefix,
    pub signature_type: SignatureType,
}

impl Transaction {
    pub fn from_bytes<T: AsRef<[u8]>>(v: T) -> Result<Self, Error> {
        from_binary::<Self>(v.as_ref())
    }

    pub fn hash(&self) -> H256 {
        match self.prefix.version {
            1 => H256::fast_hash(to_binary(self)),
            2 => unimplemented!(),
            _ => panic!("invalid tx version"),
        }
    }
}

pub enum SignatureType {
    Normal(Vec<Vec<Signature>>),
    RingCt(RctSignature),
}

impl Deserialize for Transaction {
    fn deserialize(mut deserializer: DeserializerStream) -> Result<Self, Error> {
        let prefix: TransactionPrefix = deserializer.get_deserializable()?;

        let signature_type = match prefix.version {
             1 => {
                let mut signatures = Vec::with_capacity(prefix.vin.len());
                for txin in prefix.vin.iter() {
                    if txin.signature_size() != 0 {
                        let mut txin_sigs = Vec::with_capacity(txin.signature_size());

                        for _ in 0..txin.signature_size() {
                            let blob = deserializer.get_blob(SIGNATURE_LENGTH)?;
                            txin_sigs.push(Signature::from_bytes(blob));
                        }

                        signatures.push(txin_sigs);
                    } else {
                        signatures.push(Vec::new())
                    }
                }

                SignatureType::Normal(signatures)
            },
            2 => unimplemented!(),
            n => return Err(Error::custom(format!("invalid transaction version ({})", n))),
        };

        Ok(Transaction {
            prefix,
            signature_type,
        })
    }
}

impl Serialize for Transaction {
    fn serialize(&self, mut serializer: SerializerStream) {
        serializer.put_serializable(&self.prefix);

        match self.signature_type {
            SignatureType::Normal(ref signatures) => {
                assert_eq!(self.prefix.version, 1, "signature type doesn't match version");

                for sigv in signatures.iter() {
                    for sig in sigv.iter() {
                        serializer.put_blob(sig.as_bytes());
                    }
                }
            },
            SignatureType::RingCt(_) => {
                assert_eq!(self.prefix.version, 2, "signature type doesn't match version");
                unimplemented!()
            },
        }
    }
}
