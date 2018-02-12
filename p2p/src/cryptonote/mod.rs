use hash::H256;

/// Synchronization information between nodes.
///
/// This is used by default in the [`Handshake`][handshake]
/// and the [`TimedSync`][timedsync] command in the payload
/// field.
///
/// [handshake]: ../commands/handshake/struct.Handshake.html
/// [timedsync]: ../commands/timedsync/struct.TimedSync.html
#[derive(Debug, Default, Clone)]
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

serializable! {
    CoreSyncData {
        current_height,
        cumulative_difficulty,
        top_id,
        top_version,
    }
}
