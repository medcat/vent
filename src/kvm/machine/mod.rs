use std::os::unix::io::RawFd;
use std::result::Result as StdResult;
use nix;
use super::{sys, Core, ErrorKind, Result, ResultExt, CheckCapability, Capability};

#[derive(Debug)]
pub struct Machine(pub(super) RawFd);

fn chain_then(res: StdResult<i32, nix::Error>) -> Result<()> {
    res
        .chain_err(|| ErrorKind::KvmMachineOperationError)
        .and_then(|v| if v == 0 { Ok(())} else { Err(ErrorKind::KvmMachineOperationError.into())} )
}

impl Machine {
    pub fn create_core(&mut self, id: i32) -> Result<Core> {
        unsafe { sys::kvm_create_vcpu(self.0, id).map(|v| Core(v)) }
            .chain_err(|| ErrorKind::KvmMachineOperationError)
    }

    pub fn create_irq(&mut self) -> Result<()> {
        chain_then(unsafe { sys::kvm_create_irqchip(self.0) })
    }

    pub fn get_clock(&mut self, stable: bool) -> Result<u64> {
        unsafe {
            let flags = if stable { sys::KVM_CLOCK_TSC_STABLE } else { 0 };
            let mut clock = sys::ClockData { clock: 0, flags: flags, _pad: [0; 9] };
            chain_then(sys::kvm_get_clock(self.0, &mut clock as *mut sys::ClockData))?;
            Ok(clock.clock)
        }
    }

    pub fn set_clock(&mut self, clock: u64) -> Result<()> {
        unsafe {
            let clock = sys::ClockData { clock: clock, flags: 0, _pad: [0; 9] };
            chain_then(sys::kvm_set_clock(self.0, &clock as *const sys::ClockData))
        }
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
