use super::instance::device;

mod configuration;
use self::configuration::Address;
mod host;
mod invalid;

pub trait Pci {
    fn config_read(&mut self, address: Address, pointer: &mut u32) -> device::Action;
    fn config_write(&mut self, address: Address, pointer: &mut u32) -> device::Action;
}
