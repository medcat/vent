use kvm_sys as sys;
use error::*;

pub trait CheckCapability {
    fn check_capability(&mut self, Capability) -> Result<i32>;

    fn has_capability(&mut self, cap: Capability) -> bool {
        self.check_capability(cap).map(|v| v > 0).unwrap_or(false)
    }

    fn assert_capability(&mut self, cap: Capability) -> Result<()> {
        self.check_capability(cap)
            .and_then(|v| if v < 0 { Err(ErrorKind::KvmMissingCapabilityError(cap).into()) } else { Ok(()) })
    }
}

#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq)]
pub enum Capability {
    EnableCap,
    EnableCapVm,
    IrqChip,
    AdjustClock,
    MpState,
    UserspaceMemory,
}

use std::fmt;
use std::result::Result as StdResult;

impl fmt::Display for Capability {
    fn fmt<'a>(&self, fmt: &mut fmt::Formatter<'a>) -> StdResult<(), fmt::Error> {
        match *self {
            Capability::EnableCap => write!(fmt, "KVM_CAP_ENABLE_CAP"),
            Capability::EnableCapVm => write!(fmt, "KVM_CAP_ENABLE_CAP_VM"),
            Capability::IrqChip => write!(fmt, "KVM_CAP_IRQCHIP"),
            Capability::AdjustClock => write!(fmt, "KVM_CAP_ADJUST_CLOCK"),
            Capability::MpState => write!(fmt, "KVM_CAP_MP_STATE"),
            Capability::UserspaceMemory => write!(fmt, "KVM_CAP_USER_MEMORY")
        }
    }
}

impl Into<i32> for Capability {
    fn into(self) -> i32 {
        match self {
            Capability::EnableCap => sys::KVM_CAP_ENABLE_CAP,
            Capability::EnableCapVm => sys::KVM_CAP_ENABLE_CAP_VM,
            Capability::IrqChip => sys::KVM_CAP_IRQCHIP,
            Capability::AdjustClock => sys::KVM_CAP_ADJUST_CLOCK,
            Capability::MpState => sys::KVM_CAP_MP_STATE,
            Capability::UserspaceMemory => sys::KVM_CAP_USER_MEMORY
        }
    }
}
