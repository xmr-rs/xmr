extern crate uuid;
extern crate serde;
extern crate bytes;

extern crate failure;
#[macro_use]
extern crate failure_derive;

pub mod stl;

mod bytes_uuid;
pub use bytes_uuid::BytesUuid;
