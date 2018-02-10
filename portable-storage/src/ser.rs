use {Result, Section, StorageEntry};

pub mod bytes {
    use bytes::{ByteOrder, BytesMut, Buf};
    use Result;

    pub trait SerializeBytes: Sized {
        fn to_bytes<T: ByteOrder>(&self, buf: &mut BytesMut);
        fn from_bytes<T: ByteOrder, B: Buf>(buf: &mut B) -> Result<Self>;
    }
}

#[derive(Debug, Clone, Copy, Fail)]
#[fail(display = "unexpected portable-storage entry, expected {}", expected)]
pub struct InvalidStorageEntry {
    pub expected: &'static str,
}

impl InvalidStorageEntry {
    pub fn new(expected: &'static str) -> InvalidStorageEntry {
        InvalidStorageEntry {
            expected
        }
    }
}

pub trait Serialize {
    fn serialize(&self) -> Section;
}

pub trait Deserialize: Default {
    fn deserialize(section: &Section) -> Result<Self>;
}

pub trait ToUnderlying: Sized { 
    fn to_underlying(entry: &StorageEntry) -> Result<Self>;
}

pub trait Serializable: Sized + Deserialize + Serialize + Default + Clone + Into<StorageEntry> + ToUnderlying {}

impl<T> Serializable for T where T: Sized + Deserialize + Serialize + Default + Clone + Into<StorageEntry> + ToUnderlying {}

macro_rules! impl_to_underlying {
    ($variant:path => $ty:ty) => {
        impl $crate::ser::ToUnderlying for $ty {
            fn to_underlying(entry: &$crate::StorageEntry) -> $crate::Result<$ty>
            {
                match entry {
                    &$variant(ref v) => Ok(v.clone()),
                    _ => Err($crate::failure::Error::from(
                            $crate::ser::InvalidStorageEntry::new(stringify!($variant))
                        ))
                }
            }
        }
    }
}

macro_rules! impl_from_for_storage_entry {
    ($ty:ty => $entry:path) => {
        impl From<$ty> for $crate::StorageEntry {
            fn from(v: $ty) -> $crate::StorageEntry {
                $entry(v)
            }
        }
    }
}

impl_to_underlying!(StorageEntry::U64 => u64);
impl_to_underlying!(StorageEntry::U32 => u32);
impl_to_underlying!(StorageEntry::U16 => u16);
impl_to_underlying!(StorageEntry::U8 => u8);
impl_to_underlying!(StorageEntry::I64 => i64);
impl_to_underlying!(StorageEntry::I32 => i32);
impl_to_underlying!(StorageEntry::I16 => i16);
impl_to_underlying!(StorageEntry::I8 => i8);
impl_to_underlying!(StorageEntry::Double => f64);
impl_to_underlying!(StorageEntry::Bool => bool);
impl_to_underlying!(StorageEntry::Buf => Vec<u8>);

impl_from_for_storage_entry!(u64 => StorageEntry::U64);
impl_from_for_storage_entry!(u32 => StorageEntry::U32);
impl_from_for_storage_entry!(u16 => StorageEntry::U16);
impl_from_for_storage_entry!(u8 => StorageEntry::U8);
impl_from_for_storage_entry!(i64 => StorageEntry::I64);
impl_from_for_storage_entry!(i32 => StorageEntry::I32);
impl_from_for_storage_entry!(i16 => StorageEntry::I16);
impl_from_for_storage_entry!(i8 => StorageEntry::I8);
impl_from_for_storage_entry!(f64 => StorageEntry::Double);
impl_from_for_storage_entry!(bool => StorageEntry::Bool);
impl_from_for_storage_entry!(Vec<u8> => StorageEntry::Buf);

#[macro_export]
macro_rules! serializable {
    (
        $struct_name:ident {
            $($fname:ident ,)+
        }
    ) =>{
        impl $crate::ser::Deserialize for $struct_name {
            fn deserialize(section: &$crate::Section) -> $crate::Result<$struct_name>
            {
                let mut result = Self::default();
                for (k, v) in section.entries.iter() {
                    $(
                        if k == stringify!($fname) {
                            result.$fname = $crate::ser::ToUnderlying::to_underlying(v)?;
                        }
                    )+
                }
                Ok(result)
            }
        }

        impl $crate::ser::Serialize for $struct_name {
            fn serialize(&self) -> $crate::Section {
                let mut section = $crate::Section::new();
                $(
                    section.insert(stringify!($fname).to_string(), self.$fname.clone());
                )+

                section
            }
        }

        impl From<$struct_name> for $crate::StorageEntry {
            fn from(v: $struct_name) -> $crate::StorageEntry {
                use $crate::ser::Serialize;
                $crate::StorageEntry::Section(v.serialize())
            }
        }

        impl $crate::ser::ToUnderlying for $struct_name {
            fn to_underlying(entry: &$crate::StorageEntry) -> $crate::Result<$struct_name>
            {
                use $crate::ser::Deserialize;
                match entry {
                    &$crate::StorageEntry::Section(ref v) => Self::deserialize(v),
                    _ => Err(
                        $crate::failure::Error::from(
                            $crate::ser::InvalidStorageEntry::new("StorageEntry::Section")
                        )
                    ),
                }
            }
        }
    };

    (
        $struct_name:ident<$tyargs:tt> where ($($tytraits:tt)+) {
            $($fname:ident ,)+
        }
    ) =>{
        impl<$($tytraits)+> $crate::ser::Deserialize for $struct_name<$tyargs> {
            fn deserialize(section: &$crate::Section) -> $crate::Result<$struct_name<$tyargs>>
            {
                let mut result = Self::default();
                for (k, v) in section.entries.iter() {
                    $(
                        if k == stringify!($fname) {
                            result.$fname = $crate::ser::ToUnderlying::to_underlying(v)?;
                        }
                    )+
                }
                Ok(result)
            }
        }

        impl<$($tytraits)+> $crate::ser::Serialize for $struct_name<$tyargs> {
            fn serialize(&self) -> $crate::Section {
                let mut section = $crate::Section::new();
                $(
                    section.insert(stringify!($fname).to_string(), self.$fname.clone());
                )+

                section
            }
        }

        impl<$($tytraits)+> From<$struct_name<$tyargs>> for $crate::StorageEntry {
            fn from(v: $struct_name<$tyargs>) -> $crate::StorageEntry {
                use $crate::ser::Serialize;
                $crate::StorageEntry::Section(v.serialize())
            }
        }

        impl<$($tytraits)+> $crate::ser::ToUnderlying for $struct_name<$tyargs> {
            fn to_underlying(entry: &$crate::StorageEntry) -> $crate::Result<$struct_name<$tyargs>>
            {
                use $crate::ser::Deserialize;
                match entry {
                    &$crate::StorageEntry::Section(ref v) => Self::deserialize(v),
                    _ => Err(
                        $crate::failure::Error::from(
                            $crate::ser::InvalidStorageEntry::new("StorageEntry::Section")
                        )
                    ),
                }
            }
        }
    };
}
