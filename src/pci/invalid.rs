use super::{Pci, Address};
use instance::device;

pub struct Invalid;

impl Pci for Invalid {
    fn config_read(&mut self, _address: Address, _pointer: &mut u32) -> device::Action {
        device::Action::Stop
    }

    fn config_write(&mut self, _address: Address, _pointer: &mut u32) -> device::Action {
        device::Action::Stop
    }
}
