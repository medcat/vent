/// The registers.  Note that this definition only works for x64 hosts and
/// guests; we'll assume that this is the case.  If this a problem, we'll
/// extract this behavior out to be more platform independent.
#[repr(C)]
#[derive(Debug, Copy, Clone, PartialEq, Eq)]
/// From the struct `kvm_regs`.
pub struct Regs {
    pub rax: u64, pub rbx: u64, pub rcx: u64, pub rdx: u64,
    pub rsi: u64, pub rdi: u64, pub rsp: u64, pub rbp: u64,
    pub r8:  u64, pub r9:  u64, pub r10: u64, pub r11: u64,
    pub r12: u64, pub r13: u64, pub r14: u64, pub r15: u64,
    pub rip: u64, pub rflags: u64
}


#[repr(C)]
#[derive(Debug, Copy, Clone, PartialEq, Eq)]
/// From the struct `kvm_segment`.
pub struct Segment {
    pub base: u64, pub limit: u32, pub selector: u16,
    /// Note: in the linux kernel, this is named `type`
    pub kind: u8,
    pub present: u8, pub dp1: u8, pub dp: u8, pub s: u8, pub l: u8,
    pub g: u8, pub avl: u8,
    pub unusable: u8, pub padding: u8
}

#[repr(C)]
#[derive(Debug, Copy, Clone, PartialEq, Eq)]
/// From the struct `kvm_dtable`.
pub struct Dtable {
    pub base: u64, pub limit: u16,
    pub padding: [u16; 3]
}

#[repr(C)]
#[derive(Debug, Copy, Clone, PartialEq, Eq)]
/// From the struct `sregs`.
pub struct Sregs {
    pub cs: Segment, pub ds: Segment, pub es: Segment,
    pub fs: Segment, pub gs: Segment, pub ss: Segment,
    pub tr: Segment, pub ldt: Segment,
    pub gdt: Dtable, pub idt: Dtable,
    pub cr0: u64, pub cr2: u64, pub cr3: u64, pub cr4: u64, pub cr8: u64,
    pub efer: u64, pub apic_base: u64,
    /// The size of this field is actually `(KVM_NR_INTERRUPTS + 63) / 64`,
    /// where `KVM_NR_INTERRUPTS = 256`; this was defined in the linux
    /// kernel; see `/arch/x86/include/uapi/asm/kvm.h`.
    ///
    /// This is a bitmap of pending external interrupts.  At most, one bit
    /// may be set.  The interrupt has been acknowledged by the APIC, but
    /// not yet injected.
    pub interrupt_bitmap: [u64; (256 + 63) / 64]
}

#[repr(C)]
#[derive(Debug, Copy, Clone, PartialEq, Eq)]
/// From the struct `kvm_msr_entry`.
pub struct MsrEntry {
    pub index: u32,
    pub reserved: u32,
    pub data: u64
}

#[repr(C)]
/// From the struct `kvm_msrs`.
pub struct Msrs {
    pub nmsrs: u32,
    pub pad: u32,

    pub entries: [MsrEntry; 0]
}
