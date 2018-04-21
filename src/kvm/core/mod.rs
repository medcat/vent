use std::os::unix::io::RawFd;
use std::result::Result as StdResult;
use nix;
use kvm_sys as sys;
use super::{CheckCapability, Capability};
use error::*;

mod run;


pub use self::run::{Run, Exit};

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub enum State {
    Unknown,
    /// The vCPU is currently running.  Only supported on x86, ARM, and arm64.
    Runnable,
    /// The vCPU is an application processor which has not yet received an INIT
    /// signal.  Only supported on x86.
    Uninitialized,
    /// The vCPU has received an INIT signal, and is now ready for a SIPI.
    /// Only supoprted on x86.
    InitReceived,
    /// The vCPU has executed a HLT instruction and is waiting for an interrupt.
    /// Only supported on x86.
    Halted,
    /// The vCPU has just received a SIPI.  Only supported on x86.
    SipiReceived,
    /// The vCPU is stopped.  Only supported on s390, ARM, and arm64.
    Stopped,
    /// The vCPU is in a special error state.  Only supported on s390.
    CheckStop,
    /// The vCPU is operating (running or halted).  Only supported on s390.
    Operating,
    /// The vCPU is in a special load/startup state.  Only supported on s390.
    Load
}

impl Into<u32> for State {
    fn into(self) -> u32 {
        match self {
            State::Runnable => sys::KVM_MP_STATE_RUNNABLE,
            State::Uninitialized => sys::KVM_MP_STATE_UNINITIALIZED,
            State::InitReceived => sys::KVM_MP_STATE_INIT_RECEIVED,
            State::Halted => sys::KVM_MP_STATE_HALTED,
            State::SipiReceived => sys::KVM_MP_STATE_SIPI_RECEIVED,
            State::Stopped => sys::KVM_MP_STATE_STOPPED,
            State::CheckStop => sys::KVM_MP_STATE_CHECK_STOP,
            State::Operating => sys::KVM_MP_STATE_OPERATING,
            State::Load => sys::KVM_MP_STATE_LOAD,
            State::Unknown => -1i32 as u32
        }
    }
}

impl From<u32> for State {
    fn from(v: u32) -> State {
        match v {
            sys::KVM_MP_STATE_RUNNABLE => State::Runnable,
            sys::KVM_MP_STATE_UNINITIALIZED => State::Uninitialized,
            sys::KVM_MP_STATE_INIT_RECEIVED => State::InitReceived,
            sys::KVM_MP_STATE_HALTED => State::Halted,
            sys::KVM_MP_STATE_SIPI_RECEIVED => State::SipiReceived,
            sys::KVM_MP_STATE_STOPPED => State::Stopped,
            sys::KVM_MP_STATE_CHECK_STOP => State::CheckStop,
            sys::KVM_MP_STATE_OPERATING => State::Operating,
            sys::KVM_MP_STATE_LOAD => State::Load,
            _ => State::Unknown
        }
    }
}

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
