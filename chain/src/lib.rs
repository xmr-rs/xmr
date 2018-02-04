extern crate hash;

use hash::H256;

/// The metadata at the beginning of each block.
#[derive(Debug)]
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
