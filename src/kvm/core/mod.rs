use std::os::unix::io::RawFd;
use std::result::Result as StdResult;
use nix;
use kvm_sys as sys;
use super::{CheckCapability, Capability};
use error::*;

mod run;
mod state;

pub use self::run::{Run, Exit};
pub use self::state::*;

#[derive(Debug)]
pub struct Core(pub(super) RawFd, pub(super) Option<*mut sys::Run>);

fn chain_then(res: StdResult<i32, nix::Error>) -> Result<i32> {
    res
        .chain_err(|| ErrorKind::KvmCoreOperationError)
        .and_then(|v| if v >= 0 { Ok(v) } else { Err(ErrorKind::KvmCoreOperationError.into()) })
}

impl Core {
    pub fn run(&mut self, run: Run) -> Result<Run> {
        let runp = self.1.ok_or_else(|| -> Error { ErrorKind::KvmCoreUninitializedError.into() })?;

        unsafe {
            *runp = run.into();
            let _ = sys::kvm_run(self.0).chain_err(|| ErrorKind::KvmCoreOperationError)?;
            Run::from(*runp)
        }
    }

    pub fn get_state(&mut self) -> Result<State> {
        self.assert_capability(Capability::MpState)?;
        unsafe {
            let mut state = sys::MpState { mp_state: sys::KVM_MP_STATE_RUNNABLE };
            chain_then(sys::kvm_get_mp_state(self.0, &mut state as *mut sys::MpState))?;
            Ok(state.mp_state.into())
        }
    }

    pub fn set_state(&mut self, statev: State) -> Result<()> {
        self.assert_capability(Capability::MpState)?;
        unsafe {
            let state = sys::MpState { mp_state: statev.into() };
            chain_then(sys::kvm_set_mp_state(self.0, &state as *const sys::MpState)).map(|_| ())
        }
    }

    pub fn get_mmap_size(&mut self) -> Result<usize> {
        unsafe {
            chain_then(sys::kvm_get_vcpu_mmap_size(self.0)).map(|v| v as usize)
        }
    }

    pub fn set_tss(&mut self, addr: usize) -> Result<()> {
        unsafe {
            chain_then(sys::kvm_set_tss_addr(self.0, addr as i32)).map(|_| ())
        }
    }

    pub fn mmap(&mut self) -> Result<()> {
        use nix::sys::mman;
        use nix::libc;
        let size = self.get_mmap_size()?;
        unsafe {
            let prot = mman::ProtFlags::PROT_READ | mman::ProtFlags::PROT_WRITE;
            let map = mman::MapFlags::MAP_FILE | mman::MapFlags::MAP_SHARED;
            let ptr = mman::mmap(0 as *mut libc::c_void, size, prot, map, self.0, 0)
                .chain_err(|| ErrorKind::KvmCoreOperationError)?;
            self.1 = Some(ptr as *mut sys::Run);
        }

        Ok(())
    }
}

impl Drop for Core {
    fn drop(&mut self) {
        nix::unistd::close(self.0).unwrap();
        match self.1 {
            Some(ptr) => unsafe { nix::sys::mman::munmap(ptr as *mut nix::libc::c_void, self.get_mmap_size().unwrap()).unwrap(); },
            _ => {}
        }
    }
}

impl CheckCapability for Core {
    fn check_capability(&mut self, cap: Capability) -> Result<i32> {
        unsafe { sys::kvm_check_extension(self.0, cap.into()) }
            .chain_err(|| ErrorKind::KvmCapabilityCheckError)
    }
}

unsafe impl Send for Core {}
