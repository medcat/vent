use super::kvm;
// use super::device;

// #[cfg(target_os = "linux")]
mod notify;

pub struct Instance {
    machine: kvm::Machine,
    cores: Vec<kvm::Core>,
    // devices: Vec<device::Device>,
}
