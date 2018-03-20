use levin::Command;
use portable_storage_utils::Blob;

use types::{P2P_COMMAND_BASE_ID, PeerId};

/// The ping command.
#[derive(Debug)]
pub struct Ping;

impl Command for Ping {
    const ID: u32 = P2P_COMMAND_BASE_ID + 3;
}

const PING_RESPONSE_STATUS: &'static [u8] = b"OK\0";

/// The response of the ping command.
#[derive(Debug, Default, Clone, Deserialize, Serialize)]
pub struct PingResponse {
    /// The ping status.
    pub status: Blob,
    /// The ID of the peer being ping'ed.
    pub peer_id: PeerId,
}

impl PingResponse {
    /// Creates a new ping response.
    pub fn new(peer_id: PeerId) -> PingResponse {
        PingResponse {
            status: PING_RESPONSE_STATUS.into(),
            peer_id,
        }
    }

    /// Checks the status of the ping command, returns `true` if it's ok.
    pub fn is_ok(&self) -> bool {
        &*self.status.0 == PING_RESPONSE_STATUS
    }
}
