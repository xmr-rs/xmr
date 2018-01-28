use portable_storage::{Serialize, Deserialize};

pub const COMMAND_BASE_ID: u32 = 1000;

pub trait Command {
    type Request: Serialize + Deserialize;
    type Response: Serialize + Deserialize;

    const ID: u32;
}
