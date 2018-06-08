use super::error::*;
use super::machine::Machine;
use kvm;
use std::fmt::Debug;
use std::sync::Arc;

pub mod cmos;
pub mod debug;
pub mod pci;
pub mod virtio;

pub trait Device: Debug + Send + Sync {
    fn request(&self) -> Vec<kvm::core::IoAddress>;
    fn handle(&self, io: kvm::core::IoAction, memory: &mut [u8]) -> Option<()>;
}

impl<T: Device> Device for Box<T> {
    fn request(&self) -> Vec<kvm::core::IoAddress> {
        self.as_ref().request()
    }

    fn handle(&self, io: kvm::core::IoAction, memory: &mut [u8]) -> Option<()> {
        self.as_ref().handle(io, memory)
    }
}

pub(crate) fn prepare(machine: &mut Machine) -> Result<()> {
    for device in debug::default().into_iter() {
        machine.push(device)?;
    }

    machine.push(Arc::new(cmos::Cmos::new()))?;

    machine.push(Arc::new(pci::Host::new(
        None,
        Some(Arc::new(virtio::Console::new()) as Arc<pci::Pci>),
    )))?;

    Ok(())
}
