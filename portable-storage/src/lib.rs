// Copyright 2018 Jean Pierre Dudey <jeandudey@hotmail.com>
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

extern crate linked_hash_map;
extern crate bytes;

#[macro_use]
extern crate serde;
#[cfg_attr(test, macro_use)]
#[cfg(test)]
extern crate serde_derive;

extern crate failure;
#[macro_use]
extern crate failure_derive;

use std::ops::Index;

use linked_hash_map::LinkedHashMap;
use bytes::{Buf, BufMut, BytesMut, LittleEndian};

pub mod ser;
pub mod de;

pub use ser::to_section;
pub use de::from_section;

#[macro_export]
macro_rules! ensure_eof {
    ($buf:expr, $needed:expr) => {
        if $buf.remaining() < $needed {
            return Err($crate::Error::UnexpectedEof { needed: $needed });
        }
    };
}

pub mod header;
pub mod raw_size;

pub type Result<T> = ::std::result::Result<T, Error>;

#[derive(Debug, Clone, Fail)]
pub enum Error {
    #[fail(display = "reached EOF, needed {}", needed)]
    UnexpectedEof { needed: usize },
    #[fail(display = "the header isn't valid")]
    InvalidHeader,
    #[fail(display = "the storage entry serialize type isn't valid ({:X})", _0)]
    InvalidSerializeType(u8),
    #[fail(display = "the array serialize type isn't valid ({:X})", _0)]
    InvalidArrayType(u8),
}

const SERIALIZE_TYPE_INT64: u8 = 1;
const SERIALIZE_TYPE_INT32: u8 = 2;
const SERIALIZE_TYPE_INT16: u8 = 3;
const SERIALIZE_TYPE_INT8: u8 = 4;
const SERIALIZE_TYPE_UINT64: u8 = 5;
const SERIALIZE_TYPE_UINT32: u8 = 6;
const SERIALIZE_TYPE_UINT16: u8 = 7;
const SERIALIZE_TYPE_UINT8: u8 = 8;
const SERIALIZE_TYPE_DOUBLE: u8 = 9;
const SERIALIZE_TYPE_STRING: u8 = 10;
const SERIALIZE_TYPE_BOOL: u8 = 11;
const SERIALIZE_TYPE_OBJECT: u8 = 12;
const SERIALIZE_TYPE_ARRAY: u8 = 13;
const SERIALIZE_FLAG_ARRAY: u8 = 0x80;

#[derive(Debug, Clone)]
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
    fn read<B: Buf>(buf: &mut B) -> Result<StorageEntry> {
        let serialize_type = buf.get_u8();
        if serialize_type & SERIALIZE_FLAG_ARRAY == SERIALIZE_FLAG_ARRAY {
            let arr = Array::read::<B>(buf, serialize_type)?;
            return Ok(StorageEntry::Array(arr));
        }

        Self::read_entry_raw::<B>(buf, serialize_type)
    }

    fn read_entry_raw<B: Buf>(buf: &mut B, serialize_type: u8) -> Result<StorageEntry> {
        let entry = match serialize_type {
            SERIALIZE_TYPE_INT64 => {
                ensure_eof!(buf, 8);
                StorageEntry::I64(buf.get_i64::<LittleEndian>())
            }
            SERIALIZE_TYPE_INT32 => {
                ensure_eof!(buf, 4);
                StorageEntry::I32(buf.get_i32::<LittleEndian>())
            }
            SERIALIZE_TYPE_INT16 => {
                ensure_eof!(buf, 2);
                StorageEntry::I16(buf.get_i16::<LittleEndian>())
            }
            SERIALIZE_TYPE_INT8 => {
                ensure_eof!(buf, 1);
                StorageEntry::I8(buf.get_i8())
            }
            SERIALIZE_TYPE_UINT64 => {
                ensure_eof!(buf, 8);
                StorageEntry::U64(buf.get_u64::<LittleEndian>())
            }
            SERIALIZE_TYPE_UINT32 => {
                ensure_eof!(buf, 4);
                StorageEntry::U32(buf.get_u32::<LittleEndian>())
            }
            SERIALIZE_TYPE_UINT16 => {
                ensure_eof!(buf, 2);
                StorageEntry::U16(buf.get_u16::<LittleEndian>())
            }
            SERIALIZE_TYPE_UINT8 => {
                ensure_eof!(buf, 1);
                StorageEntry::U8(buf.get_u8())
            }
            SERIALIZE_TYPE_DOUBLE => {
                ensure_eof!(buf, 8);
                StorageEntry::Double(buf.get_f64::<LittleEndian>())
            }
            SERIALIZE_TYPE_STRING => {
                let b = read_buf::<B>(buf)?;
                StorageEntry::Buf(b)
            }
            SERIALIZE_TYPE_BOOL => {
                ensure_eof!(buf, 1);
                StorageEntry::Bool(buf.get_u8() != 0)
            }
            SERIALIZE_TYPE_OBJECT => StorageEntry::Section(Section::read::<B>(buf)?),
            SERIALIZE_TYPE_ARRAY => {
                ensure_eof!(buf, 1);
                let serialize_type = buf.get_u8();
                if serialize_type & SERIALIZE_FLAG_ARRAY != SERIALIZE_FLAG_ARRAY {
                    panic!();
                }

                let arr = Array::read::<B>(buf, serialize_type)?;
                StorageEntry::Array(arr)
            }
            _ => {
                return Err(Error::InvalidSerializeType(serialize_type));
            }
        };

        Ok(entry)
    }

    fn write(buf: &mut BytesMut, entry: &Self) {
        match *entry {
            StorageEntry::U64(ref v) => {
                buf.reserve(9);
                buf.put_u8(SERIALIZE_TYPE_UINT64);
                buf.put_u64::<LittleEndian>(*v);
            }
            StorageEntry::U32(ref v) => {
                buf.reserve(5);
                buf.put_u8(SERIALIZE_TYPE_UINT32);
                buf.put_u32::<LittleEndian>(*v);
            }
            StorageEntry::U16(ref v) => {
                buf.reserve(3);
                buf.put_u8(SERIALIZE_TYPE_UINT16);
                buf.put_u16::<LittleEndian>(*v);
            }
            StorageEntry::U8(ref v) => {
                buf.reserve(2);
                buf.put_u8(SERIALIZE_TYPE_UINT8);
                buf.put_u8(*v);
            }
            StorageEntry::I64(ref v) => {
                buf.reserve(9);
                buf.put_u8(SERIALIZE_TYPE_INT64);
                buf.put_i64::<LittleEndian>(*v);
            }
            StorageEntry::I32(ref v) => {
                buf.reserve(5);
                buf.put_u8(SERIALIZE_TYPE_INT32);
                buf.put_i32::<LittleEndian>(*v);
            }
            StorageEntry::I16(ref v) => {
                buf.reserve(3);
                buf.put_u8(SERIALIZE_TYPE_INT16);
                buf.put_i16::<LittleEndian>(*v);
            }
            StorageEntry::I8(ref v) => {
                buf.reserve(2);
                buf.put_u8(SERIALIZE_TYPE_INT8);
                buf.put_i8(*v);
            }
            StorageEntry::Double(ref v) => {
                buf.reserve(9);
                buf.put_u8(SERIALIZE_TYPE_DOUBLE);
                buf.put_f64::<LittleEndian>(*v);
            }
            StorageEntry::Bool(ref v) => {
                buf.reserve(2);
                buf.put_u8(SERIALIZE_TYPE_BOOL);
                buf.put_u8(if *v == false { 0 } else { 1 });
            }
            StorageEntry::Buf(ref v) => {
                buf.reserve(1);
                buf.put_u8(SERIALIZE_TYPE_STRING);
                write_buf(buf, v);
            }
            StorageEntry::Array(ref v) => {
                buf.reserve(1);
                buf.put_u8(SERIALIZE_TYPE_ARRAY);
                Array::write(buf, v);
            }
            StorageEntry::Section(ref v) => {
                buf.reserve(1);
                buf.put_u8(SERIALIZE_TYPE_OBJECT);
                Section::write(buf, v);
            }
        }
    }

    fn serialize_type(&self) -> u8 {
        match *self {
            StorageEntry::U64(_) => SERIALIZE_TYPE_UINT64,
            StorageEntry::U32(_) => SERIALIZE_TYPE_UINT32,
            StorageEntry::U16(_) => SERIALIZE_TYPE_UINT16,
            StorageEntry::U8(_) => SERIALIZE_TYPE_UINT8,
            StorageEntry::I64(_) => SERIALIZE_TYPE_INT64,
            StorageEntry::I32(_) => SERIALIZE_TYPE_INT32,
            StorageEntry::I16(_) => SERIALIZE_TYPE_INT16,
            StorageEntry::I8(_) => SERIALIZE_TYPE_INT8,
            StorageEntry::Double(_) => SERIALIZE_TYPE_DOUBLE,
            StorageEntry::Bool(_) => SERIALIZE_TYPE_BOOL,
            StorageEntry::Buf(_) => SERIALIZE_TYPE_STRING,
            StorageEntry::Array(_) => SERIALIZE_TYPE_ARRAY,
            StorageEntry::Section(_) => SERIALIZE_TYPE_OBJECT,
        }
    }
}

#[derive(Debug, Clone)]
pub struct Array {
    array: Vec<StorageEntry>,
    serialize_type: Option<u8>,
}

impl Array {
    pub fn new() -> Array {
        Array {
            array: Vec::new(),
            serialize_type: None,
        }
    }

    pub fn with_capacity(capacity: usize) -> Array {
        Array {
            array: Vec::with_capacity(capacity),
            serialize_type: None,
        }
    }

    pub fn len(&self) -> usize {
        self.array.len()
    }

    pub fn push(&mut self, entry: StorageEntry) -> std::result::Result<(), ()> {
        match self.serialize_type {
            Some(serialize_type) => {
                if serialize_type & SERIALIZE_FLAG_ARRAY != entry.serialize_type() {
                    return Err(());
                }
            }
            None => self.serialize_type = Some(entry.serialize_type() | SERIALIZE_FLAG_ARRAY),
        }

        self.array.push(entry);
        Ok(())
    }

    pub fn into_iter(self) -> std::vec::IntoIter<StorageEntry> {
        self.array.into_iter()
    }

    fn read<B: Buf>(buf: &mut B, mut serialize_type: u8) -> Result<Array> {
        let orig_serialize_type = serialize_type;
        if serialize_type & SERIALIZE_FLAG_ARRAY != SERIALIZE_FLAG_ARRAY {
            return Err(Error::InvalidArrayType(serialize_type));
        } else {
            serialize_type &= !SERIALIZE_FLAG_ARRAY;
        }

        let size = raw_size::read::<B>(buf)?;

        let mut array = Array {
            array: Vec::with_capacity(size),
            serialize_type: Some(orig_serialize_type),
        };
        array.array.reserve(size);

        for _ in 0..size {
            array
                .array
                .push(StorageEntry::read_entry_raw::<B>(buf, serialize_type)?);
        }

        Ok(array)
    }

    fn write(buf: &mut BytesMut, array: &Array) {
        buf.reserve(1);
        buf.put_u8(array.serialize_type.unwrap());
        raw_size::write(buf, array.array.len());
        for entry in array.array.iter() {
            StorageEntry::write(buf, &entry);
        }
    }
}

impl Index<usize> for Array {
    type Output = StorageEntry;

    fn index(&self, index: usize) -> &Self::Output {
        &self.array[index]
    }
}

#[derive(Debug, Clone)]
pub struct Section {
    pub entries: LinkedHashMap<String, StorageEntry>,
}

impl Section {
    pub fn new() -> Section {
        Section { entries: LinkedHashMap::new() }
    }

    pub fn with_capacity(capacity: usize) -> Section {
        Section { entries: LinkedHashMap::with_capacity(capacity) }
    }

    /// Insernt an storage entry.
    pub fn insert<T: Into<StorageEntry>>(&mut self, name: String, entry: T) {
        self.entries.insert(name, entry.into());
    }

    /// Length of this section.
    pub fn len(&self) -> usize {
        self.entries.len()
    }

    pub fn into_iter(self) -> linked_hash_map::IntoIter<String, StorageEntry> {
        self.entries.into_iter()
    }

    fn read<B: Buf>(buf: &mut B) -> Result<Section> {
        let mut section = Section::new();
        let count = raw_size::read::<B>(buf)?;

        section.entries.reserve(count);
        for _ in 0..count {
            let name = read_name::<B>(buf)?;
            let entry = StorageEntry::read::<B>(buf)?;
            section.entries.insert(name.clone(), entry);
        }

        Ok(section)
    }

    fn write(buf: &mut BytesMut, section: &Self) {
        raw_size::write(buf, section.entries.len());

        for (name, entry) in section.entries.iter() {

            write_name(buf, &*name);
            StorageEntry::write(buf, &entry);
        }
    }
}

impl Index<&'static str> for Section {
    type Output = StorageEntry;

    fn index(&self, index: &'static str) -> &Self::Output {
        &self.entries[index]
    }
}

pub fn read<B: Buf>(buf: &mut B) -> Result<Section> {
    header::StorageBlockHeader::read::<B>(buf)?;
    Section::read::<B>(buf)
}

pub fn write(buf: &mut BytesMut, section: &Section) {
    header::StorageBlockHeader::write(buf);
    Section::write(buf, section);
}

fn read_name<B: Buf>(buf: &mut B) -> Result<String> {
    ensure_eof!(buf, 1);
    let length = buf.get_u8() as usize;
    ensure_eof!(buf, length);

    let s = String::from_utf8_lossy(&buf.bytes()[..length]).into_owned();
    buf.advance(length);
    Ok(s)
}

fn read_buf<B: Buf>(buf: &mut B) -> Result<Vec<u8>> {
    let length = raw_size::read::<B>(buf)?;
    ensure_eof!(buf, length);

    let mut b = Vec::with_capacity(length);
    b.extend_from_slice(&buf.bytes()[..length]);
    buf.advance(length);
    Ok(b)
}

fn write_buf(buf: &mut BytesMut, b: &Vec<u8>) {
    raw_size::write(buf, b.len());

    buf.reserve(b.len());
    buf.put(b.as_slice());
}

fn write_name(buf: &mut BytesMut, name: &str) {
    buf.reserve(name.as_bytes().len() + 1);
    buf.put_u8(name.as_bytes().len() as u8);
    buf.put(name.as_bytes());
}
