// Copyright 2018 Jean Pierre Dudey <jeandudey@hotmail.com>
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

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
