use super::{sys, Result, ErrorKind};

pub trait CheckCapability {
    fn check_capability(&mut self, Capability) -> Result<i32>;

    fn has_capability(&mut self, cap: Capability) -> bool {
        self.check_capability(cap).map(|v| v > 0).unwrap_or(false)
    }

    fn assert_capability(&mut self, cap: Capability) -> Result<()> {
        self.check_capability(cap)
            .and_then(|v| if v < 0 { Err(ErrorKind::KvmMissingCapabilityError.into()) } else { Ok(()) })
    }
}

#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq)]
pub enum Capability {
    EnableCap,
    EnableCapVm,
}

impl Into<i32> for Capability {
    fn into(self) -> i32 {
        match self {
            Capability::EnableCap => sys::KVM_CAP_ENABLE_CAP,
            Capability::EnableCapVm => sys::KVM_CAP_ENABLE_CAP_VM
        }
    }
}
