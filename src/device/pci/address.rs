#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub struct Address(pub u8, pub u8, pub u8, pub u8);

impl Address {
    pub fn from(value: u32) -> Option<Address> {
        let enabled = value >> 31;
        let bus = (value >> 16) & 0b1111111;
        let device = (value >> 11) & 0b1111;
        let function = (value >> 8) & 0b11;
        let register = value & 0b111111;
        if enabled > 0 {
            Some(Address(
                bus as u8,
                device as u8,
                function as u8,
                register as u8,
            ))
        } else {
            None
        }
    }

    pub fn bridge(&self) -> u8 {
        self.0
    }

    pub fn device(&self) -> u8 {
        self.1
    }

    // pub fn function(&self) -> u8 {
    //     self.2
    // }

    pub fn register(&self) -> u8 {
        self.3
    }
}
