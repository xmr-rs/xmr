use hash::H256;

/// Synchronization information between nodes.
#[derive(Debug, Default, Clone, Deserialize, Serialize)]
pub struct CoreSyncData {
    /// The current block height
    pub current_height: u64,
    /// The cumulative difficulty.
    pub cumulative_difficulty: u64,
    /// The top block id.
    pub top_id: H256,
    /// The top block version.
    pub top_version: u8,
}
