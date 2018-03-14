use std::os::unix::io::RawFd;
use std::result::Result as StdResult;
use nix;
use super::{sys, ErrorKind, Result, ResultExt, CheckCapability, Capability};

pub use super::sys::StateValue as State;

#[derive(Debug)]
pub struct Core(pub(super) RawFd);

fn chain_then(res: StdResult<i32, nix::Error>) -> Result<()> {
    res
        .chain_err(|| ErrorKind::KvmCoreOperationError)
        .and_then(|v| if v == 0 { Ok(()) } else { Err(ErrorKind::KvmCoreOperationError.into()) })
}

impl Core {
    pub fn get_state(&mut self) -> Result<State> {
        unsafe {
            let mut state = sys::MpState { mp_state: State::Runnable };
            chain_then(sys::kvm_get_mp_state(self.0, &mut state as *mut sys::MpState))?;
            Ok(state.mp_state)
        }
    }

    pub fn set_state(&mut self, statev: State) -> Result<()> {
        unsafe {
            let state = sys::MpState { mp_state: statev };
            chain_then(sys::kvm_set_mp_state(self.0, &state as *const sys::MpState))
        }
    }
}

impl Drop for Core {
    fn drop(&mut self) {
        nix::unistd::close(self.0).unwrap()
    }
}

impl CheckCapability for Core {
    fn check_capability(&mut self, cap: Capability) -> Result<i32> {
        unsafe { sys::kvm_check_extension(self.0, cap.into()) }
            .chain_err(|| ErrorKind::KvmCapabilityCheckError)
    }
}

unsafe impl Send for Core {}
