//! `KVM_GET_IRQCHIP`, `KVM_SET_IRQCHIP`, `KVM_GET_VCPU_EVENTS`, `KVM_SET_VCPU_EVENTS`
//! `KVM_GET_DEBUGREGS`, and `KVM_SET_DEBUGREGS` are unsupported.

use super::*;

const KVMIO: u8 = 0xAE;

ioctl! {
    /// This identifies the API version as the stable kvm API. It is not
    /// expected that this number will change.  However, Linux 2.6.20 and
    /// 2.6.21 report earlier versions; these are not documented and not
    /// supported.  Applications should refuse to run if this returns a
    /// value other than 12.
    ///
    /// # Support
    /// This ioctl is supported by all architectures, and is a basic
    /// capability. This should only be run on the system file descriptor.
    none kvm_get_api_version with KVMIO, 0x00
}

ioctl! {
    /// Creates a VM.  The new VM has no virtual CPUs and no memory.  This
    /// returns a new file descriptor on success that can then be used with
    /// other ioctls.
    ///
    /// # Arguments
    /// - `data` - The machine type.  You probably want this to be 0.
    ///
    /// # Support
    /// This ioctl is supported by all architectures, and is a basic
    /// capability. This should only be run on the system file descriptor.
    write_int kvm_create_vm with KVMIO, 0x01
}

ioctl! {
    /// The passed `kvm_msr_list` should have the size of the number of indicies
    /// it can hold, and in return, KVM will fill both the incidies and the
    /// `nmsrs` with the right data.
    ///
    /// This returns the guest MSRs that are supported.  The list varies by
    /// KVM version and host processor, but does not change otherwise.
    ///
    /// **Note:** if KVM indicates support for MCE (via the `KVM_CAP_MCE`
    /// capability check), then the MCE bank MSRs are not returned in the MSR list,
    /// as different vCPUs can have a different number of banks (as set via
    /// `x86_setup_mce`).
    ///
    /// # Support
    /// This ioctl is supported only by the x86 architecture, and is a basic
    /// capability. This should only be run on the system file descriptor.
    readwrite kvm_get_msr_index_list with KVMIO, 0x02; MsrList
}

ioctl! {
    /// The passed `MsrList` should have the size of the number of indicies
    /// it can hold, and in return, KVM will fill both the incidies and the
    /// `nmsrs` with the right data.
    ///
    /// This returns the list of MSRs that can be passed to `get_msrs`.  This lets
    /// userspace probe host capabilities and processor features that are exposed via
    /// MSRs (e.g., VMX capabilities).  This list also varies by kvm version and host
    /// processor, but does not change otherwise.
    ///
    /// # Support
    /// This ioctl is supported only by the x86 architecture, and requires the
    /// `KVM_CAP_GET_MSR_FEATURES` capability. This should only be run on the
    /// system file descriptor.
    readwrite kvm_get_msr_feature_index_list with KVMIO, 0x03; MsrList
}

ioctl! {
    /// The API allows the application to query about extensions to the core
    /// kvm API.  Userspace passes an extension identifier (an integer) and
    /// receives an integer that describes the extension availability.
    /// Generally 0 means no and 1 means yes, but some extensions may report
    /// additional information in the integer return value.
    ///
    /// Based on their initialization different VMs may have different
    /// capabilities. It is thus encouraged to use the VM ioctl to query for
    /// capabilities (available with `KVM_CAP_CHECK_EXTENSION_VM` on the VM file
    /// descriptor).
    ///
    /// Capabilities can be found in the `capabilities` module.
    ///
    /// # Support
    /// This ioctl is supported by all architectures, and is a basic
    /// capability.  This is available on both the system and VM file
    /// descriptors.
    write_int kvm_check_extension with KVMIO, 0x03
}

ioctl! {
    /// The size of the memory region that the `kvm_run` ioctl uses to communicate
    /// with userspace, in bytes.
    ///
    /// # Support
    /// This ioctl is supported by all architectures, and is a basic
    /// capability.  This is available only on the system file descriptor.
    none kvm_get_vcpu_mmap_size with KVMIO, 0x04
}


ioctl! {
    /// This adds a vCPU to a virtual machine.  No more than `max_cpu`s may be
    /// added.  The vCPU id is an integer that is `0 <= id < max_vcpu_id`.  This
    /// returns a new file descriptor on success that can then be used with
    /// other ioctls.
    ///
    /// The recommended `max_vcpu` can be retrieved using the `KVM_CAP_NR_VCPUS`
    /// capability check (see `check_extension`) at run time.  The absolute
    /// maximum possible value for `max_vcpu` can be retrieved using the
    /// `KVM_CAP_MAX_VCPUS` capability check.  If `KVM_CAP_NR_VCPUS` doesn't
    /// exist, assume that `max_vcpu = 4`; if `KVM_CAP_MAX_VCPUS` doesn't exist,
    /// assume that it is the same as the result of `KVM_CAP_NR_VCPUS`.
    ///
    /// The value for `max_vcpu_id`  can be retrieved using the
    /// `KVM_CAP_MAX_VCPU_ID` capability check.  If it does not exist, assume
    /// that it is the same as the result of `KVM_CAP_MAX_VCPUS`.
    ///
    /// # Support
    /// This ioctl is supported by all architectures, and is a basic
    /// capability.  This is available only on the VM file descriptor.
    write_int kvm_create_vcpu with KVMIO, 0x41
}

ioctl! {
    /// Given a memory slot, return a bitmap containing any pages dirtied
    /// since the last call to this ioctl.  Bit 0 is the first page in the
    /// memory slot.  Ensure the entire structure is cleared to avoid padding
    /// issues.
    ///
    /// If `KVM_CAP_MULTI_ADDRESS_SPACE` is available, bits 16-31 specifies
    /// the address space for which you want to return the dirty bitmap.
    /// They must be less than the value that `KVM_CHECK_EXTENSION` returns for
    /// the `KVM_CAP_MULTI_ADDRESS_SPACE` capability.
    ///
    /// # Support
    /// This ioctl is supported only by the x86 architecture, and is a basic
    /// capability.  This is available only on the VM file descriptor.
    write_ptr kvm_get_dirty_log with KVMIO, 0x42; DirtyLog
}

ioctl! {
    /// This ioctl is used to run a guest virtual cpu.  While there are no
    /// explicit parameters, there is an implicit parameter block that can be
    /// obtained by `mmap`ing the vCPU file descriptor at offset 0, with the size
    /// given by `kvm_get_vcpu_mmap_size`.
    ///
    /// # Support
    /// This ioctl is supported by all architectures, and is a basic
    /// capability.  This is available only on the vCPU file descriptor.
    none kvm_run with KVMIO, 0x80
}

ioctl! {
    /// Reads the general-purpose registers from the given vCPU.  The
    /// result of this call is dependent on the architecture.
    ///
    /// # Safety
    /// Right now, this assumes that the host/guest architecture will be
    /// x86.  Therefore, this assumes that the host/guest architecture
    /// will have the registers listed in `Regs`.
    ///
    /// # Support
    /// This ioctl is supported by all architectures (except for ARM and
    /// arm64), and is a basic capability.  This is available only on
    /// the vCPU file descriptor.
    read kvm_get_regs with KVMIO, 0x81; Regs
}

ioctl! {
    /// Writes to the general-purpose registers into the given vCPU.  The
    /// parameter passed into this call is dependent on the architecture.
    ///
    /// # Safety
    /// Right now, this assumes that the host/guest architecture will be
    /// x86.  Therefore, this assumes that the host/guest architecture
    /// will have the registers listed in `Regs`.
    ///
    /// # Support
    /// This ioctl is supported by all architectures (except for ARM and
    /// arm64), and is a basic capability.  This is available only on
    /// the vCPU file descriptor.
    write_ptr kvm_set_regs with KVMIO, 0x82; Regs
}

ioctl! {
    /// Reads special registers from the vCPU.  This assumes that the
    /// vCPU is x86-based.
    ///
    /// # Support
    /// This ioctl is supported only by x86 and ppc, and is a basic
    /// capability.  This is available only on the vCPU file descriptor.
    read kvm_get_sregs with KVMIO, 0x83; Sregs
}

ioctl! {
    /// Writes special registers to the vCPU.  This assumes that the
    /// vCPU is x86-based.
    ///
    /// # Support
    /// This ioctl is supported only by x86 and ppc, and is a basic
    /// capability.  This is available only on the vCPU file descriptor.
    write_ptr kvm_set_sregs with KVMIO, 0x84; Sregs
}

ioctl! {
    /// Translates a virtual address according to the vCPU's current
    /// address translation mode.
    ///
    /// # Support
    /// This ioctl is supported only by x86, and is a basic capability.
    /// This is available only on the vCPU file descriptor.
    readwrite kvm_translate with KVMIO, 0x85; Translation
}

ioctl! {
    /// Queues a hardware interrupt vector to be injected.  Note that
    /// the interrupt is an interrupt _vector_.
    ///
    /// # Support
    /// This ioctl is supported only by x86, ppc, and mips, and is a
    /// basic compatability.  This is available only on the vCPU
    /// file descriptor.
    write_ptr kvm_interrupt with KVMIO, 0x86; Interrupt
}

ioctl! {
    /// This ioctl has varying behavior based on whether it is used on a
    /// system file descriptor or a vCPU file descriptor.
    ///
    /// When used on a system file descriptor, it reads the values of
    /// MSR-based features that are available for the VM.  This is similar
    /// to `kvm_get_supported_cpuid`, but it returns MSR indices and values.
    /// The list of MSR-based features can be optained using
    /// `kvm_get_msr_feature_index_list`.
    ///
    /// When used on a vCPU file descriptor, it reads model-specific
    /// registers from the vCPU.  Supported MSR indicies can be obtained
    /// using `kvm_get_msr_index_list`.
    ///
    /// # Support
    /// This ioctl is supported only by the x86 architecture.  This is
    /// available only on either the system or vCPU file descriptors.
    /// If used on a vCPU file descriptor, it is a basic capability.
    /// If used on a system file descriptor, it requires the
    /// `KVM_CAP_GET_MSR_FEATURES` capability.
    readwrite kvm_get_msrs with KVMIO, 0x88; Msrs
}

ioctl! {
    /// Defines the vCPU responses to the CPUID instruction.
    ///
    /// # Support
    /// This ioctl is supported only by x86, and is a basic capability.
    /// This is available only on the vCPU file descriptor.
    write_ptr kvm_set_cpuid with KVMIO, 0x8a; CpuId
}

ioctl! {
    /// Defines which signals are blocked during execution of KVM_RUN.  This
    /// signal mask temporarily overrides the threads signal mask.  Any
    /// unblocked signal received (except SIGKILL and SIGSTOP, which retain
    /// their traditional behaviour) will cause KVM_RUN to return with -EINTR.
    ///
    /// Note the signal will only be delivered if not blocked by the original
    /// signal mask.
    ///
    /// # Support
    /// This ioctl is supported by all architectures, and is a basic
    /// capability.  This is available only on the vCPU file descriptor.
    write_ptr kvm_set_signal_mask with KVMIO, 0x8b; SignalMask
}

ioctl! {
    /// Reads the floating-point state from the vCPU.
    ///
    /// # Support
    /// This ioctl is supported only by x86, and is a basic capability.
    /// This is available only on the vCPU file descriptor.
    read kvm_get_fpu with KVMIO, 0x8c; Fpu
}

ioctl! {
    /// Writes the floating-point state to the vCPU.
    ///
    /// # Support
    /// This ioctl is supported only by x86, and is a basic capability.
    /// This is available only on the vCPU file descriptor.
    write_ptr kvm_set_fpu with KVMIO, 0x8d; Fpu
}

ioctl! {
    /// Creates an interrupt control model in the kernel.  For x86,
    /// it creates a virtual ioapic, a virtual PIC (two PICs, nested), and
    /// sets up future vCPUs to have a local APIC.  IRQ routing for GSIs 0-15
    /// is set to both PIC and IOAPIC; GSI 16-23 only go to the IOAPIC.
    /// On ARM/arm64, a GICv2 is created. Any other GIC versions require the usage
    /// of `kvm_create_device`, which also supports creating a GICv2.  Using
    /// `kvm_create_device` is preferred over KVM_CREATE_IRQCHIP for GICv2. On
    /// s390, a dummy irq routing table is created.
    ///
    /// # Support
    /// This ioctl is only supported by x86, ARM, arm64, and s390 architectures.
    /// This requires the `KVM_CAP_IRQCHIP` for x86, ARM, and arm64, and
    /// `KVM_CAP_S390_IRQCHIP` for s390 capabilities.  This is available only on
    /// the VM file descriptor.
    none kvm_create_irqchip with KVMIO, 0x60
}

ioctl! {
    /// Sets the level of a GSI input to the interrupt controller model in the kernel.
    /// On some architectures it is required that an interrupt controller model has
    /// been previously created with `kvm_create_irqchip`.  Note that edge-triggered
    /// interrupts require the level to be set to 1 and then back to 0.
    ///
    /// On real hardware, interrupt pins can be active-low or active-high.  This
    /// does not matter for the level field of struct kvm_irq_level: 1 always
    /// means active (asserted), 0 means inactive (deasserted).
    ///
    /// # Support
    /// This ioctl is only supported by x86, ARM, and arm64 architectures.
    /// This requires the `KVM_CAP_IRQCHIP` capability.  This is available only on
    /// the VM file descriptor.
    write_ptr kvm_irq_line with KVMIO, 0x61; IrqLevel
}

ioctl! {
    /// Sets the MSR that the Xen HVM guest uses to initialize its hypercall
    /// page, and provides the starting address and size of the hypercall
    /// blobs in userspace.  When the guest writes the MSR, kvm copies one
    /// page of a blob (32- or 64-bit, depending on the vcpu mode) to guest
    /// memory.
    ///
    /// # Support
    /// This ioctl is only supported by x86 architecture. This requires the
    /// `KVM_CAP_XEN_HVM` capability.  This is available only on
    /// the VM file descriptor.
    write_ptr kvm_xen_hvm_config with KVMIO, 0x7a; XenHvmConfig
}

ioctl! {
    /// Gets the current timestamp of kvmclock as seen by the current guest. In
    /// conjunction with `kvm_set_clock`, it is used to ensure monotonicity on scenarios
    /// such as migration.
    ///
    /// When `KVM_CAP_ADJUST_CLOCK` is passed to `KVM_CHECK_EXTENSION`, it returns the
    /// set of bits that KVM can return in struct kvm_clock_data's flag member.
    ///
    /// The only flag defined now is `KVM_CLOCK_TSC_STABLE`.  If set, the returned
    /// value is the exact kvmclock value seen by all VCPUs at the instant
    /// when `KVM_GET_CLOCK` was called.  If clear, the returned value is simply
    /// `CLOCK_MONOTONIC` plus a constant offset; the offset can be modified
    /// with `KVM_SET_CLOCK`.  KVM will try to make all VCPUs follow this clock,
    /// but the exact value read by each VCPU could differ, because the host
    /// TSC is not stable.
    ///
    /// # Support
    /// This ioctl is only supported by x86 architecture. This requires the
    /// `KVM_CAP_ADJUST_CLOCK` capability.  This is available only on
    /// the VM file descriptor.
    read kvm_get_clock with KVMIO, 0x7c; ClockData
}

ioctl! {
    /// Sets the current timestamp of kvmclock as seen by the current guest.  In
    /// conjunction with KVM_GET_CLOCK, it is used to ensure monotonicity on
    /// scenarios such as migration.
    ///
    /// # Support
    /// This ioctl is only supported by x86 architecture. This requires the
    /// `KVM_CAP_ADJUST_CLOCK` capability.  This is available only on
    /// the VM file descriptor.
    write_ptr kvm_set_clock with KVMIO, 0x7b; ClockData
}

ioctl! {
    /// This ioctl allows the user to create or modify a guest physical memory
    /// slot.  When changing an existing slot, it may be moved in the guest
    /// physical memory space, or its flags may be modified.  It may not be
    /// resized.  Slots may not overlap in guest physical address space.
    /// Bits 0-15 of `slot` specifies the slot id and this value should be
    /// less than the maximum number of user memory slots supported per VM.
    /// The maximum allowed slots can be queried using the `KVM_CAP_NR_MEMSLOTS`
    /// capability check, if this capability is supported by the architecture.
    ///
    /// If KVM_CAP_MULTI_ADDRESS_SPACE is available, bits 16-31 of "slot"
    /// specifies the address space which is being modified.  They must be
    /// less than the value that `kvm_check_extension` returns for the
    /// `KVM_CAP_MULTI_ADDRESS_SPACE` capability.  Slots in separate address spaces
    /// are unrelated; the restriction on overlapping slots only applies within
    /// each address space.
    ///
    /// Memory for the region is taken starting at the address denoted by the
    /// field `userspace_addr`, which must point at user addressable memory for
    /// the entire memory slot size.  Any object may back this memory, including
    /// anonymous memory, ordinary files, and hugetlbfs.
    ///
    /// It is recommended that the lower 21 bits of guest_phys_addr and userspace_addr
    /// be identical.  This allows large pages in the guest to be backed by large
    /// pages in the host.
    ///
    /// The flags field supports two flags: `KVM_MEM_LOG_DIRTY_PAGES` and
    /// `KVM_MEM_READONLY`.  The former can be set to instruct KVM to keep track of
    /// writes to memory within the slot.  See `kvm_get_dirty_log` to know how to
    /// use it.  The latter can be set, if `KVM_CAP_READONLY_MEM` capability allows it,
    /// to make a new slot read-only.  In this case, writes to this memory will be
    /// posted to userspace as `KVM_EXIT_MMIO` exits.
    ///
    /// When the `KVM_CAP_SYNC_MMU` capability is available, changes in the backing of
    /// the memory region are automatically reflected into the guest.  For example, an
    /// `mmap` that affects the region will be made visible immediately.  Another
    /// example is madvise(MADV_DROP).
    ///
    /// It is recommended to use this API instead of the `KVM_SET_MEMORY_REGION` ioctl.
    /// The `KVM_SET_MEMORY_REGION` does not allow fine grained control over memory
    /// allocation and is deprecated.
    ///
    /// # Support
    /// This ioctl is supported by all architectures, and requires the
    /// `KVM_CAP_USER_MEM` basic capability.  This is available only on the VM file
    /// descriptor.
    write_ptr kvm_set_user_memory_region with KVMIO, 0x46; UserspaceMemoryRegion
}

ioctl! {
    /// This ioctl defines the physical address of a three-page region in the guest
    /// physical address space.  The region must be within the first 4GB of the
    /// guest physical address space and must not conflict with any memory slot
    /// or any mmio address.  The guest may malfunction if it accesses this memory
    /// region.
    ///
    /// **This ioctl is required on Intel-based hosts.**  This is needed on Intel
    /// hardware because of a quirk in the virtualization implementation (see the
    /// internals documentation when it pops into existence).
    ///
    /// A good choice for this may be `0xfffbd000`.
    ///
    /// # Support
    /// This ioctl is supported only by the x86 architecture, and requires the
    /// `KVM_CAP_SET_TSS_ADDR` basic capability.  This is available only on the VM
    /// file descriptor.
    write_int kvm_set_tss_addr with KVMIO, 0x47
}

ioctl! {
    /// Not all extensions are enabled by default. Using this ioctl the application
    /// can enable an extension, making it available to the guest.
    ///
    /// On systems that do not support this ioctl, it always fails. On systems that
    /// do support it, it only works for extensions that are supported for enablement.
    ///
    /// To check if a capability can be enabled, the `kvm_check_extension` ioctl should
    /// be used.
    ///
    /// # Support
    /// This is only available on either the vCPU file descriptor, or on the VM file
    /// descriptor.  It is available:
    ///
    /// - on x86, requiring the `KVM_CAP_ENABLE_CAP_VM` capability, and without vCPU
    ///   file descriptor support;
    /// - on mips, requiring the `KVM_CAP_ENABLE_CAP` capability, and without VM
    ///   file descriptor support;
    /// - on ppc, requiring the `KVM_CAP_ENABLE_CAP` for vCPU file descriptor support,
    ///   and `KVM_CAP_ENABLE_CAP_VM` for VM file descriptor support;
    /// - on s390, requiring the `KVM_CAP_ENABLE_CAP` for vCPU file descriptor support,
    ///   and `KVM_CAP_ENABLE_CAP_VM` for VM file descriptor support.
    write_ptr kvm_enable_cap with KVMIO, 0xa3; EnableCap
}

ioctl! {
    /// Returns the vCPU's current "multiprocessing state" (though also valid on
    /// uniprocessor guests).  Possible values are `KVM_MP_STATE_RUNNABLE`,
    /// `KVM_MP_STATE_UNINITIALIZED`, `KVM_MP_STATE_INIT_RECEIVED`,
    /// `KVM_MP_STATE_HALTED`, `KVM_MP_STATE_SIPI_RECEIVED`, `KVM_MP_STATE_STOPPED`,
    /// `KVM_MP_STATE_CHECK_STOP`, `KVM_MP_STATE_OPERATING`, and `KVM_MP_STATE_LOAD`.
    ///
    /// On x86, this ioctl is only useful after `kvm_create_irqchip`. Without an
    /// in-kernel irqchip, the multiprocessing state must be maintained by userspace on
    /// these architectures.
    ///
    /// On ARM/arm64, the only states that are valid are `KVM_MP_STATE_STOPPED` and
    /// `KVM_MP_STATE_RUNNABLE` which reflect if the vcpu is paused or not.
    ///
    /// # Support
    /// This ioctl is supported only by the x86, s390, ARM, and arm64 architectures,
    /// and requires the `KVM_CAP_MP_STATE` capability.  This is only available on the
    /// vCPU file descriptor.
    read kvm_get_mp_state with KVMIO, 0x98; MpState
}

ioctl! {
    /// Sets the vCPU's current "multiprocessing state." Possible values are
    /// `KVM_MP_STATE_RUNNABLE`, `KVM_MP_STATE_UNINITIALIZED`,
    /// `KVM_MP_STATE_INIT_RECEIVED`, `KVM_MP_STATE_HALTED`,
    /// `KVM_MP_STATE_SIPI_RECEIVED`, `KVM_MP_STATE_STOPPED`,
    /// `KVM_MP_STATE_CHECK_STOP`, `KVM_MP_STATE_OPERATING`, and
    /// `KVM_MP_STATE_LOAD`.
    ///
    /// On x86, this ioctl is only useful after `kvm_create_irqchip`. Without an
    /// in-kernel irqchip, the multiprocessing state must be maintained by userspace on
    /// these architectures.
    ///
    /// On ARM/arm64, The only states that are valid are `KVM_MP_STATE_STOPPED` and
    /// `KVM_MP_STATE_RUNNABLE` which reflect if the vCPU should be paused or not.
    ///
    /// # Support
    /// This ioctl is supported only by the x86, s390, ARM, and arm64 architectures,
    /// and requires the `KVM_CAP_MP_STATE` capability.  This is only available on the
    /// vCPU file descriptor.
    write_ptr kvm_set_mp_state with KVMIO, 0x99; MpState
}

ioctl! {
    /// This ioctl defines the physical address of a one-page region in the guest
    /// physical address space.  The region must be within the first 4GB of the
    /// guest physical address space and must not conflict with any memory slot
    /// or any mmio address.  The guest may malfunction if it accesses this memory
    /// region.
    ///
    /// Setting the address to 0 will result in resetting the address to its default
    /// (`0xfffbc000`).
    ///
    /// **This ioctl is required on Intel-based hosts.**  This is needed on Intel
    /// hardware because of a quirk in the virtualization implementation (see the
    /// internals documentation when it pops into existence).
    ///
    /// Fails if any vCPU has already been created.
    ///
    /// # Support
    /// This ioctl is supported only by the x86 architecture, and requires the
    /// `KVM_CAP_SET_IDENTITY_MAP_ADDR` capability.  This is only available on the
    /// VM file descriptor.
    write_int kvm_set_identity_map_addr with KVMIO, 0x48
}
