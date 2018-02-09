use uuid::Uuid;

pub const MAINNET_NETWORK_ID: [u8; 16] = [
    0x12 ,0x30, 0xF1, 0x71, 0x61, 0x04, 0x41, 0x61, 0x17, 0x31, 0x00, 0x82,
    0x16, 0xA1, 0xA1, 0x10,
];
pub const TESTNET_NETWORK_ID: [u8; 16] = [
    0x12 ,0x30, 0xF1, 0x71, 0x61, 0x04, 0x41, 0x61, 0x17, 0x31, 0x00, 0x82,
    0x16, 0xA1, 0xA1, 0x11,
];

/// The Monero network we are in
#[derive(Debug, Clone, Copy, Eq, PartialEq)]
pub enum Network {
    /// The real world, the place where Monero coins have real world economic
    /// value.
    Mainnet,
    /// The main Monero testnet, the place where you can dream you are rich.
    Testnet,
}

impl Network {
    /// Returns the network's ID.
    pub fn id(&self) -> Uuid {
        let id = match *self {
            Network::Mainnet => MAINNET_NETWORK_ID,
            Network::Testnet => TESTNET_NETWORK_ID,
        };

        Uuid::from_bytes(&id).expect("invalid network id")
    }
}
