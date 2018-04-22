#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub struct Address(pub u8, pub u8, pub u8, pub u8);

impl From<u32> for Address {
    fn from(value: u32) -> Address {
        let bus = (value >> 16) & 0b1111111;
        let device = (value >> 11) & 0b1111;
        let function = (value >> 8) & 0b11;
        let register = value & 0b111111;
        Address(bus as u8, device as u8, function as u8, register as u8)
    }
}
