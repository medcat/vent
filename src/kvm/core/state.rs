use kvm_sys as sys;

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
