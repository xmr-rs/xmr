extern crate linked_hash_map;
extern crate bytes;

#[macro_use]
extern crate failure_derive;

#[macro_use]
pub extern crate failure;

use linked_hash_map::LinkedHashMap;
use bytes::{Buf, BufMut, BytesMut, ByteOrder};

pub mod ser;
pub use ser::{Deserialize, Serialize};

#[macro_use]
pub mod errors;
pub use errors::Result;

pub mod header;
pub mod raw_size;

pub const SERIALIZE_TYPE_INT64: u8 = 1;
pub const SERIALIZE_TYPE_INT32: u8 = 2;
pub const SERIALIZE_TYPE_INT16: u8 = 3;
pub const SERIALIZE_TYPE_INT8: u8 = 4;
pub const SERIALIZE_TYPE_UINT64: u8 = 5;
pub const SERIALIZE_TYPE_UINT32: u8 = 6;
pub const SERIALIZE_TYPE_UINT16: u8 = 7;
pub const SERIALIZE_TYPE_UINT8: u8 = 8;
pub const SERIALIZE_TYPE_DOUBLE: u8 = 9;
pub const SERIALIZE_TYPE_STRING: u8 = 10;
pub const SERIALIZE_TYPE_BOOL: u8 = 11;
pub const SERIALIZE_TYPE_OBJECT: u8 = 12;
pub const SERIALIZE_TYPE_ARRAY: u8 = 13;
pub const SERIALIZE_FLAG_ARRAY: u8 = 0x80;

#[derive(Debug)]
pub enum StorageEntry {
    U64(u64),
    U32(u32),
    U16(u16),
    U8(u8),
    I64(i64),
    I32(i32),
    I16(i16),
    I8(i8),
    Double(f64),
    Bool(bool),
    Buf(Vec<u8>),
    Array(Array),
    Section(Section),
}

impl StorageEntry {
    fn read<T: ByteOrder, B: Buf>(buf: &mut B) -> Result<StorageEntry> {
        let serialize_type = buf.get_u8();
        if serialize_type & SERIALIZE_FLAG_ARRAY == SERIALIZE_FLAG_ARRAY {
            let arr = Array::read::<T, B>(buf, serialize_type)?;
            return Ok(StorageEntry::Array(arr));
        }

        Self::read_entry_raw::<T, B>(buf, serialize_type)
    }

    fn read_entry_raw<T: ByteOrder, B: Buf>(buf: &mut B, serialize_type: u8) -> Result<StorageEntry> {
        let entry = match serialize_type {
            SERIALIZE_TYPE_INT64 => {
                ensure_eob!(buf, 8);
                StorageEntry::I64(buf.get_i64::<T>())
            },
            SERIALIZE_TYPE_INT32 => {
                ensure_eob!(buf, 4);
                StorageEntry::I32(buf.get_i32::<T>())
            },
            SERIALIZE_TYPE_INT16 => {
                ensure_eob!(buf, 2);
                StorageEntry::I16(buf.get_i16::<T>())
            },
            SERIALIZE_TYPE_INT8 => {
                ensure_eob!(buf, 1);
                StorageEntry::I8(buf.get_i8())
            },
            SERIALIZE_TYPE_UINT64 => {
                ensure_eob!(buf, 8);
                StorageEntry::U64(buf.get_u64::<T>())
            },
            SERIALIZE_TYPE_UINT32 => {
                ensure_eob!(buf, 4);
                StorageEntry::U32(buf.get_u32::<T>())
            },
            SERIALIZE_TYPE_UINT16 => {
                ensure_eob!(buf, 2);
                StorageEntry::U16(buf.get_u16::<T>())
            },
            SERIALIZE_TYPE_UINT8 => {
                ensure_eob!(buf, 1);
                StorageEntry::U8(buf.get_u8())
            },
            SERIALIZE_TYPE_DOUBLE => {
                ensure_eob!(buf, 8);
                StorageEntry::Double(buf.get_f64::<T>())
            },
            SERIALIZE_TYPE_STRING => {
                let b = read_buf::<T, B>(buf)?;
                StorageEntry::Buf(b)
            },
            SERIALIZE_TYPE_BOOL => {
                ensure_eob!(buf, 1);
                StorageEntry::Bool(buf.get_u8() != 0)
            },
            SERIALIZE_TYPE_OBJECT => {
                StorageEntry::Section(Section::read::<T, B>(buf)?)
            },
            SERIALIZE_TYPE_ARRAY => {
                ensure_eob!(buf, 1);
                let serialize_type = buf.get_u8();
                if serialize_type & SERIALIZE_FLAG_ARRAY != SERIALIZE_FLAG_ARRAY {
                    panic!();
                }

                let arr = Array::read::<T, B>(buf, serialize_type)?;
                StorageEntry::Array(arr)
            },
            _ => panic!(), // TODO: failure
        };

        Ok(entry)
    }

    fn write<T: ByteOrder>(buf: &mut BytesMut, entry: &Self) {
        match entry {
            &StorageEntry::U64(ref v) => {
                buf.reserve(9);
                buf.put_u8(SERIALIZE_TYPE_UINT64);
                buf.put_u64::<T>(*v);
            },
            &StorageEntry::U32(ref v) => {
                buf.reserve(5);
                buf.put_u8(SERIALIZE_TYPE_UINT32);
                buf.put_u32::<T>(*v);
            },
            &StorageEntry::U16(ref v) => {
                buf.reserve(3);
                buf.put_u8(SERIALIZE_TYPE_UINT16);
                buf.put_u16::<T>(*v);
            },
            &StorageEntry::U8(ref v) => {
                buf.reserve(2);
                buf.put_u8(SERIALIZE_TYPE_UINT8);
                buf.put_u8(*v);
            },
            &StorageEntry::I64(ref v) => {
                buf.reserve(9);
                buf.put_u8(SERIALIZE_TYPE_INT64);
                buf.put_i64::<T>(*v);
            },
            &StorageEntry::I32(ref v) => {
                buf.reserve(5);
                buf.put_u8(SERIALIZE_TYPE_INT32);
                buf.put_i32::<T>(*v);
            },
            &StorageEntry::I16(ref v) => {
                buf.reserve(3);
                buf.put_u8(SERIALIZE_TYPE_INT16);
                buf.put_i16::<T>(*v);
            },
            &StorageEntry::I8(ref v) => {
                buf.reserve(2);
                buf.put_u8(SERIALIZE_TYPE_INT8);
                buf.put_i8(*v);
            },
            &StorageEntry::Double(ref v) => {
                buf.reserve(9);
                buf.put_u8(SERIALIZE_TYPE_DOUBLE);
                buf.put_f64::<T>(*v);
            },
            &StorageEntry::Bool(ref v) => {
                buf.reserve(2);
                buf.put_u8(SERIALIZE_TYPE_BOOL);
                buf.put_u8(if *v == false { 0 } else { 1 });
            },
            &StorageEntry::Buf(ref v) => {
                buf.reserve(1);
                buf.put_u8(SERIALIZE_TYPE_STRING);
                write_buf::<T>(buf, v);
            },
            &StorageEntry::Array(ref v) => Array::write::<T>(buf, v), 
            &StorageEntry::Section(ref v) => Section::write::<T>(buf, v), 
        }
    }
}

#[derive(Debug)]
pub struct Array(pub Vec<StorageEntry>, pub u8);

impl Array {
    fn read<T: ByteOrder, B: Buf>(buf: &mut B, mut serialize_type: u8) -> Result<Array> {
        let orig_serialize_type = serialize_type;
        if serialize_type & SERIALIZE_FLAG_ARRAY != SERIALIZE_FLAG_ARRAY {
            // TODO: failure
            panic!();
        } else {
            serialize_type &= !SERIALIZE_FLAG_ARRAY;
        }

        let size = raw_size::read::<T, B>(buf)?;

        let mut array = Array(Vec::with_capacity(size), orig_serialize_type);
        array.0.reserve(size);

        for _ in 0..size {
            array.0.push(StorageEntry::read_entry_raw::<T, B>(buf, serialize_type)?);
        }

        Ok(array)
    }

    fn write<T: ByteOrder>(buf: &mut BytesMut, array: &Array) {
        buf.reserve(1);
        buf.put_u8(array.1);
        for entry in array.0.iter() {
            StorageEntry::write::<T>(buf, &entry);
        }
    }
}

#[derive(Debug)]
pub struct Section {
    pub entries: LinkedHashMap<String, StorageEntry>,
}

impl Section {
    pub fn new() -> Section {
        Section {
            entries: LinkedHashMap::new(),
        }
    }

    /// Insernt an storage entry.
    pub fn insert<T: Into<StorageEntry>>(&mut self, name: String, entry: T) {
        self.entries.insert(name, entry.into());
    }

    fn read<T: ByteOrder, B: Buf>(buf: &mut B) -> Result<Section> {
        let mut section = Section::new();
        let count = raw_size::read::<T, B>(buf)?;

        section.entries.reserve(count);
        for _ in 0..count {
            let name = read_name::<B>(buf)?;
            let entry = StorageEntry::read::<T, B>(buf)?;
            section.entries.insert(name.clone(), entry);
        }

        Ok(section)
    }

    fn write<T: ByteOrder>(buf: &mut BytesMut, section: &Self) {
        raw_size::write::<T>(buf, section.entries.len());

        for (name, entry) in section.entries.iter() {

            write_name(buf, &*name);
            StorageEntry::write::<T>(buf, &entry);
        }
    }
}

pub fn read<T: ByteOrder, B: Buf>(buf: &mut B) -> Result<Section> {
    let header = header::StorageBlockHeader::read::<T, B>(buf)?;
    if (header.signature_a != header::PORTABLE_STORAGE_SIGNATUREA ||
        header.signature_b != header::PORTABLE_STORAGE_SIGNATUREB) &&
        header.version != header::PORTABLE_STORAGE_FORMAT_VER {
        // TODO: failure
        panic!()
    }

    Section::read::<T, B>(buf)
}

pub fn write<T: ByteOrder>(buf: &mut BytesMut, section: &Section) {
    header::StorageBlockHeader::write::<T>(buf);
    Section::write::<T>(buf, section);
}

fn read_name<B: Buf>(buf: &mut B) -> Result<String> {
    ensure_eob!(buf, 1);
    let length = buf.get_u8() as usize;
    ensure_eob!(buf, length);

    let s = String::from_utf8_lossy(&buf.bytes()[..length]).into_owned();
    buf.advance(length);
    Ok(s)
}

fn read_buf<T: ByteOrder, B: Buf>(buf: &mut B) -> Result<Vec<u8>> {
    let length = raw_size::read::<T, B>(buf)?;
    ensure_eob!(buf, length);

    let mut b = Vec::with_capacity(length);
    b.extend_from_slice(&buf.bytes()[..length]);
    buf.advance(length);
    Ok(b)
}

fn write_buf<T: ByteOrder>(buf: &mut BytesMut, b: &Vec<u8>) {
    raw_size::write::<T>(buf, b.len());

    buf.reserve(b.len());
    buf.put(b.as_slice());
}

fn write_name(buf: &mut BytesMut, name: &str) {
    buf.reserve(name.as_bytes().len() + 1);
    buf.put_u8(name.as_bytes().len() as u8);
    buf.put(name.as_bytes());
}
