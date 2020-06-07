use bytes::{Buf, BufMut};

#[cfg(test)]
use bytes::BytesMut;

use crate::Error;

pub static MAX_SIZE: usize = 512;
const PACKET_MAGIC: u16 = 0xEC_AB;

#[repr(C)]
pub struct Header {
    magic: u16,
    pub size: u16,
    pub msg_id: u16,
    pub major_ver: u8,
    pub minor_ver: u8,
}

#[cfg(not(feature = "bitflags"))]
type MsgId = u16;

#[cfg(feature = "bitflags")]
bitflags::bitflags! {
    pub struct MsgId: u16 {
        const CACHE_GLOBAL = 0x80_00;
        const CACHE_TEMPORARY = 0x70_00;
        const CACHE = Self::CACHE_GLOBAL.bits | Self::CACHE_TEMPORARY.bits;
        const REQUEST = 0x00_01;
        const TYPE = 0xFF_FF ^ Self::CACHE.bits;
    }
}

impl Header {
    pub fn new(size: u16, msg_id: u16, major_ver: u8, minor_ver: u8) -> Header {
        assert!((size as usize) < MAX_SIZE);
        Header {
            magic: PACKET_MAGIC,
            size, msg_id,
            major_ver, minor_ver,
        }
    }

    pub fn read_from<T: Buf>(buf: &mut T) -> Result<Header, Error> {
        if buf.remaining() < std::mem::size_of::<Header>() {
            return Err(Error::BufferTooShort);
        }

        let mut header = std::mem::MaybeUninit::<Header>::uninit();
        let header = unsafe {
            let mut bytes = std::slice::from_raw_parts_mut(
                header.as_mut_ptr() as *mut u8,
                std::mem::size_of::<Header>(),
            );
            buf.copy_to_slice(&mut bytes);

            header.assume_init()
        };

        if header.magic != PACKET_MAGIC {
            return Err(Error::InvalidHeader);
        }
        if header.size as usize > MAX_SIZE {
            return Err(Error::InvalidHeader);
        }

        Ok(header)
    }

    pub fn write_to<T: BufMut>(&self, buf: &mut T) -> Result<(), Error> {
        if buf.remaining_mut() < std::mem::size_of::<Header>() {
            return Err(Error::BufferTooShort);
        }

        unsafe {
            let bytes = std::slice::from_raw_parts(
                (self as *const Header) as *const u8,
                std::mem::size_of::<Header>(),
            );
            buf.put_slice(bytes);
        }
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_read_from() {
        let mut buf = BytesMut::with_capacity(std::mem::size_of::<Header>() + 8);
        buf.put_u16(0xEC_AB);
        buf.put_u16(8);
        buf.put_u16(0x70_10);
        buf.put_u8(0x01);
        buf.put_u8(0x00);

        buf.put_slice(&b"\x11\x22\x33\x44\x55\x66\x77\x88"[..]);

        let header = Header::read_from(&mut buf)
            .expect("header should be valid");

        assert_eq!(header.size, 8);
        assert_eq!(header.msg_id, 0x70_10);
        assert_eq!(header.major_ver, 0x01);
        assert_eq!(header.minor_ver, 0x00);

        assert_eq!(buf.remaining(), 8);
    }
}