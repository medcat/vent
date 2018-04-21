use std::os::unix::io::{RawFd, AsRawFd};
use std::fs::File;
use std::result::Result as StdResult;
use nix;
use kvm_sys as sys;
use super::{Core, CheckCapability, Capability};
use error::*;

pub use kvm_sys::UserspaceMemoryRegion as SysUserspaceMemoryRegion;

#[derive(Debug)]
pub struct Machine(RawFd, Vec<UserspaceMemoryRegion>);

#[derive(Debug, Clone)]
pub struct UserspaceMemoryRegion(SysUserspaceMemoryRegion);

impl UserspaceMemoryRegion {
    pub fn file(file: &File, slot: u32, addr: u64) -> Result<UserspaceMemoryRegion> {
        use nix::sys::mman;
        let meta = file.metadata().chain_err(|| ErrorKind::MemoryMapError)?;
        let len = meta.len();

        let data = unsafe {
            let protflag = mman::ProtFlags::PROT_READ | mman::ProtFlags::PROT_WRITE;
            let mapflag = mman::MapFlags::MAP_FILE | mman::MapFlags::MAP_SHARED;
            mman::mmap(0 as *mut nix::libc::c_void, len as usize, protflag, mapflag, file.as_raw_fd(), 0)
                .chain_err(|| ErrorKind::MemoryMapError)?
        };

        Ok(UserspaceMemoryRegion(SysUserspaceMemoryRegion {
            slot, flags: 0,
            guest_phys_addr: addr as u64,
            /// in bytes.
            memory_size: len,
            /// the start of th userspace allocated memory.
            userspace_addr: data as u64
        }))
    }
}

impl Drop for UserspaceMemoryRegion {
    fn drop(&mut self) {
        use nix::sys::mman;
        unsafe {
            mman::munmap(self.0.userspace_addr as *mut nix::libc::c_void, self.0.memory_size as usize).unwrap();
        }
    }
}

fn chain_then(res: StdResult<i32, nix::Error>) -> Result<()> {
    res
        .chain_err(|| ErrorKind::KvmMachineOperationError)
        .and_then(|v| if v == 0 { Ok(())} else { Err(ErrorKind::KvmMachineOperationError.into())} )
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
