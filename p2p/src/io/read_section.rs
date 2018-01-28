use std::io;
use std::mem::replace;

use bytes::ByteOrder;
use tokio_io::AsyncRead;
use futures::{Future, Poll};

use io::{
    ReadStorageEntry,
    ReadRawSize,
    ReadName,

    read_storage_entry,
    read_raw_size,
    read_name,
};
use portable_storage::{Section, StorageEntry};

/// Read a portable storage section.
pub fn read_section<A, E>(a: A) -> ReadSection<A, E>
    where A: AsyncRead,
          E: ByteOrder,
{
    ReadSection {
        section: Section::new(),
        state: ReadSectionState::ReadLength {
            future: read_raw_size(a),
        },
    }
}

enum ReadSectionState<A, E> {
    /// Read the length
    ReadLength {
        future: ReadRawSize<A, E>
    },
    /// Read the actual entry.
    ReadEntry {
        count: usize,
        future: ReadNamedEntry<A, E>
    },
}

pub struct ReadSection<A, E> {
    section: Section,
    state: ReadSectionState<A, E>,
}

impl<A, E> Future for ReadSection<A, E>
    where A: AsyncRead,
          E: ByteOrder,
{
    type Item = (A, Section);
    type Error = io::Error;

	fn poll(&mut self) -> Poll<Self::Item, Self::Error> {
        loop {
            let next_state = match self.state {
                ReadSectionState::ReadLength { ref mut future } => {
                    let (stream, length) = try_ready!(future.poll());
                    
                    ReadSectionState::ReadEntry {
                        count: length,
                        future: read_named_entry(stream),
                    }
                },
                ReadSectionState::ReadEntry { ref count, ref mut future} => {
                    let (stream, (name, entry)) = try_ready!(future.poll());

                    self.section.insert(name, entry);
                    let count = *count - 1;

                    if count == 0 {
                        return Ok((stream, replace(&mut self.section, Section::new())).into());
                    }

                    ReadSectionState::ReadEntry {
                        count,
                        future: read_named_entry(stream),
                    }
                }
            };
            self.state = next_state;
        }
	}
}

/// Read a named entry
fn read_named_entry<A, E>(a: A) -> ReadNamedEntry<A, E>
    where A: AsyncRead,
          E: ByteOrder,
{
    ReadNamedEntry {
        state: ReadNamedEntryState::ReadName {
            future: read_name(a),
        },
    }
}

enum ReadNamedEntryState<A, E> {
    /// Read the entry name.
    ReadName {
        future: ReadName<A>
    },
    /// Read the actual entry.
    ReadEntry {
        name: String,
        future: Box<ReadStorageEntry<A, E>>,
    },
}

struct ReadNamedEntry<A, E> {
    state: ReadNamedEntryState<A, E>,
}

impl<A, E> Future for ReadNamedEntry<A, E>
    where A: AsyncRead,
          E: ByteOrder,
{
    type Item = (A, (String, StorageEntry));
    type Error = io::Error;

	fn poll(&mut self) -> Poll<Self::Item, Self::Error> {
        loop {
            let next_state = match self.state {
                ReadNamedEntryState::ReadName { ref mut future } => {
                    let (stream, name) = try_ready!(future.poll());
                    ReadNamedEntryState::ReadEntry {
                        name,
                        future: Box::new(read_storage_entry(stream)),
                    }
                },
                ReadNamedEntryState::ReadEntry { ref mut name, ref mut future} => {
                    let (stream, entry) = try_ready!(future.poll());
                    return Ok((stream, (replace(name, String::new()), entry)).into());
                }
            };
            self.state = next_state;
        }
	}
}
