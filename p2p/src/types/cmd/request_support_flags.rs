use types::P2P_COMMAND_BASE_ID;
use levin::{Command, Empty};

#[derive(Debug)]
pub struct RequestSupportFlags;

impl Command for RequestSupportFlags {
    type Request = Empty;
    type Response = RequestSupportFlagsResponse;

    const ID: u32 = P2P_COMMAND_BASE_ID + 7;
}

/// The handshake command response.
#[derive(Debug, Default, Clone, Deserialize, Serialize)]
pub struct RequestSupportFlagsResponse {
    pub support_flags: u32,
}
