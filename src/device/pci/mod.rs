use super::Device;

mod address;
mod host;

pub use self::address::Address;
pub use self::host::Host;

pub trait Pci: Device {
    fn config_read(&self, address: Address) -> Option<u32>;
    // fn config_space(&self) -> &[u8];
    fn config_write(&self, address: Address, value: u32) -> Option<()>;
}
