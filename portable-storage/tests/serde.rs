#[macro_use]
extern crate portable_storage;
extern crate bytes;

use portable_storage::{Serialize, Deserialize};

#[derive(Default, Eq, PartialEq)]
pub struct VersionCommand {
    pub version: u8,
}

serializable! {
    VersionCommand { version, }
}

#[test]
fn serde() {
    let orig = VersionCommand {
        version: 0xFE,
    };

    let section = orig.serialize();

    let deserialized = VersionCommand::deserialize(&section).unwrap();

    assert!(orig == deserialized);
}

#[test]
fn serde_buf() {
    use bytes::{BytesMut, IntoBuf, LittleEndian};

    let orig = VersionCommand {
        version: 0xFE,
    };

    let section = orig.serialize();
    let mut buf = BytesMut::new();
    portable_storage::write::<LittleEndian>(&mut buf, &section);

    let mut buf = buf.into_buf();
    buf.set_position(0);
    let section = portable_storage::read::<LittleEndian, _>(&mut buf).unwrap();
    let deserialized = VersionCommand::deserialize(&section).unwrap();

    assert!(orig == deserialized);
}
