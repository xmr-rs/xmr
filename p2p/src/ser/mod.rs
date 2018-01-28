//! Some values that can be used with the `serialize!` macro 

pub mod linked_list;
pub use self::linked_list::{DefaultSerializableLinkedList, SerializableLinkedList};

pub mod uuid;
pub use self::uuid::SerializableUuid;

