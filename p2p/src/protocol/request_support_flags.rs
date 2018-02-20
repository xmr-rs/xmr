use portable_storage::ser::Empty;

use protocol::P2P_COMMAND_BASE_ID;
use levin::Command;

#[derive(Debug)]
pub struct RequestSupportFlags;

impl Command for RequestSupportFlags {
    type Request = Empty;
    type Response = RequestSupportFlagsResponse;

    const ID: u32 = P2P_COMMAND_BASE_ID + 7;
}

/// The handshake command response.
#[derive(Debug, Default, Clone)]
pub struct RequestSupportFlagsResponse {
    pub support_flags: u32,
}

serializable! {
    RequestSupportFlagsResponse {
        support_flags,
    }
}
