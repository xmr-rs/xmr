use std::io;

use bytes::ByteOrder;
use tokio_io::AsyncRead;
use tokio_io::io::{ReadExact, read_exact};
use futures::{Future, Poll};

use portable_storage::{
    SERIALIZE_TYPE_INT64,
    SERIALIZE_TYPE_INT32,
    SERIALIZE_TYPE_INT16,
    SERIALIZE_TYPE_INT8,
    SERIALIZE_TYPE_UINT64,
    SERIALIZE_TYPE_UINT32,
    SERIALIZE_TYPE_UINT16,
    SERIALIZE_TYPE_UINT8,
    SERIALIZE_TYPE_DOUBLE,
    SERIALIZE_TYPE_STRING,
    SERIALIZE_TYPE_BOOL,
    SERIALIZE_TYPE_OBJECT,
    SERIALIZE_TYPE_ARRAY,
    StorageEntry,
    Array,
};

use io::{
    ReadBuf,
    ReadArray,
    ReadSection,
    ReadSerializeType,

    read_buf,
    read_array,
    read_section,
    read_serialize_type,
};

// TODO: type links in docs
/// Read a `StorageEntry`given a `serialize_type` specifier.
///
/// This intended to be used by high level futures such as [`ReadStorageEntry`][readentry].
///
/// [readentry]: struct.ReadStorageEntry.html
pub fn read_storage_entry_raw<A, E>(a: A, serialize_type: u8) -> ReadStorageEntryRaw<A, E>
    where A: AsyncRead,
          E: ByteOrder,
{
    let state = match serialize_type {
        SERIALIZE_TYPE_INT64 => {
            ReadStorageEntryRawState::I64 { reader: read_exact(a, [0u8; 8]) }
        },
        SERIALIZE_TYPE_INT32 => {
            ReadStorageEntryRawState::I32 { reader: read_exact(a, [0u8; 4]) }
        },
        SERIALIZE_TYPE_INT16 => {
            ReadStorageEntryRawState::I16 { reader: read_exact(a, [0u8; 2]) }
        },
        SERIALIZE_TYPE_INT8 => {
            ReadStorageEntryRawState::I8 { reader: read_exact(a, [0u8; 1]) }
        },
        SERIALIZE_TYPE_UINT64 => {
            ReadStorageEntryRawState::U64 { reader: read_exact(a, [0u8; 8]) }
        },
        SERIALIZE_TYPE_UINT32 => {
            ReadStorageEntryRawState::U32 { reader: read_exact(a, [0u8; 4]) }
        },
        SERIALIZE_TYPE_UINT16 => {
            ReadStorageEntryRawState::U16 { reader: read_exact(a, [0u8; 2]) }
        },
        SERIALIZE_TYPE_UINT8 => {
            ReadStorageEntryRawState::U8 { reader: read_exact(a, [0u8; 1]) }
        },
        SERIALIZE_TYPE_DOUBLE => { 
            ReadStorageEntryRawState::Double { reader: read_exact(a, [0u8; 8]) }
        },
        SERIALIZE_TYPE_STRING => {
            ReadStorageEntryRawState::Buf { future: read_buf(a) }
        }
        SERIALIZE_TYPE_BOOL => {
            ReadStorageEntryRawState::Bool { reader: read_exact(a, [0u8; 1]) }
        },
        SERIALIZE_TYPE_OBJECT => {
          ReadStorageEntryRawState::Section { future: read_section(a) }
        }
        SERIALIZE_TYPE_ARRAY => {
          ReadStorageEntryRawState::Array {
              state: ReadArrayState::ReadSerializeType {
                  future: read_serialize_type(a),
              }
          }
        }
        _ => unreachable!(),
    };

    ReadStorageEntryRaw {
        state: state,
    }
}

pub enum ReadArrayState<A, E> {
    ReadSerializeType { future: ReadSerializeType<A> },
    ReadArray { future: Box<ReadArray<A, E>> },
}

pub enum ReadStorageEntryRawState<A, E> {
    I64 { reader: ReadExact<A, [u8; 8]> },
    I32 { reader: ReadExact<A, [u8; 4]> },
    I16 { reader: ReadExact<A, [u8; 2]> },
    I8  { reader: ReadExact<A, [u8; 1]> },
    U64 { reader: ReadExact<A, [u8; 8]> },
    U32 { reader: ReadExact<A, [u8; 4]> },
    U16 { reader: ReadExact<A, [u8; 2]> },
    U8  { reader: ReadExact<A, [u8; 1]> },
    Double { reader: ReadExact<A, [u8; 8]> },
    Buf { future: ReadBuf<A, E> },
    Bool { reader: ReadExact<A, [u8; 1]> },
    Section { future: ReadSection<A, E> },
    Array { state: ReadArrayState<A, E> },
}

pub struct ReadStorageEntryRaw<A, E> {
    state: ReadStorageEntryRawState<A, E>,
}

impl<A, E> Future for ReadStorageEntryRaw<A, E>
    where A: AsyncRead,
          E: ByteOrder,
{
    type Item = (A, StorageEntry);
    type Error = io::Error;

	fn poll(&mut self) -> Poll<Self::Item, Self::Error> {
        match self.state {
            ReadStorageEntryRawState::I64 { ref mut reader } => {
                let (stream, data) = try_ready!(reader.poll());
                return Ok((stream, StorageEntry::I64(E::read_i64(&data))).into());
            },
            ReadStorageEntryRawState::I32 { ref mut reader } => {
                let (stream, data) = try_ready!(reader.poll());
                return Ok((stream, StorageEntry::I32(E::read_i32(&data))).into());
            },
            ReadStorageEntryRawState::I16 { ref mut reader } => {
                let (stream, data) = try_ready!(reader.poll());
                return Ok((stream, StorageEntry::I16(E::read_i16(&data))).into());
            },
            ReadStorageEntryRawState::I8  { ref mut reader } => {
                let (stream, data) = try_ready!(reader.poll());
                return Ok((stream, StorageEntry::I8(data[0] as i8)).into());
            },
            ReadStorageEntryRawState::U64 { ref mut reader } => {
                let (stream, data) = try_ready!(reader.poll());
                return Ok((stream, StorageEntry::U64(E::read_u64(&data))).into());
            },
            ReadStorageEntryRawState::U32 { ref mut reader } => {
                let (stream, data) = try_ready!(reader.poll());
                return Ok((stream, StorageEntry::U32(E::read_u32(&data))).into());
            },
            ReadStorageEntryRawState::U16 { ref mut reader } => {
                let (stream, data) = try_ready!(reader.poll());
                return Ok((stream, StorageEntry::U16(E::read_u16(&data))).into());
            },
            ReadStorageEntryRawState::U8  { ref mut reader } => {
                let (stream, data) = try_ready!(reader.poll());
                return Ok((stream, StorageEntry::U8(data[0])).into());
            },
            ReadStorageEntryRawState::Double { ref mut reader } => {
                let (stream, data) = try_ready!(reader.poll());
                return Ok((stream, StorageEntry::Double(E::read_f64(&data))).into());
            },
            ReadStorageEntryRawState::Buf { ref mut future } => {
                let (stream, buf) = try_ready!(future.poll());
                return Ok((stream, StorageEntry::Buf(buf)).into());
            },
            ReadStorageEntryRawState::Bool { ref mut reader } => {
                let (stream, data) = try_ready!(reader.poll());
                return Ok((stream, StorageEntry::Bool(data[0] != 0)).into());
            },
            ReadStorageEntryRawState::Section { ref mut future } => {
                let (stream, section) = try_ready!(future.poll());
                return Ok((stream, StorageEntry::Section(section)).into());
            },
            ReadStorageEntryRawState::Array { ref mut state } => {
                loop {
                    let next_state = match state {
                        &mut ReadArrayState::ReadSerializeType { ref mut future } => {
                            let (stream, serialize_type) = try_ready!(future.poll());
                            ReadArrayState::ReadArray {
                                future: Box::new(read_array(stream, serialize_type)),
                            }
                        },
                        &mut ReadArrayState::ReadArray { ref mut future } => {
                            let (stream, data) = try_ready!(future.poll());
                            return Ok((stream, (StorageEntry::Array(Array(data.0, data.1)))).into());
                        },
                    };
                    *state = next_state;
                }
            },
        };
	}
}
