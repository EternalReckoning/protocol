#[cfg(not(feature = "bitflags"))]
pub type Field = u8;

#[cfg(feature = "bitflags")]
bitflags::bitflags! {
    pub struct Field: u8 {
        const TYPE = 0b1100_0000;
        const ID = 0b0011_1111;
    }
}

#[repr(u8)]
pub enum FieldType {
    Bytes = 0xC0,
}