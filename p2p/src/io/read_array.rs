use std::io;
use std::mem::replace;

use bytes::ByteOrder;
use tokio_io::AsyncRead;
use futures::{Future, Poll};

use portable_storage::{SERIALIZE_FLAG_ARRAY, StorageEntry};
use io::{
    ReadRawSize,
    ReadStorageEntryRaw,

    read_raw_size,
    read_storage_entry_raw
};

/// Read a portable storage array.
pub fn read_array<A, E>(a: A, mut serialize_type: u8) -> ReadArray<A, E>
    where A: AsyncRead,
          E: ByteOrder,
{
    let orig_serialize_type = serialize_type;
    if serialize_type & SERIALIZE_FLAG_ARRAY != SERIALIZE_FLAG_ARRAY {
        panic!();
    } else {
        serialize_type &= !SERIALIZE_FLAG_ARRAY;
    }

    ReadArray {
        serialize_type,
        orig_serialize_type,
        array: Vec::new(),
        state: ReadArrayState::ReadLength {
            future: read_raw_size(a),
        }
    }
}

enum ReadArrayState<A, E> {
    ReadLength {
        future: ReadRawSize<A, E>,
    },
    ReadArrayContents {
        future: ReadStorageEntryRaw<A, E>,
        count: usize,
    }
}

pub struct ReadArray<A, E> {
    serialize_type: u8,
    orig_serialize_type: u8,
    array: Vec<StorageEntry>,
    state: ReadArrayState<A, E>,
}

impl<A, E> Future for ReadArray<A, E>
    where A: AsyncRead,
          E: ByteOrder,
{
    type Item = (A, (Vec<StorageEntry>, u8));
    type Error = io::Error;

	fn poll(&mut self) -> Poll<Self::Item, Self::Error> {
        loop {
            let next_state = match self.state {
                ReadArrayState::ReadLength { ref mut future } => {
                    let (stream, length) = try_ready!(future.poll());
                    self.array.reserve(length);
                    ReadArrayState::ReadArrayContents {
                        future: read_storage_entry_raw(stream, self.serialize_type),
                        count: length,
                    }
                },
                ReadArrayState::ReadArrayContents { ref mut future, ref count } => {
                    let (stream, entry) = try_ready!(future.poll());

                    self.array.push(entry);
                    let count = *count - 1;

                    if count == 0 {
                        return Ok((stream, (replace(&mut self.array, Vec::new()), self.serialize_type)).into());
                    }

                    ReadArrayState::ReadArrayContents {
                        future: read_storage_entry_raw(stream, self.orig_serialize_type),
                        count,
                    }
                }
            };
            self.state = next_state;
        }
	}
}
