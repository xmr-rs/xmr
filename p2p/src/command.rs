use portable_storage::{Serialize, Deserialize};

pub trait Command {
    type Request: Serialize + Deserialize;
    type Response: Serialize + Deserialize;
    const ID: usize;
}
