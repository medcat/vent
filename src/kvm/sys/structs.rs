pub const KVM_CLOCK_TSC_STABLE: u32 = 2;

#[repr(C)]
/// From the struct `kvm_msr_list`.
pub struct MsrList {
    pub nmsrs: u32,
    pub indicies: [u32; 0]
}

#[repr(C)]
#[derive(Debug, Copy, Clone, PartialEq, Eq)]
/// From the struct `kvm_dirty_log`.
pub struct DirtyLog {
    pub slot: u32,
    pub _pad: u32,
    /// This is meant to be a union of a pointer and a u64; the pointer has
    /// type `struct __user *dirty_bitmap`, with the u64 having the type
    /// `__u64 padding2`.  Make of that what you will.
    pub value: u64
}

#[repr(C)]
#[derive(Debug, Copy, Clone, PartialEq, Eq)]
/// From the struct `kvm_interrupt`.
pub struct Interrupt {
    pub irq: u32
}

#[repr(C)]
#[derive(Copy, Clone)]
/// From the struct `kvm_fpu`.
pub struct Fpu {
    pub fpr: [[u8; 16]; 8],
    pub fcw: u16,
    pub fsw: u16,
    pub ftwx: u8,
    pub pad1: u8,
    pub last_opcode: u16,
    pub last_ip: u64,
    pub last_dp: u64,
    pub xmm: [[u8; 16]; 16],
    pub mxcsr: u32,
    pub pad2: u32
}

#[repr(C)]
#[derive(Debug, Copy, Clone, PartialEq, Eq)]
/// From the struct `kvm_cpuid_entry`.
pub struct CpuIdEntry {
    pub function: u32,
    pub eax: u32,
    pub ebx: u32,
    pub ecx: u32,
    pub edx: u32,
    pub _pad: u32
}

#[repr(C)]
/// From the struct `kvm_cpuid`.
pub struct CpuId {
    pub nent: u32,
    pub padding: u32,
    pub entries: [CpuIdEntry; 0]
}

#[repr(C)]
/// From the struct `kvm_signal_mask`.
pub struct SignalMask {
    pub len: u32,
    pub sigset: [u8; 0]
}

#[repr(C)]
#[derive(Debug, Copy, Clone, PartialEq, Eq)]
/// From the struct `kvm_irq_level`.
pub struct IrqLevel {
    /// This field also acts as the `status` field.
    pub irq: u32,
    pub level: u32
}

#[repr(C)]
#[derive(Debug, Copy, Clone, PartialEq, Eq)]
/// From the struct `kvm_xen_hvm_config`.
pub struct XenHvmConfig {
    pub flags: u32,
    pub msr: u32,
    pub blob_addr_32: u64,
    pub blob_addr_64: u64,
    pub blob_size_32: u8,
    pub blob_size_64: u8,
    pub _pad: [u8; 30]
}

#[repr(C)]
#[derive(Debug, Copy, Clone, PartialEq, Eq)]
/// From the struct `kvm_clock_data`.
pub struct ClockData {
    pub clock: u64,
    pub flags: u32,
    pub _pad: [u32; 9]
}

pub const KVM_MEM_LOG_DIRTY_PAGES: u32 = 1u32 << 0;
pub const KVM_MEM_READONLY: u32 = 1u32 << 1;

#[repr(C)]
#[derive(Debug, Copy, Clone, PartialEq, Eq)]
/// From the struct `kvm_userspace_memory_region`.
pub struct UserspaceMemoryRegion {
    pub slot: u32,
    pub flags: u32,
    pub guest_phys_addr: u64,
    /// in bytes.
    pub memory_size: u64,
    /// the start of th userspace allocated memory.
    pub userspace_addr: u64
}

#[repr(C)]
#[derive(Copy, Clone)]
/// From the struct `kvm_enable_cap`.
pub struct EnableCap {
    /// The capability that is to be enabled.
    pub cap: i32,
    /// Should always be 0.
    pub flags: u32,
    pub args: [u64; 4],
    pub _pad: [u8; 64]
}

#[repr(u32)]
#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub enum StateValue {
    /// The vCPU is currently running.  Only supported on x86, ARM, and arm64.
    Runnable = 0u32,
    /// The vCPU is an application processor which has not yet received an INIT
    /// signal.  Only supported on x86.
    Uninitialized = 1u32,
    /// The vCPU has received an INIT signal, and is now ready for a SIPI.
    /// Only supoprted on x86.
    InitReceived = 2u32,
    /// The vCPU has executed a HLT instruction and is waiting for an interrupt.
    /// Only supported on x86.
    StateHalted = 3u32,
    /// The vCPU has just received a SIPI.  Only supported on x86.
    SipiReceived = 4u32,
    /// The vCPU is stopped.  Only supported on s390, ARM, and arm64.
    Stopped = 5u32,
    /// The vCPU is in a special error state.  Only supported on s390.
    CheckStop = 6u32,
    /// The vCPU is operating (running or halted).  Only supported on s390.
    Operating = 7u32,
    /// The vCPU is in a special load/startup state.  Only supported on s390.
    Load = 8u32
}

#[repr(C)]
#[derive(Debug, Copy, Clone, PartialEq, Eq)]
/// From the struct `kvm_mp_state`
pub struct MpState {
    pub mp_state: StateValue
}

#[repr(C)]
#[derive(Debug, Copy, Clone, PartialEq, Eq)]
/// From the struct `kvm_translation`.
pub struct Translation {
    /// The input parameter.
    pub linear_address: u64,
    /// The output parameter.
    pub physical_address: u64,
    pub valid: u8,
    pub writable: u8,
    pub usermode: u8,
    pub _pad: [u8; 5]
}
