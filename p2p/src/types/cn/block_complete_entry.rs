// Xmr, Monero node.
// Copyright (C) 2018  Jean Pierre Dudey
//
// This program is free software: you can redistribute it and/or modify
// it under the terms of the GNU General Public License as published by
// the Free Software Foundation, either version 3 of the License, or
// (at your option) any later version.
//
// This program is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
// GNU General Public License for more details.
//
// You should have received a copy of the GNU General Public License
// along with this program.  If not, see <http://www.gnu.org/licenses/>.

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
            }
            entry => panic!("invalid entry: {:?}", entry),
        }
    }
}
