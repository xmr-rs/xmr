use types::{P2P_COMMAND_BASE_ID, PeerId};
use levin::{Command, Empty};

#[derive(Debug)]
pub struct Ping;

impl Command for Ping {
    type Request = Empty;
    type Response = PingResponse;

    const ID: u32 = P2P_COMMAND_BASE_ID + 3;
}

const PING_RESPONSE_STATUS: &'static [u8] = b"OK\0";

/// The handshake command response.
#[derive(Debug, Default, Clone, Deserialize, Serialize)]
pub struct PingResponse {
    pub status: Vec<u8>,
    pub peer_id: PeerId,
}

impl PingResponse {
    pub fn is_ok(&self) -> bool {
        &*self.status == PING_RESPONSE_STATUS
    }
}
