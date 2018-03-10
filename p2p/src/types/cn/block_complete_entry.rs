use portable_storage_utils::Blob;

#[derive(Debug, Deserialize, Serialize)]
pub struct BlockCompleteEntry {
    pub block: Blob,
    pub txs: Vec<Blob>,
}

#[cfg(test)]
pub mod tests {
    use super::*;
    use portable_storage::{StorageEntry, to_section};

    #[test]
    fn serialize() {
        let b = BlockCompleteEntry {
            block: vec![].into(),
            txs: vec![vec![0x20].into()],
        };

        let section = to_section(&b).unwrap();

        match &section["block"] {
            &StorageEntry::Buf(ref buf) => assert_eq!(buf.len(), 0),
            entry => panic!("invalid entry: {:?}", entry),
        }

        match &section["txs"] {
            &StorageEntry::Array(ref array) => {
                assert_eq!(array.len(), 1); 
                match &array[0] {
                    &StorageEntry::Buf(ref buf) => assert_eq!(buf[0], 0x20),
                    entry => panic!("invalid entry: {:?}", entry),
                }
            },
            entry => panic!("invalid entry: {:?}", entry),
        }
    }
}
