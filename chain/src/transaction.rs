use keys::{KeyImage, Signature, PublicKey};
use rct::RctSignature;
use hash::H256;

pub type RingSignature = Vec<Signature>;

/// A transaction.
#[derive(Debug)]
pub struct Transaction {
    pub prefix: TransactionPrefix,
    pub signatures: Vec<Vec<Signature>>,
    pub rct_signatures: RctSignature,
}

/// Transaction prefix.
#[derive(Debug, Default)]
pub struct TransactionPrefix {
    pub version: u8,
    pub vin: Vec<TxIn>,
    pub vout: Vec<TxOut>,
    pub extra: Vec<u8>,
}

serialize2! {
    TransactionPrefix {
        version -> (uvarint),
        vin -> (array),
        vout -> (array),
        extra -> (array),
    }
}

/// Transaction input.
#[derive(Debug)]
pub enum TxIn {
    Gen(TxInGen),
    ToScript(TxInToScript),
    ToScriptHash(TxInToScriptHash),
    ToKey(TxInToKey),
}

impl Default for TxIn {
    fn default() -> TxIn {
        TxIn::Gen(TxInGen::default())
    }
}

serialize2_variant! {
    TxIn {
        TxIn::Gen => (TxInGen::deserialize, 0xff),
        TxIn::ToScript => (TxInToScript::deserialize, 0x0),
        TxIn::ToScriptHash => (TxInToScriptHash::deserialize, 0x1),
        TxIn::ToKey => (TxInToKey::deserialize, 0x2),
    }
}

#[derive(Debug, Default)]
pub struct TxInGen {
    pub height: usize,
}

serialize2! {
    TxInGen {
        height -> (uvarint),
    }
}

#[derive(Debug, Default)]
pub struct TxInToScript {
    pub prev: H256,
    pub prevout: usize,
    pub sigset: Vec<u8>,
}

serialize2! {
    TxInToScript {
        prev -> (blob),
        prevout -> (uvarint),
        sigset -> (array),
    }
}

#[derive(Debug, Default)]
pub struct TxInToScriptHash {
    pub prev: H256,
    pub prevout: usize,
    pub script: TxOutToScript,
    pub sigset: Vec<u8>,
}

serialize2! {
    TxInToScriptHash {
        prev -> (blob),
        prevout -> (uvarint),
        script -> (struct),
        sigset -> (array),
    }
}

#[derive(Debug, Default)]
pub struct TxInToKey {
    pub amount: u64,
    pub key_offsets: Vec<u64>,
    pub k_image: KeyImage,
}

serialize2! {
    TxInToKey {
        amount -> (uvarint),
        key_offsets -> (array),
        k_image -> (blob),
    }
}

/// Transaction output.
#[derive(Debug, Default)]
pub struct TxOut {
    pub amount: u64,
    pub target: TxOutTarget,
}

serialize2! {
    TxOut {
        amount -> (uvarint),
        target -> (struct),
    }
}

/// Transaction output target.
#[derive(Debug)]
pub enum TxOutTarget {
    ToScript(TxOutToScript),
    ToScriptHash(TxOutToScriptHash),
    ToKey(TxOutToKey)
}

impl Default for TxOutTarget {
    fn default() -> TxOutTarget {
        TxOutTarget::ToScript(TxOutToScript::default())
    }
}

serialize2_variant! {
    TxOutTarget {
        TxOutTarget::ToScript => (TxOutToScript::deserialize, 0x0),
        TxOutTarget::ToScriptHash => (TxOutToScriptHash::deserialize, 0x1),
        TxOutTarget::ToKey => (TxOutToKey::deserialize, 0x2),
    }
}

#[derive(Debug, Default)]
pub struct TxOutToScript {
    pub keys: Vec<PublicKey>,
    pub script: Vec<u8>,
}

serialize2! {
    TxOutToScript {
        keys -> (array),
        script -> (array),
    }
}

#[derive(Debug, Default)]
pub struct TxOutToScriptHash {
    pub hash: H256,
}

serialize2! {
    TxOutToScriptHash {
        hash -> (blob),
    }
}

#[derive(Debug, Default)]
pub struct TxOutToKey {
    pub key: PublicKey,
}

serialize2! {
    TxOutToKey {
        key -> (blob),
    }
}

#[cfg(test)]
pub mod tests {
    use super::*;

    use hash::H256;
    use serialization::binary_serialize as serialize;

    #[test]
    fn block_header_test_vector() {
        let test_vector = include_bytes!("../../compat/test-vectors/data/BLOCK_HEADER_TEST_VECTOR").to_vec();

        let hdr = BlockHeader {
            major_version: 1, 
            minor_version: 0,
            timestamp: 0,
            prev_id: H256::default(),
            nonce: 0,
        };

        let buf = serialize(&hdr);

        if buf.as_ref() != test_vector.as_slice() {
            panic!("buf doesn't match test vector.");
        }
    }
}
