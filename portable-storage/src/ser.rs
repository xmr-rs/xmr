use serde::Serializer;
use serde::ser::{Error as ErrorTrait, Impossible, Serialize, SerializeSeq, SerializeStruct};
use serde::de::value::Error;

use {Section, Array, StorageEntry};

pub fn to_section<T: Serialize>(v: &T) -> Result<Section, Error> {
    v.serialize(RootSectionSerializer)
}

macro_rules! unsupported {
    ($method:ident, $ty:ty) => {
        fn $method(self, _: $ty) -> Result<Self::Ok, Self::Error> {
            Err(Error::custom(concat!("serializing a `", stringify!($ty), "` isn't supported")))
        }
    }
}

struct RootSectionSerializer;

impl Serializer for RootSectionSerializer {
    type Ok = Section;
    type Error = Error;
    type SerializeSeq = Impossible<Self::Ok, Self::Error>;
    type SerializeTuple = Impossible<Self::Ok, Self::Error>;
    type SerializeTupleStruct = Impossible<Self::Ok, Self::Error>;
    type SerializeTupleVariant = Impossible<Self::Ok, Self::Error>;
    type SerializeMap = Impossible<Self::Ok, Self::Error>;
    type SerializeStruct = KvSerializer;
    type SerializeStructVariant = Impossible<Self::Ok, Self::Error>;

    unsupported!(serialize_bool, bool);
    unsupported!(serialize_i8, i8);
    unsupported!(serialize_i16, i16);
    unsupported!(serialize_i32, i32);
    unsupported!(serialize_i64, i64);
    unsupported!(serialize_u8, u8);
    unsupported!(serialize_u16, u16);
    unsupported!(serialize_u32, u32);
    unsupported!(serialize_u64, u64);
    unsupported!(serialize_f32, f32);
    unsupported!(serialize_f64, f64);
    unsupported!(serialize_char, char);
    unsupported!(serialize_str, &str);
    unsupported!(serialize_bytes, &[u8]);

    fn serialize_none(self) -> Result<Self::Ok, Self::Error> {
        Err(Error::custom("serializing `None` isn't supported"))
    }

    fn serialize_some<T: ?Sized>(
            self, 
            _value: &T
        ) -> Result<Self::Ok, Self::Error>
        where
            T: Serialize {
        Err(Error::custom("serializing `Some(_)` isn't supported"))
    }

    fn serialize_unit(self) -> Result<Self::Ok, Self::Error> {
        Err(Error::custom("serializing `()` isn't supported"))
    }

    fn serialize_unit_struct(self, _name: &'static str) -> Result<Self::Ok, Self::Error> {
        Err(Error::custom("serializing `()` isn't supported"))
    }

    fn serialize_unit_variant(
            self, 
            _name: &'static str, 
            _variant_index: u32, 
            _variant: &'static str
        ) -> Result<Self::Ok, Self::Error> {
        Err(Error::custom("serializing `()` isn't supported"))
    }

    fn serialize_newtype_struct<T: ?Sized>(
            self, 
            _name: &'static str, 
            _value: &T
        ) -> Result<Self::Ok, Self::Error>
        where
            T: Serialize {
        Err(Error::custom("serializing a newtype struct isn't supported"))
    }

    fn serialize_newtype_variant<T: ?Sized>(
            self, 
            _name: &'static str, 
            _variant_index: u32, 
            _variant: &'static str, 
            _value: &T
        ) -> Result<Self::Ok, Self::Error>
        where
            T: Serialize {
        Err(Error::custom("serializing a newtype variant isn't supported"))
    }


    fn serialize_seq(
        self, 
        _len: Option<usize>
    ) -> Result<Self::SerializeSeq, Self::Error> {
        Err(Error::custom("serializing a sequence isn't supported"))
    }

    fn serialize_tuple(
        self, 
        _len: usize
    ) -> Result<Self::SerializeTuple, Self::Error> {
        Err(Error::custom("serializing a tuple isn't supported"))
    }

    fn serialize_tuple_struct(
        self, 
        _name: &'static str, 
        _len: usize
    ) -> Result<Self::SerializeTupleStruct, Self::Error> {;
        Err(Error::custom("serializing a tuple struct isn't supported"))
    }

    fn serialize_tuple_variant(
        self, 
        _name: &'static str, 
        _variant_index: u32, 
        _variant: &'static str, 
        _len: usize
    ) -> Result<Self::SerializeTupleVariant, Self::Error> {
        Err(Error::custom("serializing a tuple variant isn't supported"))
    }

    fn serialize_map(
        self, 
        _len: Option<usize>
    ) -> Result<Self::SerializeMap, Self::Error> {
        Err(Error::custom("serializing a map isn't supported"))
    }

    fn serialize_struct(
        self, 
        _name: &'static str, 
        len: usize
    ) -> Result<Self::SerializeStruct, Self::Error> {
        Ok(KvSerializer(Section::with_capacity(len)))
    }

    fn serialize_struct_variant(
        self, 
        _name: &'static str, 
        _variant_index: u32, 
        _variant: &'static str, 
        _len: usize
    ) -> Result<Self::SerializeStructVariant, Self::Error> {
        Err(Error::custom("serializing a struct variant isn't supported"))
    }

    fn is_human_readable(&self) -> bool { false }
}

struct KvSerializer(Section);

impl SerializeStruct for KvSerializer {
    type Ok = Section;
    type Error = Error;

    fn serialize_field<T: ?Sized>(
        &mut self, 
        key: &'static str, 
        value: &T
    ) -> Result<(), Self::Error>
    where
        T: Serialize {
        let entry = value.serialize(StorageEntrySerializer)?;
        self.0.insert(key.to_string(), entry);
        Ok(())
    }

    fn end(self) -> Result<Self::Ok, Self::Error> {
        Ok(self.0)
    }

    fn skip_field(&mut self, _key: &'static str) -> Result<(), Self::Error> {
        Err(Error::custom("fields can't be skipped"))
    }
}

struct ArraySerializer(Array);

impl ArraySerializer {
    fn push(&mut self, entry: StorageEntry) -> Result<(), Error> {
        self.0.push(entry).map_err(|_| {
            Error::custom("array entries need to be of the same type.")
        })
    }
}

impl SerializeSeq for ArraySerializer {
    type Ok = StorageEntry;
    type Error = Error;

    fn serialize_element<T: ?Sized>(
            &mut self, 
            value: &T
        ) -> Result<(), Self::Error>
        where
            T: Serialize {
        let entry = value.serialize(StorageEntrySerializer)?;
        self.push(entry)
    }

    fn end(self) -> Result<Self::Ok, Self::Error> {
        Ok(StorageEntry::Array(self.0))
    }
}

macro_rules! storage_entry {
    ($method:ident, $ty:ty, $path:path) => {
        fn $method(self, v: $ty) -> Result<Self::Ok, Self::Error> {
            Ok($path(v))
        }
    }
}

struct EntryKvSerializer(Section);

impl SerializeStruct for EntryKvSerializer {
    type Ok = StorageEntry;
    type Error = Error;

    fn serialize_field<T: ?Sized>(
        &mut self, 
        key: &'static str, 
        value: &T
    ) -> Result<(), Self::Error>
    where
        T: Serialize {
        let entry = value.serialize(StorageEntrySerializer)?;
        self.0.insert(key.to_string(), entry);
        Ok(())
    }

    fn end(self) -> Result<Self::Ok, Self::Error> {
        Ok(StorageEntry::Section(self.0))
    }

    fn skip_field(&mut self, _key: &'static str) -> Result<(), Self::Error> {
        Err(Error::custom("fields can't be skipped"))
    }
}

struct StorageEntrySerializer;

impl Serializer for StorageEntrySerializer {
    type Ok = StorageEntry;
    type Error = Error;
    type SerializeSeq = ArraySerializer;
    type SerializeTuple = Impossible<Self::Ok, Self::Error>;
    type SerializeTupleStruct = Impossible<Self::Ok, Self::Error>;
    type SerializeTupleVariant = Impossible<Self::Ok, Self::Error>;
    type SerializeMap = Impossible<Self::Ok, Self::Error>;
    type SerializeStruct = EntryKvSerializer;
    type SerializeStructVariant = Impossible<Self::Ok, Self::Error>;

    storage_entry!(serialize_bool, bool, StorageEntry::Bool);
    storage_entry!(serialize_i8, i8, StorageEntry::I8);
    storage_entry!(serialize_i16, i16, StorageEntry::I16);
    storage_entry!(serialize_i32, i32, StorageEntry::I32);
    storage_entry!(serialize_i64, i64, StorageEntry::I64);
    storage_entry!(serialize_u8, u8, StorageEntry::U8);
    storage_entry!(serialize_u16, u16, StorageEntry::U16);
    storage_entry!(serialize_u32, u32, StorageEntry::U32);
    storage_entry!(serialize_u64, u64, StorageEntry::U64);
    unsupported!(serialize_f32, f32);
    storage_entry!(serialize_f64, f64, StorageEntry::Double);
    unsupported!(serialize_char, char);
    unsupported!(serialize_str, &str);

    fn serialize_bytes(self, v: &[u8]) -> Result<Self::Ok, Self::Error> {
        Ok(StorageEntry::Buf(v.to_vec()))
    }

    fn serialize_none(self) -> Result<Self::Ok, Self::Error> {
        Err(Error::custom("serializing `None` isn't supported"))
    }

    fn serialize_some<T: ?Sized>(
            self, 
            _value: &T
        ) -> Result<Self::Ok, Self::Error>
        where
            T: Serialize {
        Err(Error::custom("serializing `Some(_)` isn't supported"))
    }

    fn serialize_unit(self) -> Result<Self::Ok, Self::Error> {
        Err(Error::custom("serializing `()` isn't supported"))
    }

    fn serialize_unit_struct(self, _name: &'static str) -> Result<Self::Ok, Self::Error> {
        Err(Error::custom("serializing `()` isn't supported"))
    }

    fn serialize_unit_variant(
            self, 
            _name: &'static str, 
            _variant_index: u32, 
            _variant: &'static str
        ) -> Result<Self::Ok, Self::Error> {
        Err(Error::custom("serializing `()` isn't supported"))
    }

    fn serialize_newtype_struct<T: ?Sized>(
            self, 
            _name: &'static str, 
            _value: &T
        ) -> Result<Self::Ok, Self::Error>
        where
            T: Serialize {
        Err(Error::custom("serializing a newtype struct isn't supported"))
    }

    fn serialize_newtype_variant<T: ?Sized>(
            self, 
            _name: &'static str, 
            _variant_index: u32, 
            _variant: &'static str, 
            _value: &T
        ) -> Result<Self::Ok, Self::Error>
        where
            T: Serialize {
        Err(Error::custom("serializing a newtype variant isn't supported"))
    }


    fn serialize_seq(
        self, 
        len: Option<usize>
    ) -> Result<Self::SerializeSeq, Self::Error> {
        if let Some(len) = len {
            Ok(ArraySerializer(Array::with_capacity(len)))
        } else {
            Ok(ArraySerializer(Array::new()))
        }
    }

    fn serialize_tuple(
        self, 
        _len: usize
    ) -> Result<Self::SerializeTuple, Self::Error> {
        Err(Error::custom("serializing a tuple isn't supported"))
    }

    fn serialize_tuple_struct(
        self, 
        _name: &'static str, 
        _len: usize
    ) -> Result<Self::SerializeTupleStruct, Self::Error> {;
        Err(Error::custom("serializing a tuple struct isn't supported"))
    }

    fn serialize_tuple_variant(
        self, 
        _name: &'static str, 
        _variant_index: u32, 
        _variant: &'static str, 
        _len: usize
    ) -> Result<Self::SerializeTupleVariant, Self::Error> {
        Err(Error::custom("serializing a tuple variant isn't supported"))
    }

    fn serialize_map(
        self, 
        _len: Option<usize>
    ) -> Result<Self::SerializeMap, Self::Error> {
        Err(Error::custom("serializing a map isn't supported"))
    }

    fn serialize_struct(
        self, 
        _name: &'static str, 
        len: usize
    ) -> Result<Self::SerializeStruct, Self::Error> {
        Ok(EntryKvSerializer(Section::with_capacity(len)))
    }

    fn serialize_struct_variant(
        self, 
        _name: &'static str, 
        _variant_index: u32, 
        _variant: &'static str, 
        _len: usize
    ) -> Result<Self::SerializeStructVariant, Self::Error> {
        Err(Error::custom("serializing a struct variant isn't supported"))
    }

    fn is_human_readable(&self) -> bool { false }
}

#[cfg(test)]
pub mod tests {
    use super::*;

    use portable_storage::StorageEntry;

    #[derive(Serialize)]
    struct TestVector0 {
        id: u8,
        transaction_proof: u32,
    }

    #[test]
    fn test_vector_0() {
        let test_vector_0 = TestVector0 {
            id: 56,
            transaction_proof: 1337,
        };

        let section = to_section(&test_vector_0).unwrap();
        match section["id"] {
            StorageEntry::U8(56) => (),
            _ => panic!(),
        }

        match section["transaction_proof"] {
            StorageEntry::U32(1337) => (),
            _ => panic!(),
        }
    }
}
