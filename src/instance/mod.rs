use super::kvm;
pub mod device;
use kvm_sys as sys;
use error::*;

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub enum PortDirection {
    In = sys::KVM_EXIT_IO_IN as isize,
    Out = sys::KVM_EXIT_IO_OUT as isize,
}

pub struct Instance {
        machine: kvm::Machine,
        cores: Vec<kvm::Core>,
        devices: Vec<Box<device::Device>>
}

impl Instance {
    pub fn add(&mut self, mut device: Box<device::Device>) -> Result<()> {
        device.initialize(self)?;
        self.devices.push(device);
        Ok(())
    }
}
