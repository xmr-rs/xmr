/// The start ID of levin commands and notifications.
pub const COMMAND_BASE_ID: u32 = 1000;

/// The id of a command.
pub type Id = u32;

/// A levin command.
///
/// [*See the* `Bucket` *type for more information.*](/bucket/struct.Bucket.html)
pub trait Command {
    /// The ID of this notification.
    ///
    /// Should be higher than [`COMMAND_BASE_ID`][1] and should
    /// be different to other commands/notifications IDs.
    ///
    /// [1]: const.COMMAND_BASE_ID.html
    const ID: Id;
}
