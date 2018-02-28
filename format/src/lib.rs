extern crate bytes;
extern crate varint;
extern crate failure;
#[macro_use]
extern crate failure_derive;

mod de;
mod ser;

pub use de::{Deserialize, DeserializerStream, Error, from_binary};
pub use ser::{Serialize, SerializerStream, to_binary};
