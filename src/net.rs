#[cfg(not(feature = "bitflags"))]
type ServiceId = u16;

#[cfg(feature = "bitflags")]
bitflags::bitflags! {
    pub struct ServiceId: u16 {
        const SERVER = 0x80_00;
        const TYPE = 0xFF_FF ^ Self::SERVER.bits;
    }
}
