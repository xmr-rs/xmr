use std::io;

use bytes::ByteOrder;
use tokio_io::AsyncRead;
use futures::{Future, Poll};

use io::{
    ReadStorageEntryRaw,
    ReadSerializeType,
    ReadArray,

    read_storage_entry_raw,
    read_serialize_type,
    read_array,
};
use portable_storage::{SERIALIZE_FLAG_ARRAY, StorageEntry, Array};

/// Read a storage entry.
pub fn read_storage_entry<A, E>(a: A) -> ReadStorageEntry<A, E>
    where A: AsyncRead,
          E: ByteOrder,
{
    ReadStorageEntry {
        state: ReadStorageEntryState::ReadSerializeType {
            future: read_serialize_type(a),
        },
    }
}

pub enum ReadStorageEntryState<A, E> {
    /// Read the serialize type
    ReadSerializeType {
        future: ReadSerializeType<A>
    },
    /// Read the actual entry.
    ReadEntry {
        future: ReadStorageEntryRaw<A, E>
    },
    /// Or... read an array.
    ReadArray {
        future: ReadArray<A, E>,
    }
}

pub struct ReadStorageEntry<A, E> {
    state: ReadStorageEntryState<A, E>,
}

impl<A, E> Future for ReadStorageEntry<A, E>
    where A: AsyncRead,
          E: ByteOrder,
{
    type Item = (A, StorageEntry);
    type Error = io::Error;

	fn poll(&mut self) -> Poll<Self::Item, Self::Error> {
        loop {
            let next_state = match self.state {
                ReadStorageEntryState::ReadSerializeType { ref mut future } => {
                    let (stream, serialize_type) = try_ready!(future.poll());
                    if serialize_type & SERIALIZE_FLAG_ARRAY == SERIALIZE_FLAG_ARRAY {
                        ReadStorageEntryState::ReadArray {
                            future: read_array(stream, serialize_type),
                        }
                    } else {
                        ReadStorageEntryState::ReadEntry {
                            future: read_storage_entry_raw(stream, serialize_type),
                        }
                    }
                },
                ReadStorageEntryState::ReadEntry { ref mut future} => {
                    let (stream, entry) = try_ready!(future.poll());
                    return Ok((stream, entry).into());
                }
                ReadStorageEntryState::ReadArray { ref mut future } => {
                    let (stream, data) = try_ready!(future.poll());
                    return Ok((stream, StorageEntry::Array(Array(data.0, data.1))).into());
                }
            };
            self.state = next_state;
        }
	}
}
