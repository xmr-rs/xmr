use storage::Storage;

/// The start ID of levin commands and notifications.
pub const COMMAND_BASE_ID: u32 = 1000;

/// A levin command.
///
/// Commands are *invoked* and every command that is invoked
/// **should** have a response.
///
/// [*See the* `Bucket` *type for more information.*](/bucket/struct.Bucket.html)
pub trait Command {
    /// The request data of this command.
    type Request: Storage;

    /// The response data of this command.
    type Response: Storage;

    /// The ID of this notification.
    ///
    /// Should be higher than [`COMMAND_BASE_ID`][1] and should
    /// be different to other commands/notifications IDs.
    ///
    /// [1]: const.COMMAND_BASE_ID.html
    const ID: u32;
}

/// A levin notification.
pub trait Notify {
    /// The request data of a notification.
    type Request: Storage + Send + Sync + 'static;

    /// The ID of this notification.
    ///
    /// Should be higher than [`COMMAND_BASE_ID`][1] and should
    /// be different to other commands/notifications IDs.
    ///
    /// [1]: const.COMMAND_BASE_ID.html
    const ID: u32;
}
