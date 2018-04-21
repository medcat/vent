use super::sys::run::*;
use super::sys::run::{Run as SysRun, Exit as SysExit};
use super::sys;
use error::*;

#[derive(Debug, Copy, Clone)]
pub struct Run {
    /* in */
    pub request_interrupt_window: u8,
    pub immediate_exit: u8,

    /* out */
    pub ready_for_interrupt_injection: u8,
    pub if_flag: u8,
    pub flags: u16,

    /* in (pre_kvm_run), out (post_kvm_run) */
    pub cr8: u64,
    pub apic_base: u64,

    /* the processor status word for s390 */
    pub psw_mask: u64, /* psw upper half */
    pub psw_addr: u64, /* psw lower half */

    pub exit: Exit,

    /*
     * shared registers between kvm and userspace.
     * kvm_valid_regs specifies the register classes set by the host
     * kvm_dirty_regs specified the register classes dirtied by userspace
     * struct kvm_sync_regs is architecture specific, as well as the
     * bits for kvm_valid_regs and kvm_dirty_regs
     */
    pub kvm_valid_regs: u64,
    pub kvm_dirty_regs: u64,
    // union {
    //     struct kvm_sync_regs regs;
    // } s;
    //
    // x86 doesn't have anything in struct kvm_sync_regs, so ignore.
}

#[derive(Debug, Copy, Clone)]
pub enum Exit {
    Unknown(ExitUnknown),
    FailEntry(ExitFailEntry),
    Exception(ExitException),
    Io(ExitIo),
    // debug: ExitDebug
    Mmio(ExitMmio),
    Hypercall(ExitHypercall),
    TprAccess(ExitTprAccess),
    S390Sieic(ExitS390Sieic),
    S390ResetFlags(u64),
    S390Ucontrol(ExitS390Ucontrol),
    Dcr(ExitDcr),
    Internal(ExitInternal),
    Osi(ExitOsi),
    PaprHcall(ExitPaprHcall),
    S390Tsch(ExitS390Tsch),
    Epr(ExitEpr),
    SystemEvent(ExitSystemEvent),
    S390Stsi(ExitS390Stsi),
    Eoi(ExitEoi)
}

impl Run {
    pub fn from(sys: SysRun) -> Result<Run> {
        Ok(Run {
            request_interrupt_window: sys.request_interrupt_window,
            immediate_exit: sys.immediate_exit,
            ready_for_interrupt_injection: sys.ready_for_interrupt_injection,
            if_flag: sys.if_flag,
            flags: sys.flags,
            cr8: sys.cr8,
            apic_base: sys.apic_base,
            psw_mask: sys.psw_mask,
            psw_addr: sys.psw_addr,
            exit: Exit::from((sys.exit_reason, sys.exit))?,
            kvm_valid_regs: sys.kvm_valid_regs,
            kvm_dirty_regs: sys.kvm_dirty_regs
        })
    }

    pub fn into(self) -> SysRun {
        let (reason, exit) = self.exit.into();
        SysRun {
            request_interrupt_window: self.request_interrupt_window,
            immediate_exit: self.immediate_exit,
            _pad1: [0u8; 6],
            exit_reason: reason,
            ready_for_interrupt_injection: self.ready_for_interrupt_injection,
            if_flag: self.if_flag,
            flags: self.flags,
            cr8: self.cr8,
            apic_base: self.apic_base,
            psw_mask: self.psw_mask,
            psw_addr: self.psw_addr,

            exit: exit,
            kvm_valid_regs: self.kvm_valid_regs,
            kvm_dirty_regs: self.kvm_dirty_regs,
            _pad2: [0u8; 2048],
        }
    }
}

impl Exit {
    pub fn from((reason, exit): (u32, SysExit)) -> Result<Exit> {
        match reason {
            sys::KVM_EXIT_UNKNOWN => Ok(Exit::Unknown(unsafe { exit.hw })),
            sys::KVM_EXIT_FAIL_ENTRY => Ok(Exit::FailEntry(unsafe { exit.fail_entry })),
            sys::KVM_EXIT_EXCEPTION => Ok(Exit::Exception(unsafe { exit.ex })),
            sys::KVM_EXIT_IO => Ok(Exit::Io(unsafe { exit.io })),
            sys::KVM_EXIT_MMIO => Ok(Exit::Mmio(unsafe { exit.mmio })),
            sys::KVM_EXIT_HYPERCALL => Ok(Exit::Hypercall(unsafe { exit.hypercall })),
            sys::KVM_EXIT_TPR_ACCESS => Ok(Exit::TprAccess(unsafe { exit.tpr_access })),
            sys::KVM_EXIT_S390_SIEIC => Ok(Exit::S390Sieic(unsafe { exit.s390_sieic })),
            sys::KVM_EXIT_S390_RESET => Ok(Exit::S390ResetFlags(unsafe { exit.s390_reset_flags })),
            sys::KVM_EXIT_S390_UCONTROL => Ok(Exit::S390Ucontrol(unsafe { exit.s390_ucontrol })),
            sys::KVM_EXIT_DCR => Ok(Exit::Dcr(unsafe { exit.dcr })),
            sys::KVM_EXIT_INTERNAL_ERROR => Ok(Exit::Internal(unsafe { exit.internal })),
            sys::KVM_EXIT_OSI => Ok(Exit::Osi(unsafe { exit.osi })),
            sys::KVM_EXIT_PAPR_HCALL => Ok(Exit::PaprHcall(unsafe { exit.papr_hcall })),
            sys::KVM_EXIT_S390_TSCH => Ok(Exit::S390Tsch(unsafe { exit.s390_tsch })),
            sys::KVM_EXIT_EPR => Ok(Exit::Epr(unsafe { exit.epr })),
            sys::KVM_EXIT_SYSTEM_EVENT => Ok(Exit::SystemEvent(unsafe { exit.system_event })),
            sys::KVM_EXIT_S390_STSI => Ok(Exit::S390Stsi(unsafe { exit.s390_stsi })),
            sys::KVM_EXIT_IOAPIC_EOI => Ok(Exit::Eoi(unsafe { exit.eoi })),
            _ => Err(ErrorKind::KvmIllegalExitReasonError(reason).into())
        }
    }

    pub fn into(self) -> (u32, SysExit) {
        match self {
            Exit::Unknown(d) => (sys::KVM_EXIT_UNKNOWN, SysExit { hw: d }),
            Exit::FailEntry(d) => (sys::KVM_EXIT_FAIL_ENTRY, SysExit { fail_entry: d }),
            Exit::Exception(d) => (sys::KVM_EXIT_EXCEPTION, SysExit { ex: d }),
            Exit::Io(d) => (sys::KVM_EXIT_IO, SysExit { io: d }),
            Exit::Mmio(d) => (sys::KVM_EXIT_MMIO, SysExit { mmio: d }),
            Exit::Hypercall(d) => (sys::KVM_EXIT_HYPERCALL, SysExit { hypercall: d }),
            Exit::TprAccess(d) => (sys::KVM_EXIT_TPR_ACCESS, SysExit { tpr_access: d }),
            Exit::S390Sieic(d) => (sys::KVM_EXIT_S390_SIEIC, SysExit { s390_sieic: d }),
            Exit::S390ResetFlags(d) => (sys::KVM_EXIT_S390_RESET, SysExit { s390_reset_flags: d }),
            Exit::S390Ucontrol(d) => (sys::KVM_EXIT_S390_UCONTROL, SysExit { s390_ucontrol: d }),
            Exit::Dcr(d) => (sys::KVM_EXIT_DCR, SysExit { dcr: d }),
            Exit::Internal(d) => (sys::KVM_EXIT_INTERNAL_ERROR, SysExit { internal: d }),
            Exit::Osi(d) => (sys::KVM_EXIT_OSI, SysExit { osi: d }),
            Exit::PaprHcall(d) => (sys::KVM_EXIT_PAPR_HCALL, SysExit { papr_hcall: d }),
            Exit::S390Tsch(d) => (sys::KVM_EXIT_S390_TSCH, SysExit { s390_tsch: d }),
            Exit::Epr(d) => (sys::KVM_EXIT_EPR, SysExit { epr: d }),
            Exit::SystemEvent(d) => (sys::KVM_EXIT_SYSTEM_EVENT, SysExit { system_event: d }),
            Exit::S390Stsi(d) => (sys::KVM_EXIT_S390_STSI, SysExit { s390_stsi: d }),
            Exit::Eoi(d) => (sys::KVM_EXIT_IOAPIC_EOI, SysExit { eoi: d })
        }
    }
}
