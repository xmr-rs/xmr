use levin::Command;

use types::P2P_COMMAND_BASE_ID;

/// Support for fluffypony's compact blocks.
pub const P2P_SUPPORT_FLAG_FLUFFY_BLOCKS: u32 = 0x01;

/// All the support flags.
pub const P2P_SUPPORT_FLAGS: u32 = P2P_SUPPORT_FLAG_FLUFFY_BLOCKS;

/// Request for support flags.
#[derive(Debug)]
pub struct RequestSupportFlags;

impl Command for RequestSupportFlags {
    const ID: u32 = P2P_COMMAND_BASE_ID + 7;
}

/// The support flags of a node.
#[derive(Debug, Default, Clone, Deserialize, Serialize)]
pub struct SupportFlagsResponse {
    pub support_flags: u32,
}

impl SupportFlagsResponse {
    /// Creates a `SupportFlagsResponse` with the flags
    /// supported by this implementation.
    pub fn supported() -> SupportFlagsResponse {
        SupportFlagsResponse { support_flags: P2P_SUPPORT_FLAGS, }
    }
}
