use std::os::unix::io::RawFd;
use std::result::Result as StdResult;
use nix;
use kvm_sys as sys;
use super::{Core, CheckCapability, Capability};
use error::*;

mod ioeventfd;
mod memory;
pub use self::ioeventfd::*;
pub use self::memory::*;

#[derive(Debug)]
pub struct Machine(RawFd, Vec<UserspaceMemoryRegion>);

fn chain_then(res: StdResult<i32, nix::Error>) -> Result<()> {
    res
        .chain_err(|| ErrorKind::KvmMachineOperationError)
        .and_then(|v| if v == 0 { Ok(()) } else { Err(ErrorKind::KvmMachineOperationError.into()) } )
}

impl Machine {
    pub fn new(fd: RawFd) -> Machine {
        Machine(fd, vec![])
    }

    pub fn push(&mut self, region: UserspaceMemoryRegion) -> Result<()> {
        self.1.push(region);
        Ok(())
    }

    pub fn create_core(&mut self, id: i32) -> Result<Core> {
        unsafe { sys::kvm_create_vcpu(self.0, id).map(|v| Core(v, None)) }
            .chain_err(|| ErrorKind::KvmMachineOperationError)
    }

    pub fn create_irq(&mut self) -> Result<()> {
        self.assert_capability(Capability::IrqChip)?;
        chain_then(unsafe { sys::kvm_create_irqchip(self.0) })
    }

    pub fn set_identity_map(&mut self, addr: usize) -> Result<()> {
        unsafe {
            chain_then(sys::kvm_set_identity_map_addr(self.0, addr as i32)).map(|_| ())
        }
    }

    pub fn get_clock(&mut self, stable: bool) -> Result<u64> {
        self.assert_capability(Capability::AdjustClock)?;
        unsafe {
            let flags = if stable { sys::KVM_CLOCK_TSC_STABLE } else { 0 };
            let mut clock = sys::ClockData { clock: 0, flags: flags, _pad: [0; 9] };
            chain_then(sys::kvm_get_clock(self.0, &mut clock as *mut sys::ClockData))?;
            Ok(clock.clock)
        }
    }

    pub fn set_clock(&mut self, clock: u64) -> Result<()> {
        self.assert_capability(Capability::AdjustClock)?;
        unsafe {
            let clock = sys::ClockData { clock: clock, flags: 0, _pad: [0; 9] };
            chain_then(sys::kvm_set_clock(self.0, &clock as *const sys::ClockData))
        }
    }

    pub fn modify_memory_region(&mut self, region: &UserspaceMemoryRegion) -> Result<()> {
        self.assert_capability(Capability::UserspaceMemory)?;
        unsafe {
            chain_then(sys::kvm_set_user_memory_region(self.0, &region.0 as *const SysUserspaceMemoryRegion))
                .map(|_| ())
        }
    }

    pub fn io<'a>(&'a mut self, address: IoAddress, length: u32) -> Result<IoEventFd<'a>> {
        self.assert_capability(Capability::IoEventFd)?;
        IoEventFd::new(&*self, address, length).chain_err(|| ErrorKind::KvmCoreOperationError)
    }
}

impl Drop for Machine {
    fn drop(&mut self) {
        nix::unistd::close(self.0).unwrap()
    }
}

impl CheckCapability for Machine {
    fn check_capability(&mut self, cap: Capability) -> Result<i32> {
        unsafe { sys::kvm_check_extension(self.0, cap.into()) }
            .chain_err(|| ErrorKind::KvmCapabilityCheckError)
    }
}

unsafe impl Send for Machine {}
