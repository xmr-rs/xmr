use bytes::{Buf, BufMut, BytesMut, LittleEndian};
use errors::Result;

pub const PORTABLE_RAW_SIZE_MARK_MASK: u8 = 0x03;
pub const PORTABLE_RAW_SIZE_MARK_BYTE: u8 = 0;
pub const PORTABLE_RAW_SIZE_MARK_WORD: u8 = 1;
pub const PORTABLE_RAW_SIZE_MARK_DWORD: u8 = 2;
pub const PORTABLE_RAW_SIZE_MARK_INT64: u8 = 3;

pub fn read<B: Buf>(buf: &mut B) -> Result<usize> {
    ensure_eob!(buf, 1);
    let mark = buf.bytes()[0] & PORTABLE_RAW_SIZE_MARK_MASK;

    match mark {
        PORTABLE_RAW_SIZE_MARK_BYTE => {
            Ok((buf.get_u8() >> 2) as usize)
        }
        PORTABLE_RAW_SIZE_MARK_WORD => {
            ensure_eob!(buf, 2);
            Ok((buf.get_u16::<LittleEndian>() >> 2) as usize)
        }
        PORTABLE_RAW_SIZE_MARK_DWORD => {
            ensure_eob!(buf, 4);
            Ok((buf.get_u32::<LittleEndian>() >> 2) as usize)
        }
        PORTABLE_RAW_SIZE_MARK_INT64 => {
            ensure_eob!(buf, 8);
            Ok((buf.get_u64::<LittleEndian>() >> 2) as usize)
        }
        _ => unreachable!(),
    }
}

pub fn write(buf: &mut BytesMut, val: usize) {
    if val <= 63 {
        buf.reserve(1);
        buf.put_u8(((val as u8) << 2) | PORTABLE_RAW_SIZE_MARK_BYTE);
    } else if val <= 16383 {
        buf.reserve(2);
        buf.put_u16::<LittleEndian>(((val as u16) << 2) | PORTABLE_RAW_SIZE_MARK_WORD as u16);
    } else if val <= 1073741823 {
        buf.reserve(4);
        buf.put_u32::<LittleEndian>(((val as u32) << 2) | PORTABLE_RAW_SIZE_MARK_DWORD as u32);
    } else if val as u64 <= 4611686018427387903 {
        buf.reserve(8);
        buf.put_u64::<LittleEndian>(((val as u64) << 2) | PORTABLE_RAW_SIZE_MARK_INT64 as u64);
    } else {
        /// XXX: Hope some day monero never uses a value too large.
        panic!("too large");
    }
}
