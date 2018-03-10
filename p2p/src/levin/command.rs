use serde::{Serialize, Deserialize};
use serde::de::value::Error;

use portable_storage::{self, Section};

pub const COMMAND_BASE_ID: u32 = 1000;

pub trait Command {
    type Request: Storage;
    type Response: Storage;

    const ID: u32;
}

pub trait Notify {
    type Request: Storage;

    const ID: u32;
}

pub trait Storage: Sized {
    fn from_section(section: Section) -> Result<Self, Error>;
    fn to_section(&self) -> Result<Section, Error>;
}

impl<'de, T> Storage for T
    where T: Serialize + Deserialize<'de> + Sized
{
    fn from_section(section: Section) -> Result<Self, Error> {
        portable_storage::from_section(section)
    }

    fn to_section(&self) -> Result<Section, Error> {
        portable_storage::to_section(self)
    }
}

#[derive(Debug, Clone, Copy, Eq, PartialEq)]
pub struct Empty;

impl Storage for Empty {
    fn from_section(_section: Section) -> Result<Self, Error> {
        Ok(Empty)
    }

    fn to_section(&self) -> Result<Section, Error> {
        Ok(Section::new())
    }
}
