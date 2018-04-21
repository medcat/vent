use std::fs::File;
use super::kvm;
use error::*;

use super::device::Device;
pub use super::kvm::core::{Run, Exit};

pub struct Instance {
    machine: kvm::Machine,
    cores: Vec<kvm::Core>,
    devices: Vec<Box<Device>>
}

impl Instance {
    pub fn new(system: &mut kvm::System, bios: &File) -> Result<Instance> {
        let mut machine = system.create_machine()?;
        machine.create_irq()?;
        machine.set_identity_map(0)?;
        let region = kvm::UserspaceMemoryRegion::file(bios, 0, 0xFFFFFFF0)?;
        machine.push(region)?;
        Ok(Instance { machine, cores: vec![], devices: vec![] })
    }

    pub fn push(&mut self, device: Box<Device>) -> Result<()> {
        self.devices.push(device);
        Ok(())
    }

    pub fn create_cores(&mut self, num: usize) -> Result<()> {
        let start = self.cores.len();
        for i in 0..num {
            let mut core = self.machine.create_core((start + i) as i32)?;
            core.mmap()?;
            core.set_tss(0xfffbd000)?;
            self.cores.push(self.machine.create_core((start + i) as i32)?);
        }

        Ok(())
    }

    pub fn primary_core(&mut self) -> Result<&mut kvm::Core> {
        self.cores.first_mut().ok_or_else(|| ErrorKind::KvmMachineMissingPrimaryCoreError.into())
    }
}
