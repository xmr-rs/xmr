extern crate uuid;

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

    /// Returns this peer's listening port.
    pub fn listen_port(&self) -> u32 {
        match *self {
            Network::Mainnet => 18080,
            Network::Testnet => 28080,
        }
    }

    pub fn hard_forks(&self) -> HardForks {
        match *self {
            Network::Mainnet => {
                &[
                    HardFork { version: 1, height: 1, threshold: 0, time: 1341378000 },
                    HardFork { version: 2, height: 1009827, threshold: 0, time: 1442763710 },
                    HardFork { version: 3, height: 1141317, threshold: 0, time: 1458558528 },
                    HardFork { version: 4, height: 1220516, threshold: 0, time: 1483574400 },
                    HardFork { version: 5, height: 1288616, threshold: 0, time: 1489520158 },
                    HardFork { version: 6, height: 1400000, threshold: 0, time: 1503046577 },
                ]
            },
            Network::Testnet => {
                &[
                    HardFork { version: 1, height: 1, threshold: 0, time: 1341378000 },
                    HardFork { version: 2, height: 624634, threshold: 0, time: 1445355000 },
                    HardFork { version: 3, height: 800500, threshold: 0, time: 1472415034 },
                    HardFork { version: 4, height: 801219, threshold: 0, time: 1472415035 },
                    HardFork { version: 5, height: 802660, threshold: 0, time: 1472415036 + 86400*180 }
                    HardFork { version: 6, height: 971400, threshold: 0, time: 1501709789 },
                    HardFork { version: 7, height: 1057028, threshold: 0, time: 1512211236 },
                ]
            }
        }
    }
}

#[derive(Debug)]
pub struct HardForks {
    pub parameters: &'static [HardForkParameters],
}

impl HardForks {
    pub fn ideal_version(&self) -> u8 {
        let back = self.parameters.len() - 1;
        self.parameters[back].version
    }
}

impl From<&'static [HardForkParameters]> for HardForks {
    fn from(parameters: &'static [HardForkParameters]) -> HardForks {
        HardForks {
            parameters,
        }
    }
}

/// The information about a hard fork.
#[derive(Debug)]
pub struct HardForkParameters {
    /// The version.
    pub version: u8,
    /// The block height.
    pub height: u64,
    // XXX: what does this mean on monero.
    /// Threshold
    pub threshold: u8,
    /// Time since epoch.
    pub time: u64,
}
