extern crate bytes;
extern crate failure;
#[macro_use]
extern crate portable_storage;

pub mod codec;
pub mod command;
pub mod address;

pub type DefaultEndian = bytes::LittleEndian;
