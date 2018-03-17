use serde::{Serialize, Deserialize};
use serde::de::value::Error;

use portable_storage::{self, Section};

/// A type that can be converted from/to a `Section`.
pub trait Storage: Sized {
    /// Convert a section into this type.
    fn from_section(section: Section) -> Result<Self, Error>;

    /// Convert this type to a `Section`.
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

/// An empty type without any data.
///
/// This type can be used in request/response fields
/// when one of them isn't needed.
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
